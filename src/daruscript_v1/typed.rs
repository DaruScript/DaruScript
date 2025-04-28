use super::{type_checker::resolve, untyped::Expr as Untyped};
use core::fmt;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum Type {
    Num,
    Fun(Box<Type>, Box<Type>),
    // Type Variable
    Var { typ: Rc<RefCell<Option<Type>>> },
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Num, Type::Num) => true,
            (Type::Fun(x1, r1), Type::Fun(x2, r2)) => x1 == x2 && r1 == r2,
            (Type::Var { typ: typ1 }, Type::Var { typ: typ2 }) => Rc::ptr_eq(typ1, typ2),
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match resolve(self.clone()) {
            Type::Num => write!(f, "Num"),
            Type::Fun(l, r) => write!(f, "{} -> {}", l, r),
            Type::Var { typ } => write!(
                f,
                "Var({})",
                match typ.borrow().clone() {
                    Some(x) => x.to_string(),
                    None => "None".to_string(),
                }
            ),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "Num({})", n),
            Expr::Add(left, right) => write!(f, "Add({}, {})", left.to_string(), right.to_string()),
            Expr::Sub(left, right) => write!(f, "Sub({}, {})", left.to_string(), right.to_string()),
            Expr::Val(ident, _typ, expr, body) => write!(
                f,
                "Val(\"{}\", {}, {})",
                ident.to_string(),
                expr.to_string(),
                body.to_string()
            ),
            Expr::Id(ident) => write!(f, "Id(\"{}\")", ident.to_string()),
            Expr::Fun(param, _typ, body) => {
                write!(f, "Fun(\"{}\", {})", param.to_string(), body.to_string())
            }
            Expr::App(func, arg) => write!(f, "App({}, {})", func.to_string(), arg.to_string()),
            Expr::Mul(left, right) => write!(f, "Mul({}, {})", left.to_string(), right.to_string()),
            Expr::Div(left, right) => write!(f, "Div({}, {})", left.to_string(), right.to_string()),
        }
    }
}

impl Expr {
    pub fn type_erase(&self) -> Untyped {
        Expr::erase(self)
    }

    fn erase(expr: &Expr) -> Untyped {
        match expr {
            Expr::Num(n) => Untyped::Num(*n),
            Expr::Add(l, r) => Untyped::Add(Box::new(Expr::erase(l)), Box::new(Expr::erase(r))),
            Expr::Sub(l, r) => Untyped::Sub(Box::new(Expr::erase(l)), Box::new(Expr::erase(r))),
            Expr::Mul(l, r) => Untyped::Mul(Box::new(Expr::erase(l)), Box::new(Expr::erase(r))),
            Expr::Div(l, r) => Untyped::Div(Box::new(Expr::erase(l)), Box::new(Expr::erase(r))),
            Expr::Val(x, _, e, b) => Untyped::Val(
                x.to_string(),
                Box::new(Expr::erase(e)),
                Box::new(Expr::erase(b)),
            ),
            Expr::Id(x) => Untyped::Id(x.to_string()),
            Expr::Fun(p, _, b) => Untyped::Fun(p.to_string(), Box::new(Expr::erase(b))),
            Expr::App(f, a) => Untyped::App(Box::new(Expr::erase(f)), Box::new(Expr::erase(a))),
        }
    }
}

