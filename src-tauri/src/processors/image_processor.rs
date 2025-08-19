use crate::media::Image;
use crate::utils::load_image;
use crate::{
    media::{image::get_image_format_string, Logo},
    utils::resize_image,
};
use image::ImageFormat;
use image::{DynamicImage, ImageReader};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use std::{error::Error, path::Path};

pub fn process_image(
    image: &Image,
    logo: Option<&Logo>,
    output_directory: &Path,
) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let load_start = Instant::now();
    let img = load_image(&image.file_path)?;
    let load_duration = load_start.elapsed();
    println!("Image load time: {:?}", load_duration);

    let resize_start = Instant::now();
    let mut resized_img = resize_image(img, image)?;
    let resize_duration = resize_start.elapsed();
    println!("Image resize time: {:?}", resize_duration);

    let conversion_start = Instant::now();
    check_for_rgb_conversion(&mut resized_img, &image.file_type);
    let conversion_duration = conversion_start.elapsed();
    println!("Color conversion time: {:?}", conversion_duration);

    let logo_start = Instant::now();
    if let Some(logo) = logo {
        apply_logo_to_image(&mut resized_img, logo)?
    }
    let logo_duration = logo_start.elapsed();
    if logo.is_some() {
        println!("Logo application time: {:?}", logo_duration);
    }

    let save_start = Instant::now();
    let file_stem = image
        .file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid file name")?;

    let new_extension = get_image_format_string(&image.file_type);
    let new_filename = format!("{}.{}", file_stem, new_extension);
    let output_path = output_directory.join(new_filename);

    let file = File::create(&output_path)?;
    let mut writer = BufWriter::with_capacity(8192 * 128, file); // 1MB buffer
    resized_img.write_to(&mut writer, image.file_type)?;
    writer.flush()?;
    let save_duration = save_start.elapsed();
    println!("Image save time: {:?}", save_duration);

    let total_duration = start_time.elapsed();
    println!(
        "Total image processing time for '{}': {:?}",
        image
            .file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy(),
        total_duration
    );

    Ok(())
}

fn apply_logo_to_image(img: &mut DynamicImage, logo: &Logo) -> Result<(), Box<dyn Error>> {
    let logo_load_start = Instant::now();
    let logo_img = ImageReader::open(&logo.file_path)?.decode()?;
    let logo_load_duration = logo_load_start.elapsed();
    println!("  Logo load time: {:?}", logo_load_duration);

    let overlay_start = Instant::now();
    image::imageops::overlay(
        img,
        &logo_img,
        logo.position.x as i64,
        logo.position.y as i64,
    );
    let overlay_duration = overlay_start.elapsed();
    println!("  Logo overlay time: {:?}", overlay_duration);

    Ok(())
}

/// Check if the image needs to be converted to the correct color format for the target image format
pub fn check_for_rgb_conversion(img: &mut DynamicImage, format: &ImageFormat) {
    match format {
        // Formats that require RGB8 (no alpha channel)
        ImageFormat::Jpeg | ImageFormat::Bmp => {
            *img = DynamicImage::ImageRgb8(img.to_rgb8());
        }
        // Formats that require RGB32F (floating point)
        ImageFormat::Hdr | ImageFormat::OpenExr => {
            *img = DynamicImage::ImageRgb32F(img.to_rgb32f());
        }
        // Formats that require RGBA8 (with alpha channel)
        ImageFormat::Ico => {
            *img = DynamicImage::ImageRgba8(img.to_rgba8());
        }
        // Farbfeld requires RGBA16
        ImageFormat::Farbfeld => {
            *img = DynamicImage::ImageRgba16(img.to_rgba16());
        }
        // Formats that work with various color types but benefit from RGBA8
        ImageFormat::Png | ImageFormat::WebP | ImageFormat::Tiff | ImageFormat::Avif => {
            // These formats support alpha, so convert to RGBA8 if not already
            if !matches!(
                img,
                DynamicImage::ImageRgba8(_)
                    | DynamicImage::ImageRgba16(_)
                    | DynamicImage::ImageRgba32F(_)
            ) {
                *img = DynamicImage::ImageRgb8(img.to_rgb8());
            }
        }
        // Formats that are flexible with color types - no conversion needed
        ImageFormat::Gif
        | ImageFormat::Pnm
        | ImageFormat::Tga
        | ImageFormat::Dds
        | ImageFormat::Qoi => {
            // These formats can handle various color types, no conversion needed
        }
        // Default case for any other formats
        _ => {
            // Convert to RGB8 as a safe default
            *img = DynamicImage::ImageRgb8(img.to_rgb8());
        }
    }
}
