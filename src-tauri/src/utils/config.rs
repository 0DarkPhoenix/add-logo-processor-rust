use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};
use std::{error::Error, fs};
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use crate::formats::image_format_types::image_format;
use crate::media::video::{video_codec_strings, video_format_strings};
use crate::media::Corner;

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
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    #[ts(type = "string")]
    pub input_directory: PathBuf,

    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    #[ts(type = "string")]
    pub output_directory: PathBuf,

    pub clear_files_input_directory: bool,
    pub search_child_folders: bool,
    pub clear_files_output_directory: bool,
    pub keep_child_folders_structure_in_output_directory: bool,
    pub overwrite_existing_files_output_directory: bool,
    pub min_pixel_count: u32,
    pub add_logo: bool,

    #[serde(
        serialize_with = "serialize_optional_pathbuf",
        deserialize_with = "deserialize_optional_pathbuf"
    )]
    #[ts(type = "string | null")]
    pub logo_path: Option<PathBuf>,

    pub logo_scale: u32,
    pub logo_x_offset_scale: i32,
    pub logo_y_offset_scale: i32,
    pub logo_corner: Corner,
    pub should_convert_format: bool,
    #[ts(
        type = "\"png\" | \"jpeg\" | \"webp\" | \"bmp\" | \"gif\" | \"tiff\" | \"ico\" | \"pnm\" | \"tga\" | \"hdr\" | \"exr\" | \"ff\" | \"avif\" | \"qoi\""
    )]
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct VideoSettings {
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    #[ts(type = "string")]
    pub input_directory: PathBuf,

    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    #[ts(type = "string")]
    pub output_directory: PathBuf,

    pub clear_files_input_directory: bool,
    pub search_child_folders: bool,
    pub clear_files_output_directory: bool,
    pub keep_child_folders_structure_in_output_directory: bool,
    pub overwrite_existing_files_output_directory: bool,
    pub min_pixel_count: u32,
    pub add_logo: bool,

    #[serde(
        serialize_with = "serialize_optional_pathbuf",
        deserialize_with = "deserialize_optional_pathbuf"
    )]
    #[ts(type = "string | null")]
    pub logo_path: Option<PathBuf>,

    pub logo_scale: u32,
    pub logo_x_offset_scale: i32,
    pub logo_y_offset_scale: i32,
    pub logo_corner: Corner,
    pub should_convert_format: bool,

    #[ts(
        type = "\"3g2\" | \"3gp\" | \"a64\" | \"adts\" | \"amv\" | \"asf\" | \"avi\" | \"avif\" | \"swf\" | \"txt\" | \"crc\" | \"mpd\" | \"vob\" | \"f4v\" | \"fifo\" | \"flv\" | \"hash\" | \"md5\" | \"gif\" | \"f4m\" | \"m3u8\" | \"jpg\" | \"m4v\" | \"ismv\" | \"latm\" | \"mkv\" | \"mov\" | \"mp2\" | \"mp4\" | \"mpg\" | \"m1v\" | \"m2v\" | \"ts\" | \"mjpg\" | \"mxf\" | \"null\" | \"oga\" | \"ogv\" | \"opus\" | \"rtp\" | \"rtsp\" | \"sap\" | \"sdl\" | \"ism\" | \"spx\" | \"tee\" | \"ttml\" | \"webm\" | \"webp\""
    )]
    pub format: String,

    pub should_convert_codec: bool,

    #[ts(
        type = "\"a64_multi\" | \"a64_multi5\" | \"alias_pix\" | \"amv\" | \"apng\" | \"asv1\" | \"asv2\" | \"av1\" | \"avrp\" | \"avui\" | \"ayuv\" | \"bitpacked\" | \"bmp\" | \"cfhd\" | \"cinepak\" | \"cljr\" | \"dnxhd\" | \"dpx\" | \"dvvideo\" | \"exr\" | \"ffv1\" | \"ffvhuff\" | \"fits\" | \"flashsv\" | \"flashsv2\" | \"flv1\" | \"gif\" | \"h261\" | \"h263\" | \"h263p\" | \"h264\" | \"hdr\" | \"hevc\" | \"huffyuv\" | \"jpeg2000\" | \"jpegls\" | \"ljpeg\" | \"magicyuv\" | \"mjpeg\" | \"mpeg1video\" | \"mpeg2video\" | \"mpeg4\" | \"msmpeg4v2\" | \"msmpeg4v3\" | \"msvideo1\" | \"pam\" | \"pbm\" | \"pcx\" | \"pfm\" | \"pgm\" | \"pgmyuv\" | \"phm\" | \"png\" | \"ppm\" | \"prores\" | \"qoi\" | \"qtrle\" | \"r10k\" | \"r210\" | \"rawvideo\" | \"roq\" | \"rpza\" | \"rv10\" | \"rv20\" | \"sgi\" | \"smc\" | \"snow\" | \"speedhq\" | \"sunrast\" | \"svq1\" | \"targa\" | \"theora\" | \"tiff\" | \"utvideo\" | \"v210\" | \"v308\" | \"v408\" | \"v410\" | \"vbn\" | \"vnull\" | \"vp8\" | \"vp9\" | \"wbmp\" | \"webp\" | \"wmv1\" | \"wmv2\" | \"wrapped_avframe\" | \"xbm\" | \"xface\" | \"xwd\" | \"y41p\" | \"yuv4\" | \"zlib\" | \"zmbv\""
    )]
    pub codec: String,
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
                format: image_format::PNG.extensions[0].to_string(),
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
                format: video_format_strings::MP4.to_string(),
                should_convert_codec: false,
                codec: video_codec_strings::H264.to_string(),
                clear_files_input_directory: false,
                clear_files_output_directory: false,
                overwrite_existing_files_output_directory: false,
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
