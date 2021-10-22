use std::{
  env,
  path::{Path, PathBuf},
};

pub const CONFIG_DIR: &str = "workouts";
pub const DATA_DIR: &str = "data";
pub const IMPORT_DIR: &str = "import";
pub const SOUNDS_DIR: &str = "sounds";
const CONFIG_HOME_ENV_VAR: &str = "XDG_CONFIG_HOME";

pub enum Source {
  Input,
  Output,
}
impl Source {
  pub fn path(&self) -> PathBuf {
    match self {
      Source::Input => Path::new(".").to_path_buf(),
      Source::Output => config_path(),
    }
  }
}

pub fn config_home() -> PathBuf {
  if let Some(p) = env::var_os(CONFIG_HOME_ENV_VAR) {
    Path::new(&p).to_path_buf()
  } else {
    let p = env!("HOME");
    Path::new(p).join(".config")
  }
}

pub fn config_path() -> PathBuf {
  config_home().join(CONFIG_DIR)
}

pub fn data_path() -> PathBuf {
  config_path().join(DATA_DIR)
}

pub fn import_path() -> PathBuf {
  config_path().join(IMPORT_DIR)
}

pub fn sounds_path() -> PathBuf {
  config_path().join(SOUNDS_DIR)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_generates_config_path_with_config_dir() {
    assert_eq!(config_path().file_name().unwrap(), CONFIG_DIR);
  }

  #[test]
  fn it_generates_data_path_with_data_dir() {
    assert_eq!(data_path().file_name().unwrap(), DATA_DIR);
  }

  #[test]
  fn it_generates_import_path_with_import_dir() {
    assert_eq!(import_path().file_name().unwrap(), IMPORT_DIR);
  }

  #[test]
  fn it_generates_sounds_path_with_sounds_dir() {
    assert_eq!(sounds_path().file_name().unwrap(), SOUNDS_DIR);
  }
}
