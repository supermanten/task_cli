use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task_manager")]
#[command(about = "A powerful task manager CLI with priorities, notes, subtasks, and time tracking")]
#[command(long_about = "A comprehensive task management tool that helps you organize and track your tasks efficiently.

EXAMPLES:
    # Add a high priority task
    task_manager add \"Complete project\" --priority high

    # List all tasks
    task_manager list

    # Mark task as done
    task_manager done 1

    # Add a note to a task
    task_manager note 1 \"This is very important\"

    # Start interactive mode
    task_manager interactive

    # Get detailed help
    task_manager help")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Add a new task")]
    Add {
        #[arg(help = "Task description")]
        description: String,
        #[arg(short, long, help = "Priority level: low, medium, high")]
        priority: Option<String>,
    },
    #[command(about = "List all active tasks")]
    List,
    #[command(about = "Mark a task as completed")]
    Done {
        #[arg(help = "Task ID to mark as done")]
        id: u32
    },
    #[command(about = "Delete a task (soft delete)")]
    Delete {
        #[arg(help = "Task ID to delete")]
        id: u32
    },
    #[command(about = "Add a note to a task")]
    Note {
        #[arg(help = "Task ID")]
        id: u32,
        #[arg(help = "Note content")]
        note: String,
    },
    #[command(about = "Add a subtask to a task")]
    Subtask {
        #[arg(help = "Task ID")]
        id: u32,
        #[arg(help = "Subtask description")]
        description: String,
    },
    #[command(about = "Start time tracking for a task")]
    Start {
        #[arg(help = "Task ID")]
        id: u32
    },
    #[command(about = "Stop time tracking for a task")]
    Stop {
        #[arg(help = "Task ID")]
        id: u32
    },
    #[command(about = "Start interactive mode with menus")]
    Interactive,
    #[command(about = "Show detailed usage and examples")]
    Usage,
}