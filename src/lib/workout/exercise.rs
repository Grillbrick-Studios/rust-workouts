use crate::lib::enums::ExerciseType;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use termion::{color, cursor, style};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Exercise {
  pub name: String,
  pub description: String,
  pub selected: bool,
}

impl Exercise {
  pub fn new(name: &str, description: &str) -> Self {
    let name = name.to_owned();
    let description = description.to_owned();
    Exercise { name, description, selected: false }
  }

  pub fn from_vec(v: Vec<String>) -> Self {
    let mut i = v.into_iter();
    let name = if let Some(s) = i.next() { s } else { "".to_owned() };
    let mut description = String::new();
    loop {
      if let Some(s) = i.next() {
        description.push_str(s.as_str());
      } else {
        return Self { name, description, selected: false };
      };
    }
  }
}

impl fmt::Display for Exercise {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "\n\
      {}{}{}{}{}\n\
      {}{}{}{}\n\
      \n",
      cursor::Left(u16::MAX),
      style::Bold,
      color::Fg(color::Red),
      if self.selected { " --> " } else { "     " },
      self.name,
      cursor::Left(u16::MAX),
      style::Reset,
      color::Fg(color::Reset),
      self.description,
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExerciseSet {
  pub exercises: (Exercise, Exercise, Exercise),
  pub exercise_type: ExerciseType,
}

impl ExerciseSet {
  pub fn from_vec(v: Vec<Vec<String>>, t: &ExerciseType) -> Self {
    let mut i = v.into_iter();
    let e1 = if let Some(s) = i.next() {
      Exercise::from_vec(s)
    } else {
      panic!("Invalid vector sent to Exercise Set");
    };
    let e2 = if let Some(s) = i.next() {
      Exercise::from_vec(s)
    } else {
      panic!("Invalid vector sent to Exercise Set");
    };
    let e3 = if let Some(s) = i.next() {
      Exercise::from_vec(s)
    } else {
      panic!("Invalid vector sent to Exercise Set");
    };
    ExerciseSet { exercises: (e1, e2, e3), exercise_type: *t }
  }

  pub fn select(&mut self, i: u8) {
    match i {
      1 => {
        self.exercises.0.selected = true;
        self.exercises.1.selected = false;
        self.exercises.2.selected = false;
      }
      2 => {
        self.exercises.0.selected = false;
        self.exercises.1.selected = true;
        self.exercises.2.selected = false;
      }
      3 => {
        self.exercises.0.selected = false;
        self.exercises.1.selected = false;
        self.exercises.2.selected = true;
      }
      _ => {
        self.exercises.0.selected = false;
        self.exercises.1.selected = false;
        self.exercises.2.selected = false;
      }
    }
  }
}

impl fmt::Display for ExerciseSet {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}\n{}\n{}\n",
      self.exercises.0, self.exercises.1, self.exercises.2
    )
  }
}
