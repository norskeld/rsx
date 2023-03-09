use std::process;

use super::{AppError, Message, PackageManager, Script};
use crate::app;
use crate::prompt::*;

/// Runs a prompt to select a script interactively, and then execute it.
pub fn run_interactive(
  pm: PackageManager,
  options: Vec<Script>,
  args: Vec<String>,
) -> Result<(), AppError> {
  let prompt_message = Message {
    message: "Pick a script to execute".to_string(),
    pm: pm.clone(),
  };

  let mut prompt = SelectPrompt::new(prompt_message.to_string(), options);

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

  run((pm, script), args)
}

/// Checks if provided script name is present in scripts loaded from `package.json`, and then
/// executes it.
pub fn run_direct(
  pm: PackageManager,
  options: Vec<Script>,
  selection: String,
  args: Vec<String>,
) -> Result<(), AppError> {
  options
    .iter()
    .find(|&Script { script, .. }| script == &selection)
    .ok_or_else(|| AppError(format!("There's no script named '{selection}'.")))
    .map(|script| run((pm, script.to_owned()), args))?
}

fn run((pm, selection): (PackageManager, Script), args: Vec<String>) -> Result<(), AppError> {
  let command = format!(
    "{cmd} -- {args}",
    cmd = &selection.command,
    args = args.join(" ")
  );

  println!(
    "Executing: {script} {separator} {command}",
    script = &selection.script,
    separator = Symbols::MiddleDot.as_str()
  );

  if let Err(AppError(error)) = app::execute_script(pm, selection, args) {
    eprintln!("{error}");
    process::exit(1);
  }

  Ok(())
}
