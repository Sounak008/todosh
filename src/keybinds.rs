use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::disable_raw_mode;

pub fn handle_keybinds(key: KeyEvent, user_input: &mut String, todo_tasks: &mut Vec<String>) {
    let task: String = user_input.drain(..).collect();
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
                    todo_tasks.push(task);
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
            _ => {}
        }
    }
}