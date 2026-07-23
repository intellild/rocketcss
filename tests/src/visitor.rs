use rocketcss_allocator::Allocator;
use rocketcss_ast::{SelectorComponent, VisitMutContext};
use rocketcss_codegen::{PrinterOptions, ToCssWithGhost};
use rocketcss_parser::{ParserOptions, parse};
use rocketcss_visitor::{PluginContext, Plugins, VisitMut, VisitorMut};

use crate::{expected_path, fixture_paths, read_fixture};

struct RenameClass;

impl<'a, 'ghost> VisitorMut<'a, 'ghost> for RenameClass {
    fn visit_selector_component(
        &mut self,
        component: &mut SelectorComponent<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        if let SelectorComponent::Class(name) = component
            && *name == "before"
        {
            *name = "after";
        }
        component.visit_mut_children(self, cx);
    }
}

#[test]
fn plugins_transform_expected_css() {
    for input in fixture_paths("visitor") {
        let source = read_fixture(&input);
        let expected = read_fixture(&expected_path(&input));
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut stylesheet = parse(&source, &allocator, &mut token, ParserOptions::default())
                .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));
            let mut context = PluginContext::new(&allocator, &mut token);
            let mut plugins = Plugins::new();
            plugins.add_visitor("rename-class", RenameClass);

            plugins
                .run(&mut stylesheet, &mut context)
                .unwrap_or_else(|error| panic!("{} should transform: {error}", input.display()));
            let actual = stylesheet
                .to_css_string(&token, PrinterOptions::default())
                .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

            assert_eq!(actual, expected, "fixture: {}", input.display());
        });
    }
}
