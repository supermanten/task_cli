use serde_json;
use std::fs;
use std::path::Path;
use super::task::Task;

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: load_tasks(),
        }
    }

    pub fn add_task(&mut self, description: String) {
        let id = self.tasks.len() as u32 + 1;
        self.tasks.push(Task { id, description, done: false });
    }

    pub fn mark_done(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            true
        } else {
            false
        }
    }

    pub fn delete_task(&mut self, id: u32) -> bool {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            true
        } else {
            false
        }
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