use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::disable_raw_mode;

pub fn handle_keybinds(
    key: KeyEvent,
    user_input: &mut String,
    todo_list: &mut Vec<String>,
    doing_list: &mut Vec<String>,
    done_list: &mut Vec<String>,
    selected_column: &mut usize,
    selected_index: &mut usize,

) {
    let current_column_list = match *selected_column {
        0 => todo_list.len(),
        1 => doing_list.len(),
        2 => done_list.len(),
        _ => 0,
    };
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Char('q') => {
                let _ = disable_raw_mode();
                let _ = crossterm::execute!(
                    std::io::stdout(),
                    crossterm::terminal::LeaveAlternateScreen
                );
                std::process::exit(0);
            }
            KeyCode::Enter => {
                if !user_input.is_empty() {
                    let task: String = user_input.drain(..).collect();
                    todo_list.push(task);
                }
            }
            KeyCode::Backspace => {
                if !user_input.is_empty() {
                    user_input.pop();
                }
            }
            KeyCode::Char(c) => {
                    user_input.push(c);
                
            }
            KeyCode::Left => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    match *selected_column {
                        2 => {
                            if !done_list.is_empty() {
                                let task = done_list.remove(*selected_index);
                                doing_list.push(task);
                            }
                        }
                        1 => {
                            if !doing_list.is_empty() {
                                let task = doing_list.remove(*selected_index);
                                todo_list.push(task);
                            }
                        }
                        _ => {}
                    }
                } else if *selected_column > 0 {
                    *selected_index = 0;
                    *selected_column -= 1;
                }
            }
            KeyCode::Right => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    match *selected_column {
                        0 => {
                            if !todo_list.is_empty() {
                                let task = todo_list.remove(*selected_index);
                                doing_list.push(task);
                            }
                        }
                        1 => {
                            if !doing_list.is_empty() {
                                let task = doing_list.remove(*selected_index);
                                done_list.push(task);
                            }
                        }
                        _ => {}
                    }
                } else if *selected_column < 2 {
                    *selected_index = 0;
                    *selected_column += 1;
                }
            }
            KeyCode::Up => {
                if *selected_index > 0 {
                    *selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if current_column_list > 0 && *selected_index < current_column_list - 1 {
                    *selected_index += 1;
                }
            }
            _ => {}
        }
    }
}