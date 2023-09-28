#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    start: usize,
    end: usize,
}

impl From<logos::Span> for Span {
    fn from(value: logos::Span) -> Self {
        Span {
            start: value.start,
            end: value.end,
        }
    }
}
