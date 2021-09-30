pub trait Prompt<T> {
  fn run(&mut self) -> std::result::Result<Option<T>, crossterm::ErrorKind>;
  fn render(&mut self) -> crossterm::Result<()>;
  fn handle(&mut self, event: crossterm::event::KeyEvent);
}

#[derive(Eq, PartialEq, Debug)]
pub enum State {
  Created,
  Running,
  Aborted,
  Completed,
}

impl State {
  pub fn is_done(&self) -> bool {
    *self == State::Aborted || *self == State::Completed
  }
}

impl Default for State {
  fn default() -> State {
    State::Created
  }
}
