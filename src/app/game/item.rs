use std::io::{Result, Stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Print, Stylize},
};
use rand::{RngExt, rngs::SmallRng};

use velocitui::{HEIGHT, Point, WIDTH};

#[derive(Clone, Copy)]
pub enum Variant {
    ScoreUp,
    HealthUp,
}

pub struct Item {
    pos: Point,
    variant: Variant,
}

impl Item {
    const HEALTH_CHANCE: f64 = 0.05;

    pub fn new() -> Self {
        Self {
            pos: Point { x: 1.0, y: 1.0 },
            variant: Variant::ScoreUp,
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn variant(&self) -> Variant {
        self.variant
    }

    pub fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        let s = match self.variant {
            Variant::ScoreUp => "(o)".stylize(),
            Variant::HealthUp => "[+]".dark_green(),
        };

        stdout
            .queue(MoveTo(
                self.pos.x.round() as u16 - 1,
                self.pos.y.round() as u16,
            ))?
            .queue(Print(s))?;

        Ok(())
    }

    pub fn clear(&self, stdout: &mut Stdout) -> Result<()> {
        stdout
            .queue(MoveTo(
                self.pos.x.round() as u16 - 1,
                self.pos.y.round() as u16,
            ))?
            .queue(Print("   "))?;

        Ok(())
    }

    pub fn reposition(&mut self, rng: &mut SmallRng) {
        self.pos.x = rng.random_range(3..WIDTH - 2) as f32;
        self.pos.y = rng.random_range(5..HEIGHT - 2) as f32;

        if rng.random_bool(Self::HEALTH_CHANCE) {
            self.variant = Variant::HealthUp;
        } else {
            self.variant = Variant::ScoreUp;
        }
    }
}
