use std::{error::Error, fmt};

use rocketcss_visitor::prelude::*;

struct Rename<'a> {
    from: &'static str,
    to: Atom<'a>,
}

impl<'a, 'ghost> VisitorMut<'a, 'ghost> for Rename<'a> {
    fn visit_selector_component(
        &mut self,
        component: &mut SelectorComponent<'a>,
        cx: &mut VisitMutContext<'_, 'a, 'ghost>,
    ) {
        if let SelectorComponent::Class(name) = component
            && *name == self.from
        {
            *name = self.to;
        }
        component.visit_mut_children(self, cx);
    }
}

struct RecordPlugin(&'static str);

impl<'a, 'ghost> Plugin<'a, 'ghost> for RecordPlugin {
    fn name(&self) -> &str {
        self.0
    }

    fn transform(
        &mut self,
        _compilation: &mut Compilation<'a>,
        context: &mut PluginContext<'a, '_, 'ghost>,
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
    allocator.with_ghost(|mut token| {
        let mut compiler = rocketcss_parser::Compiler::new(&allocator);
        let mut sheet = compiler
            .parse(
                ".first {}",
                &mut token,
                rocketcss_parser::ParserOptions::default(),
            )
            .unwrap();
        let middle = compiler.intern("middle");
        let last = compiler.intern("last");
        let mut context = PluginContext::new(&allocator, &mut token);
        context.insert(std::vec::Vec::<&'static str>::new());
        let mut plugins = Plugins::new();
        plugins.add(RecordPlugin("one"));
        plugins.add_visitor(
            "first-rename",
            Rename {
                from: "first",
                to: middle,
            },
        );
        plugins.add(RecordPlugin("two"));
        plugins.add_visitor(
            "second-rename",
            Rename {
                from: "middle",
                to: last,
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
        let rule = rule.as_ref().get_ref();
        assert!(matches!(
            rule.selectors[0][0],
            SelectorComponent::Class(name) if name == "last"
        ));
    });
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

impl<'a, 'ghost> Plugin<'a, 'ghost> for FailingPlugin {
    fn name(&self) -> &str {
        "failing"
    }

    fn transform(
        &mut self,
        _compilation: &mut Compilation<'a>,
        _context: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError> {
        Err(std::boxed::Box::new(ExpectedFailure))
    }
}

#[test]
fn plugin_errors_include_the_plugin_name() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut sheet = rocketcss_parser::parse(
            "a {}",
            &allocator,
            &mut token,
            rocketcss_parser::ParserOptions::default(),
        )
        .unwrap();
        let mut context = PluginContext::new(&allocator, &mut token);
        let mut plugins = Plugins::new();
        plugins.add(FailingPlugin);

        let error = plugins.run(&mut sheet, &mut context).unwrap_err();

        assert_eq!(error.plugin(), "failing");
        assert_eq!(
            error.to_string(),
            "plugin `failing` failed: expected failure"
        );
    });
}
