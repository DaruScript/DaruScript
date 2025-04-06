use super::ast::Expr;
use super::scanner::{Literal, Token, TokenKind};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    pub tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        if self
            .tokens
            .peek()
            .is_some_and(|tok| tok.kind == TokenKind::Eof)
            || self.tokens.peek().is_none()
        {
            None
        } else {
            let expr = Some(self.expression());
            if self.advance().expect("expected EOF").kind != TokenKind::Eof {
                panic!("expected EOF")
            }
            expr
        }
    }

    fn expression(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let expr = self.parse_comparison();

        while let Some(kind) = [TokenKind::BangEquals, TokenKind::EqualsEquals]
            .iter()
            .find(|x| self.check(x))
        {
            self.advance().expect("expected a token");
            let _right = self.parse_comparison();
            expr = match kind {
                TokenKind::Plus => todo!(),
                TokenKind::Minus => todo!(),
                _ => panic!("expected BangEquals or EqualsEquals"), // todo! error handling
            };
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let expr = self.parse_term();

        while let Some(kind) = [
            TokenKind::Greater,
            TokenKind::GreaterOrEquals,
            TokenKind::Less,
            TokenKind::LessOrEquals,
        ]
        .iter()
        .find(|x| self.check(x))
        {
            self.advance().expect("expected a token");
            let _right = self.parse_term();
            expr = match kind {
                TokenKind::Greater => todo!(),
                TokenKind::GreaterOrEquals => todo!(),
                TokenKind::Less => todo!(),
                TokenKind::LessOrEquals => todo!(),
                _ => panic!("expected Plus or Minus"), // todo! error handling
            };
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while let Some(kind) = [TokenKind::Plus, TokenKind::Minus]
            .iter()
            .find(|x| self.check(x))
        {
            self.advance().expect("expected a token");
            let right = self.parse_factor();
            expr = match kind {
                TokenKind::Plus => Expr::Add(Box::new(expr), Box::new(right)),
                TokenKind::Minus => Expr::Sub(Box::new(expr), Box::new(right)),
                _ => panic!("expected Plus or Minus"), // todo! error handling
            };
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let expr = self.parse_unary();

        while let Some(kind) = [TokenKind::Slash, TokenKind::Star]
            .iter()
            .find(|x| self.check(x))
        {
            self.advance().expect("expected a token");
            let _right = self.parse_unary();
            expr = match kind {
                TokenKind::Slash => todo!(),           // unsupported
                TokenKind::Star => todo!(),            // unsupported
                _ => panic!("expected Slash or Star"), // todo! error handling
            };
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        let next_token = match self.peek() {
            Some(tok) => tok,
            None => return self.parse_primary(),
        };

        match next_token.kind {
            TokenKind::Bang => {
                // unsupported
                todo!()
            }
            TokenKind::Minus => {
                self.advance().expect("expected a Minus");
                let right = self.parse_unary();
                // desugaring
                return Expr::Sub(Box::new(Expr::Num(0)), Box::new(right));
            }
            _ => (),
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        match &self.peek().expect("expected a token").kind {
            TokenKind::Literal => {
                match self
                    .advance()
                    .expect("expected a Literal")
                    .literal
                    .expect("expected a literal")
                {
                    Literal::Str(x) => Expr::Id(x),
                    Literal::Num(n) => Expr::Num(n),
                    Literal::Float(_) => {
                        // unsupported
                        todo!()
                    }
                }
            }
            TokenKind::LeftParen => {
                self.advance().expect("expected LeftParen");
                let expr = self.expression();
                self.consume(&TokenKind::RightParen, "Expected ')' after expression.");
                expr
            }

            TokenKind::LeftBrace => {
                self.advance().expect("expected LeftBrace");
                if self.check(&TokenKind::Var) { panic!("'var' is unsupported"); }

                self.consume(&TokenKind::Val, "expected 'val'");
                let ident = self.advance().expect("expected identifier");
                if ident.kind != TokenKind::Ident { panic!("not an identifier"); }
                let ident = ident.lexeme;

                self.consume(&TokenKind::Equals, "expected '='");
                let expr = self.expression();

                self.consume(&TokenKind::Semicolon, "expected ';'");
                let body  = self.expression();

                self.consume(&TokenKind::RightBrace, "Expected '}' after expression.");
                
                Expr::Val(ident, Box::new(expr), Box::new(body))
            }

            TokenKind::Ident => {
                let ident = self.advance().expect("expected Ident").lexeme;
                Expr::Id(ident)
            }

            x => {
                // todo! error handling
                panic!("unexpected token {:?}", x)
            }
        }
    }

    fn consume(&mut self, kind: &TokenKind, message: &str) -> Token {
        if self.check(kind) {
            return self.advance().expect("expected a token");
        };
        // todo! proper error handling
        panic!("{} {}", self.peek().expect("expected a token"), message);
    }

    // fn take_if(&mut self, types: &[TokenKind]) -> bool {
    //     if types.iter().any(|x| self.check(x)) {
    //         self.advance().expect("expected a token");
    //         true
    //     } else {
    //         false
    //     }
    // }

    fn check(&mut self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().expect("expected a token").kind == *kind
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().expect("expected a token").kind == TokenKind::Eof
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}
