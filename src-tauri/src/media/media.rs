use crate::media::Resolution;

pub fn calculate_resize_dimensions(original: &Resolution, min_pixel_count: &u32) -> Resolution {
    let min_pixels = *min_pixel_count;

    let (new_width, new_height) = if original.width < original.height {
        // Portrait: width is the constraining dimension
        let width = min_pixels;
        let height = (min_pixels * original.height + original.width / 2) / original.width;
        (width, height)
    } else {
        // Landscape: height is the constraining dimension
        let height = min_pixels;
        let width = (min_pixels * original.width + original.height / 2) / original.height;
        (width, height)
    };

    Resolution {
        width: new_width,
        height: new_height,
    }
}

pub trait Media {
    type FileType;

    // Required methods that must be implemented by concrete types
    fn get_resolution(&self) -> &Resolution;
    fn get_file_size(&self) -> u64;
    fn get_file_type(&self) -> &Self::FileType;
    fn set_resolution(&mut self, resolution: Resolution);

    /// Calculate the aspect ration of the media file by using the original resolution
    fn calculate_aspect_ratio(&self) -> f64 {
        let resolution = self.get_resolution();
        resolution.width as f64 / resolution.height as f64
    }

    fn resize_dimensions(&mut self, min_pixel_count: &u32) {
        let new_resolution = calculate_resize_dimensions(self.get_resolution(), min_pixel_count);
        self.set_resolution(new_resolution);
    }
}
