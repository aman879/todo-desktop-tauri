use std::{fs::File, io::{Read, Write}, path::PathBuf};
use serde_json::{self, Value};
use tauri::{AppHandle, Manager};
use crate::models::file_data::FileData;

const FILE_NAME: &str = "data.json";

pub fn get_json_path(app_handle: &AppHandle) -> PathBuf {
    app_handle.path().app_data_dir().expect("Failed to get app dir").join(FILE_NAME)
}

pub async fn read_json(app: &AppHandle) -> Result<FileData, String> {
    let path = get_json_path(&app);
    if path.exists() {
        let mut file = File::open(&path).map_err(|e| format!("Error: {}", e))?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| format!("Error: {}", e))?;

        let json_value: Value = serde_json::from_str(&content)
            .map_err(|e| format!("Error parsing JSON: {}", e))?;

        Ok(FileData::from(json_value))
    } else {
        Err("File not found".to_string())
    }
}

pub async fn write_json(app: &AppHandle, file_data: &FileData) -> Result<(), String> {
    let path = get_json_path(&app);
    let json_string = serde_json::to_string_pretty(&file_data)
        .map_err(|e| format!("Error serializing JSON: {}", e))?;

    let mut file = File::create(&path)
        .map_err(|e| format!("Error creating file: {}", e))?;

    file.write_all(json_string.as_bytes())
        .map_err(|e| format!("Error writing to file: {}", e))?;
    Ok(())
}
