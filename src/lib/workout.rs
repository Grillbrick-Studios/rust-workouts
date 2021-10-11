use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::error::Error;
use std::fs::File;
// use std::thread::sleep;
use crate::lib::util::clear_screen;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use termion::{
  color, cursor, event::Key, input::TermRead, raw::IntoRawMode, style,
};

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

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkoutType {
  LowerBodyAbs,
  UpperBodyAbs,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DayOfWeek {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}

type Exercise = (String, String);

type ExerciseSet = Vec<Exercise>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workout {
  pub title: String,
  pub link: String,
  pub day: DayOfWeek,
  pub warmup_length: u64,
  pub workout_type: WorkoutType,
  pub sets: Vec<ExerciseSet>,
}

impl Workout {
  /// Load a single yaml file as a workout.
  pub fn load_file(filename: &str) -> Result<Self, Box<dyn Error>> {
    let f = File::open(filename)?;
    let result = from_reader(f)?;
    Ok(result)
  }

  /// Load everything - currently manually updated.
  pub fn load_all() -> Result<Vec<Self>, Box<dyn Error>> {
    let workouts = vec![
      Workout::load_file("data/3-1-LBA.yml")?,
      Workout::load_file("data/3-2-UBA.yml")?,
      Workout::load_file("data/3-3-LBA.yml")?,
      Workout::load_file("data/3-4-UBA.yml")?,
    ];

    Ok(workouts)
  }

  pub fn duration(&self) -> Duration {
    let mut seconds = self.warmup_length; // 5 minute warm-up.

    self.sets.iter().for_each(|s| {
      seconds +=
        // Repeat 3 times
        3 *
        (s.len() as u64
        // 20 seconds per workout
        * 20
        // 60 seconds rest
        + 60);
    });

    Duration::from_secs(seconds)
  }

  pub fn run(&self) {
    // reconfigure the constant exercises to be compatible with owned exercises.
    let _rest = (&String::from(REST.0), &String::from(REST.1));
    let warmup = (String::from(WARMUP.0), String::from(WARMUP.1));
    let cooldown_exercise =
      (String::from(COOLDOWN.0), String::from(COOLDOWN.1));
    let cooldown_set = vec![cooldown_exercise];

    // Get the timers
    let _elapsed: u64 = 0;
    let _current: u64 = 0;
    let _remaining = self.duration().as_secs();

    // Get the number of sets
    let num_sets = self.sets.len();

    // Go into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

    // show warmup
    // TODO: Break warmup and each set into screens that are generated and iterated through.
    clear_screen();
    write!(stdout, "{}", Self::show_exercise(&warmup)).unwrap();
    stdout.flush().unwrap();

    // Iterate through the sets.
    let mut i = 0;
    loop {
      // get the current set
      let current_set = self.sets.get(i).or(Some(&cooldown_set)).unwrap();

      // show the set
      write!(
        stdout,
        "{}Set {}! \n{}",
        style::Framed,
        i + 1,
        Self::show_set(current_set)
      )
      .unwrap();
      stdout.flush().unwrap();

      // wait for input then clear the screen in preparation for the next set.
      // Get stdin and lock it.
      let stdin = stdin();
      let keys = stdin.keys();
      for key in keys {
        match key.unwrap() {
          // q - quits the program
          Key::Char('q') => return,
          // up and left will both go back one screen.
          Key::Up | Key::Left => {
            write!(stdout, "Going back...").unwrap();
            i -= if i > 0 { 1 } else { 0 };
            clear_screen();
            stdout.flush().unwrap();
            break;
          }
          // down and right will both go forward one screen.
          Key::Down | Key::Right => {
            write!(stdout, "Going forth...").unwrap();
            i += if i < num_sets { 1 } else { 0 };
            clear_screen();
            stdout.flush().unwrap();
            break;
          }
          _ => (),
        }
      }
    }
  }

  /// Iterates through a given set and displays to the screen
  fn show_set(set: &[Exercise]) -> String {
    let mut result = String::new();
    for exercise in set.iter() {
      result += &Self::show_exercise(exercise);
    }
    result
  }

  /// Show a single exercise
  fn show_exercise(exercise: &Exercise) -> String {
    let (exercise, description) = exercise;
    format!(
      "\n\
      {}{}{}{}\n\
      {}{}{}{}\n\
      \n",
      cursor::Left(u16::MAX),
      style::Bold,
      color::Fg(color::Red),
      exercise,
      cursor::Left(u16::MAX),
      style::Reset,
      color::Fg(color::Reset),
      description,
    )
  }
}

impl Default for Workout {
  fn default() -> Self {
    Workout {
      title: "".to_string(),
      link: "".to_string(),
      day: DayOfWeek::Monday,
      warmup_length: 0,
      workout_type: WorkoutType::LowerBodyAbs,
      sets: vec![
        vec![
          (
            "Do stuff".to_string(),
            "This is you how you do that stuff".to_string(),
          ),
          (
            "Do other stuff".to_string(),
            "This is you how you do that other stuff".to_string(),
          ),
          (
            "Do more stuff".to_string(),
            "This is you how you do that stuff".to_string(),
          ),
        ],
        vec![
          (
            "Do stuff".to_string(),
            "This is you how you do that stuff".to_string(),
          ),
          (
            "Do other stuff".to_string(),
            "This is you how you do that other stuff".to_string(),
          ),
          (
            "Do more stuff".to_string(),
            "This is you how you do that stuff".to_string(),
          ),
        ],
        vec![
          (
            "Do stuff".to_string(),
            "This is you how you do that stuff".to_string(),
          ),
          (
            "Do other stuff".to_string(),
            "This is you how you do that other stuff".to_string(),
          ),
          (
            "Do more stuff".to_string(),
            "This is you how you do that stuff".to_string(),
          ),
        ],
      ],
    }
  }
}
