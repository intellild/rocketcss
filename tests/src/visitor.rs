use rocketcss_ast_next::{Atom, SelectorComponentData, SelectorData};
use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::Allocator;
use rocketcss_parser::{Compiler, ParserOptions};
use rocketcss_visitor::{PluginContext, Plugins, VisitMutContext, VisitorMut};

use crate::{expected_path, fixture_paths, read_fixture};

struct RenameClass<'a> {
    after: Atom<'a>,
}

impl<'ast> VisitorMut<'ast> for RenameClass<'ast> {
    fn visit_selector(
        &mut self,
        selector: rocketcss_ast_next::Selector<'ast>,
        cx: &mut VisitMutContext<'_, 'ast>,
    ) {
        let SelectorData::Parsed(components) = selector.data(cx.ast()) else {
            return;
        };
        for component in components.iter().copied() {
            if matches!(
                component.data(cx.ast()),
                SelectorComponentData::Class(name) if name == "before"
            ) {
                cx.ast_mut()
                    .set_data(component, SelectorComponentData::Class(self.after));
            }
        }
    }
}

#[test]
fn plugins_transform_expected_css() {
    for input in fixture_paths("visitor") {
        let source = read_fixture(&input);
        let expected = read_fixture(&expected_path(&input));
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut compiler = Compiler::new(&allocator);
            let mut stylesheet = compiler
                .parse(&source, &mut token, ParserOptions::default())
                .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));
            let mut context = PluginContext::new(&allocator, &mut token);
            let mut plugins = Plugins::new();
            plugins.add_visitor(
                "rename-class",
                RenameClass {
                    after: compiler.intern("after"),
                },
            );

            plugins
                .run(&mut stylesheet, &mut context)
                .unwrap_or_else(|error| panic!("{} should transform: {error}", input.display()));
            let actual = stylesheet
                .to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::new(&token, stylesheet.ast()),
                )
                .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

            assert_eq!(actual, expected, "fixture: {}", input.display());
        });
    }
}
