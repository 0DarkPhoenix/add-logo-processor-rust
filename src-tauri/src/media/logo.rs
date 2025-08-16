use std::{error::Error, path::PathBuf};

use crate::media::{Corner, Position, Resolution};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo {
    pub file_path: PathBuf,
    pub resolution: Resolution,
    pub compatible_image_resolution: Resolution,
    pub position: Position,
}

impl Logo {
    pub fn new(
        file_path: PathBuf,
        scale: u32,
        corner: Corner,
        x_offset_scale: i32,
        y_offset_scale: i32,
        compatible_image_resolution: Resolution,
    ) -> Result<Self, Box<dyn Error>> {
        let resolution = transform_resolution_with_scale(&compatible_image_resolution, scale);

        let position = calculate_position(
            corner,
            &compatible_image_resolution,
            &resolution,
            x_offset_scale,
            y_offset_scale,
        );

        Ok(Self {
            file_path,
            resolution,
            compatible_image_resolution,
            position,
        })
    }
}

fn calculate_position(
    corner: Corner,
    image_resolution: &Resolution,
    logo_resolution: &Resolution,
    x_offset_scale: i32,
    y_offset_scale: i32,
) -> Position {
    let x_offset = (image_resolution.width as f64 * x_offset_scale as f64 / 100.0) as i32;
    let y_offset = (image_resolution.height as f64 * y_offset_scale as f64 / 100.0) as i32;

    let (base_x, base_y, x_direction, y_direction) = match corner {
        Corner::TopLeft => (
            0, // x position origin
            0, // y position origin
            1, // Move right
            1, // Move down
        ),
        Corner::TopRight => (
            image_resolution.width as i32 - logo_resolution.width as i32,
            0,
            -1, // Move left
            1,  // Move down
        ),
        Corner::BottomLeft => (
            0,
            image_resolution.height as i32 - logo_resolution.height as i32,
            1,  // Move right
            -1, // Move up
        ),
        Corner::BottomRight => (
            image_resolution.width as i32 - logo_resolution.width as i32,
            image_resolution.height as i32 - logo_resolution.height as i32,
            -1, // Move left
            -1, // Move up
        ),
    };

    let final_x = (base_x + x_offset * x_direction)
        .max(0)
        .min(image_resolution.width as i32 - logo_resolution.width as i32)
        .max(0) as u32;

    let final_y = (base_y + y_offset * y_direction)
        .max(0)
        .min(image_resolution.height as i32 - logo_resolution.height as i32)
        .max(0) as u32;

    Position {
        x: final_x,
        y: final_y,
    }
}

fn transform_resolution_with_scale(resolution: &Resolution, scale: u32) -> Resolution {
    let decimal_scale = scale as f64 / 100.0;
    Resolution {
        width: (resolution.width as f64 * decimal_scale) as u32,
        height: (resolution.height as f64 * decimal_scale) as u32,
    }
}
