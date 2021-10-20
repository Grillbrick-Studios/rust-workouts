use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct LockedUInt {
  pub value: usize,
  pub max: usize,
}

impl PartialEq<usize> for LockedUInt {
  fn eq(&self, other: &usize) -> bool {
    self.value == *other
  }
}

impl PartialOrd<usize> for LockedUInt {
  fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
    self.value.partial_cmp(other)
  }
}

impl AddAssign<usize> for LockedUInt {
  fn add_assign(&mut self, rhs: usize) {
    self.value =
      if self.value + rhs >= self.max { self.max } else { self.value + rhs }
  }
}

impl Add<usize> for LockedUInt {
  type Output = LockedUInt;

  fn add(self, rhs: usize) -> Self::Output {
    let max = self.max;
    let value =
      if self.value + rhs >= self.max { self.max } else { self.value + rhs };
    LockedUInt { max, value }
  }
}

impl SubAssign<usize> for LockedUInt {
  fn sub_assign(&mut self, rhs: usize) {
    self.value = if rhs > self.value { 0 } else { self.value - rhs }
  }
}

impl Sub<usize> for LockedUInt {
  type Output = LockedUInt;

  fn sub(self, rhs: usize) -> Self::Output {
    let max = self.max;
    let value = if rhs > self.value { 0 } else { self.value - rhs };
    LockedUInt { max, value }
  }
}
