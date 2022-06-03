mod app;
mod cli;
mod prompt;

use std::process;

use cli::Cli;
use app::AppError;

fn main() {
  let cli = Cli::new();

  match app::load_scripts() {
    | Ok(options) => {
      let pm = cli.get_pm();

      let result = match cli.get_script() {
        | Some(script) => app::run_direct(pm, options, script),
        | None => app::run_interactive(pm, options),
      };

      if let Err(AppError(error)) = result {
        println!("{error}");
        process::exit(1);
      }
    },
    | Err(AppError(error)) => {
      println!("{error}");
      process::exit(1);
    },
  }
}
