use std::{error::Error, fmt};

use rocketcss_visitor::prelude::*;

struct Rename<'a> {
    from: &'static str,
    to: Atom<'a>,
}

impl<'a> rocketcss_ast::StyleSheetVisitorMut<'a> for Rename<'a> {
    fn visit_selector_value(
        &mut self,
        _id: rocketcss_ast::SelectorValueId,
        selectors: &mut SelectorList<'a>,
    ) {
        for selector in selectors {
            let Selector::Parsed(components) = selector else {
                continue;
            };
            for component in components {
                if let SelectorComponent::Class(name) = component
                    && *name == self.from
                {
                    *name = self.to;
                }
            }
        }
    }
}

struct RecordPlugin(&'static str);

impl<'a, 'ghost> Plugin<'a, 'ghost> for RecordPlugin {
    fn name(&self) -> &str {
        self.0
    }

    fn transform(
        &mut self,
        _stylesheet: &mut StyleSheet<'a>,
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
        let (_, rule) = sheet.root_rules().next().unwrap();
        let rocketcss_ast::CssRule::Style(rule) = rule.payload() else {
            panic!("expected style rule")
        };
        let selectors = sheet
            .selector_value(rule.selector_value)
            .expect("the selector value remains valid")
            .selectors();
        assert!(matches!(
            selectors[0][0],
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
        _stylesheet: &mut StyleSheet<'a>,
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

struct RecordRadixPlugin(&'static str);

impl<'a, 'ghost> Plugin<'a, 'ghost> for RecordRadixPlugin {
    fn name(&self) -> &str {
        self.0
    }

    fn transform(
        &mut self,
        _stylesheet: &mut StyleSheet<'a>,
        context: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError> {
        context
            .get_mut::<std::vec::Vec<&'static str>>()
            .unwrap()
            .push(self.0);
        Ok(())
    }
}

struct TombstoneProperties;

impl<'a> rocketcss_ast::StyleSheetVisitorMut<'a> for TombstoneProperties {
    fn visit_declaration(
        &mut self,
        block: rocketcss_ast::CssDeclarationBlockId<'a>,
        declaration: rocketcss_ast::DeclarationId,
        cx: &mut rocketcss_ast::StyleSheetVisitMutContext<'_, 'a>,
    ) {
        cx.replace_property_declaration(block, declaration, Declaration::Tombstone)
            .unwrap();
    }
}

#[test]
fn radix_plugins_run_on_one_authoritative_stylesheet_in_registration_order() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = rocketcss_parser::Compiler::new(&allocator)
            .parse(
                "a{color:red}",
                &mut token,
                rocketcss_parser::ParserOptions::default(),
            )
            .unwrap();
        let mut context = PluginContext::new(&allocator, &mut token);
        context.insert(std::vec::Vec::<&'static str>::new());
        let mut plugins = Plugins::new();
        plugins.add(RecordRadixPlugin("one"));
        plugins.add_visitor("tombstone-properties", TombstoneProperties);
        plugins.add(RecordRadixPlugin("two"));

        plugins.run(&mut stylesheet, &mut context).unwrap();

        assert_eq!(
            context.get::<std::vec::Vec<&str>>().unwrap(),
            &["one", "two"]
        );
        let block = stylesheet
            .root_rules()
            .next()
            .unwrap()
            .1
            .declaration_block()
            .unwrap();
        assert!(matches!(
            stylesheet
                .declarations_in_block(block)
                .unwrap()
                .next()
                .unwrap()
                .payload(),
            rocketcss_ast::CssDeclaration::Property(Declaration::Tombstone)
        ));
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    });
}
