use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::utils::{read_file_size, read_file_type};

use super::media::Media;
use super::types::Resolution;
use fast_image_resize::{self as fr, FilterType, ResizeAlg};
use image::{DynamicImage, ImageFormat, ImageReader, RgbaImage};
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

pub fn load_image(path: &PathBuf) -> Result<DynamicImage, Box<dyn Error>> {
    let img = image::open(path)?;
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

    let original_width = img.width();
    let original_height = img.height();
    let target_width = resolution.width;
    let target_height = resolution.height;

    // If no resizing is needed, return the original image
    if original_width == target_width && original_height == target_height {
        return Ok(img);
    }

    // Convert DynamicImage to RGBA8 for fast_image_resize
    let rgba_img = img.to_rgba8();

    // Create source image for fast_image_resize
    let src_image = fr::images::Image::from_vec_u8(
        original_width,
        original_height,
        rgba_img.into_raw(),
        fr::PixelType::U8x4,
    )?;

    // Create destination image
    let mut dst_image = fr::images::Image::new(target_width, target_height, fr::PixelType::U8x4);

    // Create resizer with high-quality algorithm
    let mut resizer = fr::Resizer::new();

    // Perform the resize with CatmullRom filter
    let resize_options =
        fr::ResizeOptions::new().resize_alg(ResizeAlg::Interpolation(FilterType::CatmullRom));

    resizer.resize(&src_image, &mut dst_image, &resize_options)?;

    // Convert back to DynamicImage
    let resized_rgba = RgbaImage::from_raw(target_width, target_height, dst_image.into_vec())
        .ok_or("Failed to create RgbaImage from resized data")?;

    Ok(DynamicImage::ImageRgba8(resized_rgba))
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

            // Use fast_image_resize for ICO format as well for consistency
            let rgba_img = img.to_rgba8();

            if let Ok(src_image) = fr::images::Image::from_vec_u8(
                img.width(),
                img.height(),
                rgba_img.into_raw(),
                fr::PixelType::U8x4,
            ) {
                let mut dst_image = fr::images::Image::new(width, height, fr::PixelType::U8x4);
                let mut resizer = fr::Resizer::new();

                // Perform the resize with CatmullRom filter
                let resize_options = fr::ResizeOptions::new()
                    .resize_alg(ResizeAlg::Interpolation(fr::FilterType::CatmullRom));

                if resizer
                    .resize(&src_image, &mut dst_image, &resize_options)
                    .is_ok()
                {
                    if let Some(resized_rgba) =
                        RgbaImage::from_raw(width, height, dst_image.into_vec())
                    {
                        return Some(Ok(DynamicImage::ImageRgba8(resized_rgba)));
                    }
                }
            }

            // Fallback to original method if fast_image_resize fails
            let resized = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
            return Some(Ok(resized));
        } else {
            // Image is already within limits, use original dimensions
            let rgba_img = img.to_rgba8();

            if let Ok(src_image) = fr::images::Image::from_vec_u8(
                img.width(),
                img.height(),
                rgba_img.into_raw(),
                fr::PixelType::U8x4,
            ) {
                let mut dst_image = fr::images::Image::new(
                    resolution.width,
                    resolution.height,
                    fr::PixelType::U8x4,
                );
                let mut resizer = fr::Resizer::new();

                // Perform the resize with CatmullRom filter
                let resize_options = fr::ResizeOptions::new()
                    .resize_alg(ResizeAlg::Interpolation(fr::FilterType::CatmullRom));

                if resizer
                    .resize(&src_image, &mut dst_image, &resize_options)
                    .is_ok()
                {
                    if let Some(resized_rgba) = RgbaImage::from_raw(
                        resolution.width,
                        resolution.height,
                        dst_image.into_vec(),
                    ) {
                        return Some(Ok(DynamicImage::ImageRgba8(resized_rgba)));
                    }
                }
            }

            // Fallback to original method if fast_image_resize fails
            let resized = img.resize_exact(
                resolution.width,
                resolution.height,
                image::imageops::FilterType::Lanczos3,
            );
            return Some(Ok(resized));
        }
    }
    None
}
