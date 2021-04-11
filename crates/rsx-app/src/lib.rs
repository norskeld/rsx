// Public API.
pub use loader::load_scripts;
pub use script::Script;
pub use error::*;

// Private modules.
mod error;
mod loader;
mod script;
