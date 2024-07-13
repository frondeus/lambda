use std::sync::Arc;

// pub struct Source {}
#[derive(Debug, PartialEq, Clone)]
pub struct Spanned<T> {
    pub range: tree_sitter::Range,
    pub filename: Arc<str>,
    pub source: Arc<str>,
    pub node: T,
}

impl<T: WithRange> Spanned<T> {
    pub fn map<O: WithRange>(self, f: impl FnOnce(T) -> O) -> Spanned<O> {
        let o = f(self.node);
        Spanned {
            range: o.range(),
            filename: self.filename.clone(),
            source: self.source.clone(),
            node: o,
        }
    }
}

pub trait WithRange {
    fn range(&self) -> tree_sitter::Range;
}
impl WithRange for tree_sitter::Node<'_> {
    fn range(&self) -> tree_sitter::Range {
        self.range()
    }
}
