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
