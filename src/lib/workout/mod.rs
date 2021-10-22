use crate::{
  lib::util::pause,
  lib::workout::exercise::{Exercise, ExerciseSet},
  lib::{
    enums::{DayOfWeek, ExerciseType},
    screen::{Screen, ScreenType},
    timer::Timer,
    util::{clear_screen, just_left},
  },
};
use anyhow::Result;
use rusty_audio::Audio;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};

use std::{
  fs::File,
  io::{stdin, stdout, Write},
  path::Path,
  sync::mpsc,
  thread::{sleep, spawn},
  time::Duration,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, style};
use workout_paths::*;

pub mod exercise;
pub mod workout_list;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkoutImport {
  pub title: String,
  pub link: String,
  pub day: DayOfWeek,
  pub warmup_length: u64,
  pub workout_type: ExerciseType,
  pub sets: Vec<Vec<Vec<String>>>,
}

impl WorkoutImport {
  /// Load a single yaml file as a workout.
  pub fn load_file(filename: &Path) -> Result<Self> {
    let f = File::open(filename)?;
    let result: WorkoutImport = from_reader(f)?;
    Ok(result.compress())
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

  pub fn upgrade(self) -> Workout {
    let mut sets = vec![];
    for set in self.sets {
      sets.push(ExerciseSet::from_vec(set, &self.workout_type));
    }
    Workout {
      title: self.title,
      link: self.link,
      day: self.day,
      warmup_length: self.warmup_length,
      workout_type: self.workout_type,
      sets,
    }
  }

  /// Load everything
  pub fn load_all() -> Result<Vec<Self>> {
    println!("Loading Workouts from {:?}", import_path());
    let paths = match std::fs::read_dir(import_path()) {
      Ok(p) => p,
      Err(_) => {
        return Ok(vec![]);
      }
    };
    let mut paths = paths
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, std::io::Error>>()?;
    paths.sort();
    let mut workouts = vec![];
    for path in paths {
      match Self::load_file(&path) {
        Ok(workout) => {
          workouts.push(workout);
        }
        Err(e) => {
          println!("Error importing workout {:?}", e);
          pause()?;
        }
      }
    }

    Ok(workouts)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workout {
  pub title: String,
  pub link: String,
  pub day: DayOfWeek,
  pub warmup_length: u64,
  pub workout_type: ExerciseType,
  pub sets: Vec<ExerciseSet>,
}

impl Workout {
  /// new generates a default hashmap and then fills it with the provided workouts.
  pub fn new(
    title: &str,
    link: Option<String>,
    day: DayOfWeek,
    workout_type: ExerciseType,
    sets: Vec<ExerciseSet>,
  ) -> Self {
    Workout {
      title: title.to_string(),
      link: if let Some(link) = link { link } else { "".to_string() },
      day,
      warmup_length: 60 * 5,
      workout_type,
      sets,
    }
  }

  /// Load a single yaml file as a workout.
  pub fn load_file(filename: &Path) -> Result<Self> {
    let f = File::open(filename)?;
    let result: Workout = from_reader(f)?;
    Ok(result)
  }

  /// Load everything
  pub fn load_all() -> Result<Vec<Self>> {
    println!("Loading Workouts from {:?}", data_path());
    let paths = std::fs::read_dir(data_path())?
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, std::io::Error>>()?;
    let mut workouts = vec![];
    for path in paths {
      match Self::load_file(&path) {
        Ok(workout) => {
          workouts.push(workout);
        }
        Err(e) => {
          println!("Error loading workout file {:?}", e);
          pause()?;
        }
      }
    }
    pause()?;

    Ok(workouts)
  }

  pub fn save(&self) -> Result<()> {
    if std::fs::read_dir(DATA_DIR).is_err() {
      std::fs::create_dir(DATA_DIR)?;
    }
    let path = format!("{}/{}.yml", DATA_DIR, self.title);
    let f = File::create(path)?;
    to_writer(f, &self)?;
    Ok(())
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
          result.push(Screen::warmup_with_set(set, self.warmup_length * 60));
        } else {
          result.push(Screen::rest_with_set(set));
        }
        result.push(Screen::exercise_set_with_rest(set, 1));
        result.push(Screen::exercise_set_with_rest(set, 2));
        result.push(Screen::exercise_set_with_rest(set, 3));
        result.push(Screen::rest_with_set(set));
        result.push(Screen::exercise_set_with_rest(set, 1));
        result.push(Screen::exercise_set_with_rest(set, 2));
        result.push(Screen::exercise_set_with_rest(set, 3));
        result.push(Screen::rest_with_set(set));
        result.push(Screen::exercise_set_with_rest(set, 1));
        result.push(Screen::exercise_set_with_rest(set, 2));
        if i == self.sets.len() - 1 {
          result.push(Screen::exercise_set_with_cooldown(set, 3));
        } else {
          result.push(Screen::exercise_set_with_rest(set, 3));
        }
      }
    }
    result.push(Screen::cooldown());

    // result
    result
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
    let sound_path = sounds_path();

    audio.add("tick", sound_path.join(Workout::TICK).to_str().unwrap());
    audio.add("bell", sound_path.join(Workout::BELL).to_str().unwrap());
    audio.add("whistle", sound_path.join(Workout::WHISTLE).to_str().unwrap());

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
    let mut i = 0;
    let mut current_time = 0;
    let mut overtime = false;
    loop {
      // get the current screen
      let screen = screens.get(i).or(Some(&cooldown)).unwrap();
      let time_elapsed = *times.get(i).unwrap();
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
      if current_time_remaining == 7 && i < screens.len() - 1 {
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
            decrement(&mut i);
            current_time = 0;
            write!(stdout, "{}", clear_screen()).unwrap();
            stdout.flush().unwrap();
            continue;
          }
          Key::Home => {
            i = 0;
            current_time = 0;
            write!(stdout, "{}", clear_screen()).unwrap();
            stdout.flush().unwrap();
            continue;
          }
          // down and right will both go forward one screen.
          Key::Down | Key::Right => {
            if i < screens.len() - 1 {
              increment(&mut i, screens.len() - 1);
              current_time = 0;
              write!(stdout, "{}", clear_screen()).unwrap();
              stdout.flush().unwrap();
              continue;
            }
          }
          Key::End => {
            i = screens.len() - 1;
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
      if current_time >= current_total && i < screens.len() - 1 {
        i += 1;
        current_time = 0;
        continue;
      }
    }
  }

  const TICK: &'static str = "tick.wav";
  const BELL: &'static str = "bell.wav";
  const WHISTLE: &'static str = "whistle.wav";
}

impl Default for Workout {
  fn default() -> Self {
    Workout::new(
      "Default Workout",
      None,
      DayOfWeek::Monday,
      ExerciseType::UpperBodyAbs,
      vec![
        ExerciseSet {
          exercises: (
            Exercise::new("Do stuff", "This is how you do that stuff"),
            Exercise::new(
              "Do other stuff",
              "This is how you do that other stuff",
            ),
            Exercise::new("Do more stuff", "This is how you do that stuff"),
          ),
          exercise_type: ExerciseType::LowerBodyAbs,
        },
        ExerciseSet {
          exercises: (
            Exercise::new("Do stuff", "This is how you do that stuff"),
            Exercise::new(
              "Do other stuff",
              "This is how you do that other stuff",
            ),
            Exercise::new("Do more stuff", "This is how you do that stuff"),
          ),
          exercise_type: ExerciseType::LowerBodyAbs,
        },
        ExerciseSet {
          exercises: (
            Exercise::new("Do stuff", "This is how you do that stuff"),
            Exercise::new(
              "Do other stuff",
              "This is how you do that other stuff",
            ),
            Exercise::new("Do more stuff", "This is how you do that stuff"),
          ),
          exercise_type: ExerciseType::LowerBodyAbs,
        },
      ],
    )
  }
}

fn increment(i: &mut usize, max: usize) {
  if *i < max {
    *i += 1;
  }
}

fn decrement(i: &mut usize) {
  if *i > 0 {
    *i -= 1;
  }
}
