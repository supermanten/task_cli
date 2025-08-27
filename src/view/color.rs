use colored::*;

/// Color management utilities
pub struct ColorScheme;

impl ColorScheme {
    pub fn success() -> Color {
        Color::Green
    }

    pub fn error() -> Color {
        Color::Red
    }

    pub fn warning() -> Color {
        Color::Yellow
    }

    pub fn info() -> Color {
        Color::Blue
    }

    pub fn accent() -> Color {
        Color::Cyan
    }

    pub fn muted() -> Color {
        Color::White
    }

    pub fn priority_high() -> Color {
        Color::Red
    }

    pub fn priority_medium() -> Color {
        Color::Yellow
    }

    pub fn priority_low() -> Color {
        Color::Green
    }

    pub fn status_done() -> Color {
        Color::Green
    }

    pub fn status_todo() -> Color {
        Color::Yellow
    }

    pub fn board_name() -> Color {
        Color::Magenta
    }

    pub fn tag() -> Color {
        Color::Cyan
    }

    pub fn note() -> Color {
        Color::White
    }

    pub fn subtask() -> Color {
        Color::White
    }
}

/// Apply colors to text based on content
pub fn colorize_text(text: &str, color: Color) -> colored::ColoredString {
    text.color(color)
}

/// Colorize based on priority
pub fn colorize_priority(priority: &str) -> colored::ColoredString {
    match priority.to_lowercase().as_str() {
        "high" => priority.color(ColorScheme::priority_high()).bold(),
        "medium" => priority.color(ColorScheme::priority_medium()),
        "low" => priority.color(ColorScheme::priority_low()),
        _ => priority.normal(),
    }
}

/// Colorize based on status
pub fn colorize_status(status: &str) -> colored::ColoredString {
    match status {
        "✓" | "Done" => status.color(ColorScheme::status_done()),
        "✗" | "Todo" => status.color(ColorScheme::status_todo()),
        _ => status.normal(),
    }
}

/// Colorize board names
pub fn colorize_board(board: &str) -> colored::ColoredString {
    board.color(ColorScheme::board_name())
}

/// Colorize tags
pub fn colorize_tags(tags: &str) -> colored::ColoredString {
    tags.color(ColorScheme::tag())
}

/// Colorize notes and secondary text
pub fn colorize_note(text: &str) -> colored::ColoredString {
    text.color(ColorScheme::note()).dimmed()
}

/// Colorize subtasks
pub fn colorize_subtask(text: &str) -> colored::ColoredString {
    text.color(ColorScheme::subtask()).dimmed()
}