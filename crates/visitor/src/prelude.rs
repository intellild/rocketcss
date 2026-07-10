//! Common visitor and plugin types.

pub use crate::{
    AstType, BoxError, Plugin, PluginContext, PluginError, Plugins, Visit, VisitMut, VisitorPlugin,
    walk, walk_mut,
};
pub use rs_css_allocator::Allocator;
pub use rs_css_ast::prelude::*;
