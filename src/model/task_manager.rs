use chrono::Utc;
use serde_json;
use std::fs;
use std::path::Path;
use super::task::{Task, Priority, SubTask};

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: load_tasks(),
        }
    }

    pub fn add_task(&mut self, description: String, priority: Priority) {
        let id = self.tasks.len() as u32 + 1;
        let created_at = Utc::now();
        self.tasks.push(Task {
            id,
            description,
            done: false,
            created_at,
            deleted_at: None,
            priority,
            notes: None,
            subtasks: Vec::new(),
            time_spent: 0,
            timer_start: None,
        });
    }

    pub fn mark_done(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            task.done = true;
            true
        } else {
            false
        }
    }

    pub fn delete_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            task.deleted_at = Some(Utc::now());
            true
        } else {
            false
        }
    }

    pub fn add_note(&mut self, id: u32, note: String) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            task.notes = Some(note);
            true
        } else {
            false
        }
    }

    pub fn add_subtask(&mut self, id: u32, description: String) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            let subtask_id = task.subtasks.len() as u32 + 1;
            task.subtasks.push(SubTask {
                id: subtask_id,
                description,
                done: false,
            });
            true
        } else {
            false
        }
    }

    pub fn start_timer(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none() && t.timer_start.is_none()) {
            task.timer_start = Some(Utc::now());
            true
        } else {
            false
        }
    }

    pub fn stop_timer(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none() && t.timer_start.is_some()) {
            let start = task.timer_start.unwrap();
            let duration = Utc::now().signed_duration_since(start);
            task.time_spent += duration.num_seconds() as u64;
            task.timer_start = None;
            true
        } else {
            false
        }
    }

    pub fn get_active_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.deleted_at.is_none()).collect()
    }

    pub fn save(&self) {
        save_tasks(&self.tasks);
    }
}

fn load_tasks() -> Vec<Task> {
    let path = Path::new("tasks.json");
    if path.exists() {
        let data = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
}