use aes::Aes128;

use std::{thread, time::Duration};
use tui::{
    widgets::{Block, Borders},
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
