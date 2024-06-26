use super::*;

pub trait BuilderFn {
    fn build(self, exprs: &mut Exprs) -> Expr;
    fn dependency(self, exprs: &mut Exprs) -> ExprId
    where
        Self: Sized,
    {
        let ast = self.build(exprs);
        exprs.push(ast)
    }

    fn root(self) -> (ExprId, Exprs)
    where
        Self: Sized,
    {
        let mut exprs: Exprs = Default::default();
        let id = self.dependency(&mut exprs);
        (id, exprs)
    }
}

impl<F> BuilderFn for F
where
    F: FnOnce(&mut Exprs) -> Expr,
{
    fn build(self, exprs: &mut Exprs) -> Expr {
        (self)(exprs)
    }
}

pub fn atom(ex: Expr) -> impl BuilderFn {
    |_e: &mut Exprs| ex
}

pub fn var(name: &'static str) -> impl BuilderFn {
    atom(Expr::Var(name))
}

pub fn boolean(b: bool) -> impl BuilderFn {
    atom(Expr::Bool(b))
}

pub fn def(arg: &'static str, ret: impl BuilderFn) -> impl BuilderFn {
    move |e: &mut Exprs| Expr::Def(arg, ret.dependency(e))
}

pub fn _let(name: &'static str, value: impl BuilderFn, then: impl BuilderFn) -> impl BuilderFn {
    move |e: &mut Exprs| Expr::Let(name, value.dependency(e), then.dependency(e))
}

pub fn call(fun: impl BuilderFn, arg: impl BuilderFn) -> impl BuilderFn {
    move |e: &mut Exprs| Expr::Call(fun.dependency(e), arg.dependency(e))
}

// Syntax Sugar
impl BuilderFn for bool {
    fn build(self, _: &mut Exprs) -> Expr {
        Expr::Bool(self)
    }
}

impl BuilderFn for &'static str {
    fn build(self, _: &mut Exprs) -> Expr {
        Expr::Var(self)
    }
}

pub trait BuilderFnExt: BuilderFn {
    fn call(self, arg: impl BuilderFn) -> impl BuilderFn
    where
        Self: Sized,
    {
        call(self, arg)
    }

    fn call_n(self, arg: impl CallnArgs) -> impl BuilderFn
    where
        Self: Sized,
    {
        arg.call(self)
    }
}
impl<B: BuilderFn> BuilderFnExt for B {}

pub fn calln(func: impl BuilderFn, args: impl CallnArgs) -> impl BuilderFn {
    args.call(func)
}

pub trait CallnArgs {
    fn call(self, func: impl BuilderFn) -> impl BuilderFn;
}

impl<T: BuilderFn> CallnArgs for (T,) {
    fn call(self, func: impl BuilderFn) -> impl BuilderFn {
        call(func, self.0)
    }
}

impl<T1, T2> CallnArgs for (T1, T2)
where
    T1: BuilderFn,
    T2: BuilderFn,
{
    fn call(self, func: impl BuilderFn) -> impl BuilderFn {
        call(call(func, self.0), self.1)
    }
}

pub fn let_n<F: BuilderFn>(name: &'static str, value: F) -> Let<F> {
    Let(name, value)
}

pub trait DefLike {
    fn ret(self, ret: impl BuilderFn) -> impl BuilderFn;
}

impl DefLike for &'static str {
    fn ret(self, ret: impl BuilderFn) -> impl BuilderFn {
        def(self, ret)
    }
}

impl DefLike for (&'static str, &'static str) {
    fn ret(self, ret: impl BuilderFn) -> impl BuilderFn {
        let (a, b) = self;
        def(a, def(b, ret))
    }
}

pub struct Let<F: BuilderFn>(&'static str, F);
pub trait LetLike {
    fn _in(self, then: impl BuilderFn) -> impl BuilderFn;

    fn and_let<G: BuilderFn>(self, name: &'static str, value: G) -> (Self, Let<G>)
    where
        Self: Sized,
    {
        (self, Let(name, value))
    }
}

impl<A, B> LetLike for (A, B)
where
    A: LetLike,
    B: LetLike,
{
    fn _in(self, then: impl BuilderFn) -> impl BuilderFn {
        let (a, b) = self;
        let b = b._in(then);
        a._in(b)
    }
}

impl<F: BuilderFn> LetLike for Let<F> {
    fn _in(self, then: impl BuilderFn) -> impl BuilderFn {
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
        let a = def("a", def("b", var("a"))).root();

        let b = ("a", "b").ret("a").root();

        assert_e!(a, b);
    }

    #[test]
    fn calln_tests() {
        let a = call(call(var("a"), boolean(true)), boolean(false)).root();

        let b = calln("a", (true, false)).root();

        assert_e!(a, b);
    }
}
