use std::{error::Error, path::PathBuf};

use crate::utils::{read_file_size, read_file_type, read_image_resolution};

use super::media::Media;
use super::types::Resolution;
use image::ImageFormat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub file_path: PathBuf,
    pub resolution: Resolution,
    pub file_size: u64,
    pub file_type: ImageFormat,
}

impl Image {
    pub fn new(file_path: PathBuf) -> Result<Self, Box<dyn Error>> {
        // Get file size
        let file_size = read_file_size(&file_path)?;

        // Get file type from extension
        let file_type = read_image_file_type(&file_path)?;

        // Read image dimensions
        let resolution = read_image_resolution(&file_path)?;

        Ok(Self {
            file_path,
            resolution,
            file_size,
            file_type,
        })
    }
}

impl Media for Image {
    type FileType = ImageFormat;

    fn get_resolution(&self) -> &Resolution {
        &self.resolution
    }

    fn get_file_size(&self) -> u64 {
        self.file_size
    }

    fn get_file_type(&self) -> &Self::FileType {
        &self.file_type
    }

    fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution;
    }
}

/// Read the image file type, and maps it to a `ImageFormat` enum used by the `image` crate.
fn read_image_file_type(file_path: &PathBuf) -> Result<ImageFormat, Box<dyn Error>> {
    let file_type = read_file_type(file_path);

    let format = ImageFormat::from_extension(&file_type)
        .ok_or_else(|| format!("Unsupported image format: {}", file_type))?;
    Ok(format)
}
