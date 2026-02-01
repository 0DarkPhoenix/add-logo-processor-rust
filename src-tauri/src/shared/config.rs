use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};
use std::{error::Error, fs};
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use crate::image::image_formats::image_format;
use crate::video::video_codecs::video_codec;
use crate::video::video_formats::video_format;
use crate::Corner;

/// Custom serialization for `PathBuf`
#[allow(clippy::ptr_arg)]
fn serialize_pathbuf<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    path.to_string_lossy().serialize(serializer)
}

/// Custom deserialization for `PathBuf`
fn deserialize_pathbuf<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(PathBuf::from(s))
}

/// Custom serialization for `Option<PathBuf>`
fn serialize_optional_pathbuf<S>(path: &Option<PathBuf>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match path {
        Some(p) => Some(p.to_string_lossy().to_string()).serialize(serializer),
        None => serializer.serialize_none(),
    }
}

/// Custom deserialization for `Option<PathBuf>`
fn deserialize_optional_pathbuf<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt.map(PathBuf::from))
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub image_settings: ImageSettings,
    pub video_settings: VideoSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ImageSettings {
    pub add_logo: bool,
    pub clear_files_input_directory: bool,
    pub clear_files_output_directory: bool,
    #[serde(alias = "favorite_formats")] // Deprecated field names
    pub format_favorite_list: Vec<String>,
    pub format: String,
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    #[ts(type = "string")]
    pub input_directory: PathBuf,
    pub keep_child_folders_structure_in_output_directory: bool,
    pub logo_corner: Corner,
    #[serde(
        serialize_with = "serialize_optional_pathbuf",
        deserialize_with = "deserialize_optional_pathbuf"
    )]
    #[ts(type = "string | null")]
    pub logo_path: Option<PathBuf>,
    pub logo_scale: u32,
    pub logo_x_offset_scale: i32,
    pub logo_y_offset_scale: i32,
    pub min_pixel_count: u32,
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    #[ts(type = "string")]
    pub output_directory: PathBuf,
    pub overwrite_existing_files_output_directory: bool,
    pub search_child_folders: bool,
    pub should_convert_format: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct VideoSettings {
    pub add_logo: bool,
    pub clear_files_input_directory: bool,
    pub clear_files_output_directory: bool,
    #[serde(alias = "favorite_codecs")] // Deprecated field names
    pub codec_favorite_list: Vec<String>,
    pub codec: String,
    #[serde(alias = "favorite_formats")] // Deprecated field names
    pub format_favorite_list: Vec<String>,
    pub format: String,
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    #[ts(type = "string")]
    pub input_directory: PathBuf,
    pub keep_child_folders_structure_in_output_directory: bool,
    pub logo_corner: Corner,
    #[serde(
        serialize_with = "serialize_optional_pathbuf",
        deserialize_with = "deserialize_optional_pathbuf"
    )]
    #[ts(type = "string | null")]
    pub logo_path: Option<PathBuf>,
    pub logo_scale: u32,
    pub logo_x_offset_scale: i32,
    pub logo_y_offset_scale: i32,
    pub min_pixel_count: u32,
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    #[ts(type = "string")]
    pub output_directory: PathBuf,
    pub overwrite_existing_files_output_directory: bool,
    pub search_child_folders: bool,
    pub should_convert_codec: bool,
    pub should_convert_format: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            image_settings: ImageSettings {
                add_logo: false,
                clear_files_input_directory: false,
                clear_files_output_directory: false,
                format_favorite_list: vec![
                    image_format::JPEG.extensions[0].to_string(),
                    image_format::PNG.extensions[0].to_string(),
                    image_format::WEBP.extensions[0].to_string(),
                ],
                format: image_format::PNG.extensions[0].to_string(),
                input_directory: PathBuf::from("input"),
                keep_child_folders_structure_in_output_directory: false,
                logo_corner: Corner::TopLeft,
                logo_path: None,
                logo_scale: 10,
                logo_x_offset_scale: 0,
                logo_y_offset_scale: 0,
                min_pixel_count: 1080,
                output_directory: PathBuf::from("output"),
                overwrite_existing_files_output_directory: false,
                search_child_folders: false,
                should_convert_format: false,
            },
            video_settings: VideoSettings {
                add_logo: false,
                clear_files_input_directory: false,
                clear_files_output_directory: false,
                codec_favorite_list: vec![
                    video_codec::H264.name.to_string(),
                    video_codec::HEVC.name.to_string(),
                    video_codec::VP9.name.to_string(),
                ],
                codec: video_codec::H264.name.to_string(),
                format_favorite_list: vec![
                    video_format::MKV.extensions[0].to_string(),
                    video_format::MOV.extensions[0].to_string(),
                    video_format::MP4.extensions[0].to_string(),
                ],
                format: video_format::MP4.extensions[0].to_string(),
                input_directory: PathBuf::from("input"),
                keep_child_folders_structure_in_output_directory: false,
                logo_corner: Corner::TopLeft,
                logo_path: None,
                logo_scale: 10,
                logo_x_offset_scale: 0,
                logo_y_offset_scale: 0,
                min_pixel_count: 1080,
                output_directory: PathBuf::from("output"),
                overwrite_existing_files_output_directory: false,
                search_child_folders: false,
                should_convert_codec: false,
                should_convert_format: false,
            },
        }
    }
}

