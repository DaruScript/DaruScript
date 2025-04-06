mod ast;
mod interp;
mod parser;
mod scanner;

use scanner::Scanner;
use ast::Expr;
use parser::Parser;

pub fn gen_ast(source: &str) -> Expr {
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan();
    let mut parser = Parser::new(tokens);
    parser.parse().expect("expected expr")
}

pub fn run(source: &str) -> isize {
    interp::interp(gen_ast(source), vec![])
}
