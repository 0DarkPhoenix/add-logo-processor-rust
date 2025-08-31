use crate::media::image::{apply_image_format_specific_args, ffmpeg_logger};
use crate::media::{Image, Logo};
use ffmpeg_sidecar::command::FfmpegCommand;
use log::info;
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;

pub fn process_image_batch(
    batch_data: &[(Image, PathBuf)], // (Image, output_directory)
    logo: Option<&Logo>,
) -> Result<(), Box<dyn Error>> {
    if batch_data.is_empty() {
        return Ok(());
    }

    let start_time = Instant::now();

    // All images in a batch should have the same resolution and file type
    let first_image = &batch_data[0].0;
    let target_resolution = &first_image.resolution;
    let target_file_type = &first_image.file_type;

    info!(
        "Processing batch of {} images with resolution {}x{} and format {}",
        batch_data.len(),
        target_resolution.width,
        target_resolution.height,
        target_file_type
    );

    // Create output directories for all images first
    for (_, output_directory) in batch_data {
        std::fs::create_dir_all(output_directory)?;
    }

    // Process each image individually to avoid complex filter issues
    for (i, (image, output_directory)) in batch_data.iter().enumerate() {
        let file_stem = image
            .file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid file name")?;

        let new_filename = format!("{}.{}", file_stem, target_file_type);
        let output_file = output_directory.join(new_filename);

        // Start building the ffmpeg command for this individual image
        let mut cmd = FfmpegCommand::new();

        // Hide command window on Windows
        #[cfg(target_os = "windows")]
        cmd.hide_banner();

        // Add optimization flags first
        cmd.args([
            "-y", // Overwrite output file
            "-threads",
            "0",   // Use all available CPU cores
            "-an", // No audio processing
            "-vsync",
            "0", // Disable video sync (not needed for single images)
            "-frame_drop_threshold",
            "0", // Don't drop frames
        ]);

        // Add input image
        cmd.input(image.file_path.to_str().ok_or("Invalid image file path")?);

        if let Some(logo) = logo {
            // Add logo input image
            cmd.input(logo.file_path.to_str().ok_or("Invalid logo file path")?);

            // Scale image and overlay logo with optimized scaling
            let filter = format!(
                "[0:v]scale={}:{}:flags=fast_bilinear[scaled];[scaled][1:v]overlay={}:{}",
                target_resolution.width, target_resolution.height, logo.position.x, logo.position.y
            );
            cmd.args(["-filter_complex", &filter]);
        } else {
            // Just scale the image if no logo with optimized scaling
            let filter = format!(
                "scale={}:{}:flags=fast_bilinear",
                target_resolution.width, target_resolution.height
            );
            cmd.args(["-vf", &filter]);
        }

        // Set output format and quality settings for the target file type
        apply_image_format_specific_args(target_file_type, &mut cmd);

        // Set output file
        cmd.output(output_file.to_str().ok_or("Invalid output file path")?);

        // Execute the command
        let ffmpeg_child = cmd.spawn()?;

        ffmpeg_logger(ffmpeg_child)?;
    }
    let total_duration = start_time.elapsed();
    info!(
        "Total batch processing time for {} images: {:?}",
        batch_data.len(),
        total_duration
    );

    Ok(())
}
