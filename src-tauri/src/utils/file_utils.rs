use std::{
    error::Error,
    fs::metadata,
    path::{Path, PathBuf},
};

use image::{DynamicImage, ImageFormat, ImageReader};

use crate::media::{Image, Resolution};

pub fn read_file_size(file_path: &PathBuf) -> Result<u64, Box<dyn Error>> {
    let metadata = metadata(file_path)?;
    Ok(metadata.len())
}

pub fn read_file_type(file_path: &Path) -> String {
    file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_lowercase()
}

pub fn load_image(path: &PathBuf) -> Result<DynamicImage, Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}

pub fn read_image_resolution(path: &PathBuf) -> Result<Resolution, Box<dyn Error>> {
    let reader = ImageReader::open(path)?;
    let dimensions = reader.into_dimensions()?;
    let resolution = Resolution {
        width: dimensions.0,
        height: dimensions.1,
    };
    Ok(resolution)
}

pub fn resize_image(img: DynamicImage, image: &Image) -> Result<DynamicImage, Box<dyn Error>> {
    let resolution = &image.resolution;

    if let Some(ico_image) = handle_resize_to_ico_format(&img, image, resolution) {
        return ico_image;
    }

    // Two-stage resize for better quality
    let original_width = img.width();
    let original_height = img.height();
    let target_width = resolution.width;
    let target_height = resolution.height;

    // Calculate if we're doing a significant downscale
    let scale_factor_x = original_width as f32 / target_width as f32;
    let scale_factor_y = original_height as f32 / target_height as f32;
    let max_scale_factor = scale_factor_x.max(scale_factor_y);

    let resized = if max_scale_factor > 1.2 {
        // Stage 1: Quick downscale with Nearest to intermediate size
        let intermediate_scale = 1.2;
        let intermediate_width = (target_width as f32 * intermediate_scale) as u32;
        let intermediate_height = (target_height as f32 * intermediate_scale) as u32;

        let intermediate = img.resize_exact(
            intermediate_width,
            intermediate_height,
            image::imageops::FilterType::Nearest,
        );

        // Stage 2: High-quality resize to final size
        intermediate.resize_exact(
            target_width,
            target_height,
            image::imageops::FilterType::CatmullRom,
        )
    } else {
        // Single-stage resize for smaller scale changes
        img.resize_exact(
            target_width,
            target_height,
            image::imageops::FilterType::CatmullRom,
        )
    };

    Ok(resized)
}

/// Handle resizing an image while it is an ICO format, as this format supports a maximum size of 256x256 pixels.
fn handle_resize_to_ico_format(
    img: &DynamicImage,
    image: &Image,
    resolution: &Resolution,
) -> Option<Result<DynamicImage, Box<dyn Error>>> {
    if image.file_type == ImageFormat::Ico {
        // ICO format maximum size is 256x256, preserve aspect ratio
        let max_dimension = resolution.width.max(resolution.height);

        if max_dimension > 256 {
            // Scale down proportionally to fit within 256x256
            let scale_factor = 256.0 / max_dimension as f32;
            let width = (resolution.width as f32 * scale_factor) as u32;
            let height = (resolution.height as f32 * scale_factor) as u32;

            let resized = img.resize_exact(width, height, image::imageops::FilterType::Nearest);

            return Some(Ok(resized));
        } else {
            // Image is already within limits, use original dimensions
            let resized = img.resize_exact(
                resolution.width,
                resolution.height,
                image::imageops::FilterType::Nearest,
            );

            return Some(Ok(resized));
        }
    }
    None
}
