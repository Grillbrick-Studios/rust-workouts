use std::fmt;
use std::time::Duration;

use termion::{color, cursor};

use super::workout_plus::exercise::{Exercise, ExerciseSet};

/// [WARMUP] is a constant exercise that is shown during the warmup period.
const WARMUP: Exercise = Exercise {
  name: "Warmup",
  description: "Run in place, \
jumping-jacks, or anything \
    to get \
    your \
    heart rate up.",
  selected: false,
};

/// [REST] is a constant exercise that is shown in between each set of exercises.
const REST: Exercise = Exercise {
  name: "REST",
  description: "Take a break, \
  Get a drink of water, \
  Take it easy!",
  selected: false,
};

/// [COOLDOWN] is a constant exercise that is shown at the end of the workout until the program
/// exits.
const COOLDOWN: Exercise =
  Exercise { name: "Cooldown", description: "Great Job!", selected: false };

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
  pub fn warmup_with_set(set: &ExerciseSet, duration: u64) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::WarmUp(Duration::from_secs(duration));

    output += WARMUP.to_string().as_str();
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += set.to_string().as_str();

    Screen { output, screen_type }
  }

  pub fn set_with_rest(set: &ExerciseSet, id: usize) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::exercise(id);

    output += set.to_string().as_str();
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += REST.to_string().as_str();

    Screen { output, screen_type }
  }

  pub fn set_with_cooldown(set: &ExerciseSet, id: usize) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::exercise(id);

    output += set.to_string().as_str();
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += COOLDOWN.to_string().as_str();

    Screen { output, screen_type }
  }

  pub fn cooldown() -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::cooldown();

    output += COOLDOWN.to_string().as_str();

    Screen { output, screen_type }
  }

  pub fn rest_with_set(set: &ExerciseSet) -> Self {
    let mut output = String::new();
    let screen_type = ScreenType::rest();

    output += REST.to_string().as_str();
    output += format!(
      "{}{}UP NEXT:{}",
      cursor::Left(u16::MAX),
      color::Fg(color::Red),
      color::Fg(color::Reset)
    )
    .as_str();
    output += set.to_string().as_str();

    Screen { output, screen_type }
  }
}

impl fmt::Display for Screen {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.output, cursor::Left(u16::MAX))
  }
}
