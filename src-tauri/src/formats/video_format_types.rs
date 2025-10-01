use std::collections::HashMap;

use crate::formats::format::{Format, FormatSupport};

pub type VideoFormat = Format;

pub mod video_format {
    use super::{FormatSupport, VideoFormat};

    // Container formats
    pub const MP4: VideoFormat = VideoFormat::new(
        "mp4",
        &["mp4", "m4v"],
        FormatSupport::read_write(),
        "MPEG-4 Part 14",
    );

    pub const AVI: VideoFormat = VideoFormat::new(
        "avi",
        &["avi"],
        FormatSupport::read_write(),
        "Audio Video Interleaved",
    );

    pub const MOV: VideoFormat = VideoFormat::new(
        "mov",
        &["mov"],
        FormatSupport::read_write(),
        "QuickTime Movie",
    );

    pub const MKV: VideoFormat = VideoFormat::new(
        "mkv",
        &["mkv"],
        FormatSupport::write_only(),
        "Matroska Video",
    );

    pub const WEBM: VideoFormat =
        VideoFormat::new("webm", &["webm"], FormatSupport::read_write(), "WebM");

    pub const FLV: VideoFormat =
        VideoFormat::new("flv", &["flv"], FormatSupport::read_write(), "Flash Video");

    pub const WMV: VideoFormat = VideoFormat::new(
        "wmv",
        &["wmv"],
        FormatSupport::read_only(),
        "Windows Media Video",
    );

    pub const ASF: VideoFormat = VideoFormat::new(
        "asf",
        &["asf"],
        FormatSupport::read_write(),
        "Advanced Systems Format",
    );

    pub const _3GP: VideoFormat = VideoFormat::new(
        "3gp",
        &["3gp"],
        FormatSupport::write_only(),
        "3GPP file format",
    );

    pub const _3G2: VideoFormat = VideoFormat::new(
        "3g2",
        &["3g2"],
        FormatSupport::write_only(),
        "3GPP2 file format",
    );

    pub const F4V: VideoFormat = VideoFormat::new(
        "f4v",
        &["f4v"],
        FormatSupport::write_only(),
        "F4V Adobe Flash Video",
    );

    pub const MXF: VideoFormat = VideoFormat::new(
        "mxf",
        &["mxf"],
        FormatSupport::read_write(),
        "Material eXchange Format",
    );

    pub const GXF: VideoFormat = VideoFormat::new(
        "gxf",
        &["gxf"],
        FormatSupport::read_write(),
        "General eXchange Format",
    );

    pub const RM: VideoFormat = VideoFormat::new(
        "rm",
        &["rm", "rmvb"],
        FormatSupport::read_write(),
        "RealMedia",
    );

    pub const OGV: VideoFormat =
        VideoFormat::new("ogv", &["ogv"], FormatSupport::write_only(), "Ogg Video");

    pub const MPEG: VideoFormat = VideoFormat::new(
        "mpeg",
        &["mpg", "mpeg", "m2v"],
        FormatSupport::read_write(),
        "MPEG-1/2 Systems",
    );

    pub const MPEGTS: VideoFormat = VideoFormat::new(
        "mpegts",
        &["ts", "m2ts", "mts"],
        FormatSupport::read_write(),
        "MPEG-2 Transport Stream",
    );

    pub const VOB: VideoFormat = VideoFormat::new(
        "vob",
        &["vob"],
        FormatSupport::write_only(),
        "MPEG-2 PS (VOB)",
    );

    pub const DVD: VideoFormat = VideoFormat::new(
        "dvd",
        &["dvd"],
        FormatSupport::write_only(),
        "MPEG-2 PS (DVD VOB)",
    );

    pub const SVCD: VideoFormat = VideoFormat::new(
        "svcd",
        &["svcd"],
        FormatSupport::write_only(),
        "MPEG-2 PS (SVCD)",
    );

    pub const VCD: VideoFormat = VideoFormat::new(
        "vcd",
        &["vcd"],
        FormatSupport::write_only(),
        "MPEG-1 Systems (VCD)",
    );

