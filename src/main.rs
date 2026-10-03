use std::{io, time::Duration};

use crossterm::{cursor, event, execute, terminal};

use rustyquarium::{
    self,
    fish::{Fish, Fishes},
    fish_handler,
};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(
        io::stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::DisableLineWrap
    )?;

    run()?;

    execute!(
        io::stdout(),
        terminal::LeaveAlternateScreen,
        cursor::Show,
        terminal::DisableLineWrap
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn run() -> io::Result<()> {
    let mut fishes: Vec<Box<dyn Fish>> = Vec::new();

    loop {
        if fishes.len() < 20 {
            fish_handler::new_random_fish(&mut fishes);
        }

        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;

        rustyquarium::update(&mut fishes)?;

        rustyquarium::render(&mut fishes)?;

        if event::poll(Duration::from_millis(50))? {
            let Some(event) = event::read()?.as_key_press_event() else {
                continue;
            };
            if event.code == event::KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}
