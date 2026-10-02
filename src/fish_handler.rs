use crate::fish::{Direction, FihFish, Fish, Fishes, Point};
use crossterm::terminal;
use rand::{self, RngExt};

pub fn new_fish(fish: Fishes, fishes: &mut Vec<Box<dyn Fish>>) {
    let mut rng = rand::rng();

    let speed = rng.random_range(1..15);

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
        Fishes::FihFish => fishes.push(Box::new(FihFish::new(spawn, speed, direction))),
    };
}
