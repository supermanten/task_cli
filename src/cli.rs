use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task_manager")]
#[command(about = "A simple task manager CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        description: String,
        #[arg(short, long)]
        priority: Option<String>,
    },
    List,
    Done { id: u32 },
    Delete { id: u32 },
    Note {
        id: u32,
        note: String,
    },
    Subtask {
        id: u32,
        description: String,
    },
    Start { id: u32 },
    Stop { id: u32 },
    Interactive,
}