use ffmpeg_sidecar::download::auto_download;
use tauri::{AppHandle, Manager, RunEvent};
use tauri_plugin_log::{Target, TargetKind};
// Re-export types for ts-rs
pub use shared::commands;
pub use shared::config::{AppConfig, ImageSettings, VideoSettings};
pub use shared::media_structs::Corner;
pub use shared::progress_handler::ProgressInfo;

use crate::shared::process_manager::ProcessManager;

mod image;
mod shared;
mod video;

// Create a wrapper for the AppHandle
pub struct AppState {
    pub app_handle: AppHandle,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("app".to_string()),
                    }),
                    Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize the global configuration
            AppConfig::init(app.handle())?;

            // Store the app handle in state
            app.manage(AppState {
                app_handle: app.handle().clone(),
            });

            // Download FFmpeg if not already downloaded
            auto_download()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::get_progress_info,
            commands::cancel_process,
            commands::show_config_in_folder,
            commands::show_log_in_folder,
            commands::process_images,
            commands::get_supported_image_formats,
            commands::process_videos,
            commands::get_supported_video_formats,
            commands::get_supported_video_codecs
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            // Handle app exit events
            if let RunEvent::Exit = event {
                log::info!("Application is exiting, cleaning up FFmpeg processes...");
                if let Err(e) = ProcessManager::kill_all_processes() {
                    log::error!("Failed to kill FFmpeg processes on exit: {}", e);
                }
            }
        });
}
