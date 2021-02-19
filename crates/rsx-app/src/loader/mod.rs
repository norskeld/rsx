// Re-export public API.
pub use script::Script;
pub use loader::load_scripts;

// Private modules.
mod loader;
mod script;
