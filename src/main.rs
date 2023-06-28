use aes::Aes128;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

use anyhow::Result;

use namecrypt::{restore_terminal, setup_terminal};

struct App {
    cipher: Aes128,
    found_file_names: Vec<String>,
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    restore_terminal(terminal)?;

    Ok(())
}
