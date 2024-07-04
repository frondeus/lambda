use std::{cell::RefCell, fmt::Display, mem::MaybeUninit, rc::Rc};

use crate::ast::{Expr, ExprId, Exprs, InternId};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    // Keeping string only for displaying
    Fn(String, InternId, ExprId, RunEnv),
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
            Value::Fn(arg, _, _ret, _) => write!(f, "fn {arg}."),
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
    name: InternId,
    value: RefCell<MaybeUninit<Value>>,
    parent: Option<Rc<Scope>>,
}

impl RunEnv {
    fn get(&self, name: InternId) -> Option<Value> {
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

    fn push_uninit(&self, name: InternId) -> Self {
        Self {
            scope: Some(Rc::new(Scope {
                name,
                value: RefCell::new(MaybeUninit::uninit()),
                parent: self.scope.clone(),
            })),
        }
    }

    fn push(&self, name: InternId, value: Value) -> Self {
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
        Expr::Bool { value: b, node: _ } => Value::Bool(*b),
        Expr::Var { name: v, node: _ } => env.get(*v).expect("Var not found"),
        Expr::Def {
            arg: name,
            body,
            node: _,
        } => Value::Fn(e.get_str(*name).into(), *name, *body, env.clone()),
        Expr::Call {
            func: f,
            arg,
            node: _,
        } => match eval(e, env, *f) {
            Value::Fn(_name, name, body, captured_scope) => {
                let arg = eval(e, env, *arg);
                let mut inner = captured_scope.push(name, arg);
                eval(e, &mut inner, body)
            }
            _ => panic!("Expected function"),
        },
        Expr::Let {
            name,
            value,
            body,
            node: _,
        } => {
            let mut inner = env.push_uninit(*name);
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
