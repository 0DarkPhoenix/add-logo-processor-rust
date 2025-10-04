use log::{error, info};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{error::Error, fs::read_dir, path::Path};
use walkdir::WalkDir;

use crate::handlers::process_handler::ProcessManager;
use crate::handlers::progress_handler::ProgressManager;
use crate::utils::{clear_and_create_folder, get_relative_path};
use crate::{
    handlers::handle_logos,
    media::{Image, Logo, Media, Resolution},
    processors::process_image_batch,
    utils::config::ImageSettings,
};

pub fn handle_images(image_settings: &ImageSettings) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Clear any previous processes at the start
    ProcessManager::clear();

    let input_directory = &image_settings.input_directory;
    let output_directory = &image_settings.output_directory;

    let mut image_list = Vec::new();

    let start_time = std::time::Instant::now();

    ProgressManager::start_progress_with_terminal("Reading images... (Step 1/5)".to_string(), None);

    if image_settings.clear_files_output_directory || !output_directory.exists() {
        let clear_folder_time = std::time::Instant::now();
        clear_and_create_folder(output_directory).unwrap();
        info!(
            "Clearing and creating output directory took: {:?}",
            clear_folder_time.elapsed()
        );
    }

    let read_images_time = std::time::Instant::now();
    read_images_in_input_directory(
        image_settings,
        input_directory,
        &mut image_list,
        output_directory,
    )?;
    info!("Reading images took: {:?}", read_images_time.elapsed());

    if image_list.is_empty() {
        ProgressManager::set_status("No images found in the input directory".to_string());
        info!("No images found in the input directory, returning early.");
        info!("Total time: {:?}", start_time.elapsed());
        return Ok(());
    }

    ProgressManager::set_status("Sorting images by file size... (Step 2/5)".to_string());
    let sort_start = std::time::Instant::now();
    sort_list_by_file_size(&mut image_list);
    info!(
        "Sorting images by file size took: {:?}",
        sort_start.elapsed()
    );

    ProgressManager::set_status("Applying image settings... (Step 3/5)".to_string());
    let apply_settings_start = std::time::Instant::now();
    apply_image_settings_per_image(image_settings, &mut image_list);
    info!(
        "Applying image settings took: {:?}",
        apply_settings_start.elapsed()
    );

    ProgressManager::set_status("Processing logos... (Step 4/5)".to_string());
    let logo_processing_start = std::time::Instant::now();
    let logo_list = process_logos_for_image_resolutions(image_settings, &image_list)?;
    info!(
        "Processing logos took: {:?}",
        logo_processing_start.elapsed()
    );

    ProgressManager::set_status("Processing images... (Step 5/5)".to_string());
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
fn apply_image_settings_per_image(image_settings: &ImageSettings, image_list: &mut Vec<Image>) {
    image_list.par_iter_mut().for_each(|image| {
        image.resize_dimensions(&image_settings.min_pixel_count);
        image.file_type = image_settings.format.clone();
    });
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

    // Print batch sizes for debugging
    for (key, images) in &batches {
        info!(
            "Batch {}x{} ({}): {} images",
            key.resolution.width,
            key.resolution.height,
            key.file_type,
            images.len()
        );
    }

    // Calculate optimal number of threads
    let cpu_thread_count = num_cpus::get();
    let total_images = batches.values().map(|v| v.len()).sum::<usize>();

    // Using more batches for better thread utilization
    // After much testing, the optimal number of batches is 2 times the number of CPU threads
    // * 1.5 = +5.08%
    // * 1.75 = +0.86%
    // * 2 = 0% - benchmark
    // * 2.25 = +7.84%
    // * 2.5 = +3.30%
    let optimal_batches = cpu_thread_count * 2;

    info!(
        "Using {} batches for {} total images",
        optimal_batches, total_images
    );

    // Split large batches to better utilize threads
    let work_units = split_batches_optimally(batches, optimal_batches);

    info!(
        "Split into {} work units for optimal thread utilization",
        work_units.len()
    );

    // Process work units in parallel with ordered task distribution
    work_units.into_iter().par_bridge().try_for_each(
        |(batch_key, images)| -> Result<(), Box<dyn Error + Send + Sync>> {
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
                    let final_output_directory =
                        if image_settings.keep_child_folders_structure_in_output_directory {
                            let relative_image_path =
                                get_relative_path(input_directory, &image.file_path)
                                    .unwrap_or_else(|_| PathBuf::from(""));
                            let relative_dir_path =
                                relative_image_path.parent().unwrap_or(Path::new(""));
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

            process_image_batch(&batch_data, logo).map_err(
                |e| -> Box<dyn Error + Send + Sync> {
                    format!("Failed to process image batch: {}", e).into()
                },
            )?;

            ProgressManager::increment_progress(Some(batch_data.len()));

            Ok(())
        },
    )?;

    Ok(())
}

/// Split batches optimally to utilize all available threads
fn split_batches_optimally(
    batches: HashMap<BatchKey, Vec<Image>>,
    target_threads: usize,
) -> Vec<(BatchKey, Vec<Image>)> {
    let mut work_units = Vec::new();

    // Calculate total images and target images per work unit
    let total_images: usize = batches.values().map(|v| v.len()).sum();
    let target_images_per_unit = total_images.div_ceil(target_threads); // Ceiling division

    // Minimum batch size to avoid too many tiny batches
    let min_batch_size = std::cmp::max(1, target_images_per_unit / 4);

    info!(
        "Target images per work unit: {}, minimum batch size: {}",
        target_images_per_unit, min_batch_size
    );

    for (batch_key, mut images) in batches {
        if images.len() <= target_images_per_unit {
            // Small batch, keep as is
            work_units.push((batch_key, images));
        } else {
            // Large batch, split it
            let num_splits = images.len().div_ceil(target_images_per_unit);
            let actual_split_size = images.len().div_ceil(num_splits);

            info!(
                "Splitting batch of {} images into {} units of ~{} images each",
                images.len(),
                num_splits,
                actual_split_size
            );

            // Sort images by file size within each batch for better load balancing
            images.sort_by(|a, b| b.file_size.cmp(&a.file_size));

            // Split using round-robin to distribute large and small files evenly
            let mut splits: Vec<Vec<Image>> = vec![Vec::new(); num_splits];

            for (index, image) in images.into_iter().enumerate() {
                splits[index % num_splits].push(image);
            }

            // Add non-empty splits to work units
            for split in splits {
                if !split.is_empty() && split.len() >= min_batch_size {
                    work_units.push((batch_key.clone(), split));
                } else if !split.is_empty() {
                    // If split is too small, try to merge with the last work unit of the same type
                    if let Some(last_unit) = work_units
                        .iter_mut()
                        .rev()
                        .find(|(key, _)| key == &batch_key)
                    {
                        last_unit.1.extend(split);
                    } else {
                        // If no existing unit to merge with, create a new one anyway
                        work_units.push((batch_key.clone(), split));
                    }
                }
            }
        }
    }

    // Sort work units by size (largest first) for better scheduling
    // This now happens AFTER all work units are created, ensuring proper ordering
    work_units.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    work_units
}

fn process_logos_for_image_resolutions(
    image_settings: &ImageSettings,
    image_list: &Vec<Image>,
) -> Result<Option<Vec<Logo>>, Box<dyn Error + Send + Sync>> {
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

/// Reads all images in the input directory, and adds them to the image list
fn read_images_in_input_directory(
    image_settings: &ImageSettings,
    input_directory: &Path,
    image_list: &mut Vec<Image>,
    output_directory: &Path,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if image_settings.search_child_folders {
        read_images_recursive_parallel(
            input_directory,
            image_list,
            output_directory,
            image_settings,
        )?;
    } else {
        let dir_read_start = std::time::Instant::now();
        let entries: Result<Vec<_>, _> = read_dir(input_directory)?.collect();
        let entries = entries?;
        info!("Directory read took: {:?}", dir_read_start.elapsed());

        let filter_start = std::time::Instant::now();
        let entry_paths = entries.iter().map(|entry| entry.path());
        let valid_image_paths = filter_valid_image_paths(
            entry_paths,
            input_directory,
            output_directory,
            image_settings,
        );
        info!("Path filtering took: {:?}", filter_start.elapsed());
        info!("Found {} valid image paths", valid_image_paths.len());

        let image_creation_start = std::time::Instant::now();
        let images = create_images_from_paths_parallel(&valid_image_paths);
        info!("Image creation took: {:?}", image_creation_start.elapsed());

        image_list.extend(images);
    }
    Ok(())
}

/// Determine if the image should be written to the output directory.
///
/// This is determined based on if the image already exists in the output directory and if it is allowed to be overwritten
fn write_to_output_directory(
    path: &Path,
    input_directory: &Path,
    output_directory: &Path,
    image_settings: &ImageSettings,
) -> bool {
    if image_settings.overwrite_existing_files_output_directory {
        return true;
    }

    // Get the file stem (filename without extension)
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Get the target extension based on the format setting
    let target_extension = &image_settings.format;

    let target_filename = format!("{}.{}", file_stem, target_extension);

    if image_settings.keep_child_folders_structure_in_output_directory {
        let relative_image_path = get_relative_path(input_directory, path).unwrap();
        let relative_dir_path = relative_image_path.parent().unwrap_or(Path::new(""));
        let target_output_path = output_directory
            .join(relative_dir_path)
            .join(target_filename);
        return !target_output_path.exists();
    }

    let target_output_path = output_directory.join(target_filename);
    !target_output_path.exists()
}

fn is_supported_image_extension(path: &Path) -> bool {
    if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
        matches!(
            extension.to_lowercase().as_str(),
            "png"
                | "jpg"
                | "jpeg"
                | "webp"
                | "bmp"
                | "gif"
                | "tiff"
                | "ico"
                | "pnm"
                | "tga"
                | "hdr"
                | "exr"
                | "ff"
                | "avif"
                | "qoi"
        )
    } else {
        false
    }
}

/// Recursively read all images from child directories in parallel
fn read_images_recursive_parallel(
    directory: &Path,
    image_list: &mut Vec<Image>,
    output_directory: &Path,
    image_settings: &ImageSettings,
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

    let valid_image_paths = filter_valid_image_paths(
        walkdir_paths,
        directory, // Use directory as input_directory for recursive case
        output_directory,
        image_settings,
    );

    info!("Found {} image files to process", valid_image_paths.len());

    let images = create_images_from_paths_parallel(&valid_image_paths);
    image_list.extend(images);

    Ok(())
}

/// Sorts the image list by file size in descending order (largest to smallest)
fn sort_list_by_file_size(image_list: &mut [Image]) {
    image_list.sort_by(|a, b| b.file_size.cmp(&a.file_size));
}

/// Filters paths to only include valid image files that should be processed
fn filter_valid_image_paths(
    paths: impl Iterator<Item = PathBuf>,
    input_directory: &Path,
    output_directory: &Path,
    image_settings: &ImageSettings,
) -> Vec<PathBuf> {
    paths
        .filter(|path| {
            path.is_file()
                && is_supported_image_extension(path)
                && write_to_output_directory(
                    path,
                    input_directory,
                    output_directory,
                    image_settings,
                )
        })
        .collect()
}

/// Creates Image objects from paths in parallel, filtering out failed creations
fn create_images_from_paths_parallel(paths: &[PathBuf]) -> Vec<Image> {
    paths
        .par_iter()
        .filter_map(|path| match Image::new(path.clone()) {
            Ok(image) => Some(image),
            Err(e) => {
                error!("Failed to load image {}: {}", path.display(), e);
                None
            }
        })
        .collect()
}
