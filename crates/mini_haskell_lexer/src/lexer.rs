use crate::error::LexingError;
use logos::{Logos, Source};
use mini_haskell_diagnostic::span::Span;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum TokenTy {
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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ty: TokenTy,
    pub span: Span,
}

impl Token {
    pub fn new(ty: TokenTy, span: Span) -> Self {
        Token { ty, span }
    }
}

impl Token {
    pub fn tokens<S: AsRef<str>>(source: &S) -> Vec<Result<Token, LexingError>> {
        let lexer = TokenTy::lexer(source.as_ref());
        lexer
            .spanned()
            .map(|(token, span)| match token {
                Ok(ty) => Ok(Token::new(ty, span.into())),
                Err(_) => Err(LexingError::UnexpectedToken(span.into())),
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use testsuite::unittest;

    unittest!(all_tokens, |src| {
        let tokens = Token::tokens(&src);
        insta::assert_debug_snapshot!(tokens);
    });

    unittest!(invalid_token, |src| {
        let tokens = Token::tokens(&src);
        insta::assert_debug_snapshot!(tokens);
    });
}
