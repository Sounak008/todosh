mod ui;
mod keybinds;
mod storage;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::storage::{save_tasks, load_tasks};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut user_input = String::new();

    let data = load_tasks();
    let mut todo = data.todo;
    let mut doing = data.doing;
    let mut done = data.done;
    let mut selected_column = 0;
    let mut selected_index = 0;

    loop {
        terminal.draw(|frame| {
            ui::render(frame, &user_input, &todo, &doing, &done, &selected_column, &selected_index);
        })?;

        if let Event::Key(key) = event::read()? {
            keybinds::handle_keybinds(key, &mut user_input, &mut todo, &mut doing, &mut done, &mut selected_column, &mut selected_index);
        }

        save_tasks(&todo, &doing, &done);
    }
}
