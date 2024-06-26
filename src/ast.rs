#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(usize);

#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    Var(&'static str),                 // "x"
    Def(&'static str, ExprId),         // "fn x: x"
    Call(ExprId, ExprId),              // x(y)
    Let(&'static str, ExprId, ExprId), // let x = 0; x
}

#[derive(Default)]
pub struct Exprs {
    e: Vec<Expr>,
}

impl Exprs {
    pub fn push(&mut self, e: Expr) -> ExprId {
        let id = ExprId(self.e.len());
        self.e.push(e);
        id
    }

    pub fn get(&self, id: ExprId) -> &Expr {
        &self.e[id.0]
    }
}

#[derive(Default)]
pub struct Builder {
    exprs: Exprs,
}

impl Builder {
    pub fn root(root: impl Fn(&mut Builder) -> ExprId) -> (ExprId, Exprs) {
        let mut builder = Builder::default();
        let id = (root)(&mut builder);
        (id, builder.exprs)
    }

    pub fn var(&mut self, name: &'static str) -> ExprId {
        self.exprs.push(Expr::Var(name))
    }

    pub fn tru(&mut self) -> ExprId {
        self.exprs.push(Expr::Bool(true))
    }

    pub fn fals(&mut self) -> ExprId {
        self.exprs.push(Expr::Bool(false))
    }

    pub fn def(&mut self, arg: &'static str, ret: impl Fn(&mut Builder) -> ExprId) -> ExprId {
        let ret = (ret)(self);
        self.exprs.push(Expr::Def(arg, ret))
    }

    pub fn let_(
        &mut self,
        name: &'static str,
        value: impl Fn(&mut Builder) -> ExprId,
        then: impl Fn(&mut Builder) -> ExprId,
    ) -> ExprId {
        let value = (value)(self);
        let then = (then)(self);
        self.exprs.push(Expr::Let(name, value, then))
    }

    pub fn call(
        &mut self,
        func: impl Fn(&mut Builder) -> ExprId,
        arg: impl Fn(&mut Builder) -> ExprId,
    ) -> ExprId {
        let func = (func)(self);
        let arg = (arg)(self);
        self.exprs.push(Expr::Call(func, arg))
    }
}

impl std::fmt::Debug for ExprId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$e{}", self.0)
    }
}
