use tauri::{AppHandle, Manager, State};

use crate::{
    image::{image_formats::IMAGE_FORMAT_REGISTRY, image_handler::handle_images},
    shared::{process_manager::ProcessManager, progress_handler::ProgressManager},
    video::{
        video_codecs::VIDEO_CODEC_REGISTRY, video_formats::VIDEO_FORMAT_REGISTRY,
        video_handler::handle_videos,
    },
    AppConfig, AppState, ImageSettings, ProgressInfo, VideoSettings,
};

use std::process::Command;

/* -------------------------------------------------------------------------- */
/*                                   GENERAL                                  */
/* -------------------------------------------------------------------------- */
#[tauri::command]
pub fn load_config() -> Result<AppConfig, String> {
    Ok(AppConfig::global())
}

#[tauri::command]
pub fn get_progress_info() -> Result<Option<ProgressInfo>, String> {
    Ok(ProgressManager::get_progress())
}

#[tauri::command]
pub fn cancel_process() -> Result<(), String> {
    ProcessManager::request_cancel();

    // Wait a moment to make sure no new processes are created
    std::thread::sleep(std::time::Duration::from_secs(1));

    ProcessManager::kill_all_processes().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn show_config_in_folder(app_handle: AppHandle) -> Result<(), String> {
    let config_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    // Open the directory in the native file explorer
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&config_dir)
            .spawn()
            .map_err(|e| format!("Failed to open file explorer: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&config_dir)
            .spawn()
            .map_err(|e| format!("Failed to open file explorer: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&config_dir)
            .spawn()
            .map_err(|e| format!("Failed to open file explorer: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn show_log_in_folder(app_handle: AppHandle) -> Result<(), String> {
    let log_dir = app_handle
        .path()
        .app_log_dir()
        .map_err(|e| format!("Failed to get log directory: {}", e))?;

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&log_dir)
        .map_err(|e| format!("Failed to create log directory: {}", e))?;

    // Open the directory in the native file explorer
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| format!("Failed to open file explorer: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| format!("Failed to open file explorer: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| format!("Failed to open file explorer: {}", e))?;
    }

    Ok(())
}

/* -------------------------------------------------------------------------- */
/*                                   IMAGES                                   */
/* -------------------------------------------------------------------------- */
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

#[tauri::command]
pub fn get_supported_image_formats() -> Result<Vec<String>, String> {
    let formats = IMAGE_FORMAT_REGISTRY
        .get_writable_formats()
        .iter()
        .map(|format| format.name.to_string())
        .collect();
    Ok(formats)
}

/* -------------------------------------------------------------------------- */
/*                                   VIDEOS                                   */
/* -------------------------------------------------------------------------- */
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

#[tauri::command]
pub fn get_supported_video_formats() -> Result<Vec<String>, String> {
    let formats = VIDEO_FORMAT_REGISTRY
        .get_writable_formats()
        .iter()
        .map(|format| format.name.to_string())
        .collect();
    Ok(formats)
}

#[tauri::command]
pub fn get_supported_video_codecs() -> Result<Vec<String>, String> {
    let codecs = VIDEO_CODEC_REGISTRY
        .get_codecs_with_encoding()
        .iter()
        .map(|codec| codec.name.to_string())
        .collect();
    Ok(codecs)
}
