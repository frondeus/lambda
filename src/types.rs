use std::collections::{BTreeSet, HashMap, HashSet};

use crate::ast::{Expr, ExprId, Exprs};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Bool,
    Function(TermId, TermId),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Mono(Type),
    Poly(Vec<TermId>, TermId), // For All T, U: ...
    Var(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TermId(usize);

#[derive(Default)]
pub struct TypeEnv {
    vars: Vec<HashMap<&'static str, TermId>>,
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
    cons: HashSet<Con>,
}

impl Cons {
    fn push(&mut self, con: Con) {
        if con.left == con.right {
            return;
        }
        // eprintln!("Insert {con:?}");
        self.cons.insert(con);
    }

    fn pop(&mut self) -> Option<Con> {
        let cons = std::mem::take(&mut self.cons);
        let mut cons = cons.into_iter().collect::<Vec<_>>();
        let con = cons.pop();
        self.cons = cons.into_iter().collect();
        con
    }
}

pub fn type_of(e: &Exprs, env: &mut TypeEnv, id: ExprId) -> Term {
    let mut cons = Default::default();
    let term_id = gather_cons(e, env, id, &mut cons);
    // eprintln!("Before unify:");
    // eprintln!("Cons: {cons:?}");
    expr_print(e, id, env);
    let term_id = unify(env, term_id, cons);

    env.get_term(term_id).expect("Term")
}

fn gather_cons(e: &Exprs, env: &mut TypeEnv, id: ExprId, cons: &mut Cons) -> TermId {
    let expr = e.get(id);
    // eprintln!("* Gather cons: {id:?} {expr:?}");
    match expr {
        Expr::Bool(_) => env.term_for_expr(id, Term::Mono(Type::Bool)),
        Expr::Var(name) => {
            let term_id = env.get_id(name).expect("Undefined var");
            env.term_id_for_expr(id, term_id)
        }
        Expr::Def(name, body) => {
            let var = env.new_var();
            let var = env.push(name, var);
            let ret = gather_cons(e, env, *body, cons);
            env.pop();
            env.term_for_expr(id, Term::Mono(Type::Function(var, ret)))
        }
        Expr::Call(func, arg) => {
            // Func term
            let term_id = gather_cons(e, env, *func, cons);
            let term = env.get_term(term_id).expect("Term");
            // eprintln!("{id:?} - Call term: {term:?}");
            // Currently there is no let polymorphism

            let arg_id = gather_cons(e, env, *arg, cons);
            let (from, to) = match term {
                Term::Var(_) => {
                    // eprintln!("{id:?} - Call term: {term:?} is a variable");
                    let some_to = env.new_var_as_term();
                    let has_to_be_function = Term::Mono(Type::Function(arg_id, some_to));
                    // eprintln!("{id:?} - term_id {term_id:?} must be fn: {has_to_be_function:?}");
                    let has_to_be_function = env.add_term(has_to_be_function);
                    // dbg!(&term_id, has_to_be_function);
                    cons.push(Con {
                        left: term_id,
                        right: has_to_be_function,
                    });

                    (arg_id, some_to)
                }
                Term::Poly(vars, term) => {
                    let inner = env.get_term(term).expect("Term");
                    match inner {
                        Term::Mono(Type::Function(from, to)) => instantiate(env, vars, from, to),
                        Term::Poly(_, _) => panic!("Higher order polymorphism is not supported"),
                        Term::Mono(Type::Bool) | Term::Var(_) => panic!("Expected function"),
                    }
                }
                Term::Mono(Type::Function(from, to)) => (from, to),
                Term::Mono(Type::Bool) => {
                    panic!("Expected function, found {term:?}")
                }
            };

            // dbg!(&from, &to);
            // eprintln!("{id:?} - From must be equal to arg_id");
            cons.push(Con {
                left: from,
                right: arg_id,
            });
            env.term_id_for_expr(id, to)
        }
        Expr::Let(name, value_id, then) => {
            let value = gather_cons(e, env, *value_id, cons);
            let value_type = env.get_term(value).expect("Type");

            let value = match value_type {
                Term::Mono(Type::Function(from, to)) => {
                    let terms = [from, to];
                    let poly_var = terms
                        .into_iter()
                        .flat_map(|t| env.get_term(t).map(|term| (t, term)))
                        .filter(|(_term_id, term)| matches!(term, Term::Var(_)))
                        .map(|(term_id, _)| term_id)
                        .collect::<BTreeSet<_>>();

                    if !poly_var.is_empty() {
                        env.term_for_expr(
                            *value_id,
                            Term::Poly(poly_var.into_iter().collect(), value),
                        )
                    } else {
                        value
                    }
                }
                _ => value,
            };

            env.push_id(name, value);

            let then = gather_cons(e, env, *then, cons);
            env.pop();
            env.term_id_for_expr(id, then)
        }
    }
}

fn unify(env: &mut TypeEnv, mut root_id: TermId, mut cons: Cons) -> TermId {
    while let Some(Con { left, right }) = cons.pop() {
        if left == right {
            continue;
        }
        let l = env.get_term(left).expect("Term");
        let r = env.get_term(right).expect("Term");

        match (l, r) {
            (Term::Var(_), _r) => {
                replace_all(env, left, right, &mut cons, &mut root_id);
                continue;
            }
            (_l, Term::Var(_)) => {
                replace_all(env, right, left, &mut cons, &mut root_id);
                continue;
            }
            (Term::Mono(Type::Function(fr_a, to_a)), Term::Mono(Type::Function(fr_b, to_b))) => {
                todo!()
            }
            (l, r) => panic!("Does not unify: {l:?} {r:?}"),
        }
    }
    root_id
}

fn instantiate(
    env: &mut TypeEnv,
    vars: Vec<TermId>,
    mut from: TermId,
    mut to: TermId,
) -> (TermId, TermId) {
    // eprintln!("Instantiate {vars:?} {from:?} -> {to:?}");
    for var in vars {
        let new_var = env.new_var();
        let new_var_id = env.add_term(new_var);
        from = replace(env, from, var, new_var_id);
        to = replace(env, var, to, new_var_id);
    }
    // eprintln!("Instantiate completed: {from:?} -> {to:?}");
    (from, to)
}

fn replace_all(
    env: &mut TypeEnv,
    left: TermId,
    right: TermId,
    cons: &mut Cons,
    root_id: &mut TermId,
) {
    let l_debug = env.get_term(left).unwrap();
    let r_debug = env.get_term(right).unwrap();
    // eprintln!("Replace all {left:?} ({l_debug:?}) with {right:?} ({r_debug:?})");

    let mut cons_vec = std::mem::take(&mut cons.cons)
        .into_iter()
        .collect::<Vec<_>>();
    for c in cons_vec.iter_mut() {
        c.left = replace(env, left, c.left, right);
        c.right = replace(env, left, c.right, right);
    }

    let mut exprs = std::mem::take(&mut env.exprs);

    for (_e_id, term_id) in exprs.iter_mut() {
        *term_id = replace(env, left, *term_id, right);
    }
    env.exprs = exprs;

    *root_id = replace(env, left, *root_id, right);

    // eprintln!("Replace done");
    cons.cons = cons_vec.into_iter().collect();
}

fn replace(env: &mut TypeEnv, left: TermId, term_id: TermId, right: TermId) -> TermId {
    // eprintln!("Replace {left:?} with {right:?} (in {term_id:?})");

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
    fn get_id(&self, name: &'static str) -> Option<TermId> {
        self.vars.iter().rev().find_map(|v| v.get(name)).copied()
    }

    // fn get(&self, name: &'static str) -> Option<Term> {
    //     let id = self.get_id(name)?;

    //     self.get_term(id)
    // }

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

    fn term_of(&self, id: ExprId) -> Option<Term> {
        let id = self.term_id_of(id)?;
        self.get_term(id)
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

        vars.insert(name, term_id);
        self.vars.push(vars);
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

impl std::fmt::Debug for TermId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$t{}", self.0)
    }
}
impl std::fmt::Debug for TypeEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (e_id, e) in self.exprs.iter() {
            writeln!(f, "| {e_id:?} | {e:?} |")?;
        }
        writeln!(f, "---")?;
        for (t_id, t) in self.terms.iter().enumerate() {
            writeln!(f, "| {:?} | {t:?} |", TermId(t_id))?;
        }
        Ok(())
    }
}
impl std::fmt::Debug for Con {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} = {:?}", self.left, self.right)
    }
}

