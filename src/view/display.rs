use crate::model::Task;

pub fn display_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            let status = if task.done { "[DONE]" } else { "[TODO]" };
            println!("{} {}: {}", status, task.id, task.description);
        }
    }
}

pub fn display_task_added(id: u32) {
    println!("Task added with ID: {}", id);
}

pub fn display_task_done(id: u32) {
    println!("Task {} marked as done.", id);
}

pub fn display_task_deleted(id: u32) {
    println!("Task {} deleted.", id);
}

pub fn display_task_not_found(id: u32) {
    println!("Task with ID {} not found.", id);
}