use failure;
use goblin;
use std;
use url;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    error: Box<dyn std::error::Error + Send + Sync>,
}

impl Error {
    pub fn new<E>(error: E) -> Error
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Error {
            error: error.into(),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.error, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

// This is important for other errors to wrap this one.
impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.error.description()
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.error.cause()
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        self.error.source()
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::new(error)
    }
}

impl std::convert::From<pdb::Error> for Error {
    fn from(error: pdb::Error) -> Self {
        Self::new(error)
    }
}

impl std::convert::From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::new(error)
    }
}

impl std::convert::From<failure::Error> for Error {
    fn from(error: failure::Error) -> Self {
        Self::new(error)
    }
}

impl std::convert::From<goblin::error::Error> for Error {
    fn from(error: goblin::error::Error) -> Self {
        Self::new(error)
    }
}