use miette::{Error, GraphicalReportHandler, GraphicalTheme, NamedSource};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct DiagnosticTuple {
    path: PathBuf,
    errors: Vec<Error>,
}

impl<E: Into<Error>, T: IntoIterator<Item = E>, P: AsRef<Path>> From<(P, T)> for DiagnosticTuple {
    fn from(val: (P, T)) -> Self {
        let mut builder = DiagnosticTupleBuilder::new(val.0);
        builder.diagnoses(val.1);
        builder.build()
    }
}

pub struct DiagnosticTupleBuilder {
    path: PathBuf,
    errors: Vec<Error>,
}

impl DiagnosticTupleBuilder {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            errors: Vec::new(),
        }
    }

    pub fn diagnose(&mut self, err: impl Into<Error>) -> &mut Self {
        self.errors.push(err.into());
        self
    }

    pub fn diagnoses(&mut self, errs: impl IntoIterator<Item = impl Into<Error>>) -> &mut Self {
        self.errors.extend(errs.into_iter().map(|err| err.into()));
        self
    }

    pub fn build(self) -> DiagnosticTuple {
        let src = fs::read_to_string(&self.path).unwrap();
        let errors = Self::wrap_diagnostics(&self.path, &src, self.errors);
        DiagnosticTuple {
            path: self.path,
            errors,
        }
    }

    pub fn wrap_diagnostics(
        path: &Path,
        source_text: &str,
        diagnostics: Vec<impl Into<Error>>,
    ) -> Vec<Error> {
        let source = Arc::new(NamedSource::new(
            path.to_string_lossy(),
            source_text.to_owned(),
        ));

        diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.into().with_source_code(Arc::clone(&source)))
            .collect()
    }
}

pub struct Reporter {
    handler: GraphicalReportHandler,
    diagnostics: Vec<DiagnosticTuple>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            handler: GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor()),
            diagnostics: Vec::new(),
        }
    }

    pub fn report(&mut self, err: DiagnosticTuple) {
        self.diagnostics.push(err);
    }

    pub fn string(&self) -> String {
        let mut err = String::new();
        for diagnostic in &self.diagnostics {
            for error in &diagnostic.errors {
                self.handler
                    .render_report(&mut err, error.as_ref())
                    .unwrap();
                err.write_str("\n").unwrap();
            }
        }
        err
    }
}

#[cfg(test)]
mod test {
    use crate::reporter::{DiagnosticTuple, Reporter};
    use crate::span::Span;
    use miette::Diagnostic;
    use testsuite::unittest;
    use thiserror::Error;

    #[derive(Diagnostic, Error, Debug, Clone)]
    pub enum Foo {
        #[error("This is an example error")]
        #[diagnostic(help("This is an example help msg"))]
        Bar(#[label("Remove this space")] Span),
    }

    unittest!(simple_err, |path, _| {
        let err = Foo::Bar(Span { start: 5, end: 5 });

        let errors = vec![err.clone()];
        let diagnostic: DiagnosticTuple = (path, errors).into();

        let mut reporter = Reporter::new();
        reporter.report(diagnostic);

        let result = reporter.string();
        insta::assert_snapshot!(result);
    });
}
