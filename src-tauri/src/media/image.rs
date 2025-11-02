use crate::{
    formats::image_format_types::{image_format, IMAGE_FORMAT_REGISTRY},
    handlers::process_handler::ProcessManager,
    utils::{read_file_size, read_file_type},
};
use ffmpeg_sidecar::{child::FfmpegChild, command::FfmpegCommand};
use log::error;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

use super::media::Media;
use super::types::Resolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub file_path: PathBuf,
    pub resolution: Resolution,
    pub file_size: u64,
    pub file_type: String,
}

impl Image {
    pub fn new(file_path: PathBuf) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Get file size
        let file_size = read_file_size(&file_path)?;

        // Get file type from extension and validate it's supported by FFmpeg
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
    type FileType = String;

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

/// Read the image file type and validate it's supported by FFmpeg
fn read_image_file_type(file_path: &Path) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file_type = read_file_type(file_path);

    if IMAGE_FORMAT_REGISTRY.is_supported_for_reading(file_type.as_str()) {
        Ok(file_type)
    } else {
        Err(format!("Unsupported image format for reading: {}", file_type).into())
    }
}

pub fn read_image_resolution(path: &Path) -> Result<Resolution, Box<dyn Error + Send + Sync>> {
    // Check if the file is an SVG
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    if image_format::SVG.extensions.contains(&extension.as_str()) {
        // SVG files are vector format - use a default resolution
        // FFmpeg will handle the actual rendering at the target size
        return Ok(Resolution {
            width: 1920,
            height: 1080,
        });
    }

    // For non-SVG images, use imagesize
    let dimensions =
        imagesize::size(path).map_err(|e| format!("Failed to read image dimensions: {}", e))?;

    Ok(Resolution {
        width: dimensions.width as u32,
        height: dimensions.height as u32,
    })
}

/// Apply image format specific arguments to the FFmpeg command
pub fn apply_image_format_specific_args(image_format: &str, cmd: &mut FfmpegCommand) {
    // Add general performance improvements
    cmd.args([
        "-preset", "fast", // Faster encoding preset
    ]);

    match image_format {
        name if image_format::PNG.extensions.contains(&name) => {
            cmd.args([
                "-pix_fmt",
                "rgba",
                "-compression_level",
                "1",
                "-pred",
                "sub",
            ]);
        }
        name if image_format::JPEG.extensions.contains(&name) => {
            cmd.args(["-pix_fmt", "yuv420p", "-q:v", "3", "-huffman", "0"]);
        }
        name if image_format::WEBP.extensions.contains(&name) => {
            cmd.args([
                "-quality", "75", "-pix_fmt", "yuva420p", "-preset", "default", "-method", "2",
            ]);
        }
        name if image_format::BMP.extensions.contains(&name) => {
            cmd.args(["-pix_fmt", "bgr24"]);
        }
        name if image_format::GIF.extensions.contains(&name) => {
            cmd.args(["-pix_fmt", "rgb8"]);
        }
        name if image_format::TIFF.extensions.contains(&name) => {
            cmd.args([
                "-pix_fmt",
                "rgba",
                "-compression_algo",
                "deflate",
                "-pred",
                "0",
            ]);
        }
        _ => {}
    }
}

// /// Handle resizing an image to ICO format with FFmpeg
// fn handle_resize_to_ico_format(
//     input_path: &Path,
//     output_path: &Path,
//     resolution: &Resolution,
// ) -> Result<(), Box<dyn Error>> {
//     // ICO format maximum size is 256x256, preserve aspect ratio
//     let max_dimension = resolution.width.max(resolution.height);

//     let (width, height) = if max_dimension > 256 {
//         // Scale down proportionally to fit within 256x256
//         let scale_factor = 256.0 / max_dimension as f32;
//         let width = (resolution.width as f32 * scale_factor) as u32;
//         let height = (resolution.height as f32 * scale_factor) as u32;
//         (width, height)
//     } else {
//         (resolution.width, resolution.height)
//     };

//     let ffmpeg_child = FfmpegCommand::new()
//         .args([
//             "-y", // Overwrite output file
//             "-i",
//             input_path.to_str().ok_or("Invalid input path")?,
//             "-vf",
//             &format!("scale={}:{}", width, height),
//             "-pix_fmt",
//             "bgra", // ICO format typically uses BGRA
//             "-f",
//             image_format::ICO.name,
//         ])
//         .output(output_path.to_str().ok_or("Invalid output path")?)
//         .spawn()?;

//     ffmpeg_logger(ffmpeg_child)
// }

/// Logger that processes FFmpeg events and waits for completion
pub fn ffmpeg_logger(mut ffmpeg_child: FfmpegChild) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Register the ffmpeg process to the process manager
    let pid = ffmpeg_child.as_inner().id();
    let process_id = ProcessManager::register_process_by_pid(pid);

    // Process FFmpeg output without holding any locks
    let result = process_ffmpeg_output(&mut ffmpeg_child);

    // Unregister after completion
    ProcessManager::unregister_process(process_id);

    result
}

/// Process FFmpeg output without any mutex operations
fn process_ffmpeg_output(
    ffmpeg_child: &mut FfmpegChild,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Iterate over FFmpeg output events
    for event in ffmpeg_child.iter()? {
        match event {
            ffmpeg_sidecar::event::FfmpegEvent::Log(level, msg) => {
                match level {
                    ffmpeg_sidecar::event::LogLevel::Error
                    | ffmpeg_sidecar::event::LogLevel::Fatal => {
                        error!("FFmpeg: {}", msg);
                    }
                    _ => {
                        // Only log warnings and above to reduce overhead
                        if matches!(level, ffmpeg_sidecar::event::LogLevel::Warning) {
                            // info!("FFmpeg: {}", msg);
                        }
                    }
                }
            }
            ffmpeg_sidecar::event::FfmpegEvent::Progress(progress) => {
                // Optionally log progress at intervals
                // Consider removing this entirely for maximum performance
                // dbg!(progress);
            }
            ffmpeg_sidecar::event::FfmpegEvent::Done => {
                break;
            }
            _ => {}
        }
    }

    // Wait for the process to complete
    let output = ffmpeg_child.wait()?;

    if !output.success() {
        return Err(format!("FFmpeg process failed with exit code: {:?}", output.code()).into());
    }

    Ok(())
}
