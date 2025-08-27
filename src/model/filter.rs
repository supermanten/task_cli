use chrono::{DateTime, Utc, Duration};
use super::task::{Task, Priority};

/// Filtering and searching operations
pub struct TaskFilter;

impl TaskFilter {
    pub fn filter_by_priority(tasks: &[Task], priority: Priority) -> Vec<&Task> {
        tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.priority == priority)
            .collect()
    }

    pub fn filter_by_project(tasks: &[Task], project: &str) -> Vec<&Task> {
        tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.project.as_ref() == Some(&project.to_string()))
            .collect()
    }

    pub fn search_by_description(tasks: &[Task], query: &str) -> Vec<&Task> {
        tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.description.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn filter_by_due_date(tasks: &[Task], filter: &str) -> Vec<&Task> {
        let now = Utc::now();
        match filter {
            "today" => tasks.iter()
                .filter(|t| {
                    t.deleted_at.is_none() &&
                    t.due_date.map_or(false, |due| {
                        due.date() == now.date()
                    })
                })
                .collect(),
            "week" => tasks.iter()
                .filter(|t| {
                    t.deleted_at.is_none() &&
                    t.due_date.map_or(false, |due| {
                        due.signed_duration_since(now).num_days() <= 7 &&
                        due.signed_duration_since(now).num_days() >= 0
                    })
                })
                .collect(),
            "overdue" => tasks.iter()
                .filter(|t| {
                    t.deleted_at.is_none() &&
                    t.due_date.map_or(false, |due| due < now)
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    pub fn filter_by_tag(tasks: &[Task], tag: &str) -> Vec<&Task> {
        tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.tags.contains(&tag.to_string()))
            .collect()
    }

    pub fn filter_by_board(tasks: &[Task], board: &str) -> Vec<&Task> {
        tasks.iter()
            .filter(|t| t.deleted_at.is_none() && t.board.as_ref() == Some(&board.to_string()))
            .collect()
    }

    pub fn get_active_tasks(tasks: &[Task]) -> Vec<&Task> {
        tasks.iter().filter(|t| t.deleted_at.is_none()).collect()
    }

    pub fn get_calendar_view(tasks: &[Task]) -> Vec<(String, Vec<&Task>)> {
        let mut calendar = std::collections::HashMap::new();

        for task in tasks {
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

    pub fn get_timeline_view(tasks: &[Task]) -> Vec<&Task> {
        let mut tasks = Self::get_active_tasks(tasks);
        tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        tasks
    }

    pub fn get_focus_view(tasks: &[Task], focus: &str) -> Vec<&Task> {
        tasks.iter()
            .filter(|t| {
                t.deleted_at.is_none() &&
                (t.tags.contains(&focus.to_string()) ||
                 t.project.as_ref() == Some(&focus.to_string()) ||
                 t.board.as_ref() == Some(&focus.to_string()))
            })
            .collect()
    }
}