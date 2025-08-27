use crate::cli::{Commands, BoardCommands};
use crate::model::TaskManager;
use crate::controller::{validator::InputValidator, workflow::WorkflowManager};
use crate::view;

/// Individual command processors
pub struct CommandProcessor;

impl CommandProcessor {
    pub fn process_add_command(
        task_manager: &mut TaskManager,
        description: String,
        priority: Option<String>,
        project: Option<String>,
        due: Option<String>,
        tags: Option<String>,
    ) {
        // Validate inputs
        if let Err(e) = InputValidator::validate_description(&description) {
            println!("✗ {}", e);
            return;
        }

        let priority = match priority {
            Some(p) => match InputValidator::validate_priority(&p) {
                Ok(pri) => pri,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            },
            None => crate::model::Priority::Medium,
        };

        let due_date = match due {
            Some(d) => match InputValidator::parse_due_date(&d) {
                Ok(date) => Some(date),
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            },
            None => None,
        };

        let tags_vec = match tags {
            Some(t) => {
                let mut valid_tags = Vec::new();
                for tag in t.split(',') {
                    let tag = tag.trim();
                    if !tag.is_empty() {
                        match InputValidator::validate_tag(tag) {
                            Ok(_) => valid_tags.push(tag.to_string()),
                            Err(e) => {
                                println!("✗ Invalid tag '{}': {}", tag, e);
                                return;
                            }
                        }
                    }
                }
                valid_tags
            },
            None => Vec::new(),
        };

        let task_id = WorkflowManager::create_task_with_options(
            task_manager,
            description,
            priority,
            project,
            due_date,
            tags_vec,
        );

        view::display_task_added(task_id);
    }

    pub fn process_list_command(
        task_manager: &TaskManager,
        priority: Option<String>,
        project: Option<String>,
        description: Option<String>,
        due: Option<String>,
        tag: Option<String>,
        board: Option<String>,
    ) {
        let filtered_tasks = if let Some(pri) = priority {
            match InputValidator::validate_priority(&pri) {
                Ok(p) => task_manager.filter_by_priority(p),
                Err(e) => {
                    println!("✗ {}", e);
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

    pub fn process_done_command(task_manager: &mut TaskManager, id: u32) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if let Err(e) = WorkflowManager::complete_task_workflow(task_manager, id) {
            println!("✗ {}", e);
            return;
        }

        view::display_task_done(id);
    }

    pub fn process_delete_command(task_manager: &mut TaskManager, id: u32) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if task_manager.delete_task(id) {
            view::display_task_deleted(id);
        } else {
            view::display_task_not_found(id);
        }
    }

    pub fn process_note_command(task_manager: &mut TaskManager, id: u32, note: String) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if task_manager.add_note(id, note) {
            view::display_note_added(id);
        } else {
            view::display_task_not_found(id);
        }
    }

    pub fn process_subtask_command(task_manager: &mut TaskManager, id: u32, description: String) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if let Err(e) = InputValidator::validate_description(&description) {
            println!("✗ {}", e);
            return;
        }

        if task_manager.add_subtask(id, description) {
            view::display_subtask_added(id);
        } else {
            view::display_task_not_found(id);
        }
    }

    pub fn process_start_command(task_manager: &mut TaskManager, id: u32) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if let Err(e) = WorkflowManager::start_task_workflow(task_manager, id) {
            println!("✗ {}", e);
            return;
        }

