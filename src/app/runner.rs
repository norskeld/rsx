use std::process;

use crate::app;
use crate::prompt::*;
use super::{AppError, PackageManager, Script};

/// Runs a prompt to select a script interactively, and then execute it.
pub fn run_interactive(pm: PackageManager, options: Vec<Script>) -> Result<(), AppError> {
  let mut prompt = SelectPrompt::new("Pick a script to execute", options);

  let script = match prompt.run() {
    | Ok(Some(script)) => script,
    | Ok(None) => {
      println!("Nothing was picked.");
      return Ok(());
    },
    | Err(error) => {
      eprintln!("Something happened: {error:?}.");
      return Ok(());
    },
  };

  run((pm, script))
}

/// Checks if provided script name is present in the loaded from `package.json` scripts, and then executes it.
pub fn run_direct(pm: PackageManager, options: Vec<Script>, selection: String) -> Result<(), AppError> {
  options
    .iter()
    .find(|&Script { script, .. }| script == &selection)
    .ok_or_else(|| AppError(format!("There's no script named '{selection}'.")))
    .map(|script| run((pm, script.to_owned())))?
}

fn run((pm, selection): (PackageManager, Script)) -> Result<(), AppError> {
  let Script { script, command, .. } = &selection;

  println!(
    "Executing: {script} {separator} {command}",
    separator = Symbols::MiddleDot.as_str()
  );

  if let Err(AppError(error)) = app::execute_script(pm, selection) {
    eprintln!("{error}");
    process::exit(1);
  }

  Ok(())
}
