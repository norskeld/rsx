use std::process::{Command, ExitStatus};

use super::{AppError, PackageManager, Script};

/// Executes a given script with specified package manager: npm, pnpm or yarn.
pub fn execute_script(pm: PackageManager, script: Script) -> Result<ExitStatus, AppError> {
  let mut cmd = Command::new(pm.as_str());

  // We need only the `script` field, so destructuring it on assignment.
  let Script { script, .. } = script;

  // npm/pnpm run some-script
  // yarn some-script
  let cmd_args = match pm {
    | PackageManager::Npm | PackageManager::Pnpm => vec!["run", script.as_str()],
    | PackageManager::Yarn => vec![script.as_str()],
  };

  // Set arguments for running a script.
  cmd.args(&cmd_args);

  // Try to spawn a child process.
  if let Ok(mut child) = cmd.spawn() {
    match child.wait() {
      | Ok(exit_status) => Ok(exit_status),
      | Err(_) => Err(AppError(
        "Wasn't able to wait for child process to exit completely.".to_string(),
      )),
    }
  } else {
    let pm = pm.as_str();
    let args = cmd_args.join(" ");

    Err(AppError(format!("Couldn't spawn '{} {}'.", pm, args)))
  }
}
