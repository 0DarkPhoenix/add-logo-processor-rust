use crate::handlers::progress_handler::ProgressManager;
use crate::media::image::{apply_image_format_specific_args, ffmpeg_logger};
use crate::media::{Image, Logo};
use ffmpeg_sidecar::command::FfmpegCommand;
use std::error::Error;
use std::path::PathBuf;

pub fn process_image_batch(
    batch_data: &[(Image, PathBuf)],
    logo: Option<&Logo>,
) -> Result<(), Box<dyn Error>> {
    if batch_data.is_empty() {
        return Ok(());
    }

    let first_image = &batch_data[0].0;
    let target_resolution = &first_image.resolution;
    let target_file_type = &first_image.file_type;

    // Process in chunks for better load balancing and better progress bar progression
    const CHUNK_SIZE: usize = 15;

    if batch_data.len() <= CHUNK_SIZE {
        process_image_chunk(batch_data, logo, target_resolution, target_file_type)?;
    } else {
        let num_chunks = batch_data.len().div_ceil(CHUNK_SIZE);
        let optimal_chunk_size = batch_data.len().div_ceil(num_chunks);

        for chunk in batch_data.chunks(optimal_chunk_size) {
            process_image_chunk(chunk, logo, target_resolution, target_file_type)?;
        }
    }

    Ok(())
}

fn process_image_chunk(
    batch_data: &[(Image, PathBuf)],
    logo: Option<&Logo>,
    target_resolution: &crate::media::types::Resolution,
    target_file_type: &str,
) -> Result<(), Box<dyn Error>> {
    // Create output directories
    for (_, output_directory) in batch_data {
        std::fs::create_dir_all(output_directory)?;
    }

    // Build FFmpeg command for this chunk
    let mut cmd = FfmpegCommand::new();

    #[cfg(target_os = "windows")]
    cmd.hide_banner();

    cmd.args(["-y", "-an", "-vsync", "0"]);

    // Add all input images in this chunk
    for (image, _) in batch_data.iter() {
        cmd.input(image.file_path.to_str().ok_or("Invalid image file path")?);
    }

    // Add logo input if present
    if let Some(logo_ref) = logo {
        cmd.input(
            logo_ref
                .file_path
                .to_str()
                .ok_or("Invalid logo file path")?,
        );
    }

    // Build complex filter for this chunk
    let mut filter_parts = Vec::new();

    for (i, _) in batch_data.iter().enumerate() {
        if let Some(logo_ref) = logo {
            // Scale and overlay logo for each image
            let logo_idx = batch_data.len(); // Logo is the last input
            filter_parts.push(format!(
                "[{}:v]scale={}:{}:flags=fast_bilinear[scaled{}];[scaled{}][{}:v]overlay={}:{}[out{}]",
                i, target_resolution.width, target_resolution.height, i,
                i, logo_idx, logo_ref.position.x, logo_ref.position.y, i
            ));
        } else {
            // Just scale each image
            filter_parts.push(format!(
                "[{}:v]scale={}:{}:flags=fast_bilinear[out{}]",
                i, target_resolution.width, target_resolution.height, i
            ));
        }
    }

    let filter_complex = filter_parts.join(";");
    cmd.args(["-filter_complex", &filter_complex]);

    // Add output mappings and files
    for (i, (image, output_directory)) in batch_data.iter().enumerate() {
        let file_stem = image
            .file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid file name")?;

        let new_filename = format!("{}.{}", file_stem, target_file_type);
        let output_file = output_directory.join(new_filename);

        cmd.args(["-map", &format!("[out{}]", i)]);
        apply_image_format_specific_args(target_file_type, &mut cmd);
        cmd.output(output_file.to_str().ok_or("Invalid output file path")?);
    }

    // Execute the command
    let ffmpeg_child = cmd.spawn()?;
    ffmpeg_logger(ffmpeg_child)?;

    // Update progress for all images in this chunk
    for _ in 0..batch_data.len() {
        ProgressManager::increment_progress();
    }

    Ok(())
}
