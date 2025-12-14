use ffmpeg_sidecar::command::FfmpegCommand;
use log::info;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{error::Error, fs::read_dir, path::Path};

use crate::image::image_struct::{apply_image_format_specific_args, Image};
use crate::image::image_validator::ImageSettingsValidator;
use crate::shared::ffmpeg_processor::spawn_ffmpeg_process;
use crate::shared::ffmpeg_structs::FfmpegBatchCommand;
use crate::shared::file_utils::{clear_and_create_folder, get_relative_path};
use crate::shared::logo_handler::handle_logos;
use crate::shared::logo_structs::Logo;
use crate::shared::media_structs::{Media, Resolution};
use crate::shared::media_validator::{
    filter_valid_media_paths, read_media_paths_recursive, sort_by_file_size,
};
use crate::shared::process_manager::{check_process_cancelled, ProcessManager};
use crate::shared::progress_handler::{ProgressManager, ProgressMode};
use crate::ImageSettings;

pub fn handle_images(image_settings: &ImageSettings) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Clear any previous processes at the start
    ProcessManager::clear();

    let input_directory = &image_settings.input_directory;
    let output_directory = &image_settings.output_directory;

    let mut image_list;

    let start_time = std::time::Instant::now();

    ProgressManager::start_progress_with_terminal(
        "Clearing and creating output folder... (Step 1/7)".to_string(),
        None,
        Some("images".to_string()),
        None,
        None,
    );

    check_process_cancelled()?;

    if image_settings.clear_files_output_directory || !output_directory.exists() {
        let clear_folder_time = std::time::Instant::now();
        clear_and_create_folder(output_directory).unwrap();
        info!(
            "Clearing and creating output directory took: {:?}",
            clear_folder_time.elapsed()
        );
    }

    ProgressManager::set_status(
        "Reading image paths from input directory... (Step 2/7)".to_string(),
    );
    check_process_cancelled()?;

    let read_paths_time = std::time::Instant::now();
    let valid_image_paths =
        read_image_paths_from_input_directory(image_settings, input_directory, output_directory)?;
    info!("Reading image paths took: {:?}", read_paths_time.elapsed());

    if valid_image_paths.is_empty() {
        ProgressManager::set_status("No images found in the input directory".to_string());
        info!("No images found in the input directory, returning early.");
        info!("Total time: {:?}", start_time.elapsed());
        return Ok(());
    }

    check_process_cancelled()?;

    ProgressManager::set_status("Creating image structs... (Step 3/7)".to_string());
    let image_creation_time = std::time::Instant::now();
    image_list = create_images_from_paths_parallel(&valid_image_paths)?;
    info!(
        "Creating image structs took: {:?}",
        image_creation_time.elapsed()
    );

    if image_list.is_empty() {
        ProgressManager::set_status("No valid images could be loaded".to_string());
        info!("No valid images could be loaded, returning early.");
        info!("Total time: {:?}", start_time.elapsed());
        return Ok(());
    }

    check_process_cancelled()?;

    ProgressManager::set_status("Sorting images by file size... (Step 4/7)".to_string());
    let sort_start = std::time::Instant::now();
    sort_by_file_size(&mut image_list);
    info!(
        "Sorting images by file size took: {:?}",
        sort_start.elapsed()
    );

    check_process_cancelled()?;

    ProgressManager::set_status("Applying image settings... (Step 5/7)".to_string());
    let apply_settings_start = std::time::Instant::now();
    apply_image_settings_per_image(image_settings, &mut image_list)?;
    info!(
        "Applying image settings took: {:?}",
        apply_settings_start.elapsed()
    );

    ProgressManager::set_status("Processing logos... (Step 6/7)".to_string());
    let logo_processing_start = std::time::Instant::now();
    let logo_list = process_logos_for_image_resolutions(image_settings, &image_list)?;
    info!(
        "Processing logos took: {:?}",
        logo_processing_start.elapsed()
    );

    check_process_cancelled()?;

    ProgressManager::set_status("Processing images... (Step 7/7)".to_string());
    ProgressManager::set_total(image_list.len());
    let image_processing_start = std::time::Instant::now();
    process_images_from_image_list(
        output_directory,
        image_list,
        logo_list,
        image_settings,
        input_directory,
    )?;

    ProgressManager::finish_progress();

    info!(
        "Processing images took: {:?}",
        image_processing_start.elapsed()
    );

    info!("Total time: {:?}", start_time.elapsed());

    Ok(())
}

