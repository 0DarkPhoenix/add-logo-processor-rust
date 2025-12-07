use std::error::Error;

use ffmpeg_sidecar::child::FfmpegChild;
use log::error;

use crate::shared::{
    process_manager::ProcessManager,
    progress_handler::{ProgressManager, ProgressMode},
};

/// Logger that processes FFmpeg events and waits for completion
pub fn ffmpeg_logger(
    mut ffmpeg_child: FfmpegChild,
    progress_mode: ProgressMode,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Register the ffmpeg process to the process manager
    let pid = ffmpeg_child.as_inner().id();
    let process_id = ProcessManager::register_process_by_pid(pid);

    // Process FFmpeg output without holding any locks
    let result = process_ffmpeg_output(&mut ffmpeg_child, progress_mode);

    // Unregister after completion
    ProcessManager::unregister_process(process_id);

    result
}

/// Process FFmpeg output without any mutex operations
fn process_ffmpeg_output(
    ffmpeg_child: &mut FfmpegChild,
    progress_mode: ProgressMode,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut last_frame_count: usize = 0;

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
                // Only track per-frame progress for video mode
                if matches!(progress_mode, ProgressMode::PerFrame) {
                    let current_frame_count = progress.frame as usize;
                    let frame_count_increase = current_frame_count - last_frame_count;
                    ProgressManager::increment_progress(frame_count_increase);
                    last_frame_count = current_frame_count;
                }
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
