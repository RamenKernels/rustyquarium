use crossterm::style::Color;

use crate::{FishOutOfBounds, Point, Sprite};

pub const FIHFISH_SPRITE: Sprite = Sprite {
    characters: "fih",
    color: Color::Grey,
};

pub const BASICFISH_SPRITE: Sprite = Sprite {
    characters: "><>",
    color: Color::Blue,
};

pub const BIGFISH_SPRITE: Sprite = Sprite {
    characters: "
         \\,,
        ,\\\\\\\\,,, 
    \\\\;;:::::::::o
    //;:::::::::::>
       ````//````
        ",

    color: Color::Green,
};

pub struct Fish {
    pub pos: Point<i8>,
    pub sprite: Sprite,
    pub update_func: fn(&Self) -> Result<(), FishOutOfBounds>,
    speed: i8,
}

/*
pub enum Fishes {
    FihFish { data: FishData },
    BasicFish { data: FishData },
    BigFish { data: FishData },
}

impl Fishes {
    pub fn fihfish(pos: Point<i8>, speed: i8) -> Self {
        Fishes::FihFish {
            data: FishData {
                pos,
                speed,
                sprite: FIHFISH_SPRITE,
                move_frames: 0,
            },
        }
    }

    pub fn basicfish(pos: Point<i8>, speed: i8) -> Self {
        Fishes::BasicFish {
            data: FishData {
                pos,
                speed,
                sprite: FIHFISH_SPRITE,
                move_frames: 0,
            },
        }
    }

    pub fn bigfish(pos: Point<i8>, speed: i8) -> Self {
        Fishes::BasicFish {
            data: FishData {
                pos,
                speed,
                sprite: FIHFISH_SPRITE,
                move_frames: 0,
            },
        }
    }

    fn sprite(&self) -> &Sprite {
        match self {
            Fishes::FihFish { data } => &data.sprite,
            Fishes::BigFish { data } => &data.sprite,
            Fishes::BasicFish { data } => &data.sprite,
        }
    }
}

impl Fish for Fishes {
    fn update(&mut self) -> Result<(), FishOutOfBounds> {
        Ok(())
    }

    fn pos(&self) -> &Point<i8> {}

    fn render(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            cursor::MoveTo(self.pos().x as u16, self.pos().y as u16),
            style::PrintStyledContent(self.sprite().characters.with(self.sprite().color))
        )?;

        Ok(())
    }
} */

/*
pub struct FihFish {
    pub pos: Point<i8>,
    pub speed: u16,
    pub direction: Direction,
    move_frames: u16,
}

impl FihFish {
    pub fn new(pos: Point<i8>, speed: u16, direction: Direction) -> Self {
        FihFish {
            pos,
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
                            .0 as i8
                    {
                        return Err(FishOutOfBounds);
                    }
                }
            }
        }

        self.move_frames += 1;

        Ok(())
    }

    fn pos(&self) -> &Point<i8> {
        &self.pos
    }

    fn render(&self) -> io::Result<()> {
        let sprite = match self.direction {
            Direction::Left => String::from("hif"),
            Direction::Right => String::from("fih"),
        };

        execute!(
            io::stdout(),
            cursor::MoveTo(self.pos.x as u16, self.pos.y as u16),
            style::Print(sprite)
        )?;

        Ok(())
    }
}

pub struct BasicFish {
    pub pos: Point<i8>,
    pub sprite: String,
    pub speed: u16,
    pub direction: Direction,
    move_frames: u16,
}

impl BasicFish {
    pub fn new(pos: Point<i8>, speed: u16, direction: Direction) -> Self {
        let sprite = match direction {
            Direction::Left => String::from("<'><"),
            Direction::Right => String::from("><'>"),
        };

        BasicFish {
            pos,
            sprite,
            speed,
            direction,
            move_frames: 0,
        }
    }
}

impl Fish for BasicFish {
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
                            .0 as i8
                    {
                        return Err(FishOutOfBounds);
                    }
                }
            }
        }

        self.move_frames += 1;

        Ok(())
    }

    fn pos(&self) -> &Point<i8> {
        &self.pos
    }

    fn render(&self) -> io::Result<()> {
        match self.direction {
            Direction::Left => {
                execute!(
                    io::stdout(),
                    cursor::MoveTo(self.pos.x as u16, self.pos.y as u16),
                    style::PrintStyledContent(self.sprite.clone().with(Color::Blue)),
                )?;
            }
            Direction::Right => {
                execute!(
                    io::stdout(),
                    cursor::MoveTo(self.pos.x as u16, self.pos.y as u16),
                    style::PrintStyledContent(self.sprite.clone().with(Color::Blue)),
                )?;
            }
        };

        Ok(())
    }
}

pub struct BigFish {
    pub pos: Point<i8>,
    pub speed: u16,
    pub direction: Direction,
    move_frames: u16,
}

impl BigFish {
    pub fn new(pos: Point<i8>, speed: u16, direction: Direction) -> Self {
        BigFish {
            pos,
            speed,
            direction,
            move_frames: 0,
        }
    }
}

impl Fish for BigFish {
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
                            .0 as i8
                    {
                        return Err(FishOutOfBounds);
                    }
                }
            }
        }

        self.move_frames += 1;

        Ok(())
    }

    fn pos(&self) -> &Point<i8> {
        &self.pos
    }

    fn render(&self) -> io::Result<()> {
        /*use crossterm::style::{style, Color, Attribute, Stylize};
         *        ,,/
         *    ,,,////,
         *  o:::::::::;;//
         * <:::::::::::;\\
         *   ````\\````
         */

        match self.direction {
            Direction::Left => {
                let lines: Vec<&str> = vec![
                    "        ,,/",
                    "    ,,,////,",
                    "  o:::::::::;;//",
                    "<:::::::::::;\\\\",
                    "   ````\\````",
                ];

                for (i, line) in lines.iter().enumerate() {
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(self.pos.x as u16, self.pos.y as u16 + i as u16),
                        style::PrintStyledContent(line.with(Color::Green)),
                    )?;
                }
            }
            Direction::Right => {
                let lines: Vec<&str> = vec![
                    "     \\,,",
                    "    ,\\\\\\\\,,,",
                    "\\\\;;:::::::::o",
                    "//;:::::::::::>",
                    "   ````/````",
                ];

                for (i, line) in lines.iter().enumerate() {
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(self.pos.x as u16, self.pos.y as u16 + i as u16),
                        style::PrintStyledContent(line.with(Color::Green)),
                    )?;
                }
            }
        };

        Ok(())
    }
}
*/
