use serde_json::Value;
use tauri::{AppHandle, command};
use crate::models::file_data::{FileData, GetField};
use crate::utils::json_handler::{read_json, write_json};

#[command]
pub fn debug_terminal(variable: String, value: Value) {
    println!("{:?} : {:?}", variable, value);
}

#[command]
pub async fn get_task(app: AppHandle, field: Option<String>, date: Option<String>) -> Result<Value, String> {
    let value: FileData = read_json(&app).await?;
    if let (Some(field_name), Some(date_key)) = (field, date) {
        if let Some(data) = value.get_field(&field_name) {
            if let Some(tasks) = data.get(&date_key) {
                return Ok(tasks.clone());
            }
        } else {
            return Err(format!("Field {} not found", field_name));
        }
    }
    Err("Field is not correct".to_string())
}

#[command]
pub async fn delete_task(app: AppHandle, field: String, date: String, task: String) -> Result<(), String> {
    let mut file_data: FileData = read_json(&app).await?;

    if let Some(field_name) = file_data.get_field(&field) {
        let mut updated_value = field_name.clone();

        if let Some(obj) = updated_value.as_object_mut() {
            if let Some(array) = obj.get_mut(&date).and_then(|v| v.as_array_mut()) {
                if let Some(index) = array.iter().position(|t| t.as_str() == Some(&task)) {
                    array.remove(index);
                }
            }
        }

        file_data.update_field(&field, updated_value)?;

        write_json(&app, &file_data).await?;
    }

    Ok(())
}

#[command]
pub async fn add_task(app: AppHandle, field: String, date: String, task: String) -> Result<(), String> {
    let mut file_data: FileData = read_json(&app).await?;

    if let Some(field_value) = file_data.get_field(&field) {
        let mut updated_value = field_value.clone();

        if let Some(obj) = updated_value.as_object_mut() {
            let entry = obj.entry(date).or_insert_with(|| Value::Array(vec![]));
            if let Some(array) = entry.as_array_mut() {
                array.push(Value::String(task));
            }
        }

        file_data.update_field(&field, updated_value)?;

        write_json(&app, &file_data).await?;
    }

    Ok(())
}
