pub mod builder;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(usize);

#[derive(Debug, PartialEq)]
pub enum Expr {
    Bool(bool),
    Var(&'static str),                 // "x"
    Def(&'static str, ExprId),         // "fn x: x"
    Call(ExprId, ExprId),              // x(y)
    Let(&'static str, ExprId, ExprId), // let x = 0; x
}

#[derive(Default, PartialEq)]
pub struct Exprs {
    e: Vec<Expr>,
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
            Expr::Var(v) => write!(f, "{v}"), //f.debug_tuple("Var").field(v).finish(),
            Expr::Def(arg, ret) => f
                .debug_tuple("Def")
                .field(arg)
                .field(&self.ex.debug(*ret))
                .finish(),
            Expr::Call(fun, arg) => f
                .debug_tuple("Call")
                .field(&self.ex.debug(*fun))
                .field(&self.ex.debug(*arg))
                .finish(),
            Expr::Let(name, value, then) => f
                .debug_tuple("Let")
                .field(name)
                .field(&self.ex.debug(*value))
                .field(&self.ex.debug(*then))
                .finish(),
        }
    }
}