/// Apply the image settings per image in parallel
fn apply_image_settings_per_image(
    image_settings: &ImageSettings,
    image_list: &mut Vec<Image>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    check_process_cancelled()?;

    // Use try_for_each to allow early termination on cancellation
    image_list.par_iter_mut().try_for_each(
        |image| -> Result<(), Box<dyn Error + Send + Sync>> {
            check_process_cancelled()?;

            image.resize_dimensions(&image_settings.min_pixel_count);
            image.file_type = image_settings.format.clone();
            Ok(())
        },
    )?;

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct BatchKey {
    resolution: Resolution,
    file_type: String,
}

/// Process the images from the image list in batches sequentially by size
fn process_images_from_image_list(
    output_directory: &Path,
    image_list: Vec<Image>,
    logo_list: Option<Vec<Logo>>,
    image_settings: &ImageSettings,
    input_directory: &Path,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    check_process_cancelled()?;

    // Group images by resolution and file type to create initial batches
    let mut batches: HashMap<BatchKey, Vec<Image>> = HashMap::new();

    for image in image_list {
        let key = BatchKey {
            resolution: image.resolution.clone(),
            file_type: image.file_type.clone(),
        };
        batches.entry(key).or_default().push(image);
    }

    info!("Created {} initial batches for processing", batches.len());

    check_process_cancelled()?;

    let mut ffmpeg_command_list: Vec<FfmpegBatchCommand> = Vec::new();

    for (batch_key, images) in batches {
        // Check cancellation at the start of each work unit
        check_process_cancelled()?;

        let logo: Option<&Logo> = if let Some(ref logo_list) = logo_list {
            logo_list
                .iter()
                .find(|logo| logo.compatible_image_resolution == batch_key.resolution)
        } else {
            None
        };

        if logo.is_none() && logo_list.is_some() {
            return Err(format!(
                "No logo found for the given image resolution: {}",
                batch_key.resolution
            )
            .into());
        }

        // Prepare batch data with output directories
        let batch_data: Vec<(Image, PathBuf)> = images
            .iter()
            .map(|image| {
                let final_output_directory = if image_settings
                    .keep_child_folders_structure_in_output_directory
                {
                    let relative_image_path = get_relative_path(input_directory, &image.file_path)
                        .unwrap_or_else(|_| PathBuf::from(""));
                    let relative_dir_path = relative_image_path.parent().unwrap_or(Path::new(""));
                    output_directory.join(relative_dir_path)
                } else {
                    output_directory.to_path_buf()
                };
                (image.clone(), final_output_directory)
            })
            .collect();

        info!(
            "Processing work unit with {} images ({}x{}, {})",
            batch_data.len(),
            batch_key.resolution.width,
            batch_key.resolution.height,
            batch_key.file_type
        );
        ProgressManager::redraw_progress();

        create_image_ffmpeg_command_list(&batch_data, logo, &mut ffmpeg_command_list).map_err(
            |e| -> Box<dyn Error + Send + Sync> {
                format!("Failed to process image batch: {}", e).into()
            },
        )?;
    }

    // Sort the commands by batch size
    ffmpeg_command_list.sort_by(|a, b| b.batch_size.cmp(&a.batch_size));

    // Execute FFmpeg commands in parallel
    ffmpeg_command_list.into_iter().par_bridge().try_for_each(
        |mut ffmpeg_batch_command| -> Result<(), Box<dyn Error + Send + Sync>> {
            spawn_ffmpeg_process(&mut ffmpeg_batch_command, ProgressMode::Batch)?;
            Ok(())
        },
    )?;

    Ok(())
}

fn process_logos_for_image_resolutions(
    image_settings: &ImageSettings,
    image_list: &Vec<Image>,
) -> Result<Option<Vec<Logo>>, Box<dyn Error + Send + Sync>> {
    check_process_cancelled()?;

    let logo_list: Option<Vec<Logo>> = if image_settings.add_logo {
        // Make a hashset of all the unique resolutions of the Images
        let mut unique_resolutions = std::collections::HashSet::new();
        for image in image_list {
            unique_resolutions.insert(image.resolution.clone());
        }
        let unique_resolutions: Vec<Resolution> = unique_resolutions.into_iter().collect();

        // Create a vector to store Logo structs for each unique resolution
        let logos = handle_logos(image_settings, unique_resolutions)?;
        Some(logos)
    } else {
        None
    };
    Ok(logo_list)
}

/// Reads all image paths from the input directory
fn read_image_paths_from_input_directory(
    image_settings: &ImageSettings,
    input_directory: &Path,
    output_directory: &Path,
) -> Result<Vec<PathBuf>, Box<dyn Error + Send + Sync>> {
    let validator = ImageSettingsValidator::new(image_settings);

    if image_settings.search_child_folders {
        read_media_paths_recursive(input_directory, output_directory, &validator)
    } else {
        let dir_read_start = std::time::Instant::now();
        let entries: Result<Vec<_>, _> = read_dir(input_directory)?.collect();
        let entries = entries?;
        info!("Directory read took: {:?}", dir_read_start.elapsed());

        let filter_start = std::time::Instant::now();
        let entry_paths = entries.iter().map(|entry| entry.path());
        let valid_image_paths =
            filter_valid_media_paths(entry_paths, input_directory, output_directory, &validator);
        info!("Path filtering took: {:?}", filter_start.elapsed());
        info!("Found {} valid image paths", valid_image_paths.len());

        Ok(valid_image_paths)
    }
}

/// Creates Image objects from paths in parallel, filtering out failed creations
fn create_images_from_paths_parallel(
    paths: &[PathBuf],
) -> Result<Vec<Image>, Box<dyn Error + Send + Sync>> {
    paths
        .par_iter()
        .filter_map(|path| {
            // Check cancellation first
            if let Err(e) = check_process_cancelled() {
                return Some(Err(e));
            }

            match Image::new(path.clone()) {
                Ok(image) => Some(Ok(image)),
                Err(e) => {
                    log::error!("Failed to load image {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect()
}
pub fn create_image_ffmpeg_command_list(
    batch_data: &[(Image, PathBuf)],
    logo: Option<&Logo>,
    ffmpeg_command_list: &mut Vec<FfmpegBatchCommand>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if batch_data.is_empty() {
        return Ok(());
    }

    let first_image = &batch_data[0].0;
    let target_resolution = &first_image.resolution;
    let target_file_type = &first_image.file_type;

    info!(
        "Processing batch of {} images with resolution {}x{} and format {}",
        batch_data.len(),
        target_resolution.width,
        target_resolution.height,
        target_file_type,
    );

    // Process in chunks for better load balancing and more frequent progress bar progression
    const CHUNK_SIZE: usize = 10;

    if batch_data.len() <= CHUNK_SIZE {
        let batch_command =
            create_image_ffmpeg_command(batch_data, logo, target_resolution, target_file_type)?;
        info!(
            "Created command for batch of {} images",
            batch_command.batch_size
        );
        ffmpeg_command_list.push(batch_command);
    } else {
        let num_chunks = batch_data.len().div_ceil(CHUNK_SIZE);
        let optimal_chunk_size = batch_data.len().div_ceil(num_chunks);

        for chunk in batch_data.chunks(optimal_chunk_size) {
            let batch_command =
                create_image_ffmpeg_command(chunk, logo, target_resolution, target_file_type)?;
            info!(
                "Created command for batch of {} images",
                batch_command.batch_size
            );
            ffmpeg_command_list.push(batch_command);
        }
    }

    Ok(())
}

fn create_image_ffmpeg_command(
    batch_data: &[(Image, PathBuf)],
    logo: Option<&Logo>,
    target_resolution: &Resolution,
    target_file_type: &str,
) -> Result<FfmpegBatchCommand, Box<dyn Error + Send + Sync>> {
    check_process_cancelled()?;

    // Create output directories
    for (_, output_directory) in batch_data {
        std::fs::create_dir_all(output_directory)?;
    }

    let mut cmd = FfmpegCommand::new();

    #[cfg(target_os = "windows")]
    cmd.hide_banner();

    cmd.args(["-y", "-an", "-vsync", "0"]);

    // Add all input images in this chunk
    for (image, _) in batch_data.iter() {
        cmd.input(image.file_path.to_str().ok_or("Invalid image file path")?);
    }

    // Add logo input if present
    if let Some(logo_ref) = logo {
        cmd.input(
            logo_ref
                .file_path
                .to_str()
                .ok_or("Invalid logo file path")?,
        );
    }

    // Build complex filter for this chunk
    let mut filter_parts = Vec::new();

    for (i, _) in batch_data.iter().enumerate() {
        if let Some(logo_ref) = logo {
            // Scale and overlay logo for each image
            let logo_idx = batch_data.len(); // Logo is the last input
            filter_parts.push(format!(
                "[{}:v]scale={}:{}:flags=fast_bilinear[scaled{}];[scaled{}][{}:v]overlay={}:{}[out{}]",
                i, target_resolution.width, target_resolution.height, i,
                i, logo_idx, logo_ref.position.x, logo_ref.position.y, i
            ));
        } else {
            // Scale each image without overlaying logo
            filter_parts.push(format!(
                "[{}:v]scale={}:{}:flags=fast_bilinear[out{}]",
                i, target_resolution.width, target_resolution.height, i
            ));
        }
    }

    let filter_complex = filter_parts.join(";");
    cmd.args(["-filter_complex", &filter_complex]);

    // Add output mappings and files
    for (i, (image, output_directory)) in batch_data.iter().enumerate() {
        let file_stem = image
            .file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid file name")?;

        let new_filename = format!("{}.{}", file_stem, target_file_type);
        let output_file = output_directory.join(new_filename);

        cmd.args(["-map", &format!("[out{}]", i)]);
        apply_image_format_specific_args(target_file_type, &mut cmd);
        cmd.output(output_file.to_str().ok_or("Invalid output file path")?);
    }

    // Return the command wrapped in ImageBatchCommand struct
    Ok(FfmpegBatchCommand {
        command: cmd,
        batch_size: batch_data.len(),
    })
}
