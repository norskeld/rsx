use std::env;

use clap::{App, ArgGroup};

use crate::app::PackageManager;

/// Creates a clap app.
pub fn create_cli() -> App<'static> {
  const PM_HEADING: &str = "PACKAGE MANAGERS";

  App::new(clap::crate_name!())
    .version(clap::crate_version!())
    .about(clap::crate_description!())
    // Flags for selecting package manager.
    .arg(
      clap::arg!(-n --npm "Use npm to run script.")
        .required(false)
        .help_heading(PM_HEADING),
    )
    .arg(
      clap::arg!(-p --pnpm "Use pnpm to run script.")
        .required(false)
        .help_heading(PM_HEADING),
    )
    .arg(
      clap::arg!(-y --yarn "Use yarn to run script.")
        .required(false)
        .help_heading(PM_HEADING),
    )
    // Logically group flags under "package-manager" label.
    .group(
      ArgGroup::new("package-manager")
        .args(&["npm", "pnpm", "yarn"])
        .required(false),
    )
}

/// Resolves package manager from env and/or clap app. Rules:
///
/// - Flags have priority over everything.
/// - If `SX_PM` is set to something valid and no flag present, then use it.
/// - If nothing else matched, use the default package manager - `npm`.
pub fn get_pm(cli: App) -> PackageManager {
  let matches = cli.get_matches();

  let pm_matrix = (
    matches.is_present("npm"),
    matches.is_present("pnpm"),
    matches.is_present("yarn"),
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
    | (true, _, _) => PackageManager::Npm,
    | (_, true, _) => PackageManager::Pnpm,
    | (_, _, true) => PackageManager::Yarn,
    | (_, _, _) => pm_from_env,
  }
}
