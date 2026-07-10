//! Typed immutable and mutable traversal for the CSS arena AST.
//!
//! The visitor surface follows Oxc's generated `Visit`/`VisitMut` split while
//! preserving Lightning CSS's typed callbacks. Traversal is unconditional: no
//! node-type bitflags or branch masks are used.

mod generated;
mod plugin;
pub mod prelude;

pub use generated::{
    kind::AstType,
    visit::{Visit, VisitNode, walk},
    visit_mut::{VisitMut, VisitMutNode, walk as walk_mut},
};
pub use plugin::{BoxError, Plugin, PluginContext, PluginError, Plugins, VisitorPlugin};
