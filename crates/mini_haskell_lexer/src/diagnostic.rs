#[cfg(test)]
mod test {
    use crate::error::LexingError;
    use crate::lexer::Token;
    use mini_haskell_diagnostic::diagnostic::diagnostic;
    use testsuite::unittest;

    unittest!(invalid_token, |path, _| {
        let result = diagnostic(path, |src| {
            Token::tokens(src)
                .into_iter()
                .filter(|tok| tok.is_err())
                .map(|tok| tok.err().unwrap())
                .collect::<Vec<LexingError>>()
        });

        insta::assert_snapshot!(result);
    });
}
