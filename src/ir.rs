use std::collections::BTreeMap;

use tree_sitter::Node as SyntaxNode;

use crate::ast::{ExprId, InternId};

pub mod queries;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default, Debug)]
pub struct VarId(usize);

#[derive(Debug)]
pub enum Expr<'a> {
    Bool {
        value: bool,
        node: Option<SyntaxNode<'a>>,
    },
    Var {
        name: InternId,
        id: Option<VarId>,
        node: Option<SyntaxNode<'a>>,
    },
    VarDef {
        name: InternId,
        id: VarId,
        node: Option<SyntaxNode<'a>>,
    },
    Def {
        arg: ExprId,
        body: ExprId,
        node: Option<SyntaxNode<'a>>,
    },
    Call {
        func: ExprId,
        arg: ExprId,
        node: Option<SyntaxNode<'a>>,
    },
    Let {
        name: ExprId,
        value: ExprId,
        body: ExprId,
        node: Option<SyntaxNode<'a>>,
    },
}

pub struct Exprs<'a> {
    pub e: Vec<Expr<'a>>,
    pub i_to_s: BTreeMap<InternId, String>,
    pub s_to_i: BTreeMap<String, InternId>,
    pub intern_counter: InternId,
    pub vars: Vec<Variable>,
}

pub struct Variable {
    pub defined: ExprId,
}

impl<'a> Exprs<'a> {
    pub fn from_ast(e: &'a crate::ast::Exprs<'a>, root: ExprId) -> Exprs<'a> {
        let mut ir = Exprs {
            e: e.e.iter().map(Expr::from_ast).collect(),
            i_to_s: e.i_to_s.clone(),
            s_to_i: e.s_to_i.clone(),
            intern_counter: e.intern_counter,
            vars: vec![],
        };

        fix_scope(&mut ir, root);

        ir
    }

    pub fn get_mut(&mut self, id: ExprId) -> &mut Expr<'a> {
        &mut self.e[id.0]
    }

    pub fn get_str(&self, id: InternId) -> &str {
        self.i_to_s[&id].as_str()
    }

    pub fn get(&self, id: ExprId) -> &Expr {
        &self.e[id.0]
    }

    pub fn get_var(&self, id: VarId) -> &Variable {
        &self.vars[id.0]
    }
}

#[derive(Default)]
struct Scope {
    vars: BTreeMap<InternId, VarId>,
}

#[derive(Clone, Copy, Debug)]
enum StackItem {
    Expr(ExprId),
    ScopePop,
}

fn fix_scope(exprs: &mut Exprs, e: ExprId) {
    let mut var_counter = VarId(0);
    let mut scope: Vec<Scope> = vec![];
    let mut stack: Vec<StackItem> = vec![StackItem::Expr(e)];
    let mut vars: Vec<Variable> = vec![];
    while let Some(e) = stack.pop() {
        let e = match e {
            StackItem::Expr(e) => e,
            StackItem::ScopePop => {
                scope.pop();
                continue;
            }
        };

        match exprs.get_mut(e) {
            Expr::Def { arg, body, node: _ } => {
                scope.push(Scope::default());
                stack.push(StackItem::ScopePop);
                stack.push(StackItem::Expr(*body));
                stack.push(StackItem::Expr(*arg));
            }
            Expr::Bool { value: _, node: _ } => (),
            Expr::Var { name, id, node: _ } => {
                let scope = scope.last_mut().unwrap();
                let var = scope.vars.get(name).copied();
                *id = var;
            }
            Expr::VarDef { name, id, node: _ } => {
                let scope = scope.last_mut().unwrap();
                let var = var_counter;
                var_counter.0 += 1;
                scope.vars.insert(*name, var);
                vars.push(Variable { defined: e });
                *id = var;
            }
            Expr::Call { func, arg, node: _ } => {
                stack.push(StackItem::Expr(*arg));
                stack.push(StackItem::Expr(*func));
            }
            Expr::Let {
                name,
                value,
                body,
                node: _,
            } => {
                scope.push(Scope::default());
                stack.push(StackItem::ScopePop);
                stack.push(StackItem::Expr(*body));
                stack.push(StackItem::Expr(*value));
                stack.push(StackItem::Expr(*name));
            }
        }
    }
    exprs.vars = vars;
}

impl<'a> Expr<'a> {
    fn from_ast(e: &'a crate::ast::Expr<'a>) -> Expr<'a> {
        match *e {
            crate::ast::Expr::Bool { value, node } => Expr::Bool { value, node },
            crate::ast::Expr::Var { name, node } => Expr::Var {
                name,
                node,
                id: None,
            },
            crate::ast::Expr::VarDef { name, node } => Expr::VarDef {
                name,
                node,
                id: VarId(0),
            },
            crate::ast::Expr::Def { arg, body, node } => Expr::Def { arg, body, node },
            crate::ast::Expr::Call { func, arg, node } => Expr::Call { func, arg, node },
            crate::ast::Expr::Let {
                name,
                value,
                body,
                node,
            } => Expr::Let {
                name,
                value,
                body,
                node,
            },
        }
    }
}
