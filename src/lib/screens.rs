use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use termion::{
  color, cursor, event::Key, input::TermRead, raw::IntoRawMode, style,
};

use super::workout::Workout;

/// [WARMUP] is a constant exercise that is shown during the warmup period.
const WARMUP: (&str, &str) = (
  "Warmup",
  "Run in place, \
jumping-jacks, or anything \
    to get \
    your \
    heart rate up.",
);

/// [REST] is a constant exercise that is shown in between each set of exercises.
const REST: (&str, &str) = (
  "REST",
  "Take a break, \
  Get a drink of water, \
  Take it easy!",
);

/// [COOLDOWN] is a constant exercise that is shown at the end of the workout until the program
/// exits.
const COOLDOWN: (&str, &str) = ("Cooldown", "Great Job!");

/// The type of the screen used for timing
pub enum ScreenType {
  WarmUp,
  Rest,
  Exercise(usize),
  Cooldown,
}

pub struct Screen {
  pub output: String,
  pub screen_type: ScreenType,
}

impl Screen {
  pub fn warmup_with_set(set: &HashMap<String, String>) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::WarmUp;

    let warmup = (&WARMUP.0.to_string(), &WARMUP.1.to_string());

    output += &Workout::show_exercise(warmup);
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += &Workout::show_set(set);

    Screen { output, screen_type }
  }

  pub fn set_with_rest(set: &HashMap<String, String>, set_num: usize) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::Exercise(set_num);

    let rest = (&REST.0.to_string(), &REST.1.to_string());

    output += &Workout::show_set(set);
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += &Workout::show_exercise(rest);

    Screen { output, screen_type }
  }

  pub fn set_with_cooldown(
    set: &HashMap<String, String>,
    set_num: usize,
  ) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::Exercise(set_num);

    let cooldown = (&COOLDOWN.0.to_string(), &COOLDOWN.1.to_string());

    output += &Workout::show_set(set);
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += &Workout::show_exercise(cooldown);

    Screen { output, screen_type }
  }

  pub fn cooldown() -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::Cooldown;

    let cooldown = (&COOLDOWN.0.to_string(), &COOLDOWN.1.to_string());

    output += &Workout::show_exercise(cooldown);

    Screen { output, screen_type }
  }

  pub fn rest_with_set(set: &HashMap<String, String>) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::Rest;

    let rest = (&REST.0.to_string(), &REST.1.to_string());

    output += &Workout::show_exercise(rest);
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += &Workout::show_set(set);

    Screen { output, screen_type }
  }
}

impl fmt::Display for Screen {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.output, cursor::Left(u16::MAX))
  }
}