    pub const WTV: VideoFormat = VideoFormat::new(
        "wtv",
        &["wtv"],
        FormatSupport::read_write(),
        "Windows Television",
    );

    pub const DV: VideoFormat =
        VideoFormat::new("dv", &["dv"], FormatSupport::read_write(), "Digital Video");

    pub const NUT: VideoFormat =
        VideoFormat::new("nut", &["nut"], FormatSupport::read_write(), "NUT");

    pub const IVF: VideoFormat =
        VideoFormat::new("ivf", &["ivf"], FormatSupport::read_write(), "On2 IVF");

    pub const Y4M: VideoFormat = VideoFormat::new(
        "y4m",
        &["y4m"],
        FormatSupport::read_write(),
        "YUV4MPEG pipe",
    );

    // Raw video formats
    pub const H264: VideoFormat = VideoFormat::new(
        "h264",
        &["h264", "264"],
        FormatSupport::read_write(),
        "raw H.264 video",
    );

    pub const H265: VideoFormat = VideoFormat::new(
        "hevc",
        &["h265", "hevc", "265"],
        FormatSupport::read_write(),
        "raw HEVC video",
    );

    pub const AV1: VideoFormat =
        VideoFormat::new("av1", &["av1"], FormatSupport::read_only(), "AV1 Annex B");

    pub const VP8: VideoFormat =
        VideoFormat::new("vp8", &["vp8"], FormatSupport::read_only(), "VP8 video");

    pub const VP9: VideoFormat =
        VideoFormat::new("vp9", &["vp9"], FormatSupport::read_only(), "VP9 video");

    pub const MPEG1VIDEO: VideoFormat = VideoFormat::new(
        "mpeg1video",
        &["m1v"],
        FormatSupport::write_only(),
        "raw MPEG-1 video",
    );

    pub const MPEG2VIDEO: VideoFormat = VideoFormat::new(
        "mpeg2video",
        &["m2v"],
        FormatSupport::write_only(),
        "raw MPEG-2 video",
    );

    pub const MPEG4: VideoFormat = VideoFormat::new(
        "mpeg4",
        &["m4v"],
        FormatSupport::read_write(),
        "raw MPEG-4 video",
    );

    pub const MJPEG: VideoFormat = VideoFormat::new(
        "mjpeg",
        &["mjpg"],
        FormatSupport::read_write(),
        "raw MJPEG video",
    );

    pub const VC1: VideoFormat = VideoFormat::new(
        "vc1",
        &["vc1"],
        FormatSupport::read_write(),
        "raw VC-1 video",
    );

    pub const DNXHD: VideoFormat = VideoFormat::new(
        "dnxhd",
        &["dnxhd"],
        FormatSupport::read_write(),
        "raw DNxHD (SMPTE VC-3)",
    );

    pub const DIRAC: VideoFormat =
        VideoFormat::new("dirac", &["drc"], FormatSupport::read_write(), "raw Dirac");

    pub const RAWVIDEO: VideoFormat = VideoFormat::new(
        "rawvideo",
        &["yuv", "rgb"],
        FormatSupport::read_write(),
        "raw video",
    );

    pub const VVC: VideoFormat = VideoFormat::new(
        "vvc",
        &["vvc", "h266"],
        FormatSupport::read_write(),
        "raw H.266/VVC video",
    );

    // Streaming formats
    pub const HLS: VideoFormat = VideoFormat::new(
        "hls",
        &["m3u8"],
        FormatSupport::read_write(),
        "Apple HTTP Live Streaming",
    );

    pub const DASH: VideoFormat =
        VideoFormat::new("dash", &["mpd"], FormatSupport::read_write(), "DASH Muxer");

    pub const RTSP: VideoFormat = VideoFormat::new(
        "rtsp",
        &["rtsp"],
        FormatSupport::read_write(),
        "RTSP output",
    );

    pub const RTP: VideoFormat =
        VideoFormat::new("rtp", &["rtp"], FormatSupport::read_write(), "RTP output");

    // Legacy and specialized formats
    pub const SWF: VideoFormat = VideoFormat::new(
        "swf",
        &["swf"],
        FormatSupport::read_write(),
        "ShockWave Flash",
    );

