use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct ModAlreadyAdded(pub String);

impl fmt::Display for ModAlreadyAdded {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "mod '{}' is already added", self.0)
    }
}

impl Error for ModAlreadyAdded {}

#[derive(Debug)]
pub struct FormatValueExpected;

impl fmt::Display for FormatValueExpected {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "value for 'format' in 'niter.json' not found")
    }
}

impl Error for FormatValueExpected {}

#[derive(Debug)]
pub struct MainFileAlreadyExists;

impl fmt::Display for MainFileAlreadyExists {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "file 'niter.json' already exists")
    }
}

impl Error for MainFileAlreadyExists {}

#[derive(Debug)]
pub struct MainFileNotFound;

impl fmt::Display for MainFileNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "could not find 'niter.json'")
    }
}

impl Error for MainFileNotFound {}

#[derive(Debug)]
pub struct NotADirectory;

impl fmt::Display for NotADirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "not a directory")
    }
}

impl Error for NotADirectory {}

#[derive(Debug)]
pub struct UnsupportedFormat(pub String);

impl fmt::Display for UnsupportedFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "format '{}' is not supported", self.0)
    }
}

impl Error for UnsupportedFormat {}