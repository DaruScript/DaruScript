#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Num(isize),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Val(String, Box<Expr>, Box<Expr>),
    Id(String),
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use crate::language::gen_ast;

    #[test]
    fn addition_ast() {
        assert_eq!(gen_ast("1+2"), Add(Box::new(Num(1)), Box::new(Num(2))));
        assert_eq!(
            gen_ast("1+2+3"),
            Add(
                Box::new(Add(Box::new(Num(1)), Box::new(Num(2)))),
                Box::new(Num(3))
            )
        );
        assert_eq!(
            gen_ast("1+(2+3)"),
            Add(
                Box::new(Num(1)),
                Box::new(Add(Box::new(Num(2)), Box::new(Num(3))))
            )
        );
        assert_eq!(gen_ast("5-3"), Sub(Box::new(Num(5)), Box::new(Num(3))));
        assert_eq!(
            gen_ast("{val x = 6; 1 + x}"),
            Val(
                String::from("x"),
                Box::new(Num(6)),
                Box::new(Add(Box::new(Num(1)), Box::new(Id(String::from("x")))))
            )
        );
    }
}
