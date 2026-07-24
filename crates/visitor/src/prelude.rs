//! Common visitor and plugin types.

pub use crate::{
    AstType, BoxError, Plugin, PluginContext, PluginError, Plugins, Visit, VisitContext, VisitMut,
    VisitMutContext, Visitor, VisitorMut, VisitorPlugin,
};
pub use rocketcss_allocator::Allocator;
pub use rocketcss_ast::prelude::*;
