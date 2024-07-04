use crate::ast::{DebugExpr, Exprs};

use super::{Con, Type, TypeEnv, TypeId};

impl TypeEnv {
    pub fn debug(&self, id: TypeId) -> DebugType {
        let t = &self.types[id.0];
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

impl<'a> std::fmt::Display for DebugType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.t {
            Type::Bool => write!(f, "Bool"),
            Type::Function(from, to) => {
                write!(f, "{} -> {}", self.env.debug(*from), self.env.debug(*to))
            }
            Type::ForAll(args, inner) => {
                let mut args = args.iter().copied().map(|arg| self.env.debug(arg));
                write!(f, "forall <")?;
                if let Some(a) = args.next() {
                    write!(f, "{a}")?;
                }
                for a in args {
                    write!(f, ", {a}")?;
                }
                write!(f, ">: {}", self.env.debug(*inner))
            }
            Type::Var(i) => write!(f, "T{i}"),
        }
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
            Type::ForAll(args, inner) => {
                let args = args
                    .iter()
                    .copied()
                    .map(|arg| self.env.debug(arg))
                    .collect::<Vec<_>>();
                let inner = self.env.debug(*inner);
                f.debug_tuple("Poly").field(&args).field(&inner).finish()
            }
            Type::Var(i) => write!(f, "T{i}"),
        }
    }
}

impl std::fmt::Debug for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$t{}", self.0)
    }
}
pub struct DebugTypeEnv<'a> {
    pub types: &'a TypeEnv,
    pub exprs: &'a Exprs<'a>,
}
struct DebugExprType<'a> {
    expr: DebugExpr<'a>,
    ty: DebugType<'a>,
}

impl<'a> std::fmt::Debug for DebugExprType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {:?}", self.expr, self.ty)
    }
}

impl<'a> std::fmt::Debug for DebugTypeEnv<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.types.exprs.iter().map(|(e, t)| DebugExprType {
                expr: self.exprs.debug(*e),
                ty: self.types.debug(*t),
            }))
            .finish()
    }
}

impl std::fmt::Debug for TypeEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.types.iter().map(|t| t.debug(self)))
            .finish()
    }
}

impl std::fmt::Debug for Con {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} = {:?}", self.left, self.right)
    }
}
