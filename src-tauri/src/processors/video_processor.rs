use crate::media::image::ffmpeg_logger;
use crate::media::{Logo, Video};
use ffmpeg_sidecar::command::FfmpegCommand;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::Path;

pub fn process_video(
    video: &Video,
    logo: Option<&Logo>,
    output_directory: &Path,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Create output directories
    create_dir_all(output_directory)?;

    // Start building the ffmpeg command
    let mut cmd = FfmpegCommand::new();

    #[cfg(target_os = "windows")]
    cmd.hide_banner();

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

    // Add output mappings and files
    let file_stem = video
        .file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid file name")?;

    let target_file_type = &video.file_type;

    let new_filename = format!("{}.{}", file_stem, target_file_type);
    let output_file = output_directory.join(new_filename);

    cmd.output(output_file.to_str().ok_or("Invalid output file path")?);

    // Overwrite output file if it exists
    cmd.overwrite();

    let ffmpeg_child = cmd.spawn()?;

    ffmpeg_logger(ffmpeg_child)
}
