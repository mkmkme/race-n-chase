use thiserror::Error;


#[derive(Error, Debug)]
pub enum FXTError {
    #[error("Unexpected key at position {0}")]
    UnexpectedKey(usize),
    #[error("Unexpected value at position {0}")]
    UnexpectedValue(usize),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
