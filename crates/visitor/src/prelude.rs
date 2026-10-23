//! Common visitor and plugin types.

pub use crate::{
    BoxError, Plugin, PluginContext, PluginError, Plugins, VisitMutContext, Visitor, VisitorMut,
    VisitorPlugin, visit_compilation, visit_compilation_mut,
};
pub use rocketcss_ast_next::*;
pub use rocketcss_common::prelude::*;
