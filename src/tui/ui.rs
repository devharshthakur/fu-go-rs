use crate::app::App;
use crate::app::AppState;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

// --- Constants for colors and styles ---
const LOGO_GRADIENT: [Color; 7] = [
    Color::Rgb(255, 83, 112),  // Red
    Color::Rgb(247, 140, 108), // Orange
    Color::Rgb(255, 203, 107), // Yellow
    Color::Rgb(195, 232, 141), // Green
    Color::Rgb(137, 221, 255), // Cyan
    Color::Rgb(130, 170, 255), // Blue
    Color::Rgb(199, 146, 234), // Purple
];
const SPINNER_CHARS: [&str; 8] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];
const COLOR_SUCCESS: Color = Color::Rgb(195, 232, 141);
const COLOR_WARNING: Color = Color::Rgb(255, 83, 112);
const COLOR_INFO: Color = Color::Gray;
const COLOR_HIGHLIGHT: Color = Color::Rgb(130, 170, 255);

pub fn render(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(frame.size());
    render_logo(frame, main_layout[0]);
    render_subtitle(frame, main_layout[1]);

    // Render content based on the current app state.
    let content_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)])
        .split(main_layout[2])[0];

    match app.state {
        AppState::Loading => render_loading(frame, content_area, app),
        AppState::Confirm => render_confirm(frame, content_area, app),
        AppState::Deleting => render_deleting(frame, content_area, app),
        AppState::Complete => render_complete(frame, content_area, app),
    }
}

fn render_logo(frame: &mut Frame, area: Rect) {
    let logo_text = "
███████╗██╗   ██╗      ██████╗  ██████╗ 
██╔════╝██║   ██║     ██╔════╝ ██╔═══██╗
█████╗  ██║   ██║     ██║  ███╗██║   ██║
██╔══╝  ██║   ██║     ██║   ██║██║   ██║
██║     ╚██████╔╝     ╚██████╔╝╚██████╔╝
╚═╝      ╚═════╝       ╚═════╝  ╚═════╝ 
"; // The big FUGO text
    let mut spans = Vec::new();
    for line in logo_text.lines() {
        let mut line_spans = Vec::new();
        for (i, ch) in line.chars().enumerate() {
            let color = LOGO_GRADIENT[i % LOGO_GRADIENT.len()];
            line_spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        }
        spans.push(Line::from(line_spans));
    }

    let logo = Paragraph::new(spans)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(125, 86, 244))),
        )
        .alignment(Alignment::Center);

    frame.render_widget(logo, area);
}

fn render_subtitle(frame: &mut Frame, area: Rect) {
    let subtitle = Paragraph::new("A Go uninstaller for your system.")
        .alignment(Alignment::Center)
        .style(Style::default().fg(COLOR_INFO));
    frame.render_widget(subtitle, area);
}

fn render_loading(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!(
        "{} Searching for Go installations...",
        SPINNER_CHARS[app.spinner_frame]
    );
    let p = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(p, area);
}

fn render_confirm(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines = Vec::new();

    if app.go_versions.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "No Go installations found",
            Style::default().fg(COLOR_INFO),
        )]));
        lines.push(Line::from(vec![
            Span::styled("Press ", Style::default().fg(COLOR_INFO)),
            Span::styled(
                "q/ESC",
                Style::default().bg(Color::DarkGray).fg(Color::White).bold(),
            ),
            Span::styled(" to exit", Style::default().fg(COLOR_INFO)),
        ]));
    } else {
        lines.push(Line::from(vec![Span::styled(
            "Found Go installations:",
            Style::default().fg(COLOR_HIGHLIGHT).bold(),
        )]));
        lines.push(Line::default());
        for version in &app.go_versions {
            lines.push(Line::from(vec![
                Span::styled("  • ", Style::default().fg(COLOR_HIGHLIGHT)),
                Span::styled(version, Style::default().fg(Color::White)),
            ]));
        }
        lines.push(Line::default());
        lines.push(Line::from(vec![
            Span::styled("WARNING: ", Style::default().fg(COLOR_WARNING).bold()),
            Span::styled(
                "This will delete ALL Go installations!",
                Style::default().fg(COLOR_WARNING),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Path: ", Style::default().fg(COLOR_INFO)),
            Span::styled(
                app.go_install_path.to_string_lossy(),
                Style::default().fg(Color::White),
            ),
        ]));
        lines.push(Line::default());
        lines.push(Line::from(vec![
            Span::styled("Type ", Style::default().fg(COLOR_INFO)),
            Span::styled(
                "yes",
                Style::default().bg(COLOR_WARNING).fg(Color::White).bold(),
            ),
            Span::styled(" to confirm deletion: ", Style::default().fg(COLOR_INFO)),
            Span::styled(&app.input, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::default());
        lines.push(Line::from(vec![
            Span::styled(
                "ENTER ",
                Style::default().bg(COLOR_WARNING).fg(Color::White).bold(),
            ),
            Span::styled(" to continue, ", Style::default().fg(COLOR_INFO)),
            Span::styled(
                "q/ESC ",
                Style::default().bg(Color::DarkGray).fg(Color::White).bold(),
            ),
            Span::styled(" to exit", Style::default().fg(COLOR_INFO)),
        ]));
    }

    let p = Paragraph::new(lines).wrap(Wrap { trim: true });
    frame.render_widget(p, area);
}

fn render_deleting(frame: &mut Frame, area: Rect, _app: &App) {
    let p = Paragraph::new("Deleting Go installations...")
        .alignment(Alignment::Center)
        .style(Style::default().fg(COLOR_WARNING));
    frame.render_widget(p, area);
}

fn render_complete(frame: &mut Frame, area: Rect, _app: &App) {
    let p = Paragraph::new("All Go installations have been deleted. Press q/ESC to exit.")
        .alignment(Alignment::Center)
        .style(Style::default().fg(COLOR_SUCCESS));
    frame.render_widget(p, area);
}
