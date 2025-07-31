use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Gauge, Wrap},
    text::Span,
};
use crate::App;

pub fn ui(f: &mut Frame<'_>, app: &App) {
    let size = f.size();

    // Vertical layout: Title, drive+status, error, progress bar, current file
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(5),  // Drive & Status
            Constraint::Length(3),  // Error message
            Constraint::Length(3),  // Progress bar
            Constraint::Length(3),  // Current file
            Constraint::Min(0),     // filler
        ].as_ref())
        .split(size);

    // Title
    let title = Paragraph::new("RekordScratch v1.0")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Drive status + status message horizontal split
    let drive_status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[1]);

    // Drive status block
    let drive_color = if app.drive_detected { Color::Green } else { Color::Red };
    let drive_light = Span::styled("● ", Style::default().fg(drive_color));
    let drive_letter = app.drive_letter.clone().unwrap_or_else(|| "N/A".into());
    let drive_text = vec![
        ratatui::text::Line::from(vec![drive_light, Span::raw(format!("Drive detected: {}", drive_letter))]),
    ];
    let drive_paragraph = Paragraph::new(drive_text)
        .block(Block::default().title("Drive Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(drive_paragraph, drive_status_chunks[0]);

    // Status message block
    let status_paragraph = Paragraph::new(app.status_message.clone())
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(status_paragraph, drive_status_chunks[1]);

    // Error message box (red if error)
    let error_style = if app.error_message.is_some() {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let error_text = app.error_message.clone().unwrap_or_else(|| "No errors.".into());
    let error_paragraph = Paragraph::new(error_text)
        .style(error_style)
        .block(Block::default().title("Errors").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(error_paragraph, chunks[2]);

    // Progress bar
    let gauge = Gauge::default()
        .block(Block::default().title("Progress").borders(Borders::ALL))
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .percent((app.progress * 100.0) as u16);
    f.render_widget(gauge, chunks[3]);

    // Current file being processed
    let current_file_text = app.current_file.clone().unwrap_or_else(|| "None".into());
    let current_file_paragraph = Paragraph::new(current_file_text)
        .block(Block::default().title("Current File").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(current_file_paragraph, chunks[4]);
}
