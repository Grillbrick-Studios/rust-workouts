use crate::lib::workout::WorkoutImport;
use lib::{
  enums::*,
  util::*,
  workout::{workout_list::WorkoutList, Workout},
};
use rand::prelude::SliceRandom;
use std::error::Error;
use std::process::exit;
use std::str::FromStr;
use terminal_menu::{button, label, menu, mut_menu, run, TerminalMenuItem};
use workout_paths::import_path;

pub mod lib;

fn main() -> Result<(), Box<dyn Error>> {
  import_workouts()?;
  show_workouts()?;
  Ok(())
}

const FILTER_BY_DAY: &str = "Filter by day of the week";
const FILTER_BY_TYPE: &str = "Filter by workout type";
const BACK: &str = "..";
const RANDOM: &str = "Random Workout!";
const QUIT: &str = "Quit";

fn import_workouts() -> Result<(), Box<dyn Error>> {
  println!("Checking for imports...");
  let workouts = WorkoutImport::load_all()?;

  if workouts.is_empty() {
    println!("None found.");
    let path = import_path().join("example.yml");
    println!(
      "If you would like to import a workout see the example: {:?}",
      path
    );
  } else {
    println!("Saving imports...");
    for workout in workouts.into_iter() {
      workout.upgrade().save()?;
    }
    println!("Imports saved!");
  }

  pause()?;

  Ok(())
}

fn show_workouts() -> Result<(), Box<dyn Error>> {
  // first load the workouts
  let workouts = Workout::load_all()?;

  let mut result = main_menu();
  let mut filter;
  filter = filter_menu(&result, &workouts);
  loop {
    let title = if let Ok(filter) = DayOfWeek::from_str(filter.as_str()) {
      workout_menu(Filter::DayOfWeek(filter), &workouts)
    } else if let Ok(filter) = ExerciseType::from_str(filter.as_str()) {
      workout_menu(Filter::WorkoutType(filter), &workouts)
    } else if filter == BACK {
      result = main_menu();
      filter = filter_menu(&result, &workouts);
      continue;
    } else {
      exit(0);
    };

    if let Some(workout) = workouts.iter().find(|w| w.title == title) {
      workout.run();
    } else if title == BACK {
      filter = filter_menu(&result, &workouts);
    } else if title == RANDOM {
      if let Some(workout) =
        if let Ok(filter) = DayOfWeek::from_str(filter.as_str()) {
          let filtered = workouts.filter_by_day(&filter);
          filtered.choose(&mut rand::thread_rng()).cloned()
        } else if let Ok(filter) = ExerciseType::from_str(filter.as_str()) {
          let filtered = workouts.filter_by_type(&filter);
          filtered.choose(&mut rand::thread_rng()).cloned()
        } else {
          panic!("Invalid filter type");
        }
      {
        workout.run();
      }
    } else {
      break;
    }
  }

  Ok(())
}

fn main_menu() -> String {
  let list: Vec<TerminalMenuItem> =
    vec![button(FILTER_BY_DAY), button(FILTER_BY_TYPE), button(QUIT)];
  show_menu(list)
}

fn filter_menu(result: &str, workouts: &[Workout]) -> String {
  let mut list: Vec<TerminalMenuItem> = match result {
    FILTER_BY_TYPE => ExerciseType::VALUES
      .iter()
      .filter(|v| !workouts.filter_by_type(v).is_empty())
      .map(|v| button(v.to_string()))
      .collect(),
    FILTER_BY_DAY => DayOfWeek::VALUES
      .iter()
      .filter(|v| !workouts.filter_by_day(v).is_empty())
      .map(|v| button(v.to_string()))
      .collect(),
    BACK => return main_menu(),
    _ => exit(0),
  };

  list.insert(0, label("Please select a filter:"));
  list.insert(1, button(BACK));
  list.push(button(QUIT));

  show_menu(list)
}

fn workout_menu(filter: Filter, workouts: &[Workout]) -> String {
  let mut list: Vec<TerminalMenuItem> = match filter {
    Filter::DayOfWeek(filter) => {
      workouts.filter_by_day(&filter).iter().map(|w| button(&w.title)).collect()
    }
    Filter::WorkoutType(filter) => workouts
      .filter_by_type(&filter)
      .iter()
      .map(|w| button(&w.title))
      .collect(),
  };

  list.insert(0, label("Choose a workout"));
  list.insert(1, button(BACK));
  list.push(button(RANDOM));
  list.push(button(QUIT));
  show_menu(list)
}

fn show_menu(list: Vec<TerminalMenuItem>) -> String {
  let m = menu(list);
  print!("{}", clear_screen());
  run(&m);

  let result = mut_menu(&m).selected_item_name().to_owned();
  result
}
