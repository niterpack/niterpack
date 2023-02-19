use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct InsideProjectOnly;

impl fmt::Display for InsideProjectOnly {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "this can only be run inside a project")
    }
}

impl Error for InsideProjectOnly {}

#[derive(Debug)]
pub struct NotADirectory;

impl fmt::Display for NotADirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "not a directory")
    }
}

impl Error for NotADirectory {}

#[derive(Debug)]
pub struct UnsupportedFormat(String);

impl fmt::Display for UnsupportedFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "format '{}' is not supported", self.0)
    }
}

impl Error for UnsupportedFormat {}