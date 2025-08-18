use std::env;

fn main() {
    // Generate TypeScript bindings
    if env::var("PROFILE").unwrap() == "debug" {
        ts_rs::export_all!();
    }

    tauri_build::build()
}
