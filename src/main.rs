mod app;

use std::{
    env,
    io::{self, Result},
};

use crossterm::{ExecutableCommand, cursor, terminal};

const HELP_MESSAGE: &str = "\
USAGE:
    velocitui [FLAGS]

FLAGS:
    -h, --help    Prints help message

HOW TO PLAY:
    Move with the vi keys (hjkl), cursor keys, or wasd
    Collect points (o) and health [+]
    Avoid getting hit by enemies
    Press Esc or Ctrl-c to exit the program";

fn init() -> Result<()> {
    terminal::enable_raw_mode()?;
    io::stdout()
        .execute(cursor::Hide)?
        .execute(terminal::EnterAlternateScreen)?;

    Ok(())
}

fn restore() -> Result<()> {
    terminal::disable_raw_mode()?;
    io::stdout()
        .execute(cursor::Show)?
        .execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}

fn main() -> Result<()> {
    for arg in env::args() {
        if arg == "-h" || arg == "--help" {
            println!("{}", HELP_MESSAGE);
            return Ok(());
        }
    }

    init()?;

    if let Err(e) = app::run() {
        restore()?;
        return Err(e);
    };

    restore()?;

    Ok(())
}
