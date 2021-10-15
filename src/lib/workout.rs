use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use termion::{
  color, cursor, event::Key, input::TermRead, raw::IntoRawMode, style,
};

// use std::thread::sleep;
use crate::lib::screens::{Screen, ScreenType};
use crate::lib::util::clear_screen;
use crate::lib::workout::locked_u_int::LockedUInt;

mod locked_u_int;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Workout {
  pub title: String,
  pub link: String,
  pub day: DayOfWeek,
  pub warmup_length: u64,
  pub workout_type: WorkoutType,
  pub sets: Vec<Vec<Vec<String>>>,
}

impl Workout {
  /// new generates a default hashmap and then fills it with the provided workouts.
  pub fn new(
    title: &str,
    link: Option<String>,
    day: DayOfWeek,
    workout_type: WorkoutType,
    sets: Vec<Vec<Vec<String>>>,
  ) -> Self {
    Workout {
      title: title.to_string(),
      link: if let Some(link) = link { link } else { "".to_string() },
      day,
      warmup_length: 60 * 5,
      workout_type,
      sets,
    }
    .compress()
  }

  fn compress(mut self) -> Self {
    let mut sets = vec![];
    for i in 0..self.sets.len() {
      let mut set = vec![];
      let old_set = &self.sets[i];
      for old_exercise in old_set.iter() {
        let (head, tail) = old_exercise.split_at(1);
        let head = &head[0];
        let exercise = vec![String::from(head), tail.join(" ")];
        set.push(exercise);
      }
      sets.push(set);
    }
    self.sets = sets;
    self
  }

  /// Load a single yaml file as a workout.
  pub fn load_file(filename: &str) -> Result<Self, Box<dyn Error>> {
    let f = File::open(filename)?;
    let result: Workout = from_reader(f)?;
    Ok(result.compress())
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
    let mut d = Duration::default();

    for screen in self.screens() {
      d += *screen.screen_type.duration();
    }

    d
  }

  pub fn screens(&self) -> Vec<Screen> {
    let mut result = vec![];

    for i in 0..self.sets.len() {
      if let Some(set) = self.sets.get(i) {
        if i == 0 {
          result.push(Screen::warmup_with_set(set, self.warmup_length));
        }
        result.push(Screen::set_with_rest(set, i + 1));
      }
    }
    result.push(Screen::cooldown());

    result
  }

  pub fn run(&self) {
    // Get the timers
    let _elapsed: u64 = 0;
    let _current: u64 = 0;
    let _remaining = self.duration().as_secs();

    // Get the screens
    let screens = self.screens();
    let cooldown = Screen::cooldown();

    // Go into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Iterate through the sets.
    let mut i = LockedUInt { value: 0, max: screens.len() - 1 };
    loop {
      clear_screen();
      // get the current screen
      let screen = screens.get(i.value).or(Some(&cooldown)).unwrap();

      // show the screen
      write!(
        stdout,
        "{}{}{}\n{}",
        style::Bold,
        match screen.screen_type {
          ScreenType::WarmUp(_) => "WARMING UP!".to_string(),
          ScreenType::Rest(_) => "REST!".to_string(),
          ScreenType::Exercise(i, _) => format!("SET {}", i),
          ScreenType::Cooldown(_) => "Aah - Feel better?".to_string(),
        },
        style::Reset,
        screen,
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
            i -= 1;
            clear_screen();
            stdout.flush().unwrap();
            break;
          }
          // down and right will both go forward one screen.
          Key::Down | Key::Right => {
            write!(stdout, "Going forth...").unwrap();
            i += 1;
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
  pub fn show_set(set: &[Vec<String>]) -> String {
    let mut result = String::new();
    for exercise in set.iter() {
      result += &Self::show_exercise(exercise);
    }
    result
  }

  /// Show a single exercise
  pub fn show_exercise(exercise: &[String]) -> String {
    if let [exercise, description] = exercise {
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
    } else {
      format!(
        "\n\
        {}{}{}This Exercise is invalid - it does not have a title and description!{}{}\n\
        {}{:#?}\n\
        \n",
        cursor::Left(u16::MAX),
        style::Bold,
        color::Fg(color::Red),
        style::Reset,
        color::Fg(color::Reset),
        cursor::Left(u16::MAX),
        exercise,
      )
    }
  }
}

impl Default for Workout {
  fn default() -> Self {
    Workout::new(
      "Default Workout",
      None,
      DayOfWeek::Monday,
      WorkoutType::UpperBodyAbs,
      vec![
        vec![
          vec![
            "Do stuff".to_string(),
            "This is how you do that stuff".to_string(),
          ],
          vec![
            "Do other stuff".to_string(),
            "This is how you do that other stuff".to_string(),
          ],
          vec![
            "Do more stuff".to_string(),
            "This is how you do that stuff".to_string(),
          ],
        ],
        vec![
          vec![
            "Do stuff".to_string(),
            "This is how you do that stuff".to_string(),
          ],
          vec![
            "Do other stuff".to_string(),
            "This is how you do that other stuff".to_string(),
          ],
          vec![
            "Do more stuff".to_string(),
            "This is how you do that stuff".to_string(),
          ],
        ],
        vec![
          vec![
            "Do stuff".to_string(),
            "This is how you do that stuff".to_string(),
          ],
          vec![
            "Do other stuff".to_string(),
            "This is how you do that other stuff".to_string(),
          ],
          vec![
            "Do more stuff".to_string(),
            "This is how you do that stuff".to_string(),
          ],
        ],
      ],
    )
  }
}
