use colored::*;

/// Text formatting utilities
pub struct TextFormatter;

impl TextFormatter {
    pub fn format_time(seconds: u64) -> String {
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

    pub fn format_priority(priority: &super::super::model::Priority) -> colored::ColoredString {
        match priority {
            super::super::model::Priority::High => "High".red().bold(),
            super::super::model::Priority::Medium => "Medium".yellow(),
            super::super::model::Priority::Low => "Low".green(),
        }
    }

    pub fn format_status(done: bool) -> colored::ColoredString {
        if done {
            "✓".green()
        } else {
            "✗".yellow()
        }
    }

    pub fn format_checklist_status(done: bool) -> colored::ColoredString {
        if done {
            "☑".green()
        } else {
            "☐".yellow()
        }
    }

    pub fn truncate_text(text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }

    pub fn format_tags(tags: &[String]) -> String {
        if tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", tags.join(", "))
        }
    }

    pub fn format_project(project: Option<&String>) -> String {
        project.map(|p| p.clone()).unwrap_or_else(|| "-".to_string())
    }

    pub fn format_due_date(due_date: Option<&chrono::DateTime<chrono::Utc>>) -> String {
        due_date.map(|d| d.format("%m-%d").to_string()).unwrap_or_else(|| "-".to_string())
    }
}