use chrono::{DateTime, Utc};
use serde_json;
use std::fs;
use std::path::Path;
use std::io::Write;
use super::task::{Task, Priority};

/// Data persistence operations
pub struct TaskStorage;

impl TaskStorage {
    pub fn load_tasks() -> Vec<Task> {
        let path = Path::new("tasks.json");
        if path.exists() {
            let data = fs::read_to_string(path).unwrap_or_default();
            match serde_json::from_str(&data) {
                Ok(tasks) => tasks,
                Err(_) => {
                    // If deserialization fails (due to schema changes), return empty vec
                    // This handles migration from old task format to new format
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    }

    pub fn save_tasks(tasks: &[Task]) {
        let data = serde_json::to_string(tasks).unwrap();
        fs::write("tasks.json", data).unwrap();
    }

    pub fn export_csv(tasks: &[Task], filename: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(filename)?;

        writeln!(file, "ID,Description,Status,Priority,Project,Due Date,Tags,Board,Created At,Time Spent")?;

        for task in tasks {
            if task.deleted_at.is_some() { continue; }

            let status = if task.done { "Done" } else { "Todo" };
            let priority = format!("{:?}", task.priority);
            let project = task.project.as_ref().unwrap_or(&"".to_string());
            let due_date = task.due_date.map(|d| d.format("%Y-%m-%d %H:%M").to_string()).unwrap_or_default();
            let tags = task.tags.join("; ");
            let board = task.board.as_ref().unwrap_or(&"".to_string());
            let created_at = task.created_at.format("%Y-%m-%d %H:%M");
            let time_spent = format_time(task.time_spent);

            writeln!(file, "{},{},{},{},{},{},{},{},{},{}",
                    task.id, task.description, status, priority, project, due_date, tags, board, created_at, time_spent)?;
        }

        Ok(())
    }

    pub fn export_json(tasks: &[Task], filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(tasks)?;
        std::fs::write(filename, json)?;
        Ok(())
    }
}

fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}