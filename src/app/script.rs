use std::fmt::{Display, Formatter, Result};

use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};

use crate::prompt::Symbols;
use super::PackageManager;

#[derive(Clone, Debug)]
pub struct Message {
  pub message: String,
  pub pm: PackageManager,
}

impl Display for Message {
  fn fmt(&self, writer: &mut Formatter<'_>) -> Result {
    let delimiter = Symbols::MiddleDot.as_str().dim();
    let message = self.message.clone();

    let pm = match self.pm {
      | PackageManager::Npm => self.pm.as_str().red(),
      | PackageManager::Pnpm => self.pm.as_str().yellow(),
      | PackageManager::Yarn => self.pm.as_str().blue(),
    };

    write!(writer, "{message} {delimiter} {pm}")
  }
}

/// Represents a single script in `package.json`'s `scripts` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Script {
  pub id: usize,
  pub script: String,
  pub command: String,
}

impl Display for Script {
  fn fmt(&self, writer: &mut Formatter<'_>) -> Result {
    let delimiter = Symbols::MiddleDot.as_str().dim();
    let script = self.script.clone().bold();
    let command = self.command.clone().dim();

    write!(writer, "{script} {delimiter} {command}")
  }
}
