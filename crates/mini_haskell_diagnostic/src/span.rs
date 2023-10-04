use ariadne::Label;
use miette::SourceSpan;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<logos::Span> for Span {
    fn from(value: logos::Span) -> Self {
        Span {
            start: value.start,
            end: value.end,
        }
    }
}

impl From<Span> for ariadne::Label {
    fn from(val: Span) -> Self {
        Label::new(val.start..val.end)
    }
}

impl Into<miette::SourceSpan> for Span {
    fn into(self) -> SourceSpan {
        (self.start, self.end - self.start).into()
    }
}
