use std::fmt;

/// Error representing a number value not supported
/// by the crate yet
#[derive(Debug, PartialEq, Eq)]
pub struct UnsupportedLargeNumberError {
    pub number: u64,
    pub max_number: u64,
}

impl fmt::Display for UnsupportedLargeNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Numbers larger than {} not supported : {}",
            self.max_number, self.number
        )
    }
}
