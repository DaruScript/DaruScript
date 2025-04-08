use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Num,
    Fun(Box<Type>, Box<Type>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Num(isize),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Val(String, Type, Box<Expr>, Box<Expr>),
    Id(String),
    // First-class functions
    Fun(String, Type, Box<Expr>),
    // Apply first-class function
    App(Box<Expr>, Box<Expr>),
}

pub type TEnv = Vec<(String, Type)>;

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use super::Type;
    use crate::language::type_checker::type_check;

    #[test]
    fn addition_type_check() {
        assert_eq!(
            type_check(
                Add(
                    Box::new(Add(Box::new(Num(1)), Box::new(Num(2)))),
                    Box::new(Num(3))
                ),
                vec![]
            ),
            Type::Num
        );
        assert_eq!(
            type_check(
                Val(
                    String::from("x"),
                    Type::Num,
                    Box::new(Num(6)),
                    Box::new(Add(Box::new(Num(1)), Box::new(Id(String::from("x")))))
                ),
                vec![]
            ),
            Type::Num
        );
    }

    #[test]
    #[should_panic]
    fn addition_type_check_invalid() {
        assert_eq!(
            type_check(
                Add(
                    Box::new(Add(Box::new(Num(1)), Box::new(Num(2)))),
                    Box::new(Fun(String::from("x"), Type::Num, Box::new(Num(100))))
                ),
                vec![]
            ),
            Type::Num
        );
        assert_eq!(
            type_check(
                Val(
                    String::from("x"),
                    Type::Fun(Box::new(Type::Num), Box::new(Type::Num)),
                    Box::new(Num(6)),
                    Box::new(Add(Box::new(Num(1)), Box::new(Id(String::from("x")))))
                ),
                vec![]
            ),
            Type::Num
        );
    }
}
