mod app;
mod cli;
mod prompt;

use std::process;

use app::Script;
use prompt::{Prompt, SelectPrompt, Symbols};

fn main() {
  let cli = cli::create_cli();
  let pm = cli::get_pm(cli);

  match app::load_scripts() {
    | Ok(ref options) => {
      let mut prompt = SelectPrompt::new("Pick a script to execute", options.to_vec());

      match prompt.run() {
        | Ok(Some(item)) => {
          let Script { script, command, .. } = &item;
          let separator = Symbols::MiddleDot.as_str();

          println!("Executing: {script} {separator} {command}");

          match app::execute_script(pm, item) {
            | Err(error) => {
              println!("{}", error);
              process::exit(1);
            },
            | _ => {},
          }
        },
        | Ok(None) => println!("Nothing was picked. Bye!"),
        | Err(error) => println!("Oof, something happened: {:?}", error),
      }
    },
    | Err(error) => {
      println!("{}", error);
      process::exit(1);
    },
  };
}
