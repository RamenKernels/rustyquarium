use crate::fish::Fishes;
use crate::{Direction, Point};
use crossterm::terminal;
use rand::{self, RngExt};

pub fn new_random_fish(fishes: &mut Vec<Fishes>) {
    let mut rng = rand::rng();

    let speed = rng.random_range(1..10);

    let direction = match rng.random_range(1..=2) {
        1 => Direction::Left,
        _ => Direction::Right,
    };

    let pos = Point::new(
        match direction {
            Direction::Left => {
                terminal::size()
                    .expect("Should have been able to read terminal size")
                    .0 as i8
            }
            Direction::Right => 0,
        },
        rng.random_range(
            1..terminal::size()
                .expect("Should have been able to read terminal size")
                .1 as i8,
        ),
    );

    let fish = match rng.random_range(0..3) {
        0 => Fishes::fihfish(pos, speed, direction),
        1 => Fishes::bigfish(pos, speed, direction),
        _ => Fishes::basicfish(pos, speed, direction),
    };

    fishes.push(fish);
}
