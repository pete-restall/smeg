use std::error::Error;

mod string_error;
pub type StringError = string_error::StringError;

pub type ResultAnyError<T> = Result<T, Box<dyn Error>>;
