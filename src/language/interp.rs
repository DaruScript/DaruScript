use super::ast::*;

type Env = Vec<(String, isize)>;

pub fn interp(expr: Expr, env: Env) -> isize {
    match expr {
        Expr::Num(n) => n,
        Expr::Add(l, r) => interp(*l, env.clone()) + interp(*r, env),
        Expr::Sub(l, r) => interp(*l, env.clone()) - interp(*r, env),
        Expr::Mul(l, r) => interp(*l, env.clone()) * interp(*r, env),
        Expr::Div(l, r) => interp(*l, env.clone()) / interp(*r, env),
        Expr::Val(ident, expr, body) => interp(*body, {
            let mut nenv = env;
            nenv.push((ident, interp(*expr, nenv.clone())));
            nenv
        }),
        Expr::Id(ident) => {
            env.iter()
                .find(|(x, _)| *x == ident)
                .expect("free identifier error")
                .1
        }
    }
}
