use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde_json::{Map, Value};

use super::{AppError, Script};

/// Gets the current working directory path + `package.json`.
fn resolve_pkg_path() -> Result<PathBuf, AppError> {
  match env::current_dir() {
    | Ok(dir) => Ok(dir.join("package.json")),
    | Err(_) => {
      Err(AppError(
        "Couldn't resolve the current working directory.".to_string(),
      ))
    },
  }
}

/// Produces a collection of `Script`s extracted from a given `serde_json::Map`.
fn produce_scripts(scripts_map: &Map<String, Value>) -> Vec<Script> {
  scripts_map
    .into_iter()
    .enumerate()
    .map(|(id, (key, value))| {
      let script = key.to_string();

      // Raw conversion to string leaves JSON representation, so strings are in quotes. This piece
      // works that around. Also we gracefully default to an empty string if command in JSON wasn't
      // actually a string, and instead it was `null` or something else.
      let command = match value.as_str() {
        | Some(command) => command.to_string(),
        | None => String::new(),
      };

      Script {
        id,
        script,
        command,
      }
    })
    .collect::<Vec<Script>>()
}

/// Loads scripts from `package.json` in the current working directory.
pub fn load_scripts() -> Result<Vec<Script>, AppError> {
  // Get the `package.json`'s path.
  let pkg_path = resolve_pkg_path()?;

  // Check if the `package.json` exists in cwd.
  if !pkg_path.exists() {
    return Err(AppError(
      "There's no `package.json` in the current directory.".to_string(),
    ));
  }

  // Try to open file.
  let mut pkg_file = match File::open(pkg_path) {
    | Ok(file) => file,
    | Err(_) => return Err(AppError("Couldn't open the `package.json`.".to_string())),
  };

  // Try to read contents.
  let mut pkg_contents = String::new();

  if pkg_file.read_to_string(&mut pkg_contents).is_err() {
    return Err(AppError("Couldn't read the `package.json`.".to_string()));
  }

  // Try to parse JSON.
  let pkg: Value = match serde_json::from_str(&pkg_contents) {
    | Ok(contents) => contents,
    | Err(_) => return Err(AppError("Couldn't parse the `package.json`.".to_string())),
  };

  // Check if it has the `scripts` field.
  if let Some(field) = pkg.get("scripts") {
    let scripts_map = field.as_object();

    // Check if the `scripts` field is parseable, i.e. it's an object.
    if let Some(map) = scripts_map {
      if map.is_empty() {
        Err(AppError("No scripts found.".to_string()))
      } else {
        Ok(produce_scripts(map))
      }
    } else {
      Err(AppError("Couldn't parse the `scripts` field.".to_string()))
    }
  } else {
    Err(AppError("No `scripts` field found.".to_string()))
  }
}
