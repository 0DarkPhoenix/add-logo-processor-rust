use remove_dir_all::remove_dir_all;
use std::fs::{read_dir, remove_file};
use std::{
    error::Error,
    fs::{create_dir_all, metadata},
    path::{Path, PathBuf},
};

use crate::handlers::process_handler::check_cancelled;

pub fn read_file_size(file_path: &PathBuf) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let metadata = metadata(file_path)?;
    Ok(metadata.len())
}

pub fn read_file_type(file_path: &Path) -> String {
    file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_lowercase()
}

/// Clear all files and folders in the folder from the specified path.
///
/// This function clears the contents of a folder without deleting the folder itself,
/// which is significantly faster than deleting and recreating the directory.
pub fn clear_and_create_folder(folder_path: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
    if folder_path.exists() {
        // Clear contents instead of deleting the directory
        clear_directory_contents(folder_path)?;
    } else {
        create_dir_all(folder_path)?;
    }

    Ok(())
}

/// Recursively clear all contents of a directory without deleting the directory itself
fn clear_directory_contents(dir_path: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
    for entry in read_dir(dir_path)? {
        check_cancelled()?;

        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            remove_dir_all(&path)?;
        } else {
            remove_file(&path)?;
        }
    }
    Ok(())
}

/// Extract the relative path by comparing the file path to the base path
pub fn get_relative_path(
    base_directory: &Path,
    file_path: &Path,
) -> Result<PathBuf, Box<dyn Error>> {
    let relative_path = file_path.strip_prefix(base_directory)?;
    Ok(relative_path.to_path_buf())
}
