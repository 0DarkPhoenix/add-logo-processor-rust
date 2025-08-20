use rayon::prelude::*;
use std::path::PathBuf;
use std::{error::Error, fs::read_dir, path::Path};

use crate::media::image::get_image_format_string;
use crate::utils::{clear_and_create_folder, get_relative_path};
use crate::{
    handlers::handle_logos,
    media::{Image, Logo, Media, Resolution},
    processors::process_image,
    utils::config::ImageSettings,
};

pub fn handle_images(image_settings: &ImageSettings) -> Result<(), Box<dyn Error + Send + Sync>> {
    let input_directory = &image_settings.input_directory;
    let output_directory = &image_settings.output_directory;

    let mut image_list = Vec::new();

    let start_time = std::time::Instant::now();

    if image_settings.clear_files_output_directory || !output_directory.exists() {
        let clear_folder_time = std::time::Instant::now();
        clear_and_create_folder(output_directory).unwrap();
        println!(
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
    println!("Reading images took: {:?}", read_images_time.elapsed());

    let sort_start = std::time::Instant::now();
    sort_list_by_file_size(&mut image_list);
    println!(
        "Sorting images by file size took: {:?}",
        sort_start.elapsed()
    );

    let apply_settings_start = std::time::Instant::now();
    apply_image_settings_per_image(image_settings, &mut image_list);
    println!(
        "Applying image settings took: {:?}",
        apply_settings_start.elapsed()
    );

    let logo_processing_start = std::time::Instant::now();
    let logo_list = process_logos_for_image_resolutions(image_settings, &image_list)?;
    println!(
        "Processing logos took: {:?}",
        logo_processing_start.elapsed()
    );

    let image_processing_start = std::time::Instant::now();
    process_images_from_image_list(
        output_directory,
        image_list,
        logo_list,
        image_settings,
        input_directory,
    )?;
    println!(
        "Processing images took: {:?}",
        image_processing_start.elapsed()
    );

    println!("Total time: {:?}", start_time.elapsed());

    Ok(())
}

/// Apply the image settings per image in parallel
fn apply_image_settings_per_image(image_settings: &ImageSettings, image_list: &mut Vec<Image>) {
    image_list.par_iter_mut().for_each(|image| {
        image.resize_dimensions(image_settings.min_pixel_count);
        image.file_type = image_settings.format;
    });
}

/// Process the images from the image list in parallel
fn process_images_from_image_list(
    output_directory: &Path,
    image_list: Vec<Image>,
    logo_list: Option<Vec<Logo>>,
    image_settings: &ImageSettings,
    input_directory: &Path,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    image_list
        .par_iter()
        .try_for_each(|image| -> Result<(), Box<dyn Error + Send + Sync>> {
            let logo: Option<&Logo> = if let Some(ref logo_list) = logo_list {
                logo_list
                    .iter()
                    .find(|logo| logo.compatible_image_resolution == image.resolution)
            } else {
                None
            };

            if logo.is_none() && logo_list.is_some() {
                return Err(format!(
                    "No logo found for the given image resolution: {}",
                    image.resolution
                )
                .into());
            }

            let final_output_directory =
                if image_settings.keep_child_folders_structure_in_output_directory {
                    let relative_image_path = get_relative_path(input_directory, &image.file_path)
                        .map_err(|e| -> Box<dyn Error + Send + Sync> {
                            format!("Failed to get relative path: {}", e).into()
                        })?;
                    let relative_dir_path = relative_image_path.parent().unwrap_or(Path::new(""));
                    output_directory.join(relative_dir_path)
                } else {
                    output_directory.to_path_buf()
                };

            process_image(image, logo, &final_output_directory).map_err(
                |e| -> Box<dyn Error + Send + Sync> {
                    format!("Failed to process image: {}", e).into()
                },
            )
        })?;
    Ok(())
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
        println!("Directory read took: {:?}", dir_read_start.elapsed());

        let filter_start = std::time::Instant::now();
        let entry_paths = entries.iter().map(|entry| entry.path());
        let valid_image_paths = filter_valid_image_paths(
            entry_paths,
            input_directory,
            output_directory,
            image_settings,
        );
        println!("Path filtering took: {:?}", filter_start.elapsed());
        println!("Found {} valid image paths", valid_image_paths.len());

        let image_creation_start = std::time::Instant::now();
        let images = create_images_from_paths_parallel(&valid_image_paths);
        println!("Image creation took: {:?}", image_creation_start.elapsed());

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
    dbg!(&path, input_directory, output_directory);
    if image_settings.overwrite_existing_files_output_directory {
        return true;
    }

    // Get the file stem (filename without extension)
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Get the target extension based on the format setting
    let target_extension = get_image_format_string(&image_settings.format);

    let target_filename = format!("{}.{}", file_stem, target_extension);

    if image_settings.keep_child_folders_structure_in_output_directory {
        let relative_image_path = get_relative_path(input_directory, path).unwrap();
        let relative_dir_path = relative_image_path.parent().unwrap_or(Path::new(""));
        let target_output_path = output_directory
            .join(relative_dir_path)
            .join(target_filename);
        dbg!(&target_output_path);
        return !target_output_path.exists();
    }

    let target_output_path = output_directory.join(target_filename);
    !target_output_path.exists()
}

fn write_in_output_directory(
    path: &Path,
    input_directory: &Path,
    output_directory: &Path,
    image_settings: &ImageSettings,
) -> bool {
    dbg!(&path, input_directory, output_directory);
    if image_settings.overwrite_existing_files_output_directory {
        return true;
    }

    if image_settings.keep_child_folders_structure_in_output_directory {
        let relative_image_path = get_relative_path(input_directory, path).unwrap();
        dbg!(&relative_image_path);
        return !output_directory.join(relative_image_path).exists();
    }

    !output_directory.join(path.file_name().unwrap()).exists()
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
    use walkdir::WalkDir;

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

    println!("Found {} image files to process", valid_image_paths.len());

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
                eprintln!("Failed to load image {}: {}", path.display(), e);
                None
            }
        })
        .collect()
}
