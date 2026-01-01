use std::collections::HashMap;

use crate::shared::ffmpeg_structs::{Format, FormatSupport};

pub type ImageFormat = Format;

pub mod image_format {
    use super::{FormatSupport, ImageFormat};

    pub const PNG: ImageFormat = ImageFormat::new(
        "png",
        &["png"],
        FormatSupport::read_write(),
        "Portable Network Graphics",
    );

    pub const JPEG: ImageFormat = ImageFormat::new(
        "jpeg",
        &["jpg", "jpeg"],
        FormatSupport::read_write(),
        "Joint Photographic Experts Group",
    );

    pub const WEBP: ImageFormat = ImageFormat::new(
        "webp",
        &["webp"],
        FormatSupport::read_write(),
        "WebP Image Format",
    );

    pub const BMP: ImageFormat = ImageFormat::new(
        "bmp",
        &["bmp"],
        FormatSupport::read_write(),
        "Bitmap Image File",
    );

    pub const GIF: ImageFormat = ImageFormat::new(
        "gif",
        &["gif"],
        FormatSupport::read_write(),
        "Graphics Interchange Format",
    );

    pub const TIFF: ImageFormat = ImageFormat::new(
        "tiff",
        &["tiff", "tif"],
        FormatSupport::read_write(),
        "Tagged Image File Format",
    );

    pub const ICO: ImageFormat =
        ImageFormat::new("ico", &["ico"], FormatSupport::read_write(), "Windows Icon");

    pub const PNM: ImageFormat = ImageFormat::new(
        "pnm",
        &["pnm"],
        FormatSupport::read_write(),
        "Portable Anymap",
    );

    pub const TGA: ImageFormat = ImageFormat::new(
        "tga",
        &["tga"],
        FormatSupport::read_write(),
        "Truevision TGA",
    );

    pub const HDR: ImageFormat = ImageFormat::new(
        "hdr",
        &["hdr"],
        FormatSupport::read_only(),
        "High Dynamic Range",
    );

    pub const EXR: ImageFormat =
        ImageFormat::new("exr", &["exr"], FormatSupport::read_write(), "OpenEXR");

    pub const AVIF: ImageFormat = ImageFormat::new(
        "avif",
        &["avif"],
        FormatSupport::read_write(),
        "AV1 Image File Format",
    );

    pub const QOI: ImageFormat = ImageFormat::new(
        "qoi",
        &["qoi"],
        FormatSupport::read_write(),
        "Quite OK Image Format",
    );

    pub const APNG: ImageFormat = ImageFormat::new(
        "apng",
        &["apng"],
        FormatSupport::read_only(),
        "Animated Portable Network Graphics",
    );

    pub const PAM: ImageFormat = ImageFormat::new(
        "pam",
        &["pam"],
        FormatSupport::read_write(),
        "Portable Arbitrary Map",
    );

    pub const PBM: ImageFormat = ImageFormat::new(
        "pbm",
        &["pbm"],
        FormatSupport::read_write(),
        "Portable Bitmap",
    );

    pub const PCX: ImageFormat = ImageFormat::new(
        "pcx",
        &["pcx"],
        FormatSupport::read_write(),
        "PC Paintbrush",
    );

    pub const PGM: ImageFormat = ImageFormat::new(
        "pgm",
        &["pgm"],
        FormatSupport::read_write(),
        "Portable Graymap",
    );

    pub const PPM: ImageFormat = ImageFormat::new(
        "ppm",
        &["ppm"],
        FormatSupport::read_write(),
        "Portable Pixmap",
    );

    pub const PSD: ImageFormat = ImageFormat::new(
        "psd",
        &["psd"],
        FormatSupport::read_only(),
        "Adobe Photoshop Document",
    );

    pub const SGI: ImageFormat = ImageFormat::new(
        "sgi",
        &["sgi"],
        FormatSupport::read_write(),
        "Silicon Graphics Image",
    );

    pub const SVG: ImageFormat = ImageFormat::new(
        "svg",
        &["svg"],
        FormatSupport::unsupported(),
        "Scalable Vector Graphics",
    );

    pub const XBM: ImageFormat =
        ImageFormat::new("xbm", &["xbm"], FormatSupport::read_write(), "X11 Bitmap");

    pub const XPM: ImageFormat =
        ImageFormat::new("xpm", &["xpm"], FormatSupport::read_write(), "X11 Pixmap");

    pub const JPEGXL: ImageFormat =
        ImageFormat::new("jpegxl", &["jxl"], FormatSupport::read_write(), "JPEG XL");

    pub const DDS: ImageFormat = ImageFormat::new(
        "dds",
        &["dds"],
        FormatSupport::read_write(),
        "DirectDraw Surface",
    );

    // All supported formats in a single array
    pub const ALL: &[ImageFormat] = &[
        APNG, AVIF, BMP, DDS, EXR, GIF, HDR, ICO, JPEG, JPEGXL, PAM, PBM, PCX, PGM, PNG, PNM, PPM,
        PSD, QOI, SGI, SVG, TGA, TIFF, WEBP, XBM, XPM,
    ];
}

pub struct ImageFormatRegistry {
    formats_by_name: HashMap<String, &'static ImageFormat>,
    formats_by_extension: HashMap<String, &'static ImageFormat>,
}

impl ImageFormatRegistry {
    pub fn new() -> Self {
        let mut formats_by_name = HashMap::new();
        let mut formats_by_extension = HashMap::new();

        for format in image_format::ALL {
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

    pub fn get_format_by_name(&self, name: &str) -> Option<&'static ImageFormat> {
        self.formats_by_name.get(&name.to_lowercase()).copied()
    }

    pub fn get_format_by_extension(&self, extension: &str) -> Option<&'static ImageFormat> {
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

    pub fn get_writable_formats(&self) -> Vec<&'static ImageFormat> {
        image_format::ALL
            .iter()
            .filter(|f| f.support.muxing)
            .collect()
    }

    pub fn get_readable_formats(&self) -> Vec<&'static ImageFormat> {
        image_format::ALL
            .iter()
            .filter(|f| f.support.demuxing)
            .collect()
    }
}

impl Default for ImageFormatRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Global registry instance
lazy_static::lazy_static! {
    pub static ref IMAGE_FORMAT_REGISTRY: ImageFormatRegistry = ImageFormatRegistry::new();
}
