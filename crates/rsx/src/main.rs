use std::process;

use rsx_prompt::{Prompt, SelectPrompt};
use rsx_app::load_scripts;

fn main() {
  match load_scripts() {
    | Ok(options) => {
      let limit = options.len() * 2;

      let mut prompt = SelectPrompt::new("Pick a script to execute", options, limit);

      match prompt.run() {
        | Ok(Some(item)) => println!("Executing: {} {}", item.script, item.command),
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
