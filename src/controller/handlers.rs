use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, NaiveTime};
use dialoguer::{Select, Input};
use crate::cli::{Commands, BoardCommands};
use crate::model::{TaskManager, Priority};
use crate::view;

pub fn handle_command(command: Commands, task_manager: &mut TaskManager) {
    match command {
        Commands::Add { description, priority, project, due, tags } => {
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

            let due_date = if let Some(due_str) = due {
                parse_due_date(&due_str)
            } else {
                None
            };

            let tags_vec = if let Some(tags_str) = tags {
                tags_str.split(',').map(|s| s.trim().to_string()).collect()
            } else {
                Vec::new()
            };

            task_manager.add_task_with_options(description, priority, project, due_date, tags_vec);
            let active_tasks = task_manager.get_active_tasks();
            let id = active_tasks.last().unwrap().id;
            view::display_task_added(id);
        }
        Commands::List { priority, project, description, due, tag, board } => {
            let filtered_tasks = if let Some(pri) = priority {
                match pri.as_str() {
                    "high" => task_manager.filter_by_priority(Priority::High),
                    "medium" => task_manager.filter_by_priority(Priority::Medium),
                    "low" => task_manager.filter_by_priority(Priority::Low),
                    _ => {
                        println!("Invalid priority. Use: high, medium, low");
                        return;
                    }
                }
            } else if let Some(proj) = project {
                task_manager.filter_by_project(&proj)
            } else if let Some(desc) = description {
                task_manager.search_by_description(&desc)
            } else if let Some(due_filter) = due {
                task_manager.filter_by_due_date(&due_filter)
            } else if let Some(tag_filter) = tag {
                task_manager.filter_by_tag(&tag_filter)
            } else if let Some(board_filter) = board {
                task_manager.filter_by_board(&board_filter)
            } else {
                task_manager.get_active_tasks()
            };
            view::display_tasks(&filtered_tasks);
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
        Commands::Board { command } => {
            handle_board_command(command, task_manager);
        }
        Commands::Tag { id, tag } => {
            if task_manager.add_tag(id, tag.clone()) {
                println!("✓ Tag '{}' added to task {}", tag, id);
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Untag { id, tag } => {
            if task_manager.remove_tag(id, &tag) {
                println!("✓ Tag '{}' removed from task {}", tag, id);
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Project { id, project } => {
            if task_manager.set_project(id, project.clone()) {
                println!("✓ Task {} assigned to project '{}'", id, project);
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Due { id, date } => {
            if let Some(due_date) = parse_due_date(&date) {
                if task_manager.set_due_date(id, due_date) {
                    println!("✓ Due date set for task {}: {}", id, due_date.format("%Y-%m-%d %H:%M"));
                } else {
                    view::display_task_not_found(id);
                }
            } else {
                println!("✗ Invalid date format. Use YYYY-MM-DD or YYYY-MM-DD HH:MM");
            }
        }
        Commands::Checklist { id, item } => {
            if task_manager.add_checklist_item(id, item.clone()) {
                println!("✓ Checklist item '{}' added to task {}", item, id);
            } else {
                view::display_task_not_found(id);
            }
        }
        Commands::Check { task_id, item_id } => {
            if task_manager.toggle_checklist_item(task_id, item_id) {
                println!("✓ Checklist item {} toggled for task {}", item_id, task_id);
            } else {
                println!("✗ Checklist item not found");
            }
        }
        Commands::Export { format, filename } => {
            let result = match format.as_str() {
                "csv" => task_manager.export_csv(&filename),
                "json" => task_manager.export_json(&filename),
                _ => {
                    println!("✗ Unsupported format. Use: csv or json");
                    return;
                }
            };

            match result {
                Ok(_) => println!("✓ Tasks exported to {}", filename),
                Err(e) => println!("✗ Export failed: {}", e),
            }
        }
        Commands::Calendar => {
            let calendar = task_manager.get_calendar_view();
            view::display_calendar(&calendar);
        }
        Commands::Timeline => {
            let timeline = task_manager.get_timeline_view();
            view::display_timeline(&timeline);
        }
        Commands::Focus { context } => {
            let focus_tasks = task_manager.get_focus_view(&context);
            view::display_focus(&context, &focus_tasks);
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
            "Add Tag",
            "Set Project",
            "Set Due Date",
            "Add Checklist Item",
            "Show Calendar",
            "Show Board View",
            "Export Tasks",
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

                let project: String = Input::new().with_prompt("Project (optional)").allow_empty(true).interact_text().unwrap();
                let project = if project.is_empty() { None } else { Some(project) };

                let tags: String = Input::new().with_prompt("Tags (comma-separated, optional)").allow_empty(true).interact_text().unwrap();
                let tags_vec = if tags.is_empty() {
                    Vec::new()
                } else {
                    tags.split(',').map(|s| s.trim().to_string()).collect()
                };

                task_manager.add_task_with_options(description, priority, project, None, tags_vec);
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
            8 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to add tag").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                let tag: String = Input::new().with_prompt("Tag").interact_text().unwrap();
                if task_manager.add_tag(id, tag.clone()) {
                    println!("✓ Tag '{}' added to task {}", tag, id);
                }
            }
            9 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to set project").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                let project: String = Input::new().with_prompt("Project").interact_text().unwrap();
                if task_manager.set_project(id, project.clone()) {
                    println!("✓ Task {} assigned to project '{}'", id, project);
                }
            }
            10 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to set due date").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                let date: String = Input::new().with_prompt("Due date (YYYY-MM-DD)").interact_text().unwrap();
                if let Some(due_date) = parse_due_date(&date) {
                    if task_manager.set_due_date(id, due_date) {
                        println!("✓ Due date set for task {}: {}", id, due_date.format("%Y-%m-%d %H:%M"));
                    }
                } else {
                    println!("✗ Invalid date format. Use YYYY-MM-DD");
                }
            }
            11 => {
                let active_tasks = task_manager.get_active_tasks();
                if active_tasks.is_empty() {
                    println!("No tasks available.");
                    continue;
                }
                let task_options: Vec<String> = active_tasks.iter().map(|t| format!("{}: {}", t.id, t.description)).collect();
                let selection = Select::new().with_prompt("Select task to add checklist item").items(&task_options).interact().unwrap();
                let id = active_tasks[selection].id;
                let item: String = Input::new().with_prompt("Checklist item").interact_text().unwrap();
                if task_manager.add_checklist_item(id, item.clone()) {
                    println!("✓ Checklist item '{}' added to task {}", item, id);
                }
            }
            12 => {
                let calendar = task_manager.get_calendar_view();
                view::display_calendar(&calendar);
            }
            13 => {
                let board_view = task_manager.get_board_view();
                view::display_board_view(&board_view);
            }
            14 => {
                let format_options = vec!["CSV", "JSON"];
                let format_idx = Select::new().with_prompt("Export format").items(&format_options).interact().unwrap();
                let format = if format_idx == 0 { "csv" } else { "json" };
                let filename: String = Input::new().with_prompt("Filename").interact_text().unwrap();
                let result = match format {
                    "csv" => task_manager.export_csv(&filename),
                    "json" => task_manager.export_json(&filename),
                    _ => return,
                };
                match result {
                    Ok(_) => println!("✓ Tasks exported to {}", filename),
                    Err(e) => println!("✗ Export failed: {}", e),
                }
            }
            15 => break,
            _ => continue,
        }
    }
}

fn parse_due_date(date_str: &str) -> Option<DateTime<Utc>> {
    // Try parsing as YYYY-MM-DD HH:MM first
    if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M") {
        Some(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
    }
    // Try parsing as YYYY-MM-DD (set time to end of day)
    else if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let datetime = date.and_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap());
        Some(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
    }
    // Try parsing as MM/DD/YYYY
    else if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
        let datetime = date.and_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap());
        Some(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
    } else {
        None
    }
}

fn handle_board_command(command: BoardCommands, task_manager: &mut TaskManager) {
    match command {
        BoardCommands::Add { name } => {
            task_manager.create_board(name.clone());
            println!("✓ Board '{}' created", name);
        }
        BoardCommands::List => {
            println!("📋 Available Boards:");
            for board in &task_manager.boards {
                println!("  • {}", board);
            }
        }
        BoardCommands::Show => {
            let board_view = task_manager.get_board_view();
            view::display_board_view(&board_view);
        }
        BoardCommands::Move { task_id, board } => {
            if task_manager.move_task_to_board(task_id, board.clone()) {
                println!("✓ Task {} moved to board '{}'", task_id, board);
            } else {
                view::display_task_not_found(task_id);
            }
        }
    }
}