use crate::media::{Logo, Video};
use ffmpeg_sidecar::{command::FfmpegCommand, event::FfmpegEvent};
use std::error::Error;
use std::path::Path;

pub fn process_video(
    video: &Video,
    logo: Option<&Logo>,
    output_directory: &Path,
) -> Result<(), Box<dyn Error>> {
    // Start building the ffmpeg command
    let mut command = FfmpegCommand::new();

    // Input video file
    command.input(video.file_path.to_str().ok_or("Invalid video file path")?);

    // Add logo input if provided
    if logo.is_some() {
        command.input(
            logo.unwrap()
                .file_path
                .to_str()
                .ok_or("Invalid logo file path")?,
        );
    }

    // Build output file path
    let output_file = output_directory.join(
        video
            .file_path
            .file_name()
            .ok_or("Invalid video file name")?,
    );

    // Build filter complex for video processing

    if let Some(logo) = logo {
        // Scale video and overlay logo in one filter complex
        let filter_complex = format!(
            "[0:v]scale={}:{}[resized];[resized][1:v]overlay={}:{}[final]",
            video.resolution.width, video.resolution.height, logo.position.x, logo.position.y
        );
        command.args(["-filter_complex", &filter_complex]);
        command.args(["-map", "[final]"]);
    } else {
        // Just scale the video if no logo
        let filter_complex = format!(
            "[0:v]scale={}:{}[final]",
            video.resolution.width, video.resolution.height
        );
        command.args(["-filter_complex", &filter_complex]);
        command.args(["-map", "[final]"]);
    }

    // Copy audio stream
    command.args(["-map", "0:a"]);

    // Set codec
    command.args(["-c:v", &video.codec]);
    command.args(["-c:a", "copy"]); // Copy audio without re-encoding

    // Quality settings
    command.args(["-crf", "23"]); // Good quality/size balance
    command.args(["-preset", "medium"]); // Encoding speed vs compression

    // Set output file
    command.output(output_file.to_str().ok_or("Invalid output file path")?);

    // Overwrite output file if it exists
    command.overwrite();

    // Execute the command and collect events
    let mut ffmpeg_child = command.spawn()?;
    let mut error_messages = Vec::new();
    let mut log_messages = Vec::new();

    // Process events from FFmpeg
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
            FfmpegEvent::Progress(progress) => {
                // Optionally log progress
                println!(
                    "Progress: frame={}, fps={}, time={}",
                    progress.frame, progress.fps, progress.time
                );
            }
            FfmpegEvent::Done => {
                println!("FFmpeg processing completed successfully");
            }
            _ => {
                // Handle other events if needed
            }
        }
    });

    // Wait for the process to complete
    let output = ffmpeg_child.wait()?;

    if !output.success() {
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
    }

    Ok(())
}
