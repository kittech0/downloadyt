use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};

pub type BoxResult<T> = Result<T, BoxError>;

pub struct BoxError(Box<dyn Error>);

impl<T: Into<Box<dyn Error>>> From<T> for BoxError {
    fn from(value: T) -> Self {
        BoxError(value.into())
    }
}

impl Debug for BoxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Debug::fmt(&self.0, f)
    }
}

pub struct NotFoundVideoError {}

impl Debug for NotFoundVideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Video not found")
    }
}

impl Display for NotFoundVideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Video not found")
    }
}

impl Error for NotFoundVideoError {}
