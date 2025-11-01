use crate::{
    formats::video_format_types::VIDEO_FORMAT_REGISTRY,
    media::Media,
    utils::{read_file_size, read_file_type},
};

use super::types::Resolution;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};

pub mod video_codec_strings {
    pub const A64_MULTI: &str = "a64_multi";
    pub const A64_MULTI5: &str = "a64_multi5";
    pub const ALIAS_PIX: &str = "alias_pix";
    pub const AMV: &str = "amv";
    pub const APNG: &str = "apng";
    pub const ASV1: &str = "asv1";
    pub const ASV2: &str = "asv2";
    pub const AV1: &str = "av1";
    pub const AVRP: &str = "avrp";
    pub const AVUI: &str = "avui";
    pub const AYUV: &str = "ayuv";
    pub const BITPACKED: &str = "bitpacked";
    pub const BMP: &str = "bmp";
    pub const CFHD: &str = "cfhd";
    pub const CINEPAK: &str = "cinepak";
    pub const CLJR: &str = "cljr";
    pub const DNXHD: &str = "dnxhd";
    pub const DPX: &str = "dpx";
    pub const DVVIDEO: &str = "dvvideo";
    pub const EXR: &str = "exr";
    pub const FFV1: &str = "ffv1";
    pub const FFVHUFF: &str = "ffvhuff";
    pub const FITS: &str = "fits";
    pub const FLASHSV: &str = "flashsv";
    pub const FLASHSV2: &str = "flashsv2";
    pub const FLV1: &str = "flv1";
    pub const GIF: &str = "gif";
    pub const H261: &str = "h261";
    pub const H263: &str = "h263";
    pub const H263P: &str = "h263p";
    pub const H264: &str = "h264";
    pub const HDR: &str = "hdr";
    pub const HEVC: &str = "hevc";
    pub const HUFFYUV: &str = "huffyuv";
    pub const JPEG2000: &str = "jpeg2000";
    pub const JPEGLS: &str = "jpegls";
    pub const LJPEG: &str = "ljpeg";
    pub const MAGICYUV: &str = "magicyuv";
    pub const MJPEG: &str = "mjpeg";
    pub const MPEG1VIDEO: &str = "mpeg1video";
    pub const MPEG2VIDEO: &str = "mpeg2video";
    pub const MPEG4: &str = "mpeg4";
    pub const MSMPEG4V2: &str = "msmpeg4v2";
    pub const MSMPEG4V3: &str = "msmpeg4v3";
    pub const MSVIDEO1: &str = "msvideo1";
    pub const PAM: &str = "pam";
    pub const PBM: &str = "pbm";
    pub const PCX: &str = "pcx";
    pub const PFM: &str = "pfm";
    pub const PGM: &str = "pgm";
    pub const PGMYUV: &str = "pgmyuv";
    pub const PHM: &str = "phm";
    pub const PNG: &str = "png";
    pub const PPM: &str = "ppm";
    pub const PRORES: &str = "prores";
    pub const QOI: &str = "qoi";
    pub const QTRLE: &str = "qtrle";
    pub const R10K: &str = "r10k";
    pub const R210: &str = "r210";
    pub const RAWVIDEO: &str = "rawvideo";
    pub const ROQ: &str = "roq";
    pub const RPZA: &str = "rpza";
    pub const RV10: &str = "rv10";
    pub const RV20: &str = "rv20";
    pub const SGI: &str = "sgi";
    pub const SMC: &str = "smc";
    pub const SNOW: &str = "snow";
    pub const SPEEDHQ: &str = "speedhq";
    pub const SUNRAST: &str = "sunrast";
    pub const SVQ1: &str = "svq1";
    pub const TARGA: &str = "targa";
    pub const THEORA: &str = "theora";
    pub const TIFF: &str = "tiff";
    pub const UTVIDEO: &str = "utvideo";
    pub const V210: &str = "v210";
    pub const V308: &str = "v308";
    pub const V408: &str = "v408";
    pub const V410: &str = "v410";
    pub const VBN: &str = "vbn";
    pub const VNULL: &str = "vnull";
    pub const VP8: &str = "vp8";
    pub const VP9: &str = "vp9";
    pub const WBMP: &str = "wbmp";
    pub const WEBP: &str = "webp";
    pub const WMV1: &str = "wmv1";
    pub const WMV2: &str = "wmv2";
    pub const WRAPPED_AVFRAME: &str = "wrapped_avframe";
    pub const XBM: &str = "xbm";
    pub const XFACE: &str = "xface";
    pub const XWD: &str = "xwd";
    pub const Y41P: &str = "y41p";
    pub const YUV4: &str = "yuv4";
    pub const ZLIB: &str = "zlib";
    pub const ZMBV: &str = "zmbv";
}

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
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let file_size = read_file_size(&path)?;

        // Get file type from extension and validate it's supported by FFmpeg
        let file_type = read_video_file_type(&path)?;

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

/// Read the video file type and validate it's supported by FFmpeg
fn read_video_file_type(
    file_path: &std::path::Path,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file_type = read_file_type(file_path);

    if VIDEO_FORMAT_REGISTRY.is_supported_for_reading(file_type.as_str()) {
        Ok(file_type)
    } else {
        Err(format!("Unsupported video format for reading: {}", file_type).into())
    }
}
