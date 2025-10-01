use std::collections::HashMap;

use crate::codecs::codec::{Codec, CodecSupport, CodecType};

pub type VideoCodec = Codec;

pub mod video_codec {
    use super::{CodecSupport, CodecType, VideoCodec};

    // Modern codecs
    pub const H264: VideoCodec = VideoCodec::new(
        "h264",
        "H.264 / AVC / MPEG-4 AVC / MPEG-4 part 10",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["h264", "h264_qsv", "h264_amf", "h264_cuvid"],
        &[
            "libx264",
            "libx264rgb",
            "h264_amf",
            "h264_mf",
            "h264_nvenc",
            "h264_qsv",
            "h264_vaapi",
        ],
    );

    pub const HEVC: VideoCodec = VideoCodec::new(
        "hevc",
        "H.265 / HEVC (High Efficiency Video Coding)",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["hevc", "hevc_qsv", "hevc_amf", "hevc_cuvid"],
        &[
            "libx265",
            "hevc_amf",
            "hevc_d3d12va",
            "hevc_mf",
            "hevc_nvenc",
            "hevc_qsv",
            "hevc_vaapi",
        ],
    );

    pub const AV1: VideoCodec = VideoCodec::new(
        "av1",
        "Alliance for Open Media AV1",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["libaom-av1", "av1", "av1_cuvid", "av1_qsv", "av1_amf"],
        &[
            "libaom-av1",
            "av1_nvenc",
            "av1_qsv",
            "av1_amf",
            "av1_mf",
            "av1_vaapi",
        ],
    );

    pub const VP8: VideoCodec = VideoCodec::new(
        "vp8",
        "On2 VP8",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["vp8", "libvpx", "vp8_cuvid", "vp8_qsv"],
        &["libvpx", "vp8_vaapi"],
    );

    pub const VP9: VideoCodec = VideoCodec::new(
        "vp9",
        "Google VP9",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["vp9", "libvpx-vp9", "vp9_amf", "vp9_cuvid", "vp9_qsv"],
        &["libvpx-vp9", "vp9_vaapi", "vp9_qsv"],
    );

    pub const VVC: VideoCodec = VideoCodec::new(
        "vvc",
        "H.266 / VVC (Versatile Video Coding)",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["vvc", "vvc_qsv"],
        &[],
    );

    // MPEG codecs
    pub const MPEG1VIDEO: VideoCodec = VideoCodec::new(
        "mpeg1video",
        "MPEG-1 video",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["mpeg1video", "mpeg1_cuvid"],
        &["mpeg1video"],
    );

    pub const MPEG2VIDEO: VideoCodec = VideoCodec::new(
        "mpeg2video",
        "MPEG-2 video",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["mpeg2video", "mpegvideo", "mpeg2_qsv", "mpeg2_cuvid"],
        &["mpeg2video", "mpeg2_qsv", "mpeg2_vaapi"],
    );

    pub const MPEG4: VideoCodec = VideoCodec::new(
        "mpeg4",
        "MPEG-4 part 2",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["mpeg4", "mpeg4_cuvid"],
        &["mpeg4", "libxvid"],
    );

    // Microsoft codecs
    pub const MSMPEG4V1: VideoCodec = VideoCodec::new(
        "msmpeg4v1",
        "MPEG-4 part 2 Microsoft variant version 1",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["msmpeg4v1"],
        &[],
    );

    pub const MSMPEG4V2: VideoCodec = VideoCodec::new(
        "msmpeg4v2",
        "MPEG-4 part 2 Microsoft variant version 2",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["msmpeg4v2"],
        &["msmpeg4v2"],
    );

    pub const MSMPEG4V3: VideoCodec = VideoCodec::new(
        "msmpeg4v3",
        "MPEG-4 part 2 Microsoft variant version 3",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["msmpeg4"],
        &["msmpeg4"],
    );

