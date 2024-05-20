use std::io::{stdout, Result};

use crossterm::{
    event::{self, Event::Key, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{
        block::{self, Title},
        Block, Borders, Clear, List, Paragraph,
    },
    Frame, Terminal,
};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    ui(&mut terminal)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn ui<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    loop {
        terminal.draw(draw_ui)?;

        if let Ok(evt) = event::read() {
            if let Key(key) = evt {
                if let KeyCode::Esc = key.code {
                    return Ok(());
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}

fn draw_ui(f: &mut Frame) {
    let split_layout = Layout::default()
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    let requests = Block::default()
        .title(block::Title::from("[Capturing...]").alignment(Alignment::Left))
        .title(block::Title::from(Line::from("HTTP Requests".green())).alignment(Alignment::Center))
        .title(block::Title::from("[12/1200]").alignment(Alignment::Right))
        .borders(Borders::ALL);

    let request_info = Block::default()
        .title(block::Title::from("Request Info").alignment(Alignment::Center))
        .borders(Borders::ALL);

    f.render_widget(requests, split_layout[0]);
    f.render_widget(request_info, split_layout[1]);

    // Capture and filter setup

    let (width, height) = (70, 30);
    let vertical_margin = (f.size().height - height) / 2;
    let horizontal_margin = (f.size().width - width) / 2;
    let dialog_rect = Rect::new(horizontal_margin, vertical_margin, width, height);

    let dialog_window = Block::default()
        .title(Title::from("Capture filters").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .bg(Color::DarkGray);

    let dialog_window_inner = dialog_window.inner(dialog_rect);

    let dialog_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(2), Constraint::Percentage(100)])
        .split(dialog_window_inner);

    let dialog_usage = Paragraph::new("Select device and enter");

    let dialog_list = List::new(vec!["first", "second"]).bg(Color::Black);

    f.render_widget(Clear, dialog_rect);
    f.render_widget(dialog_window, dialog_rect);
    f.render_widget(dialog_usage, dialog_layout[0]);
    f.render_widget(dialog_list, dialog_layout[1]);
}
