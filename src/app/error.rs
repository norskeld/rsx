use std::fmt;

/// Custom error to be used and propagated across the app.
pub struct AppError(pub String);

impl fmt::Debug for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("AppError").field("message", &self.0).finish()
  }
}

impl fmt::Display for AppError {
  fn fmt(&self, writer: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(writer, "{}", self.0)
  }
}
