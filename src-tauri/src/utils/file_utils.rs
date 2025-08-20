use std::{
    error::Error,
    fs::{create_dir_all, metadata, remove_dir_all},
    path::{Path, PathBuf},
};

pub fn read_file_size(file_path: &PathBuf) -> Result<u64, Box<dyn Error>> {
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
/// Instead of deleting all entries inside a folder, this function deletes the folder itself when it exists.
/// Because the folder is (re-)created in the last step, it can also be used to create a folder when it doesn't exist yet.
pub fn clear_and_create_folder(folder_path: &Path) -> Result<(), Box<dyn Error>> {
    if folder_path.exists() {
        remove_dir_all(folder_path)?;
    }

    create_dir_all(folder_path)?;

    Ok(())
}

/// Extract the relative path by comparing the file path to the base path
pub fn get_relative_path(
    base_directory: &Path,
    file_path: &Path,
) -> Result<PathBuf, Box<dyn Error>> {
    dbg!(&base_directory);
    dbg!(&file_path);
    let relative_path = file_path.strip_prefix(base_directory)?;
    Ok(relative_path.to_path_buf())
}
