use crate::test_suite::*;
use lambda::ast::builder::*;
use lambda::runtime::Value;
use lambda::types::Type;

#[test]
fn test_fn_call() {
    let_n("f", "x".ret("x"))
        ._in("f".call(true))
        .test()
        .assert_type(Type::Bool)
        .assert_print_type("Bool")
        .assert_eval(Value::Bool(true));
}

#[test]
fn test_ident() {
    "x".ret("x")
        .test()
        .assert_print_type("(T0 -> T0)")
        .assert_print_eval("fn x.");
}

#[test]
fn test_closure_only() {
    ("y", "x")
        .ret("y")
        .test()
        .dbg_env()
        .assert_print_type("(T0 -> (T1 -> T0))")
        .assert_print_eval("fn y.");
}

#[test]
fn test_closure_curry_call() {
    _let("f", ("y", "x").ret("y"), "f".call(true))
        .test()
        .assert_print_type("(T2 -> Bool)")
        .assert_print_eval("fn x.");
}

#[test]
fn test_closure_call() {
    ("y", "x")
        .ret("y")
        .call_n((true, false))
        .test()
        .assert_print_type("Bool")
        .assert_print_eval("true");
}

#[test]
fn test_fn_ret_tru() {
    "x".ret(true)
        .test()
        .assert_print_type("(T0 -> Bool)")
        .assert_print_eval("fn x.");
}

#[test]
fn test_two_fn() {
    let_n("f", "a".ret("a"))
        .and_let("g", "a".ret(true))
        .and_let("h", ("x", "y").ret("x"))
        ._in("h".call_n(("f".call(true), "g".call(true))))
        .test()
        .assert_print_type("Bool")
        .assert_print_eval("true");
}

#[test]
fn test_let_poly() {
    let_n("id", "x".ret("x"))
        .and_let("h", ("a", "b").ret("a"))
        ._in("h".call_n(("id".call("id"), "id".call(true))))
        .test()
        .assert_print_type("ForAll (T0): (T0 -> T0)")
        .assert_print_eval("fn x.");
}

#[test]
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
    .assert_error("Could not unify Bool != Fn(Bool, T1)");
}

#[test]
fn calling_bool() {
    true.call(false)
        .test()
        .assert_error("Could not unify Fn(?, ?) != Bool");
}

#[test]
fn calling_bool_in_closure() {
    _let(
        "call",
        ("x", "y").ret("x".call("y")),
        "call".call_n((true, false)),
    )
    .test()
    .assert_error("Could not unify Fn(T1, T2) != Bool");
}

#[test]
fn calling_bool_in_fn() {
    _let("call", "x".ret("x".call(true)), "call".call(false))
        .test()
        .assert_error("Could not unify Fn(Bool, T1) != Bool");
}

#[test]
fn poly_2() {
    _let("f", ("a", "b").ret("b"), "f")
        .test()
        .assert_print_type("ForAll (T0, T1): (T0 -> (T1 -> T1))");
}

#[test]
fn calling_bool_in_let() {
    _let("x", true.call(false), "x")
        .test()
        .dbg_type()
        .assert_error("Could not unify Fn(?, ?) != Bool");
}

#[test]
fn infinite_recursion() {
    _let("x", "a".ret("x"), "x".call("x"))
        .test()
        .dbg_type()
        .eval();
}

#[test]
fn infinite_let() {
    _let("x", "x", "x")
        .test()
        .dbg_type()
        .assert_error("Use of uninitialized value: x");
}

#[test]
fn infinite_type() {
    "a".ret("a".call("a"))
        .test()
        .dbg_env()
        .dbg_type()
        .assert_error("Infinite type is not allowed");
}

#[test]
fn undefined_var() {
    "a".test()
        .dbg_env()
        .dbg_type()
        .assert_error("Variable `a` is not defined anywhere")
        .eval();
}
