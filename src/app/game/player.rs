use std::{
    cmp,
    io::{Result, Stdout},
};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use velocitui::{Direction, HEIGHT, Point, WIDTH};

pub struct Player {
    score: u32,
    health: u16,
    pos: Point,
    trail: [Point; 4],
    dir: Direction,
    speed: f32,
    iframes: u16,
}

impl Player {
    pub const MAX_HEALTH: u16 = 5;

    pub fn new() -> Self {
        Self {
            score: 0,
            health: 5,
            pos: Point {
                x: WIDTH as f32 / 2.0,
                y: HEIGHT as f32 / 2.0,
            },
            trail: [Point { x: 1.0, y: 1.0 }; 4],
            dir: Direction::None,
            speed: 1.0,
            iframes: 0,
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn health(&self) -> u16 {
        self.health
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn update(&mut self, input: KeyCode) {
        self.trail.rotate_right(1);
        self.trail[0] = self.pos;

        self.iframes = self.iframes.saturating_sub(1);

        self.dir = match input {
            KeyCode::Char('k') => Direction::Up,
            KeyCode::Char('j') => Direction::Down,
            KeyCode::Char('h') => Direction::Left,
            KeyCode::Char('l') => Direction::Right,
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            KeyCode::Char('w') => Direction::Up,
            KeyCode::Char('s') => Direction::Down,
            KeyCode::Char('a') => Direction::Left,
            KeyCode::Char('d') => Direction::Right,
            _ => self.dir,
        };

        match self.dir {
            Direction::Up => self.pos.y -= self.speed / 2.0,
            Direction::Down => self.pos.y += self.speed / 2.0,
            Direction::Left => self.pos.x -= self.speed,
            Direction::Right => self.pos.x += self.speed,
            Direction::None => (),
        }

        self.check_screen_edge();
    }

    pub fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        if self.iframes > 0 {
            stdout.queue(SetForegroundColor(Color::DarkRed))?;
        } else {
            stdout.queue(SetForegroundColor(Color::DarkCyan))?;
        }

        stdout
            .queue(MoveTo(
                self.trail[0].x.round() as u16,
                self.trail[0].y.round() as u16,
            ))?
            .queue(Print(":"))?
            .queue(MoveTo(self.pos.x.round() as u16, self.pos.y.round() as u16))?
            .queue(Print("@"))?
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

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    pub fn add_health(&mut self) {
        self.health = cmp::min(Self::MAX_HEALTH, self.health + 1);
    }

    pub fn take_damage(&mut self) {
        if self.iframes == 0 {
            self.health = self.health.saturating_sub(1);
            self.iframes = 100;
        }
    }

    fn check_screen_edge(&mut self) {
        if self.pos.x < 1.0 {
            self.pos.x = WIDTH as f32 - 1.0;
        } else if self.pos.x > WIDTH as f32 - 1.0 {
            self.pos.x = 1.0;
        }

        if self.pos.y < 3.0 {
            self.pos.y = HEIGHT as f32 - 1.0;
        } else if self.pos.y > HEIGHT as f32 - 1.0 {
            self.pos.y = 3.0;
        }
    }
}
