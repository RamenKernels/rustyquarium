use std::io;

use crate::fish::Fish;

use crossterm::{cursor, execute, style, terminal};

pub mod fish;
pub mod fish_handler;

pub fn render(fishes: &mut Vec<Box<dyn Fish>>) -> io::Result<()> {
    for fish in fishes {
        execute!(
            io::stdout(),
            cursor::MoveTo(fish.pos().x, fish.pos().y),
            style::Print(fish.sprite())
        )?;
    }

    Ok(())
}

pub fn update(fishes: &mut Vec<Box<dyn Fish>>) -> io::Result<()> {
    let terminal_width = terminal::size()
        .expect("Should have been able to read terminal size")
        .0;

    fishes.retain_mut(|fish| fish.update().is_ok());

    Ok(())
}
