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
    TaskActions(usize),
    AddNote(usize),
    AddSubtask(usize),
    AddTag(usize),
    SetProject(usize),
    SetDueDate(usize),
    AddChecklistItem(usize),
    BoardView,
    CalendarView,
    TimelineView,
    FocusView,
    ExportView,
    Help,
}

#[derive(Debug)]
pub struct App {
    pub state: AppState,
    pub task_manager: TaskManager,
    pub selected_task: usize,
    pub selected_board: usize,
    pub selected_export: usize,
    pub scroll_position: usize,
    pub input_buffer: String,
    pub input_field: InputField,
    pub should_quit: bool,
    pub current_task_id: Option<u32>,
    pub filter_query: String,
    pub focus_context: String,
}

#[derive(Debug)]
pub enum InputField {
    None,
    TaskDescription,
    TaskPriority,
    TaskProject,
    TaskTags,
    TaskDueDate,
    NoteContent,
    SubtaskDescription,
    TagName,
    ProjectName,
    DueDate,
    ChecklistItem,
    FilterQuery,
    FocusContext,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::MainMenu,
            task_manager: TaskManager::new(),
            selected_task: 0,
            selected_board: 0,
            selected_export: 0,
            scroll_position: 0,
            input_buffer: String::new(),
            input_field: InputField::None,
            should_quit: false,
            current_task_id: None,
            filter_query: String::new(),
            focus_context: String::new(),
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
            AppState::TaskActions(_) => self.handle_task_actions_input(key),
            AppState::AddNote(_) => self.handle_add_note_input(key),
            AppState::AddSubtask(_) => self.handle_add_subtask_input(key),
            AppState::AddTag(_) => self.handle_add_tag_input(key),
            AppState::SetProject(_) => self.handle_set_project_input(key),
            AppState::SetDueDate(_) => self.handle_set_due_date_input(key),
            AppState::AddChecklistItem(_) => self.handle_add_checklist_item_input(key),
            AppState::BoardView => self.handle_board_view_input(key),
            AppState::CalendarView => self.handle_calendar_view_input(key),
            AppState::TimelineView => self.handle_timeline_view_input(key),
            AppState::FocusView => self.handle_focus_view_input(key),
            AppState::ExportView => self.handle_export_view_input(key),
            AppState::Help => self.handle_help_input(key),
        }
    }

    fn handle_main_menu_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('1') => {
                self.state = AppState::TaskList;
                self.reset_task_selection();
            }
            KeyCode::Char('2') => {
                self.state = AppState::AddTask;
                self.input_field = InputField::TaskDescription;
            }
            KeyCode::Char('3') => self.state = AppState::BoardView,
            KeyCode::Char('4') => self.state = AppState::CalendarView,
            KeyCode::Char('5') => self.state = AppState::TimelineView,
            KeyCode::Char('6') => {
                self.state = AppState::FocusView;
                self.input_field = InputField::FocusContext;
            }
            KeyCode::Char('7') => self.state = AppState::ExportView,
            KeyCode::Char('8') => self.state = AppState::Help,
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_task_list_input(&mut self, key: event::KeyEvent) {
        let tasks = self.task_manager.get_active_tasks();
        let max_index = tasks.len().saturating_sub(1);

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_task < max_index {
                    self.selected_task += 1;
                } else if !tasks.is_empty() {
                    // Wrap to beginning if at end
                    self.selected_task = 0;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_task > 0 {
                    self.selected_task -= 1;
                } else if !tasks.is_empty() {
                    // Wrap to end if at beginning
                    self.selected_task = max_index;
                }
            }
            KeyCode::PageDown => {
                // Jump down by 5 items
                self.selected_task = (self.selected_task + 5).min(max_index);
            }
            KeyCode::PageUp => {
                // Jump up by 5 items
                self.selected_task = self.selected_task.saturating_sub(5);
            }
            KeyCode::Home => {
                // Go to first item
                self.selected_task = 0;
            }
            KeyCode::End => {
                // Go to last item
                self.selected_task = max_index;
            }
            KeyCode::Enter => {
                if !tasks.is_empty() {
                    self.state = AppState::TaskActions(self.selected_task);
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
            KeyCode::Char('n') => {
                if !tasks.is_empty() {
                    self.current_task_id = Some(tasks[self.selected_task].id);
                    self.state = AppState::AddNote(self.selected_task);
                    self.input_field = InputField::NoteContent;
                }
            }
            KeyCode::Char('s') => {
                if !tasks.is_empty() {
                    self.current_task_id = Some(tasks[self.selected_task].id);
                    self.state = AppState::AddSubtask(self.selected_task);
                    self.input_field = InputField::SubtaskDescription;
                }
            }
            KeyCode::Char('t') => {
                if !tasks.is_empty() {
                    self.current_task_id = Some(tasks[self.selected_task].id);
                    self.state = AppState::AddTag(self.selected_task);
                    self.input_field = InputField::TagName;
                }
            }
            KeyCode::Char('p') => {
                if !tasks.is_empty() {
                    self.current_task_id = Some(tasks[self.selected_task].id);
                    self.state = AppState::SetProject(self.selected_task);
                    self.input_field = InputField::ProjectName;
                }
            }
            KeyCode::Char('c') => {
                if !tasks.is_empty() {
                    self.current_task_id = Some(tasks[self.selected_task].id);
                    self.state = AppState::AddChecklistItem(self.selected_task);
                    self.input_field = InputField::ChecklistItem;
                }
            }
            KeyCode::Char('/') => {
                self.input_field = InputField::FilterQuery;
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
                        // Stay on the same task details but refresh the view
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_task_actions_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('d') => {
                if let AppState::TaskActions(index) = self.state {
                    let tasks = self.task_manager.get_active_tasks();
                    if index < tasks.len() {
                        let task_id = tasks[index].id;
                        self.task_manager.mark_done(task_id);
                        self.state = AppState::TaskList;
                    }
                }
            }
            KeyCode::Char('n') => {
                if let AppState::TaskActions(index) = self.state {
                    self.current_task_id = Some(self.task_manager.get_active_tasks()[index].id);
                    self.state = AppState::AddNote(index);
                    self.input_field = InputField::NoteContent;
                }
            }
            KeyCode::Char('s') => {
                if let AppState::TaskActions(index) = self.state {
                    self.current_task_id = Some(self.task_manager.get_active_tasks()[index].id);
                    self.state = AppState::AddSubtask(index);
                    self.input_field = InputField::SubtaskDescription;
                }
            }
            KeyCode::Char('t') => {
                if let AppState::TaskActions(index) = self.state {
                    self.current_task_id = Some(self.task_manager.get_active_tasks()[index].id);
                    self.state = AppState::AddTag(index);
                    self.input_field = InputField::TagName;
                }
            }
            KeyCode::Char('p') => {
                if let AppState::TaskActions(index) = self.state {
                    self.current_task_id = Some(self.task_manager.get_active_tasks()[index].id);
                    self.state = AppState::SetProject(index);
                    self.input_field = InputField::ProjectName;
                }
            }
            KeyCode::Char('c') => {
                if let AppState::TaskActions(index) = self.state {
                    self.current_task_id = Some(self.task_manager.get_active_tasks()[index].id);
                    self.state = AppState::AddChecklistItem(index);
                    self.input_field = InputField::ChecklistItem;
                }
            }
            KeyCode::Char('r') => {
                if let AppState::TaskActions(index) = self.state {
                    let tasks = self.task_manager.get_active_tasks();
                    if index < tasks.len() {
                        let task_id = tasks[index].id;
                        if tasks[index].timer_start.is_some() {
                            self.task_manager.stop_timer(task_id);
                        } else {
                            self.task_manager.start_timer(task_id);
                        }
                    }
                }
            }
            KeyCode::Char('v') => {
                if let AppState::TaskActions(index) = self.state {
                    self.state = AppState::TaskDetails(index);
                }
            }
            KeyCode::Esc => {
                self.state = AppState::TaskList;
            }
            _ => {}
        }
    }

    fn handle_add_note_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(task_id) = self.current_task_id {
                    self.task_manager.add_note(task_id, self.input_buffer.clone());
                    self.input_buffer.clear();
                    self.input_field = InputField::None;
                    self.state = AppState::TaskList;
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
                self.input_field = InputField::None;
                self.state = AppState::TaskList;
            }
            _ => {}
        }
    }

    fn handle_add_subtask_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(task_id) = self.current_task_id {
                    self.task_manager.add_subtask(task_id, self.input_buffer.clone());
                    self.input_buffer.clear();
                    self.input_field = InputField::None;
                    self.state = AppState::TaskList;
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
                self.input_field = InputField::None;
                self.state = AppState::TaskList;
            }
            _ => {}
        }
    }

    fn handle_add_tag_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(task_id) = self.current_task_id {
                    self.task_manager.add_tag(task_id, self.input_buffer.clone());
                    self.input_buffer.clear();
                    self.input_field = InputField::None;
                    self.state = AppState::TaskList;
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
                self.input_field = InputField::None;
                self.state = AppState::TaskList;
            }
            _ => {}
        }
    }

    fn handle_set_project_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(task_id) = self.current_task_id {
                    self.task_manager.set_project(task_id, self.input_buffer.clone());
                    self.input_buffer.clear();
                    self.input_field = InputField::None;
                    self.state = AppState::TaskList;
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
                self.input_field = InputField::None;
                self.state = AppState::TaskList;
            }
            _ => {}
        }
    }

    fn handle_set_due_date_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(task_id) = self.current_task_id {
                    // For simplicity, we'll just set a dummy date for now
                    // In a real implementation, you'd parse the date string
                    let dummy_date = chrono::Utc::now() + chrono::Duration::days(7);
                    self.task_manager.set_due_date(task_id, dummy_date);
                    self.input_buffer.clear();
                    self.input_field = InputField::None;
                    self.state = AppState::TaskList;
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
                self.input_field = InputField::None;
                self.state = AppState::TaskList;
            }
            _ => {}
        }
    }

    fn handle_add_checklist_item_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(task_id) = self.current_task_id {
                    self.task_manager.add_checklist_item(task_id, self.input_buffer.clone());
                    self.input_buffer.clear();
                    self.input_field = InputField::None;
                    self.state = AppState::TaskList;
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
                self.input_field = InputField::None;
                self.state = AppState::TaskList;
            }
            _ => {}
        }
    }

    fn handle_board_view_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.state = AppState::MainMenu,
            _ => {}
        }
    }

    fn handle_calendar_view_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.state = AppState::MainMenu,
            _ => {}
        }
    }

    fn handle_timeline_view_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.state = AppState::MainMenu,
            _ => {}
        }
    }

    fn handle_focus_view_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                // Search is already handled in real-time
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Esc => {
                self.input_buffer.clear();
                self.input_field = InputField::None;
                self.state = AppState::MainMenu;
            }
            _ => {}
        }
    }

    fn handle_export_view_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('1') => {
                // Export as CSV
                let _ = self.task_manager.export_csv("tasks_export.csv");
                self.state = AppState::MainMenu;
            }
            KeyCode::Char('2') => {
                // Export as JSON
                let _ = self.task_manager.export_json("tasks_export.json");
                self.state = AppState::MainMenu;
            }
            KeyCode::Esc => self.state = AppState::MainMenu,
            _ => {}
        }
    }

    fn handle_help_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.state = AppState::MainMenu,
            _ => {}
        }
    }

    fn reset_task_selection(&mut self) {
        let tasks = self.task_manager.get_active_tasks();
        if tasks.is_empty() {
            self.selected_task = 0;
        } else if self.selected_task >= tasks.len() {
            self.selected_task = tasks.len().saturating_sub(1);
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let size = f.size();

        match self.state {
            AppState::MainMenu => self.draw_main_menu(f, size),
            AppState::TaskList => self.draw_task_list(f, size),
            AppState::AddTask => self.draw_add_task(f, size),
            AppState::TaskDetails(index) => self.draw_task_details(f, size, index),
            AppState::TaskActions(index) => self.draw_task_actions(f, size, index),
            AppState::AddNote(index) => self.draw_add_note(f, size, index),
            AppState::AddSubtask(index) => self.draw_add_subtask(f, size, index),
            AppState::AddTag(index) => self.draw_add_tag(f, size, index),
            AppState::SetProject(index) => self.draw_set_project(f, size, index),
            AppState::SetDueDate(index) => self.draw_set_due_date(f, size, index),
            AppState::AddChecklistItem(index) => self.draw_add_checklist_item(f, size, index),
            AppState::BoardView => self.draw_board_view(f, size),
            AppState::CalendarView => self.draw_calendar_view(f, size),
            AppState::TimelineView => self.draw_timeline_view(f, size),
            AppState::FocusView => self.draw_focus_view(f, size),
            AppState::ExportView => self.draw_export_view(f, size),
            AppState::Help => self.draw_help(f, size),
        }
    }

    fn draw_main_menu(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(15),
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
            "3. 📊 Board View",
            "4. 📅 Calendar View",
            "5. ⏰ Timeline View",
            "6. 🎯 Focus View",
            "7. 💾 Export Data",
            "8. ❓ Help",
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
        let footer = Paragraph::new("Use number keys to select • q: Quit")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    }

    fn draw_task_list(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Task list
        let tasks = self.task_manager.get_active_tasks();

        // Header with selection info
        let header_text = if tasks.is_empty() {
            "📋 Your Tasks (No tasks)".to_string()
        } else {
            format!("📋 Your Tasks ({}/{})", self.selected_task + 1, tasks.len())
        };

        let header = Paragraph::new(header_text)
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);
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

                // Build metadata string
                let mut metadata_parts = Vec::new();

                // Add project if exists
                if let Some(project) = &task.project {
                    metadata_parts.push(format!("📁 {}", project));
                }

                // Add tags if exist
                if !task.tags.is_empty() {
                    metadata_parts.push(format!("🏷️ {}", task.tags.join(", ")));
                }

                // Add note indicator if exists
                if task.notes.is_some() {
                    metadata_parts.push("📝 Has note".to_string());
                }

                // Add subtasks indicator if exist
                if !task.subtasks.is_empty() {
                    let done_count = task.subtasks.iter().filter(|st| st.done).count();
                    metadata_parts.push(format!("➕ {}/{}", done_count, task.subtasks.len()));
                }

                // Add checklist indicator if exist
                if !task.checklists.is_empty() {
                    let done_count = task.checklists.iter().filter(|ci| ci.done).count();
                    metadata_parts.push(format!("📋 {}/{}", done_count, task.checklists.len()));
                }

                // Add time spent if any
                if task.time_spent > 0 {
                    metadata_parts.push(format!("⏱️ {}", format_time(task.time_spent)));
                }

                let metadata = if metadata_parts.is_empty() {
                    String::new()
                } else {
                    format!(" | {}", metadata_parts.join(" | "))
                };

                let cursor = if i == self.selected_task { "▶" } else { " " };

                let mut line_parts = vec![
                    Span::styled(format!("{} {} ", cursor, status), Style::default().fg(Color::Green)),
                    Span::styled(&task.description, style),
                    Span::styled(format!(" ({:?})", task.priority), Style::default().fg(priority_color)),
                ];

                if !metadata.is_empty() {
                    line_parts.push(Span::styled(metadata, Style::default().fg(Color::Gray)));
                }

                ListItem::new(Line::from(line_parts))
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
        let footer = Paragraph::new("↑/↓/j/k: Navigate • PgUp/PgDn: Jump • Home/End: First/Last • Enter: Actions • d: Done • a: Add • n: Note • s: Subtask • t: Tag • p: Project • c: Checklist • /: Filter • Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
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
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(8),  // Basic info
                Constraint::Min(5),     // Details content
                Constraint::Length(3),  // Footer
            ])
            .split(area);

        // Header
        let header = Paragraph::new(format!("📝 Task Details - {}", task.description))
            .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Basic Information
        let priority_color = match task.priority {
            Priority::High => Color::Red,
            Priority::Medium => Color::Yellow,
            Priority::Low => Color::Green,
        };

        let basic_info = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(if task.done { "✅ Completed" } else { "⏳ In Progress" },
                             Style::default().fg(if task.done { Color::Green } else { Color::Yellow })),
            ]),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:?}", task.priority), Style::default().fg(priority_color)),
            ]),
            Line::from(vec![
                Span::styled("Created: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(task.created_at.format("%Y-%m-%d %H:%M").to_string(),
                             Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("Time Spent: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(format_time(task.time_spent), Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::styled("Task ID: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(format!("#{}", task.id), Style::default().fg(Color::Gray)),
            ]),
        ];

        let basic_paragraph = Paragraph::new(Text::from(basic_info))
            .block(Block::default().borders(Borders::ALL).title("📊 Basic Information"))
            .wrap(Wrap { trim: true });
        f.render_widget(basic_paragraph, chunks[1]);

        // Detailed Information
        let mut details_lines = Vec::new();

        // Project
        if let Some(project) = &task.project {
            details_lines.push(Line::from(vec![
                Span::styled("📁 Project: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(project, Style::default().fg(Color::Cyan)),
            ]));
        }

        // Due Date
        if let Some(due_date) = task.due_date {
            details_lines.push(Line::from(vec![
                Span::styled("📅 Due Date: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(due_date.format("%Y-%m-%d %H:%M").to_string(),
                             Style::default().fg(Color::Red)),
            ]));
        }

        // Board
        if let Some(board) = &task.board {
            details_lines.push(Line::from(vec![
                Span::styled("📋 Board: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(board, Style::default().fg(Color::Green)),
            ]));
        }

        // Tags
        if !task.tags.is_empty() {
            details_lines.push(Line::from(vec![
                Span::styled("🏷️ Tags: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(task.tags.join(", "), Style::default().fg(Color::Magenta)),
            ]));
        }

        // Notes
        if let Some(note) = &task.notes {
            details_lines.push(Line::from(""));
            details_lines.push(Line::from(vec![
                Span::styled("📝 Notes:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));
            details_lines.push(Line::from(vec![
                Span::styled(note, Style::default().fg(Color::Gray)),
            ]));
        }

        // Subtasks
        if !task.subtasks.is_empty() {
            let completed_subtasks = task.subtasks.iter().filter(|st| st.done).count();
            let total_subtasks = task.subtasks.len();
            let completion_percentage = if total_subtasks > 0 {
                (completed_subtasks as f64 / total_subtasks as f64 * 100.0) as u32
            } else {
                0
            };

            details_lines.push(Line::from(""));
            details_lines.push(Line::from(vec![
                Span::styled(format!("➕ Subtasks: {} ({:.0}%)", total_subtasks, completion_percentage),
                             Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));

            for (i, subtask) in task.subtasks.iter().enumerate() {
                let status_icon = if subtask.done { "✅" } else { "⏳" };
                let status_color = if subtask.done { Color::Green } else { Color::Yellow };
                details_lines.push(Line::from(vec![
                    Span::styled(format!("  {}. {} ", i + 1, status_icon), Style::default().fg(status_color)),
                    Span::styled(&subtask.description, Style::default().fg(Color::White)),
                ]));
            }
        }

        // Checklist Items
        if !task.checklists.is_empty() {
            let completed_items = task.checklists.iter().filter(|ci| ci.done).count();
            let total_items = task.checklists.len();
            let completion_percentage = if total_items > 0 {
                (completed_items as f64 / total_items as f64 * 100.0) as u32
            } else {
                0
            };

            details_lines.push(Line::from(""));
            details_lines.push(Line::from(vec![
                Span::styled(format!("📋 Checklist: {} ({:.0}%)", total_items, completion_percentage),
                             Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));

            for (i, checklist_item) in task.checklists.iter().enumerate() {
                let status_icon = if checklist_item.done { "☑" } else { "☐" };
                let status_color = if checklist_item.done { Color::Green } else { Color::Yellow };
                details_lines.push(Line::from(vec![
                    Span::styled(format!("  {}. {} ", i + 1, status_icon), Style::default().fg(status_color)),
                    Span::styled(&checklist_item.description, Style::default().fg(Color::White)),
                ]));
            }
        }

        // If no additional details
        if details_lines.is_empty() {
            details_lines.push(Line::from(vec![
                Span::styled("No additional details available", Style::default().fg(Color::Gray)),
            ]));
        }

        let details_paragraph = Paragraph::new(Text::from(details_lines))
            .block(Block::default().borders(Borders::ALL).title("📋 Detailed Information"))
            .wrap(Wrap { trim: true });
        f.render_widget(details_paragraph, chunks[2]);

        // Footer
        let footer = Paragraph::new("d: Mark Done • Esc: Back to List")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[3]);
    }

    fn draw_task_actions(&self, f: &mut Frame, area: Rect, index: usize) {
        let tasks = self.task_manager.get_active_tasks();
        if index >= tasks.len() {
            return;
        }

        let task = &tasks[index];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(5), Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new(format!("⚡ Task Actions - {}", task.description))
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Task info
        let priority_color = match task.priority {
            Priority::High => Color::Red,
            Priority::Medium => Color::Yellow,
            Priority::Low => Color::Green,
        };

        let task_info = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::White)),
                Span::styled(if task.done { "✅ Done" } else { "⏳ In Progress" }, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(Color::White)),
                Span::styled(format!("{:?}", task.priority), Style::default().fg(priority_color)),
            ]),
        ];

        let info_paragraph = Paragraph::new(Text::from(task_info))
            .block(Block::default().borders(Borders::ALL).title("Task Info"))
            .wrap(Wrap { trim: true });
        f.render_widget(info_paragraph, chunks[1]);

        // Actions menu
        let actions = vec![
            "d. ✅ Mark as Done",
            "n. 📝 Add Note",
            "s. ➕ Add Subtask",
            "t. 🏷️  Add Tag",
            "p. 📁 Set Project",
            "c. 📋 Add Checklist Item",
            "r. ⏰ Start/Stop Timer",
            "v. 👁️  View Full Details",
            "Esc. ↩️  Back to List",
        ];

        let actions_list = List::new(
            actions
                .iter()
                .map(|action| ListItem::new(*action))
                .collect::<Vec<_>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("Available Actions"))
        .style(Style::default().fg(Color::White));

        f.render_widget(actions_list, chunks[2]);

        // Footer
        let footer = Paragraph::new("Press key to select action • Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[3]);
    }

    fn draw_add_note(&self, f: &mut Frame, area: Rect, index: usize) {
        let tasks = self.task_manager.get_active_tasks();
        if index >= tasks.len() {
            return;
        }

        let task = &tasks[index];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new(format!("📝 Add Note - {}", task.description))
            .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Input field
        let input = Paragraph::new(format!("Note: {}", self.input_buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Note Content"));
        f.render_widget(input, chunks[1]);

        // Instructions
        let instructions = Paragraph::new("Type your note and press Enter to save")
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

    fn draw_add_subtask(&self, f: &mut Frame, area: Rect, index: usize) {
        let tasks = self.task_manager.get_active_tasks();
        if index >= tasks.len() {
            return;
        }

        let task = &tasks[index];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new(format!("➕ Add Subtask - {}", task.description))
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Input field
        let input = Paragraph::new(format!("Subtask: {}", self.input_buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Subtask Description"));
        f.render_widget(input, chunks[1]);

        // Instructions
        let instructions = Paragraph::new("Type your subtask description and press Enter to add")
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

    fn draw_add_tag(&self, f: &mut Frame, area: Rect, index: usize) {
        let tasks = self.task_manager.get_active_tasks();
        if index >= tasks.len() {
            return;
        }

        let task = &tasks[index];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new(format!("🏷️ Add Tag - {}", task.description))
            .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Input field
        let input = Paragraph::new(format!("Tag: {}", self.input_buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Tag Name"));
        f.render_widget(input, chunks[1]);

        // Instructions
        let instructions = Paragraph::new("Type tag name and press Enter to add")
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

    fn draw_set_project(&self, f: &mut Frame, area: Rect, index: usize) {
        let tasks = self.task_manager.get_active_tasks();
        if index >= tasks.len() {
            return;
        }

        let task = &tasks[index];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new(format!("📁 Set Project - {}", task.description))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Input field
        let input = Paragraph::new(format!("Project: {}", self.input_buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Project Name"));
        f.render_widget(input, chunks[1]);

        // Instructions
        let instructions = Paragraph::new("Type project name and press Enter to set")
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

    fn draw_set_due_date(&self, f: &mut Frame, area: Rect, index: usize) {
        let tasks = self.task_manager.get_active_tasks();
        if index >= tasks.len() {
            return;
        }

        let task = &tasks[index];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new(format!("📅 Set Due Date - {}", task.description))
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Input field
        let input = Paragraph::new(format!("Date (YYYY-MM-DD): {}", self.input_buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Due Date"));
        f.render_widget(input, chunks[1]);

        // Instructions
        let instructions = Paragraph::new("Enter date in YYYY-MM-DD format and press Enter")
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

    fn draw_add_checklist_item(&self, f: &mut Frame, area: Rect, index: usize) {
        let tasks = self.task_manager.get_active_tasks();
        if index >= tasks.len() {
            return;
        }

        let task = &tasks[index];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new(format!("📋 Add Checklist Item - {}", task.description))
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Input field
        let input = Paragraph::new(format!("Item: {}", self.input_buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Checklist Item"));
        f.render_widget(input, chunks[1]);

        // Instructions
        let instructions = Paragraph::new("Type checklist item and press Enter to add")
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

    fn draw_board_view(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new("📋 Board View")
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Board content
        let board_view = self.task_manager.get_board_view();
        let mut board_text = Vec::new();

        for (board_name, tasks) in board_view {
            board_text.push(Line::from(vec![
                Span::styled(format!("📂 {} ({})", board_name, tasks.len()),
                             Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]));

            if tasks.is_empty() {
                board_text.push(Line::from("  (empty)"));
            } else {
                for task in tasks {
                    let status = if task.done { "✅" } else { "⏳" };
                    let priority_color = match task.priority {
                        Priority::High => Color::Red,
                        Priority::Medium => Color::Yellow,
                        Priority::Low => Color::Green,
                    };
                    board_text.push(Line::from(vec![
                        Span::styled(format!("  {} ", status), Style::default().fg(Color::Green)),
                        Span::styled(&task.description, Style::default().fg(Color::White)),
                        Span::styled(format!(" ({:?})", task.priority), Style::default().fg(priority_color)),
                    ]));
                }
            }
            board_text.push(Line::from(""));
        }

        let board_paragraph = Paragraph::new(Text::from(board_text))
            .block(Block::default().borders(Borders::ALL).title("Kanban Boards"))
            .wrap(Wrap { trim: true });
        f.render_widget(board_paragraph, chunks[1]);

        // Footer
        let footer = Paragraph::new("Esc: Back to Main Menu")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    }

    fn draw_calendar_view(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new("📅 Calendar View")
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Calendar content
        let calendar = self.task_manager.get_calendar_view();
        let mut calendar_text = Vec::new();

        for (date, tasks) in &calendar {
            calendar_text.push(Line::from(vec![
                Span::styled(format!("📆 {} ({})", date, tasks.len()),
                             Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]));

            for task in tasks {
                let status = if task.done { "✅" } else { "⏳" };
                let priority_color = match task.priority {
                    Priority::High => Color::Red,
                    Priority::Medium => Color::Yellow,
                    Priority::Low => Color::Green,
                };
                calendar_text.push(Line::from(vec![
                    Span::styled(format!("  {} ", status), Style::default().fg(Color::Green)),
                    Span::styled(&task.description, Style::default().fg(Color::White)),
                    Span::styled(format!(" ({:?})", task.priority), Style::default().fg(priority_color)),
                ]));
            }
            calendar_text.push(Line::from(""));
        }

        if calendar.is_empty() {
            calendar_text.push(Line::from("No tasks with due dates found"));
        }

        let calendar_paragraph = Paragraph::new(Text::from(calendar_text))
            .block(Block::default().borders(Borders::ALL).title("Tasks by Due Date"))
            .wrap(Wrap { trim: true });
        f.render_widget(calendar_paragraph, chunks[1]);

        // Footer
        let footer = Paragraph::new("Esc: Back to Main Menu")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    }

    fn draw_timeline_view(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new("⏰ Timeline View")
            .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Timeline content
        let timeline = self.task_manager.get_timeline_view();
        let mut timeline_text = Vec::new();

        for task in &timeline {
            let status = if task.done { "✅" } else { "⏳" };
            let priority_color = match task.priority {
                Priority::High => Color::Red,
                Priority::Medium => Color::Yellow,
                Priority::Low => Color::Green,
            };
            let time_spent = if task.time_spent > 0 {
                format!(" ({})", format_time(task.time_spent))
            } else {
                String::new()
            };

            timeline_text.push(Line::from(vec![
                Span::styled(format!("{} ", task.created_at.format("%Y-%m-%d %H:%M")),
                             Style::default().fg(Color::Gray)),
                Span::styled(format!("{} ", status), Style::default().fg(Color::Green)),
                Span::styled(&task.description, Style::default().fg(Color::White)),
                Span::styled(format!(" ({:?})", task.priority), Style::default().fg(priority_color)),
                Span::styled(time_spent, Style::default().fg(Color::Yellow)),
            ]));
        }

        if timeline.is_empty() {
            timeline_text.push(Line::from("No tasks found"));
        }

        let timeline_paragraph = Paragraph::new(Text::from(timeline_text))
            .block(Block::default().borders(Borders::ALL).title("Tasks by Creation Date"))
            .wrap(Wrap { trim: true });
        f.render_widget(timeline_paragraph, chunks[1]);

        // Footer
        let footer = Paragraph::new("Esc: Back to Main Menu")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    }

    fn draw_focus_view(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new("🎯 Focus View")
            .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Input field
        let input = Paragraph::new(format!("Focus on: {}", self.input_buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Tag/Project/Board"));
        f.render_widget(input, chunks[1]);

        // Focus content
        let focus_tasks = if !self.input_buffer.is_empty() {
            self.task_manager.get_focus_view(&self.input_buffer)
        } else {
            Vec::new()
        };

        let mut focus_text = Vec::new();
        if !self.input_buffer.is_empty() {
            focus_text.push(Line::from(vec![
                Span::styled(format!("Found {} tasks", focus_tasks.len()),
                             Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]));
            focus_text.push(Line::from(""));

            for task in &focus_tasks {
                let status = if task.done { "✅" } else { "⏳" };
                let priority_color = match task.priority {
                    Priority::High => Color::Red,
                    Priority::Medium => Color::Yellow,
                    Priority::Low => Color::Green,
                };
                focus_text.push(Line::from(vec![
                    Span::styled(format!("{} ", status), Style::default().fg(Color::Green)),
                    Span::styled(&task.description, Style::default().fg(Color::White)),
                    Span::styled(format!(" ({:?})", task.priority), Style::default().fg(priority_color)),
                ]));
            }
        } else {
            focus_text.push(Line::from("Enter a tag, project, or board name to focus on"));
        }

        let focus_paragraph = Paragraph::new(Text::from(focus_text))
            .block(Block::default().borders(Borders::ALL).title("Focused Tasks"))
            .wrap(Wrap { trim: true });
        f.render_widget(focus_paragraph, chunks[2]);

        // Footer
        let footer = Paragraph::new("Enter: Search • Esc: Back to Main Menu")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[3]);
    }

    fn draw_export_view(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(area);

        // Header
        let header = Paragraph::new("💾 Export Data")
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Export options
        let export_options = vec![
            "1. 📄 Export as CSV",
            "2. 📋 Export as JSON",
        ];

        let export_list = List::new(
            export_options
                .iter()
                .enumerate()
                .map(|(i, option)| {
                    let style = if i == self.selected_export {
                        Style::default().fg(Color::Black).bg(Color::White)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(*option, style),
                    ]))
                })
                .collect::<Vec<_>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("Export Format"))
        .highlight_style(Style::default().bg(Color::Blue));

        f.render_widget(export_list, chunks[1]);

        // Footer
        let footer = Paragraph::new("Use number keys to select format • Esc: Back")
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
            Line::from("  • Number keys: Select menu options"),
            Line::from("  • Arrow keys/j/k: Navigate lists"),
            Line::from("  • Enter: Select/confirm"),
            Line::from("  • Esc: Go back/cancel"),
            Line::from(""),
            Line::from("⚡ Task List Actions:"),
            Line::from("  • Enter: Open task actions menu"),
            Line::from("  • d: Mark task as done"),
            Line::from("  • a: Add new task"),
            Line::from("  • n: Add note to selected task"),
            Line::from("  • s: Add subtask to selected task"),
            Line::from("  • t: Add tag to selected task"),
            Line::from("  • p: Set project for selected task"),
            Line::from("  • c: Add checklist item"),
            Line::from("  • /: Filter/search tasks"),
            Line::from(""),
            Line::from("🎨 Interface:"),
            Line::from("  • 🔴 High priority (red)"),
            Line::from("  • 🟡 Medium priority (yellow)"),
            Line::from("  • 🟢 Low priority (green)"),
            Line::from("  • ✅ Done tasks"),
            Line::from("  • ⏳ In progress tasks"),
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