pub type TEnv = Vec<(String, Type)>;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Expr::*;
    use super::Type;
    use crate::daruscript_v1::type_checker::type_check;

    #[test]
    fn addition_type_check() {
        assert_eq!(
            type_check(
                Add(
                    Box::new(Add(Box::new(Num(1)), Box::new(Num(2)))),
                    Box::new(Num(3))
                ),
                vec![]
            )
            .to_string(),
            "Num".to_string()
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
            )
            .to_string(),
            "Num".to_string()
        );
    }

    #[test]
    fn type_inferred() {
        // {x => y => x}(1)
        assert_eq!(
            type_check(
                App(
                    Box::new(Fun(
                        "x".to_string(),
                        Type::Var {
                            typ: Rc::new(RefCell::new(None))
                        },
                        Box::new(Fun(
                            "y".to_string(),
                            Type::Var {
                                typ: Rc::new(RefCell::new(None))
                            },
                            Box::new(Id("x".to_string()))
                        )),
                    )),
                    Box::new(Num(1))
                ),
                vec![]
            )
            .to_string(),
            "Var(None) -> Num".to_string()
        );

        assert_eq!(
            // {x => x + 1}(2)
            type_check(
                App(
                    Box::new(Fun(
                        "x".to_string(),
                        Type::Var {
                            typ: Rc::new(RefCell::new(None))
                        },
                        Box::new(Add(Box::new(Id("x".to_string())), Box::new(Num(1))))
                    )),
                    Box::new(Num(2)),
                ),
                vec![]
            )
            .to_string(),
            "Num".to_string()
        );

        assert_eq!(
            // val x = 1; val x = x + 1; x
            type_check(
                Val(
                    "x".to_string(),
                    Type::Var {
                        typ: Rc::new(RefCell::new(None))
                    },
                    Box::new(Num(1)),
                    Box::new(Val(
                        "x".to_string(),
                        Type::Var {
                            typ: Rc::new(RefCell::new(None))
                        },
                        Box::new(Add(Box::new(Id("x".to_string())), Box::new(Num(1)))),
                        Box::new(Id("x".to_string()))
                    )),
                ),
                vec![]
            )
            .to_string(),
            "Num".to_string()
        );

        assert_eq!(
            type_check(
                Val(
                    "f".to_string(),
                    Type::Var {
                        typ: Rc::new(RefCell::new(None))
                    },
                    Box::new(Fun(
                        "x".to_string(),
                        Type::Var {
                            typ: Rc::new(RefCell::new(None))
                        },
                        Box::new(Add(Box::new(Id("x".to_string())), Box::new(Num(1))))
                    )),
                    Box::new(App(Box::new(Id("f".to_string())), Box::new(Num(3))))
                ),
                vec![]
            )
            .to_string(),
            "Num".to_string()
        );

        assert_eq!(
            // x = 6; 1 + x
            type_check(
                Val(
                    String::from("x"),
                    Type::Var {
                        typ: Rc::new(RefCell::new(None))
                    },
                    Box::new(Num(6)),
                    Box::new(Add(Box::new(Num(1)), Box::new(Id(String::from("x")))))
                ),
                vec![]
            )
            .to_string(),
            "Num".to_string()
        );

        assert_eq!(
            // val x1 = x => x; val x2 = x1; val x3= x2(1); x1
            type_check(
                Val(
                    "x1".to_string(),
                    Type::Var {
                        typ: Rc::new(RefCell::new(None))
                    },
                    // expr
                    Box::new(Fun(
                        "x".to_string(),
                        Type::Var {
                            typ: Rc::new(RefCell::new(None))
                        },
                        Box::new(Id("x".to_string()))
                    )),
                    // body
                    Box::new(Val(
                        "x2".to_string(),
                        Type::Var {
                            typ: Rc::new(RefCell::new(None))
                        },
                        Box::new(Id("x1".to_string())),
                        Box::new(Val(
                            "x3".to_string(),
                            Type::Var {
                                typ: Rc::new(RefCell::new(None))
                            },
                            Box::new(App(Box::new(Id("x2".to_string())), Box::new(Num(1)))),
                            Box::new(Id("x1".to_string()))
                        ))
                    ))
                ),
                vec![]
            )
            .to_string(),
            "Num -> Num".to_string()
        );
    }

    #[test]
    #[should_panic]
    fn type_inferred_invalid() {
        type_check(
            // x => x(x)
            Fun(
                "x".to_string(),
                Type::Var {
                    typ: Rc::new(RefCell::new(None)),
                },
                Box::new(App(
                    Box::new(Id("x".to_string())),
                    Box::new(Id("x".to_string())),
                )),
            ),
            vec![],
        );

        // x = x => x; 1 + x
        type_check(
            Val(
                String::from("x"),
                Type::Var {
                    typ: Rc::new(RefCell::new(None)),
                },
                Box::new(Fun(
                    "x".to_string(),
                    Type::Num,
                    Box::new(Id("x".to_string())),
                )),
                Box::new(Add(Box::new(Num(1)), Box::new(Id(String::from("x"))))),
            ),
            vec![],
        );

        type_check(App(Box::new(Num(1)), Box::new(Num(2))), vec![]);
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
            )
            .to_string(),
            "Num".to_string()
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
            )
            .to_string(),
            "Num".to_string()
        );
    }
}
