use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct StringError {
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use smeg_testing_host_utils::strings::utf8;
    use super::*;

    #[test]
    fn message__get_after_from_string__expect_same_string() {
        let message = utf8::any_nonempty();
        let error = StringError::from(message.as_str());
        expect!(error.message).to_equal(message);
    }

    #[test]
    fn message__get_after_string_into__expect_same_string() {
        let message = utf8::any_nonempty();
        let error: StringError = message.as_str().into();
        expect!(error.message).to_equal(message);
    }

    #[derive(Debug)]
    struct StubError {
        string_representation: String
    }

    impl StubError {
        fn new(string_representation: String) -> Self {
            StubError { string_representation }
        }
    }

    impl Error for StubError {
    }

    impl Display for StubError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "The stubbed error is...{}", &self.string_representation)
        }
    }

    #[test]
    fn message__get_after_from_boxed_error__expect_error_to_string() {
        let any_error = StubError::new(utf8::any_nonempty());
        let any_error_to_string = any_error.to_string();
        let error = StringError::from(Box::<dyn Error>::from(any_error));
        expect!(error.message).to_equal(any_error_to_string);
    }

    #[test]
    fn message__get_after_boxed_error_into__expect_error_to_string() {
        let any_error: Box<dyn Error> = Box::from(StubError::new(utf8::any_nonempty()));
        let any_error_to_string = any_error.to_string();
        let error: StringError = any_error.into();
        expect!(error.message).to_equal(any_error_to_string);
    }
}
