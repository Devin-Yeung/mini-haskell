use crate::reporter::{DiagnosticTuple, Reporter};
use miette::Error;
use std::fs;
use std::path::Path;

pub fn diagnostic<P, T, I, F>(path: P, f: F) -> String
where
    P: AsRef<Path>,
    T: Into<Error>,
    I: IntoIterator<Item = T>,
    F: Fn(&str) -> I,
{
    let src = fs::read_to_string(path.as_ref()).unwrap();
    let diagnostic: DiagnosticTuple = (path, f(&src)).into();

    let mut reporter = Reporter::new();
    reporter.report(diagnostic);

    reporter.string()
}
