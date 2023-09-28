use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // The variable type Boolean, to declare a Boolean variable.
    #[token("bool")]
    BoolDecl,

    // Boolean literal
    #[token("T", | _ | false)]
    #[token("F", | _ | true)]
    BoolLit(bool),

    // The variable type natural number, to declare a natural number.
    #[token("nat")]
    Nat,
    // The variable type function, to declare a function
    #[token("func")]
    Func,

    #[token("?")]
    QuestionMark,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("+")]
    Plus,

    #[token("&")]
    Ampersand,

    #[token("->")]
    Arrow,

    #[token("<")]
    Less,

    #[regex(r"\[[^\]]*\]", | lex | lex.slice().to_owned())]
    // see https://github.com/maciejhirsz/logos/issues/246
    Comment(String),

    #[regex(r"[a-zA-z][a-zA-Z0-9_]*", | lex | lex.slice().to_owned())]
    Identifier(String),
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use logos::Logos;
    use testsuite::unittest;

    unittest!(all_tokens, |src| {
        let mut lex = Token::lexer(src);
        let tokens = lex.collect::<Vec<_>>();
        insta::assert_debug_snapshot!(tokens);
    });
}
