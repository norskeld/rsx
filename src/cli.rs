use std::env;
use std::rc::Rc;

use clap::{ArgMatches, Command};

use crate::app::PackageManager;

pub struct Cli {
  matches: Rc<ArgMatches>,
}

impl Cli {
  pub fn new() -> Self {
    let app = Self::create_app();
    let matches = Rc::new(app.get_matches());

    Cli { matches }
  }

  /// Creates a clap app.
  pub fn create_app() -> Command {
    const PM_HEADING: &str = "Package managers";

    Command::new(clap::crate_name!())
      .version(clap::crate_version!())
      .about(clap::crate_description!())
      // Positional argument.
      .arg(clap::arg!([script] "Script to execute directly").required(false))
      // Rest arguments (after --) that can be passed to a script.
      .arg(
        clap::arg!([args] "Arguments to pass to a script")
          .num_args(1..)
          .last(true),
      )
      // Flags for selecting package manager.
      .arg(
        clap::arg!(-n --npm "Use npm to run script")
          .required(false)
          .help_heading(PM_HEADING),
      )
      .arg(
        clap::arg!(-p --pnpm "Use pnpm to run script")
          .required(false)
          .help_heading(PM_HEADING),
      )
      .arg(
        clap::arg!(-y --yarn "Use yarn to run script")
          .required(false)
          .help_heading(PM_HEADING),
      )
  }

  /// Gets a script passed in as positional argument.
  pub fn get_script(&self) -> Option<String> {
    self.matches.get_one::<String>("script").cloned()
  }

  /// Resolves package manager from env and/or clap app. Rules:
  ///
  /// - Flags have priority over everything.
  /// - If env variable `SX_PM` is set and no flag present, then use it.
  /// - If nothing else matched, use the default package manager - `npm`.
  pub fn get_pm(&self) -> PackageManager {
    let matches = &self.matches;

    let pm_matrix = (
      matches.get_flag("npm"),
      matches.get_flag("pnpm"),
      matches.get_flag("yarn"),
    );

    let pm_from_env = env::var_os("SX_PM").map_or(PackageManager::Npm, |value| {
      if let Some(pm) = value.to_ascii_lowercase().to_str() {
        return match pm {
          | "npm" => PackageManager::Npm,
          | "pnpm" => PackageManager::Pnpm,
          | "yarn" => PackageManager::Yarn,
          | _ => PackageManager::Npm,
        };
      }

      PackageManager::Npm
    });

    match pm_matrix {
      | (true, ..) => PackageManager::Npm,
      | (_, true, _) => PackageManager::Pnpm,
      | (_, _, true) => PackageManager::Yarn,
      | (..) => pm_from_env,
    }
  }

  /// Gets additional arguments (after --) that can be passed to a script.
  pub fn get_args(&self) -> Vec<String> {
    self
      .matches
      .get_many::<String>("args")
      .map(|vals| vals.cloned().collect::<Vec<String>>())
      .unwrap_or_default()
  }
}
