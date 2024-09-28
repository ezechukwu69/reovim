use std::{
    io::stdout,
    time::{Duration, Instant},
};

mod buffer;
mod components;
mod logger;

use anyhow::Error;
use buffer::buffer::Buffer;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> anyhow::Result<(), Error> {
    let args: Vec<String> = std::env::args().into_iter().collect();
    if args.len() < 2 {
        println!("Usage requires a file name {}", args[0]);
        return Ok(());
    }
    let _ = terminal::enable_raw_mode();
    let mut buffer = Buffer::new(args[1].to_owned())?;

    // let mut last_tick = Instant::now();
    // let tick_rate = Duration::from_millis(200);

    execute!(stdout(), EnterAlternateScreen)?;
    loop {
        // if last_tick.elapsed() >= tick_rate {
        // last_tick = Instant::now();
        // }
        buffer.draw()?;
        // if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            event::Event::Key(key_event) => {
                if let KeyCode::Char('q') = key_event.code {
                    break;
                }
                buffer.handle_key(key_event)?;
            }
            _ => {}
        }
        // }
    }
    execute!(stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
