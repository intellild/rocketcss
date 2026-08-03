use rocketcss_ast::{Atom, Selector, SelectorComponent, radix_ast::CompilationVisitorMut};
use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::Allocator;
use rocketcss_parser::{Compiler, ParserOptions};
use rocketcss_visitor::{PluginContext, Plugins};

use crate::{expected_path, fixture_paths, read_fixture};

struct RenameClass<'a> {
    after: Atom<'a>,
}

impl<'a> CompilationVisitorMut<'a> for RenameClass<'a> {
    fn visit_selector_value(
        &mut self,
        _id: rocketcss_ast::radix_ast::SelectorValueId,
        selectors: &mut rocketcss_ast::SelectorList<'a>,
    ) {
        for selector in selectors {
            let Selector::Parsed(components) = selector else {
                continue;
            };
            for component in components {
                if let SelectorComponent::Class(name) = component
                    && *name == "before"
                {
                    *name = self.after;
                }
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
                .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

            assert_eq!(actual, expected, "fixture: {}", input.display());
        });
    }
}
