use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    text::{Line, Span},
};
use ratatui::layout::Alignment;
use ratatui::prelude::Rect;
use crate::App;

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.size();

    // Layout with no extra box below progress, so just 5 chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(8),     // Middle: status boxes
            Constraint::Length(6),  // Progress bar + text (combined)
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

    // Inside the progress chunk, split vertically into two parts:
    // 1) Gauge (progress bar) top 3 lines
    // 2) Progress info text bottom 3 lines
    let _progress_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(chunks[2]);

    // Outer block with borders around both progress bar and text
    let progress_block = Block::default().title("Progress").borders(Borders::ALL);
    f.render_widget(progress_block.clone(), chunks[2]);

    // Render progress bar gauge inside the top half (minus borders)
    // Note: To avoid double borders, render gauge inside inner area (chunks[2] shrunk by borders)
    let inner = progress_block.inner(chunks[2]);
    let gauge_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: 3,
    };

    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(app.progress as f64);
    f.render_widget(gauge, gauge_area);

    // Render the progress info text in the bottom half of the box
    let info_area = Rect {
        x: inner.x,
        y: inner.y + 3,
        width: inner.width,
        height: 3,
    };

    let progress_text = format!("Progress: {:.0}%", app.progress * 100.0);
    let files_text = format!("Files processed: {}/{}", app.files_cleared, app.files_total);

    let progress_info = Paragraph::new(vec![
        Line::from(progress_text),
        Line::from(files_text),
    ])
    .alignment(Alignment::Left);

    f.render_widget(progress_info, info_area);

    // Current file display
    let current_file_text = app.current_file.clone().unwrap_or_else(|| "None".into());
    let current_file_paragraph = Paragraph::new(current_file_text)
        .block(Block::default().title("Current File").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(current_file_paragraph, chunks[3]);

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
    f.render_widget(controls_paragraph, chunks[4]);
}
