use std::sync::Arc;

use tree_sitter::Range;

// pub struct Source {}
#[derive(Debug, PartialEq, Clone)]
pub struct Spanned<T> {
    pub range: Range,
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

impl<T> Spanned<Option<T>> {
    pub fn transpose(self) -> Option<Spanned<T>> {
        self.node.map(|node| Spanned {
            range: self.range,
            filename: self.filename,
            source: self.source,
            node,
        })
    }
}

pub trait WithRange {
    fn range(&self) -> Range;
}
impl WithRange for tree_sitter::Node<'_> {
    fn range(&self) -> Range {
        self.range()
    }
}
impl WithRange for Option<tree_sitter::Node<'_>> {
    fn range(&self) -> Range {
        self.map(|n| n.range()).unwrap_or_else(default_range)
    }
}

fn default_range() -> Range {
    Range {
        start_byte: Default::default(),
        end_byte: Default::default(),
        start_point: Default::default(),
        end_point: Default::default(),
    }
}
