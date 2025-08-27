use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubTask {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckItem {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub priority: Priority,
    pub notes: Option<String>,
    pub subtasks: Vec<SubTask>,
    pub time_spent: u64, // in seconds
    pub timer_start: Option<DateTime<Utc>>,
    pub project: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub board: Option<String>,
    pub checklists: Vec<CheckItem>,
}