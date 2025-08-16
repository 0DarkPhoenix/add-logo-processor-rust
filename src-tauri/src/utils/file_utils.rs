use std::{
    error::Error,
    fs::metadata,
    path::{Path, PathBuf},
};

use image::{DynamicImage, GenericImageView, ImageReader};

use crate::media::Resolution;

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
    let img = load_image(path)?;
    let (width, height) = img.dimensions();
    let resolution = Resolution { width, height };
    Ok(resolution)
}

pub fn resize_image(
    img: DynamicImage,
    resolution: &Resolution,
) -> Result<DynamicImage, Box<dyn Error>> {
    let resized = img.resize(
        resolution.width,
        resolution.height,
        image::imageops::FilterType::Lanczos3,
    );

    Ok(resized)
}
