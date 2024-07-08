use std::sync::Arc;

// pub struct Source {}
pub struct Spanned<T> {
    pub range: tree_sitter::Range,
    pub file: Arc<str>,
    pub node: T,
}