pub fn expr_print(exprs: &Exprs, id: ExprId, env: &TypeEnv) {
    fn expr_print_inner(
        exprs: &Exprs,
        id: ExprId,
        env: &TypeEnv,
        indent: usize,
        ignore_indent: bool,
    ) {
        let term_id = env.term_id_of(id).expect("Term");
        let term = env.print_term_id(term_id);
        let print_indent = |indent, ignore_indent: bool| {
            if !ignore_indent {
                print!("{ }", " ".repeat(indent * 4));
            }
        };
        print_indent(indent, ignore_indent);
        match exprs.get(id) {
            Expr::Bool(true) => println!("true # {term} \t {term_id:?}"),
            Expr::Bool(false) => println!("false # {term} \t {term_id:?}"),
            Expr::Var(name) => println!("{name} # {term} \t {term_id:?}"),
            Expr::Def(arg, ret) => {
                println!("fn ({arg}) => # {term} \t {term_id:?}");
                expr_print_inner(exprs, *ret, env, indent + 1, false);
            }
            Expr::Call(f, a) => {
                expr_print_inner(exprs, *f, env, indent, true);
                print_indent(indent, false);
                println!("(");
                expr_print_inner(exprs, *a, env, indent + 1, false);
                print_indent(indent, false);
                println!(") # {term} \t {term_id:?}");
            }
            Expr::Let(name, value, then) => {
                print!("let {name} = ");
                expr_print_inner(exprs, *value, env, indent, true);
                print_indent(indent, ignore_indent);
                println!("in");
                expr_print_inner(exprs, *then, env, indent + 1, false);
            }
        }
    }

    expr_print_inner(exprs, id, env, 0, false);
}
