use std::collections::{BTreeMap, VecDeque};

use tree_sitter::Node as SyntaxNode;

use crate::ast::{ExprId, InternId};

pub mod queries;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default, Debug)]
pub struct VarId(usize);

#[derive(Debug, Clone, Copy)]
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

impl<'a> Expr<'a> {
    pub fn unwrap_var_def(self) -> VarId {
        match self {
            Expr::VarDef { name: _, id, node: _ } => id,
            e => unreachable!("{:?} is not VarDef", e),
        }
    }
}

#[derive(Debug)]
pub struct Exprs<'a> {
    pub e: Vec<Expr<'a>>,
    pub i_to_s: BTreeMap<InternId, String>,
    pub s_to_i: BTreeMap<String, InternId>,
    pub intern_counter: InternId,
    pub vars: Vec<Variable>,
}

#[derive(Debug)]
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

    pub fn iter(&self) -> impl Iterator<Item = (ExprId, &Expr<'a>)> {
        self.e.iter().enumerate().map(|(id, e)| (ExprId(id), e))
    }

    pub fn debug(&self, root: ExprId) -> DebugExpr {
        self.get(root).debug(self)
    }
}

#[derive(Default, Debug)]
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
    let mut scopes: Vec<Scope> = vec![];
    let mut stack: VecDeque<StackItem> = {
        let mut v: VecDeque<_> = Default::default();
        v.push_front(StackItem::Expr(e));
        v
    };
    let mut vars: Vec<Variable> = vec![];
    while let Some(e) = stack.pop_back() {
        let e = match e {
            StackItem::Expr(e) => e,
            StackItem::ScopePop => {
                scopes.pop();
                continue;
            }
        };

        match exprs.get_mut(e) {
            Expr::Def { arg, body, node: _ } => {
                stack.push_back(StackItem::ScopePop);
                stack.push_back(StackItem::Expr(*body));
                stack.push_back(StackItem::Expr(*arg));
                scopes.push(Scope::default());
            }
            Expr::Bool { value: _, node: _ } => (),
            Expr::Var { name, id, node: _ } => {
                let mut scopes = scopes.iter().rev();

                let var = scopes.find_map(|s| s.vars.get(name).copied());
                *id = var;
            }
            Expr::VarDef { name, id, node: _ } => {
                if let Some(scope) = scopes.last_mut() {
                    let var = var_counter;
                    var_counter.0 += 1;

                    scope.vars.insert(*name, var);
                    vars.push(Variable { defined: e });
                    *id = var;
                }
            }
            Expr::Call { func, arg, node: _ } => {
                stack.push_back(StackItem::Expr(*arg));
                stack.push_back(StackItem::Expr(*func));
            }
            Expr::Let {
                name,
                value,
                body,
                node: _,
            } => {
                stack.push_back(StackItem::ScopePop);
                stack.push_back(StackItem::Expr(*body));
                stack.push_back(StackItem::Expr(*value));
                stack.push_back(StackItem::Expr(*name));
                scopes.push(Scope::default());
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

pub struct DebugExpr<'a> {
    ex: &'a Exprs<'a>,
    e: &'a Expr<'a>,
}
impl<'a> Expr<'a> {
    pub fn debug(&'a self, e: &'a Exprs) -> DebugExpr<'a> {
        DebugExpr { e: self, ex: e }
    }
}
impl<'a> std::fmt::Debug for DebugExpr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.e {
            Expr::Bool { value: b, node: _ } => f.debug_tuple("Bool").field(b).finish(),
            Expr::Var {
                name: v,
                id,
                node: _,
            } => write!(f, "Var({}, {id:?})", self.ex.get_str(*v)),
            Expr::VarDef { name, id, node: _ } => {
                write!(f, "VarDef({}, {id:?})", self.ex.get_str(*name))
            }
            Expr::Def {
                arg,
                body: ret,
                node: _,
            } => f
                .debug_tuple("Def")
                .field(&self.ex.debug(*arg))
                .field(&self.ex.debug(*ret))
                .finish(),
            Expr::Call {
                func: fun,
                arg,
                node: _,
            } => f
                .debug_tuple("Call")
                .field(&self.ex.debug(*fun))
                .field(&self.ex.debug(*arg))
                .finish(),
            Expr::Let {
                name,
                value,
                body: then,
                node: _,
            } => f
                .debug_tuple("Let")
                .field(&self.ex.debug(*name))
                .field(&self.ex.debug(*value))
                .field(&self.ex.debug(*then))
                .finish(),
        }
    }
}
