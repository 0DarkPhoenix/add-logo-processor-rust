#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatSupport {
    pub demuxing: bool, // Can read/decode this format
    pub muxing: bool,   // Can write/encode this format
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
