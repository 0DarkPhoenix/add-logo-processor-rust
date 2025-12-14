use ffmpeg_sidecar::command::FfmpegCommand;
use log::info;
use rayon::prelude::*;
use std::path::PathBuf;
use std::{error::Error, fs::read_dir, path::Path};

use crate::shared::ffmpeg_processor::spawn_ffmpeg_process;
use crate::shared::ffmpeg_structs::FfmpegBatchCommand;
use crate::shared::file_utils::{clear_and_create_folder, get_relative_path};
use crate::shared::logo_handler::handle_logos;
use crate::shared::logo_structs::Logo;
use crate::shared::media_structs::{Media, Resolution};
use crate::shared::media_validator::{
    create_media_from_paths_parallel, filter_valid_media_paths, read_media_paths_recursive,
    sort_by_file_size,
};
use crate::shared::process_manager::{check_process_cancelled, ProcessManager};
use crate::shared::progress_handler::{ProgressManager, ProgressMode};
use crate::video::video_structs::Video;
use crate::video::video_validator::VideoSettingsValidator;
use crate::VideoSettings;

pub fn handle_videos(video_settings: &VideoSettings) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Clear any previous processes at the start
    ProcessManager::clear();

    let input_directory = &video_settings.input_directory;
    let output_directory = &video_settings.output_directory;

    let mut video_list;

    let start_time = std::time::Instant::now();

    ProgressManager::start_progress_with_terminal(
        "Clearing and creating output folder... (Step 1/6)".to_string(),
        None,
        Some("frames".to_string()),
        None,
        Some("videos".to_string()),
    );

    check_process_cancelled()?;

    if video_settings.clear_files_output_directory || !output_directory.exists() {
        let clear_folder_time = std::time::Instant::now();
        clear_and_create_folder(output_directory).unwrap();
        info!(
            "Clearing and creating output directory took: {:?}",
            clear_folder_time.elapsed()
        );
    }

    ProgressManager::set_status(
        "Reading video paths from input directory... (Step 2/6)".to_string(),
    );
    check_process_cancelled()?;

    let read_paths_time = std::time::Instant::now();
    let valid_video_paths =
        read_video_paths_from_input_directory(video_settings, input_directory, output_directory)?;
    info!("Reading video paths took: {:?}", read_paths_time.elapsed());

    if valid_video_paths.is_empty() {
        ProgressManager::set_status("No videos found in the input directory".to_string());
        info!("No videos found in the input directory, returning early.");
        info!("Total time: {:?}", start_time.elapsed());
        return Ok(());
    }

    check_process_cancelled()?;

    ProgressManager::set_status("Creating video structs... (Step 3/6)".to_string());
    let video_creation_time = std::time::Instant::now();
    video_list = create_media_from_paths_parallel(&valid_video_paths, Video::new)?;
    info!(
        "Creating video structs took: {:?}",
        video_creation_time.elapsed()
    );

    if video_list.is_empty() {
        ProgressManager::set_status("No valid videos could be loaded".to_string());
        info!("No valid videos could be loaded, returning early.");
        info!("Total time: {:?}", start_time.elapsed());
        return Ok(());
    }

    check_process_cancelled()?;

    ProgressManager::set_status("Sorting videos by file size... (Step 4/6)".to_string());
    let sort_start = std::time::Instant::now();
    sort_by_file_size(&mut video_list);
    info!(
        "Sorting videos by file size took: {:?}",
        sort_start.elapsed()
    );

    check_process_cancelled()?;

    ProgressManager::set_status("Applying video settings... (Step 5/6)".to_string());
    let apply_settings_start = std::time::Instant::now();
    apply_video_settings_per_video(video_settings, &mut video_list)?;
    info!(
        "Applying video settings took: {:?}",
        apply_settings_start.elapsed()
    );

    ProgressManager::set_status("Processing logos... (Step 6/6)".to_string());
    let logo_processing_start = std::time::Instant::now();
    let logo_list = process_logos_for_video_resolutions(video_settings, &video_list)?;
    info!(
        "Processing logos took: {:?}",
        logo_processing_start.elapsed()
    );

    check_process_cancelled()?;

    let total_frame_count: usize = video_list.iter().map(|video| video.frame_count).sum();

    ProgressManager::set_status("Processing videos... (Step 7/7)".to_string());
    ProgressManager::set_total(total_frame_count);
    ProgressManager::set_alternative_total(video_list.len());
    let video_processing_start = std::time::Instant::now();

    process_videos_from_video_list(
        output_directory,
        video_list,
        logo_list,
        video_settings,
        input_directory,
    )?;

    ProgressManager::finish_progress();

    info!(
        "Processing videos took: {:?}",
        video_processing_start.elapsed()
    );

    info!("Total time: {:?}", start_time.elapsed());

    Ok(())
}

/// Apply the video settings per video in parallel
fn apply_video_settings_per_video(
    video_settings: &VideoSettings,
    video_list: &mut [Video],
) -> Result<(), Box<dyn Error + Send + Sync>> {
    check_process_cancelled()?;

    video_list.par_iter_mut().try_for_each(
        |video| -> Result<(), Box<dyn Error + Send + Sync>> {
            check_process_cancelled()?;

            video.resize_dimensions(&video_settings.min_pixel_count);
            video.file_type = video_settings.format.clone();
            video.codec = video_settings.codec.clone();
            Ok(())
        },
    )?;

    Ok(())
}

