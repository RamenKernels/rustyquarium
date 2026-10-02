use std::error::Error;
use std::fmt;

use crossterm::terminal;

pub trait Fish {
    fn update(&mut self) -> Result<(), FishOutOfBounds>;
    fn pos(&self) -> &Point<u16>;
    fn sprite(&self) -> &String;
}

pub enum Fishes {
    FihFish,
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

pub struct FihFish {
    pub pos: Point<u16>,
    pub sprite: String,
    pub speed: u16,
    pub direction: Direction,
    move_frames: u16,
}

impl FihFish {
    pub fn new(pos: Point<u16>, speed: u16, direction: Direction) -> Self {
        let sprite = match direction {
            Direction::Left => String::from("hif"),
            Direction::Right => String::from("fih"),
        };

        FihFish {
            pos,
            sprite,
            speed,
            direction,
            move_frames: 0,
        }
    }
}

impl Fish for FihFish {
    fn update(&mut self) -> Result<(), FishOutOfBounds> {
        if self.move_frames % self.speed == 0 {
            match self.direction {
                Direction::Left => {
                    if self.pos.x == 0 {
                        return Err(FishOutOfBounds);
                    }

                    self.pos.x -= 1;
                }
                Direction::Right => {
                    self.pos.x += 1;

                    if self.pos.x
                        >= terminal::size()
                            .expect("Should have been able to read terminal size")
                            .0
                    {
                        return Err(FishOutOfBounds);
                    }
                }
            }
        }

        self.move_frames += 1;

        Ok(())
    }

    fn pos(&self) -> &Point<u16> {
        &self.pos
    }

    fn sprite(&self) -> &String {
        &self.sprite
    }
}
