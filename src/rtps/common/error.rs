use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;
use std::num::ParseIntError;

pub type GenError = Box<dyn std::error::Error>;
pub type GenResult<T> = Result<T, GenError>;

#[derive(Debug)]
pub struct RtpsError {
    details: String,
}

impl RtpsError {
    pub fn new(msg: &str) -> RtpsError {
        RtpsError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for RtpsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RtpsError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseFloatError> for RtpsError {
    fn from(err: ParseFloatError) -> Self {
        RtpsError::new(&err.to_string())
    }
}

impl From<ParseIntError> for RtpsError {
    fn from(err: ParseIntError) -> Self {
        RtpsError::new(&err.to_string())
    }
}
