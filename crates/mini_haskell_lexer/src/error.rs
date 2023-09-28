use mini_haskell_diagnostic::diagnostic::Diagnostic;
use mini_haskell_diagnostic::span::Span;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum LexingError {
    #[error("Unexpected token")]
    UnexpectedToken(Span),
}

impl Diagnostic for LexingError {
    fn span(&self) -> Span {
        match self {
            LexingError::UnexpectedToken(span) => span.clone(),
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }
}
