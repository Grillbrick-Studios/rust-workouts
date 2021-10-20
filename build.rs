use std::error::Error;
use std::{
  env,
  fs::{copy, create_dir_all, read_dir},
  path::Path,
};

const DATA_DIR: &str = "data";
const IMPORT_DIR: &str = "import";
const SOUNDS_DIR: &str = "sounds";

fn main() -> Result<(), Box<dyn Error>> {
  println!("cargo:rerun-if-changed=data");
  println!("cargo:rerun-if-changed=import");

  // join them with the final output directories
  my_copy(DATA_DIR)?;
  my_copy(IMPORT_DIR)?;
  my_copy(SOUNDS_DIR)?;

  Ok(())
}

fn my_copy(path: &str) -> Result<(), Box<dyn Error>> {
  // first get the input and output directories
  let out_dir = match env::var("WORKOUT_CONFIG_DIR") {
    Ok(s) => {
      let path = Path::new(&s);
      path.to_path_buf().into_os_string()
    }
    Err(_) => {
      let home = env::var_os("HOME").unwrap();
      let path = Path::new(&home);
      let path = path.join(".config").join("workouts");
      path.into_os_string()
    }
  };
  let in_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

  // generate paths for them
  let out_dir = Path::new(&out_dir);
  let in_dir = Path::new(&in_dir);

  // join them with the final output directories
  let out_dir = out_dir.join(path);

  // join input directories
  let in_dir = in_dir.join(path);

  // then generate the output directories if they don't exist
  if read_dir(&out_dir).is_err() {
    create_dir_all(&out_dir)?;
  }

  let paths = std::fs::read_dir(in_dir)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, std::io::Error>>()?;
  for path in paths {
    let filename = path.file_name().unwrap();
    let out_path = out_dir.join(filename);
    let out_path = out_path.as_os_str();
    let s = path.as_os_str();
    println!("{:?} -> {:?}", s, out_path);
    copy(s, out_path)?;
  }

  Ok(())
}
