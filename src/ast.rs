use std::collections::BTreeMap;
pub use tree_sitter::Node as SyntaxNode;
pub use tree_sitter::Tree as SyntaxTree;

pub mod arbitrary;
pub mod builder;
pub mod from_cst;
pub mod queries;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default, Debug)]
pub struct InternId(usize);

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Bool {
        value: bool,
        node: Option<SyntaxNode<'a>>,
    },
    Var {
        name: InternId,
        node: Option<SyntaxNode<'a>>,
    }, // "x"
    VarDef {
        name: InternId,
        node: Option<SyntaxNode<'a>>,
    },
    Def {
        arg: ExprId,
        body: ExprId,
        node: Option<SyntaxNode<'a>>,
    }, // "fn x: x"
    Call {
        func: ExprId,
        arg: ExprId,
        node: Option<SyntaxNode<'a>>,
    }, // x(y)
    Let {
        name: ExprId,
        value: ExprId,
        body: ExprId,
        node: Option<SyntaxNode<'a>>,
    }, // let x = 0; x
}

pub fn var_def_to_str<'a>(e: &'a Exprs<'a>, id: ExprId) -> &'a str {
    match e.get(id) {
        Expr::VarDef { name, node: _ } => e.get_str(*name),
        e => unreachable!("{e:?} is not VarDef"),
    }
}
pub fn var_def_to_intern(e: &Exprs, id: ExprId) -> InternId {
    match e.get(id) {
        Expr::VarDef { name, node: _ } => *name,
        e => unreachable!("{e:?} is not VarDef"),
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct Exprs<'a> {
    e: Vec<Expr<'a>>,
    i_to_s: BTreeMap<InternId, String>,
    s_to_i: BTreeMap<String, InternId>,
    intern_counter: InternId,
}

impl<'a> Exprs<'a> {
    pub fn push(&mut self, e: Expr<'a>) -> ExprId {
        if let Some(id) = self
            .e
            .iter()
            .enumerate()
            .find(|(_, en)| en == &&e)
            .map(|(id, _)| ExprId(id))
        {
            return id;
        }
        let id = ExprId(self.e.len());
        self.e.push(e);
        id
    }

    pub fn push_str(&mut self, s: impl ToString) -> InternId {
        let id = *self.s_to_i.entry(s.to_string()).or_insert_with(|| {
            let id = self.intern_counter;
            self.intern_counter.0 += 1;
            id
        });
        self.i_to_s.insert(id, s.to_string());
        id
    }

    pub fn get_str(&self, id: InternId) -> &str {
        self.i_to_s[&id].as_str()
    }

    pub fn get(&self, id: ExprId) -> &Expr {
        &self.e[id.0]
    }

    pub fn debug(&self, root: ExprId) -> DebugExpr {
        self.get(root).debug(self)
    }
}

#[derive(PartialEq)]
pub struct DebugExpr<'a> {
    ex: &'a Exprs<'a>,
    e: &'a Expr<'a>,
}
impl<'a> Expr<'a> {
    pub fn debug(&'a self, e: &'a Exprs) -> DebugExpr<'a> {
        DebugExpr { e: self, ex: e }
    }
}
impl std::fmt::Debug for ExprId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$e{}", self.0)
    }
}
impl<'a> std::fmt::Debug for DebugExpr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.e {
            Expr::Bool { value: b, node: _ } => f.debug_tuple("Bool").field(b).finish(),
            Expr::Var { name: v, node: _ } => write!(f, "{}", self.ex.get_str(*v)),
            Expr::VarDef { name, node: _ } => write!(f, "Var({})", self.ex.get_str(*name)),
            Expr::Def {
                arg,
                body: ret,
                node: _,
            } => f
                .debug_tuple("Def")
                .field(&self.ex.debug(*arg))
                .field(&self.ex.debug(*ret))
                .finish(),
            Expr::Call {
                func: fun,
                arg,
                node: _,
            } => f
                .debug_tuple("Call")
                .field(&self.ex.debug(*fun))
                .field(&self.ex.debug(*arg))
                .finish(),
            Expr::Let {
                name,
                value,
                body: then,
                node: _,
            } => f
                .debug_tuple("Let")
                .field(&self.ex.debug(*name))
                .field(&self.ex.debug(*value))
                .field(&self.ex.debug(*then))
                .finish(),
        }
    }
}
