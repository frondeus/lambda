use std::collections::{BTreeMap, VecDeque};

use crate::{
    ast::{ExprId, InternId, SyntaxNode},
    diagnostics::Diagnostics,
};

pub mod queries;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default, Debug)]
pub struct VarId(usize);

#[derive(Debug, Clone)]
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
        arg: Option<ExprId>,
        body: Option<ExprId>,
        node: Option<SyntaxNode<'a>>,
    },
    Call {
        func: Option<ExprId>,
        arg: Option<ExprId>,
        node: Option<SyntaxNode<'a>>,
    },
    Let {
        name: Option<ExprId>,
        value: Option<ExprId>,
        body: Option<ExprId>,
        node: Option<SyntaxNode<'a>>,
    },
}

impl<'a> Expr<'a> {
    pub fn unwrap_var_def(&self) -> VarId {
        match self {
            Expr::VarDef {
                name: _,
                id,
                node: _,
            } => *id,
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
    pub fn from_ast(
        e: &'a crate::ast::Exprs<'a>,
        root: ExprId,
        diagnostics: &mut Diagnostics,
    ) -> Exprs<'a> {
        let ir = Exprs {
            e: e.e.iter().map(Expr::from_ast).collect(),
            i_to_s: e.i_to_s.clone(),
            s_to_i: e.s_to_i.clone(),
            intern_counter: e.intern_counter,
            vars: vec![],
        };

        let ir = fix_scope(ir, root, diagnostics);

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

    pub fn debug(&self, root: Option<ExprId>) -> Option<DebugExpr> {
        let root = root?;
        Some(self.get(root).debug(self))
    }
}

#[derive(Default, Debug)]
struct Scope {
    vars: BTreeMap<InternId, VarId>,
}

#[derive(Clone, Copy, Debug)]
enum StackItem {
    Expr(Option<ExprId>),
    ScopePop,
}

fn fix_scope<'a>(exprs: Exprs<'a>, e: ExprId, diagnostics: &mut Diagnostics) -> Exprs<'a> {
    let Exprs {
        e: mut exprs,
        i_to_s,
        s_to_i,
        intern_counter,
        mut vars,
    } = exprs;
    let mut var_counter = VarId(0);
    let mut scopes: Vec<Scope> = vec![];
    let mut stack: VecDeque<StackItem> = {
        let mut v: VecDeque<_> = Default::default();
        v.push_front(StackItem::Expr(Some(e)));
        v
    };
    while let Some(e) = stack.pop_back() {
        let e = match e {
            StackItem::Expr(Some(e)) => e,
            StackItem::Expr(None) => continue,
            StackItem::ScopePop => {
                scopes.pop();
                continue;
            }
        };

        match &mut exprs[e.0] {
            Expr::Def { arg, body, node: _ } => {
                stack.push_back(StackItem::ScopePop);
                stack.push_back(StackItem::Expr(*body));
                stack.push_back(StackItem::Expr(*arg));
                scopes.push(Scope::default());
            }
            Expr::Bool { value: _, node: _ } => (),
            Expr::Var { name, id, node } => {
                let mut scopes = scopes.iter().rev();

                let var = scopes.find_map(|s| s.vars.get(name).copied());
                if var.is_none() {
                    diagnostics.push(
                        node,
                        format!("Variable `{}` is not defined anywhere", i_to_s[name]),
                    );
                }
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
    Exprs {
        e: exprs,
        i_to_s,
        s_to_i,
        intern_counter,
        vars,
    }
}

impl<'a> Expr<'a> {
    fn from_ast(e: &'a crate::ast::Expr<'a>) -> Expr<'a> {
        match *e {
            crate::ast::Expr::Bool { value, ref node } => Expr::Bool {
                value,
                node: node.clone(),
            },
            crate::ast::Expr::Var { name, ref node } => Expr::Var {
                name,
                node: node.clone(),
                id: None,
            },
            crate::ast::Expr::VarDef { name, ref node } => Expr::VarDef {
                name,
                node: node.clone(),
                id: VarId(0),
            },
            crate::ast::Expr::Def {
                arg,
                body,
                ref node,
            } => Expr::Def {
                arg,
                body,
                node: node.clone(),
            },
            crate::ast::Expr::Call {
                func,
                arg,
                ref node,
            } => Expr::Call {
                func,
                arg,
                node: node.clone(),
            },
            crate::ast::Expr::Let {
                name,
                value,
                body,
                ref node,
            } => Expr::Let {
                name,
                value,
                body,
                node: node.clone(),
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

#[allow(clippy::expect_used)]
#[cfg(test)]
mod tests {
    use crate::ast::from_cst::{from_tree, get_tree};

    use super::*;

    #[test]
    fn ir_tests() -> test_runner::Result {
        test_runner::test_snapshots("tests/", "ir", |input, _deps| {
            let tree = get_tree(input);
            let (r, exprs) = from_tree(&tree, input, "test");
            let mut diagnostics = Diagnostics::default();
            let ir = Exprs::from_ast(&exprs, r.expect("Root node"), &mut diagnostics);
            format!("{:#?}", ir.debug(r))
        })
    }
}
