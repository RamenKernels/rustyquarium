use std::{error::Error, fmt, io};

use crossterm::style::Color;

use crate::fish::{Fish, Fishes};

pub mod fish;
pub mod fish_handler;

pub fn render(fishes: &mut Vec<Fishes>) -> io::Result<()> {
    for fish in fishes {
        fish.render()?;
    }

    Ok(())
}

pub fn update(fishes: &mut Vec<Fishes>) -> io::Result<()> {
    fishes.retain_mut(|fish| fish.update().is_ok());

    Ok(())
}

pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct FishOutOfBounds;

impl fmt::Display for FishOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fish moved out of bounds")
    }
}

impl Error for FishOutOfBounds {}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

pub struct Sprite {
    characters: &'static str,
    color: Color,
}

impl Sprite {
    fn new(characters: &'static str, color: Color) -> Self {
        Self { characters, color }
    }
}
