use std::{fmt::Display, io::{Write, stdout}, mem::swap};

use crossterm::{ExecutableCommand, QueueableCommand, cursor::MoveTo, execute, style::{Print, SetBackgroundColor}, terminal::{Clear, ClearType, SetSize, size}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    let (width, height) = (150 as usize, 100 as usize);
    let mut screen = Vec::<char>::new();
    let mut back_screen = Vec::<char>::new();
    screen.resize(width as usize * height as usize, '1');
    back_screen.resize(width as usize * height as usize, '0');

    stdout.execute(SetSize(width as u16, height  as u16))?;

    loop {
        stdout.queue(MoveTo(0, 0))?;
        for index in 0..screen.len() {
            stdout.queue(Print(screen[index]))?;
        }
        stdout.flush()?;

        swap(&mut screen, &mut back_screen);
    }
}
