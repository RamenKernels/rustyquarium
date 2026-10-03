use std::io;

use crate::fish::Fish;

pub mod fish;
pub mod fish_handler;

pub fn render(fishes: &mut Vec<Box<dyn Fish>>) -> io::Result<()> {
    for fish in fishes {
        fish.render()?;
    }

    Ok(())
}

pub fn update(fishes: &mut Vec<Box<dyn Fish>>) -> io::Result<()> {
    fishes.retain_mut(|fish| fish.update().is_ok());

    Ok(())
}
