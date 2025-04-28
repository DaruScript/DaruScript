use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Num(isize),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Val(String, Box<Expr>, Box<Expr>),
    Id(String),
    // First-class functions
    Fun(String, Box<Expr>),
    // Apply first-class function
    App(Box<Expr>, Box<Expr>),
}

pub type Env = Vec<(String, Value)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(isize),
    Closure(String, Expr, Env),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "Num({})", n),
            Expr::Add(left, right) => write!(f, "Add({}, {})", left.to_string(), right.to_string()),
            Expr::Sub(left, right) => write!(f, "Sub({}, {})", left.to_string(), right.to_string()),
            Expr::Val(ident, expr, body) => write!(
                f,
                "Val(\"{}\", {}, {})",
                ident.to_string(),
                expr.to_string(),
                body.to_string()
            ),
            Expr::Id(ident) => write!(f, "Id(\"{}\")", ident.to_string()),
            Expr::Fun(param, body) => {
                write!(f, "Fun(\"{}\", {})", param.to_string(), body.to_string())
            }
            Expr::App(func, arg) => write!(f, "App({}, {})", func.to_string(), arg.to_string()),
            Expr::Mul(left, right) => write!(f, "Mul({}, {})", left.to_string(), right.to_string()),
            Expr::Div(left, right) => write!(f, "Div({}, {})", left.to_string(), right.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use crate::daruscript_v1::gen_ast as gen_ast_typed;
    
    fn gen_ast(source: &str) -> super::Expr {
        gen_ast_typed(source).type_erase()
    }

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
            Add(
                Box::new(Mul(Box::new(Num(6)), Box::new(Num(3)))),
                Box::new(Num(2))
            )
        );
        assert_eq!(
            gen_ast("10/(5-3)"),
            Div(
                Box::new(Num(10)),
                Box::new(Sub(Box::new(Num(5)), Box::new(Num(3))))
            )
        );
    }

    #[test]
    fn first_class_functions() {
        assert_eq!(
            gen_ast("{x => 100}"),
            Fun(String::from("x"), Box::new(Num(100))),
        );
        assert_eq!(
            gen_ast("{x => x}({x => 100})(10)").to_string(),
            r#"App(App(Fun("x", Id("x")), Fun("x", Num(100))), Num(10))"#,
        );
    }
}
