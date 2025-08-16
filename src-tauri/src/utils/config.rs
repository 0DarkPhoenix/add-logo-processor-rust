use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::{error::Error, fs};
use tauri::{AppHandle, Manager};

use crate::media::Corner;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub image_settings: ImageSettings,
    pub video_settings: VideoSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSettings {
    pub input_directory: PathBuf,
    pub output_directory: PathBuf,
    pub search_child_folders: bool,
    pub keep_child_folders_structure_in_output_directory: bool,
    pub min_pixel_count: u32,
    pub add_logo: bool,
    pub logo_path: Option<PathBuf>,
    pub logo_scale: u32,
    pub logo_x_offset_scale: i32,
    pub logo_y_offset_scale: i32,
    pub logo_corner: Corner,
    pub should_convert_format: bool,
    pub format: ImageFormat,
    pub clear_files_input_directory: bool,
    pub clear_files_output_directory: bool,
    pub overwrite_existing_files_output_directory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    pub input_directory: PathBuf,
    pub output_directory: PathBuf,
    pub search_child_folders: bool,
    pub keep_child_folders_structure_in_output_directory: bool,
    pub min_pixel_count: u32,
    pub add_logo: bool,
    pub logo_path: Option<PathBuf>,
    pub logo_scale: u32,
    pub logo_x_offset_scale: i32,
    pub logo_y_offset_scale: i32,
    pub logo_corner: Corner,
    pub should_convert_format: bool,
    pub format: ImageFormat,
    pub clear_files_input_directory: bool,
    pub clear_files_output_directory: bool,
    pub overwrite_existing_files_output_directory: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            image_settings: ImageSettings {
                input_directory: PathBuf::from("input"),
                output_directory: PathBuf::from("output"),
                search_child_folders: false,
                keep_child_folders_structure_in_output_directory: false,
                min_pixel_count: 1080,
                add_logo: false,
                logo_path: None,
                logo_scale: 10,
                logo_x_offset_scale: 0,
                logo_y_offset_scale: 0,
                logo_corner: Corner::TopLeft,
                should_convert_format: false,
                format: ImageFormat::Png,
                clear_files_input_directory: false,
                clear_files_output_directory: false,
                overwrite_existing_files_output_directory: false,
            },
            video_settings: VideoSettings {
                input_directory: PathBuf::from("input"),
                output_directory: PathBuf::from("output"),
                search_child_folders: false,
                keep_child_folders_structure_in_output_directory: false,
                min_pixel_count: 1080,
                add_logo: false,
                logo_path: None,
                logo_scale: 10,
                logo_x_offset_scale: 0,
                logo_y_offset_scale: 0,
                logo_corner: Corner::TopLeft,
                should_convert_format: false,
                format: ImageFormat::Png,
                clear_files_input_directory: false,
                clear_files_output_directory: false,
                overwrite_existing_files_output_directory: false,
            },
        }
    }
}

// Global configuration instance
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    /// Initialize the global configuration with app handle
    pub fn init(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
        let config = Self::load_or_create_default(app_handle)?;
        CONFIG
            .set(config)
            .map_err(|_| "Failed to set global config")?;
        Ok(())
    }

    /// Get the global configuration instance
    pub fn global() -> &'static AppConfig {
        CONFIG
            .get()
            .expect("Config not initialized. Call AppConfig::init() first.")
    }

    /// Load configuration from file or create default
    pub fn load_or_create_default(app_handle: &AppHandle) -> Result<AppConfig, Box<dyn Error>> {
        let config_path = Self::get_config_path(app_handle)?;

        if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)?;
            let config: AppConfig = serde_json::from_str(&config_str)?;
            Ok(config)
        } else {
            let default_config = AppConfig::default();
            default_config.save(app_handle)?;
            Ok(default_config)
        }
    }

    /// Save configuration to file
    pub fn save(&self, app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
        let config_path = Self::get_config_path(app_handle)?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, config_str)?;
        Ok(())
    }

    /// Get the configuration file path using Tauri's path resolver
    fn get_config_path(app_handle: &AppHandle) -> Result<PathBuf, Box<dyn Error>> {
        let config_dir = app_handle
            .path()
            .app_config_dir()
            .map_err(|e| format!("Failed to get config directory: {}", e))?;

        Ok(config_dir.join("config.json"))
    }
}
