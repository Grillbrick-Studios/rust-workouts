use self::exercise::Exercise;
use super::super::{
  screens::{Screen, ScreenType},
  util::{clear_screen, just_left},
};
use super::{
  enums::{DayOfWeek, WorkoutType},
  locked_u_int::LockedUInt,
  timer::Timer,
};
use rusty_audio::Audio;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{
  error::Error,
  ffi::OsStr,
  fs::File,
  io::{stdin, stdout, Write},
  sync::mpsc,
  thread::{sleep, spawn},
  time::Duration,
};
use termion::{
  color, cursor, event::Key, input::TermRead, raw::IntoRawMode, style,
};

mod exercise;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkoutPlus {
  pub title: String,
  pub link: String,
  pub day: DayOfWeek,
  pub warmup_length: u64,
  pub workout_type: WorkoutType,
  pub sets: Vec<Vec<Exercise>>,
}

impl WorkoutPlus {
  /// new generates a default hashmap and then fills it with the provided workouts.
  pub fn new(
    title: &str,
    link: Option<String>,
    day: DayOfWeek,
    workout_type: WorkoutType,
    sets: Vec<Vec<Exercise>>,
  ) -> Self {
    WorkoutPlus {
      title: title.to_string(),
      link: if let Some(link) = link { link } else { "".to_string() },
      day,
      warmup_length: 60 * 5,
      workout_type,
      sets,
    }
  }

  /// Load a single yaml file as a workout.
  pub fn load_file(filename: &OsStr) -> Result<Self, Box<dyn Error>> {
    let f = File::open(filename)?;
    let result: WorkoutPlus = from_reader(f)?;
    Ok(result)
  }

  /// Load everything - currently manually updated.
  pub fn load_all() -> Result<Vec<Self>, Box<dyn Error>> {
    let mut paths = std::fs::read_dir("data")?
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, std::io::Error>>()?;
    paths.sort();
    let mut workouts = vec![];
    for path in paths {
      let s = path.as_os_str();
      if let Ok(workout) = WorkoutPlus::load_file(s) {
        workouts.push(workout);
      }
    }

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
    // let mut result = vec![];

    // TODO: make this work with Exercises
    // for i in 0..self.sets.len() {
    //   if let Some(set) = self.sets.get(i) {
    //     if i == 0 {
    //       result.push(Screen::warmup_with_set(set, self.warmup_length * 60));
    //     } else {
    //       result.push(Screen::rest_with_set(set));
    //     }
    //     result.push(Screen::set_with_rest(set, 1));
    //     result.push(Screen::set_with_rest(set, 2));
    //     result.push(Screen::set_with_rest(set, 3));
    //     result.push(Screen::rest_with_set(set));
    //     result.push(Screen::set_with_rest(set, 1));
    //     result.push(Screen::set_with_rest(set, 2));
    //     result.push(Screen::set_with_rest(set, 3));
    //     result.push(Screen::rest_with_set(set));
    //     result.push(Screen::set_with_rest(set, 1));
    //     result.push(Screen::set_with_rest(set, 2));
    //     if i == self.sets.len() - 1 {
    //       result.push(Screen::set_with_cooldown(set, 3));
    //     } else {
    //       result.push(Screen::set_with_rest(set, 3));
    //     }
    //   }
    // }
    // result.push(Screen::cooldown());

    // result
    vec![]
  }

