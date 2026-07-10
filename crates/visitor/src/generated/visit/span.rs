#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, VisitNode};
use crate::AstType;
use rs_css_ast::*;
pub fn walk_span<'a, VisitorT>(visitor: &mut VisitorT, node: &Span)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Span);
    visitor.leave_node(AstType::Span);
}
