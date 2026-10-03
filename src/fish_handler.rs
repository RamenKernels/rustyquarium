use crate::fish::{self, Direction, Fish, Fishes, Point};
use crossterm::terminal;
use rand::{self, RngExt};

pub fn new_fish(fish: Fishes, fishes: &mut Vec<Box<dyn Fish>>) {
    let mut rng = rand::rng();

    let speed = rng.random_range(1..10);

    let direction = match rng.random_range(1..=2) {
        1 => Direction::Left,
        _ => Direction::Right,
    };

    let spawn = Point::new(
        match direction {
            Direction::Left => {
                terminal::size()
                    .expect("Should have been able to read terminal size")
                    .0
            }
            Direction::Right => 0,
        },
        rng.random_range(
            1..terminal::size()
                .expect("Should have been able to read terminal size")
                .1,
        ),
    );

    match fish {
        Fishes::FihFish => fishes.push(Box::new(fish::FihFish::new(spawn, speed, direction))),
        Fishes::BigFish => fishes.push(Box::new(fish::BigFish::new(spawn, speed, direction))),
        _ => fishes.push(Box::new(fish::BasicFish::new(spawn, speed, direction))),
    };
}

pub fn new_random_fish(fishes: &mut Vec<Box<dyn Fish>>) {
    let mut rng = rand::rng();

    let fish = match rng.random_range(0..3) {
        0 => Fishes::FihFish,
        1 => Fishes::BigFish,
        _ => Fishes::BasicFish,
    };

    new_fish(fish, fishes);
}
