use std::process;

use rsx_prompt::{Prompt, SelectPrompt, Symbols};
use rsx_app::{PackageManager, Script};

fn main() {
  match rsx_app::load_scripts() {
    | Ok(options) => {
      let pm = PackageManager::Npm; // TODO: Get this from env/cmd.
      let limit = options.len() * 2;

      let mut prompt = SelectPrompt::new("Pick a script to execute", options, limit);

      match prompt.run() {
        | Ok(Some(item)) => {
          let Script { script, command, .. } = &item;
          let separator = Symbols::MiddleDot.as_str();

          println!("Executing: {} {} {}", script, separator, command);

          match rsx_app::execute_script(pm, item) {
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
