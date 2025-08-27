use dialoguer::{Select, Input};
use crate::cli::Commands;
use crate::model::{TaskManager, Priority};
use crate::view;

pub fn handle_command(command: Commands, task_manager: &mut TaskManager) {
    match command {
        Commands::Add { description, priority } => {
            let priority = match priority.as_deref() {
                Some("high") | Some("High") => Priority::High,
                Some("medium") | Some("Medium") => Priority::Medium,
                Some("low") | Some("Low") => Priority::Low,
                Some(p) => {
                    view::display_invalid_priority();
                    println!("Using default priority 'medium' for: {}", p);
                    Priority::Medium
                }
                None => Priority::Medium,
            };
            task_manager.add_task(description, priority);
            let active_tasks = task_manager.get_active_tasks();
            let id = active_tasks.last().unwrap().id;
            view::display_task_added(id);
        }
        Commands::List => {
            let active_tasks = task_manager.get_active_tasks();
            view::display_tasks(&active_tasks);
        }
        Commands::Done { id } => {
            if task_manager.mark_done(id) {
                view::display_task_done(id);
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Delete { id } => {
            if task_manager.delete_task(id) {
                view::display_task_deleted(id);
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Note { id, note } => {
            if task_manager.add_note(id, note) {
                view::display_note_added(id);
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Subtask { id, description } => {
            if task_manager.add_subtask(id, description) {
                view::display_subtask_added(id);
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Start { id } => {
            if let Some(task) = task_manager.tasks.iter().find(|t| t.id == id && t.deleted_at.is_none()) {
                if task.timer_start.is_some() {
                    view::display_timer_already_running(id);
                } else if task_manager.start_timer(id) {
                    view::display_timer_started(id);
                } else {
                    view::display_task_not_found(id);
                }
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Stop { id } => {
            if let Some(task) = task_manager.tasks.iter().find(|t| t.id == id && t.deleted_at.is_none()) {
                if task.timer_start.is_none() {
                    view::display_no_timer_running(id);
                } else {
                    let time_spent = task.time_spent;
                    if task_manager.stop_timer(id) {
                        let new_time = task_manager.tasks.iter().find(|t| t.id == id).map(|t| t.time_spent).unwrap_or(0);
                        view::display_timer_stopped(id, new_time - time_spent);
                    } else {
                        view::display_task_not_found(id);
                    }
                }
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Interactive => {
            run_interactive_mode(task_manager);
        }
        Commands::Usage => {
            show_detailed_help();
        }
    }
    task_manager.save();
}

fn show_detailed_help() {
    println!("🚀 Task Manager CLI - Detailed Help");
    println!("=====================================");
    println!();
    println!("📋 BASIC COMMANDS:");
    println!("  • Add task:        task_manager add \"Task description\" [--priority high|medium|low]");
    println!("  • List tasks:      task_manager list");
    println!("  • Mark done:       task_manager done <task_id>");
    println!("  • Delete task:     task_manager delete <task_id>");
    println!();
    println!("📝 ADVANCED FEATURES:");
    println!("  • Add note:        task_manager note <task_id> \"Your note\"");
    println!("  • Add subtask:     task_manager subtask <task_id> \"Subtask description\"");
    println!("  • Start timer:     task_manager start <task_id>");
    println!("  • Stop timer:      task_manager stop <task_id>");
    println!();
    println!("🎮 INTERACTIVE MODE:");
    println!("  • Menu interface:  task_manager interactive");
    println!("    - Easy-to-use menus for all operations");
    println!("    - No need to remember command syntax");
    println!();
    println!("🎨 FEATURES:");
    println!("  • Priority levels with color coding (red=high, yellow=medium, green=low)");
    println!("  • Task notes for additional context");
    println!("  • Subtasks for breaking down complex tasks");
    println!("  • Time tracking with start/stop functionality");
    println!("  • Soft delete (tasks are hidden but preserved)");
    println!();
    println!("💡 EXAMPLES:");
    println!("  task_manager add \"Complete project proposal\" --priority high");
    println!("  task_manager note 1 \"Due by Friday\"");
    println!("  task_manager subtask 1 \"Research competitors\"");
    println!("  task_manager start 1  # Start working");
    println!("  task_manager stop 1   # Stop and record time");
    println!("  task_manager interactive  # Use menu interface");
    println!();
    println!("📖 GETTING HELP:");
    println!("  • Basic help:      task_manager --help");
    println!("  • Command help:    task_manager <command> --help");
    println!("  • Detailed help:   task_manager help (this screen)");
    println!();
    println!("🔧 DATA STORAGE:");
    println!("  Tasks are stored in 'tasks.json' in the current directory.");
    println!("  The file is created automatically when you add your first task.");
}

fn run_interactive_mode(task_manager: &mut TaskManager) {
    loop {
        let options = vec![
            "Add Task",
            "List Tasks",
            "Mark Task as Done",
            "Delete Task",
            "Add Note",
            "Add Subtask",
            "Start Timer",
            "Stop Timer",
            "Exit",
        ];

        let selection = Select::new()
            .with_prompt("Choose an action")
            .items(&options)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let description: String = Input::new().with_prompt("Task description").interact_text().unwrap();
                let priority_options = vec!["Low", "Medium", "High"];
                let priority_idx = Select::new().with_prompt("Priority").items(&priority_options).interact().unwrap();
                let priority = match priority_idx {
                    0 => Priority::Low,
                    1 => Priority::Medium,
                    2 => Priority::High,
                    _ => Priority::Medium,
                };
                task_manager.add_task(description, priority);
                let active_tasks = task_manager.get_active_tasks();
                let id = active_tasks.last().unwrap().id;
                view::display_task_added(id);
            }
            1 => {
                let active_tasks = task_manager.get_active_tasks();
                view::display_tasks(&active_tasks);
            }
            2 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to mark as done").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                if task_manager.mark_done(id) {
                    view::display_task_done(id);
                }
            }
            3 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to delete").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                if task_manager.delete_task(id) {
                    view::display_task_deleted(id);
                }
            }
            4 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to add note").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                let note: String = Input::new().with_prompt("Note").interact_text().unwrap();
                if task_manager.add_note(id, note) {
                    view::display_note_added(id);
                }
            }
            5 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to add subtask").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                let description: String = Input::new().with_prompt("Subtask description").interact_text().unwrap();
                if task_manager.add_subtask(id, description) {
                    view::display_subtask_added(id);
                }
            }
            6 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to start timer").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                if task_manager.start_timer(id) {
                    view::display_timer_started(id);
                }
            }
            7 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to stop timer").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                let time_spent = task_manager.tasks.iter().find(|t| t.id == id).map(|t| t.time_spent).unwrap_or(0);
                if task_manager.stop_timer(id) {
                    let new_time = task_manager.tasks.iter().find(|t| t.id == id).map(|t| t.time_spent).unwrap_or(0);
                    view::display_timer_stopped(id, new_time - time_spent);
                }
            }
            8 => break,
            _ => continue,
        }
    }
}