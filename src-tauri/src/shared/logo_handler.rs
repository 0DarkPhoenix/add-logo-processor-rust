use rayon::prelude::*;
use std::{error::Error, path::PathBuf};

use crate::{
    shared::{
        file_utils::clear_and_create_folder, logo_processor::process_logo, logo_structs::Logo,
        media_structs::Resolution, process_manager::check_process_cancelled,
    },
    Corner, ImageSettings, VideoSettings,
};

pub trait LogoSettings {
    fn logo_path(&self) -> &Option<PathBuf>;
    fn logo_scale(&self) -> u32;
    fn logo_corner(&self) -> Corner;
    fn logo_x_offset_scale(&self) -> i32;
    fn logo_y_offset_scale(&self) -> i32;
}

impl LogoSettings for ImageSettings {
    fn logo_path(&self) -> &Option<PathBuf> {
        &self.logo_path
    }
    fn logo_scale(&self) -> u32 {
        self.logo_scale
    }
    fn logo_corner(&self) -> Corner {
        self.logo_corner
    }
    fn logo_x_offset_scale(&self) -> i32 {
        self.logo_x_offset_scale
    }
    fn logo_y_offset_scale(&self) -> i32 {
        self.logo_y_offset_scale
    }
}

impl LogoSettings for VideoSettings {
    fn logo_path(&self) -> &Option<PathBuf> {
        &self.logo_path
    }
    fn logo_scale(&self) -> u32 {
        self.logo_scale
    }
    fn logo_corner(&self) -> Corner {
        self.logo_corner
    }
    fn logo_x_offset_scale(&self) -> i32 {
        self.logo_x_offset_scale
    }
    fn logo_y_offset_scale(&self) -> i32 {
        self.logo_y_offset_scale
    }
}

pub fn handle_logos<T: LogoSettings>(
    settings: &T,
    unique_resolutions: Vec<Resolution>,
) -> Result<Vec<Logo>, Box<dyn Error + Send + Sync>> {
    // Create a fixed folder structure in the application root
    let app_root = std::env::current_exe()?
        .parent()
        .ok_or("Failed to get application directory")?
        .to_path_buf();

    let output_directory = app_root.join("temp_processed_images");

    let _ = clear_and_create_folder(&output_directory);

    let mut logos = Vec::new();
    for resolution in &unique_resolutions {
        check_process_cancelled()?;

        let logo = Logo::new(
            settings
                .logo_path()
                .clone()
                .ok_or("Logo path is required")?,
            settings.logo_scale(),
            settings.logo_corner(),
            settings.logo_x_offset_scale(),
            settings.logo_y_offset_scale(),
            resolution.clone(),
        )
        .map_err(|e| -> Box<dyn Error + Send + Sync> {
            format!("Failed to create logo: {}", e).into()
        })?;
        logos.push(logo);
    }
    let output_dir_clone = output_directory.clone();
    logos
        .par_iter_mut()
        .try_for_each(|logo| -> Result<(), Box<dyn Error + Send + Sync>> {
            process_logo(logo, &output_dir_clone)
                .map_err(|e| format!("Failed to process logo: {}", e).into())
        })?;
    Ok(logos)
}
