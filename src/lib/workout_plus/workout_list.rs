use super::WorkoutPlus;
use crate::lib::enums::{DayOfWeek, WorkoutType};

pub trait WorkoutList {
  /// Filter by the day of the week
  fn filter_by_day(&self, _day: &DayOfWeek) -> Vec<&WorkoutPlus>;

  /// Filter by the workout type
  fn filter_by_type(&self, _type: &WorkoutType) -> Vec<&WorkoutPlus>;
}

impl WorkoutList for Vec<WorkoutPlus> {
  fn filter_by_day(&self, day: &DayOfWeek) -> Vec<&WorkoutPlus> {
    self.iter().filter(|w| w.day == *day).collect()
  }

  fn filter_by_type(&self, workout_type: &WorkoutType) -> Vec<&WorkoutPlus> {
    self.iter().filter(|w| w.workout_type == *workout_type).collect()
  }
}
impl WorkoutList for &[WorkoutPlus] {
  fn filter_by_day(&self, day: &DayOfWeek) -> Vec<&WorkoutPlus> {
    self.iter().filter(|w| w.day == *day).collect()
  }

  fn filter_by_type(&self, workout_type: &WorkoutType) -> Vec<&WorkoutPlus> {
    self.iter().filter(|w| w.workout_type == *workout_type).collect()
  }
}
