// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use add_logo_processor_lib::Corner;
use add_logo_processor_lib::{AppConfig, ImageSettings, VideoSettings};
use ts_rs::TS;

fn main() {
    // Generate TypeScript bindings
    #[cfg(debug_assertions)]
    {
        AppConfig::export().expect("Failed to export AppConfig types");
        ImageSettings::export().expect("Failed to export ImageSettings types");
        VideoSettings::export().expect("Failed to export VideoSettings types");
        Corner::export().expect("Failed to export Corner types");
    }

    add_logo_processor_lib::run()
}
