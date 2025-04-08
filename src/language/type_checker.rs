use super::typed::*;

pub fn type_check(expr: Expr, mut env: TEnv) -> Type {
    match expr {
        Expr::Num(_) => Type::Num,
        Expr::Add(left, right)
        | Expr::Sub(left, right)
        | Expr::Mul(left, right)
        | Expr::Div(left, right) => {
            if type_check(*left, env.clone()) == Type::Num && type_check(*right, env) == Type::Num {
                Type::Num
            } else {
                panic!("incompatible types")
            }
        }
        Expr::Val(name, typ, expr, body) => {
            if type_check(*expr, env.clone()) == typ {
                type_check(*body, {
                    env.push((name, typ));
                    env
                })
            } else {
                panic!("incompatible types")
            }
        }
        Expr::Id(ident) => env
            .iter()
            .find(|(x, _)| *x == ident)
            .expect("free identifier error")
            .1
            .clone(),
        Expr::Fun(name, typ, body) => Type::Fun(
            Box::new(typ.clone()),
            Box::new(type_check(*body, {
                env.push((name, typ));
                env
            })),
        ),
        Expr::App(func, arg) => match type_check(*func, env.clone()) {
            Type::Fun(left, right) => {
                if *left == type_check(*arg, env) {
                    *right
                } else {
                    panic!("incompatible type")
                }
            }
            _ => panic!("incompatible type"),
        },
    }
}

