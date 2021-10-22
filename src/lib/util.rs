use std::error::Error;
use std::io::{Read, Write};
use termion::{clear, cursor};

pub fn clear_screen() -> String {
  format!("{}{}", clear::All, cursor::Goto(1, 1))
}

pub fn just_left() -> String {
  cursor::Left(u16::MAX).to_string()
}

pub fn pause() -> Result<(), Box<dyn Error>> {
  use std::io::{self};
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press Enter key to continue...")?;
  stdout.flush()?;

  let _ = stdin.read(&mut [0u8])?;

  Ok(())
}
