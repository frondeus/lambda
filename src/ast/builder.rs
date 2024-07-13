use super::*;

pub trait BuilderFn<'a> {
    fn build(self, exprs: &mut Exprs<'a>) -> Option<Expr<'a>>;
    fn build_with_node(self, exprs: &mut Exprs<'a>, node: SyntaxNode<'a>) -> Option<Expr<'a>>
    where
        Self: Sized,
    {
        Some(match self.build(exprs)? {
            Expr::Bool { value: b, node: _ } => Expr::Bool {
                value: b,
                node: Some(node),
            },
            Expr::VarDef { name, node: _ } => Expr::VarDef {
                name,
                node: Some(node),
            },
            Expr::Var { name: id, node: _ } => Expr::Var {
                name: id,
                node: Some(node),
            },
            Expr::Def {
                arg: id,
                body: ret,
                node: _,
            } => Expr::Def {
                arg: id,
                body: ret,
                node: Some(node),
            },
            Expr::Let {
                name: id,
                value,
                body: then,
                node: _,
            } => Expr::Let {
                name: id,
                value,
                body: then,
                node: Some(node),
            },
            Expr::Call { func, arg, node: _ } => Expr::Call {
                func,
                arg,
                node: Some(node),
            },
        })
    }

    fn dependency(self, exprs: &mut Exprs<'a>) -> Option<ExprId>
    where
        Self: Sized,
    {
        let ast = self.build(exprs)?;
        Some(exprs.push(ast))
    }

    fn root(self) -> (Option<ExprId>, Exprs<'a>)
    where
        Self: Sized,
    {
        let mut exprs: Exprs = Default::default();
        let id = self.dependency(&mut exprs);
        (id, exprs)
    }
}

impl<'a, F> BuilderFn<'a> for F
where
    F: FnOnce(&mut Exprs<'a>) -> Option<Expr<'a>>,
{
    fn build(self, exprs: &mut Exprs<'a>) -> Option<Expr<'a>> {
        (self)(exprs)
    }
}

pub fn atom(ex: Expr) -> impl BuilderFn {
    move |_e: &mut Exprs| Some(ex)
}

pub fn var<'t>(name: impl ToString) -> impl BuilderFn<'t> {
    let name = name.to_string();
    move |e: &mut Exprs| {
        Some(Expr::Var {
            name: e.push_str(name),
            node: None,
        })
    }
}

pub fn boolean<'t>(b: bool) -> impl BuilderFn<'t> {
    atom(Expr::Bool {
        value: b,
        node: None,
    })
}

fn var_def<'t>(arg: impl ToString) -> impl BuilderFn<'t> {
    let arg = arg.to_string();
    move |e: &mut Exprs<'t>| {
        Some(Expr::VarDef {
            name: e.push_str(arg),
            node: None,
        })
    }
}

pub fn def<'t>(arg: impl VarDefLike<'t>, ret: impl BuilderFn<'t>) -> impl BuilderFn<'t> {
    move |e: &mut Exprs<'t>| {
        Some(Expr::Def {
            arg: arg.var_def_dep(e),
            body: ret.dependency(e),
            node: None,
        })
    }
}

pub struct VarDef<'t> {
    pub arg: String,
    pub node: Option<SyntaxNode<'t>>,
}
impl<'t> BuilderFn<'t> for VarDef<'t> {
    fn build(self, exprs: &mut Exprs<'t>) -> Option<Expr<'t>> {
        match self.node {
            None => var_def(self.arg).build(exprs),
            Some(node) => var_def(self.arg).build_with_node(exprs, node),
        }
    }
}

pub trait VarDefLike<'t>: BuilderFn<'t> + Sized {
    fn build_var_def(self, exprs: &mut Exprs<'t>) -> Option<Expr<'t>> {
        Some(match self.build(exprs)? {
            Expr::Var { name, node } => Expr::VarDef { name, node },
            Expr::VarDef { name, node } => Expr::VarDef { name, node },
            e => unreachable!("{:?} is not Var", e),
        })
    }
    fn var_def_dep(self, expr: &mut Exprs<'t>) -> Option<ExprId> {
        let e = self.build_var_def(expr)?;
        Some(expr.push(e))
    }
}

impl<'t> VarDefLike<'t> for &'t str {}
impl<'t> VarDefLike<'t> for VarDef<'t> {}

pub fn _let<'t>(
    name: impl VarDefLike<'t>,
    value: impl BuilderFn<'t>,
    then: impl BuilderFn<'t>,
) -> impl BuilderFn<'t> {
    move |e: &mut Exprs<'t>| {
        Some(Expr::Let {
            name: name.var_def_dep(e),
            value: value.dependency(e),
            body: then.dependency(e),
            node: None,
        })
    }
}

