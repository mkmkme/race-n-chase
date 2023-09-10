use std::fmt;

#[derive(Debug, derive_more::From)]
pub enum RNCError {
    InvalidArguments,
    // FileNotFound,
}

impl std::fmt::Display for RNCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RNCError::InvalidArguments => write!(f, "Invalid arguments!\n\nUsage: car-dump <car_file>"),
            // RNCError::FileNotFound => write!(f, "File not found"),
        }
    }
}

impl std::error::Error for RNCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
