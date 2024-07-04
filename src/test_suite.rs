use lambda::ast::builder::*;
use lambda::runtime::{eval, Value};
use lambda::types::{DebugTypeEnv, Type};

pub struct Test<'t> {
    exprs: lambda::ast::Exprs<'t>,
    root: lambda::ast::ExprId,
    types: lambda::types::TypeEnv,
    rt: lambda::runtime::RunEnv,
    ty: lambda::types::Result<lambda::types::Type>,
}

pub trait TestExt<'t>: BuilderFn<'t> + Sized {
    fn test(self) -> Test<'t> {
        let (root, exprs) = self.root();
        let mut types = Default::default();
        let rt = Default::default();
        let ty = lambda::types::type_of(&exprs, &mut types, root);
        Test {
            exprs,
            root,
            rt,
            types,
            ty,
        }
    }
}
impl<'t, B: BuilderFn<'t>> TestExt<'t> for B {}
impl<'t> Test<'t> {
    #[track_caller]
    pub fn eval(&mut self) -> &mut Self {
        eval(&self.exprs, &mut self.rt, self.root);
        self
    }
    #[track_caller]
    pub fn assert_error(&mut self, expected: &str) -> &mut Self {
        let ty = self
            .ty
            .as_ref()
            .map(|t| t.debug(&self.types))
            .expect_err("Expected type error, but found type");
        assert_eq!(expected.trim(), format!("{ty}"));
        self
    }
    #[track_caller]
    pub fn assert_eval(&mut self, expected: Value) -> &mut Self {
        let ret = eval(&self.exprs, &mut self.rt, self.root);
        assert_eq!(expected, ret);
        self
    }
    #[track_caller]
    pub fn assert_type(&mut self, expected: Type) -> &mut Self {
        let ty = self.ty.as_ref().expect("Valid type");
        assert_eq!(expected.debug(&self.types), ty.debug(&self.types));
        self
    }
    #[track_caller]
    pub fn assert_print_type(&mut self, expected: &str) -> &mut Self {
        let ty = self.ty.as_ref().expect("Valid type");
        assert_eq!(expected.trim(), self.types.print_type(ty.clone()));
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

    pub fn dbg_type(&mut self) -> &mut Self {
        if let Ok(ty) = self.ty.as_ref() {
            eprintln!("{:?}", ty.debug(&self.types));
        }
        self
    }
}
