use crate::report::Report;

pub trait Diagnostic {
    fn diagnose(&mut self, ctx: &mut DiagnosticContext);
}

pub struct DiagnosticContext {
    pub reports: Vec<Report>,
}

impl DiagnosticContext {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
        }
    }

    pub fn report(&mut self, report: impl Into<Report>) {
        self.reports.push(report.into());
    }
}
