use std::error::Error;
use std::fmt::{self, Debug, Display};

macro_rules! input_error {
    ( $x:expr  ) => {
        Err(Box::new(InputError::new(String::from($x))))
    };
}

pub struct InputError(String);

impl InputError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Error for InputError {}
