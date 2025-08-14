use crate::{
    media::Media,
    utils::{read_file_size, read_file_type},
};

use super::types::Resolution;
use ffmpeg_sidecar::download::auto_download;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub file_path: PathBuf,
    pub resolution: Resolution,
    pub file_size: u64,
    pub file_type: String,
    pub duration: f64,
    pub codec: String,
}

impl Video {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        // Auto-download ffmpeg if not available
        auto_download()?;

        let file_size = read_file_size(&path)?;

        // Get file type from extension
        let file_type = read_file_type(&path);

        // Use ffprobe to get video information
        let output = std::process::Command::new("ffprobe")
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                path.to_str().unwrap(),
            ])
            .output()?;

        let probe_result: serde_json::Value = serde_json::from_slice(&output.stdout)?;

        // Extract video stream information
        let video_stream = probe_result["streams"]
            .as_array()
            .and_then(|streams| {
                streams
                    .iter()
                    .find(|stream| stream["codec_type"].as_str() == Some("video"))
            })
            .ok_or("No video stream found")?;

        let width = video_stream["width"].as_u64().unwrap_or(0) as u32;
        let height = video_stream["height"].as_u64().unwrap_or(0) as u32;
        let resolution = Resolution { width, height };

        let codec = video_stream["codec_name"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let duration = probe_result["format"]["duration"]
            .as_str()
            .and_then(|d| d.parse::<f64>().ok())
            .unwrap_or(0.0);

        Ok(Self {
            file_path: path,
            resolution,
            file_size,
            file_type,
            duration,
            codec,
        })
    }

    pub fn get_duration(&self) -> f64 {
        self.duration
    }

    pub fn set_codec(&mut self, new_codec: String) {
        self.codec = new_codec;
    }
}

impl Media for Video {
    type FileType = String;

    fn get_resolution(&self) -> &Resolution {
        &self.resolution
    }

    fn get_file_size(&self) -> u64 {
        self.file_size
    }

    fn get_file_type(&self) -> &Self::FileType {
        &self.file_type
    }

    fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution;
    }
}
