// Public API.
pub use error::AppError;
pub use loader::load_scripts;
pub use script::Script;

// Private modules.
mod error;
mod loader;
mod script;
