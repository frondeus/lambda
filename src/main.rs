fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use lambda::ast::builder::*;
    use lambda::runtime::{eval, Value};
    use lambda::types::{Term, Type};

    struct Test {
        exprs: lambda::ast::Exprs,
        root: lambda::ast::ExprId,
        types: lambda::types::TypeEnv,
        rt: lambda::runtime::RunEnv,
        term: lambda::types::Term,
    }

    trait TestExt: BuilderFn + Sized {
        fn test(self) -> Test {
            let (root, exprs) = self.root();
            let mut types = Default::default();
            let rt = Default::default();
            let term = lambda::types::type_of(&exprs, &mut types, root);
            Test {
                exprs,
                root,
                rt,
                types,
                term,
            }
        }
    }
    impl<B: BuilderFn> TestExt for B {}
    impl Test {
        fn assert_eval(&mut self, expected: Value) -> &mut Self {
            let ret = eval(&self.exprs, &mut self.rt, self.root);
            assert_eq!(expected, ret);
            self
        }
        fn assert_term(&mut self, expected: Term) -> &mut Self {
            assert_eq!(expected, self.term);
            self
        }
        fn assert_print_term(&mut self, expected: &str) -> &mut Self {
            assert_eq!(expected, self.types.print_term(self.term.clone()));
            self
        }
        fn assert_print_eval(&mut self, expected: &str) -> &mut Self {
            let ret = eval(&self.exprs, &mut self.rt, self.root);
            assert_eq!(expected, ret.to_string());
            self
        }
    }

    #[test]
    fn test_fn_call() {
        let_n("f", "x".ret("x"))
            ._in("f".call(true))
            .test()
            .assert_term(Term::Mono(Type::Bool))
            .assert_print_term("Bool")
            .assert_eval(Value::Bool(true));
    }

    #[test]
    fn test_ident() {
        "x".ret("x")
            .test()
            .assert_print_term("(T0 -> T0)")
            .assert_print_eval("fn x.");
    }

    #[test]
    fn test_closure() {
        ("y", "x")
            .ret("y")
            .test()
            .assert_print_term("(T0 -> (T1 -> T0))")
            .assert_print_eval("fn y.");
    }

    #[test]
    fn test_closure_curry_call() {
        _let("f", ("y", "x").ret("y"), "f".call(true))
            .test()
            .assert_print_term("(T1 -> Bool)")
            .assert_print_eval("fn x.");
    }

    #[test]
    fn test_closure_call() {
        ("y", "x")
            .ret("y")
            .call_n((true, false))
            .test()
            .assert_print_term("Bool")
            .assert_print_eval("true");
    }

    #[test]
    fn test_fn_ret_tru() {
        "x".ret(true)
            .test()
            .assert_print_term("(T0 -> Bool)")
            .assert_print_eval("fn x.");
    }

    #[test]
    fn test_two_fn() {
        let_n("f", "a".ret("a"))
            .and_let("g", "a".ret(true))
            .and_let("h", ("x", "y").ret("x"))
            ._in("h".call_n(("f".call(true), "g".call(true))))
            .test()
            .assert_print_term("Bool")
            .assert_print_eval("true");
    }

    #[test]
    fn test_let_poly() {
        let_n("id", "x".ret("x"))
            .and_let("h", ("a", "b").ret("a"))
            ._in("h".call_n(("id".call("id"), "id".call(true))))
            .test()
            .assert_print_term("ForAll (T0): (T0 -> T0)")
            .assert_print_eval("fn x.");
    }

    #[test]
    #[should_panic(expected = "Does not unify: Mono(Bool) Mono(Function($t1, $t9))")]
    fn test_not_let_poly() {
        let_n(
            "f",
            "g".ret(
                let_n("a", "g".call(true))
                    .and_let("b", "g".call("g"))
                    ._in("a"),
            ),
        )
        ._in("f".call("id".ret("id")))
        .test()
        .assert_print_term("Bool")
        .assert_print_eval("true");
    }
}
