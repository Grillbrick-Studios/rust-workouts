use std::fmt;
use std::time::Duration;

use termion::{color, cursor};

use super::workout::Workout;

/// [WARMUP] is a constant exercise that is shown during the warmup period.
const WARMUP: [&str; 2] = [
  "Warmup",
  "Run in place, \
jumping-jacks, or anything \
    to get \
    your \
    heart rate up.",
];

/// [REST] is a constant exercise that is shown in between each set of exercises.
const REST: [&str; 2] = [
  "REST",
  "Take a break, \
  Get a drink of water, \
  Take it easy!",
];

/// [COOLDOWN] is a constant exercise that is shown at the end of the workout until the program
/// exits.
const COOLDOWN: [&str; 2] = ["Cooldown", "Great Job!"];

/// The type of the screen used for timing
pub enum ScreenType {
  WarmUp(Duration),
  Rest(Duration),
  Exercise(usize, Duration),
  Cooldown(Duration),
}

impl ScreenType {
  pub fn warm_up() -> Self {
    ScreenType::WarmUp(Duration::from_secs(60 * 5))
  }

  pub fn rest() -> Self {
    ScreenType::Rest(Duration::from_secs(60))
  }

  pub fn exercise(id: usize) -> Self {
    ScreenType::Exercise(id, Duration::from_secs(20))
  }

  pub fn cooldown() -> Self {
    ScreenType::Cooldown(Duration::from_secs(60 * 10))
  }

  pub fn duration(&self) -> &Duration {
    match self {
      ScreenType::WarmUp(d) => d,
      ScreenType::Rest(d) => d,
      ScreenType::Exercise(_, d) => d,
      ScreenType::Cooldown(d) => d,
    }
  }
}

pub struct Screen {
  pub output: String,
  pub screen_type: ScreenType,
}

impl Screen {
  pub fn warmup_with_set(set: &[Vec<String>], duration: u64) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::WarmUp(Duration::from_secs(duration));

    let warmup = [WARMUP[0].to_string(), WARMUP[1].to_string()];

    output += &Workout::show_exercise(&warmup);
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

  pub fn set_with_rest(set: &[Vec<String>], id: usize) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::exercise(id);

    let rest = [REST[0].to_string(), REST[1].to_string()];

    output += &Workout::show_set(set);
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += &Workout::show_exercise(&rest);

    Screen { output, screen_type }
  }

  pub fn set_with_cooldown(set: &[Vec<String>], id: usize) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::exercise(id);

    let cooldown = [COOLDOWN[0].to_string(), COOLDOWN[1].to_string()];

    output += &Workout::show_set(set);
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += &Workout::show_exercise(&cooldown);

    Screen { output, screen_type }
  }

  pub fn cooldown() -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::cooldown();

    let cooldown = [COOLDOWN[0].to_string(), COOLDOWN[1].to_string()];

    output += &Workout::show_exercise(&cooldown);

    Screen { output, screen_type }
  }

  pub fn rest_with_set(set: &[Vec<String>]) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::rest();

    let rest = [REST[0].to_string(), REST[1].to_string()];

    output += &Workout::show_exercise(&rest);
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
