use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use sqlx::Sqlite;
use sqlx::migrate::MigrateDatabase;

mod commands;
mod utils;

use commands::commands::{get_task, add_task, delete_task, debug_terminal};
use utils::database::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            let path = handle.path().app_data_dir().expect("Failed to get app dir").join("tasks.db");
            tauri::async_runtime::spawn(async move {
                let db_path = path.to_str().unwrap();

                if !Sqlite::database_exists(&db_path).await.unwrap_or(false) {
                    Sqlite::create_database(&db_path).await.unwrap();
                } else {
                    println!("database exists");
                }
                
                match Database::new(&db_path).await {
                    Ok(db) => {
                        handle.manage(Arc::new(Mutex::new(db)));
                        println!("Database initialized successfully!");
                    }
                    Err(e) => eprintln!("Failed to initialize database: {:?}", e),
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_task,
            add_task,
            delete_task,
            debug_terminal
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
