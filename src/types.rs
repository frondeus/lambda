use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use thiserror::Error;

use crate::ast::{ExprId, InternId, SyntaxNode};
use crate::diagnostics::Diagnostics;
use crate::ir::{Expr, Exprs, VarId};

mod debug;
pub use debug::*;

#[derive(Clone, PartialEq)]
pub enum Type {
    Bool,
    Function(TypeId, TypeId),
    /// For All T, U: ...
    ForAll(Vec<TypeId>, TypeId),
    Var(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeId(usize);

#[derive(Default)]
pub struct TypeEnv {
    vars: BTreeMap<VarId, Option<TypeId>>,
    exprs: HashMap<ExprId, TypeId>,
    types: Vec<Type>,
    /// For Type::Var counter
    var_counter: usize,
    constraints: Cons,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Con {
    left: TypeId,
    left_node: ExprId,
    right: TypeId,
}

#[derive(Default, Debug)]
struct Cons {
    cons: VecDeque<Con>,
}

impl Cons {
    fn push(&mut self, left: TypeId, right: TypeId, left_node: ExprId) {
        if left == right {
            return;
        }

        if self.cons.iter().any(|c| c.left == left && c.right == right) {
            return;
        }

        self.cons.push_back(Con {
            left,
            right,
            left_node,
        });
    }

    fn pop(&mut self) -> Option<Con> {
        self.cons.pop_front()
    }
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Could not unify {left} != {right}")]
    UnifyError { left: String, right: String },

    #[error("Variable `{name}` is not defined anywhere")]
    UndefinedVariable { name: String },

    #[error("Use of uninitialized value: {name}")]
    Uninitialized { name: String },

    #[error("Infinite type is not allowed")]
    InfiniteType,
}

impl TypeEnv {
    pub fn infer(e: &Exprs, root: ExprId, diagnostics: &mut Diagnostics) -> (Self, Type) {
        let mut env = TypeEnv::default();
        let id = type_of(e, &mut env, root, diagnostics);
        (env, id)
    }
}

/// Infers the type of an expression
fn type_of(e: &Exprs, env: &mut TypeEnv, id: ExprId, diagnostics: &mut Diagnostics) -> Type {
    let type_id = gather_cons(e, env, id, diagnostics);
    let type_id = unify(env, e, type_id, diagnostics);

    env.get_type(type_id)
}

/// First step of type inference - gathering constraints, and solving trivial types
fn gather_cons(e: &Exprs, env: &mut TypeEnv, id: ExprId, diagnostics: &mut Diagnostics) -> TypeId {
    match e.get(id) {
        Expr::Bool { value: _, node: _ } => env.set_type_for_expr(id, Type::Bool),
        Expr::Var {
            name,
            id: var_id,
            node,
        } => {
            let type_id = env
                .get_vars_type_id(e, diagnostics, *name, *var_id, node)
                .unwrap_or_else(|| env.new_type_var_id());
            env.set_type_id_for_expr(id, type_id)
        }
        Expr::VarDef { .. } => unreachable!(),
        Expr::Def {
            arg: name,
            body,
            node: _,
        } => {
            let var = env.new_type_var_id();
            let name_var = e.get(*name).unwrap_var_def();
            env.set_var(name_var, var);
            env.set_type_id_for_expr(*name, var);
            let ret = gather_cons(e, env, *body, diagnostics);
            env.set_type_for_expr(id, Type::Function(var, ret))
        }
        Expr::Call { func, arg, node: _ } => {
            let func_node = e.get(*func).node();
            let func_type_id = gather_cons(e, env, *func, diagnostics);
            let func_type = env.get_type(func_type_id);

            let arg_id = gather_cons(e, env, *arg, diagnostics);
            let (from, to) = match func_type.clone() {
                Type::Var(_) => {
                    let some_to = env.new_type_var_id();
                    let has_to_be_function = env.add_type(Type::Function(arg_id, some_to));

                    env.constraints
                        .push(func_type_id, has_to_be_function, *func);

                    (arg_id, some_to)
                }
                poly @ Type::ForAll(_, _) => instantiate_poly(env, poly),
                Type::Function(from, to) => (from, to),
                Type::Bool => {
                    diagnostics.push(
                        &func_node,
                        TypeError::UnifyError {
                            left: "Fn(?, ?)".to_string(),
                            right: format!("{:?}", func_type.debug(env)),
                        },
                    );
                    let some_to = env.new_type_var_id();
                    (arg_id, some_to)
                }
            };

            env.constraints.push(from, arg_id, *func);
            env.set_type_id_for_expr(id, to)
        }
        Expr::Let {
            name,
            value: value_id,
            body: then,
            node: _,
        } => {
            let name_var = e.get(*name).unwrap_var_def();
            env.new_var(name_var);

            let value = gather_cons(e, env, *value_id, diagnostics);

            let value_type = env.get_type(value);
            let value = match value_type {
                Type::Function(from, to) => {
                    let poly_var = [from, to]
                        .into_iter()
                        .flat_map(|t| collect_vars(env, t))
                        .collect::<BTreeSet<_>>();

                    if poly_var.is_empty() {
                        value
                    } else {
                        env.set_type_for_expr(
                            *value_id,
                            Type::ForAll(poly_var.into_iter().collect(), value),
                        )
                    }
                }
                _ => value,
            };

            env.set_type_id_for_expr(*name, value);
            env.set_var(name_var, value);

            let then = gather_cons(e, env, *then, diagnostics);
            env.set_type_id_for_expr(id, then)
        }
    }
}

/// For let polymorphism, we want to see if the function takes generic argument.
/// If its signature has `Type::Var(_)`, then we add it to the list of generics.
/// In Rust syntax:
/// We want to transform:
///
/// ```example
/// fn foo(a: ?T0, b: ?T1) -> ?T1
/// ```
/// Into:
/// ```example
/// fn foo<T0, T1>(a: T0, b: T1) -> T1
/// ```
///
/// And this is the step where we collect all `?Tn` variables.
fn collect_vars(env: &TypeEnv, id: TypeId) -> Vec<TypeId> {
    let mut vars: HashMap<usize, TypeId> = Default::default();
    let mut queue: VecDeque<TypeId> = Default::default();
    queue.push_back(id);

    while let Some(id) = queue.pop_front() {
        match env.get_type(id) {
            Type::Bool => (),
            Type::Function(from, to) => {
                queue.push_back(from);
                queue.push_back(to);
            }
            Type::ForAll(_, _) => (),
            Type::Var(var_id) => {
                vars.insert(var_id, id);
            }
        }
    }
    vars.into_values().collect()
}

/// Second step of type inference.
fn unify(
    env: &mut TypeEnv,
    e: &Exprs,
    mut root_id: TypeId,
    diagnostics: &mut Diagnostics,
) -> TypeId {
    let mut cons = std::mem::take(&mut env.constraints);
    while let Some(Con {
        left,
        right,
        left_node,
    }) = cons.pop()
    {
        if left == right {
            continue;
        }
        let l = env.get_type(left);
        let r = env.get_type(right);
        let left_n = e.get(left_node).node();

        match (l, r) {
            (Type::Var(_), _r) => replace_all(
                env,
                left,
                &left_n,
                right,
                &mut cons,
                &mut root_id,
                diagnostics,
            ),
            (_l, Type::Var(_)) => replace_all(
                env,
                right,
                &left_n,
                left,
                &mut cons,
                &mut root_id,
                diagnostics,
            ),
            (Type::Function(fr_a, to_a), Type::Function(fr_b, to_b)) => {
                cons.push(fr_a, fr_b, left_node);
                cons.push(to_a, to_b, left_node);
            }
            (l, r) => {
                diagnostics.push(
                    &left_n,
                    TypeError::UnifyError {
                        left: format!("{:?}", l.debug(env)),
                        right: format!("{:?}", r.debug(env)),
                    },
                );
            }
        }
    }
    root_id
}

/// Whenever a polymorphic (via. let polymorphism) function is called, we want to instantiate it into separate function
/// Because in (Rust pseudo)code, let polymorphism allows us to define polymorphic closure:
/// ```example
/// let f = |a| a;
/// f(1);
/// f(bool); // Even though we just called f(int), f(Bool) is also valid. That is not possible in Rust.
/// ```
/// Instantiating allows us to avoid type error Int != Bool
fn instantiate_poly(env: &mut TypeEnv, poly: Type) -> (TypeId, TypeId) {
    match poly.clone() {
        Type::ForAll(vars, poly_type) => match env.get_type(poly_type) {
            Type::ForAll(_, _) => panic!("Higher order polymorphism is not supported"),

            Type::Function(from, to) => instantiate(env, vars, from, to),
            Type::Bool | Type::Var(_) => panic!("Expected function"),
        },
        _ => unreachable!(),
    }
}

fn instantiate(
    env: &mut TypeEnv,
    vars: Vec<TypeId>,
    mut from: TypeId,
    mut to: TypeId,
) -> (TypeId, TypeId) {
    // Limitations of borrow checker, we can't replace types, while iterating over constraints.
    // So temporairly we move constraints out of `env`
    let mut cons = std::mem::take(&mut env.constraints);
    let mut new_cons = vec![];

    for var in vars.into_iter().rev() {
        let new_var = env.new_type_var();
        let new_var_id = env.add_type(new_var);
        from = replace(env, var, from, new_var_id);
        to = replace(env, var, to, new_var_id);

        for c in cons.cons.iter() {
            let maybe_new_cons = Con {
                left_node: c.left_node,
                left: replace(env, var, c.left, new_var_id),
                right: replace(env, var, c.right, new_var_id),
            };
            if c != &maybe_new_cons {
                new_cons.push(maybe_new_cons);
            }
        }
    }

    cons.cons.extend(new_cons);
    env.constraints = cons;

    (from, to)
}

fn replace_all(
    env: &mut TypeEnv,
    all_occurrences: TypeId,
    all_occurrences_node: &Option<SyntaxNode>,
    with: TypeId,
    cons: &mut Cons,
    root_id: &mut TypeId,
    diagnostics: &mut Diagnostics,
) {
    if occurs(env, all_occurrences, with) {
        diagnostics.push(all_occurrences_node, TypeError::InfiniteType);
        return;
    }

    for c in cons.cons.iter_mut() {
        c.left = replace(env, all_occurrences, c.left, with);
        c.right = replace(env, all_occurrences, c.right, with);
    }

    let mut exprs = std::mem::take(&mut env.exprs);

    for (_e_id, type_id) in exprs.iter_mut() {
        *type_id = replace(env, all_occurrences, *type_id, with);
    }
    env.exprs = exprs;

    *root_id = replace(env, all_occurrences, *root_id, with);
}

fn occurs(env: &mut TypeEnv, ty: TypeId, inside: TypeId) -> bool {
    if inside == ty {
        return true;
    }
    match env.get_type(inside) {
        Type::Bool => false,
        Type::Function(arg, ret) => occurs(env, ty, arg) || occurs(env, ty, ret),
        Type::ForAll(vars, inside) => {
            vars.iter().any(|v| occurs(env, ty, *v)) || occurs(env, ty, inside)
        }
        Type::Var(_) => false,
    }
}

fn replace(env: &mut TypeEnv, all_occurrences: TypeId, inside: TypeId, with: TypeId) -> TypeId {
    match env.get_type(inside) {
        Type::Function(in_arg, in_ret) => {
            let arg = replace(env, all_occurrences, in_arg, with);
            let ret = replace(env, all_occurrences, in_ret, with);

            env.add_type(Type::Function(arg, ret))
        }
        _ if all_occurrences == inside => with,
        _ => inside,
    }
}

impl TypeEnv {
    fn get_vars_type_id(
        &mut self,
        e: &Exprs,
        diagnostics: &mut Diagnostics,
        name: InternId,
        id: Option<VarId>,
        node: &Option<SyntaxNode>,
    ) -> Option<TypeId> {
        let id = id?;

        match self.vars.get(&id) {
            None => {
                let name = e.get_str(name).into();
                diagnostics.push(node, TypeError::UndefinedVariable { name });
                None
            }
            Some(None) => {
                let name = e.get_str(name).into();
                diagnostics.push(node, TypeError::Uninitialized { name });
                None
            }
            Some(Some(ty)) => Some(*ty),
        }
    }

    pub fn get_type(&self, id: TypeId) -> Type {
        self.types[id.0].clone()
    }

    pub fn type_of(&self, id: ExprId) -> Option<Type> {
        self.exprs.get(&id).map(|id| self.get_type(*id))
    }

    pub fn exprs(&self) -> impl Iterator<Item = (ExprId, Type)> + '_ {
        self.exprs
            .iter()
            .map(move |(id, ty_id)| (*id, self.get_type(*ty_id)))
    }

    fn add_type(&mut self, ty: Type) -> TypeId {
        if let Some(t_id) = self
            .types
            .iter()
            .enumerate()
            .find(|(_, t)| t == &&ty)
            .map(|(id, _)| id)
        {
            return TypeId(t_id);
        }

        let id = TypeId(self.types.len());
        self.types.push(ty);
        id
    }

    fn set_type_id_for_expr(&mut self, id: ExprId, type_id: TypeId) -> TypeId {
        self.exprs.insert(id, type_id);
        type_id
    }
    fn set_type_for_expr(&mut self, id: ExprId, ty: Type) -> TypeId {
        let type_id = self.add_type(ty);
        self.set_type_id_for_expr(id, type_id)
    }

    fn new_var(&mut self, id: VarId) {
        self.vars.insert(id, None);
    }

    fn set_var(&mut self, id: VarId, ty: TypeId) {
        self.vars.insert(id, Some(ty));
    }

    fn new_type_var(&mut self) -> Type {
        let id = self.var_counter;
        self.var_counter += 1;
        Type::Var(id)
    }

    fn new_type_var_id(&mut self) -> TypeId {
        let ty = self.new_type_var();
        self.add_type(ty)
    }

    fn print_type_id(&self, id: TypeId) -> String {
        let ty = self.get_type(id);
        self.print_type(ty)
    }

    pub fn print_type(&self, ty: Type) -> String {
        match ty {
            Type::Bool => "Bool".to_owned(),
            Type::Function(from, to) => {
                let from = self.print_type_id(from);
                let to = self.print_type_id(to);
                format!("({from} -> {to})")
            }
            Type::Var(i) => format!("T{i}"),
            Type::ForAll(vars, ty) => {
                let vars = vars
                    .iter()
                    .copied()
                    .map(|v| self.print_type_id(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("ForAll ({vars}): {}", self.print_type_id(ty))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::from_cst::{from_tree, get_tree},
        diagnostics::Diagnostics,
    };

    use super::*;

    #[test]
    fn types_tests() -> test_runner::Result {
        test_runner::test_snapshots("tests/", "type", |input, _deps| {
            let tree = get_tree(input);
            let (r, exprs) = from_tree(&tree, input);
            let mut diagnostics = Diagnostics::default();
            let ir = Exprs::from_ast(&exprs, r, &mut diagnostics);
            let mut types = TypeEnv::default();
            let ty = type_of(&ir, &mut types, r, &mut diagnostics);
            // let ty = ty.as_ref().map(|t| t.debug(&types));
            format!("{:#?}", ty.debug(&types))
        })
    }

    mod instantiate_tests {
        use super::*;

        #[test]
        fn ident() {
            let mut env = TypeEnv::default();

            let t0 = env.new_type_var_id();
            let source = Type::ForAll(vec![t0], env.add_type(Type::Function(t0, t0)));

            let (a_from, a_to) = instantiate_poly(&mut env, source);
            let actual = Type::Function(a_from, a_to);

            let t1 = TypeId(2);
            let expected = Type::Function(t1, t1);

            assert_eq!(expected.debug(&env), actual.debug(&env));
        }

        #[test]
        fn nested() {
            let mut env = TypeEnv::default();
            let t0 = env.new_type_var_id();
            let t1 = env.new_type_var_id();

            let t2 = env.add_type(Type::Function(t1, t0));
            let source = Type::ForAll(vec![t0, t1], env.add_type(Type::Function(t0, t2)));

            let (a_from, a_to) = instantiate_poly(&mut env, source);
            let actual = Type::Function(a_from, a_to);

            let t3 = TypeId(4);
            let t4 = TypeId(6);
            let expected_inner = env.add_type(Type::Function(t3, t4));
            let expected = Type::Function(t4, expected_inner);

            println!("{:#?}", env);

            assert_eq!(expected.debug(&env), actual.debug(&env));
        }
    }
}
