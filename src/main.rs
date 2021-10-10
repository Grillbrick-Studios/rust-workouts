use crate::lib::workout::{DayOfWeek, Workout, WorkoutType};
use std::error::Error;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
  let workouts = vec![
    Workout::load_file("data/3-1-LBA.yml")?,
    Workout::load_file("data/3-2-UBA.yml")?,
    Workout::load_file("data/3-3-LBA.yml")?,
    Workout::load_file("data/3-4-UBA.yml")?,
  ];

  println!("workouts = {:#?}", workouts);
  Ok(())
}
