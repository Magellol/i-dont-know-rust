use std::error;
use std::fmt;

#[derive(Debug)]
pub struct PoolCreationError {
  pub message: String,
}

impl fmt::Display for PoolCreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

// Looks like we have to implement this trait for custom errors
// because it provides a bunch of defaults? Unclear what these defaults are doing yet.
impl error::Error for PoolCreationError {}