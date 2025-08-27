mod cli;
mod controller;
mod model;
mod view;

use clap::Parser;
use cli::Cli;
use controller::handle_command;
use model::TaskManager;

fn main() {
    let cli = Cli::parse();
    let mut task_manager = TaskManager::new();

    if let Some(command) = cli.command {
        handle_command(command, &mut task_manager);
    } else {
        show_welcome();
    }
}

fn show_welcome() {
    println!("🚀 Welcome to Task Manager CLI!");
    println!();
    println!("This tool helps you manage your tasks efficiently with features like:");
    println!("• Priority levels with color coding");
    println!("• Task notes and subtasks");
    println!("• Time tracking");
    println!("• Interactive mode");
    println!();
    println!("📖 Quick Start:");
    println!("  • Add a task: task_manager add \"My first task\" --priority high");
    println!("  • List tasks: task_manager list");
    println!("  • Interactive mode: task_manager interactive");
    println!("  • Get help: task_manager --help");
    println!("  • Detailed help: task_manager help");
    println!();
    println!("💡 Tip: Use 'task_manager interactive' for an easy-to-use menu interface!");
}