    pub const WMV1: VideoCodec = VideoCodec::new(
        "wmv1",
        "Windows Media Video 7",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["wmv1"],
        &["wmv1"],
    );

    pub const WMV2: VideoCodec = VideoCodec::new(
        "wmv2",
        "Windows Media Video 8",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["wmv2"],
        &["wmv2"],
    );

    pub const WMV3: VideoCodec = VideoCodec::new(
        "wmv3",
        "Windows Media Video 9",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["wmv3"],
        &[],
    );

    pub const VC1: VideoCodec = VideoCodec::new(
        "vc1",
        "SMPTE VC-1",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["vc1", "vc1_qsv", "vc1_cuvid"],
        &[],
    );

    // Apple codecs
    pub const PRORES: VideoCodec = VideoCodec::new(
        "prores",
        "Apple ProRes (iCodec Pro)",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["prores"],
        &["prores", "prores_aw", "prores_ks"],
    );

    pub const PRORES_RAW: VideoCodec = VideoCodec::new(
        "prores_raw",
        "Apple ProRes RAW",
        CodecSupport::decode_only(),
        CodecType::Lossless,
        &["prores_raw"],
        &[],
    );

    // Motion JPEG
    pub const MJPEG: VideoCodec = VideoCodec::new(
        "mjpeg",
        "Motion JPEG",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["mjpeg", "mjpeg_cuvid", "mjpeg_qsv"],
        &["mjpeg", "mjpeg_qsv", "mjpeg_vaapi"],
    );

    // Lossless codecs
    pub const HUFFYUV: VideoCodec = VideoCodec::new(
        "huffyuv",
        "HuffYUV",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["huffyuv"],
        &["huffyuv"],
    );

    pub const FFVHUFF: VideoCodec = VideoCodec::new(
        "ffvhuff",
        "Huffyuv FFmpeg variant",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["ffvhuff"],
        &["ffvhuff"],
    );

    pub const FFV1: VideoCodec = VideoCodec::new(
        "ffv1",
        "FFmpeg video codec #1",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["ffv1"],
        &["ffv1"],
    );

    pub const UTVIDEO: VideoCodec = VideoCodec::new(
        "utvideo",
        "Ut Video",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["utvideo"],
        &["utvideo"],
    );

    pub const MAGICYUV: VideoCodec = VideoCodec::new(
        "magicyuv",
        "MagicYUV video",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["magicyuv"],
        &["magicyuv"],
    );

    // Uncompressed formats
    pub const RAWVIDEO: VideoCodec = VideoCodec::new(
        "rawvideo",
        "raw video",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["rawvideo"],
        &["rawvideo"],
    );

    pub const V210: VideoCodec = VideoCodec::new(
        "v210",
        "Uncompressed 4:2:2 10-bit",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["v210"],
        &["v210"],
    );

    pub const V308: VideoCodec = VideoCodec::new(
        "v308",
        "Uncompressed packed 4:4:4",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["v308"],
        &["v308"],
    );

    pub const V408: VideoCodec = VideoCodec::new(
        "v408",
        "Uncompressed packed QT 4:4:4:4",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["v408"],
        &["v408"],
    );

    pub const V410: VideoCodec = VideoCodec::new(
        "v410",
        "Uncompressed 4:4:4 10-bit",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["v410"],
        &["v410"],
    );

    // Legacy and specialized codecs
    pub const THEORA: VideoCodec = VideoCodec::new(
        "theora",
        "Theora",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["theora"],
        &["libtheora"],
    );

    pub const DIRAC: VideoCodec = VideoCodec::new(
        "dirac",
        "Dirac",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["dirac"],
        &["vc2"],
    );

    pub const SNOW: VideoCodec = VideoCodec::new(
        "snow",
        "Snow",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["snow"],
        &["snow"],
    );

    pub const CINEPAK: VideoCodec = VideoCodec::new(
        "cinepak",
        "Cinepak",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["cinepak"],
        &["cinepak"],
    );

