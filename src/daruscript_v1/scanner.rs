use std::fmt;

fn error(line: usize, message: String) {
    report(line, "", message);
}

fn report(line: usize, err_where: &str, message: String) {
    eprintln!("[line {line} ] Error{err_where}: {message}");
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,

    // One or two character tokens
    Bang,
    BangEquals,
    Equals,
    EqualsEquals,
    Greater,
    GreaterOrEquals,
    Less,
    LessOrEquals,

    // Literals
    Ident,
    Literal,

    // Keywords
    And,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    True,
    Var,
    Val,
    While,

    // LineComment,
    // BlockComment { terminated: bool },
    Eof,
}

use TokenKind::*;

#[derive(Debug)]
pub enum Literal {
    Str(String),
    Num(isize),
    Float(f32),
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Hex formatting so that each u32 can be formatted independently.
        write!(f, "{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // At the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            kind: Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        self.tokens
    }

    // todo! return Error https://craftinginterpreters.com/scanning.html#lexical-errors
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // Single-character tokens
            '(' => self.add_token(LeftParen, None),
            ')' => self.add_token(RightParen, None),
            '{' => self.add_token(LeftBrace, None),
            '}' => self.add_token(RightBrace, None),
            ',' => self.add_token(Comma, None),
            '.' => self.add_token(Dot, None),
            '-' => self.add_token(Minus, None),
            '+' => self.add_token(Plus, None),
            ';' => self.add_token(Semicolon, None),
            '*' => self.add_token(Star, None),
            '!' => {
                if self.take_if('=') {
                    self.add_token(BangEquals, None);
                } else {
                    self.add_token(Bang, None);
                }
            }
            '=' => {
                if self.take_if('=') {
                    self.add_token(EqualsEquals, None);
                } else {
                    self.add_token(Equals, None);
                }
            }
            '<' => {
                if self.take_if('=') {
                    self.add_token(LessOrEquals, None);
                } else {
                    self.add_token(Less, None);
                }
            }
            '>' => {
                if self.take_if('=') {
                    self.add_token(GreaterOrEquals, None);
                } else {
                    self.add_token(Greater, None);
                }
            }
            // Comments
            '/' => {
                if self.take_if('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        let _ = self.advance();
                    }
                    // self.add_token(LineComment, None);
                } else if self.take_if('*') {
                    let mut depth = 1usize;
                    while !self.is_at_end() {
                        let c = self.advance();
                        match c {
                            '/' if self.take_if('*') => {
                                depth += 1;
                            }
                            '*' if self.take_if('/') => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            '\n' => {
                                self.line += 1;
                            }
                            _ => (),
                        }
                    }
                    // self.add_token(
                    //     BlockComment {
                    //         terminated: depth == 0,
                    //     },
                    //     None,
                    // );
                } else {
                    self.add_token(Slash, None);
                }
            }
            // Whitespace
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            // Literals
            '"' => self.string(),
            n if n.is_ascii_digit() => {
                self.number();
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                self.identifier();
            }
            // Unsupported character
            _ => error(self.line, "Unexpected character.".to_string()),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let kind = match text {
            "and" => And,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "true" => True,
            "var" => Var,
            "while" => While,
            "val" => Val,
            // Not a reserved keyword
            _ => Ident,
        };

        self.add_token(kind, None);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let literal = if self.peek() == '.' && self.peek_second().is_ascii_digit() {
            // If float.
            // Consume '.'
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }

            Literal::Float(
                self.source[self.start..self.current]
                    .parse()
                    .expect("expected a float"),
            )
        } else {
            // Not a float
            Literal::Num(
                self.source[self.start..self.current]
                    .parse()
                    .expect("expected an integer"),
            )
        };

        self.add_token(Literal, Some(literal));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.".to_string());
            return;
        }

        // closing '"'.
        self.advance();

        // Trim quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        let literal = Literal::Str(value);
        self.add_token(Literal, Some(literal));
    }

    /// Takes the current character and returns true if it is not EOF and is `c`, else returns false.
    fn take_if(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.current_char() != c {
            return false;
        };

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current_char()
        }
    }

    fn peek_second(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.nth_char(1)
        }
    }

    fn advance(&mut self) -> char {
        let c = self.current_char();
        self.current += 1;
        c
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>) {
        // fishy
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            kind,
            lexeme,
            literal,
            line: self.line,
        });
    }

    fn current_char(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }

    fn nth_char(&self, n: usize) -> char {
        self.source.as_bytes()[self.current + n] as char
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