    pub const ROQ: VideoFormat =
        VideoFormat::new("roq", &["roq"], FormatSupport::read_write(), "raw id RoQ");

    pub const FILM_CPK: VideoFormat = VideoFormat::new(
        "film_cpk",
        &["cpk"],
        FormatSupport::read_write(),
        "Sega FILM / CPK",
    );

    pub const SMJPEG: VideoFormat = VideoFormat::new(
        "smjpeg",
        &["mjpg"],
        FormatSupport::read_write(),
        "Loki SDL MJPEG",
    );

    pub const FILMSTRIP: VideoFormat = VideoFormat::new(
        "filmstrip",
        &["flm"],
        FormatSupport::read_write(),
        "Adobe Filmstrip",
    );

    pub const GIF: VideoFormat = VideoFormat::new(
        "gif",
        &["gif"],
        FormatSupport::read_write(),
        "CompuServe Graphics Interchange Format",
    );

    pub const APNG: VideoFormat = VideoFormat::new(
        "apng",
        &["apng"],
        FormatSupport::read_write(),
        "Animated Portable Network Graphics",
    );

    pub const WEBP: VideoFormat =
        VideoFormat::new("webp", &["webp"], FormatSupport::write_only(), "WebP");

    pub const AVIF: VideoFormat =
        VideoFormat::new("avif", &["avif"], FormatSupport::write_only(), "AVIF");

    // All supported formats in a single array
    pub const ALL: &[VideoFormat] = &[
        MP4, AVI, MOV, MKV, WEBM, FLV, WMV, ASF, _3GP, _3G2, F4V, MXF, GXF, RM, OGV, MPEG, MPEGTS,
        VOB, DVD, SVCD, VCD, WTV, DV, NUT, IVF, Y4M, H264, H265, AV1, VP8, VP9, MPEG1VIDEO,
        MPEG2VIDEO, MPEG4, MJPEG, VC1, DNXHD, DIRAC, RAWVIDEO, VVC, HLS, DASH, RTSP, RTP, SWF, ROQ,
        FILM_CPK, SMJPEG, FILMSTRIP, GIF, APNG, WEBP, AVIF,
    ];
}

pub struct VideoFormatRegistry {
    formats_by_name: HashMap<String, &'static VideoFormat>,
    formats_by_extension: HashMap<String, &'static VideoFormat>,
}

impl VideoFormatRegistry {
    pub fn new() -> Self {
        let mut formats_by_name = HashMap::new();
        let mut formats_by_extension = HashMap::new();

        for format in video_format::ALL {
            formats_by_name.insert(format.name.to_lowercase(), format);

            for &extension in format.extensions {
                formats_by_extension.insert(extension.to_lowercase(), format);
            }
        }

        Self {
            formats_by_name,
            formats_by_extension,
        }
    }

    pub fn get_format_by_name(&self, name: &str) -> Option<&'static VideoFormat> {
        self.formats_by_name.get(&name.to_lowercase()).copied()
    }

    pub fn get_format_by_extension(&self, extension: &str) -> Option<&'static VideoFormat> {
        self.formats_by_extension
            .get(&extension.to_lowercase())
            .copied()
    }

    pub fn is_supported_for_reading(&self, extension: &str) -> bool {
        self.get_format_by_extension(extension)
            .map(|f| f.support.demuxing)
            .unwrap_or(false)
    }

    pub fn is_supported_for_writing(&self, extension: &str) -> bool {
        self.get_format_by_extension(extension)
            .map(|f| f.support.muxing)
            .unwrap_or(false)
    }

    pub fn get_writable_formats(&self) -> Vec<&'static VideoFormat> {
        video_format::ALL
            .iter()
            .filter(|f| f.support.muxing)
            .collect()
    }

    pub fn get_readable_formats(&self) -> Vec<&'static VideoFormat> {
        video_format::ALL
            .iter()
            .filter(|f| f.support.demuxing)
            .collect()
    }
}

impl Default for VideoFormatRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Global registry instance
lazy_static::lazy_static! {
    pub static ref VIDEO_FORMAT_REGISTRY: VideoFormatRegistry = VideoFormatRegistry::new();
}
