use crate::diagnostic::DiagnosticContext;
use ariadne::{ReportBuilder, Source};
use std::io::Cursor;
use std::ops::Range;

pub struct Reporter<'a> {
    // name of the source file
    // name: &'a str,
    // source code
    source: &'a str,
    // reporter config
    config: ReporterConfig,
    // diagnostic context
    context: DiagnosticContext,
}

impl<'a> Reporter<'a> {
    pub fn stringify(self) -> Vec<String> {
        self.context
            .reports
            .into_iter()
            .map(|report| {
                let mut builder: ReportBuilder<Range<usize>> =
                    ariadne::Report::build(ariadne::ReportKind::Error, (), report.offset)
                        .with_message(report.message)
                        .with_config(ariadne::Config::default().with_color(self.config.color));

                for label in report.labels {
                    builder = builder
                        .with_label(label.span.into())
                        .with_message(label.hint);
                }
                let report = builder.finish();

                let mut cursor = Cursor::new(Vec::<u8>::new());
                report
                    .write(ariadne::Source::from(self.source), &mut cursor)
                    .unwrap();
                String::from_utf8(cursor.into_inner()).unwrap()
            })
            .collect::<Vec<_>>()
    }
}

pub struct ReporterBuilder<'a> {
    source: Option<&'a str>,
    config: Option<ReporterConfig>,
    context: Option<DiagnosticContext>,
}

impl<'a> ReporterBuilder<'a> {
    pub fn new() -> Self {
        Self {
            source: None,
            config: None,
            context: None,
        }
    }

    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }

    pub fn context(mut self, context: DiagnosticContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn config(mut self, config: ReporterConfig) -> Self {
        self.config = Some(config);
        self
    }
}

pub struct ReporterConfig {
    color: bool,
}
