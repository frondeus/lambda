use std::{cell::RefCell, fmt::Display, mem::MaybeUninit, rc::Rc};

use crate::ast::{Expr, ExprId, Exprs};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Fn(&'static str, ExprId, RunEnv),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Fn(arg, _ret, _) => write!(f, "fn {arg}."),
        }
    }
}
#[derive(Default, Debug, Clone)]
pub struct RunEnv {
    scope: Option<Rc<Scope>>,
}

/// Our scope stores only one variable
#[derive(Debug)]
struct Scope {
    name: &'static str,
    value: RefCell<MaybeUninit<Value>>,
    parent: Option<Rc<Scope>>,
}

impl RunEnv {
    fn get(&self, name: &'static str) -> Option<Value> {
        std::iter::successors(self.scope.clone(), |scope| scope.parent.clone()).find_map(|scope| {
            let Scope {
                name: scoped_name,
                value,
                parent: _,
            } = &*scope;
            if &name == scoped_name {
                let value = value.borrow();
                let value = unsafe { value.assume_init_ref() };
                Some(value.clone())
            } else {
                None
            }
        })
    }

    fn push_uninit(&self, name: &'static str) -> Self {
        Self {
            scope: Some(Rc::new(Scope {
                name,
                value: RefCell::new(MaybeUninit::uninit()),
                parent: self.scope.clone(),
            })),
        }
    }

    fn push(&self, name: &'static str, value: Value) -> Self {
        Self {
            scope: Some(Rc::new(Scope {
                name,
                value: RefCell::new(MaybeUninit::new(value)),
                parent: self.scope.clone(),
            })),
        }
    }
}

/// Runtime does not have any error handling, it always panics,
/// because these are unrecoverable and unexpected errors.
/// The whole point of having type system is to prevent those from occurring
pub fn eval(e: &Exprs, env: &mut RunEnv, id: ExprId) -> Value {
    match e.get(id) {
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Var(v) => env.get(v).expect("Var not found"),
        Expr::Def(name, body) => Value::Fn(name, *body, env.clone()),
        Expr::Call(f, arg) => match eval(e, env, *f) {
            Value::Fn(name, body, captured_scope) => {
                let arg = eval(e, env, *arg);
                let mut inner = captured_scope.push(name, arg);
                eval(e, &mut inner, body)
            }
            _ => panic!("Expected function"),
        },
        Expr::Let(name, value, body) => {
            let mut inner = env.push_uninit(name);
            let value = eval(e, &mut inner, *value);
            inner
                .scope
                .as_mut()
                .expect("Scope")
                .value
                .borrow_mut()
                .write(value);
            eval(e, &mut inner, *body)
        }
    }
}
