use std::error::Error;

use crate::shared::{
    ffmpeg_logger::ffmpeg_logger, ffmpeg_structs::FfmpegBatchCommand,
    progress_handler::ProgressManager,
};

pub fn spawn_ffmpeg_process(
    ffmpeg_batch_command: &mut FfmpegBatchCommand,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ffmpeg_child = ffmpeg_batch_command.command.spawn()?;

    ffmpeg_logger(ffmpeg_child)?;

    ProgressManager::increment_progress(ffmpeg_batch_command.batch_size);

    Ok(())
}
