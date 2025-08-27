use crate::cli::Commands;
use crate::model::TaskManager;
use crate::view;

pub fn handle_command(command: Commands, task_manager: &mut TaskManager) {
    match command {
        Commands::Add { description } => {
            task_manager.add_task(description.clone());
            let id = task_manager.tasks.len() as u32;
            view::display_task_added(id);
        }
        Commands::List => {
            view::display_tasks(&task_manager.tasks);
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
    }
    task_manager.save();
}