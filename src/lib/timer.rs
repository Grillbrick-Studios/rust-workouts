pub trait Timer {
  fn as_time(&self) -> String;
}

impl Timer for u64 {
  fn as_time(&self) -> String {
    format!("{:02}:{:02}:{:02}", self / 60 / 60, (self / 60) % 60, self % 60)
  }
}
