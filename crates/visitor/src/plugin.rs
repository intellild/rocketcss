use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::Error,
    fmt,
};

use rocketcss_allocator::{Allocator, GhostToken};
use rocketcss_ast::{StyleSheet, VisitMutContext};

use crate::{VisitMut, VisitorMut};

/// Type-erased error returned by a plugin.
pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

/// Shared services available to every plugin in a pipeline.
pub struct PluginContext<'a, 'token, 'ghost> {
    allocator: &'a Allocator,
    token: &'token mut GhostToken<'ghost>,
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl<'a, 'token, 'ghost> PluginContext<'a, 'token, 'ghost> {
    #[inline]
    pub fn new(allocator: &'a Allocator, token: &'token mut GhostToken<'ghost>) -> Self {
        Self {
            allocator,
            token,
            data: HashMap::new(),
        }
    }

    /// Returns the arena that owns the stylesheet being transformed.
    #[inline]
    pub fn allocator(&self) -> &'a Allocator {
        self.allocator
    }

    #[inline]
    pub fn ghost_token(&mut self) -> &mut GhostToken<'ghost> {
        self.token
    }

    /// Inserts shared typed state, returning the previous value of that type.
    pub fn insert<T: Any>(&mut self, value: T) -> Option<T> {
        self.data
            .insert(TypeId::of::<T>(), Box::new(value))
            .and_then(|value| value.downcast::<T>().ok())
            .map(|value| *value)
    }

    #[inline]
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref()
    }

    #[inline]
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.data.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }

    pub fn remove<T: Any>(&mut self) -> Option<T> {
        self.data
            .remove(&TypeId::of::<T>())?
            .downcast::<T>()
            .ok()
            .map(|value| *value)
    }
}

/// A stylesheet transform that can participate in a [`Plugins`] pipeline.
///
/// The trait is object-safe so independently implemented plugins can be
/// selected at runtime. Plugins that only need a typed mutable visitor can use
/// [`VisitorPlugin`] or [`Plugins::add_visitor`].
pub trait Plugin<'a, 'ghost> {
    fn name(&self) -> &str;

    fn transform(
        &mut self,
        stylesheet: &mut StyleSheet<'a, 'ghost>,
        context: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError>;
}

/// Runs plugins in registration order over one stylesheet.
pub struct Plugins<'plugin, 'a, 'ghost> {
    plugins: Vec<Box<dyn Plugin<'a, 'ghost> + 'plugin>>,
}

impl<'plugin, 'a, 'ghost> Default for Plugins<'plugin, 'a, 'ghost> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'plugin, 'a, 'ghost> Plugins<'plugin, 'a, 'ghost> {
    #[inline]
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn add<P>(&mut self, plugin: P)
    where
        P: Plugin<'a, 'ghost> + 'plugin,
    {
        self.plugins.push(Box::new(plugin));
    }

    pub fn add_visitor<V>(&mut self, name: &'static str, visitor: V)
    where
        V: VisitorMut<'a, 'ghost> + 'plugin,
    {
        self.add(VisitorPlugin::new(name, visitor));
    }

    pub fn run(
        &mut self,
        stylesheet: &mut StyleSheet<'a, 'ghost>,
        context: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), PluginError> {
        for plugin in &mut self.plugins {
            plugin
                .transform(stylesheet, context)
                .map_err(|source| PluginError {
                    plugin: plugin.name().to_owned(),
                    source,
                })?;
        }
        Ok(())
    }
}

/// Adapts an infallible [`VisitorMut`] implementation into a dynamic plugin.
pub struct VisitorPlugin<V> {
    name: &'static str,
    visitor: V,
}

impl<V> VisitorPlugin<V> {
    #[inline]
    pub fn new(name: &'static str, visitor: V) -> Self {
        Self { name, visitor }
    }

    #[inline]
    pub fn visitor(&self) -> &V {
        &self.visitor
    }

    #[inline]
    pub fn visitor_mut(&mut self) -> &mut V {
        &mut self.visitor
    }

    #[inline]
    pub fn into_visitor(self) -> V {
        self.visitor
    }
}

impl<'a, 'ghost, V: VisitorMut<'a, 'ghost>> Plugin<'a, 'ghost> for VisitorPlugin<V> {
    #[inline]
    fn name(&self) -> &str {
        self.name
    }

    fn transform(
        &mut self,
        stylesheet: &mut StyleSheet<'a, 'ghost>,
        context: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError> {
        let mut visit_context = VisitMutContext::new(context.ghost_token());
        stylesheet.visit_mut(&mut self.visitor, &mut visit_context);
        Ok(())
    }
}

/// Error annotated with the plugin that returned it.
#[derive(Debug)]
pub struct PluginError {
    plugin: String,
    source: BoxError,
}

impl PluginError {
    #[inline]
    pub fn plugin(&self) -> &str {
        &self.plugin
    }
}

impl fmt::Display for PluginError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "plugin `{}` failed: {}",
            self.plugin, self.source
        )
    }
}

impl Error for PluginError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
