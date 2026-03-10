mod game;
mod gameover;

use std::{
    io::Result,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, KeyCode, KeyModifiers};

use game::Game;

trait State {
    fn enter(&mut self) -> Result<()>;
    fn update(&mut self, input: KeyCode) -> Result<NewState>;
}

enum NewState {
    None,
    State(Box<dyn State>),
}

struct StateMachine {
    state: Box<dyn State>,
}

impl StateMachine {
    pub fn init() -> Result<Self> {
        let mut state = Box::new(Game::new());
        state.enter()?;

        Ok(Self { state })
    }

    pub fn update(&mut self, input: KeyCode) -> Result<()> {
        if let NewState::State(s) = self.state.update(input)? {
            self.state = s;
            self.state.enter()?;
        }

        Ok(())
    }
}

pub fn run() -> Result<()> {
    let mut state_machine = StateMachine::init()?;
    let frame_interval = Duration::from_millis(10);

    loop {
        let frame_timer = Instant::now();
        let (input, modifiers) = read_key_event()?;

        if input == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL)
            || input == KeyCode::Esc
        {
            break;
        }

        state_machine.update(input)?;

        let delta = frame_timer.elapsed();

        if delta < frame_interval {
            thread::sleep(frame_interval - delta);
        }
    }

    Ok(())
}

fn read_key_event() -> Result<(KeyCode, KeyModifiers)> {
    let mut input = KeyCode::Null;
    let mut modifiers = KeyModifiers::empty();

    if event::poll(Duration::from_secs(0))? {
        if let Some(ke) = event::read()?.as_key_press_event() {
            input = ke.code;
            modifiers = ke.modifiers;
        }
    }

    Ok((input, modifiers))
}
