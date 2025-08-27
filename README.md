# Task Manager CLI

A powerful and user-friendly command-line task management tool built with Rust. Organize your tasks with priorities, notes, subtasks, and time tracking capabilities.

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Build](https://img.shields.io/badge/build-passing-green)

## 🚀 Features

- **Priority Management**: High, medium, and low priority levels with color coding
- **Task Notes**: Add detailed notes and context to your tasks
- **Subtasks**: Break down complex tasks into manageable subtasks
- **Time Tracking**: Start/stop timers to track time spent on tasks
- **Interactive Mode**: User-friendly menu interface for easy operation
- **Color-Coded Output**: Visual priority indicators and status displays
- **Soft Delete**: Tasks are hidden but preserved in the data file
- **Persistent Storage**: All data stored in JSON format locally

## 📦 Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo package manager

### Build from Source
```bash
# Clone the repository
git clone <repository-url>
cd my_cli

# Build the project
cargo build --release

# Run the application
cargo run
```

### Direct Usage
```bash
# Run without building
cargo run -- <command>

# Or after building
./target/release/my_cli <command>
```

## 🎯 Quick Start

### First Time Setup
When you run the tool without arguments, you'll see a welcome message with quick start guide:

```bash
cargo run
```

### Basic Usage
```bash
# Add your first task
cargo run -- add "Complete project proposal" --priority high

# List all tasks
cargo run -- list

# Mark a task as done
cargo run -- done 1

# Start interactive mode (recommended for beginners)
cargo run -- interactive
```

## 📖 Usage Guide

### Command Reference

#### Basic Commands
```bash
# Add a new task
cargo run -- add "Task description" [--priority low|medium|high]

# List all active tasks
cargo run -- list

# Mark task as completed
cargo run -- done <task_id>

# Delete a task (soft delete)
cargo run -- delete <task_id>
```

#### Advanced Features
```bash
# Add a note to a task
cargo run -- note <task_id> "Your detailed note here"

# Add a subtask
cargo run -- subtask <task_id> "Subtask description"

# Time tracking
cargo run -- start <task_id>  # Start timer
cargo run -- stop <task_id>   # Stop timer and record time
```

#### Help & Information
```bash
# Show basic help
cargo run -- --help

# Show detailed usage guide
cargo run -- usage

# Get help for specific command
cargo run -- <command> --help
```

### Interactive Mode
For the best user experience, use interactive mode:

```bash
cargo run -- interactive
```

This provides a menu-driven interface where you can:
- Navigate with arrow keys
- Select tasks from lists
- Access all features through simple menus
- No need to remember command syntax

## 🎨 Features in Detail

### Priority System
Tasks can have three priority levels:
- **🔴 High**: Critical tasks (displayed in red)
- **🟡 Medium**: Important tasks (displayed in yellow)
- **🟢 Low**: Optional tasks (displayed in green)

### Task Display
The list view shows:
- Task ID and status (✓ Done / ✗ Todo)
- Priority with color coding
- Description
- Creation date
- Time spent (if tracked)
- Notes (if added)
- Subtasks with completion status

### Time Tracking
- Start timers on any task
- Stop timers to record time spent
- View accumulated time in the task list
- Time displayed in human-readable format (hours/minutes/seconds)

### Data Storage
- All data stored in `tasks.json` in the current directory
- JSON format for easy backup and portability
- Soft delete preserves data integrity
- Automatic file creation on first use

## 🔧 Configuration

### Data Location
Tasks are stored in `tasks.json` in the current working directory. To use a different location, you can:
- Change directory before running commands
- Move/copy the `tasks.json` file to your preferred location

### Customizing Colors
The application uses colored output by default. If you prefer plain text:
- The colors are handled by the `colored` crate
- No configuration file is currently supported

## 🐛 Troubleshooting

### Common Issues

**"Task not found" error**
- Use `cargo run -- list` to see available task IDs
- Check if the task was deleted (soft delete hides tasks)

**Invalid priority error**
- Use only: `low`, `medium`, or `high`
- Case-insensitive: `Low`, `Medium`, `High` also work

**Timer already running**
- Stop the current timer before starting a new one
- Use `cargo run -- stop <task_id>` to stop active timers

### Getting Help
```bash
# Welcome screen with quick start
cargo run

# Built-in help
cargo run -- --help

# Detailed usage guide
cargo run -- usage

# Interactive help
cargo run -- interactive
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Development Setup
```bash
# Clone the repository
git clone <repository-url>
cd my_cli

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy
```

### Adding Features
1. Create a feature branch: `git checkout -b feature/your-feature-name`
2. Make your changes
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Format code: `cargo fmt`
6. Run linter: `cargo clippy`
7. Commit your changes
8. Push and create a pull request

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add comments for complex logic
- Write tests for new features

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI framework: [clap](https://github.com/clap-rs/clap)
- Color output: [colored](https://github.com/colored-rs/colored)
- Interactive prompts: [dialoguer](https://github.com/console-rs/dialoguer)
- Date/time handling: [chrono](https://github.com/chronotope/chrono)
- JSON serialization: [serde](https://github.com/serde-rs/serde)

## 📞 Support

If you encounter any issues or have questions:

1. Check the [troubleshooting section](#-troubleshooting)
2. Use the built-in help: `cargo run -- usage`
3. Try interactive mode: `cargo run -- interactive`
4. Create an issue in the repository

---

**Happy task managing! 🎯**