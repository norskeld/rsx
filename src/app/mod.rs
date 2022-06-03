// Public API.
pub use error::AppError;
pub use executor::execute_script;
pub use loader::load_scripts;
pub use pm::PackageManager;
pub use runner::*;
pub use script::Script;

// Private modules.
mod error;
mod executor;
mod loader;
mod pm;
mod runner;
mod script;
