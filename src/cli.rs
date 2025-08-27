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
        #[arg(short = 'P', long, help = "Project name")]
        project: Option<String>,
        #[arg(long, help = "Due date (YYYY-MM-DD or YYYY-MM-DD HH:MM)")]
        due: Option<String>,
        #[arg(short, long, help = "Tags (comma-separated)")]
        tags: Option<String>,
    },
    #[command(about = "List all active tasks")]
    List {
        #[arg(long, help = "Filter by priority (low/medium/high)")]
        priority: Option<String>,
        #[arg(long, help = "Filter by project")]
        project: Option<String>,
        #[arg(long, help = "Search in description")]
        description: Option<String>,
        #[arg(long, help = "Filter by due date (today/week/overdue)")]
        due: Option<String>,
        #[arg(long, help = "Filter by tag")]
        tag: Option<String>,
        #[arg(long, help = "Filter by board")]
        board: Option<String>,
    },
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
    #[command(about = "Start beautiful TUI mode with Ratatui")]
    Tui,
    #[command(about = "Show detailed usage and examples")]
    Usage,
    #[command(about = "Board management commands")]
    Board {
        #[command(subcommand)]
        command: BoardCommands,
    },
    #[command(about = "Add a tag to a task")]
    Tag {
        #[arg(help = "Task ID")]
        id: u32,
        #[arg(help = "Tag to add")]
        tag: String,
    },
    #[command(about = "Remove a tag from a task")]
    Untag {
        #[arg(help = "Task ID")]
        id: u32,
        #[arg(help = "Tag to remove")]
        tag: String,
    },
    #[command(about = "Set project for a task")]
    Project {
        #[arg(help = "Task ID")]
        id: u32,
        #[arg(help = "Project name")]
        project: String,
    },
    #[command(about = "Set due date for a task")]
    Due {
        #[arg(help = "Task ID")]
        id: u32,
        #[arg(help = "Due date (YYYY-MM-DD or YYYY-MM-DD HH:MM)")]
        date: String,
    },
    #[command(about = "Add checklist item to a task")]
    Checklist {
        #[arg(help = "Task ID")]
        id: u32,
        #[arg(help = "Checklist item description")]
        item: String,
    },
    #[command(about = "Toggle checklist item status")]
    Check {
        #[arg(help = "Task ID")]
        task_id: u32,
        #[arg(help = "Checklist item ID")]
        item_id: u32,
    },
    #[command(about = "Export tasks to file")]
    Export {
        #[arg(help = "Export format (csv/json)")]
        format: String,
        #[arg(help = "Output filename")]
        filename: String,
    },
    #[command(about = "Show calendar view")]
    Calendar,
    #[command(about = "Show timeline view")]
    Timeline,
    #[command(about = "Show focus view for specific context")]
    Focus {
        #[arg(help = "Focus context (tag/project/board)")]
        context: String,
    },
}

#[derive(Subcommand)]
pub enum BoardCommands {
    #[command(about = "Create a new board")]
    Add {
        #[arg(help = "Board name")]
        name: String,
    },
    #[command(about = "List all boards")]
    List,
    #[command(about = "Show board view")]
    Show,
    #[command(about = "Move task to different board")]
    Move {
        #[arg(help = "Task ID")]
        task_id: u32,
        #[arg(help = "Target board name")]
        board: String,
    },
}