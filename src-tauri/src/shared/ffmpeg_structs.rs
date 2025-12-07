use ffmpeg_sidecar::command::FfmpegCommand;

#[derive(Debug)]
pub struct FfmpegBatchCommand {
    pub command: FfmpegCommand,
    pub batch_size: usize,
}
/* -------------------------------------------------------------------------- */
/*                                   FORMAT                                   */
/* -------------------------------------------------------------------------- */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatSupport {
    pub demuxing: bool, // read/decode
    pub muxing: bool,   // write/encode
}

impl FormatSupport {
    pub const fn new(demuxing: bool, muxing: bool) -> Self {
        Self { demuxing, muxing }
    }

    pub const fn read_only() -> Self {
        Self::new(true, false)
    }

    pub const fn write_only() -> Self {
        Self::new(false, true)
    }

    pub const fn read_write() -> Self {
        Self::new(true, true)
    }

    pub const fn unsupported() -> Self {
        Self::new(false, false)
    }
}

#[derive(Debug, Clone)]
pub struct Format {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub support: FormatSupport,
    pub description: &'static str,
}

impl Format {
    pub const fn new(
        name: &'static str,
        extensions: &'static [&'static str],
        support: FormatSupport,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            extensions,
            support,
            description,
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                    CODEC                                   */
/* -------------------------------------------------------------------------- */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodecSupport {
    pub decoding: bool, // decode support
    pub encoding: bool, // encode support
}

impl CodecSupport {
    pub const fn new(decoding: bool, encoding: bool) -> Self {
        Self { decoding, encoding }
    }

    pub const fn decode_only() -> Self {
        Self::new(true, false)
    }

    pub const fn encode_only() -> Self {
        Self::new(false, true)
    }

    pub const fn decode_encode() -> Self {
        Self::new(true, true)
    }

    pub const fn unsupported() -> Self {
        Self::new(false, false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecType {
    Intra,    // Intra frame-only codec
    Lossy,    // Lossy compression
    Lossless, // Lossless compression
    Standard, // Standard codec (neither specifically lossy nor lossless)
}

#[derive(Debug, Clone)]
pub struct Codec {
    pub name: &'static str,
    pub long_name: &'static str,
    pub support: CodecSupport,
    pub codec_type: CodecType,
    pub decoders: &'static [&'static str],
    pub encoders: &'static [&'static str],
}

impl Codec {
    pub const fn new(
        name: &'static str,
        long_name: &'static str,
        support: CodecSupport,
        codec_type: CodecType,
        decoders: &'static [&'static str],
        encoders: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            long_name,
            support,
            codec_type,
            decoders,
            encoders,
        }
    }
}
