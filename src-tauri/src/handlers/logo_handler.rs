use crate::{
    media::{Corner, Logo, Resolution},
    processors::logo_processor::process_logo,
    utils::config::{ImageSettings, VideoSettings},
};
use rayon::prelude::*;
use std::{error::Error, path::PathBuf};

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
    let mut logos = Vec::new();
    for resolution in &unique_resolutions {
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
    logos
        .par_iter_mut()
        .try_for_each(|logo| -> Result<(), Box<dyn Error + Send + Sync>> {
            process_logo(logo).map_err(|e| -> Box<dyn Error + Send + Sync> {
                format!("Failed to process logo: {}", e).into()
            })
        })?;
    Ok(logos)
}
