use std::error::Error;

use terminal_menu::{button, label, menu, mut_menu, run, TerminalMenuItem};

use lib::workout::Workout;
// use std::thread::sleep;
use crate::lib::util::clear_screen;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
  show_workouts()?;
  Ok(())
}

fn show_workouts() -> Result<(), Box<dyn Error>> {
  let workouts = Workout::load_all()?;

  let mut list: Vec<TerminalMenuItem> =
    workouts.iter().map(|w| button(&w.title)).collect();
  list.insert(0, label("Please select a workout:"));
  let m = menu(list);
  clear_screen();
  run(&m);

  let title = mut_menu(&m).selected_item_name().to_owned();

  if let Some(workout) = workouts.iter().find(|w| w.title == title) {
    workout.run();
  }

  Ok(())
}

fn show_default_workout() -> Result<(), Box<dyn Error>> {
  let workout = Workout::default();

  println!("{}", serde_yaml::to_string(&workout)?);
  Ok(())
}
