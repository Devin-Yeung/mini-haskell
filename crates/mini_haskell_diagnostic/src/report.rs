use crate::span::Span;

pub struct Label {
    pub span: Span,
    pub hint: String,
}

pub struct Report {
    pub message: String,
    pub offset: usize,
    pub labels: Vec<Label>,
}

pub struct ReportBuilder {
    pub message: Option<String>,
    pub offset: Option<usize>,
    pub labels: Vec<Label>,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self {
            message: None,
            offset: None,
            labels: Vec::new(),
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn label(mut self, span: Span, hint: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            hint: hint.into(),
        });
        self
    }

    pub fn finish(self) -> Report {
        Report {
            message: self.message.unwrap(),
            offset: self.offset.unwrap(),
            labels: self.labels,
        }
    }
}
