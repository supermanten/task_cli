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
    Add { description: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
}