//! Common visitor and plugin types.

pub use crate::{
    AstType, BoxError, Plugin, PluginContext, PluginError, Plugins, Visit, VisitMut, Visitor,
    VisitorMethods, VisitorMut, VisitorPlugin, visitor,
};
pub use rocketcss_allocator::Allocator;
pub use rocketcss_ast::prelude::*;