pub fn call<'t>(fun: impl BuilderFn<'t>, arg: impl BuilderFn<'t>) -> impl BuilderFn<'t> {
    move |e: &mut Exprs<'t>| {
        Some(Expr::Call {
            func: fun.dependency(e),
            arg: arg.dependency(e),
            node: None,
        })
    }
}

// Syntax Sugar
impl<'t> BuilderFn<'t> for bool {
    fn build(self, _: &mut Exprs<'t>) -> Option<Expr<'t>> {
        Some(Expr::Bool {
            value: self,
            node: None,
        })
    }
}

impl<'t> BuilderFn<'t> for &'t str {
    fn build(self, e: &mut Exprs<'t>) -> Option<Expr<'t>> {
        var(self.to_string()).build(e)
    }
}

pub trait BuilderFnExt<'t>: BuilderFn<'t> {
    fn call(self, arg: impl BuilderFn<'t>) -> impl BuilderFn<'t>
    where
        Self: Sized,
    {
        call(self, arg)
    }

    fn call_n(self, arg: impl CallnArgs<'t>) -> impl BuilderFn<'t>
    where
        Self: Sized,
    {
        arg.call(self)
    }
}
impl<'t, B: BuilderFn<'t>> BuilderFnExt<'t> for B {}

pub fn calln<'t>(func: impl BuilderFn<'t>, args: impl CallnArgs<'t>) -> impl BuilderFn<'t> {
    args.call(func)
}

pub trait CallnArgs<'t> {
    fn call(self, func: impl BuilderFn<'t>) -> impl BuilderFn<'t>;
}

impl<'t, T: BuilderFn<'t>> CallnArgs<'t> for (T,) {
    fn call(self, func: impl BuilderFn<'t>) -> impl BuilderFn<'t> {
        call(func, self.0)
    }
}

impl<'t, T1, T2> CallnArgs<'t> for (T1, T2)
where
    T1: BuilderFn<'t>,
    T2: BuilderFn<'t>,
{
    fn call(self, func: impl BuilderFn<'t>) -> impl BuilderFn<'t> {
        call(call(func, self.0), self.1)
    }
}

pub fn let_n<'t, F: BuilderFn<'t>>(name: &'t str, value: F) -> Let<'t, F> {
    Let(name, value)
}

pub trait DefLike<'t> {
    fn ret(self, ret: impl BuilderFn<'t>) -> impl BuilderFn<'t>;
}

impl<'t> DefLike<'t> for &'static str {
    fn ret(self, ret: impl BuilderFn<'t>) -> impl BuilderFn<'t> {
        def(self, ret)
    }
}

impl<'t> DefLike<'t> for (&'static str, &'static str) {
    fn ret(self, ret: impl BuilderFn<'t>) -> impl BuilderFn<'t> {
        let (a, b) = self;
        def(a, def(b, ret))
    }
}

pub struct Let<'t, F: BuilderFn<'t>>(&'t str, F);
pub trait LetLike<'t> {
    fn _in(self, then: impl BuilderFn<'t>) -> impl BuilderFn<'t>;

    fn and_let<G: BuilderFn<'t>>(self, name: &'t str, value: G) -> (Self, Let<G>)
    where
        Self: Sized,
    {
        (self, Let(name, value))
    }
}

impl<'t, A, B> LetLike<'t> for (A, B)
where
    A: LetLike<'t>,
    B: LetLike<'t>,
{
    fn _in(self, then: impl BuilderFn<'t>) -> impl BuilderFn<'t> {
        let (a, b) = self;
        let b = b._in(then);
        a._in(b)
    }
}

impl<'t, F: BuilderFn<'t>> LetLike<'t> for Let<'t, F> {
    fn _in(self, then: impl BuilderFn<'t>) -> impl BuilderFn<'t> {
        _let(self.0, self.1, then)
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    macro_rules! assert_e {
        ($a: expr, $b: expr) => {
            let (a_root, a_exprs) = $a;
            let (b_root, b_exprs) = $b;
            let a_dbg = format!("{:?}", a_exprs.debug(a_root));
            let b_dbg = format!("{:?}", b_exprs.debug(b_root));

            assert_eq!(a_dbg, b_dbg);
        };
    }

    #[test]
    fn letn_tests() {
        let a = _let(
            "a",
            boolean(true),
            _let("b", boolean(false), boolean(false)),
        )
        .root();

        let b = let_n("a", true).and_let("b", false)._in(false).root();

        assert_e!(a, b);
    }

    #[test]
    fn defn_tests() {
        let a = def("a", def("b", var("a".to_string()))).root();

        let b = ("a", "b").ret("a").root();

        assert_e!(a, b);
    }

    #[test]
    fn calln_tests() {
        let a = call(call(var("a".to_string()), boolean(true)), boolean(false)).root();

        let b = calln("a", (true, false)).root();

        assert_e!(a, b);
    }
}
