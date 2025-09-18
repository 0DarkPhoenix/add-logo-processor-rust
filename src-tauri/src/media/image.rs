use crate::{
    formats::image_format_types::{image_format, IMAGE_FORMAT_REGISTRY},
    utils::{read_file_size, read_file_type},
};
use ffmpeg_sidecar::{child::FfmpegChild, command::FfmpegCommand, event::FfmpegEvent};
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
    pub fn new(file_path: PathBuf) -> Result<Self, Box<dyn Error>> {
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
fn read_image_file_type(file_path: &Path) -> Result<String, Box<dyn Error>> {
    let file_type = read_file_type(file_path);

    if IMAGE_FORMAT_REGISTRY.is_supported_for_reading(file_type.as_str()) {
        Ok(file_type)
    } else {
        Err(format!("Unsupported image format for reading: {}", file_type).into())
    }
}

pub fn read_image_resolution(path: &Path) -> Result<Resolution, Box<dyn Error>> {
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
            cmd.args([
                "-pix_fmt", "yuv420p", // Standard format, faster than yuvj420p
                "-q:v", "3", // High enough quality while maintaining performance
                "-huffman", "0", // Use default huffman tables (faster than optimal (1))
            ]);
        }
        name if image_format::WEBP.extensions.contains(&name) => {
            cmd.args([
                "-quality", "75", // Slightly lower quality for better performance
                "-pix_fmt", "yuva420p", "-preset", "default", "-method",
                "2", // Compression method (0-6, 4 is good balance)
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
                "deflate", // Good compression
                "-pred",
                "0", // Horizontal prediction for better compression
            ]);
        }
        _ => {}
    }
}

/// Handle resizing an image to ICO format with FFmpeg
fn handle_resize_to_ico_format(
    input_path: &Path,
    output_path: &Path,
    resolution: &Resolution,
) -> Result<(), Box<dyn Error>> {
    // ICO format maximum size is 256x256, preserve aspect ratio
    let max_dimension = resolution.width.max(resolution.height);

    let (width, height) = if max_dimension > 256 {
        // Scale down proportionally to fit within 256x256
        let scale_factor = 256.0 / max_dimension as f32;
        let width = (resolution.width as f32 * scale_factor) as u32;
        let height = (resolution.height as f32 * scale_factor) as u32;
        (width, height)
    } else {
        (resolution.width, resolution.height)
    };

    let ffmpeg_child = FfmpegCommand::new()
        .args([
            "-y", // Overwrite output file
            "-i",
            input_path.to_str().ok_or("Invalid input path")?,
            "-vf",
            &format!("scale={}:{}", width, height),
            "-pix_fmt",
            "bgra", // ICO format typically uses BGRA
            "-f",
            image_format::ICO.name,
        ])
        .output(output_path.to_str().ok_or("Invalid output path")?)
        .spawn()?;

    ffmpeg_logger(ffmpeg_child)?;

    Ok(())
}

pub fn ffmpeg_logger(mut ffmpeg_child: FfmpegChild) -> Result<(), Box<dyn Error>> {
    let mut error_messages = Vec::new();
    let mut log_messages = Vec::new();
    ffmpeg_child.iter()?.for_each(|event| {
        match event {
            FfmpegEvent::Log(log_level, message) => {
                log_messages.push(format!("{:?}: {}", log_level, message));
                // Collect error and warning messages
                match log_level {
                    ffmpeg_sidecar::event::LogLevel::Error
                    | ffmpeg_sidecar::event::LogLevel::Fatal => {
                        error_messages.push(message);
                    }
                    _ => {}
                }
            }
            FfmpegEvent::Error(error) => {
                error_messages.push(error);
            }
            FfmpegEvent::Done => {
                println!("FFmpeg image processing completed successfully");
            }
            _ => {}
        }
    });
    let output = ffmpeg_child.wait()?;
    let _: () = if !output.success() {
        let error_message = if !error_messages.is_empty() {
            format!(
                "FFmpeg command failed with exit code: {:?}\n\nErrors:\n{}",
                output.code(),
                error_messages.join("\n")
            )
        } else if !log_messages.is_empty() {
            format!(
                "FFmpeg command failed with exit code: {:?}\n\nLogs:\n{}",
                output.code(),
                log_messages.join("\n")
            )
        } else {
            format!("FFmpeg command failed with exit code: {:?}", output.code())
        };

        return Err(error_message.into());
    };
    Ok(())
}
