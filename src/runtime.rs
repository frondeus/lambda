use std::{collections::HashMap, fmt::Display};

use crate::ast::{Expr, ExprId, Exprs};

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Bool(bool),
    Fn(&'static str, ExprId, RunEnv),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Fn(arg, _ret, _) => write!(f, "fn {arg}."),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RunEnv {
    vars: HashMap<&'static str, Value>,
}

impl RunEnv {
    fn get(&self, name: &'static str) -> Option<Value> {
        // self.vars.iter().rev().find_map(|v| v.get(name)).copied()
        self.vars.get(name).cloned()
    }

    fn push(&self, name: &'static str, value: Value) -> Self {
        let mut inner = Self {
            vars: self.vars.clone(),
        };
        inner.vars.insert(name, value);
        inner
    }
}
pub fn eval(e: &Exprs, env: &mut RunEnv, id: ExprId) -> Value {
    match e.get(id) {
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Var(v) => env.get(v).expect("Var not found"),
        Expr::Def(name, body) => Value::Fn(name, *body, env.clone()),
        Expr::Call(f, arg) => match eval(e, env, *f) {
            Value::Fn(name, body, env_inner) => {
                let arg = eval(e, env, *arg);
                let mut inner = env_inner.push(name, arg);
                let res = eval(e, &mut inner, body);
                res
            }
            _ => panic!("Expected function"),
        },
        Expr::Let(name, value, body) => {
            let value = eval(e, env, *value);
            let mut inner = env.push(name, value);
            let res = eval(e, &mut inner, *body);
            res
        }
    }
}
