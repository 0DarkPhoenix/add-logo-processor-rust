use std::{error::Error, path::PathBuf};

use crate::{
    media::{Image, Logo},
    utils::{load_image, resize_image},
};
use image::{DynamicImage, ImageReader};

pub fn process_image(
    image: &Image,
    logo: Option<&Logo>,
    output_directory: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let img = load_image(&image.file_path)?;

    let mut resized_img = resize_image(img, &image.resolution)?;

    if let Some(logo) = logo {
        apply_logo_to_image(&mut resized_img, logo)?
    }

    let output_path = output_directory.join(image.file_path.file_name().unwrap());
    resized_img.save_with_format(&output_path, image.file_type)?;

    Ok(())
}

fn apply_logo_to_image(img: &mut DynamicImage, logo: &Logo) -> Result<(), Box<dyn Error>> {
    let logo_img = ImageReader::open(&logo.file_path)?.decode()?;

    image::imageops::overlay(
        img,
        &logo_img,
        logo.position.x as i64,
        logo.position.y as i64,
    );

    Ok(())
}
