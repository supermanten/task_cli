use colored::*;
use crate::model::{Task, Priority};

pub fn display_tasks(tasks: &[&Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        println!("{:<5} {:<12} {:<15} {:<50} {:<20} {:<10}",
                 "ID", "Status", "Priority", "Description", "Created At", "Time Spent");
        println!("{}", "-".repeat(115));
        for task in tasks {
            let status = if task.done { "✓ Done".green() } else { "✗ Todo".yellow() };
            let priority = match task.priority {
                Priority::High => "High".red().bold(),
                Priority::Medium => "Medium".yellow(),
                Priority::Low => "Low".green(),
            };
            let created_at = task.created_at.format("%Y-%m-%d %H:%M");
            let time_spent = format_time(task.time_spent);
            let mut description = task.description.clone();
            if !task.subtasks.is_empty() {
                let done_count = task.subtasks.iter().filter(|st| st.done).count();
                description = format!("{} [{}/{}]", description, done_count, task.subtasks.len());
            }
            println!("{:<5} {:<12} {:<15} {:<50} {:<20} {:<10}",
                     task.id.to_string().bold(),
                     status,
                     priority,
                     description,
                     created_at,
                     time_spent);
            if let Some(note) = &task.notes {
                println!("      Note: {}", note.dimmed());
            }
            if !task.subtasks.is_empty() {
                for subtask in &task.subtasks {
                    let sub_status = if subtask.done { "  ✓".green() } else { "  ✗".yellow() };
                    println!("      {} {}", sub_status, subtask.description.dimmed());
                }
            }
        }
    }
}

pub fn display_task_added(id: u32) {
    println!("{} Task added with ID: {}", "✓".green(), id.to_string().bold());
}

pub fn display_task_done(id: u32) {
    println!("{} Task {} marked as done.", "✓".green(), id.to_string().bold());
}

pub fn display_task_deleted(id: u32) {
    println!("{} Task {} deleted.", "✓".green(), id.to_string().bold());
}

pub fn display_task_not_found(id: u32) {
    println!("{} Task with ID {} not found.", "✗".red(), id.to_string().bold());
    println!("💡 Tip: Use 'task_manager list' to see all available tasks.");
}

pub fn display_invalid_priority() {
    println!("{} Invalid priority. Use: low, medium, or high.", "⚠".yellow());
}

pub fn display_no_tasks_for_action() {
    println!("{} No tasks available for this action.", "ℹ".blue());
    println!("💡 Tip: Add a task first with 'task_manager add \"Your task\"'");
}

pub fn display_timer_already_running(id: u32) {
    println!("{} Timer is already running for task {}.", "⚠".yellow(), id.to_string().bold());
    println!("💡 Tip: Stop the current timer first with 'task_manager stop {}'", id);
}

pub fn display_no_timer_running(id: u32) {
    println!("{} No timer is running for task {}.", "⚠".yellow(), id.to_string().bold());
    println!("💡 Tip: Start a timer first with 'task_manager start {}'", id);
}

pub fn display_note_added(id: u32) {
    println!("{} Note added to task {}.", "✓".green(), id.to_string().bold());
}

pub fn display_subtask_added(id: u32) {
    println!("{} Subtask added to task {}.", "✓".green(), id.to_string().bold());
}

pub fn display_timer_started(id: u32) {
    println!("{} Timer started for task {}.", "▶".green(), id.to_string().bold());
}

pub fn display_timer_stopped(id: u32, time_spent: u64) {
    println!("{} Timer stopped for task {}. Time spent: {}",
             "⏹".red(), id.to_string().bold(), format_time(time_spent).cyan());
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