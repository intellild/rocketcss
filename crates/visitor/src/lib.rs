//! Typed immutable and mutable traversal for the CSS arena AST.
//!
//! Concrete AST nodes implement [`Visit`] and [`VisitMut`], while visitor
//! implementations customize typed callbacks through [`Visitor`] and
//! [`VisitorMut`]. Apply [`visitor`] to an implementation so traversal can skip
//! callbacks that use their default implementation.

extern crate self as rocketcss_visitor;

mod plugin;
pub mod prelude;

pub use plugin::{BoxError, Plugin, PluginContext, PluginError, Plugins, VisitorPlugin};
pub use rocketcss_ast::{AstType, Visit, VisitMut, Visitor, VisitorMethods, VisitorMut};
pub use rocketcss_macros::visitor;
