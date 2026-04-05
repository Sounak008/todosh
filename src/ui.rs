use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

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

    // Input Box (Bottom)
    frame.render_widget(
        Paragraph::new(format!(" enter task: {}", user_input))
            .alignment(Alignment::Left)
            .block(Block::new().borders(Borders::ALL)),
        main_container[2],
    );

    // Cursor
    let cursor_x = main_container[2].x + user_input.len() as u16 + 14;
    let cursor_y = main_container[2].y + 1;

    frame.set_cursor_position((cursor_x, cursor_y));

    // Styling
    let (todo_color, _) = if *selected_column == 0 {
        (Color::Green, Color::Indexed(236))
    } else {
        (Color::Indexed(8), Color::Reset)
    };

    let todo_block = Block::default()
        .title(" todo ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(todo_color));

    let (doing_color, _) = if *selected_column == 1 {
        (Color::Green, Color::Indexed(236))
    } else {
        (Color::Indexed(8), Color::Reset)
    };

    let doing_block = Block::default()
        .title(" doing ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(doing_color));

    let (done_color, _) = if *selected_column == 2 {
        (Color::Green, Color::Indexed(236))
    } else {
        (Color::Indexed(8), Color::Reset)
    };

    let done_block = Block::default()
        .title(" done ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(done_color));

    let todo_list = List::new(todo_items)
        .block(todo_block)
        .highlight_symbol(Span::styled(">> ", Style::default().fg(Color::Blue)));
    let doing_list = List::new(doing_items)
        .block(doing_block)
        .highlight_symbol(Span::styled(">> ", Style::default().fg(Color::Blue)));
    let done_list = List::new(done_items)
        .block(done_block)
        .highlight_symbol(Span::styled(">> ", Style::default().fg(Color::Blue)));

    // States
    let mut todo_state = ListState::default();
    let mut doing_state = ListState::default();
    let mut done_state = ListState::default();

    todo_state.select((*selected_column == 0).then_some(*selected_index));
    doing_state.select((*selected_column == 1).then_some(*selected_index));
    done_state.select((*selected_column == 2).then_some(*selected_index));

    frame.render_stateful_widget(todo_list, task_containers[0], &mut todo_state);
    frame.render_stateful_widget(doing_list, task_containers[1], &mut doing_state);
    frame.render_stateful_widget(done_list, task_containers[2], &mut done_state);
}
