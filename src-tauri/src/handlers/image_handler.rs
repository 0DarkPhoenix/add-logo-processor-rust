use rayon::prelude::*;
use std::{error::Error, fs::read_dir, path::Path};

use crate::{
    handlers::handle_logos,
    media::{Image, Logo, Media, Resolution},
    processors::process_image,
    utils::{config::ImageSettings, AppConfig},
};

pub fn handle_images(config: &AppConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
    let image_settings = &config.image_settings;

    let input_directory = &image_settings.input_directory;
    let output_directory = &image_settings.output_directory;

    let mut image_list = Vec::new();

    read_images_in_input_directory(image_settings, input_directory, &mut image_list)?;

    sort_list_by_file_size(&mut image_list);

    determ_resized_resolution_per_image(image_settings, &mut image_list);

    let logo_list = process_logos_for_image_resolutions(image_settings, &image_list)?;

    process_images_from_image_list(output_directory, image_list, logo_list)?;

    Ok(())
}

/// Determine the new resized dimensions for each image in the image list
fn determ_resized_resolution_per_image(
    image_settings: &ImageSettings,
    image_list: &mut Vec<Image>,
) {
    for image in image_list {
        image.resize_dimensions(image_settings.min_pixel_count);
    }
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
    let _: () = if image_settings.search_child_folders {
        read_images_recursive(input_directory, image_list)?;
    } else {
        // Non-recursive search (only search images in current directory)
        for entry in read_dir(input_directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Ok(image) = Image::new(path) {
                    image_list.push(image);
                }
            }
        }
    };
    Ok(())
}

/// Recursively read all images from child directories
fn read_images_recursive(
    dir: &Path,
    images: &mut Vec<Image>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively search subdirectories
            read_images_recursive(&path, images)?;
        } else if path.is_file() {
            if let Ok(image) = Image::new(path) {
                images.push(image);
            }
        }
    }
    Ok(())
}

/// Sorts the image list by file size in descending order (largest to smallest)
fn sort_list_by_file_size(image_list: &mut [Image]) {
    image_list.sort_by(|a, b| b.file_size.cmp(&a.file_size));
}
