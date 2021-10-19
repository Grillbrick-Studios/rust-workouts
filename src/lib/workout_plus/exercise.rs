use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use termion::{color, cursor, style};

#[derive(Serialize, Deserialize, Debug)]
pub struct Exercise<'a> {
  pub name: &'a str,
  pub description: &'a str,
  pub selected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseSet<'a>(
  pub Exercise<'a>,
  pub Exercise<'a>,
  pub Exercise<'a>,
);

impl<'a> ExerciseSet<'a> {
  pub fn from_vec(v: Vec<Exercise>) -> Vec<ExerciseSet> {
    let mut sets: Vec<ExerciseSet> = vec![];
    let mut v = v;
    while v.len() >= 3 {
      let mut set = v.drain(..3);
      sets.push(ExerciseSet(
        set.next().unwrap(),
        set.next().unwrap(),
        set.next().unwrap(),
      ));
    }
    sets
  }
}

impl<'a> Exercise<'a> {
  pub fn new(name: &'static str, description: &'static str) -> Self {
    Exercise { name, description, selected: false }
  }

  pub fn from_vec(v: &'a [std::string::String]) -> Self {
    // todo: create an exercise from a vector
    let (head, tail) = v.split_at(1);
    let head = &head[0];
    Exercise {
      name: head.as_str(),
      description: tail.join(" ").as_str(),
      selected: false,
    }
  }
}

impl<'a> fmt::Display for Exercise<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "\n\
      {}{}{}{}{}\n\
      {}{}{}{}\n\
      \n",
      cursor::Left(u16::MAX),
      style::Bold,
      if self.selected { " --> " } else { "     " },
      color::Fg(color::Red),
      self.name,
      cursor::Left(u16::MAX),
      style::Reset,
      color::Fg(color::Reset),
      self.description,
    )
  }
}

impl<'a> fmt::Display for ExerciseSet<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}\n{}\n{}\n", self.0, self.1, self.2)
  }
}
