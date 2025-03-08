/// Error conglomeration for every possible error that can occur in cfront.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("I/O error occured: {0}")]
  IoError(#[from] std::io::Error),
}
