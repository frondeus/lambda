fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use lambda::ast::builder::*;
    use lambda::runtime::{eval, Value};
    use lambda::types::{DebugTypeEnv, Term, Type};

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
        #[track_caller]
        fn eval(&mut self) -> &mut Self {
            eval(&self.exprs, &mut self.rt, self.root);
            self
        }
        #[track_caller]
        fn assert_eval(&mut self, expected: Value) -> &mut Self {
            let ret = eval(&self.exprs, &mut self.rt, self.root);
            assert_eq!(expected, ret);
            self
        }
        #[track_caller]
        fn assert_term(&mut self, expected: Term) -> &mut Self {
            assert_eq!(expected.debug(&self.types), self.term.debug(&self.types));
            self
        }
        #[track_caller]
        fn assert_print_term(&mut self, expected: &str) -> &mut Self {
            assert_eq!(expected.trim(), self.types.print_term(self.term.clone()));
            self
        }
        #[track_caller]
        fn assert_print_eval(&mut self, expected: &str) -> &mut Self {
            let ret = eval(&self.exprs, &mut self.rt, self.root);
            assert_eq!(expected, ret.to_string());
            self
        }
        fn dbg_env(&mut self) -> &mut Self {
            eprintln!(
                "{:#?}",
                DebugTypeEnv {
                    types: &self.types,
                    exprs: &self.exprs
                }
            );

            self
        }

        fn dbg_tree(&mut self) -> &mut Self {
            // expr_print(&self.exprs, self.root, &self.types);
            eprintln!("{:#?}", self.exprs.debug(self.root));
            self
        }

        fn dbg_term(&mut self) -> &mut Self {
            eprintln!("{:?}", self.term.debug(&self.types));
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
            .assert_print_term("(T2 -> Bool)")
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
            .dbg_env()
            // .dbg_tree()
            .assert_print_term("ForAll (T0): (T0 -> T0)")
            .assert_print_eval("fn x.");
    }

    #[test]
    #[should_panic(expected = "Does not unify: Bool Fn(Bool, T1)")]
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

    #[test]
    #[should_panic(expected = "Expected function, found Bool")]
    fn calling_bool() {
        true.call(false).test();
    }

    #[test]
    #[should_panic(expected = "Does not unify: Fn(T1, T2) Bool")]
    fn calling_bool_in_closure() {
        _let(
            "call",
            ("x", "y").ret("x".call("y")),
            "call".call_n((true, false)),
        )
        .test()
        .dbg_env()
        .dbg_tree()
        .dbg_term()
        .eval();
    }

    #[test]
    #[should_panic(expected = "Does not unify: Fn(Bool, T1) Bool")]
    fn calling_bool_in_fn() {
        _let("call", "x".ret("x".call(true)), "call".call(false))
            .test()
            .dbg_env()
            .dbg_tree()
            .dbg_term()
            .eval();
    }

    #[test]
    fn poly_2() {
        _let("f", ("a", "b").ret("b"), "f")
            .test()
            .assert_print_term(
                "
               ForAll (T0, T1): (T0 -> (T1 -> T1))     
            ",
            );
    }

    #[test]
    #[should_panic(expected = "Expected function, found Bool")]
    fn calling_bool_in_let() {
        _let("x", true.call(false), "x").test().dbg_term().eval();
    }
}
