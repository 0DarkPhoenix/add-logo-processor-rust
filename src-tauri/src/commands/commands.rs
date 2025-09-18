use crate::{
    handlers::{
        handle_images, handle_videos,
        progress_handler::{ProgressInfo, ProgressManager},
    },
    utils::AppConfig,
    AppState, ImageSettings, VideoSettings,
};
use tauri::State;

#[tauri::command]
pub fn load_config() -> Result<AppConfig, String> {
    Ok(AppConfig::global())
}

#[tauri::command]
pub fn get_progress_info() -> Result<Option<ProgressInfo>, String> {
    Ok(ProgressManager::get_progress())
}

#[tauri::command(async)]
pub fn process_images(
    app_state: State<AppState>,
    image_settings: ImageSettings,
) -> Result<(), String> {
    AppConfig::update_global_image_settings(image_settings.clone(), &app_state.app_handle)
        .map_err(|e| e.to_string())?;

    handle_images(&image_settings).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command(async)]
pub fn process_videos(
    app_state: State<AppState>,
    video_settings: VideoSettings,
) -> Result<(), String> {
    AppConfig::update_global_video_settings(video_settings.clone(), &app_state.app_handle)
        .map_err(|e| e.to_string())?;

    handle_videos(&video_settings).map_err(|e| e.to_string())?;

    Ok(())
}
