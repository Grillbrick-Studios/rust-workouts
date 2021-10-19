use super::enums::{DayOfWeek, WorkoutType};
use crate::lib::workout_plus::WorkoutPlus;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::{error::Error, ffi::OsStr, fs::File};

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
  pub fn load_file(filename: &OsStr) -> Result<WorkoutPlus, Box<dyn Error>> {
    let f = File::open(filename)?;
    let result: Workout = from_reader(f)?;
    let workout = WorkoutPlus::from_workout(result.compress());
    let f = File::open(format!("{:?}+", filename))?;
    to_writer(f, &workout);
    Ok(workout)
  }

  /// Load everything - currently manually updated.
  pub fn load_all() -> Result<Vec<WorkoutPlus<'static>>, Box<dyn Error>> {
    let mut paths = std::fs::read_dir("data")?
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, std::io::Error>>()?;
    paths.sort();
    let mut workouts = vec![];
    for path in paths {
      let s = path.as_os_str();
      if let Ok(workout) = Workout::load_file(s) {
        workouts.push(workout);
      }
    }

    Ok(workouts)
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
