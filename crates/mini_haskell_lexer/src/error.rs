use mini_haskell_diagnostic::report::{Report, ReportBuilder};
use mini_haskell_diagnostic::span::Span;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum LexingError {
    #[error("Unexpected token")]
    UnexpectedToken(Span),
}

impl Into<Report> for LexingError {
    fn into(self) -> Report {
        match &self {
            LexingError::UnexpectedToken(span) => ReportBuilder::new()
                .message(self.to_string())
                .offset(span.start)
                .label(span.clone(), self.to_string())
                .finish(),
        }
    }
}
