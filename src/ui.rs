use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn render(
    frame: &mut Frame,
    user_input: &str,
    todo: &[String],
    doing: &[String],
    done: &[String],
    selected_column: &usize,
    selected_index: &usize,
) {
    let todo_items: Vec<ListItem> = todo.iter().map(|s| ListItem::new(s.as_str())).collect();
    let doing_items: Vec<ListItem> = doing.iter().map(|s| ListItem::new(s.as_str())).collect();
    let done_items: Vec<ListItem> = done.iter().map(|s| ListItem::new(s.as_str())).collect();
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
        Paragraph::new(Line::from(Span::styled(
            " todosh ",
            Style::new()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        main_container[0],
    );

    // Input Box (Bottom) - Using the variable passed into the function
    frame.render_widget(
        Paragraph::new(format!(" enter task: {}", user_input))
            .alignment(Alignment::Left)
            .block(Block::new().borders(Borders::ALL)),
        main_container[2],
    );

    // 1. TODO Border (Index 0)
    let todo_color = if *selected_column == 0 {
        Color::Green
    } else {
        Color::White
    };
    let todo_block = Block::default()
        .title(" todo ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(todo_color));

    // 2. DOING Border (Index 1)
    let doing_color = if *selected_column == 1 {
        Color::Green
    } else {
        Color::White
    };
    let doing_block = Block::default()
        .title(" doing ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(doing_color));

    // 3. DONE Border (Index 2)
    let done_color = if *selected_column == 2 {
        Color::Green
    } else {
        Color::White
    };
    let done_block = Block::default()
        .title(" done ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(done_color));

    
    let todo_list = List::new(todo_items).block(todo_block);
    let doing_list = List::new(doing_items).block(doing_block);
    let done_list = List::new(done_items).block(done_block);

    frame.render_widget(todo_list, task_containers[0]);
    frame.render_widget(doing_list, task_containers[1]);
    frame.render_widget(done_list, task_containers[2]);
}
