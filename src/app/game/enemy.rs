use std::io::{Result, Stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rand::{RngExt, rngs::SmallRng};

use velocitui::{Direction, HEIGHT, Point, WIDTH};

pub struct Enemy {
    pos: Point,
    trail: [Point; 4],
    dir: Direction,
    speed: f32,
}

impl Enemy {
    pub fn new(rng: &mut SmallRng) -> Self {
        let mut enemy = Self {
            pos: Point { x: 1.0, y: 1.0 },
            trail: [Point { x: 1.0, y: 1.0 }; 4],
            dir: Direction::None,
            speed: 0.5,
        };

        enemy.reposition(rng);

        enemy
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn update(&mut self, rng: &mut SmallRng) {
        self.trail.rotate_right(1);
        self.trail[0] = self.pos;

        match self.dir {
            Direction::Up => self.pos.y -= self.speed / 2.0,
            Direction::Down => self.pos.y += self.speed / 2.0,
            Direction::Left => self.pos.x -= self.speed,
            Direction::Right => self.pos.x += self.speed,
            Direction::None => (),
        }

        self.check_screen_edge(rng);
    }

    pub fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        stdout
            .queue(SetForegroundColor(Color::DarkYellow))?
            .queue(MoveTo(
                self.trail[1].x.round() as u16,
                self.trail[1].y.round() as u16,
            ))?
            .queue(Print(":"))?
            .queue(MoveTo(self.pos.x.round() as u16, self.pos.y.round() as u16))?
            .queue(Print("#"))?
            .queue(ResetColor)?;

        Ok(())
    }

    pub fn clear(&self, stdout: &mut Stdout) -> Result<()> {
        stdout
            .queue(MoveTo(
                self.trail.last().unwrap().x.round() as u16,
                self.trail.last().unwrap().y.round() as u16,
            ))?
            .queue(Print(" "))?;

        Ok(())
    }

    fn check_screen_edge(&mut self, rng: &mut SmallRng) {
        if self.pos.x < 1.0
            || self.pos.x > WIDTH as f32 - 1.0
            || self.pos.y < 3.0
            || self.pos.y > HEIGHT as f32 - 1.0
        {
            self.reposition(rng);
        }
    }

    fn reposition(&mut self, rng: &mut SmallRng) {
        let dirs = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        self.dir = dirs[rng.random_range(0..4)];

        match self.dir {
            Direction::Up => {
                self.pos.x = rng.random_range(1..WIDTH) as f32;
                self.pos.y = HEIGHT as f32 - 1.0;
            }
            Direction::Down => {
                self.pos.x = rng.random_range(1..WIDTH) as f32;
                self.pos.y = 3.0;
            }
            Direction::Left => {
                self.pos.x = WIDTH as f32 - 1.0;
                self.pos.y = rng.random_range(3..HEIGHT) as f32;
            }
            Direction::Right => {
                self.pos.x = 1.0;
                self.pos.y = rng.random_range(3..HEIGHT) as f32;
            }
            Direction::None => (),
        }
    }
}
