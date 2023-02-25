use std::ffi::OsString;
use std::fmt;
use std::fmt::Formatter;
use std::path::Path;

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
pub struct ValueExpected {
    pub value: String,
    pub file: Option<OsString>
}

impl ValueExpected {
    pub fn new(value: String, file: Option<OsString>) -> Self {
        ValueExpected {
            value,
            file
        }
    }

    pub fn from_path(value: String, path: &Path) -> Self {
        Self::new(
            value,
            path.file_name().map(|name| name.to_os_string())
        )
    }
}

impl fmt::Display for ValueExpected {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(file) = self.file.as_ref().and_then(|f| f.to_str()) {
            write!(f, "value for '{}' in '{}' expected, but not found", self.value, file)
        } else {
            write!(f, "value for '{}' expected, but not found", self.value)
        }
    }
}

impl std::error::Error for ValueExpected {}

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
