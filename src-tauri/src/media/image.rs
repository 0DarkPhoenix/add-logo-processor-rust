use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::utils::{read_file_size, read_file_type, read_image_resolution};

use super::media::Media;
use super::types::Resolution;
use image::ImageFormat;
use serde::{Deserialize, Serialize};

pub mod image_format_strings {
    pub const PNG: &str = "png";
    pub const JPEG: &str = "jpeg";
    pub const WEBP: &str = "webp";
    pub const BMP: &str = "bmp";
    pub const GIF: &str = "gif";
    pub const TIFF: &str = "tiff";
    pub const ICO: &str = "ico";
    pub const PNM: &str = "pnm";
    pub const TGA: &str = "tga";
    pub const HDR: &str = "hdr";
    pub const EXR: &str = "exr";
    pub const FF: &str = "ff";
    pub const AVIF: &str = "avif";
    pub const QOI: &str = "qoi";
}

pub fn get_image_format_string(format: &ImageFormat) -> &'static str {
    (match format {
        ImageFormat::Png => image_format_strings::PNG,
        ImageFormat::Jpeg => image_format_strings::JPEG,
        ImageFormat::WebP => image_format_strings::WEBP,
        ImageFormat::Bmp => image_format_strings::BMP,
        ImageFormat::Gif => image_format_strings::GIF,
        ImageFormat::Tiff => image_format_strings::TIFF,
        ImageFormat::Ico => image_format_strings::ICO,
        ImageFormat::Pnm => image_format_strings::PNM,
        ImageFormat::Tga => image_format_strings::TGA,
        ImageFormat::Hdr => image_format_strings::HDR,
        ImageFormat::OpenExr => image_format_strings::EXR,
        ImageFormat::Farbfeld => image_format_strings::FF,
        ImageFormat::Avif => image_format_strings::AVIF,
        ImageFormat::Qoi => image_format_strings::QOI,
        _ => image_format_strings::PNG, // default fallback
    }) as _
}

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
fn read_image_file_type(file_path: &Path) -> Result<ImageFormat, Box<dyn Error>> {
    let file_type = read_file_type(file_path);

    let format = ImageFormat::from_extension(&file_type)
        .ok_or_else(|| format!("Unsupported image format: {}", file_type))?;
    Ok(format)
}
