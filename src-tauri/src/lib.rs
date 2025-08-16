use crate::utils::AppConfig;
use tauri::{AppHandle, Manager};

mod commands;
mod handlers;
mod media;
mod processors;
mod utils;

// Create a wrapper for the AppHandle
pub struct AppState {
    pub app_handle: AppHandle,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize the global configuration
            AppConfig::init(app.handle())?;

            // Store the app handle in state
            app.manage(AppState {
                app_handle: app.handle().clone(),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::process_images,
            commands::process_videos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
