use std::{error::Error, fmt};

use rocketcss_visitor::prelude::*;

use rocketcss_ast_next::{
    Compilation, DeclarationData, DeclarationEntry, DeclarationPayloadData, SelectorComponentData,
    SelectorData,
};

struct Rename<'a> {
    from: &'static str,
    to: Atom<'a>,
}

impl<'ast> VisitorMut<'ast> for Rename<'ast> {
    fn visit_selector(
        &mut self,
        selector: rocketcss_ast_next::Selector<'ast>,
        cx: &mut VisitMutContext<'_, 'ast>,
    ) {
        let ast = cx.ast();
        let SelectorData::Parsed(components) = selector.data(ast) else {
            return;
        };
        for component in components.iter().copied() {
            if matches!(component.data(cx.ast()), SelectorComponentData::Class(name) if name == self.from)
            {
                cx.ast_mut()
                    .set_data(component, SelectorComponentData::Class(self.to));
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
        let ast = sheet.ast();
        let rule = sheet.style_sheet().rules(ast)[0];
        let rocketcss_ast_next::CssRulePayloadData::Style(style) = rule.payload(ast).data(ast)
        else {
            panic!("expected style rule")
        };
        let selector = style.selectors(ast)[0];
        let SelectorData::Parsed(components) = selector.data(ast) else {
            panic!("expected parsed selector")
        };
        assert!(matches!(
            components[0].data(ast),
            SelectorComponentData::Class(name) if name == "last"
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

struct RecordRadixPlugin(&'static str);

impl<'a, 'ghost> Plugin<'a, 'ghost> for RecordRadixPlugin {
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

struct TombstoneProperties;

impl<'ast> VisitorMut<'ast> for TombstoneProperties {
    fn visit_declaration(
        &mut self,
        entry: DeclarationEntry<'ast>,
        cx: &mut VisitMutContext<'_, 'ast>,
    ) {
        let ast = cx.ast();
        let Some(declaration) = entry.payload.as_property(ast) else {
            return;
        };
        cx.ast_mut()
            .set_data(declaration, DeclarationData::Tombstone);
    }
}

#[test]
fn radix_plugins_run_on_one_authoritative_compilation_in_registration_order() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut compilation = rocketcss_parser::Compiler::new(&allocator)
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

        plugins.run(&mut compilation, &mut context).unwrap();

        assert_eq!(
            context.get::<std::vec::Vec<&str>>().unwrap(),
            &["one", "two"]
        );
        let ast = compilation.ast();
        let rule = compilation.style_sheet().rules(ast)[0];
        let block = rule.declarations(ast).unwrap();
        assert!(matches!(
            block.declarations(ast)[0].data(ast),
            DeclarationPayloadData::Property(declaration)
                if matches!(declaration.data(ast), DeclarationData::Tombstone)
        ));
    });
}
