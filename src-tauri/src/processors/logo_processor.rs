use std::error::Error;

use image::{DynamicImage, ImageFormat};

use crate::media::{image::load_image, Logo, Resolution};

pub fn process_logo(logo: &mut Logo) -> Result<(), Box<dyn Error>> {
    let logo_img = load_image(&logo.file_path)?;

    let resized_logo_img = resize_logo(logo_img, &logo.resolution)?;

    // Create a fixed folder structure in the application root
    let app_root = std::env::current_exe()?
        .parent()
        .ok_or("Failed to get application directory")?
        .to_path_buf();

    let output_directory = app_root.join("temp_processed_images");

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&output_directory)?;

    let file_stem = logo.file_path.file_stem().unwrap().to_str().unwrap();
    let file_extension = logo.file_path.extension().unwrap().to_str().unwrap();
    let new_filename = format!(
        "{}_{}_{}x{}.{}",
        file_stem,
        "logo",
        logo.compatible_image_resolution.width,
        logo.compatible_image_resolution.height,
        file_extension
    );

    let output_path = output_directory.join(new_filename);
    resized_logo_img.save_with_format(
        &output_path,
        ImageFormat::from_extension(file_extension)
            .ok_or_else(|| format!("Unsupported image format for logo: {}", file_extension))?,
    )?;

    // Overwrite the original logo path with the resized one to be used by images and videos in their processes
    logo.file_path = output_path;

    Ok(())
}

fn resize_logo(
    logo_img: DynamicImage,
    resolution: &Resolution,
) -> Result<DynamicImage, Box<dyn Error>> {
    let resized_logo_img = logo_img.resize(
        resolution.width,
        resolution.height,
        image::imageops::FilterType::Lanczos3,
    );
    Ok(resized_logo_img)
}
