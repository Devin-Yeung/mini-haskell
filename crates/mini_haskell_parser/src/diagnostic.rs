#[cfg(test)]
mod test {
    use crate::parser::Parser;
    use mini_haskell_diagnostic::diagnostic::diagnostic;
    use testsuite::unittest;

    unittest!(unexpected_token, |path, _| {
        let result = diagnostic(path, |src| {
            let (_, errors) = Parser::parse(src);
            errors
        });

        insta::assert_snapshot!(result);
    });
}
