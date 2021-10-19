use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Exercise {
  pub name: String,
  pub description: String,
}

impl Exercise {
  pub fn new(name: &str, description: &str) -> Self {
    let name = name.to_owned();
    let description = description.to_owned();
    Exercise { name, description }
  }
}
