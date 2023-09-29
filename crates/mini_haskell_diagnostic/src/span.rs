use ariadne::Label;

#[derive(Debug, PartialEq, Clone)]
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
