use crate::media::{Logo, Video};
use ffmpeg_sidecar::{command::FfmpegCommand, download::auto_download};
use std::error::Error;
use std::path::PathBuf;

pub fn process_video(
    video: &Video,
    logo: Option<&Logo>,
    output_directory: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    // Ensure ffmpeg is available
    auto_download()?;

    // Start building the ffmpeg command
    let mut command = FfmpegCommand::new();

    // Input video file
    command.input(&video.file_path.to_str().ok_or("Invalid video file path")?);

    // Build filter complex for video processing
    let mut filter_parts = Vec::new();
    let mut current_stream = "[0:v]".to_string();

    filter_parts.push(format!(
        "{}scale={}:{}[resized]",
        current_stream, video.resolution.width, video.resolution.height
    ));
    current_stream = "[resized]".to_string();

    // Apply logo using overlay filter if selected
    if let Some(logo) = logo {
        let logo_input = "[1:v]".to_string();
        let output_label = "[final]".to_string();

        let overlay_filter = format!(
            "{}{}overlay={}:{}{}",
            current_stream, logo_input, logo.position.x, logo.position.y, output_label
        );

        filter_parts.push(overlay_filter);
        current_stream = output_label;
    }

    // Apply filter complex
    let filter_complex = filter_parts.join(";");
    command.args(["-filter_complex", &filter_complex]);
    command.args(["-map", &current_stream.trim_matches(['[', ']'])]);
    command.args(["-map", "0:a"]); // Copy audio stream

    // Set codec
    command.args(["-c:v", &video.codec]);

    // Set output format
    command.args(["-f", &video.file_type]);

    // Quality settings
    command.args(["-crf", "23"]); // Good quality/size balance
    command.args(["-preset", "medium"]); // Encoding speed vs compression

    // Output file
    command.output(&output_directory.to_str().ok_or("Invalid output path")?);

    // Overwrite output file if it exists
    command.overwrite();

    // Execute the command
    let mut child = command.spawn()?;
    let output = child.wait()?;

    if !output.success() {
        return Err(format!("FFmpeg command failed with exit code: {:?}", output.code()).into());
    }

    Ok(())
}
