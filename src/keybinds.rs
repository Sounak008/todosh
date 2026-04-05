use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
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
    let task: String = user_input.drain(..).collect();
    let corrent_column_list = match *selected_column {
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
                    todo_list.push(task);
                    user_input.clear();
                }
            }
            KeyCode::Backspace => {
                if !user_input.is_empty() {
                    user_input.pop();
                }
            }
            KeyCode::Char(c) => {
                if !user_input.is_empty() {
                    user_input.push(c);
                }
                
            }
            KeyCode::Left => {
                if *selected_column < 0 {
                    *selected_index = 0;
                    *selected_column -= 1;
                }
            }
            KeyCode::Right => {
                if *selected_column > 2 {
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
                if *selected_index < corrent_column_list - 1 {
                    *selected_index += 1;
                }
            }
            _ => {}
        }
    }
}