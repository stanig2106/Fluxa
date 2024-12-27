// src/error.rs

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String),
    SyntaxError(String),
    UnsupportedMimeType(String),
    IoError(std::io::Error),
    Other(String),
}

impl From<std::io::Error> for ParserError {
    fn from(err: std::io::Error) -> Self {
        ParserError::IoError(err)
    }
}
