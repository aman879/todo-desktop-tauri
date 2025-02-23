use std::sync::Arc;
use tauri::{command, State};
use tokio::sync::Mutex;
use serde_json::Value;

use crate::utils::database::Database;

#[command]
pub fn debug_terminal(variable: String, value: Value) {
    println!("{:?} : {:?}", variable, value);
}

#[command]
pub async fn get_task(
    field: String, 
    date: String, 
    db: State<'_, Arc<Mutex<Database>>>
) -> Result<Vec<String>, String> {
    let db = db.lock().await;
    db.get_tasks(&field, &date).await.map_err(|e| e.to_string())
}

#[command]
pub async fn add_task(
    field: String, 
    date: String, 
    task: String, 
    db: State<'_, Arc<Mutex<Database>>>
) -> Result<(), String> {
    let db = db.lock().await;
    db.add_task(&field, &date, &task).await.map_err(|e| e.to_string())
}

#[command]
pub async fn delete_task(
    field: String, 
    date: String, 
    task: String, 
    db: State<'_, Arc<Mutex<Database>>>
) -> Result<(), String> {
    let db = db.lock().await;
    db.remove_task(&field, &date, &task).await.map_err(|e| e.to_string())
}
