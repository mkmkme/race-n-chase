use std::fmt;

#[derive(Debug, derive_more::From)]
pub enum RNCError {
    InvalidArguments,
}

impl std::fmt::Display for RNCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RNCError::InvalidArguments => {
                write!(f, "Invalid arguments!\n\nUsage: car-dump <car_file>")
            }
        }
    }
}

impl std::error::Error for RNCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
