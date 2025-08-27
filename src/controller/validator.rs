use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, NaiveTime};
use crate::model::Priority;

/// Input validation utilities
pub struct InputValidator;

impl InputValidator {
    pub fn validate_priority(priority_str: &str) -> Result<Priority, String> {
        match priority_str.to_lowercase().as_str() {
            "high" | "h" => Ok(Priority::High),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "low" | "l" => Ok(Priority::Low),
            _ => Err(format!("Invalid priority '{}'. Use: high, medium, or low", priority_str)),
        }
    }

    pub fn parse_due_date(date_str: &str) -> Result<DateTime<Utc>, String> {
        // Try parsing as YYYY-MM-DD HH:MM first
        if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M") {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc));
        }

        // Try parsing as YYYY-MM-DD (set time to end of day)
        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            let datetime = date.and_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap());
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc));
        }

        // Try parsing as MM/DD/YYYY
        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
            let datetime = date.and_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap());
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc));
        }

        Err(format!("Invalid date format '{}'. Use YYYY-MM-DD or YYYY-MM-DD HH:MM", date_str))
    }

    pub fn validate_task_id(id: u32, tasks: &[crate::model::Task]) -> Result<(), String> {
        let exists = tasks.iter().any(|t| t.id == id && t.deleted_at.is_none());
        if exists {
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    pub fn validate_board_name(name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Board name cannot be empty".to_string());
        }

        if name.len() > 50 {
            return Err("Board name too long (max 50 characters)".to_string());
        }

        Ok(())
    }

    pub fn validate_tag(tag: &str) -> Result<(), String> {
        if tag.trim().is_empty() {
            return Err("Tag cannot be empty".to_string());
        }

        if tag.len() > 30 {
            return Err("Tag too long (max 30 characters)".to_string());
        }

        if tag.contains(',') {
            return Err("Tag cannot contain commas".to_string());
        }

        Ok(())
    }

    pub fn validate_description(description: &str) -> Result<(), String> {
        if description.trim().is_empty() {
            return Err("Task description cannot be empty".to_string());
        }

        if description.len() > 200 {
            return Err("Task description too long (max 200 characters)".to_string());
        }

        Ok(())
    }
}