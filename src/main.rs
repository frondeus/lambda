fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use lambda::ast::Builder;
    use lambda::runtime::{eval, Value};
    use lambda::types::{type_of, Term, Type};

    macro_rules! debug_ty {
        ($exprs: expr, $id: expr) => {
            let mut env = Default::default();
            type_of($exprs, &mut env, $id);
            lambda::types::expr_print($exprs, $id, &env);
        };
    }

    macro_rules! assert_ty {
        ($exprs: expr, $id: expr, $expected: expr) => {
            assert_eq!($expected, type_of($exprs, &mut Default::default(), $id));
        };
    }

    macro_rules! assert_print_ty {
        ($exprs: expr, $id: expr, $expected: expr) => {
            let mut env = Default::default();
            let id = type_of($exprs, &mut env, $id);
            lambda::types::expr_print($exprs, $id, &env);
            assert_eq!($expected, env.print_term(id));
        };
    }

    macro_rules! assert_val {
        ($exprs: expr, $id: expr, $expected: expr) => {
            assert_eq!($expected, eval($exprs, &mut Default::default(), $id));
        };
    }

    macro_rules! assert_print_val {
        ($exprs: expr, $id: expr, $expected: expr) => {
            assert_eq!(
                $expected,
                eval($exprs, &mut Default::default(), $id).to_string()
            );
        };
    }

    #[test]
    fn test_fn_call() {
        let (root, exprs) = Builder::root(|b| {
            b.let_(
                "f",
                |b| b.def("x", |b| b.var("x")),
                |b| b.call(|b| b.var("f"), |b| b.tru()),
            )
        });

        assert_ty!(&exprs, root, Term::Mono(Type::Bool));
        assert_print_ty!(&exprs, root, "Bool");
        assert_val!(&exprs, root, Value::Bool(true));
    }

    #[test]
    fn test_ident() {
        let (root, exprs) = Builder::root(|b| b.def("x", |b| b.var("x")));

        assert_print_ty!(&exprs, root, "(T0 -> T0)");
        assert_print_val!(&exprs, root, "fn x.");
    }

    #[test]
    fn test_closure() {
        let (root, exprs) = Builder::root(|b| b.def("y", |b| b.def("x", |b| b.var("y"))));

        assert_print_ty!(&exprs, root, "(T0 -> (T1 -> T0))");
        assert_print_val!(&exprs, root, "fn y.");
    }

    #[test]
    fn test_closure_curry_call() {
        let (root, exprs) = Builder::root(|b| {
            b.let_(
                "f",
                |b| b.def("y", |b| b.def("x", |b| b.var("y"))),
                |b| b.call(|b| b.var("f"), |b| b.tru()),
            )
        });

        assert_print_ty!(&exprs, root, "(T1 -> Bool)");
        assert_print_val!(&exprs, root, "fn x.");
    }

    #[test]
    fn test_closure_call() {
        let (root, exprs) = Builder::root(|b| {
            b.call(
                |b| b.call(|b| b.def("y", |b| b.def("x", |b| b.var("y"))), |b| b.tru()),
                |b| b.fals(),
            )
        });

        assert_print_ty!(&exprs, root, "Bool");
        assert_print_val!(&exprs, root, "true");
    }

    #[test]
    fn test_fn_ret_tru() {
        let (root, exprs) = Builder::root(|b| b.def("x", |b| b.tru()));

        assert_print_ty!(&exprs, root, "(T0 -> Bool)");
        assert_print_val!(&exprs, root, "fn x.");
    }

    #[test]
    fn test_two_fn() {
        let (root, exprs) = Builder::root(|b| {
            b.let_(
                "f",
                |b| b.def("a", |b| b.var("a")),
                |b| {
                    b.let_(
                        "g",
                        |b| b.def("a", |b| b.tru()),
                        |b| {
                            b.let_(
                                "h",
                                |b| b.def("x", |b| b.def("y", |b| b.var("x"))),
                                |b| {
                                    b.call(
                                        |b| {
                                            b.call(
                                                |b| b.var("h"),
                                                |b| b.call(|b| b.var("f"), |b| b.tru()),
                                            )
                                        },
                                        |b| b.call(|b| b.var("g"), |b| b.tru()),
                                    )
                                },
                            )
                        },
                    )
                },
            )
        });

        assert_print_ty!(&exprs, root, "Bool");
        assert_print_val!(&exprs, root, "true");
    }

    #[test]
    fn test_let_poly() {
        let (root, exprs) = Builder::root(|b| {
            b.let_(
                "id",
                |b| b.def("x", |b| b.var("x")),
                |b| {
                    b.let_(
                        "h",
                        |b| b.def("a", |b| b.def("b", |b| b.var("a"))),
                        |b| {
                            b.call(
                                |b| {
                                    b.call(
                                        |b| b.var("h"),
                                        |b| b.call(|b| b.var("id"), |b| b.var("id")),
                                    )
                                },
                                |b| {
                                    b.call(
                                        // ID with bool
                                        |b| b.var("id"),
                                        |b| b.tru(),
                                    )
                                },
                            )
                        },
                    )
                },
            )
        });

        assert_print_ty!(&exprs, root, "ForAll (T0): (T0 -> T0)");
        assert_print_val!(&exprs, root, "fn x.");
    }
}
