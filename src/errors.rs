extern crate pest;

use std::error;
use std::fmt;

use grammar::*;


#[derive(Debug)]
pub enum ParseError<'i> {
    EmptyInput,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Pest(pest::Error<'i, Rule>),
}

impl<'i> fmt::Display for ParseError<'i> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::EmptyInput =>
                write!(f, "please provide valid input for parsing"),
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            ParseError::Pest(ref e) => e.fmt(f),
        }
    }
}

impl<'i> error::Error for ParseError<'i> {
    fn description(&self) -> &str {
        match *self {
            ParseError::EmptyInput => "empty input is not allowed",
            // This already impls `Error`, so defer to its own implementation.
            ParseError::Pest(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParseError::EmptyInput => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            ParseError::Pest(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `pest::Error` to `ParseError`.
// This will be automatically called by `?` if a `pest::Error`
// needs to be converted into a `ParseError`.
impl<'i> From<pest::Error<'i, Rule>> for ParseError<'i> {
    fn from(err: pest::Error<Rule>) -> ParseError {
        ParseError::Pest(err)
    }
}
