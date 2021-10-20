use super::super::{
  enums::{DayOfWeek, WorkoutType},
  workout::Workout,
};

pub trait WorkoutList {
  /// Filter by the day of the week
  fn filter_by_day(&self, _day: &DayOfWeek) -> Vec<&Workout>;

  /// Filter by the workout type
  fn filter_by_type(&self, _type: &WorkoutType) -> Vec<&Workout>;
}

impl WorkoutList for Vec<Workout> {
  fn filter_by_day(&self, day: &DayOfWeek) -> Vec<&Workout> {
    self.iter().filter(|w| w.day == *day).collect()
  }

  fn filter_by_type(&self, workout_type: &WorkoutType) -> Vec<&Workout> {
    self.iter().filter(|w| w.workout_type == *workout_type).collect()
  }
}
impl WorkoutList for &[Workout] {
  fn filter_by_day(&self, day: &DayOfWeek) -> Vec<&Workout> {
    self.iter().filter(|w| w.day == *day).collect()
  }

  fn filter_by_type(&self, workout_type: &WorkoutType) -> Vec<&Workout> {
    self.iter().filter(|w| w.workout_type == *workout_type).collect()
  }
}
