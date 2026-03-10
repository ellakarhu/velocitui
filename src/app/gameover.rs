use std::{
    io::{self, Result, Stdout, Write},
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::Print,
    terminal::{Clear, ClearType},
};

use velocitui::{HEIGHT, WIDTH};

use super::{NewState, State, game::Game};

pub struct GameOver {
    stdout: Stdout,
    score: u32,
    time: Duration,
}

impl GameOver {
    pub fn new(score: u32, time: Duration) -> Self {
        Self {
            stdout: io::stdout(),
            score,
            time,
        }
    }
}

impl State for GameOver {
    fn enter(&mut self) -> Result<()> {
        self.stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(WIDTH / 2 - 5, HEIGHT / 2 - 4))?
            .queue(Print("Game Over"))?
            .queue(MoveTo(WIDTH / 2 - 5, HEIGHT / 2 - 2))?
            .queue(Print("Score: "))?
            .queue(Print(self.score))?
            .queue(MoveTo(WIDTH / 2 - 5, HEIGHT / 2))?
            .queue(Print("Time: "))?
            .queue(Print(velocitui::format_time(self.time)))?
            .queue(MoveTo(WIDTH / 2 - 9, HEIGHT / 2 + 4))?
            .queue(Print("press r to restart"))?;

        self.stdout.flush()?;

        Ok(())
    }

    fn update(&mut self, input: KeyCode) -> Result<NewState> {
        if input == KeyCode::Char('r') {
            self.stdout.execute(Clear(ClearType::All))?;

            let s = Box::new(Game::new());
            return Ok(NewState::State(s));
        }

        Ok(NewState::None)
    }
}
