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

impl Into<ariadne::Label> for Span {
    fn into(self) -> Label {
        Label::new(self.start..self.end)
    }
}
