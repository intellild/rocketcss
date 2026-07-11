#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitMutNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_span<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Span)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Span);
    visitor.leave_node(AstType::Span);
}
