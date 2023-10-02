use thiserror::Error;
#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum SyntaxError {
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Expected token {expected}, found {found}")]
    UnexpectedToken {
        expected: &'static str,
        found: &'static str,
    },
    #[error("Expected {0}")]
    Expected(&'static str),
}
