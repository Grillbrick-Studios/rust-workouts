use termion::{clear, cursor};

pub fn clear_screen() {
  print!("{}{}", clear::All, cursor::Goto(1, 1))
}

pub fn just_left() -> String {
  cursor::Left(u16::MAX).to_string()
}
