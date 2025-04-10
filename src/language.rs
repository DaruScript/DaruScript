mod untyped;
mod interp;
mod parser;
mod scanner;
mod typed;
mod type_checker;

use scanner::Scanner;
use untyped::{Expr, Value};
use parser::Parser;

pub fn gen_ast(source: &str) -> Expr {
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan();
    let mut parser = Parser::new(tokens);
    parser.parse().expect("expected expr")
}

pub fn run(source: &str) -> Value {
    interp::interp(gen_ast(source), vec![])
}
