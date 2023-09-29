use mini_haskell_diagnostic::report::{Report, ReportBuilder};
use mini_haskell_diagnostic::span::Span;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum LexingError {
    #[error("Unexpected token")]
    UnexpectedToken(Span),
}

impl From<LexingError> for Report {
    fn from(val: LexingError) -> Self {
        match &val {
            LexingError::UnexpectedToken(span) => ReportBuilder::new()
                .message(val.to_string())
                .offset(span.start)
                .label(span.clone(), val.to_string())
                .finish(),
        }
    }
}
