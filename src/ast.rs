use std::collections::BTreeMap;

pub mod arbitrary;
pub mod builder;
pub mod from_cst;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default, Debug)]
pub struct InternId(usize);

#[derive(Debug, PartialEq)]
pub enum Expr {
    Bool(bool),
    Var(InternId),                 // "x"
    Def(InternId, ExprId),         // "fn x: x"
    Call(ExprId, ExprId),          // x(y)
    Let(InternId, ExprId, ExprId), // let x = 0; x
}

#[derive(Default, PartialEq, Debug)]
pub struct Exprs {
    e: Vec<Expr>,
    i_to_s: BTreeMap<InternId, String>,
    s_to_i: BTreeMap<String, InternId>,
    intern_counter: InternId,
}

impl Exprs {
    pub fn push(&mut self, e: Expr) -> ExprId {
        if let Some(id) = self
            .e
            .iter()
            .enumerate()
            .find(|(_, en)| en == &&e)
            .map(|(id, _)| ExprId(id))
        {
            return id;
        }
        let id = ExprId(self.e.len());
        self.e.push(e);
        id
    }

    pub fn push_str(&mut self, s: impl ToString) -> InternId {
        let id = *self.s_to_i.entry(s.to_string()).or_insert_with(|| {
            let id = self.intern_counter;
            self.intern_counter.0 += 1;
            id
        });
        self.i_to_s.insert(id, s.to_string());
        id
    }

    pub fn get_str(&self, id: InternId) -> &str {
        self.i_to_s[&id].as_str()
    }

    pub fn get(&self, id: ExprId) -> &Expr {
        &self.e[id.0]
    }

    pub fn debug(&self, root: ExprId) -> DebugExpr {
        self.get(root).debug(self)
    }
}

#[derive(PartialEq)]
pub struct DebugExpr<'a> {
    ex: &'a Exprs,
    e: &'a Expr,
}
impl Expr {
    pub fn debug<'a>(&'a self, e: &'a Exprs) -> DebugExpr<'a> {
        DebugExpr { e: self, ex: e }
    }
}
impl std::fmt::Debug for ExprId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$e{}", self.0)
    }
}
impl<'a> std::fmt::Debug for DebugExpr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.e {
            Expr::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            Expr::Var(v) => write!(f, "{}", self.ex.get_str(*v)), //f.debug_tuple("Var").field(v).finish(),
            Expr::Def(arg, ret) => f
                .debug_tuple("Def")
                .field(&self.ex.get_str(*arg))
                .field(&self.ex.debug(*ret))
                .finish(),
            Expr::Call(fun, arg) => f
                .debug_tuple("Call")
                .field(&self.ex.debug(*fun))
                .field(&self.ex.debug(*arg))
                .finish(),
            Expr::Let(name, value, then) => f
                .debug_tuple("Let")
                .field(&self.ex.get_str(*name))
                .field(&self.ex.debug(*value))
                .field(&self.ex.debug(*then))
                .finish(),
        }
    }
}
