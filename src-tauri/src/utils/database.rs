use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Error as SqlxError, Row};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tasks {
    pub day: HashMap<String, Vec<String>>,
    pub week: HashMap<String, Vec<String>>,
    pub month: HashMap<String, Vec<String>>,
    pub year: HashMap<String, Vec<String>>,
}

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self, SqlxError> {
        let pool = SqlitePool::connect(db_path).await?;
        
        // Ensure the table exists
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS tasks (
                field TEXT NOT NULL,
                date TEXT NOT NULL,
                task TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn get_tasks(&self, field: &str, date: &str) -> Result<Vec<String>, SqlxError> {
        let rows = sqlx::query(
            "SELECT task FROM tasks WHERE field = ? AND date = ?"
        )
        .bind(field)
        .bind(date)
        .fetch_all(&self.pool)
        .await?;

        let tasks: Vec<String> = rows.into_iter().map(|row| row.get::<String, _>("task")).collect();

        Ok(tasks)
    }

    pub async fn add_task(&self, field: &str, date: &str, task: &str) -> Result<(), SqlxError> {
        sqlx::query(
            "INSERT INTO tasks (field, date, task) VALUES (?, ?, ?)"
        )
        .bind(field)
        .bind(date)
        .bind(task)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_task(&self, field: &str, date: &str, task: &str) -> Result<(), SqlxError> {
        sqlx::query(
            "DELETE FROM tasks WHERE field = ? AND date = ? AND task = ?"
        )
        .bind(field)
        .bind(date)
        .bind(task)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
