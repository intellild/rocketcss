//! Common visitor and plugin types.

pub use crate::{
    AstType, BoxError, Plugin, PluginContext, PluginError, Plugins, Visit, VisitMut, VisitorPlugin,
    walk, walk_mut,
};
pub use rocketcss_allocator::Allocator;
pub use rocketcss_ast::prelude::*;