        view::display_timer_started(id);
    }

    pub fn process_stop_command(task_manager: &mut TaskManager, id: u32) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        match WorkflowManager::pause_task_workflow(task_manager, id) {
            Ok(time_spent) => view::display_timer_stopped(id, time_spent),
            Err(e) => println!("✗ {}", e),
        }
    }

    pub fn process_board_command(command: BoardCommands, task_manager: &mut TaskManager) {
        match command {
            BoardCommands::Add { name } => {
                if let Err(e) = InputValidator::validate_board_name(&name) {
                    println!("✗ {}", e);
                    return;
                }
                task_manager.create_board(name.clone());
                println!("✓ Board '{}' created", name);
            }
            BoardCommands::List => {
                println!("📋 Available Boards:");
                for board in task_manager.get_board_list() {
                    println!("  • {}", board);
                }
            }
            BoardCommands::Show => {
                let board_view = task_manager.get_board_view();
                view::display_board_view(&board_view);
            }
            BoardCommands::Move { task_id, board } => {
                if let Err(e) = InputValidator::validate_task_id(task_id, &task_manager.tasks) {
                    println!("✗ {}", e);
                    return;
                }

                if task_manager.move_task_to_board(task_id, board.clone()) {
                    println!("✓ Task {} moved to board '{}'", task_id, board);
                } else {
                    println!("✗ Failed to move task {} to board '{}'", task_id, board);
                }
            }
        }
    }

    pub fn process_tag_command(task_manager: &mut TaskManager, id: u32, tag: String) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if let Err(e) = InputValidator::validate_tag(&tag) {
            println!("✗ {}", e);
            return;
        }

        if task_manager.add_tag(id, tag.clone()) {
            println!("✓ Tag '{}' added to task {}", tag, id);
        } else {
            view::display_task_not_found(id);
        }
    }

    pub fn process_untag_command(task_manager: &mut TaskManager, id: u32, tag: String) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if task_manager.remove_tag(id, &tag) {
            println!("✓ Tag '{}' removed from task {}", tag, id);
        } else {
            println!("✗ Tag '{}' not found on task {}", tag, id);
        }
    }

    pub fn process_project_command(task_manager: &mut TaskManager, id: u32, project: String) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if task_manager.set_project(id, project.clone()) {
            println!("✓ Task {} assigned to project '{}'", id, project);
        } else {
            view::display_task_not_found(id);
        }
    }

    pub fn process_due_command(task_manager: &mut TaskManager, id: u32, date: String) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        match InputValidator::parse_due_date(&date) {
            Ok(due_date) => {
                if task_manager.set_due_date(id, due_date) {
                    println!("✓ Due date set for task {}: {}", id, due_date.format("%Y-%m-%d %H:%M"));
                } else {
                    view::display_task_not_found(id);
                }
            }
            Err(e) => println!("✗ {}", e),
        }
    }

    pub fn process_checklist_command(task_manager: &mut TaskManager, id: u32, item: String) {
        if let Err(e) = InputValidator::validate_task_id(id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if let Err(e) = InputValidator::validate_description(&item) {
            println!("✗ {}", e);
            return;
        }

        if task_manager.add_checklist_item(id, item.clone()) {
            println!("✓ Checklist item '{}' added to task {}", item, id);
        } else {
            view::display_task_not_found(id);
        }
    }

    pub fn process_check_command(task_manager: &mut TaskManager, task_id: u32, item_id: u32) {
        if let Err(e) = InputValidator::validate_task_id(task_id, &task_manager.tasks) {
            println!("✗ {}", e);
            return;
        }

        if task_manager.toggle_checklist_item(task_id, item_id) {
            println!("✓ Checklist item {} toggled for task {}", item_id, task_id);
        } else {
            println!("✗ Checklist item {} not found for task {}", item_id, task_id);
        }
    }

    pub fn process_export_command(task_manager: &TaskManager, format: String, filename: String) {
        let result = match format.as_str() {
            "csv" => task_manager.export_csv(&filename),
            "json" => task_manager.export_json(&filename),
            _ => {
                println!("✗ Unsupported format '{}'. Use: csv or json", format);
                return;
            }
        };

        match result {
            Ok(_) => println!("✓ Tasks exported to {}", filename),
            Err(e) => println!("✗ Export failed: {}", e),
        }
    }

    pub fn process_calendar_command(task_manager: &TaskManager) {
        let calendar = task_manager.get_calendar_view();
        view::display_calendar(&calendar);
    }

    pub fn process_timeline_command(task_manager: &TaskManager) {
        let timeline = task_manager.get_timeline_view();
        view::display_timeline(&timeline);
    }

    pub fn process_focus_command(task_manager: &TaskManager, context: String) {
        let focus_tasks = task_manager.get_focus_view(&context);
        view::display_focus(&context, &focus_tasks);
    }
}