  pub fn run(&self) {
    // Get the timers
    let _elapsed: u64 = 0;
    let _current: u64 = 0;
    let _remaining = self.duration().as_secs();

    // Get the screens and times
    let screens = self.screens();
    let cooldown = Screen::cooldown();
    // first get all the times for each screen
    let times: Vec<u64> =
      screens.iter().map(|s| s.screen_type.duration().as_secs()).collect();
    // calculate total time
    let total_time: u64 = times.iter().sum();
    // Now modify to show the accumulated time for each screen
    let times: Vec<u64> = times
      .iter()
      .scan(0, |state, x| {
        *state += x;
        Some(*state - x)
      })
      .collect();

    // initialize audio
    let mut audio = Audio::new();
    audio.add("tick", WorkoutPlus::TICK);
    audio.add("bell", WorkoutPlus::BELL);
    audio.add("whistle", WorkoutPlus::WHISTLE);

    // Go into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

    // start thread and create channel
    let (tx, rx) = mpsc::channel();

    let _handle = spawn(move || {
      // wait for input then clear the screen in preparation for the next set.
      // Get stdin and lock it.
      let stdin = stdin();
      let keys = stdin.keys();
      for key in keys.flatten() {
        if tx.send(key).is_err() {
          return;
        }
      }
    });

    // Iterate through the screens.
    let mut i = LockedUInt { value: 0, max: screens.len() - 1 };
    let mut current_time = 0;
    let mut overtime = false;
    loop {
      // get the current screen
      let screen = screens.get(i.value).or(Some(&cooldown)).unwrap();
      let time_elapsed = *times.get(i.value).unwrap();
      let current_total = screen.screen_type.duration().as_secs();
      let current_time_remaining = if current_time > current_total {
        overtime = true;
        0
      } else {
        current_total - current_time
      };
      let total_time_elapsed = time_elapsed + current_time;
      let total_time_remaining = if total_time_elapsed > total_time {
        overtime = true;
        0
      } else {
        total_time - total_time_elapsed
      };

      // check if a sound needs to be played.
      if current_time_remaining == 7 && i.value < i.max {
        audio.play("tick");
      }
      if current_time == 0 {
        match screen.screen_type {
          ScreenType::Cooldown(_) | ScreenType::Rest(_) => {
            audio.play("whistle");
          }
          ScreenType::Exercise(_, _) => {
            audio.play("bell");
          }
          _ => {}
        }
      }

      let current_time_remaining = current_time_remaining.as_time();
      let total_time_remaining = total_time_remaining.as_time();

      // show the screen
      write!(
        stdout,
        "{}{}Total Elapsed: {}\n\
        {}Total Remaining: {}\n\
        {}Current Elapsed: {}\n\
        {}Current Remaining: {}\n\
        {}{}{}\n{}",
        clear_screen(),
        just_left(),
        total_time_elapsed.as_time(),
        just_left(),
        if overtime { "OVERTIME!" } else { total_time_remaining.as_str() },
        just_left(),
        current_time.as_time(),
        just_left(),
        if overtime { "OVERTIME!" } else { current_time_remaining.as_str() },
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

      if let Ok(key) = rx.try_recv() {
        match key {
          // q - quits the program
          Key::Char('q') => {
            return;
          }
          // up and left will both go back one screen.
          Key::Up | Key::Left => {
            i -= 1;
            current_time = 0;
            write!(stdout, "{}", clear_screen()).unwrap();
            stdout.flush().unwrap();
            continue;
          }
          Key::Home => {
            i.value = 0;
            current_time = 0;
            write!(stdout, "{}", clear_screen()).unwrap();
            stdout.flush().unwrap();
            continue;
          }
          // down and right will both go forward one screen.
          Key::Down | Key::Right => {
            if i.value < i.max {
              i += 1;
              current_time = 0;
              write!(stdout, "{}", clear_screen()).unwrap();
              stdout.flush().unwrap();
              continue;
            }
          }
          Key::End => {
            i.value = i.max;
            current_time = 0;
            write!(stdout, "{}", clear_screen()).unwrap();
            stdout.flush().unwrap();
            continue;
          }
          _ => (),
        }
      }

      // update timer.
      sleep(Duration::from_secs(1));
      current_time += 1;

      // check if current timer is >= screen's duration and increment the screen if necessary.
      if current_time >= current_total && i.value < i.max {
        i += 1;
        current_time = 0;
        continue;
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

  const TICK: &'static str = "sounds/tick.wav";
  const BELL: &'static str = "sounds/bell.wav";
  const WHISTLE: &'static str = "sounds/whistle.wav";
}

impl Default for WorkoutPlus {
  fn default() -> Self {
    WorkoutPlus::new(
      "Default Workout",
      None,
      DayOfWeek::Monday,
      WorkoutType::UpperBodyAbs,
      vec![
        vec![
          Exercise::new("Do stuff", "This is how you do that stuff"),
          Exercise::new(
            "Do other stuff",
            "This is how you do that other stuff",
          ),
          Exercise::new("Do more stuff", "This is how you do that stuff"),
        ],
        vec![
          Exercise::new("Do stuff", "This is how you do that stuff"),
          Exercise::new(
            "Do other stuff",
            "This is how you do that other stuff",
          ),
          Exercise::new("Do more stuff", "This is how you do that stuff"),
        ],
        vec![
          Exercise::new("Do stuff", "This is how you do that stuff"),
          Exercise::new(
            "Do other stuff",
            "This is how you do that other stuff",
          ),
          Exercise::new("Do more stuff", "This is how you do that stuff"),
        ],
      ],
    )
  }
}
