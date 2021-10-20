use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use DayOfWeek::*;

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub enum WorkoutType {
  LowerBodyAbs,
  UpperBodyAbs,
}

const LOWER_BODY_ABS: &str = "Lower Body & Abs";
const UPPER_BODY_ABS: &str = "Upper Body & Abs";
impl fmt::Display for WorkoutType {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        WorkoutType::LowerBodyAbs => LOWER_BODY_ABS,
        WorkoutType::UpperBodyAbs => UPPER_BODY_ABS,
      }
    )
  }
}

impl WorkoutType {
  pub const VALUES: [Self; 2] = [Self::LowerBodyAbs, Self::UpperBodyAbs];
}

impl FromStr for WorkoutType {
  type Err = ();

  fn from_str(str: &str) -> std::result::Result<Self, ()> {
    match str {
      LOWER_BODY_ABS => Ok(Self::LowerBodyAbs),
      UPPER_BODY_ABS => Ok(Self::UpperBodyAbs),
      &_ => Err(()),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub enum DayOfWeek {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}

const MONDAY: &str = "Monday";
const TUESDAY: &str = "Tuesday";
const WEDNESDAY: &str = "Wednesday";
const THURSDAY: &str = "Thursday";
const FRIDAY: &str = "Friday";
const SATURDAY: &str = "Saturday";
const SUNDAY: &str = "Sunday";

impl DayOfWeek {
  pub const VALUES: [Self; 7] =
    [Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday];
}
impl FromStr for DayOfWeek {
  type Err = ();

  fn from_str(str: &str) -> Result<Self, ()> {
    match str {
      MONDAY => Ok(Monday),
      TUESDAY => Ok(Tuesday),
      WEDNESDAY => Ok(Wednesday),
      THURSDAY => Ok(Thursday),
      FRIDAY => Ok(Friday),
      SATURDAY => Ok(Saturday),
      SUNDAY => Ok(Sunday),
      &_ => Err(()),
    }
  }
}

impl fmt::Display for DayOfWeek {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Monday => MONDAY,
        Tuesday => TUESDAY,
        Wednesday => WEDNESDAY,
        Thursday => THURSDAY,
        Friday => FRIDAY,
        Saturday => SATURDAY,
        Sunday => SUNDAY,
      }
    )
  }
}

pub enum Filter {
  DayOfWeek(DayOfWeek),
  WorkoutType(WorkoutType),
}
