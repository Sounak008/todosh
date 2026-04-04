use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut user_input = String::new();

    loop {
        terminal.draw(|frame| {
            
            let main_container = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(frame.area());
            let task_containers = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ])
                .split(main_container[1]);
            frame.render_widget(
                Paragraph::new(Line::from(
                    Span::styled(" todosh ", Style::new()
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                    )
                )).alignment(Alignment::Center).block(Block::new()),
                main_container[0],
            );
            frame.render_widget(
                Paragraph::new(format!(" enter task: {}", user_input)).alignment(Alignment::Left).block(Block::new().borders(Borders::ALL)),
                main_container[2],
            );
            frame.render_widget(
                Paragraph::new("todo").alignment(Alignment::Center).block(Block::new().borders(Borders::ALL)),
                task_containers[0],
            );
            frame.render_widget(
                Paragraph::new("doing").alignment(Alignment::Center).block(Block::new().borders(Borders::ALL)),
                task_containers[1],
            );
            frame.render_widget(
                Paragraph::new("done").alignment(Alignment::Center).block(Block::new().borders(Borders::ALL)),
                task_containers[2],
            );
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    break;
                }
                KeyCode::Enter => {
                    if !user_input.is_empty() {
                        user_input.clear();
                    }
                }
                KeyCode::Backspace => {
                    user_input.pop();
                }
                KeyCode::Char(c) => {
                    user_input.push(c);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
