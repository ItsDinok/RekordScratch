use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    text::{Line, Span},
};
use ratatui::layout::Alignment;
use crate::App;

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.size();

    // Layout: add a box below progress bar for the text lines
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(8),     // Middle: status boxes
            Constraint::Length(3),  // Progress bar
            Constraint::Length(3),  // Box for progress % + files xx/yy
            Constraint::Length(3),  // Current file
            Constraint::Length(3),  // Controls
        ])
        .split(size);

    // Title
    let title = Paragraph::new("RekordScratch v1.0")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Middle area: split horizontally
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(chunks[1]);

    // Left: Status indicators
    let bool_statuses = [
        ("Drive detected", app.drive_detected),
        ("Desktop detected", app.desktop_detected),
        ("Playlist detected", app.playlist_detected),
        ("Track map created", app.track_map_created),
    ];

    let bool_lines: Vec<Line> = bool_statuses.iter().map(|(label, state)| {
        let color = if *state { Color::Green } else { Color::Red };
        let dot = Span::styled("● ", Style::default().fg(color));
        Line::from(vec![dot, Span::raw(*label)])
    }).collect();

    let bool_paragraph = Paragraph::new(bool_lines)
        .block(Block::default().title("App Statuses").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(bool_paragraph, middle_chunks[0]);

    // Right: Status + Errors
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(middle_chunks[1]);

    let status_paragraph = Paragraph::new(app.status_message.clone())
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(status_paragraph, right_chunks[0]);

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
    f.render_widget(error_paragraph, right_chunks[1]);

    // Progress bar (without default percentage label)
    let gauge = Gauge::default()
        .block(Block::default().title("Progress").borders(Borders::ALL))
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(app.progress as f64);
    f.render_widget(gauge, chunks[2]);

    // Box below progress bar for percentage and files processed
    let progress_text = format!("Progress: {:.0}%", app.progress * 100.0);
    let files_text = format!("Files processed: {}/{}", app.files_cleared, app.files_total);

    // Compose multiline paragraph with the two lines stacked vertically
    let progress_info = Paragraph::new(vec![
        Line::from(progress_text),
        Line::from(files_text),
    ])
    .block(Block::default().borders(Borders::ALL).title("Progress Info"))
    .alignment(Alignment::Left);

    f.render_widget(progress_info, chunks[3]);

    // Current file display
    let current_file_text = app.current_file.clone().unwrap_or_else(|| "None".into());
    let current_file_paragraph = Paragraph::new(current_file_text)
        .block(Block::default().title("Current File").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(current_file_paragraph, chunks[4]);

    // Controls hint bar
    let controls_line = Line::from(vec![
        Span::styled("[Q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Exit   "),
        Span::styled("[S]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Scan Drives   "),
        Span::styled("[R]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Run RekordScratch   "),
        Span::styled("[P]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Set Playlists Path"),
    ]);
    let controls_paragraph = Paragraph::new(controls_line)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .wrap(Wrap { trim: true });
    f.render_widget(controls_paragraph, chunks[5]);
}
