use std::fmt;
use std::fmt::Formatter;

pub type Error = Box<dyn std::error::Error>;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ModAlreadyExists(pub String);

impl fmt::Display for ModAlreadyExists {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "mod '{}' already exists", self.0)
    }
}

impl std::error::Error for ModAlreadyExists {}

#[derive(Debug)]
pub struct FormatValueExpected;

impl fmt::Display for FormatValueExpected {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "value for 'format' in 'niter.json' not found")
    }
}

impl std::error::Error for FormatValueExpected {}

#[derive(Debug)]
pub struct AlreadyInitiated;

impl fmt::Display for AlreadyInitiated {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "a modpack is already initiated in this directory")
    }
}

impl std::error::Error for AlreadyInitiated {}

#[derive(Debug)]
pub struct MainFileNotFound;

impl fmt::Display for MainFileNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "could not find 'niter.json'")
    }
}

impl std::error::Error for MainFileNotFound {}

#[derive(Debug)]
pub struct NotADirectory;

impl fmt::Display for NotADirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "not a directory")
    }
}

impl std::error::Error for NotADirectory {}

#[derive(Debug)]
pub struct UnsupportedFormat(pub String);

impl fmt::Display for UnsupportedFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "format '{}' is not supported", self.0)
    }
}

impl std::error::Error for UnsupportedFormat {}
