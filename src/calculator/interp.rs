use core::fmt;

use super::untyped::*;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{}", n),
            Value::Closure(p, b, e) => write!(f, "Closure({}, {}, {:?})", p, b.to_string(), e),
        }
    }
}

pub fn interp(expr: Expr, mut env: Env) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(n),
        Expr::Add(l, r) => {
            if let (Value::Num(l), Value::Num(r)) = (interp(*l, env.clone()), interp(*r, env)) {
                Value::Num(l + r)
            } else {
                panic!("incompatible types");
            }
        }
        Expr::Sub(l, r) => {
            if let (Value::Num(l), Value::Num(r)) = (interp(*l, env.clone()), interp(*r, env)) {
                Value::Num(l - r)
            } else {
                panic!("incompatible types");
            }
        }
        Expr::Mul(l, r) => {
            if let (Value::Num(l), Value::Num(r)) = (interp(*l, env.clone()), interp(*r, env)) {
                Value::Num(l * r)
            } else {
                panic!("incompatible types");
            }
        }
        Expr::Div(l, r) => {
            if let (Value::Num(l), Value::Num(r)) = (interp(*l, env.clone()), interp(*r, env)) {
                Value::Num(l / r)
            } else {
                panic!("incompatible types");
            }
        }
        Expr::Val(name, expr, body) => interp(*body, {
            env.push((name, interp(*expr, env.clone())));
            env
        }),
        Expr::Id(ident) => env
            .iter()
            .find(|(x, _)| *x == ident)
            .expect("free identifier error")
            .1
            .clone(),
        Expr::Fun(param, body) => Value::Closure(param, *body, env),
        Expr::App(func, arg) => match interp(*func, env.clone()) {
            Value::Closure(x, b, mut fenv) => interp(b, { fenv.push((x, interp(*arg, env))); fenv }),
            _ => panic!("incompatible type"),
        },
    }
}
