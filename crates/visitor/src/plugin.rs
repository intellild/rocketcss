use std::{
    any::{Any, TypeId},
    error::Error,
    fmt,
};

use rocketcss_ast::{Compilation, CompilationVisitorMut, ConcreteMutationError};
use rocketcss_common::{Allocator, GhostToken};
use rustc_hash::FxHashMap;

/// Type-erased error returned by a plugin.
pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

/// Shared services available to every plugin in a pipeline.
pub struct PluginContext<'a, 'token, 'ghost> {
    allocator: &'a Allocator,
    token: &'token mut GhostToken<'ghost>,
    data: FxHashMap<TypeId, Box<dyn Any>>,
}

impl<'a, 'token, 'ghost> PluginContext<'a, 'token, 'ghost> {
    #[inline]
    pub fn new(allocator: &'a Allocator, token: &'token mut GhostToken<'ghost>) -> Self {
        Self {
            allocator,
            token,
            data: FxHashMap::default(),
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

/// A plugin over the compiler-owned [`Compilation`].
pub trait Plugin<'a, 'ghost> {
    fn name(&self) -> &str;

    fn transform(
        &mut self,
        compilation: &mut Compilation<'a>,
        context: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError>;
}

/// Runs plugins in registration order over one authoritative compilation.
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

    pub fn add<P>(&mut self, plugin: P)
    where
        P: Plugin<'a, 'ghost> + 'plugin,
    {
        self.plugins.push(Box::new(plugin));
    }

    pub fn add_visitor<V>(&mut self, name: &'static str, visitor: V)
    where
        V: CompilationVisitorMut<'a> + 'plugin,
    {
        self.add(VisitorPlugin::new(name, visitor));
    }

    pub fn run(
        &mut self,
        compilation: &mut Compilation<'a>,
        context: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), PluginError> {
        for plugin in &mut self.plugins {
            plugin
                .transform(compilation, context)
                .map_err(|source| PluginError {
                    plugin: plugin.name().to_owned(),
                    source,
                })?;
        }
        Ok(())
    }
}

/// Adapts an ID-based mutable visitor to the plugin pipeline.
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

impl<'a, 'ghost, V: CompilationVisitorMut<'a>> Plugin<'a, 'ghost> for VisitorPlugin<V> {
    fn name(&self) -> &str {
        self.name
    }

    fn transform(
        &mut self,
        compilation: &mut Compilation<'a>,
        _context: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError> {
        compilation
            .visit_compilation_mut(&mut self.visitor)
            .map_err(|error| {
                Box::new(RadixTraversalError(error.erase_arena_lifetime())) as BoxError
            })
    }
}

#[derive(Debug)]
struct RadixTraversalError(ConcreteMutationError<'static>);

impl fmt::Display for RadixTraversalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid AST during plugin traversal: {:?}",
            self.0
        )
    }
}

impl Error for RadixTraversalError {}

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
