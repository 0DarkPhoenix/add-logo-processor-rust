use std::error::Error;

use crate::shared::{
    ffmpeg_logger::ffmpeg_logger,
    ffmpeg_structs::FfmpegBatchCommand,
    progress_handler::{ProgressManager, ProgressMode},
};

pub fn spawn_ffmpeg_process(
    ffmpeg_batch_command: &mut FfmpegBatchCommand,
    progress_mode: ProgressMode,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ffmpeg_child = ffmpeg_batch_command.command.spawn()?;

    ffmpeg_logger(ffmpeg_child, progress_mode)?;

    match progress_mode {
        ProgressMode::Batch => {
            // Increment progress for image process
            ProgressManager::increment_progress(ffmpeg_batch_command.batch_size);
        }
        ProgressMode::PerFrame => {
            // Increment alternative progress for video process
            ProgressManager::increment_alternative_progress(1);
        }
    }

    Ok(())
}