fn process_videos_from_video_list(
    output_directory: &Path,
    video_list: Vec<Video>,
    logo_list: Option<Vec<Logo>>,
    video_settings: &VideoSettings,
    input_directory: &Path,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    check_process_cancelled()?;

    let mut ffmpeg_command_list: Vec<FfmpegBatchCommand> = Vec::new();

    for video in video_list {
        check_process_cancelled()?;

        let logo: Option<&Logo> = if let Some(ref logo_list) = logo_list {
            logo_list
                .iter()
                .find(|logo| logo.compatible_image_resolution == video.resolution)
        } else {
            None
        };

        let final_output_directory =
            if video_settings.keep_child_folders_structure_in_output_directory {
                let relative_video_path = get_relative_path(input_directory, &video.file_path)
                    .unwrap_or_else(|_| PathBuf::from(""));
                let relative_dir_path = relative_video_path.parent().unwrap_or(Path::new(""));
                output_directory.join(relative_dir_path)
            } else {
                output_directory.to_path_buf()
            };

        let batch_command = create_video_ffmpeg_command(&video, logo, &final_output_directory)?;
        ffmpeg_command_list.push(batch_command);
    }

    // Execute FFmpeg commands in parallel
    ffmpeg_command_list.into_iter().par_bridge().try_for_each(
        |mut ffmpeg_batch_command| -> Result<(), Box<dyn Error + Send + Sync>> {
            spawn_ffmpeg_process(&mut ffmpeg_batch_command, ProgressMode::PerFrame)?;
            Ok(())
        },
    )?;

    Ok(())
}

fn create_video_ffmpeg_command(
    video: &Video,
    logo: Option<&Logo>,
    output_directory: &Path,
) -> Result<FfmpegBatchCommand, Box<dyn Error + Send + Sync>> {
    check_process_cancelled()?;

    // Create output directory
    std::fs::create_dir_all(output_directory)?;

    let mut cmd = FfmpegCommand::new();

    #[cfg(target_os = "windows")]
    cmd.hide_banner();

    cmd.input(video.file_path.to_str().ok_or("Invalid video file path")?);

    if let Some(logo) = logo {
        cmd.input(logo.file_path.to_str().ok_or("Invalid logo file path")?);
    }

    if let Some(logo) = logo {
        let filter_complex = format!(
            "[0:v]scale={}:{}[resized];[resized][1:v]overlay={}:{}[final]",
            video.resolution.width, video.resolution.height, logo.position.x, logo.position.y
        );
        cmd.args(["-filter_complex", &filter_complex]);
        cmd.args(["-map", "[final]"]);
    } else {
        let filter_complex = format!(
            "[0:v]scale={}:{}[final]",
            video.resolution.width, video.resolution.height
        );
        cmd.args(["-filter_complex", &filter_complex]);
        cmd.args(["-map", "[final]"]);
    }

    cmd.args(["-map", "0:a?"]);

    let file_stem = video
        .file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid file name")?;

    let new_filename = format!("{}.{}", file_stem, video.file_type);
    let output_file = output_directory.join(new_filename);

    cmd.output(output_file.to_str().ok_or("Invalid output file path")?);

    Ok(FfmpegBatchCommand {
        command: cmd,
        batch_size: 1,
    })
}

fn process_logos_for_video_resolutions(
    video_settings: &VideoSettings,
    video_list: &Vec<Video>,
) -> Result<Option<Vec<Logo>>, Box<dyn Error + Send + Sync>> {
    let logo_list: Option<Vec<Logo>> = if video_settings.add_logo {
        // Make a hashset of all the unique resolutions of the Videos
        let mut unique_resolutions = std::collections::HashSet::new();
        for video in video_list {
            unique_resolutions.insert(video.resolution.clone());
        }
        let unique_resolutions: Vec<Resolution> = unique_resolutions.into_iter().collect();

        // Create a vector to store Logo structs for each unique resolution
        let logos = handle_logos(video_settings, unique_resolutions)?;
        Some(logos)
    } else {
        None
    };
    Ok(logo_list)
}

/// Reads all video paths from the input directory
fn read_video_paths_from_input_directory(
    video_settings: &VideoSettings,
    input_directory: &Path,
    output_directory: &Path,
) -> Result<Vec<PathBuf>, Box<dyn Error + Send + Sync>> {
    let validator = VideoSettingsValidator::new(video_settings);

    if video_settings.search_child_folders {
        read_media_paths_recursive(input_directory, output_directory, &validator)
    } else {
        let dir_read_start = std::time::Instant::now();
        let entries: Result<Vec<_>, _> = read_dir(input_directory)?.collect();
        let entries = entries?;
        info!("Directory read took: {:?}", dir_read_start.elapsed());

        let filter_start = std::time::Instant::now();
        let entry_paths = entries.iter().map(|entry| entry.path());
        let valid_video_paths =
            filter_valid_media_paths(entry_paths, input_directory, output_directory, &validator);
        info!("Path filtering took: {:?}", filter_start.elapsed());
        info!("Found {} valid video paths", valid_video_paths.len());

        Ok(valid_video_paths)
    }
}
