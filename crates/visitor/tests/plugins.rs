use std::{error::Error, fmt};

use rs_css_visitor::prelude::*;

struct Rename {
    from: &'static str,
    to: &'static str,
}

impl<'a> VisitMut<'a> for Rename {
    fn visit_selector_component(&mut self, component: &mut SelectorComponent<'a>) {
        if let SelectorComponent::Class(name) = component
            && *name == self.from
        {
            *name = self.to;
        }
        walk_mut::walk_selector_component(self, component);
    }
}

struct RecordPlugin(&'static str);

impl<'a> Plugin<'a> for RecordPlugin {
    fn name(&self) -> &str {
        self.0
    }

    fn transform(
        &mut self,
        _stylesheet: &mut StyleSheet<'a>,
        context: &mut PluginContext<'a>,
    ) -> Result<(), BoxError> {
        context
            .get_mut::<std::vec::Vec<&'static str>>()
            .unwrap()
            .push(self.0);
        Ok(())
    }
}

#[test]
fn plugins_run_in_registration_order_and_share_context() {
    let allocator = Allocator::new();
    let mut sheet = rs_css_parser::parse(
        ".first {}",
        &allocator,
        rs_css_parser::ParserOptions::default(),
    )
    .unwrap();
    let mut context = PluginContext::new(&allocator);
    context.insert(std::vec::Vec::<&'static str>::new());
    let mut plugins = Plugins::new();
    plugins.add(RecordPlugin("one"));
    plugins.add_visitor(
        "first-rename",
        Rename {
            from: "first",
            to: "middle",
        },
    );
    plugins.add(RecordPlugin("two"));
    plugins.add_visitor(
        "second-rename",
        Rename {
            from: "middle",
            to: "last",
        },
    );

    plugins.run(&mut sheet, &mut context).unwrap();

    assert_eq!(
        context.get::<std::vec::Vec<&str>>().unwrap(),
        &["one", "two"]
    );
    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    assert!(matches!(
        rule.selectors[0][0],
        SelectorComponent::Class("last")
    ));
}

#[derive(Debug)]
struct ExpectedFailure;

impl fmt::Display for ExpectedFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("expected failure")
    }
}

impl Error for ExpectedFailure {}

struct FailingPlugin;

impl<'a> Plugin<'a> for FailingPlugin {
    fn name(&self) -> &str {
        "failing"
    }

    fn transform(
        &mut self,
        _stylesheet: &mut StyleSheet<'a>,
        _context: &mut PluginContext<'a>,
    ) -> Result<(), BoxError> {
        Err(std::boxed::Box::new(ExpectedFailure))
    }
}

#[test]
fn plugin_errors_include_the_plugin_name() {
    let allocator = Allocator::new();
    let mut sheet =
        rs_css_parser::parse("a {}", &allocator, rs_css_parser::ParserOptions::default()).unwrap();
    let mut context = PluginContext::new(&allocator);
    let mut plugins = Plugins::new();
    plugins.add(FailingPlugin);

    let error = plugins.run(&mut sheet, &mut context).unwrap_err();

    assert_eq!(error.plugin(), "failing");
    assert_eq!(
        error.to_string(),
        "plugin `failing` failed: expected failure"
    );
}
