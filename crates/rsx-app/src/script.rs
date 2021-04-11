use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

/// Represents a single script in `package.json`'s `scripts` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Script {
  pub id: usize,
  pub script: String,
  pub command: String,
}

impl Display for Script {
  fn fmt(&self, writer: &mut Formatter<'_>) -> Result {
    write!(writer, "{} {}", self.script, self.command)
  }
}
