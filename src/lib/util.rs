use termion::{clear, cursor};

pub fn clear_screen() {
  print!("{}{}", clear::All, cursor::Goto(1, 1))
}
