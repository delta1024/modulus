use std::{
    error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum LexerError {
    UnknownToken(char),
    IncompleteToken(Box<dyn error::Error>),
}
impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownToken(c) => write!(f, "Error: unknown token: {c}"),
            Self::IncompleteToken(err) => write!(f, "Error: {err}"),
        }
    }
}
impl error::Error for LexerError {}

#[derive(Debug)]
pub struct ParseError(pub String);
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl error::Error for ParseError {}
#[derive(Debug)]
pub struct EvalError;
impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not eval")
    }
}
impl error::Error for EvalError {}