    pub const INDEO3: VideoCodec = VideoCodec::new(
        "indeo3",
        "Intel Indeo 3",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["indeo3"],
        &[],
    );

    pub const INDEO4: VideoCodec = VideoCodec::new(
        "indeo4",
        "Intel Indeo Video Interactive 4",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["indeo4"],
        &[],
    );

    pub const INDEO5: VideoCodec = VideoCodec::new(
        "indeo5",
        "Intel Indeo Video Interactive 5",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["indeo5"],
        &[],
    );

    // Animation and image formats
    pub const GIF: VideoCodec = VideoCodec::new(
        "gif",
        "CompuServe Graphics Interchange Format",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["gif"],
        &["gif"],
    );

    pub const APNG: VideoCodec = VideoCodec::new(
        "apng",
        "APNG (Animated Portable Network Graphics) image",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["apng"],
        &["apng"],
    );

    pub const WEBP: VideoCodec = VideoCodec::new(
        "webp",
        "WebP",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["webp"],
        &["libwebp_anim", "libwebp"],
    );

    pub const AVIF: VideoCodec = VideoCodec::new(
        "avif",
        "AVIF",
        CodecSupport::encode_only(),
        CodecType::Lossy,
        &[],
        &["avif"],
    );

    // Professional and broadcast codecs
    pub const DNXHD: VideoCodec = VideoCodec::new(
        "dnxhd",
        "VC3/DNxHD",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["dnxhd"],
        &["dnxhd"],
    );

    pub const CFHD: VideoCodec = VideoCodec::new(
        "cfhd",
        "GoPro CineForm HD",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["cfhd"],
        &["cfhd"],
    );

    pub const DV: VideoCodec = VideoCodec::new(
        "dvvideo",
        "DV (Digital Video)",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["dvvideo"],
        &["dvvideo"],
    );

    pub const SPEEDHQ: VideoCodec = VideoCodec::new(
        "speedhq",
        "NewTek SpeedHQ",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["speedhq"],
        &["speedhq"],
    );

    // Screen capture codecs
    pub const FLASHSV: VideoCodec = VideoCodec::new(
        "flashsv",
        "Flash Screen Video v1",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["flashsv"],
        &["flashsv"],
    );

    pub const FLASHSV2: VideoCodec = VideoCodec::new(
        "flashsv2",
        "Flash Screen Video v2",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["flashsv2"],
        &["flashsv2"],
    );

    pub const TSCC: VideoCodec = VideoCodec::new(
        "tscc",
        "TechSmith Screen Capture Codec",
        CodecSupport::decode_only(),
        CodecType::Lossless,
        &["camtasia"],
        &[],
    );

    pub const TSCC2: VideoCodec = VideoCodec::new(
        "tscc2",
        "TechSmith Screen Codec 2",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["tscc2"],
        &[],
    );

    // QuickTime codecs
    pub const QTRLE: VideoCodec = VideoCodec::new(
        "qtrle",
        "QuickTime Animation (RLE) video",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["qtrle"],
        &["qtrle"],
    );

    pub const RPZA: VideoCodec = VideoCodec::new(
        "rpza",
        "QuickTime video (RPZA)",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["rpza"],
        &["rpza"],
    );

    pub const SMC: VideoCodec = VideoCodec::new(
        "smc",
        "QuickTime Graphics (SMC)",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["smc"],
        &["smc"],
    );

    // RealVideo codecs
    pub const RV10: VideoCodec = VideoCodec::new(
        "rv10",
        "RealVideo 1.0",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["rv10"],
        &["rv10"],
    );

    pub const RV20: VideoCodec = VideoCodec::new(
        "rv20",
        "RealVideo 2.0",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["rv20"],
        &["rv20"],
    );

    pub const RV30: VideoCodec = VideoCodec::new(
        "rv30",
        "RealVideo 3.0",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["rv30"],
        &[],
    );

