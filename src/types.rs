use std::collections::{BTreeSet, HashMap, VecDeque};
use thiserror::Error;

use crate::ast::{Expr, ExprId, Exprs};

mod debug;
pub use debug::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    Bool,
    Function(TermId, TermId),
}

#[derive(Clone, PartialEq)]
pub enum Term {
    Mono(Type),
    /// For All T, U: ...
    Poly(Vec<TermId>, TermId),
    Var(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TermId(usize);

#[derive(Default)]
pub struct TypeEnv {
    vars: Vec<HashMap<&'static str, Option<TermId>>>,
    exprs: HashMap<ExprId, TermId>,
    terms: Vec<Term>,
    var_counter: usize,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Con {
    left: TermId,
    right: TermId,
}

#[derive(Default, Debug)]
struct Cons {
    cons: VecDeque<Con>,
}

impl Cons {
    fn push(&mut self, left: TermId, right: TermId, env: &TypeEnv) {
        eprintln!("+ {:?} = {:?}", env.debug(left), env.debug(right));
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
    UndefinedVariable { name: &'static str },

    #[error("Use of uninitialized value: {name}")]
    Uninitialized { name: &'static str },

    #[error("Infinite type is not allowed")]
    InfiniteType,
}

pub type Result<T, E = TypeError> = std::result::Result<T, E>;

pub fn type_of(e: &Exprs, env: &mut TypeEnv, id: ExprId) -> Result<Term> {
    let mut cons = Default::default();
    let term_id = gather_cons(e, env, id, &mut cons)?;
    // dbg!(&env);
    // eprintln!("Cons: {cons:?}");
    let term_id = unify(env, term_id, cons)?;

    Ok(env.get_term(term_id).expect("Term"))
}

fn gather_cons(e: &Exprs, env: &mut TypeEnv, id: ExprId, cons: &mut Cons) -> Result<TermId> {
    let expr = e.get(id);
    eprintln!("Gather con: {:?}", expr.debug(e));
    Ok(match expr {
        Expr::Bool(_) => env.term_for_expr(id, Term::Mono(Type::Bool)),
        Expr::Var(name) => {
            let term_id = env
                .get_id(name)?
                .ok_or(TypeError::UndefinedVariable { name })?;
            env.term_id_for_expr(id, term_id)
        }
        Expr::Def(name, body) => {
            let var = env.new_var();
            let var = env.push(name, var);
            let ret = gather_cons(e, env, *body, cons)?;
            env.pop();
            env.term_for_expr(id, Term::Mono(Type::Function(var, ret)))
        }
        Expr::Call(func, arg) => {
            let func_term_id = gather_cons(e, env, *func, cons)?;
            let func_term = env.get_term(func_term_id).expect("Term");
            eprintln!("Function to call: {:?}", func_term.debug(env));

            let arg_id = gather_cons(e, env, *arg, cons)?;
            let (from, to) = match func_term.clone() {
                Term::Var(_) => {
                    let some_to = env.new_var_as_term();
                    let has_to_be_function = Term::Mono(Type::Function(arg_id, some_to));
                    let has_to_be_function = env.add_term(has_to_be_function);
                    cons.push(func_term_id, has_to_be_function, env);

                    (arg_id, some_to)
                }
                poly @ Term::Poly(_, _) => instantiate_poly(env, poly, cons),
                Term::Mono(Type::Function(from, to)) => (from, to),
                Term::Mono(Type::Bool) => {
                    return Err(TypeError::UnifyError {
                        left: "Fn(?, ?)".to_string(),
                        right: format!("{:?}", func_term.debug(env)),
                    });
                }
            };

            cons.push(from, arg_id, env);
            eprintln!(
                "We return Call:  {:?} with term {:?}",
                e.debug(id),
                env.debug(to)
            );
            env.term_id_for_expr(id, to)
        }
        Expr::Let(name, value_id, then) => {
            env.push_none(name);

            let value = gather_cons(e, env, *value_id, cons)?;

            let value_type = env.get_term(value).expect("Type");
            eprintln!("Let value: {:?}", value_type.debug(env));

            let value = match value_type {
                Term::Mono(Type::Function(from, to)) => {
                    let terms = [from, to];
                    let poly_var = terms
                        .into_iter()
                        .flat_map(|t| collect_vars(env, t))
                        .collect::<BTreeSet<_>>();

                    if !poly_var.is_empty() {
                        let new_poly = Term::Poly(poly_var.into_iter().collect(), value);

                        eprintln!("NewPoly: {:?}", new_poly.debug(env));
                        env.term_for_expr(*value_id, new_poly)
                    } else {
                        value
                    }
                }
                _ => value,
            };

            env.replace_with_some(name, value);

            let then = gather_cons(e, env, *then, cons)?;
            env.pop();
            env.term_id_for_expr(id, then)
        }
    })
}

fn collect_vars(env: &TypeEnv, id: TermId) -> Vec<TermId> {
    let mut vars: Vec<TermId> = vec![];
    let mut queue: VecDeque<TermId> = Default::default();
    queue.push_back(id);

    while let Some(id) = queue.pop_front() {
        match env.get_term(id).expect("Term") {
            Term::Mono(Type::Bool) => (),
            Term::Mono(Type::Function(from, to)) => {
                queue.push_back(from);
                queue.push_back(to);
            }
            Term::Poly(_, _) => (),
            Term::Var(_) => {
                vars.push(id);
            }
        }
    }
    vars
}

fn unify(env: &mut TypeEnv, mut root_id: TermId, mut cons: Cons) -> Result<TermId> {
    while let Some(Con { left, right }) = cons.pop() {
        if left == right {
            continue;
        }
        let l = env.get_term(left).expect("Term");
        let r = env.get_term(right).expect("Term");

        match (l, r) {
            (Term::Var(_), _r) => {
                replace_all(env, left, right, &mut cons, &mut root_id)?;
                continue;
            }
            (_l, Term::Var(_)) => {
                replace_all(env, right, left, &mut cons, &mut root_id)?;
                continue;
            }
            (Term::Mono(Type::Function(fr_a, to_a)), Term::Mono(Type::Function(fr_b, to_b))) => {
                cons.push(fr_a, fr_b, env);
                cons.push(to_a, to_b, env);
                continue;
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

fn instantiate_poly(env: &mut TypeEnv, poly: Term, cons: &mut Cons) -> (TermId, TermId) {
    match poly.clone() {
        Term::Poly(vars, poly_term) => {
            let inner = env.get_term(poly_term).expect("Term");
            match inner {
                Term::Mono(Type::Function(from, to)) => {
                    eprintln!("Instantiate starts...");
                    let (f, t) = instantiate(env, vars, from, to, cons);
                    eprintln!(
                        "Instantiate {:?} into {:?} -> {:?}",
                        poly.debug(env),
                        env.debug(f),
                        env.debug(t)
                    );
                    (f, t)
                }
                Term::Poly(_, _) => panic!("Higher order polymorphism is not supported"),
                Term::Mono(Type::Bool) | Term::Var(_) => panic!("Expected function"),
            }
        }
        _ => unreachable!(),
    }
}

fn instantiate(
    env: &mut TypeEnv,
    vars: Vec<TermId>,
    mut from: TermId,
    mut to: TermId,
    cons: &mut Cons,
) -> (TermId, TermId) {
    let mut new_cons = vec![];
    for var in vars.into_iter().rev() {
        let new_var = env.new_var();
        let new_var_id = env.add_term(new_var);
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
    (from, to)
}

fn replace_all(
    env: &mut TypeEnv,
    left: TermId,
    right: TermId,
    cons: &mut Cons,
    root_id: &mut TermId,
) -> Result<()> {
    if occurs(env, left, right) {
        return Err(TypeError::InfiniteType);
    }

    for c in cons.cons.iter_mut() {
        c.left = replace(env, left, c.left, right);
        c.right = replace(env, left, c.right, right);
    }

    let mut exprs = std::mem::take(&mut env.exprs);

    for (_e_id, term_id) in exprs.iter_mut() {
        *term_id = replace(env, left, *term_id, right);
    }
    env.exprs = exprs;

    *root_id = replace(env, left, *root_id, right);
    Ok(())
}

fn occurs(env: &mut TypeEnv, term: TermId, inside: TermId) -> bool {
    if inside == term {
        return true;
    }
    let inside = env.get_term(inside).expect("Term");
    match inside {
        Term::Mono(Type::Bool) => false,
        Term::Mono(Type::Function(arg, ret)) => occurs(env, term, arg) || occurs(env, term, ret),
        Term::Poly(vars, inside) => {
            vars.iter().any(|v| occurs(env, term, *v)) || occurs(env, term, inside)
        }
        Term::Var(_) => false,
    }
}

fn replace(env: &mut TypeEnv, left: TermId, term_id: TermId, right: TermId) -> TermId {
    // eprintln!(
    //     "Replace: {:?} -- {:?} --> {:?}",
    //     env.debug(left),
    //     env.debug(term_id),
    //     env.debug(right)
    // );

    let term = env.get_term(term_id).expect("Term");
    match term {
        Term::Mono(Type::Function(arg, ret)) => {
            let arg = replace(env, left, arg, right);
            let ret = replace(env, left, ret, right);

            env.add_term(Term::Mono(Type::Function(arg, ret)))
        }
        _ => {
            if left == term_id {
                right
            } else {
                term_id
            }
        }
    }
}

impl TypeEnv {
    fn get_id(&self, name: &'static str) -> Result<Option<TermId>> {
        let mut iter = self.vars.iter().rev();

        if let Some(last) = iter.next() {
            match last.get(name) {
                Some(None) => {
                    return Err(TypeError::Uninitialized { name });
                }

                Some(Some(val)) => return Ok(Some(*val)),
                None => (),
            }
        }

        Ok(iter.find_map(|v| v.get(name).and_then(|t| *t)))
    }

    fn get_term(&self, id: TermId) -> Option<Term> {
        self.terms.get(id.0).cloned()
    }

    fn add_term(&mut self, term: Term) -> TermId {
        if let Some(t_id) = self
            .terms
            .iter()
            .enumerate()
            .find(|(_, t)| t == &&term)
            .map(|(id, _)| id)
        {
            return TermId(t_id);
        }

        let id = TermId(self.terms.len());
        self.terms.push(term);
        id
    }

    fn term_id_of(&self, id: ExprId) -> Option<TermId> {
        self.exprs.get(&id).cloned()
    }

    fn term_id_for_expr(&mut self, id: ExprId, term_id: TermId) -> TermId {
        self.exprs.insert(id, term_id);
        term_id
    }
    fn term_for_expr(&mut self, id: ExprId, term: Term) -> TermId {
        let term_id = self.add_term(term);
        self.term_id_for_expr(id, term_id)
    }

    fn push(&mut self, name: &'static str, value: Term) -> TermId {
        let term_id = self.add_term(value);
        self.push_id(name, term_id)
    }

    fn push_id(&mut self, name: &'static str, term_id: TermId) -> TermId {
        let mut vars = HashMap::new();

        vars.insert(name, Some(term_id));
        self.vars.push(vars);
        term_id
    }

    fn push_none(&mut self, name: &'static str) {
        let mut vars = HashMap::new();

        vars.insert(name, None);
        self.vars.push(vars);
    }

    fn replace_with_some(&mut self, name: &'static str, term_id: TermId) -> TermId {
        let latest = self.vars.last_mut().expect("Last scope");
        latest.insert(name, Some(term_id)).expect("There was None");
        term_id
    }

    fn pop(&mut self) {
        self.vars.pop();
    }

    fn new_var(&mut self) -> Term {
        let id = self.var_counter;
        self.var_counter += 1;
        Term::Var(id)
    }

    fn new_var_as_term(&mut self) -> TermId {
        let term = self.new_var();
        self.add_term(term)
    }

    fn print_term_id(&self, id: TermId) -> String {
        let term = self.get_term(id).expect("Term");
        self.print_term(term)
    }

    pub fn print_term(&self, term: Term) -> String {
        match term {
            Term::Mono(ty) => self.print(ty),
            Term::Var(i) => format!("T{i}"),
            Term::Poly(vars, ty) => {
                let vars = vars
                    .iter()
                    .copied()
                    .map(|v| self.print_term_id(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("ForAll ({vars}): {}", self.print_term_id(ty))
            }
        }
    }

    pub fn print(&self, ty: Type) -> String {
        match ty {
            Type::Bool => "Bool".to_owned(),
            Type::Function(from, to) => {
                let from = self.print_term_id(from);
                let to = self.print_term_id(to);
                format!("({from} -> {to})")
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
            let mut cons = Cons::default();

            let t0 = env.new_var_as_term();
            let source = Term::Poly(vec![t0], env.add_term(Term::Mono(Type::Function(t0, t0))));

            let (a_from, a_to) = instantiate_poly(&mut env, source, &mut cons);
            let actual = Term::Mono(Type::Function(a_from, a_to));

            let t1 = TermId(2);
            let expected = Term::Mono(Type::Function(t1, t1));

            assert_eq!(expected.debug(&env), actual.debug(&env));
        }

        #[test]
        fn nested() {
            let mut env = TypeEnv::default();
            let mut cons = Cons::default();
            let t0 = env.new_var_as_term();
            let t1 = env.new_var_as_term();

            let t2 = env.add_term(Term::Mono(Type::Function(t1, t0)));
            let source = Term::Poly(
                vec![t0, t1],
                env.add_term(Term::Mono(Type::Function(t0, t2))),
            );

            let (a_from, a_to) = instantiate_poly(&mut env, source, &mut cons);
            let actual = Term::Mono(Type::Function(a_from, a_to));

            let t3 = TermId(4);
            let t4 = TermId(6);
            let expected_inner = env.add_term(Term::Mono(Type::Function(t3, t4)));
            let expected = Term::Mono(Type::Function(t4, expected_inner));

            println!("{:#?}", env);

            assert_eq!(expected.debug(&env), actual.debug(&env));
        }
    }
}
