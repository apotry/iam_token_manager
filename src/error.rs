use std::fmt;

#[derive(Debug)]
pub struct LoginError {
    details: String,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to login")
    }
}

impl std::error::Error for LoginError {
    fn description(&self) -> &str {
        &self.details
    }
}
