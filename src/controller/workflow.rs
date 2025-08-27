use crate::model::{TaskManager, Priority};
use crate::view;

/// Workflow management operations
pub struct WorkflowManager;

impl WorkflowManager {
    pub fn create_task_with_options(
        task_manager: &mut TaskManager,
        description: String,
        priority: Priority,
        project: Option<String>,
        due_date: Option<chrono::DateTime<chrono::Utc>>,
        tags: Vec<String>,
    ) -> u32 {
        task_manager.add_task_with_options(description, priority, project, due_date, tags);
        let active_tasks = task_manager.get_active_tasks();
        active_tasks.last().unwrap().id
    }

    pub fn move_task_through_workflow(
        task_manager: &mut TaskManager,
        task_id: u32,
        from_board: &str,
        to_board: &str,
    ) -> Result<(), String> {
        // Validate current board
        let task = task_manager.tasks.iter().find(|t| t.id == task_id && t.deleted_at.is_none())
            .ok_or_else(|| format!("Task {} not found", task_id))?;

        let current_board = task.board.as_ref().unwrap_or(&"Todo".to_string());
        if current_board != from_board {
            return Err(format!("Task {} is not in '{}' board (currently in '{}')",
                             task_id, from_board, current_board));
        }

        // Move to new board
        if task_manager.move_task_to_board(task_id, to_board.to_string()) {
            Ok(())
        } else {
            Err(format!("Failed to move task {} to '{}' board", task_id, to_board))
        }
    }

    pub fn complete_task_workflow(task_manager: &mut TaskManager, task_id: u32) -> Result<(), String> {
        // Mark as done
        if !task_manager.mark_done(task_id) {
            return Err(format!("Task {} not found", task_id));
        }

        // Move to Done board
        task_manager.move_task_to_board(task_id, "Done".to_string());
        Ok(())
    }

    pub fn start_task_workflow(task_manager: &mut TaskManager, task_id: u32) -> Result<(), String> {
        // Move to In Progress
        task_manager.move_task_to_board(task_id, "In Progress".to_string());

        // Start timer
        if !task_manager.start_timer(task_id) {
            return Err(format!("Failed to start timer for task {}", task_id));
        }

        Ok(())
    }

    pub fn pause_task_workflow(task_manager: &mut TaskManager, task_id: u32) -> Result<u64, String> {
        // Stop timer and get time spent
        let time_spent = task_manager.tasks.iter().find(|t| t.id == task_id)
            .map(|t| t.time_spent)
            .unwrap_or(0);

        if !task_manager.stop_timer(task_id) {
            return Err(format!("Failed to stop timer for task {}", task_id));
        }

        Ok(time_spent)
    }

    pub fn assign_task_to_team_member(
        task_manager: &mut TaskManager,
        task_id: u32,
        member_name: &str,
    ) -> Result<(), String> {
        let note = format!("Assigned to: @{}", member_name);
        if task_manager.add_note(task_id, note) {
            Ok(())
        } else {
            Err(format!("Task {} not found", task_id))
        }
    }

    pub fn add_task_with_checklist(
        task_manager: &mut TaskManager,
        description: String,
        priority: Priority,
        checklist_items: Vec<String>,
    ) -> Result<u32, String> {
        // Create the task
        task_manager.add_task(description, priority);
        let active_tasks = task_manager.get_active_tasks();
        let task_id = active_tasks.last().unwrap().id;

        // Add checklist items
        for item in checklist_items {
            if !task_manager.add_checklist_item(task_id, item) {
                return Err(format!("Failed to add checklist item to task {}", task_id));
            }
        }

        Ok(task_id)
    }

    pub fn get_productivity_metrics(task_manager: &TaskManager) -> ProductivityMetrics {
        let tasks = &task_manager.tasks;
        let total_tasks = tasks.len();
        let completed_tasks = tasks.iter().filter(|t| t.done).count();
        let active_tasks = tasks.iter().filter(|t| t.deleted_at.is_none()).count();
        let deleted_tasks = tasks.iter().filter(|t| t.deleted_at.is_some()).count();

        let total_time_spent: u64 = tasks.iter().map(|t| t.time_spent).sum();
        let avg_time_per_task = if completed_tasks > 0 {
            total_time_spent / completed_tasks as u64
        } else {
            0
        };

        let high_priority_tasks = tasks.iter().filter(|t| matches!(t.priority, Priority::High)).count();
        let completed_high_priority = tasks.iter()
            .filter(|t| matches!(t.priority, Priority::High) && t.done)
            .count();

        ProductivityMetrics {
            total_tasks,
            completed_tasks,
            active_tasks,
            deleted_tasks,
            completion_rate: if total_tasks > 0 { (completed_tasks as f64 / total_tasks as f64) * 100.0 } else { 0.0 },
            total_time_spent,
            avg_time_per_task,
            high_priority_tasks,
            completed_high_priority,
        }
    }
}

#[derive(Debug)]
pub struct ProductivityMetrics {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub active_tasks: usize,
    pub deleted_tasks: usize,
    pub completion_rate: f64,
    pub total_time_spent: u64,
    pub avg_time_per_task: u64,
    pub high_priority_tasks: usize,
    pub completed_high_priority: usize,
}