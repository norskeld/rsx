use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct Script {
  pub id: usize,
  pub script: &'static str,
  pub command: &'static str,
}

impl Display for Script {
  fn fmt(&self, writer: &mut Formatter<'_>) -> Result {
    write!(writer, "{} {}", self.script, self.command)
  }
}
