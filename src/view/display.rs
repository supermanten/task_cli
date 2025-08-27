use colored::*;
use crate::model::{Task, Priority};

pub fn display_tasks(tasks: &[&Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        println!("{:<5} {:<5} {:<15} {:<40} {:<15} {:<15} {:<10}",
                 "ID", "Status", "Priority", "Description", "Project", "Due Date", "Time");
        println!("{}", "-".repeat(120));
        for task in tasks {
            let status = if task.done { "✓".green() } else { "✗".yellow() };
            let priority = match task.priority {
                Priority::High => "High".red().bold(),
                Priority::Medium => "Medium".yellow(),
                Priority::Low => "Low".green(),
            };
            let project = task.project.as_ref().map(|s| s.as_str()).unwrap_or("-");
            let due_date = task.due_date.map(|d| d.format("%m-%d").to_string()).unwrap_or("-".to_string());
            let time_spent = format_time(task.time_spent);

            let mut description = task.description.clone();
            if !task.subtasks.is_empty() {
                let done_count = task.subtasks.iter().filter(|st| st.done).count();
                description = format!("{} [{}/{}]", description, done_count, task.subtasks.len());
            }
            if !task.checklists.is_empty() {
                let done_count = task.checklists.iter().filter(|ci| ci.done).count();
                description = format!("{} ☐[{}/{}]", description, done_count, task.checklists.len());
            }

            println!("{:<5} {:<5} {:<15} {:<40} {:<15} {:<15} {:<10}",
                     task.id.to_string().bold(),
                     status,
                     priority,
                     description.chars().take(38).collect::<String>(),
                     project.chars().take(13).collect::<String>(),
                     due_date,
                     time_spent);

            // Show tags if any
            if !task.tags.is_empty() {
                println!("      Tags: {}", task.tags.join(", ").cyan());
            }

            // Show board if not default
            if let Some(board) = &task.board {
                if board != "Todo" {
                    println!("      Board: {}", board.magenta());
                }
            }

            // Show notes
            if let Some(note) = &task.notes {
                println!("      Note: {}", note.dimmed());
            }

            // Show subtasks
            if !task.subtasks.is_empty() {
                for subtask in &task.subtasks {
                    let sub_status = if subtask.done { "  ✓".green() } else { "  ✗".yellow() };
                    println!("      {} {}", sub_status, subtask.description.dimmed());
                }
            }

            // Show checklists
            if !task.checklists.is_empty() {
                for checklist in &task.checklists {
                    let check_status = if checklist.done { "  ☑".green() } else { "  ☐".yellow() };
                    println!("      {} {}", check_status, checklist.description.dimmed());
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

#[allow(dead_code)]
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

pub fn display_calendar(calendar: &[(String, Vec<&Task>)]) {
    if calendar.is_empty() {
        println!("📅 No tasks with due dates found.");
        return;
    }

    println!("📅 Calendar View");
    println!("================");

    for (date, tasks) in calendar {
        println!("\n📆 {} ({})", date, tasks.len());
        for task in tasks {
            let status = if task.done { "✓".green() } else { "✗".yellow() };
            let priority = match task.priority {
                Priority::High => "🔴",
                Priority::Medium => "🟡",
                Priority::Low => "🟢",
            };
            println!("  {} {} {}", status, priority, task.description);
        }
    }
}

pub fn display_timeline(tasks: &[&Task]) {
    if tasks.is_empty() {
        println!("⏰ No tasks found.");
        return;
    }

    println!("⏰ Timeline View");
    println!("================");

    for task in tasks {
        let status = if task.done { "✓".green() } else { "✗".yellow() };
        let created = task.created_at.format("%Y-%m-%d %H:%M");
        let time_spent = if task.time_spent > 0 {
            format!(" ({})", format_time(task.time_spent).cyan())
        } else {
            String::new()
        };
        println!("{} {} - {}{}", created, status, task.description, time_spent);
    }
}

pub fn display_focus(context: &str, tasks: &[&Task]) {
    if tasks.is_empty() {
        println!("🎯 No tasks found for focus '{}'.", context);
        return;
    }

    println!("🎯 Focus View: {}", context);
    println!("====================");
    println!("Found {} tasks", tasks.len());

    for task in tasks {
        let status = if task.done { "✓".green() } else { "✗".yellow() };
        let priority = match task.priority {
            Priority::High => "🔴",
            Priority::Medium => "🟡",
            Priority::Low => "🟢",
        };
        let project = task.project.as_ref().map(|p| format!(" [{}]", p)).unwrap_or_default();
        println!("{} {} {}{}", status, priority, task.description, project);
    }
}

pub fn display_board_view(board_view: &[(String, Vec<&Task>)]) {
    println!("📋 Board View");
    println!("=============");

    for (board, tasks) in board_view {
        println!("\n📂 {} ({})", board.bold(), tasks.len());

        if tasks.is_empty() {
            println!("  (empty)");
            continue;
        }

        for task in tasks {
            let status = if task.done { "✓".green() } else { "✗".yellow() };
            let priority = match task.priority {
                Priority::High => "🔴",
                Priority::Medium => "🟡",
                Priority::Low => "🟢",
            };
            let tags = if !task.tags.is_empty() {
                format!(" [{}]", task.tags.join(", "))
            } else {
                String::new()
            };
            println!("  {} {} {}{}", status, priority, task.description, tags);
        }
    }
}