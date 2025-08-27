use super::formatter::TextFormatter;
use super::super::model::Task;

/// Table display utilities
pub struct TableFormatter;

impl TableFormatter {
    pub fn format_task_row(task: &Task) -> String {
        let status = TextFormatter::format_status(task.done);
        let priority = TextFormatter::format_priority(&task.priority);
        let project = TextFormatter::format_project(task.project.as_ref());
        let due_date = TextFormatter::format_due_date(task.due_date.as_ref());
        let time_spent = TextFormatter::format_time(task.time_spent);

        let mut description = task.description.clone();
        if !task.subtasks.is_empty() {
            let done_count = task.subtasks.iter().filter(|st| st.done).count();
            description = format!("{} [{}/{}]", description, done_count, task.subtasks.len());
        }
        if !task.checklists.is_empty() {
            let done_count = task.checklists.iter().filter(|ci| ci.done).count();
            description = format!("{} ☐[{}/{}]", description, done_count, task.checklists.len());
        }

        format!("{:<5} {:<5} {:<15} {:<40} {:<15} {:<15} {:<10}",
                task.id.to_string().bold(),
                status,
                priority,
                TextFormatter::truncate_text(&description, 38),
                TextFormatter::truncate_text(&project, 13),
                due_date,
                time_spent)
    }

    pub fn format_task_details(task: &Task) -> Vec<String> {
        let mut details = Vec::new();

        // Add tags if any
        if !task.tags.is_empty() {
            details.push(format!("      Tags: {}", task.tags.join(", ").cyan()));
        }

        // Add board if not default
        if let Some(board) = &task.board {
            if board != "Todo" {
                details.push(format!("      Board: {}", board.magenta()));
            }
        }

        // Add notes
        if let Some(note) = &task.notes {
            details.push(format!("      Note: {}", note.dimmed()));
        }

        // Add subtasks
        if !task.subtasks.is_empty() {
            for subtask in &task.subtasks {
                let sub_status = TextFormatter::format_status(subtask.done);
                details.push(format!("      {} {}", sub_status, subtask.description.dimmed()));
            }
        }

        // Add checklists
        if !task.checklists.is_empty() {
            for checklist in &task.checklists {
                let check_status = TextFormatter::format_checklist_status(checklist.done);
                details.push(format!("      {} {}", check_status, checklist.description.dimmed()));
            }
        }

        details
    }

    pub fn format_header() -> String {
        format!("{:<5} {:<5} {:<15} {:<40} {:<15} {:<15} {:<10}",
                "ID", "Status", "Priority", "Description", "Project", "Due Date", "Time")
    }

    pub fn format_separator() -> String {
        "-".repeat(120)
    }
}