use crate::media::Resolution;

pub fn calculate_resize_dimensions(original: &Resolution, min_pixel_count: &u32) -> Resolution {
    let aspect_ratio = original.width as f64 / original.height as f64;

    let (new_width, new_height) = if original.width < original.height {
        let width = *min_pixel_count as f64;
        let height = width / aspect_ratio;
        (width, height)
    } else {
        let height = *min_pixel_count as f64;
        let width = height * aspect_ratio;
        (width, height)
    };

    Resolution {
        width: new_width as u32,
        height: new_height as u32,
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
