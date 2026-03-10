use std::time::Duration;

pub const WIDTH: u16 = 80;
pub const HEIGHT: u16 = 30;

#[derive(Clone, Copy)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn format_time(d: Duration) -> String {
    let t = d.as_secs();
    let secs = t % 60;
    let mins = (t / 60) % 60;
    let hours = t / 3600;

    if hours > 0 {
        return format!("{}:{:02}:{:02}", hours, mins, secs);
    } else if mins > 0 {
        return format!("{}:{:02}", mins, secs);
    }

    format!("{}", secs)
}
