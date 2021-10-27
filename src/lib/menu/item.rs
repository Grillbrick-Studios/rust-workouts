use crate::lib::menu::action::Action;

#[derive(Clone)]
pub struct MenuItem {
  pub title: String,
  pub action: Action,
}

impl PartialEq<Self> for MenuItem {
  fn eq(&self, other: &Self) -> bool {
    self.title == other.title
  }
}

impl MenuItem {
  pub fn new(title: String, action: &'static dyn Fn()) -> Self {
    let action = Action::new(action);
    Self { title, action }
  }
}
