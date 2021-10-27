#[derive(Clone)]
pub struct Action {
  pub run: &'static dyn Fn(),
}

impl Action {
  pub fn new(run: &'static dyn Fn()) -> Self {
    Self { run }
  }
}
