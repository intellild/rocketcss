use rocketcss_allocator::Allocator;
use rocketcss_ast::SelectorComponent;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_parser::{ParserOptions, parse};
use rocketcss_visitor::{PluginContext, Plugins, VisitMut, VisitorMut};

use crate::{expected_path, fixture_paths, read_fixture};

struct RenameClass;

impl<'a> VisitorMut<'a> for RenameClass {
    fn visit_selector_component(&mut self, component: &mut SelectorComponent<'a>) {
        if let SelectorComponent::Class(name) = component
            && *name == "before"
        {
            *name = "after";
        }
        component.visit_mut_children(self);
    }
}

#[test]
fn plugins_transform_expected_css() {
    for input in fixture_paths("visitor") {
        let source = read_fixture(&input);
        let expected = read_fixture(&expected_path(&input));
        let allocator = Allocator::new();
        let mut stylesheet = parse(&source, &allocator, ParserOptions::default())
            .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));
        let mut context = PluginContext::new(&allocator);
        let mut plugins = Plugins::new();
        plugins.add_visitor("rename-class", RenameClass);

        plugins
            .run(&mut stylesheet, &mut context)
            .unwrap_or_else(|error| panic!("{} should transform: {error}", input.display()));
        let actual = stylesheet
            .to_css_string(PrinterOptions::default())
            .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

        assert_eq!(actual, expected, "fixture: {}", input.display());
    }
}
