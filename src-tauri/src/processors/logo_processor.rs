use std::{error::Error, path::Path};

use ffmpeg_sidecar::command::FfmpegCommand;

use crate::media::{
    image::{apply_image_format_specific_args, ffmpeg_logger, read_image_resolution},
    Logo, Resolution,
};

pub fn process_logo(logo: &mut Logo, output_directory: &Path) -> Result<(), Box<dyn Error>> {
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

    // Resize logo using FFmpeg
    resize_logo(&logo.file_path, &output_path, &logo.resolution)?;

    // Overwrite the original logo path with the resized one to be used by images and videos in their processes
    logo.file_path = output_path;

    Ok(())
}

fn resize_logo(
    input_path: &std::path::PathBuf,
    output_path: &std::path::PathBuf,
    resolution: &Resolution,
) -> Result<(), Box<dyn Error>> {
    // Check if resizing is needed
    let current_resolution = read_image_resolution(input_path)?;
    if current_resolution.width == resolution.width
        && current_resolution.height == resolution.height
    {
        // No resizing needed, just copy the file
        std::fs::copy(input_path, output_path)?;
        return Ok(());
    }

    // Get file extension to determine format-specific settings
    let file_extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");

    let mut ffmpeg_command = FfmpegCommand::new();
    ffmpeg_command.args([
        "-y", // Overwrite output file
        "-i",
        input_path.to_str().ok_or("Invalid input path")?,
        "-vf",
        &format!("scale={}:{}", resolution.width, resolution.height),
        "-q:v",
        "2", // High quality
    ]);

    apply_image_format_specific_args(file_extension, &mut ffmpeg_command);

    let ffmpeg_child = ffmpeg_command
        .output(output_path.to_str().ok_or("Invalid output path")?)
        .spawn()?;

    ffmpeg_logger(ffmpeg_child)?;

    Ok(())
}
