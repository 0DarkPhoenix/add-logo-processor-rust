use crate::{
    handlers::{handle_images, handle_videos},
    utils::AppConfig,
    AppState,
};
use tauri::State;

#[tauri::command]
pub fn process_images(app_state: State<AppState>, config: AppConfig) -> Result<(), String> {
    config
        .save(&app_state.app_handle)
        .map_err(|e| e.to_string())?;

    handle_images(&config).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn process_videos(app_state: State<AppState>, config: AppConfig) -> Result<(), String> {
    config
        .save(&app_state.app_handle)
        .map_err(|e| e.to_string())?;

    handle_videos(&config).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn load_config(app_state: State<AppState>) -> Result<AppConfig, String> {
    AppConfig::load_or_create_default(&app_state.app_handle).map_err(|e| e.to_string())
}
