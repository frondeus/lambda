use lambda::ast::builder::*;
use lambda::runtime::{eval, Value};
use lambda::types::{DebugTypeEnv, Term};

pub struct Test {
    exprs: lambda::ast::Exprs,
    root: lambda::ast::ExprId,
    types: lambda::types::TypeEnv,
    rt: lambda::runtime::RunEnv,
    term: lambda::types::Result<lambda::types::Term>,
}

pub trait TestExt: BuilderFn + Sized {
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
    pub fn eval(&mut self) -> &mut Self {
        eval(&self.exprs, &mut self.rt, self.root);
        self
    }
    #[track_caller]
    pub fn assert_error(&mut self, expected: &str) -> &mut Self {
        let term = self
            .term
            .as_ref()
            .map(|t| t.debug(&self.types))
            .expect_err("Expected type error");
        assert_eq!(expected.trim(), format!("{term}"));
        self
    }
    #[track_caller]
    pub fn assert_eval(&mut self, expected: Value) -> &mut Self {
        let ret = eval(&self.exprs, &mut self.rt, self.root);
        assert_eq!(expected, ret);
        self
    }
    #[track_caller]
    pub fn assert_term(&mut self, expected: Term) -> &mut Self {
        let term = self.term.as_ref().expect("Valid type");
        assert_eq!(expected.debug(&self.types), term.debug(&self.types));
        self
    }
    #[track_caller]
    pub fn assert_print_term(&mut self, expected: &str) -> &mut Self {
        let term = self.term.as_ref().expect("Valid type");
        assert_eq!(expected.trim(), self.types.print_term(term.clone()));
        self
    }
    #[track_caller]
    pub fn assert_print_eval(&mut self, expected: &str) -> &mut Self {
        let ret = eval(&self.exprs, &mut self.rt, self.root);
        assert_eq!(expected, ret.to_string());
        self
    }
    pub fn dbg_env(&mut self) -> &mut Self {
        eprintln!(
            "{:#?}",
            DebugTypeEnv {
                types: &self.types,
                exprs: &self.exprs
            }
        );

        self
    }

    pub fn dbg_tree(&mut self) -> &mut Self {
        // expr_print(&self.exprs, self.root, &self.types);
        eprintln!("{:#?}", self.exprs.debug(self.root));
        self
    }

    pub fn dbg_term(&mut self) -> &mut Self {
        if let Ok(term) = self.term.as_ref() {
            eprintln!("{:?}", term.debug(&self.types));
        }
        self
    }
}
