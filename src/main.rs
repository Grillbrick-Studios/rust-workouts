use crate::lib::workout::{DayOfWeek, Workout, WorkoutType};

use std::collections::HashMap;
use std::error::Error;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
  let monday = Workout::load_file("data/3-1-LBA.yml")?;

  println!("workout = {:?}", monday);
  Ok(())
}