    pub const RV40: VideoCodec = VideoCodec::new(
        "rv40",
        "RealVideo 4.0",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["rv40"],
        &[],
    );

    // Sorenson codecs
    pub const SVQ1: VideoCodec = VideoCodec::new(
        "svq1",
        "Sorenson Vector Quantizer 1 / Sorenson Video 1 / SVQ1",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["svq1"],
        &["svq1"],
    );

    pub const SVQ3: VideoCodec = VideoCodec::new(
        "svq3",
        "Sorenson Vector Quantizer 3 / Sorenson Video 3 / SVQ3",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["svq3"],
        &[],
    );

    // Flash Video
    pub const FLV1: VideoCodec = VideoCodec::new(
        "flv1",
        "FLV / Sorenson Spark / Sorenson H.263 (Flash Video)",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["flv"],
        &["flv"],
    );

    // H.263 variants
    pub const H263: VideoCodec = VideoCodec::new(
        "h263",
        "H.263 / H.263-1996, H.263+ / H.263-1998 / H.263 version 2",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["h263"],
        &["h263"],
    );

    pub const H263P: VideoCodec = VideoCodec::new(
        "h263p",
        "H.263+ / H.263-1998 / H.263 version 2",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["h263p"],
        &["h263p"],
    );

    pub const H261: VideoCodec = VideoCodec::new(
        "h261",
        "H.261",
        CodecSupport::decode_encode(),
        CodecType::Lossy,
        &["h261"],
        &["h261"],
    );

    // VP3 family
    pub const VP3: VideoCodec = VideoCodec::new(
        "vp3",
        "On2 VP3",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["vp3"],
        &[],
    );

    pub const VP4: VideoCodec = VideoCodec::new(
        "vp4",
        "On2 VP4",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["vp4"],
        &[],
    );

    pub const VP5: VideoCodec = VideoCodec::new(
        "vp5",
        "On2 VP5",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["vp5"],
        &[],
    );

    pub const VP6: VideoCodec = VideoCodec::new(
        "vp6",
        "On2 VP6",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["vp6"],
        &[],
    );

    pub const VP7: VideoCodec = VideoCodec::new(
        "vp7",
        "On2 VP7",
        CodecSupport::decode_only(),
        CodecType::Lossy,
        &["vp7"],
        &[],
    );

    // Utility codecs
    pub const VNULL: VideoCodec = VideoCodec::new(
        "vnull",
        "Null video codec",
        CodecSupport::decode_encode(),
        CodecType::Standard,
        &["vnull"],
        &["vnull"],
    );

    pub const WRAPPED_AVFRAME: VideoCodec = VideoCodec::new(
        "wrapped_avframe",
        "AVFrame to AVPacket passthrough",
        CodecSupport::decode_encode(),
        CodecType::Lossless,
        &["wrapped_avframe"],
        &["wrapped_avframe"],
    );

    // All supported video codecs in a single array
    pub const ALL: &[VideoCodec] = &[
        H264,
        HEVC,
        AV1,
        VP8,
        VP9,
        VVC,
        MPEG1VIDEO,
        MPEG2VIDEO,
        MPEG4,
        MSMPEG4V1,
        MSMPEG4V2,
        MSMPEG4V3,
        WMV1,
        WMV2,
        WMV3,
        VC1,
        PRORES,
        PRORES_RAW,
        MJPEG,
        HUFFYUV,
        FFVHUFF,
        FFV1,
        UTVIDEO,
        MAGICYUV,
        RAWVIDEO,
        V210,
        V308,
        V408,
        V410,
        THEORA,
        DIRAC,
        SNOW,
        CINEPAK,
        INDEO3,
        INDEO4,
        INDEO5,
        GIF,
        APNG,
        WEBP,
        AVIF,
        DNXHD,
        CFHD,
        DV,
        SPEEDHQ,
        FLASHSV,
        FLASHSV2,
        TSCC,
        TSCC2,
        QTRLE,
        RPZA,
        SMC,
        RV10,
        RV20,
        RV30,
        RV40,
        SVQ1,
        SVQ3,
        FLV1,
        H263,
        H263P,
        H261,
        VP3,
        VP4,
        VP5,
        VP6,
        VP7,
        VNULL,
        WRAPPED_AVFRAME,
    ];
}

