use crate::media::image::ffmpeg_logger;
use crate::media::{Logo, Video};
use ffmpeg_sidecar::command::FfmpegCommand;
use std::error::Error;
use std::path::Path;

pub fn process_video(
    video: &Video,
    logo: Option<&Logo>,
    output_directory: &Path,
) -> Result<(), Box<dyn Error>> {
    // Start building the ffmpeg command
    let mut cmd = FfmpegCommand::new();

    // Input video file
    cmd.input(video.file_path.to_str().ok_or("Invalid video file path")?);

    // Add logo input if provided
    if logo.is_some() {
        cmd.input(
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
        cmd.args(["-filter_complex", &filter_complex]);
        cmd.args(["-map", "[final]"]);
    } else {
        // Just scale the video if no logo
        let filter_complex = format!(
            "[0:v]scale={}:{}[final]",
            video.resolution.width, video.resolution.height
        );
        cmd.args(["-filter_complex", &filter_complex]);
        cmd.args(["-map", "[final]"]);
    }

    // Copy audio stream
    cmd.args(["-map", "0:a"]);

    // Set codec
    cmd.args(["-c:v", &video.codec]);
    cmd.args(["-c:a", "copy"]); // Copy audio without re-encoding

    // Quality settings
    cmd.args(["-crf", "23"]); // Good quality/size balance
    cmd.args(["-preset", "medium"]); // Encoding speed vs compression

    // Set output file
    cmd.output(output_file.to_str().ok_or("Invalid output file path")?);

    // Overwrite output file if it exists
    cmd.overwrite();

    // Execute the command
    let ffmpeg_child = cmd.spawn()?;
    ffmpeg_logger(ffmpeg_child)?;

    Ok(())
}
