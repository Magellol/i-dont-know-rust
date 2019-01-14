mod error;

pub struct ThreadPool;

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> Result<ThreadPool, error::PoolCreationError> {
    if size == 0 {
      return Err(error::PoolCreationError {
        message: String::from("Creating a pool of size 0 is forbidden."),
      });
    }

    Ok(ThreadPool)
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {

  }
}