pub struct VideoCodecRegistry {
    codecs_by_name: HashMap<String, &'static VideoCodec>,
    decoders_by_name: HashMap<String, &'static VideoCodec>,
    encoders_by_name: HashMap<String, &'static VideoCodec>,
}

impl VideoCodecRegistry {
    pub fn new() -> Self {
        let mut codecs_by_name = HashMap::new();
        let mut decoders_by_name = HashMap::new();
        let mut encoders_by_name = HashMap::new();

        for codec in video_codec::ALL {
            codecs_by_name.insert(codec.name.to_lowercase(), codec);

            // Map decoders to codec
            for &decoder in codec.decoders {
                decoders_by_name.insert(decoder.to_lowercase(), codec);
            }

            // Map encoders to codec
            for &encoder in codec.encoders {
                encoders_by_name.insert(encoder.to_lowercase(), codec);
            }
        }

        Self {
            codecs_by_name,
            decoders_by_name,
            encoders_by_name,
        }
    }

    pub fn get_codec_by_name(&self, name: &str) -> Option<&'static VideoCodec> {
        self.codecs_by_name.get(&name.to_lowercase()).copied()
    }

    pub fn get_codec_by_decoder(&self, decoder: &str) -> Option<&'static VideoCodec> {
        self.decoders_by_name.get(&decoder.to_lowercase()).copied()
    }

    pub fn get_codec_by_encoder(&self, encoder: &str) -> Option<&'static VideoCodec> {
        self.encoders_by_name.get(&encoder.to_lowercase()).copied()
    }

    pub fn is_decoder_available(&self, decoder: &str) -> bool {
        self.decoders_by_name.contains_key(&decoder.to_lowercase())
    }

    pub fn is_encoder_available(&self, encoder: &str) -> bool {
        self.encoders_by_name.contains_key(&encoder.to_lowercase())
    }

    pub fn get_codecs_with_encoding(&self) -> Vec<&'static VideoCodec> {
        video_codec::ALL
            .iter()
            .filter(|c| c.support.encoding)
            .collect()
    }

    pub fn get_codecs_with_decoding(&self) -> Vec<&'static VideoCodec> {
        video_codec::ALL
            .iter()
            .filter(|c| c.support.decoding)
            .collect()
    }

    pub fn get_lossless_codecs(&self) -> Vec<&'static VideoCodec> {
        video_codec::ALL
            .iter()
            .filter(|c| c.codec_type == CodecType::Lossless)
            .collect()
    }

    pub fn get_lossy_codecs(&self) -> Vec<&'static VideoCodec> {
        video_codec::ALL
            .iter()
            .filter(|c| c.codec_type == CodecType::Lossy)
            .collect()
    }

    pub fn get_intra_codecs(&self) -> Vec<&'static VideoCodec> {
        video_codec::ALL
            .iter()
            .filter(|c| c.codec_type == CodecType::Intra)
            .collect()
    }

    pub fn get_available_encoders(&self, codec_name: &str) -> Vec<&'static str> {
        if let Some(codec) = self.get_codec_by_name(codec_name) {
            codec.encoders.to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn get_available_decoders(&self, codec_name: &str) -> Vec<&'static str> {
        if let Some(codec) = self.get_codec_by_name(codec_name) {
            codec.decoders.to_vec()
        } else {
            Vec::new()
        }
    }
}

impl Default for VideoCodecRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Global registry instance
lazy_static::lazy_static! {
    pub static ref VIDEO_CODEC_REGISTRY: VideoCodecRegistry = VideoCodecRegistry::new();
}
