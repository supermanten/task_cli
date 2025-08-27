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

    handle_command(cli.command, &mut task_manager);
}
