use mini_haskell_diagnostic::span::Span;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum LexingError {
    #[error("Unexpected token")]
    UnexpectedToken(Span),
}
