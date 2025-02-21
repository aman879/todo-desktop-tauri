pub mod commands;
pub mod models;
pub mod utils;

use crate::models::file_data::FileData;
use crate::utils::json_handler::get_json_path;
use std::{fs::{self, File}, io::Write};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();
            let path = get_json_path(&app_handle);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("Failed to create app data directory");
            }

            if !path.exists() {
                let mut file = File::create(&path).expect("Failed to create JSON file");
                let default_data = FileData::new();
                let json_string = serde_json::to_string_pretty(&default_data)
                    .expect("Failed to serialize default FileData");

                file.write_all(json_string.as_bytes())
                    .expect("Failed to write to JSON file");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::tasks::debug_terminal,
            commands::tasks::get_task,
            commands::tasks::add_task,
            commands::tasks::delete_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
