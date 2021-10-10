use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

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
  pub workout_type: WorkoutType,
  pub sets: Vec<HashMap<String, String>>,
}

impl Workout {
  pub fn load_file(filename: &str) -> Result<Self, Box<dyn Error>> {
    let f = File::open(filename)?;
    let result = from_reader(f)?;
    Ok(result)
  }
}
