use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(
    frame: &mut Frame,
    user_input: &str, 
    todo: &[String], 
    doing: &[String], 
    done: &[String],
    selected_column: &usize,
    selected_index: &usize
){
    // Vertical Layout
    let main_container = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Horizontal Layout inside the middle section
    let task_containers = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(main_container[1]);

    // Title (Top)
    frame.render_widget(
        Paragraph::new(Line::from(
            Span::styled(" todosh ", Style::new()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
            )
        )).alignment(Alignment::Center),
        main_container[0],
    );

    // Input Box (Bottom) - Using the variable passed into the function
    frame.render_widget(
        Paragraph::new(format!(" enter task: {}", user_input))
            .alignment(Alignment::Left)
            .block(Block::new().borders(Borders::ALL)),
        main_container[2],
    );

    // The three Columns (Middle)
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

    

}