use std::{cell::RefCell, rc::Rc};

use super::typed::*;

pub fn type_check(expr: Expr, mut tenv: TEnv) -> Type {
    match expr {
        Expr::Num(_) => Type::Num,
        Expr::Add(left, right)
        | Expr::Sub(left, right)
        | Expr::Mul(left, right)
        | Expr::Div(left, right) => {
            let lt = type_check(*left, tenv.clone());
            let rt = type_check(*right, tenv.clone());
            unify(lt, Type::Num);
            unify(rt, Type::Num);
            Type::Num
        }
        Expr::Val(name, typ, expr, body) => {
            let et = type_check(*expr, tenv.clone());
            unify(et, typ.clone());

            type_check(*body, {
                tenv.push((name, typ));
                tenv
            })
        }
        Expr::Id(ident) => tenv
            .iter()
            .find(|(x, _)| *x == ident)
            .expect("free identifier error")
            .1
            .clone(),
        Expr::Fun(name, typ, body) => Type::Fun(
            Box::new(typ.clone()),
            Box::new(type_check(*body, {
                tenv.push((name, typ));
                tenv
            })),
        ),
        Expr::App(func, arg) => {
            let ft = type_check(*func, tenv.clone());
            let at = type_check(*arg, tenv);
            let rt = Type::Var {
                typ: Rc::new(RefCell::new(None)),
            };
            unify(ft, Type::Fun(Box::new(at), Box::new(rt.clone())));
            rt
        }
    }
}

fn error(s: String) {
    // println!("elifihwlefnwe");
    panic!("a type error occured: {}", s);
}

fn unify(t1: Type, t2: Type) {
    match (resolve(t1), resolve(t2)) {
        (ref t1 @ Type::Var { ref typ }, t2) => {
            if t1 == &t2 {
                // NOP
            } else if occurs(t1, t2.clone()) {
                error(format!("recursive type: {}, {}", t1, t2))
            } else {
                let mut t = typ.borrow_mut();
                *t = Some(t2);
            }
        }
        (t1, t2 @ Type::Var { typ: _ }) => unify(t2, t1),
        (Type::Num, Type::Num) => {
            // NOP
        }
        (Type::Fun(t3, t4), Type::Fun(t5, t6)) => {
            unify(*t3, *t5);
            unify(*t4, *t6);
        }
        (Type::Num, Type::Fun(_, _)) | (Type::Fun(_, _), Type::Num) => {
            error(format!("incompatible types: Num and Function"))
        }
    }
}

pub fn resolve(typ: Type) -> Type {
    match typ {
        Type::Var { typ: t } => match t.borrow().as_ref() {
            Some(inner) => resolve(inner.clone()),
            None => Type::Var { typ: Rc::clone(&t) },
        },
        _ => typ,
    }
}

// checks if t1 occurs in t2
fn occurs(t1: &Type, t2: Type) -> bool {
    match resolve(t2) {
        Type::Num => false,
        Type::Fun(l, r) => occurs(t1, *l) || occurs(t1, *r),
        Type::Var { typ: typ2 } => {
            if let Type::Var { typ: typ1 } = t1 {
                Rc::ptr_eq(&typ1, &typ2)
            } else {
                false
            }
        }
    }
}
