#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Num(isize),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
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
        assert_eq!(gen_ast("4*2"), Mul(Box::new(Num(4)), Box::new(Num(2))));
        assert_eq!(gen_ast("8/4"), Div(Box::new(Num(8)), Box::new(Num(4))));
        assert_eq!(
            gen_ast("6*3+2"),
            Add(Box::new(Mul(Box::new(Num(6)), Box::new(Num(3)))), Box::new(Num(2)))
        );
        assert_eq!(
            gen_ast("10/(5-3)"),
            Div(Box::new(Num(10)), Box::new(Sub(Box::new(Num(5)), Box::new(Num(3)))))
        );
    }
}