// Global configuration instance with RwLock for thread-safe mutation
static CONFIG: OnceLock<RwLock<AppConfig>> = OnceLock::new();

impl AppConfig {
    /// Initialize the global configuration with app handle
    pub fn init(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
        let config = Self::load_or_create_default(app_handle)?;
        CONFIG
            .set(RwLock::new(config))
            .map_err(|_| "Failed to set global config")?;
        Ok(())
    }

    /// Get a clone of the global configuration instance
    pub fn global() -> AppConfig {
        CONFIG
            .get()
            .expect("Config not initialized. Call AppConfig::init() first.")
            .read()
            .unwrap()
            .clone()
    }

    /// Update only image settings in global config and save
    pub fn update_global_image_settings(
        image_settings: ImageSettings,
        app_handle: &AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        let config_lock = CONFIG
            .get()
            .expect("Config not initialized. Call AppConfig::init() first.");

        {
            let mut config = config_lock.write().unwrap();
            config.image_settings = image_settings;
        }

        // Save the updated config
        let config = config_lock.read().unwrap();
        config.save(app_handle)
    }

    /// Update only video settings in global config and save
    pub fn update_global_video_settings(
        video_settings: VideoSettings,
        app_handle: &AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        let config_lock = CONFIG
            .get()
            .expect("Config not initialized. Call AppConfig::init() first.");

        {
            let mut config = config_lock.write().unwrap();
            config.video_settings = video_settings;
        }

        // Save the updated config
        let config = config_lock.read().unwrap();
        config.save(app_handle)
    }

    /// Load configuration from file or create default
    pub fn load_or_create_default(app_handle: &AppHandle) -> Result<AppConfig, Box<dyn Error>> {
        let config_path = Self::get_config_path(app_handle)?;

        let config = if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)?;
            match serde_json::from_str::<AppConfig>(&config_str) {
                Ok(config) => config,
                Err(_) => {
                    // Deserialization failed, attempt migration
                    let mut config = AppConfig::default();
                    config.migrate_current_config(app_handle)?;
                    config
                }
            }
        } else {
            let default_config = AppConfig::default();
            default_config.save(app_handle)?;
            default_config
        };

        Ok(config)
    }

    /// Update only image settings and save (instance method)
    pub fn update_image_settings(
        &mut self,
        image_settings: ImageSettings,
        app_handle: &AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        self.image_settings = image_settings;
        self.save(app_handle)
    }

    /// Update only video settings and save (instance method)
    pub fn update_video_settings(
        &mut self,
        video_settings: VideoSettings,
        app_handle: &AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        self.video_settings = video_settings;
        self.save(app_handle)
    }

    /// Migrates the current config to a newer version of the AppConfig struct
    ///
    /// This function creates a new default AppConfig and merges in existing values from the current config file,
    /// effectively migrating the current config to the new config structure.
    ///
    /// ## Note
    /// Config settings which are renamed are migrated by reading the old setting's value
    /// using serde (e.g. `#[serde(alias = "<old-name>", alias = "<another-old-name>")]` above a settings name)
    /// and are saved using the new name (done when saving the new config to `config.json`).
    fn migrate_current_config(&mut self, app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
        let config_path = Self::get_config_path(app_handle)?;
        let config_str = fs::read_to_string(&config_path)?;
        let current_config: serde_json::Value = serde_json::from_str(&config_str)?;

        // Create default config and merge in current values
        let mut app_config_json = serde_json::to_value(AppConfig::default())?;

        if let Some(obj) = app_config_json.as_object_mut() {
            if let Some(current_obj) = current_config.as_object() {
                for (key, value) in current_obj.iter() {
                    // If both are objects, merge them recursively
                    if let (
                        Some(serde_json::Value::Object(default_nested)),
                        serde_json::Value::Object(current_nested),
                    ) = (obj.get_mut(key), value)
                    {
                        for (nested_key, nested_value) in current_nested.iter() {
                            default_nested.insert(nested_key.clone(), nested_value.clone());
                        }
                        continue;
                    }
                    // Otherwise, replace the entire value
                    obj.insert(key.clone(), value.clone());
                }
            }
        }

        let new_config: AppConfig = serde_json::from_value(app_config_json)?;
        *self = new_config;
        self.save(app_handle)?;

        Ok(())
    }

    /// Save configuration to file
    fn save(&self, app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
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
