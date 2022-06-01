use clap::{App, ArgGroup};

use crate::app::PackageManager;

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

pub fn get_pm(cli: App) -> PackageManager {
  let matches = cli.get_matches();

  let is_npm = matches.is_present("npm");
  let is_pnpm = matches.is_present("pnpm");
  let is_yarn = matches.is_present("yarn");

  let pm = match (is_npm, is_pnpm, is_yarn) {
    | (true, _, _) => PackageManager::Npm,
    | (_, true, _) => PackageManager::Pnpm,
    | (_, _, true) => PackageManager::Yarn,
    // TODO: Get from env or default to npm.
    | (_, _, _) => PackageManager::Npm,
  };

  pm
}
