use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Wrap,
    },
    Frame,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::stdout,
    time::Duration,
};

use crate::model::{TaskManager, Priority};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    MainMenu,
    TaskList,
    AddTask,
    TaskDetails(usize),
    Help,
}

#[derive(Debug)]
pub struct App {
    pub state: AppState,
    pub task_manager: TaskManager,
    pub selected_task: usize,
    pub scroll_position: usize,
    pub input_buffer: String,
    pub input_field: InputField,
    pub should_quit: bool,
}

#[derive(Debug)]
pub enum InputField {
    None,
    TaskDescription,
    TaskPriority,
    TaskProject,
    TaskTags,
    TaskDueDate,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::MainMenu,
            task_manager: TaskManager::new(),
            selected_task: 0,
            scroll_position: 0,
            input_buffer: String::new(),
            input_field: InputField::None,
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);
                }
            }

            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn handle_input(&mut self, key: event::KeyEvent) {
        match self.state {
            AppState::MainMenu => self.handle_main_menu_input(key),
            AppState::TaskList => self.handle_task_list_input(key),
            AppState::AddTask => self.handle_add_task_input(key),
            AppState::TaskDetails(_) => self.handle_task_details_input(key),
            AppState::Help => self.handle_help_input(key),
        }
    }

    fn handle_main_menu_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('1') => self.state = AppState::TaskList,
            KeyCode::Char('2') => {
                self.state = AppState::AddTask;
                self.input_field = InputField::TaskDescription;
            }
            KeyCode::Char('3') => self.state = AppState::Help,
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_task_list_input(&mut self, key: event::KeyEvent) {
        let tasks = self.task_manager.get_active_tasks();
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_task < tasks.len().saturating_sub(1) {
                    self.selected_task += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_task > 0 {
                    self.selected_task -= 1;
                }
            }
            KeyCode::Enter => {
                if !tasks.is_empty() {
                    self.state = AppState::TaskDetails(self.selected_task);
                }
            }
            KeyCode::Char('d') => {
                if !tasks.is_empty() {
                    let task_id = tasks[self.selected_task].id;
                    self.task_manager.mark_done(task_id);
                }
            }
            KeyCode::Char('a') => {
                self.state = AppState::AddTask;
                self.input_field = InputField::TaskDescription;
            }
            KeyCode::Esc | KeyCode::Char('q') => self.state = AppState::MainMenu,
            _ => {}
        }
    }

    fn handle_add_task_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                match self.input_field {
                    InputField::TaskDescription => {
                        // For now, just add with default priority
                        self.task_manager.add_task_with_options(
                            self.input_buffer.clone(),
                            Priority::Medium,
                            None,
                            None,
                            Vec::new(),
                        );
                        self.input_buffer.clear();
                        self.state = AppState::TaskList;
                        self.input_field = InputField::None;
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Esc => {
                self.input_buffer.clear();
                self.state = AppState::TaskList;
                self.input_field = InputField::None;
            }
            _ => {}
        }
    }

    fn handle_task_details_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.state = AppState::TaskList;
            }
            KeyCode::Char('d') => {
                if let AppState::TaskDetails(index) = self.state {
                    let tasks = self.task_manager.get_active_tasks();
                    if index < tasks.len() {
                        let task_id = tasks[index].id;
                        self.task_manager.mark_done(task_id);
                        self.state = AppState::TaskList;
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_help_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.state = AppState::MainMenu,
            _ => {}
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let size = f.size();

        match self.state {
            AppState::MainMenu => self.draw_main_menu(f, size),
            AppState::TaskList => self.draw_task_list(f, size),
            AppState::AddTask => self.draw_add_task(f, size),
            AppState::TaskDetails(index) => self.draw_task_details(f, size, index),
            AppState::Help => self.draw_help(f, size),
        }
    }

    fn draw_main_menu(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(area);

        // Title
        let title = Paragraph::new("🚀 Rio Task Manager")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Menu options
        let menu_items = vec![
            "1. 📋 View Tasks",
            "2. ➕ Add New Task",
            "3. ❓ Help",
            "q. 🚪 Quit",
        ];

        let menu = List::new(
            menu_items
                .iter()
                .map(|item| ListItem::new(*item))
                .collect::<Vec<_>>(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Main Menu")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(Color::White));

        f.render_widget(menu, chunks[1]);

        // Footer
        let footer = Paragraph::new("Use number keys or arrow keys to navigate")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    }

    fn draw_task_list(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new("📋 Your Tasks")
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Task list
        let tasks = self.task_manager.get_active_tasks();
        let items: Vec<ListItem> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let status = if task.done { "✅" } else { "⏳" };
                let priority_color = match task.priority {
                    Priority::High => Color::Red,
                    Priority::Medium => Color::Yellow,
                    Priority::Low => Color::Green,
                };

                let style = if i == self.selected_task {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!("{} ", status), Style::default().fg(Color::Green)),
                    Span::styled(&task.description, style),
                    Span::styled(format!(" ({:?})", task.priority), Style::default().fg(priority_color)),
                ]))
            })
            .collect();

        let task_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Tasks ({})", tasks.len()))
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(Style::default().bg(Color::Blue));

        f.render_widget(task_list, chunks[1]);

        // Footer
        let footer = Paragraph::new("↑/↓ Navigate • Enter: Details • d: Mark Done • a: Add Task • Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    }

    fn draw_add_task(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new("➕ Add New Task")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Input field
        let input = Paragraph::new(format!("Description: {}", self.input_buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Task Description"));
        f.render_widget(input, chunks[1]);

        // Instructions
        let instructions = Paragraph::new("Type your task description and press Enter to add")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(instructions, chunks[2]);

        // Footer
        let footer = Paragraph::new("Enter: Save • Esc: Cancel")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[3]);
    }

    fn draw_task_details(&self, f: &mut Frame, area: Rect, index: usize) {
        let tasks = self.task_manager.get_active_tasks();
        if index >= tasks.len() {
            return;
        }

        let task = &tasks[index];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new(format!("📝 Task Details - {}", task.description))
            .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Task details
        let priority_color = match task.priority {
            Priority::High => Color::Red,
            Priority::Medium => Color::Yellow,
            Priority::Low => Color::Green,
        };

        let details = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::White)),
                Span::styled(if task.done { "✅ Done" } else { "⏳ In Progress" }, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(Color::White)),
                Span::styled(format!("{:?}", task.priority), Style::default().fg(priority_color)),
            ]),
            Line::from(vec![
                Span::styled("Created: ", Style::default().fg(Color::White)),
                Span::styled(task.created_at.format("%Y-%m-%d %H:%M").to_string(), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("Time Spent: ", Style::default().fg(Color::White)),
                Span::styled(format_time(task.time_spent), Style::default().fg(Color::Yellow)),
            ]),
        ];

        let details_paragraph = Paragraph::new(Text::from(details))
            .block(Block::default().borders(Borders::ALL).title("Task Information"))
            .wrap(Wrap { trim: true });
        f.render_widget(details_paragraph, chunks[1]);

        // Footer
        let footer = Paragraph::new("d: Mark Done • Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    }

    fn draw_help(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new("❓ Help & Shortcuts")
            .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Help content
        let help_text = vec![
            Line::from("🧭 Navigation:"),
            Line::from("  • Use number keys in main menu"),
            Line::from("  • Arrow keys or j/k to navigate lists"),
            Line::from("  • Enter to select/view details"),
            Line::from("  • Esc to go back"),
            Line::from(""),
            Line::from("⚡ Actions:"),
            Line::from("  • d: Mark task as done"),
            Line::from("  • a: Add new task"),
            Line::from("  • q: Quit application"),
            Line::from(""),
            Line::from("🎨 Interface:"),
            Line::from("  • Color-coded priorities (🔴 High, 🟡 Medium, 🟢 Low)"),
            Line::from("  • Status icons (✅ Done, ⏳ In Progress)"),
            Line::from("  • Clean, modern terminal interface"),
        ];

        let help_paragraph = Paragraph::new(Text::from(help_text))
            .block(Block::default().borders(Borders::ALL).title("Rio TUI Help"))
            .wrap(Wrap { trim: true });
        f.render_widget(help_paragraph, chunks[1]);

        // Footer
        let footer = Paragraph::new("Esc: Back to Main Menu")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    }
}

fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.run()
}