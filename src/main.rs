mod ui;
mod keybinds;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut user_input = String::new();

    let mut todo= Vec::<String>::new();
    let mut doing = Vec::<String>::new();
    let mut done = Vec::<String>::new();
    let mut selected_column = 0;
    let mut selected_index = 0;

    loop {
        terminal.draw(|frame| {
            ui::render(frame, &user_input, &todo, &doing, &done, &selected_column, &selected_index);
        })?;

        if let Event::Key(key) = event::read()? {
            keybinds::handle_keybinds(key, &mut user_input, &mut todo, &mut selected_column, &mut selected_index);
        }
    }
}
