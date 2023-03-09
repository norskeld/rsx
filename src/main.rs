mod app;
mod cli;
mod prompt;

use std::process;

use app::AppError;
use cli::Cli;

fn main() {
  let cli = Cli::new();

  match app::load_scripts() {
    | Ok(options) => {
      let pm = cli.get_pm();
      let args = cli.get_args();

      let result = match cli.get_script() {
        | Some(script) => app::run_direct(pm, options, script, args),
        | None => app::run_interactive(pm, options, args),
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
