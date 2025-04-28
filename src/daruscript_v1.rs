mod interp;
mod parser;
mod scanner;
mod type_checker;
mod typed;
mod untyped;

use parser::Parser;
use scanner::Scanner;
use typed::Expr;
use typed::Type;
use untyped::Value;

pub fn gen_ast(source: &str) -> Expr {
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan();
    let mut parser = Parser::new(tokens);
    parser.parse().expect("expected expr")
}

pub fn type_check(source: &str) -> Type {
    type_checker::type_check(gen_ast(source), vec![])
}

pub fn run(source: &str) -> Value {
    let ast = gen_ast(source);
    let ast_untyped = ast.type_erase();
    type_checker::type_check(ast, vec![]);
    interp::interp(ast_untyped, vec![])
}
