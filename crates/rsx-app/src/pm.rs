/// Supported/available package managers.
pub enum PackageManager {
  Npm,
  Pnpm,
  Yarn,
}

impl PackageManager {
  pub fn as_str(&self) -> &'static str {
    match self {
      | PackageManager::Npm => "npm",
      | PackageManager::Pnpm => "pnpm",
      | PackageManager::Yarn => "yarn",
    }
  }
}
