use crate::span::Span;

pub trait Diagnostic {
    fn span(&self) -> Span;
    fn message(&self) -> String;
}
