use rayon::prelude::*;
use std::path::PathBuf;
use std::{error::Error, fs::read_dir, path::Path};
use walkdir::WalkDir;

use crate::utils::{clear_and_create_folder, get_relative_path};
use crate::{
    handlers::handle_logos,
    media::{Logo, Media, Resolution, Video},
    processors::process_video,
    utils::config::VideoSettings,
};

pub fn handle_videos(video_settings: &VideoSettings) -> Result<(), Box<dyn Error + Send + Sync>> {
    let input_directory = &video_settings.input_directory;
    let output_directory = &video_settings.output_directory;

    let mut video_list = Vec::new();

    let start_time = std::time::Instant::now();

    if video_settings.clear_files_output_directory || !output_directory.exists() {
        let clear_folder_time = std::time::Instant::now();
        clear_and_create_folder(output_directory).unwrap();
        println!(
            "Clearing and creating output directory took: {:?}",
            clear_folder_time.elapsed()
        );
    }

    let read_videos_time = std::time::Instant::now();
    read_videos_in_input_directory(
        video_settings,
        input_directory,
        &mut video_list,
        output_directory,
    )?;
    println!("Reading videos took: {:?}", read_videos_time.elapsed());

    if video_list.is_empty() {
        println!("No videos found in the input directory, returning early.");
        println!("Total time: {:?}", start_time.elapsed());
        return Ok(());
    }

    let sort_start = std::time::Instant::now();
    sort_list_by_file_size(&mut video_list);
    println!(
        "Sorting videos by file size took: {:?}",
        sort_start.elapsed()
    );

    let apply_settings_start = std::time::Instant::now();
    apply_video_settings_per_video(video_settings, &mut video_list);
    println!(
        "Applying video settings took: {:?}",
        apply_settings_start.elapsed()
    );

    let logo_processing_start = std::time::Instant::now();
    let logo_list = process_logos_for_video_resolutions(video_settings, &video_list)?;
    println!(
        "Processing logos took: {:?}",
        logo_processing_start.elapsed()
    );

    let video_processing_start = std::time::Instant::now();
    process_videos_from_video_list(
        output_directory,
        video_list,
        logo_list,
        video_settings,
        input_directory,
    )?;
    println!(
        "Processing videos took: {:?}",
        video_processing_start.elapsed()
    );

    println!("Total time: {:?}", start_time.elapsed());

    Ok(())
}

/// Apply the video settings per video in parallel
fn apply_video_settings_per_video(video_settings: &VideoSettings, video_list: &mut Vec<Video>) {
    let format = &video_settings.format;
    let codec = &video_settings.codec;
    let min_pixel_count = video_settings.min_pixel_count;

    video_list.par_iter_mut().for_each(|video| {
        video.resize_dimensions(min_pixel_count);
        video.file_type = format.clone();
        video.codec = codec.clone();
    });
}

