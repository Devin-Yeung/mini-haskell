use miette::Diagnostic;
use mini_haskell_diagnostic::span::Span;
use thiserror::Error;

#[derive(Diagnostic, Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum SyntaxError {
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("SyntaxError: Unexpected token")]
    UnexpectedToken {
        #[label = "Expected {expected}, found {found}"]
        span: Span,
        expected: &'static str,
        found: &'static str,
    },
    #[error("SyntaxError: Unexpected AST")]
    Expected {
        #[label = "Expected {expected}"]
        span: Span,
        expected: &'static str,
    },
}
