use std::env;

fn main() {
    // Generate TypeScript bindings
    ts_rs::export_all!();

    tauri_build::build()
}
