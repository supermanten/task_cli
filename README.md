# Task Manager CLI

A comprehensive and powerful command-line task management tool built with Rust. Features advanced project management capabilities with filtering, boards, tags, due dates, and multiple view modes.

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Build](https://img.shields.io/badge/build-passing-green)

## 🚀 Features

### Core Features
- **Priority Management**: High, medium, and low priority levels with color coding
- **Task Notes**: Add detailed notes and context to your tasks
- **Subtasks**: Break down complex tasks into manageable subtasks
- **Time Tracking**: Start/stop timers to track time spent on tasks
- **Interactive Mode**: User-friendly menu interface for easy operation
- **Color-Coded Output**: Visual priority indicators and status displays
- **Soft Delete**: Tasks are hidden but preserved in the data file
- **Persistent Storage**: All data stored in JSON format locally

### Advanced Features
- **🔍 Advanced Filtering**: Filter by priority, project, tags, due dates, and description
- **📋 Board System**: Kanban-style boards (Todo, In Progress, Done) with custom boards
- **🏷️ Tags & Categories**: Organize tasks with flexible tagging system
- **📅 Due Dates**: Set and track task deadlines with calendar view
- **✅ Checklists**: Add checklist items to tasks for detailed tracking
- **📊 Multiple Views**: Calendar, timeline, and focus views
- **💾 Data Export**: Export tasks to CSV or JSON formats
- **🎯 Project Management**: Group tasks by projects with advanced organization

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

### Advanced Usage Examples
```bash
# Add a comprehensive task with all features
cargo run -- add "Build user authentication" --priority high -P "Backend" --tags "security,urgent" --due 2024-02-15

# Filter tasks by various criteria
cargo run -- list --priority high
cargo run -- list --project Backend
cargo run -- list --tag urgent
cargo run -- list --due today

# Work with boards
cargo run -- board add "Sprint 1"
cargo run -- board move 1 "In Progress"
cargo run -- board show

# Add checklist items
cargo run -- checklist 1 "Design database schema"
cargo run -- checklist 1 "Implement JWT tokens"

# Use different views
cargo run -- calendar
cargo run -- timeline
cargo run -- focus urgent

# Export your data
cargo run -- export csv tasks.csv
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

#### Advanced Task Management
```bash
# Add comprehensive task with all options
cargo run -- add "Task" --priority high -P "Project" --tags "tag1,tag2" --due 2024-02-15

# Add notes and subtasks
cargo run -- note <task_id> "Your detailed note here"
cargo run -- subtask <task_id> "Subtask description"

# Time tracking
cargo run -- start <task_id>  # Start timer
cargo run -- stop <task_id>   # Stop timer and record time
```

#### Organization & Metadata
```bash
# Tags management
cargo run -- tag <task_id> <tag>     # Add tag
cargo run -- untag <task_id> <tag>   # Remove tag

# Project management
cargo run -- project <task_id> "Project Name"

# Due dates
cargo run -- due <task_id> 2024-02-15

# Checklists
cargo run -- checklist <task_id> "Checklist item"
cargo run -- check <task_id> <item_id>  # Toggle checklist item
```

#### Board Management
```bash
# Board operations
cargo run -- board add "Board Name"
cargo run -- board list
cargo run -- board show
cargo run -- board move <task_id> "Board Name"
```

#### Advanced Filtering
```bash
# Filter by priority
cargo run -- list --priority high

# Filter by project
cargo run -- list --project "Backend"

# Search by description
cargo run -- list --description "meeting"

# Filter by due date
cargo run -- list --due today
cargo run -- list --due week
cargo run -- list --due overdue

# Filter by tag
cargo run -- list --tag urgent

# Filter by board
cargo run -- list --board "In Progress"
```

#### Special Views
```bash
# Calendar view (by due date)
cargo run -- calendar

# Timeline view (by creation date)
cargo run -- timeline

# Focus view (by tag/project/board)
cargo run -- focus urgent
cargo run -- focus "Backend"
cargo run -- focus "In Progress"
```

#### Data Management
```bash
# Export data
cargo run -- export csv tasks.csv
cargo run -- export json tasks.json
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

### Advanced Organization
- **Projects**: Group related tasks under project names
- **Tags**: Flexible labeling system for categorization
- **Boards**: Kanban-style workflow management (Todo, In Progress, Done)
- **Due Dates**: Set deadlines with calendar integration

### Task Display
The enhanced list view shows:
- Task ID and compact status icons (✓/✗)
- Priority with color coding
- Description with progress indicators
- Project name
- Due date (MM-DD format)
- Time spent tracking
- Tags display
- Checklist progress ([completed/total])
- Board information
- Notes and subtasks (when expanded)

### Filtering & Searching
Powerful filtering capabilities:
- **Priority filtering**: Show only high/medium/low priority tasks
- **Project filtering**: Focus on specific project tasks
- **Tag filtering**: Find tasks by tags
- **Date filtering**: Today, this week, overdue tasks
- **Description search**: Text search within task descriptions
- **Board filtering**: View tasks from specific boards

### Board System
Kanban-style project management:
- **Default boards**: Todo, In Progress, Done
- **Custom boards**: Create unlimited custom boards
- **Task movement**: Move tasks between boards
- **Board view**: Visual kanban board display

### Checklists
Break down tasks with detailed checklists:
- Add multiple checklist items to any task
- Toggle items as completed
- Progress tracking ([2/5] completed)
- Visual checkbox indicators (☑/☐)

### Multiple View Modes
- **Calendar View**: Tasks organized by due date
- **Timeline View**: Chronological view by creation date
- **Focus View**: Filtered view by tag, project, or board
- **Board View**: Kanban-style board visualization

### Time Tracking
- Start timers on any task
- Stop timers to record time spent
- View accumulated time in the task list
- Time displayed in human-readable format (hours/minutes/seconds)
- Automatic time calculation

### Data Management
- **CSV Export**: Export tasks with all metadata
- **JSON Export**: Full data export for backup/integration
- **Persistent Storage**: All data stored in `tasks.json`
- **Soft Delete**: Tasks are hidden but preserved in data file
- **Migration Support**: Handles schema changes gracefully

### Interactive Mode
Enhanced menu-driven interface:
- All features accessible through menus
- No need to remember command syntax
- Guided task creation with all options
- Visual selection from task lists
- Real-time updates and feedback

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
- Verify the task ID is correct

**Invalid priority error**
- Use only: `low`, `medium`, or `high`
- Case-insensitive: `Low`, `Medium`, `High` also work

**Timer already running**
- Stop the current timer before starting a new one
- Use `cargo run -- stop <task_id>` to stop active timers

**Date parsing errors**
- Use formats: `YYYY-MM-DD` or `YYYY-MM-DD HH:MM`
- Examples: `2024-02-15` or `2024-02-15 14:30`

**Board not found**
- Check available boards with `cargo run -- board list`
- Create new boards with `cargo run -- board add "Board Name"`

**Export failures**
- Ensure write permissions in current directory
- Check if file already exists (will be overwritten)

### Advanced Filtering Issues

**No tasks found with filter**
- Try broader filters or remove filters to see all tasks
- Check spelling in project names and tags
- Use `cargo run -- list` without filters first

**Due date filtering not working**
- Ensure dates are in the future for "week" filter
- Use `overdue` to find past due tasks
- Check date format: YYYY-MM-DD

### Interactive Mode Issues

**Menu not displaying correctly**
- Ensure you're running in a proper terminal
- Try different terminal applications
- Check terminal encoding supports Unicode characters

**Selection not working**
- Use arrow keys to navigate
- Press Enter to select
- Press Ctrl+C to exit menus

### Data Recovery

**Lost tasks after update**
- Check `tasks.json` file in current directory
- Tasks are never truly deleted, only marked as deleted
- Contact support if data appears corrupted

### Performance Issues

**Slow startup**
- Large task files may load slowly
- Consider archiving old completed tasks
- Use filters to limit displayed tasks

**Memory usage**
- Application loads all tasks into memory
- Consider periodic cleanup of old tasks
- Export and archive completed projects

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

# Command-specific help
cargo run -- <command> --help
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
- Advanced terminal UI: [console](https://github.com/console-rs/console)

### Feature Inspiration
- **TaskWarrior**: Advanced filtering and command-line task management
- **Todo.txt**: Simple text-based task management
- **Trello**: Board and card-based project management
- **Microsoft Todo**: Rich task metadata and organization
- **Things 3**: Smart organization and focus modes

## 📊 Data Model

### Task Structure
Each task contains the following fields:

- **id**: Unique identifier
- **description**: Task description
- **done**: Completion status
- **created_at**: Creation timestamp
- **deleted_at**: Soft delete timestamp (optional)
- **priority**: High/Medium/Low priority level
- **notes**: Optional detailed notes
- **subtasks**: List of subtasks with completion status
- **time_spent**: Accumulated time tracking in seconds
- **timer_start**: Current timer start time (optional)
- **project**: Associated project name (optional)
- **due_date**: Deadline timestamp (optional)
- **tags**: List of tag strings
- **board**: Current board name (optional)
- **checklists**: List of checklist items with completion status

### File Storage
- **tasks.json**: Main task data file
- **boards.json**: Board configuration file
- JSON format for easy backup and portability
- Automatic schema migration for updates

## 🎯 Advanced Use Cases

### Project Management
```bash
# Set up a new project
cargo run -- add "Plan project architecture" --priority high -P "Web App" --tags "planning,architecture"
cargo run -- add "Design database schema" --priority high -P "Web App" --tags "database,design"
cargo run -- add "Implement user authentication" --priority medium -P "Web App" --tags "backend,security"

# View project tasks
cargo run -- list --project "Web App"

# Move to development board
cargo run -- board move 1 "In Progress"
cargo run -- board move 2 "In Progress"
```

### Sprint Planning
```bash
# Create sprint board
cargo run -- board add "Sprint 1"

# Add sprint tasks with due dates
cargo run -- add "Implement login page" --priority high -P "Frontend" --due 2024-02-10 --tags "ui,authentication"
cargo run -- add "Create API endpoints" --priority high -P "Backend" --due 2024-02-12 --tags "api,backend"

# Move to sprint board
cargo run -- board move 4 "Sprint 1"
cargo run -- board move 5 "Sprint 1"

# View sprint progress
cargo run -- board show
cargo run -- focus "Sprint 1"
```

### Personal Productivity
```bash
# Daily planning
cargo run -- add "Morning exercise" --priority medium --tags "health,personal" --due today
cargo run -- add "Review pull requests" --priority high --tags "work,urgent" --due today
cargo run -- add "Grocery shopping" --priority low --tags "personal,shopping"

# Focus on work tasks
cargo run -- focus work

# Check overdue tasks
cargo run -- list --due overdue
```

## 📞 Support

If you encounter any issues or have questions:

1. Check the [troubleshooting section](#-troubleshooting)
2. Use the built-in help: `cargo run -- usage`
3. Try interactive mode: `cargo run -- interactive`
4. Review the [advanced use cases](#-advanced-use-cases)
5. Create an issue in the repository

### Feature Requests
We welcome feature requests! The current architecture supports easy extension for:
- External integrations (GitHub, Jira, etc.)
- Custom fields and metadata
- Advanced reporting and analytics
- Team collaboration features
- Mobile/web companion apps

---

**Happy advanced task managing! 🚀**