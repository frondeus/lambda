fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use lambda::ast::builder::*;
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
        let (root, exprs) = _letn("f", "x".ret("x")).then("f".call(true)).root();

        assert_ty!(&exprs, root, Term::Mono(Type::Bool));
        assert_print_ty!(&exprs, root, "Bool");
        assert_val!(&exprs, root, Value::Bool(true));
    }

    #[test]
    fn test_ident() {
        let (root, exprs) = "x".ret("x").root();

        assert_print_ty!(&exprs, root, "(T0 -> T0)");
        assert_print_val!(&exprs, root, "fn x.");
    }

    #[test]
    fn test_closure() {
        let (root, exprs) = ("y", "x").ret("y").root();

        assert_print_ty!(&exprs, root, "(T0 -> (T1 -> T0))");
        assert_print_val!(&exprs, root, "fn y.");
    }

    #[test]
    fn test_closure_curry_call() {
        let (root, exprs) = _let("f", ("y", "x").ret("y"), "f".call(true)).root();

        assert_print_ty!(&exprs, root, "(T1 -> Bool)");
        assert_print_val!(&exprs, root, "fn x.");
    }

    #[test]
    fn test_closure_call() {
        let (root, exprs) = ("y", "x").ret("y").calln((true, false)).root();

        assert_print_ty!(&exprs, root, "Bool");
        assert_print_val!(&exprs, root, "true");
    }

    #[test]
    fn test_fn_ret_tru() {
        let (root, exprs) = "x".ret(true).root();

        assert_print_ty!(&exprs, root, "(T0 -> Bool)");
        assert_print_val!(&exprs, root, "fn x.");
    }

    #[test]
    fn test_two_fn() {
        let (root, exprs) = _letn("f", "a".ret("a"))
            .and("g", "a".ret(true))
            .and("h", ("x", "y").ret("x"))
            .then("h".calln(("f".call(true), "g".call(true))))
            .root();

        assert_print_ty!(&exprs, root, "Bool");
        assert_print_val!(&exprs, root, "true");
    }

    #[test]
    fn test_let_poly() {
        let (root, exprs) = _letn("id", "x".ret("x"))
            .and("h", ("a", "b").ret("a"))
            .then("h".calln(("id".call("id"), "id".call(true))))
            .root();

        assert_print_ty!(&exprs, root, "ForAll (T0): (T0 -> T0)");
        assert_print_val!(&exprs, root, "fn x.");
    }
}
