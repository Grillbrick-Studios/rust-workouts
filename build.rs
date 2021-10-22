use anyhow::{Context, Result};
use std::fs::{copy, create_dir_all};

use workout_paths::*;

fn main() -> Result<()> {
  println!("cargo:rerun-if-changed=data");
  println!("cargo:rerun-if-changed=import");

  // join them with the final output directories
  my_copy(DATA_DIR).context("Error finding the data directory")?;
  my_copy(IMPORT_DIR).context("Error finding the import directory")?;
  my_copy(SOUNDS_DIR).context("Error finding the sounds directory")?;

  Ok(())
}

fn my_copy(path: &str) -> Result<()> {
  // first get the input and output directories
  let out_dir = Source::Output.path();
  let in_dir = Source::Input.path();

  // join them with the final output directories
  let out_dir = out_dir.join(path);

  // join input directories
  let in_dir = in_dir.join(path);

  // then generate the output directories if they don't exist
  create_dir_all(&out_dir).unwrap_or(());

  let paths = std::fs::read_dir(in_dir)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, std::io::Error>>()?;
  for path in paths {
    let filename = path.file_name().unwrap();
    let out_path = out_dir.join(filename);
    let out_path = out_path.as_os_str();
    let s = path.as_os_str();
    println!("{:?} -> {:?}", s, out_path);
    copy(s, out_path)
      .context(format!("Error copying {:?} -> {:?}", s, out_path))?;
  }

  Ok(())
}
