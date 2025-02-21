use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct FileData {
    pub day: Value,
    pub week: Value,
    pub month: Value,
    pub year: Value,
}

impl FileData {
    pub fn new() -> Self {
        FileData {
            day: json!({}),
            week: json!({}),
            month: json!({}),
            year: json!({}),
        }
    }

    pub fn update_field(&mut self, field_name: &str, updated_value: Value) -> Result<(), String> {
        match field_name {
            "day" => self.day = updated_value,
            "week" => self.week = updated_value,
            "month" => self.month = updated_value,
            "year" => self.year = updated_value,
            _ => return Err("Invalid field name".to_string()),
        }
        Ok(())
    }
}

impl From<Value> for FileData {
    fn from(value: Value) -> Self {
        FileData {
            day: value.get("day").cloned().unwrap_or(json!({})),
            week: value.get("week").cloned().unwrap_or(json!({})),
            month: value.get("month").cloned().unwrap_or(json!({})),
            year: value.get("year").cloned().unwrap_or(json!({})),
        }
    }
}

pub trait GetField {
    fn get_field(&self, field_name: &str) -> Option<&Value>;
}

impl GetField for FileData {
    fn get_field(&self, field_name: &str) -> Option<&Value> {
        match field_name {
            "day" => Some(&self.day),
            "week" => Some(&self.week),
            "month" => Some(&self.month),
            "year" => Some(&self.year),
            _ => None,
        }
    }
}
