use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Other(String),
    Ruuvi(ruuvi::err::Error),
    Sqlx(sqlx::Error),
}

pub type Res<T> = Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Other(e) => write!(f, "{}", e),
            Error::Ruuvi(e) => write!(f, "{}", e),
            Error::Sqlx(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {}

impl From<macaddr::ParseError> for Error {
    fn from(value: macaddr::ParseError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_owned())
    }
}

impl From<ruuvi::err::Error> for Error {
    fn from(value: ruuvi::err::Error) -> Self {
        Self::Ruuvi(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Sqlx(value.into())
    }
}
