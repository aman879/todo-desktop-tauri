use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tasks {
    pub day: HashMap<String, Vec<String>>,
    pub week: HashMap<String, Vec<String>>,
    pub month: HashMap<String, Vec<String>>,
    pub year: HashMap<String, Vec<String>>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks {
            day: HashMap::new(),
            week: HashMap::new(),
            month: HashMap::new(),
            year: HashMap::new(),
        }
    }
    
    pub fn add_task(&mut self, category: &str, date: &str, task: &str) -> Result<(), String> {
        let entry = match category {
            "day" => &mut self.day,
            "week" => &mut self.week,
            "month" => &mut self.month,
            "year" => &mut self.year,
            _ => return Err("Invalid category".to_string()),
        };
        entry
            .entry(date.to_string())
            .or_default()
            .push(task.to_string());

        Ok(())
    }

    pub fn remove_task(&mut self, category: &str, date: &str, task: &str) -> Result<(), String> {
        let entry = match category {
            "day" => &mut self.day,
            "week" => &mut self.week,
            "month" => &mut self.month,
            "year" => &mut self.year,
            _ => return Err("Invalid category".to_string()),
        };

        if let Some(tasks) = entry.get_mut(date) {
            tasks.retain(|t| t != task);
            if tasks.is_empty() {
                entry.remove(date);
            }
        }

        Ok(())
    }

    pub fn get_tasks(&self, category: &str, date: &str) -> Result<Option<Vec<String>>, String> {
        let result = match category {
            "day" => self.day.get(date).cloned(),
            "week" => self.week.get(date).cloned(),
            "month" => self.month.get(date).cloned(),
            "year" => self.year.get(date).cloned(),
            _ => return Err("Invalid category".to_string()),
        };
        Ok(result)
    }
}
