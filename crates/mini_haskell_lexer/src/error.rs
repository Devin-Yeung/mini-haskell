use miette::Diagnostic;
use mini_haskell_diagnostic::span::Span;
use thiserror::Error;

#[derive(Diagnostic, Error, Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum LexingError {
    #[error("Unexpected token")]
    #[diagnostic()]
    UnexpectedToken(#[label = "Invalid token"] Span),
}
