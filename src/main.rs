use std::error::Error;

use terminal_menu::{button, menu, mut_menu, run, TerminalMenuItem};

use crate::lib::workout::Workout;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
  let workouts = vec![
    Workout::load_file("data/3-1-LBA.yml")?,
    Workout::load_file("data/3-2-UBA.yml")?,
    Workout::load_file("data/3-3-LBA.yml")?,
    Workout::load_file("data/3-4-UBA.yml")?,
  ];

  let list: Vec<TerminalMenuItem> =
    workouts.iter().map(|w| button(&w.title)).collect();
  let m = menu(list);
  run(&m);

  println!("workout = {:#?}", workouts[mut_menu(&m).selected_item_index()]);
  Ok(())
}
