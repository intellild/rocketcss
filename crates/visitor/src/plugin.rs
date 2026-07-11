use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::Error,
    fmt,
};

use rocketcss_allocator::Allocator;
use rocketcss_ast::StyleSheet;

use crate::VisitMut;

/// Type-erased error returned by a plugin.
pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

/// Shared services available to every plugin in a pipeline.
pub struct PluginContext<'a> {
    allocator: &'a Allocator,
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl<'a> PluginContext<'a> {
    #[inline]
    pub fn new(allocator: &'a Allocator) -> Self {
        Self {
            allocator,
            data: HashMap::new(),
        }
    }

    /// Returns the arena that owns the stylesheet being transformed.
    #[inline]
    pub fn allocator(&self) -> &'a Allocator {
        self.allocator
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
pub trait Plugin<'a> {
    fn name(&self) -> &str;

    fn transform(
        &mut self,
        stylesheet: &mut StyleSheet<'a>,
        context: &mut PluginContext<'a>,
    ) -> Result<(), BoxError>;
}

/// Runs plugins in registration order over one stylesheet.
pub struct Plugins<'plugin, 'a> {
    plugins: Vec<Box<dyn Plugin<'a> + 'plugin>>,
}

impl<'plugin, 'a> Default for Plugins<'plugin, 'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'plugin, 'a> Plugins<'plugin, 'a> {
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
        P: Plugin<'a> + 'plugin,
    {
        self.plugins.push(Box::new(plugin));
    }

    pub fn add_visitor<V>(&mut self, name: &'static str, visitor: V)
    where
        V: VisitMut<'a> + 'plugin,
    {
        self.add(VisitorPlugin::new(name, visitor));
    }

    pub fn run(
        &mut self,
        stylesheet: &mut StyleSheet<'a>,
        context: &mut PluginContext<'a>,
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

/// Adapts an infallible [`VisitMut`] implementation into a dynamic plugin.
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

impl<'a, V: VisitMut<'a>> Plugin<'a> for VisitorPlugin<V> {
    #[inline]
    fn name(&self) -> &str {
        self.name
    }

    fn transform(
        &mut self,
        stylesheet: &mut StyleSheet<'a>,
        _context: &mut PluginContext<'a>,
    ) -> Result<(), BoxError> {
        self.visitor.visit_style_sheet(stylesheet);
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
