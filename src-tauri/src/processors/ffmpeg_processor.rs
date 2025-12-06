use crate::handlers::progress_handler::ProgressManager;
use crate::media::image::ffmpeg_logger;
use ffmpeg_sidecar::command::FfmpegCommand;
use std::error::Error;

pub struct FfmpegBatchCommand {
    pub command: FfmpegCommand,
    pub batch_size: usize,
}

pub fn spawn_ffmpeg_process(
    ffmpeg_batch_command: &mut FfmpegBatchCommand,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ffmpeg_child = ffmpeg_batch_command.command.spawn()?;

    ffmpeg_logger(ffmpeg_child)?;

    ProgressManager::increment_progress(ffmpeg_batch_command.batch_size);

    Ok(())
}
