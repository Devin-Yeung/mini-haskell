use crate::ast::{Expr, Literal};
use crate::error::SyntaxError;
use mini_haskell_lexer::lexer::{TokenTy, Tokenizer};
use std::iter::Peekable;

pub struct Parser<'src> {
    tokenizer: Peekable<Tokenizer<'src>>,
}

impl<'src> Parser<'src> {
    pub fn new<S: AsRef<str> + ?Sized>(src: &'src S) -> Self {
        Self {
            tokenizer: Tokenizer::new(src).peekable(),
        }
    }

    fn consume(&mut self, ty: TokenTy) -> Result<TokenTy, SyntaxError> {
        dbg!(self.tokenizer.peek());
        match self.peek_type()? {
            found if found == ty => Ok(self.advance()?),
            found => Err(SyntaxError::UnexpectedToken {
                expected: ty.name(),
                found: found.name(),
            }),
        }
    }

    fn consume_if(&mut self, ty: TokenTy) -> bool {
        if self.peek_type() == Ok(ty) {
            self.advance().unwrap();
            return true;
        }
        false
    }

    fn peek_type(&mut self) -> Result<TokenTy, SyntaxError> {
        dbg!(self.tokenizer.peek());
        match self.tokenizer.peek() {
            None => Ok(TokenTy::EOF),
            Some(Ok(token)) => Ok(token.ty.clone()),
            Some(Err(_)) => self.advance(),
        }
    }

    fn advance(&mut self) -> Result<TokenTy, SyntaxError> {
        self.tokenizer
            .next()
            .map_or(Err(SyntaxError::UnexpectedEOF), |inner| {
                match inner {
                    Ok(tok) => Ok(tok.ty),
                    Err(_) => {
                        // lexing error is ignored and skip to next valid token
                        loop {
                            match self.tokenizer.peek() {
                                None => return Err(SyntaxError::UnexpectedEOF),
                                Some(Ok(tok)) => return Ok(tok.ty.clone()),
                                Some(Err(_)) => {
                                    self.tokenizer.next();
                                }
                            }
                        }
                    }
                }
            })
    }

    /// parse primary expression according to following rules:
    ///
    /// ```text
    /// primary  → NAT | "T" | "F" | IDENTIFIER ;
    /// ```
    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        match self.peek_type()? {
            TokenTy::BoolLit(b) => Ok(Expr::Literal(Literal::Bool(b))),
            TokenTy::NatLit(n) => Ok(Expr::Literal(Literal::NatureNum(n))),
            TokenTy::Identifier(_) => todo!(),
            _ => Err(SyntaxError::Expected("expression")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::parser::SyntaxError;
    use crate::parser::TokenTy;
    use mini_haskell_lexer::lexer::TokenTy::NatLit;
    use testsuite::unittest;

    unittest!(advance, |src| {
        let mut parser = Parser::new(src);
        let mut result = Vec::<Result<TokenTy, SyntaxError>>::new();
        loop {
            match parser.advance() {
                Err(SyntaxError::UnexpectedEOF) => break,
                Ok(tok) => result.push(Ok(tok)),
                Err(err) => result.push(Err(err)),
            }
        }
        insta::assert_debug_snapshot!(result);
    });

    unittest!(consume, |src| {
        let mut parser = Parser::new(src);
        let result = vec![
            parser.peek_type(),
            parser.consume(NatLit(1)),
            parser.peek_type(),
            parser.consume(NatLit(2)),
            parser.peek_type(),
        ];
        insta::assert_debug_snapshot!(result);
    });

    unittest!(primary, |src| {
        let asts = src
            .split('\n')
            .map(|line| Parser::new(line).primary())
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(asts);
    });
}
