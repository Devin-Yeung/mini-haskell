use crate::ast::{BinaryExpr, BinaryOp, Expr, ExprKind, Literal};
use crate::error::SyntaxError;
use mini_haskell_lexer::lexer::{Token, TokenTy, Tokenizer};
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

    fn consume(&mut self, ty: TokenTy) -> Result<Token, SyntaxError> {
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
        loop {
            match self.tokenizer.peek() {
                None => return Ok(TokenTy::EOF),
                Some(Ok(token)) => return Ok(token.ty.clone()),
                Some(Err(_)) => {
                    self.tokenizer.next();
                }
            }
        }
    }

    fn advance(&mut self) -> Result<Token, SyntaxError> {
        loop {
            match self.tokenizer.peek() {
                None => return Err(SyntaxError::UnexpectedEOF),
                Some(Ok(_)) => return Ok(self.tokenizer.next().unwrap().unwrap()),
                Some(Err(_)) => {
                    self.tokenizer.next();
                }
            }
        }
    }

    /// parse expression according to following rules:
    /// ```text
    /// expression  → conditional ;
    /// ```
    pub fn expression(&mut self) -> Result<Expr, SyntaxError> {
        todo!()
    }

    /// parse conditional expression according to following rules:
    /// ```text
    /// conditional  → logical ? logical :logical
    ///              | logical ;
    /// ```
    pub fn conditional(&mut self) -> Result<Expr, SyntaxError> {
        todo!()
    }

    /// parse logical expression according to following rules:
    /// ```text
    /// logical  → comparison ( "&" comparison )*
    /// ```
    pub fn logical(&mut self) -> Result<Expr, SyntaxError> {
        todo!()
    }

    /// parse comparison expression according to following rules:
    /// ```text
    /// comparison  → addition ( ( "<" | "=" ) addition )*
    ///             | addition ;
    /// ```
    pub fn comparison(&mut self) -> Result<Expr, SyntaxError> {
        todo!()
    }

    /// parse comparison expression according to following rules:
    /// ```text
    /// addition  → primary ("+" primary)*
    /// ```
    pub fn addition(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.primary()?;
        loop {
            let token = match self.peek_type()? {
                TokenTy::Plus => self.advance()?,
                _ => break,
            };

            let rhs = self.primary()?;
            expr = Expr {
                kind: ExprKind::BinaryExpr(BinaryExpr {
                    lhs: Box::new(expr),
                    op: BinaryOp::Plus,
                    rhs: Box::new(rhs),
                }),
                span: token.span,
            }
        }
        Ok(expr)
    }

    /// parse primary expression according to following rules:
    ///
    /// ```text
    /// primary  → NAT | "T" | "F" | IDENTIFIER ;
    /// ```
    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        match self.peek_type()? {
            TokenTy::BoolLit(b) => Ok(Expr::new(
                ExprKind::Literal(Literal::Bool(b)),
                self.advance()?.span,
            )),
            TokenTy::NatLit(n) => Ok(Expr::new(
                ExprKind::Literal(Literal::NatureNum(n)),
                self.advance()?.span,
            )),
            TokenTy::Identifier(_) => todo!(),
            _ => Err(SyntaxError::Expected("expression")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::parser::SyntaxError;
    use crate::parser::Token;
    use mini_haskell_lexer::lexer::TokenTy::NatLit;
    use testsuite::unittest;

    unittest!(advance, |src| {
        let mut parser = Parser::new(src);
        let mut result = Vec::<Result<Token, SyntaxError>>::new();
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
        let result = vec![parser.consume(NatLit(1)), parser.consume(NatLit(2))];
        insta::assert_debug_snapshot!(result);
    });

    unittest!(primary, |src| {
        let asts = src
            .split('\n')
            .map(|line| Parser::new(line).primary())
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(asts);
    });

    unittest!(addition, |src| {
        let asts = src
            .split('\n')
            .map(|line| Parser::new(line).addition())
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(asts);

        // let mut parser = Parser::new(src);
        // let mut result = Vec::<Result<Token, SyntaxError>>::new();
        // loop {
        //     match parser.advance() {
        //         Err(SyntaxError::UnexpectedEOF) => break,
        //         Ok(tok) => result.push(Ok(tok)),
        //         Err(err) => result.push(Err(err)),
        //     }
        // }
        // insta::assert_debug_snapshot!(result);
    });
}
