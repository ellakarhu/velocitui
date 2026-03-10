use std::{
    io::{Result, Stdout},
    time::Duration,
};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use velocitui::{HEIGHT, WIDTH};

use super::Player;

pub fn draw_border(stdout: &mut Stdout) -> Result<()> {
    stdout.queue(SetForegroundColor(Color::DarkCyan))?;

    for x in 1..WIDTH {
        stdout
            .queue(MoveTo(x, 0))?
            .queue(Print("─"))?
            .queue(MoveTo(x, 2))?
            .queue(Print("─"))?
            .queue(MoveTo(x, HEIGHT))?
            .queue(Print("─"))?;
    }

    for y in 1..HEIGHT {
        stdout
            .queue(MoveTo(0, y))?
            .queue(Print("│"))?
            .queue(MoveTo(WIDTH, y))?
            .queue(Print("│"))?;
    }

    stdout
        .queue(MoveTo(0, 0))?
        .queue(Print("┌"))?
        .queue(MoveTo(WIDTH, 0))?
        .queue(Print("┐"))?
        .queue(MoveTo(0, 2))?
        .queue(Print("├"))?
        .queue(MoveTo(WIDTH, 2))?
        .queue(Print("┤"))?
        .queue(MoveTo(0, HEIGHT))?
        .queue(Print("└"))?
        .queue(MoveTo(WIDTH, HEIGHT))?
        .queue(Print("┘"))?
        .queue(ResetColor)?;

    Ok(())
}

pub fn draw_status(stdout: &mut Stdout, player: &Player, timer: Duration) -> Result<()> {
    for i in 1..Player::MAX_HEALTH + 1 {
        let x = WIDTH - 4 * i;

        if i > player.health() {
            stdout.queue(SetForegroundColor(Color::DarkRed))?;
        } else {
            stdout.queue(SetForegroundColor(Color::DarkGreen))?;
        }

        stdout
            .queue(MoveTo(x, 1))?
            .queue(Print("[+]"))?
            .queue(ResetColor)?;
    }

    stdout
        .queue(MoveTo(2, 1))?
        .queue(Print(format!("Score: {}", player.score())))?
        .queue(MoveTo(WIDTH / 2 - 1, 1))?
        .queue(Print(velocitui::format_time(timer)))?;

    Ok(())
}
