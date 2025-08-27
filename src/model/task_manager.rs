use chrono::{DateTime, Utc};
use serde_json;
use std::fs;
use std::path::Path;
use super::task::{Task, Priority, SubTask, CheckItem};

pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub boards: Vec<String>,
}

impl TaskManager {
    pub fn new() -> Self {
        let mut manager = Self {
            tasks: load_tasks(),
            boards: load_boards(),
        };
        manager.initialize_default_boards();
        manager
    }

    fn initialize_default_boards(&mut self) {
        if self.boards.is_empty() {
            self.boards = vec!["Todo".to_string(), "In Progress".to_string(), "Done".to_string()];
            save_boards(&self.boards);
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
            project: None,
            due_date: None,
            tags: Vec::new(),
            board: Some("Todo".to_string()),
            checklists: Vec::new(),
        });
    }

    pub fn add_task_with_options(&mut self, description: String, priority: Priority, project: Option<String>, due_date: Option<DateTime<Utc>>, tags: Vec<String>) {
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
            project,
            due_date,
            tags,
            board: Some("Todo".to_string()),
            checklists: Vec::new(),
        });
    }

    pub fn mark_done(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            task.done = true;
            // Move to Done board if not already there
            task.board = Some("Done".to_string());
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

    // Advanced filtering methods
    pub fn filter_by_priority(&self, priority: Priority) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.priority == priority)
            .collect()
    }

    pub fn filter_by_project(&self, project: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.project.as_ref() == Some(&project.to_string()))
            .collect()
    }

    pub fn search_by_description(&self, query: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.description.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn filter_by_due_date(&self, filter: &str) -> Vec<&Task> {
        let now = Utc::now();
        match filter {
            "today" => self.tasks.iter()
                .filter(|t| {
                    t.deleted_at.is_none() &&
                    t.due_date.map_or(false, |due| {
                        due.date_naive() == now.date_naive()
                    })
                })
                .collect(),
            "week" => self.tasks.iter()
                .filter(|t| {
                    t.deleted_at.is_none() &&
                    t.due_date.map_or(false, |due| {
                        due.signed_duration_since(now).num_days() <= 7 &&
                        due.signed_duration_since(now).num_days() >= 0
                    })
                })
                .collect(),
            "overdue" => self.tasks.iter()
                .filter(|t| {
                    t.deleted_at.is_none() &&
                    t.due_date.map_or(false, |due| due < now)
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.tags.contains(&tag.to_string()))
            .collect()
    }

    pub fn filter_by_board(&self, board: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.board.as_ref() == Some(&board.to_string()))
            .collect()
    }

    // Board management
    pub fn create_board(&mut self, name: String) {
        if !self.boards.contains(&name) {
            self.boards.push(name);
            save_boards(&self.boards);
        }
    }

    pub fn move_task_to_board(&mut self, id: u32, board: String) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            task.board = Some(board);
            true
        } else {
            false
        }
    }

    // Tag management
    pub fn add_tag(&mut self, id: u32, tag: String) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            if !task.tags.contains(&tag) {
                task.tags.push(tag);
            }
            true
        } else {
            false
        }
    }

    pub fn remove_tag(&mut self, id: u32, tag: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            task.tags.retain(|t| t != tag);
            true
        } else {
            false
        }
    }

    // Due date management
    pub fn set_due_date(&mut self, id: u32, due_date: DateTime<Utc>) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            task.due_date = Some(due_date);
            true
        } else {
            false
        }
    }

    // Checklist management
    pub fn add_checklist_item(&mut self, id: u32, description: String) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            let item_id = task.checklists.len() as u32 + 1;
            task.checklists.push(CheckItem {
                id: item_id,
                description,
                done: false,
            });
            true
        } else {
            false
        }
    }

    pub fn toggle_checklist_item(&mut self, task_id: u32, item_id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id && t.deleted_at.is_none()) {
            if let Some(item) = task.checklists.iter_mut().find(|i| i.id == item_id) {
                item.done = !item.done;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    // Project management
    pub fn set_project(&mut self, id: u32, project: String) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id && t.deleted_at.is_none()) {
            task.project = Some(project);
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

    // Export functionality
    pub fn export_csv(&self, filename: &str) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create(filename)?;

        writeln!(file, "ID,Description,Status,Priority,Project,Due Date,Tags,Board,Created At,Time Spent")?;

        for task in &self.tasks {
            if task.deleted_at.is_some() { continue; }

            let status = if task.done { "Done" } else { "Todo" };
            let priority = format!("{:?}", task.priority);
            let project = task.project.as_ref().map(|s| s.as_str()).unwrap_or("");
            let due_date = task.due_date.map(|d| d.format("%Y-%m-%d %H:%M").to_string()).unwrap_or_default();
            let tags = task.tags.join("; ");
            let board = task.board.as_ref().map(|s| s.as_str()).unwrap_or("");
            let created_at = task.created_at.format("%Y-%m-%d %H:%M");
            let time_spent = format_time(task.time_spent);

            writeln!(file, "{},{},{},{},{},{},{},{},{},{}",
                    task.id, task.description, status, priority, project, due_date, tags, board, created_at, time_spent)?;
        }

        Ok(())
    }

    pub fn export_json(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        std::fs::write(filename, json)?;
        Ok(())
    }

    // Advanced views
    pub fn get_calendar_view(&self) -> Vec<(String, Vec<&Task>)> {
        let mut calendar = std::collections::HashMap::new();

        for task in &self.tasks {
            if task.deleted_at.is_some() { continue; }
            if let Some(due_date) = task.due_date {
                let date_key = due_date.format("%Y-%m-%d").to_string();
                calendar.entry(date_key).or_insert(Vec::new()).push(task);
            }
        }

        let mut sorted_calendar: Vec<_> = calendar.into_iter().collect();
        sorted_calendar.sort_by(|a, b| a.0.cmp(&b.0));
        sorted_calendar
    }

    pub fn get_timeline_view(&self) -> Vec<&Task> {
        let mut tasks = self.get_active_tasks();
        tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        tasks
    }

    pub fn get_focus_view(&self, focus: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| {
                t.deleted_at.is_none() &&
                (t.tags.contains(&focus.to_string()) ||
                 t.project.as_ref() == Some(&focus.to_string()) ||
                 t.board.as_ref() == Some(&focus.to_string()))
            })
            .collect()
    }

    pub fn get_board_view(&self) -> Vec<(String, Vec<&Task>)> {
        let mut board_view = std::collections::HashMap::new();

        for board in &self.boards {
            board_view.insert(board.clone(), Vec::new());
        }

        for task in &self.tasks {
            if task.deleted_at.is_some() { continue; }
            let board = task.board.as_ref().unwrap_or(&"Todo".to_string()).clone();
            board_view.entry(board).or_insert(Vec::new()).push(task);
        }

        let mut sorted_view: Vec<_> = board_view.into_iter().collect();
        sorted_view.sort_by(|a, b| a.0.cmp(&b.0));
        sorted_view
    }

    pub fn save(&self) {
        save_tasks(&self.tasks);
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

fn load_tasks() -> Vec<Task> {
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

fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
}

fn load_boards() -> Vec<String> {
    let path = Path::new("boards.json");
    if path.exists() {
        let data = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_boards(boards: &[String]) {
    let data = serde_json::to_string(boards).unwrap();
    fs::write("boards.json", data).unwrap();
}