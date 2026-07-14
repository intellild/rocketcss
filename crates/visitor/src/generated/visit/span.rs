#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for Span {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_span(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Span);
        let node = self;
        visitor.leave_node(AstType::Span);
    }
}
