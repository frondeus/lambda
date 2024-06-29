use crate::ast::{DebugExpr, Exprs};

use super::{Con, Term, TermId, Type, TypeEnv};

impl TypeEnv {
    pub fn debug(&self, id: TermId) -> DebugTerm {
        let t = &self.terms[id.0];
        t.debug(self)
    }
}

pub struct DebugType<'a> {
    env: &'a TypeEnv,
    t: &'a Type,
}

impl<'a> PartialEq for DebugType<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t.eq(other.t)
    }
}
impl Type {
    pub fn debug<'a>(&'a self, env: &'a TypeEnv) -> DebugType<'a> {
        DebugType { env, t: self }
    }
}

pub struct DebugTerm<'a> {
    env: &'a TypeEnv,
    t: &'a Term,
}
impl<'a> PartialEq for DebugTerm<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t.eq(other.t)
    }
}

impl Term {
    pub fn debug<'a>(&'a self, env: &'a TypeEnv) -> DebugTerm<'a> {
        DebugTerm { env, t: self }
    }
}

impl<'a> std::fmt::Debug for DebugType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.t {
            Type::Bool => write!(f, "Bool"),
            Type::Function(from, to) => f
                .debug_tuple("Fn")
                .field(&self.env.debug(*from))
                .field(&self.env.debug(*to))
                .finish(),
        }
    }
}

impl<'a> std::fmt::Debug for DebugTerm<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.t {
            Term::Mono(ty) => ty.debug(self.env).fmt(f),
            Term::ForAll(args, inner) => {
                let args = args
                    .iter()
                    .copied()
                    .map(|arg| self.env.debug(arg))
                    .collect::<Vec<_>>();
                let inner = self.env.debug(*inner);
                f.debug_tuple("Poly").field(&args).field(&inner).finish()
            }
            Term::Var(i) => write!(f, "T{i}"),
        }
    }
}

impl std::fmt::Debug for TermId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$t{}", self.0)
    }
}
pub struct DebugTypeEnv<'a> {
    pub types: &'a TypeEnv,
    pub exprs: &'a Exprs,
}
struct DebugExprTerm<'a> {
    expr: DebugExpr<'a>,
    term: DebugTerm<'a>,
}

impl<'a> std::fmt::Debug for DebugExprTerm<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {:?}", self.expr, self.term)
    }
}

impl<'a> std::fmt::Debug for DebugTypeEnv<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.types.exprs.iter().map(|(e, t)| DebugExprTerm {
                expr: self.exprs.debug(*e),
                term: self.types.debug(*t),
            }))
            .finish()
    }
}

impl std::fmt::Debug for TypeEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.terms.iter().map(|t| t.debug(self)))
            .finish()
    }
}

impl std::fmt::Debug for Con {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} = {:?}", self.left, self.right)
    }
}
