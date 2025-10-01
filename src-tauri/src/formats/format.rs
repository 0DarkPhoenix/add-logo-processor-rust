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
