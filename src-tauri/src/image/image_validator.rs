use crate::image::image_formats::IMAGE_FORMAT_REGISTRY;
use crate::shared::media_validator::MediaValidator;
use crate::ImageSettings;
use std::path::Path;

pub struct ImageSettingsValidator<'a> {
    settings: &'a ImageSettings,
}

impl<'a> ImageSettingsValidator<'a> {
    pub fn new(settings: &'a ImageSettings) -> Self {
        Self { settings }
    }
}

impl<'a> MediaValidator for ImageSettingsValidator<'a> {
    fn is_supported_extension(path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            IMAGE_FORMAT_REGISTRY.is_supported_for_reading(extension)
        } else {
            false
        }
    }

    fn get_target_extension(&self) -> &str {
        &self.settings.format
    }

    fn should_overwrite_existing(&self) -> bool {
        self.settings.overwrite_existing_files_output_directory
    }

    fn should_keep_folder_structure(&self) -> bool {
        self.settings
            .keep_child_folders_structure_in_output_directory
    }
}
