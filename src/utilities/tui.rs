use std::io::{self, stdout, Stdout}; 
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::prelude::*;

//Type alias for the terminal used
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

// Initialize the terminal

pub fn init() -> io::Result<Tui>{
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

// Restore the terminal

pub fn restore() -> io::Result<()>{
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode();
    Ok(())
}
