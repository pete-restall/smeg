use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(crate) struct StringError {
    message: String
}

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl From<Box<dyn Error>> for StringError {
    fn from(value: Box<dyn Error>) -> Self {
        StringError { message: value.to_string() }
    }
}

impl From<&str> for StringError {
    fn from(value: &str) -> Self {
        StringError { message: value.to_string() }
    }
}

impl From<String> for StringError {
    fn from(value: String) -> Self {
        StringError { message: value }
    }
}

pub(crate) type ResultAnyError<T> = Result<T, Box<dyn Error>>;
