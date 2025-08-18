use rayon::prelude::*;
use std::path::PathBuf;
use std::{error::Error, fs::read_dir, path::Path};

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

    read_images_in_input_directory(image_settings, input_directory, &mut image_list)?;
    println!("Reading images took: {:?}", start_time.elapsed());

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
    process_images_from_image_list(output_directory, image_list, logo_list)?;
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

            process_image(image, logo, output_directory).map_err(
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
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if image_settings.search_child_folders {
        read_images_recursive_parallel(input_directory, image_list)?;
    } else {
        let dir_read_start = std::time::Instant::now();
        let entries: Result<Vec<_>, _> = read_dir(input_directory)?.collect();
        let entries = entries?;
        println!("Directory read took: {:?}", dir_read_start.elapsed());

        let filter_start = std::time::Instant::now();
        let valid_image_paths: Vec<PathBuf> = entries
            .iter()
            .filter_map(|entry| {
                let path = entry.path();
                if path.is_file() && is_supported_image_extension(&path) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        println!("Path filtering took: {:?}", filter_start.elapsed());
        println!("Found {} valid image paths", valid_image_paths.len());

        let image_creation_start = std::time::Instant::now();
        let images: Vec<Image> = valid_image_paths
            .par_iter()
            .filter_map(|path| match Image::new(path.clone()) {
                Ok(image) => Some(image),
                Err(e) => {
                    eprintln!("Failed to load image {}: {}", path.display(), e);
                    None
                }
            })
            .collect();
        println!("Image creation took: {:?}", image_creation_start.elapsed());

        image_list.extend(images);
    }
    Ok(())
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
    dir: &Path,
    images: &mut Vec<Image>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut all_paths = Vec::new();
    collect_all_file_paths(dir, &mut all_paths)?;

    let parsed_images: Vec<Image> = all_paths
        .par_iter()
        .filter_map(|path| Image::new(path.clone()).ok())
        .collect();

    images.extend(parsed_images);
    Ok(())
}

fn collect_all_file_paths(
    dir: &Path,
    paths: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_all_file_paths(&path, paths)?;
        } else if path.is_file() {
            paths.push(path);
        }
    }
    Ok(())
}
/// Sorts the image list by file size in descending order (largest to smallest)
fn sort_list_by_file_size(image_list: &mut [Image]) {
    image_list.sort_by(|a, b| b.file_size.cmp(&a.file_size));
}
