use crate::diagnostic::DiagnosticContext;
use crate::report::Report;
use crate::reporter::{ReporterBuilder, ReporterConfig};

pub struct Tester<'a> {
    builder: ReporterBuilder<'a>,
}

impl<'a> Tester<'a> {
    pub fn new() -> Tester<'a> {
        let builder = ReporterBuilder::new().config(ReporterConfig { color: false });

        Tester { builder }
    }

    pub fn diagnose(self, source: &'a str, report: Vec<impl Into<Report>>) -> Vec<String> {
        let reporter = self
            .builder
            .source(source)
            .context(DiagnosticContext {
                reports: report.into_iter().map(|report| report.into()).collect(),
            })
            .build();
        reporter.stringify()
    }
}

#[cfg(test)]
mod tests {
    use crate::report::{Label, Report};
    use crate::span::Span;
    use crate::tester::Tester;
    use testsuite::unittest;

    unittest!(simple_diagnose, |src| {
        let reports = vec![Report {
            message: String::from("This is an example error"),
            offset: 0,
            labels: vec![Label {
                span: Span { start: 5, end: 6 },
                hint: "This is a hint".to_string(),
            }],
        }];
        let tester = Tester::new();
        let diagnostics = tester.diagnose(src, reports).join("\n");
        insta::assert_snapshot!(diagnostics);
    });
}
