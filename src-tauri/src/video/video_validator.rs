use crate::shared::media_validator::MediaValidator;
use crate::video::video_formats::VIDEO_FORMAT_REGISTRY;
use crate::VideoSettings;
use std::path::Path;

pub struct VideoSettingsValidator<'a> {
    settings: &'a VideoSettings,
}

impl<'a> VideoSettingsValidator<'a> {
    pub fn new(settings: &'a VideoSettings) -> Self {
        Self { settings }
    }
}

impl<'a> MediaValidator for VideoSettingsValidator<'a> {
    fn is_supported_extension(path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            VIDEO_FORMAT_REGISTRY.is_supported_for_reading(extension)
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
