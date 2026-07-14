#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_allocator::Allocator;
use rocketcss_parser::prelude::StyleSheet;
use rocketcss_visitor::prelude::*;
use std::boxed::Box as StdBox;

const BOOTSTRAP: &str = include_str!("../files/bootstrap.css");
const ITERATIONS: usize = 10;

fn main() {
    divan::main();
}

struct ParsedStyleSheet {
    // Fields are dropped in declaration order, so the stylesheet is dropped
    // before the allocator that owns its arena storage.
    stylesheet: StyleSheet<'static>,
    allocator: StdBox<Allocator>,
}

impl ParsedStyleSheet {
    fn new() -> Self {
        let allocator = StdBox::new(Allocator::new());

        // The allocator remains at a stable heap address and is owned by this
        // input for at least as long as the stylesheet.
        let allocator_ref: &'static Allocator = unsafe { &*std::ptr::from_ref(&*allocator) };
        let stylesheet = rocketcss_parser::parse(
            BOOTSTRAP,
            allocator_ref,
            rocketcss_parser::ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            stylesheet,
            allocator,
        }
    }

    fn allocator(&self) -> &'static Allocator {
        // SAFETY: the boxed allocator has a stable address and outlives the
        // stylesheet and plugin context in each benchmark input.
        unsafe { &*std::ptr::from_ref(&*self.allocator) }
    }
}

#[derive(Default)]
struct StaticMethods {
    seen: usize,
}

#[visitor]
impl<'a> VisitorMut<'a> for StaticMethods {
    fn visit_selector_component(&mut self, component: &mut SelectorComponent<'a>) {
        self.seen += 1;
        component.visit_mut_children(self);
    }
}

#[derive(Default)]
struct AllMethods {
    seen: usize,
}

impl<'a> VisitorMut<'a> for AllMethods {
    fn visit_selector_component(&mut self, component: &mut SelectorComponent<'a>) {
        self.seen += 1;
        component.visit_mut_children(self);
    }
}

fn bench_plugin_runner<V>(bencher: Bencher<'_, '_>)
where
    V: VisitorMut<'static> + Default + 'static,
{
    bencher
        .counter(BytesCount::new(BOOTSTRAP.len() * ITERATIONS))
        .with_inputs(ParsedStyleSheet::new)
        .bench_local_values(|mut input| {
            let mut plugins: Plugins<'_, 'static> = Plugins::new();
            plugins.add_visitor("selector", V::default());
            let mut context = PluginContext::new(input.allocator());
            for _ in 0..ITERATIONS {
                plugins.run(&mut input.stylesheet, &mut context).unwrap();
            }
            black_box(plugins);
        });
}

#[divan::bench]
fn static_methods(bencher: Bencher<'_, '_>) {
    bench_plugin_runner::<StaticMethods>(bencher);
}

#[divan::bench]
fn all_methods(bencher: Bencher<'_, '_>) {
    bench_plugin_runner::<AllMethods>(bencher);
}
