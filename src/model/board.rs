use chrono::{DateTime, Utc};
use serde_json;
use std::fs;
use std::path::Path;
use super::task::{Task, Priority, SubTask, CheckItem};

/// Board management operations
pub struct BoardManager {
    pub boards: Vec<String>,
}

impl BoardManager {
    pub fn new() -> Self {
        Self {
            boards: load_boards(),
        }
    }

    pub fn initialize_default_boards(&mut self) {
        if self.boards.is_empty() {
            self.boards = vec!["Todo".to_string(), "In Progress".to_string(), "Done".to_string()];
            save_boards(&self.boards);
        }
    }

    pub fn create_board(&mut self, name: String) {
        if !self.boards.contains(&name) {
            self.boards.push(name);
            save_boards(&self.boards);
        }
    }

    pub fn get_board_list(&self) -> &[String] {
        &self.boards
    }

    pub fn board_exists(&self, name: &str) -> bool {
        self.boards.contains(&name.to_string())
    }

    pub fn get_board_view(&self, tasks: &[Task]) -> Vec<(String, Vec<&Task>)> {
        let mut board_view = std::collections::HashMap::new();

        for board in &self.boards {
            board_view.insert(board.clone(), Vec::new());
        }

        for task in tasks {
            if task.deleted_at.is_some() { continue; }
            let board = task.board.as_ref().unwrap_or(&"Todo".to_string()).clone();
            board_view.entry(board).or_insert(Vec::new()).push(task);
        }

        let mut sorted_view: Vec<_> = board_view.into_iter().collect();
        sorted_view.sort_by(|a, b| a.0.cmp(&b.0));
        sorted_view
    }
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