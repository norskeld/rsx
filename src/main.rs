mod loader;
mod prompt;

use prompt::{Prompt, SelectPrompt};

fn main() {
  let options = loader::load_scripts();
  let mut prompt = SelectPrompt::new("Pick a script to execute", options, 64);

  match prompt.run() {
    | Ok(Some(item)) => println!("Executing: {} {}", item.script, item.command),
    | Ok(None) => println!("Nothing was picked. Bye!"),
    | Err(error) => println!("Oof, something happened: {:?}", error),
  }
}
