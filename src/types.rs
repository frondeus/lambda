use std::collections::{BTreeSet, HashMap, VecDeque};
use thiserror::Error;

use crate::ast::{var_def_to_intern, Expr, ExprId, Exprs, InternId};

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
    vars: Vec<HashMap<InternId, Option<TypeId>>>,
    exprs: HashMap<ExprId, TypeId>,
    types: Vec<Type>,
    var_counter: usize,
    constraints: Cons,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Con {
    left: TypeId,
    right: TypeId,
}

#[derive(Default, Debug)]
struct Cons {
    cons: VecDeque<Con>,
}

impl Cons {
    fn push(&mut self, left: TypeId, right: TypeId) {
        if left == right {
            return;
        }

        if self.cons.iter().any(|c| c == &Con { left, right }) {
            return;
        }

        self.cons.push_back(Con { left, right });
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

pub type Result<T, E = TypeError> = std::result::Result<T, E>;

/// Infers the type of an expression
pub fn type_of(e: &Exprs, env: &mut TypeEnv, id: ExprId) -> Result<Type> {
    let type_id = gather_cons(e, env, id)?;
    let type_id = unify(env, type_id)?;

    Ok(env.get_type(type_id))
}

/// First step of type inference - gathering constraints, and solving trivial types
fn gather_cons(e: &Exprs, env: &mut TypeEnv, id: ExprId) -> Result<TypeId> {
    Ok(match e.get(id) {
        Expr::Bool { value: _, node: _ } => env.set_type_for_expr(id, Type::Bool),
        Expr::Var { name, node: _ } => {
            let type_id = env
                .get_vars_type_id(e, *name)?
                .ok_or(TypeError::UndefinedVariable {
                    name: e.get_str(*name).into(),
                })?;
            env.set_type_id_for_expr(id, type_id)
        }
        Expr::VarDef { .. } => unreachable!(),
        Expr::Def {
            arg: name,
            body,
            node: _,
        } => {
            let var = env.new_var();
            let name_intern = var_def_to_intern(e, *name);
            let var = env.push_scope(name_intern, var);
            env.set_type_id_for_expr(*name, var);
            let ret = gather_cons(e, env, *body)?;
            env.pop_scope();
            env.set_type_for_expr(id, Type::Function(var, ret))
        }
        Expr::Call { func, arg, node: _ } => {
            let func_type_id = gather_cons(e, env, *func)?;
            let func_type = env.get_type(func_type_id);

            let arg_id = gather_cons(e, env, *arg)?;
            let (from, to) = match func_type.clone() {
                Type::Var(_) => {
                    let some_to = env.new_var_as_type();
                    let has_to_be_function = env.add_type(Type::Function(arg_id, some_to));

                    env.constraints.push(func_type_id, has_to_be_function);

                    (arg_id, some_to)
                }
                poly @ Type::ForAll(_, _) => instantiate_poly(env, poly),
                Type::Function(from, to) => (from, to),
                Type::Bool => {
                    return Err(TypeError::UnifyError {
                        left: "Fn(?, ?)".to_string(),
                        right: format!("{:?}", func_type.debug(env)),
                    });
                }
            };

            env.constraints.push(from, arg_id);
            env.set_type_id_for_expr(id, to)
        }
        Expr::Let {
            name,
            value: value_id,
            body: then,
            node: _,
        } => {
            let name_intern = var_def_to_intern(e, *name);
            env.push_uninitialized_scope(name_intern);

            let value = gather_cons(e, env, *value_id)?;
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
            env.replace_with_some(name_intern, value);

            let then = gather_cons(e, env, *then)?;
            env.pop_scope();
            env.set_type_id_for_expr(id, then)
        }
    })
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
fn unify(env: &mut TypeEnv, mut root_id: TypeId) -> Result<TypeId> {
    let mut cons = std::mem::take(&mut env.constraints);
    while let Some(Con { left, right }) = cons.pop() {
        if left == right {
            continue;
        }
        let l = env.get_type(left);
        let r = env.get_type(right);

        match (l, r) {
            (Type::Var(_), _r) => replace_all(env, left, right, &mut cons, &mut root_id)?,
            (_l, Type::Var(_)) => replace_all(env, right, left, &mut cons, &mut root_id)?,
            (Type::Function(fr_a, to_a), Type::Function(fr_b, to_b)) => {
                cons.push(fr_a, fr_b);
                cons.push(to_a, to_b);
            }
            (l, r) => {
                return Err(TypeError::UnifyError {
                    left: format!("{:?}", l.debug(env)),
                    right: format!("{:?}", r.debug(env)),
                })
            }
        }
    }
    Ok(root_id)
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
        let new_var = env.new_var();
        let new_var_id = env.add_type(new_var);
        from = replace(env, var, from, new_var_id);
        to = replace(env, var, to, new_var_id);

        for c in cons.cons.iter() {
            let maybe_new_cons = Con {
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
    with: TypeId,
    cons: &mut Cons,
    root_id: &mut TypeId,
) -> Result<()> {
    if occurs(env, all_occurrences, with) {
        return Err(TypeError::InfiniteType);
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
    Ok(())
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
    fn get_vars_type_id(&self, e: &Exprs, name: InternId) -> Result<Option<TypeId>> {
        let mut iter = self.vars.iter().rev();

        if let Some(last) = iter.next() {
            match last.get(&name) {
                Some(None) => {
                    return Err(TypeError::Uninitialized {
                        name: e.get_str(name).into(),
                    });
                }

                Some(Some(val)) => return Ok(Some(*val)),
                None => (),
            }
        }

        Ok(iter.find_map(|v| v.get(&name).and_then(|t| *t)))
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

    fn push_scope(&mut self, name: InternId, value: Type) -> TypeId {
        let type_id = self.add_type(value);
        self.push_scope_with_id(name, type_id)
    }

    fn push_scope_with_id(&mut self, name: InternId, type_id: TypeId) -> TypeId {
        let mut vars = HashMap::new();

        vars.insert(name, Some(type_id));
        self.vars.push(vars);
        type_id
    }

    fn push_uninitialized_scope(&mut self, name: InternId) {
        let mut vars = HashMap::new();

        vars.insert(name, None);
        self.vars.push(vars);
    }

    fn replace_with_some(&mut self, name: InternId, type_id: TypeId) -> TypeId {
        let latest = self.vars.last_mut().expect("Last scope");
        latest.insert(name, Some(type_id)).expect("There was None");
        type_id
    }

    fn pop_scope(&mut self) {
        self.vars.pop();
    }

    fn new_var(&mut self) -> Type {
        let id = self.var_counter;
        self.var_counter += 1;
        Type::Var(id)
    }

    fn new_var_as_type(&mut self) -> TypeId {
        let ty = self.new_var();
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
    use super::*;

    mod instantiate_tests {
        use super::*;

        #[test]
        fn ident() {
            let mut env = TypeEnv::default();

            let t0 = env.new_var_as_type();
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
            let t0 = env.new_var_as_type();
            let t1 = env.new_var_as_type();

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
