//! Typed immutable and mutable traversal for the CSS arena AST.
//!
//! Concrete AST nodes implement [`Visit`] and [`VisitMut`], while visitor
//! implementations customize typed callbacks through [`Visitor`] and
//! [`VisitorMut`]. Traversal is unconditional: no node-type bitflags or branch
//! masks are used.

mod generated;
mod plugin;
pub mod prelude;

pub use generated::{
    kind::AstType,
    visit::{Visit, Visitor},
    visit_mut::{VisitMut, VisitorMut},
};
pub use plugin::{BoxError, Plugin, PluginContext, PluginError, Plugins, VisitorPlugin};
