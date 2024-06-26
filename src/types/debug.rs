use crate::ast::{DebugExpr, Expr, ExprId, Exprs};

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
            Term::Poly(args, inner) => {
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
        // f.debug_list().entry(&self.expr).entry(&self.term).finish()
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

pub fn expr_print(exprs: &Exprs, id: ExprId, env: &TypeEnv) {
    fn expr_print_inner(
        exprs: &Exprs,
        id: ExprId,
        env: &TypeEnv,
        indent: usize,
        ignore_indent: bool,
    ) {
        let term_id = env.term_id_of(id).expect("Term");
        let term = env.print_term_id(term_id);
        let print_indent = |indent, ignore_indent: bool| {
            if !ignore_indent {
                print!("{ }", " ".repeat(indent * 4));
            }
        };
        print_indent(indent, ignore_indent);
        match exprs.get(id) {
            Expr::Bool(true) => println!("true # {term} ({term_id:?})"),
            Expr::Bool(false) => println!("false # {term} ({term_id:?})"),
            Expr::Var(name) => println!("{name} # {term} ({term_id:?})"),
            Expr::Def(arg, ret) => {
                println!("fn ({arg}) => # {term} ({term_id:?})");
                expr_print_inner(exprs, *ret, env, indent + 1, false);
            }
            Expr::Call(f, a) => {
                expr_print_inner(exprs, *f, env, indent, true);
                print_indent(indent, false);
                println!("(");
                expr_print_inner(exprs, *a, env, indent + 1, false);
                print_indent(indent, false);
                println!(") # {term} ({term_id:?})");
            }
            Expr::Let(name, value, then) => {
                print!("let {name} = ");
                expr_print_inner(exprs, *value, env, indent, true);
                print_indent(indent, ignore_indent);
                println!("in");
                expr_print_inner(exprs, *then, env, indent + 1, false);
            }
        }
    }

    expr_print_inner(exprs, id, env, 0, false);
}
