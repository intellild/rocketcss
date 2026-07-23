//! Typed immutable and mutable traversal for the CSS arena AST.
//!
//! Concrete AST nodes implement [`Visit`] and [`VisitMut`], while visitor
//! implementations customize typed callbacks through [`Visitor`] and
//! [`VisitorMut`]. Traversal is unconditional: no node-type bitflags or branch
//! masks are used.

mod plugin;
pub mod prelude;

pub use plugin::{BoxError, Plugin, PluginContext, PluginError, Plugins, VisitorPlugin};
pub use rocketcss_ast::{
    AstType, Visit, VisitContext, VisitMut, VisitMutContext, Visitor, VisitorMut,
};