/// Process the videos from the video list in parallel
fn process_videos_from_video_list(
    output_directory: &Path,
    video_list: Vec<Video>,
    logo_list: Option<Vec<Logo>>,
    video_settings: &VideoSettings,
    input_directory: &Path,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    video_list
        .par_iter()
        .try_for_each(|video| -> Result<(), Box<dyn Error + Send + Sync>> {
            let logo: Option<&Logo> = if let Some(ref logo_list) = logo_list {
                logo_list
                    .iter()
                    .find(|logo| logo.compatible_image_resolution == video.resolution)
            } else {
                None
            };

            if logo.is_none() && logo_list.is_some() {
                return Err(format!(
                    "No logo found for the given video resolution: {}",
                    video.resolution
                )
                .into());
            }

            let final_output_directory =
                if video_settings.keep_child_folders_structure_in_output_directory {
                    let relative_video_path = get_relative_path(input_directory, &video.file_path)
                        .map_err(|e| -> Box<dyn Error + Send + Sync> {
                            format!("Failed to get relative path: {}", e).into()
                        })?;
                    let relative_dir_path = relative_video_path.parent().unwrap_or(Path::new(""));
                    output_directory.join(relative_dir_path)
                } else {
                    output_directory.to_path_buf()
                };

            process_video(video, logo, &final_output_directory).map_err(
                |e| -> Box<dyn Error + Send + Sync> {
                    format!("Failed to process video: {}", e).into()
                },
            )
        })?;
    Ok(())
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

/// Reads all videos in the input directory, and adds them to the video list
fn read_videos_in_input_directory(
    video_settings: &VideoSettings,
    input_directory: &Path,
    video_list: &mut Vec<Video>,
    output_directory: &Path,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if video_settings.search_child_folders {
        read_videos_recursive_parallel(
            input_directory,
            video_list,
            output_directory,
            video_settings,
        )?;
    } else {
        let dir_read_start = std::time::Instant::now();
        let entries: Result<Vec<_>, _> = read_dir(input_directory)?.collect();
        let entries = entries?;
        println!("Directory read took: {:?}", dir_read_start.elapsed());

        let filter_start = std::time::Instant::now();
        let entry_paths = entries.iter().map(|entry| entry.path());
        let valid_video_paths = filter_valid_video_paths(
            entry_paths,
            input_directory,
            output_directory,
            video_settings,
        );
        println!("Path filtering took: {:?}", filter_start.elapsed());
        println!("Found {} valid video paths", valid_video_paths.len());

        let video_creation_start = std::time::Instant::now();
        let videos = create_videos_from_paths_parallel(&valid_video_paths);
        println!("Video creation took: {:?}", video_creation_start.elapsed());

        video_list.extend(videos);
    }
    Ok(())
}

/// Determine if the video should be written to the output directory.
///
/// This is determined based on if the video already exists in the output directory and if it is allowed to be overwritten
fn write_to_output_directory(
    path: &Path,
    input_directory: &Path,
    output_directory: &Path,
    video_settings: &VideoSettings,
) -> bool {
    if video_settings.overwrite_existing_files_output_directory {
        return true;
    }

    // Get the file stem (filename without extension)
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Get the target extension based on the format setting
    let target_extension = &video_settings.format;

    let target_filename = format!("{}.{}", file_stem, target_extension);

    if video_settings.keep_child_folders_structure_in_output_directory {
        let relative_video_path = get_relative_path(input_directory, path).unwrap();
        let relative_dir_path = relative_video_path.parent().unwrap_or(Path::new(""));
        let target_output_path = output_directory
            .join(relative_dir_path)
            .join(target_filename);
        return !target_output_path.exists();
    }

    let target_output_path = output_directory.join(target_filename);
    !target_output_path.exists()
}

fn is_supported_video_extension(path: &Path) -> bool {
    if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
        matches!(
            extension.to_lowercase().as_str(),
            "mp4" | "avi" | "mov" | "mkv" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ogv"
        )
    } else {
        false
    }
}

/// Recursively read all videos from child directories in parallel
fn read_videos_recursive_parallel(
    directory: &Path,
    video_list: &mut Vec<Video>,
    output_directory: &Path,
    video_settings: &VideoSettings,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let walkdir_paths = WalkDir::new(directory).into_iter().filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() {
            Some(path.to_path_buf())
        } else {
            None
        }
    });

    let valid_video_paths = filter_valid_video_paths(
        walkdir_paths,
        directory, // Use directory as input_directory for recursive case
        output_directory,
        video_settings,
    );

    println!("Found {} video files to process", valid_video_paths.len());

    let videos = create_videos_from_paths_parallel(&valid_video_paths);
    video_list.extend(videos);

    Ok(())
}

/// Sorts the video list by file size in descending order (largest to smallest)
fn sort_list_by_file_size(video_list: &mut [Video]) {
    video_list.sort_by(|a, b| b.file_size.cmp(&a.file_size));
}

/// Filters paths to only include valid video files that should be processed
fn filter_valid_video_paths(
    paths: impl Iterator<Item = PathBuf>,
    input_directory: &Path,
    output_directory: &Path,
    video_settings: &VideoSettings,
) -> Vec<PathBuf> {
    paths
        .filter(|path| {
            path.is_file()
                && is_supported_video_extension(path)
                && write_to_output_directory(
                    path,
                    input_directory,
                    output_directory,
                    video_settings,
                )
        })
        .collect()
}

/// Creates Video objects from paths in parallel, filtering out failed creations
fn create_videos_from_paths_parallel(paths: &[PathBuf]) -> Vec<Video> {
    paths
        .par_iter()
        .filter_map(|path| match Video::new(path.clone()) {
            Ok(video) => Some(video),
            Err(e) => {
                eprintln!("Failed to load video {}: {}", path.display(), e);
                None
            }
        })
        .collect()
}
