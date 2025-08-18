use rayon::prelude::*;
use std::{error::Error, fs::read_dir, path::Path};

use crate::{
    handlers::handle_logos,
    media::{Logo, Media, Resolution, Video},
    processors::process_video,
    utils::config::VideoSettings,
};

pub fn handle_videos(video_settings: &VideoSettings) -> Result<(), Box<dyn Error + Send + Sync>> {
    let input_directory = &video_settings.input_directory;
    let output_directory = &video_settings.output_directory;

    let mut video_list = Vec::new();

    read_videos_in_input_directory(video_settings, input_directory, &mut video_list)?;

    sort_list_by_file_size(&mut video_list);

    determ_resized_resolution_per_video(video_settings, &mut video_list);

    let logo_list = process_logos_for_video_resolutions(video_settings, &video_list)?;

    process_videos_from_video_list(output_directory, video_list, logo_list)?;

    Ok(())
}

/// Determine the new resized dimensions for each image in the image list
fn determ_resized_resolution_per_video(
    video_settings: &VideoSettings,
    video_list: &mut Vec<Video>,
) {
    for video in video_list {
        video.resize_dimensions(video_settings.min_pixel_count);
    }
}

/// Process the videos from the video list in parallel
fn process_videos_from_video_list(
    output_directory: &Path,
    video_list: Vec<Video>,
    logo_list: Option<Vec<Logo>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    video_list
        .par_iter()
        .try_for_each(|video| -> Result<(), Box<dyn Error + Send + Sync>> {
            let logo: Option<&Logo> = if let Some(ref logo_list) = logo_list {
                logo_list
                    .iter()
                    .find(|logo| logo.compatible_image_resolution == video.resolution)
            } else {
                None
            };

            if logo.is_none() && logo_list.is_some() {
                return Err(format!(
                    "No logo found for the given video resolution: {}",
                    video.resolution
                )
                .into());
            }

            process_video(video, logo, output_directory).map_err(
                |e| -> Box<dyn Error + Send + Sync> {
                    format!("Failed to process video: {}", e).into()
                },
            )
        })?;
    Ok(())
}

fn process_logos_for_video_resolutions(
    video_settings: &VideoSettings,
    video_list: &Vec<Video>,
) -> Result<Option<Vec<Logo>>, Box<dyn Error + Send + Sync>> {
    let logo_list: Option<Vec<Logo>> = if video_settings.add_logo {
        // Make a hashset of all the unique resolutions of the Videos
        let mut unique_resolutions = std::collections::HashSet::new();
        for video in video_list {
            unique_resolutions.insert(video.resolution.clone());
        }
        let unique_resolutions: Vec<Resolution> = unique_resolutions.into_iter().collect();

        // Create a vector to store Logo structs for each unique resolution
        let logos = handle_logos(video_settings, unique_resolutions)?;
        Some(logos)
    } else {
        None
    };
    Ok(logo_list)
}

/// Reads all videos in the input directory, and adds them to the video list
fn read_videos_in_input_directory(
    video_settings: &VideoSettings,
    input_directory: &Path,
    video_list: &mut Vec<Video>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _: () = if video_settings.search_child_folders {
        read_videos_recursive(input_directory, video_list)?;
    } else {
        // Non-recursive search (only search videos in current directory)
        for entry in read_dir(input_directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Ok(video) = Video::new(path) {
                    video_list.push(video);
                }
            }
        }
    };
    Ok(())
}

/// Recursively read all videos from child directories
fn read_videos_recursive(
    dir: &Path,
    videos: &mut Vec<Video>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively search subdirectories
            read_videos_recursive(&path, videos)?;
        } else if path.is_file() {
            if let Ok(video) = Video::new(path) {
                videos.push(video);
            }
        }
    }
    Ok(())
}

/// Sorts the video list by file size in descending order (largest to smallest)
fn sort_list_by_file_size(video_list: &mut [Video]) {
    video_list.sort_by(|a, b| b.file_size.cmp(&a.file_size));
}
