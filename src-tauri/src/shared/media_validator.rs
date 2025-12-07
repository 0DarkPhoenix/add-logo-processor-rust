use log::{error, info};
use rayon::prelude::*;
use std::error::Error;
use std::path::{Path, PathBuf};

use crate::shared::process_manager::check_process_cancelled;

/// Trait for media-specific validation logic
pub trait MediaValidator {
    /// Check if the file extension is supported for this media type
    fn is_supported_extension(path: &Path) -> bool;

    /// Get the target file extension based on settings
    fn get_target_extension(&self) -> &str;

    /// Check if existing files should be overwritten
    fn should_overwrite_existing(&self) -> bool;

    /// Check if child folder structure should be preserved
    fn should_keep_folder_structure(&self) -> bool;
}

/// Determine if a media file should be written to the output directory
pub fn should_write_to_output<V: MediaValidator>(
    path: &Path,
    input_directory: &Path,
    output_directory: &Path,
    validator: &V,
) -> bool {
    if validator.should_overwrite_existing() {
        return true;
    }

    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let target_extension = validator.get_target_extension();
    let target_filename = format!("{}.{}", file_stem, target_extension);

    if validator.should_keep_folder_structure() {
        if let Ok(relative_path) =
            crate::shared::file_utils::get_relative_path(input_directory, path)
        {
            let relative_dir_path = relative_path.parent().unwrap_or(Path::new(""));
            let target_output_path = output_directory
                .join(relative_dir_path)
                .join(target_filename);
            return !target_output_path.exists();
        }
    }

    let target_output_path = output_directory.join(target_filename);
    !target_output_path.exists()
}

/// Check if a path is a valid media file that should be processed
pub fn is_valid_media_path<V: MediaValidator>(
    path: &Path,
    input_directory: &Path,
    output_directory: &Path,
    validator: &V,
) -> bool {
    path.is_file()
        && V::is_supported_extension(path)
        && should_write_to_output(path, input_directory, output_directory, validator)
}

/// Filter paths to only include valid media files
pub fn filter_valid_media_paths<V: MediaValidator>(
    paths: impl Iterator<Item = PathBuf>,
    input_directory: &Path,
    output_directory: &Path,
    validator: &V,
) -> Vec<PathBuf> {
    paths
        .filter(|path| is_valid_media_path(path, input_directory, output_directory, validator))
        .collect()
}

/// Create media objects from paths in parallel
pub fn create_media_from_paths_parallel<T, F>(
    paths: &[PathBuf],
    constructor: F,
) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>
where
    T: Send,
    F: Fn(PathBuf) -> Result<T, Box<dyn Error + Send + Sync>> + Send + Sync,
{
    paths
        .par_iter()
        .filter_map(|path| {
            if let Err(e) = check_process_cancelled() {
                return Some(Err(e));
            }

            match constructor(path.clone()) {
                Ok(media) => Some(Ok(media)),
                Err(e) => {
                    error!("Failed to load media file {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect()
}

/// Sort media list by file size in descending order
pub fn sort_by_file_size<T>(media_list: &mut [T])
where
    T: crate::shared::media_structs::Media,
{
    media_list.sort_by_key(|b| std::cmp::Reverse(b.get_file_size()));
}

/// Recursively read media paths using jwalk
pub fn read_media_paths_recursive<V: MediaValidator>(
    directory: &Path,
    output_directory: &Path,
    validator: &V,
) -> Result<Vec<PathBuf>, Box<dyn Error + Send + Sync>> {
    let walk_start = std::time::Instant::now();

    let valid_paths: Result<Vec<PathBuf>, Box<dyn Error + Send + Sync>> =
        jwalk::WalkDir::new(directory)
            .skip_hidden(false)
            .into_iter()
            .filter_map(|entry| {
                if let Err(e) = check_process_cancelled() {
                    return Some(Err(e));
                }

                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => return None,
                };

                let path = entry.path();

                if !is_valid_media_path(&path, directory, output_directory, validator) {
                    return None;
                }

                Some(Ok(path))
            })
            .collect();

    let valid_paths = valid_paths?;

    info!(
        "Directory walk and filtering took: {:?}",
        walk_start.elapsed()
    );
    info!("Found {} valid media paths", valid_paths.len());

    Ok(valid_paths)
}
