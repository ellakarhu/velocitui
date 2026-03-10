mod collision;
mod enemy;
mod item;
mod player;
mod ui;

use std::{
    io::{self, Result, Stdout, Write},
    time::{Duration, Instant},
};

use crossterm::event::KeyCode;
use rand::rngs::SmallRng;

use super::{NewState, State, gameover::GameOver};

use enemy::Enemy;
use item::Item;
use player::Player;

pub struct Game {
    stdout: Stdout,
    rng: SmallRng,
    player: Player,
    item: Item,
    enemies: Vec<Enemy>,
    spawn_timer: Instant,
    ui_timer: Instant,
}

impl Game {
    const SPAWN_INTERVAL: Duration = Duration::from_secs(30);

    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            rng: rand::make_rng(),
            player: Player::new(),
            item: Item::new(),
            enemies: Vec::new(),
            spawn_timer: Instant::now(),
            ui_timer: Instant::now(),
        }
    }

    fn draw(&mut self) -> Result<()> {
        self.item.draw(&mut self.stdout)?;

        for enemy in &mut self.enemies {
            enemy.draw(&mut self.stdout)?;
        }

        self.player.draw(&mut self.stdout)?;

        ui::draw_status(&mut self.stdout, &self.player, self.ui_timer.elapsed())?;

        self.stdout.flush()?;
        self.clear()?;

        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.item.clear(&mut self.stdout)?;

        for enemy in &mut self.enemies {
            enemy.clear(&mut self.stdout)?;
        }

        self.player.clear(&mut self.stdout)?;

        Ok(())
    }
}

impl State for Game {
    fn enter(&mut self) -> Result<()> {
        self.item.reposition(&mut self.rng);
        self.enemies.push(Enemy::new(&mut self.rng));

        ui::draw_border(&mut self.stdout)?;

        Ok(())
    }

    fn update(&mut self, input: KeyCode) -> Result<NewState> {
        self.player.update(input);

        if self.spawn_timer.elapsed() > Self::SPAWN_INTERVAL {
            self.enemies.push(Enemy::new(&mut self.rng));
            self.spawn_timer = Instant::now();
        }

        for enemy in &mut self.enemies {
            enemy.update(&mut self.rng);
        }

        collision::check(
            &mut self.rng,
            &mut self.player,
            &mut self.item,
            &self.enemies,
        );

        self.draw()?;

        if self.player.health() == 0 {
            let s = Box::new(GameOver::new(self.player.score(), self.ui_timer.elapsed()));
            return Ok(NewState::State(s));
        }

        Ok(NewState::None)
    }
}
