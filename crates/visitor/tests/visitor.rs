use rocketcss_visitor::prelude::*;

#[derive(Default)]
struct Recorder {
    stack: std::vec::Vec<AstType>,
    style_rules: usize,
    declarations: usize,
    colors: usize,
}

impl<'a> Visit<'a> for Recorder {
    fn enter_node(&mut self, kind: AstType) {
        self.stack.push(kind);
    }

    fn leave_node(&mut self, kind: AstType) {
        assert_eq!(self.stack.pop(), Some(kind));
    }

    fn visit_style_rule(&mut self, rule: &StyleRule<'a>) {
        self.style_rules += 1;
        walk::walk_style_rule(self, rule);
    }

    fn visit_declaration(&mut self, declaration: &Declaration<'a>) {
        self.declarations += 1;
        walk::walk_declaration(self, declaration);
    }

    fn visit_css_color(&mut self, color: &CssColor<'a>) {
        self.colors += 1;
        walk::walk_css_color(self, color);
    }
}

#[test]
fn immutable_visitor_walks_the_complete_tree_with_balanced_events() {
    let allocator = Allocator::new();
    let sheet = rocketcss_parser::parse(
        ".a { color: red; background: linear-gradient(red, blue); }",
        &allocator,
        rocketcss_parser::ParserOptions::default(),
    )
    .unwrap();
    let mut recorder = Recorder::default();

    recorder.visit_style_sheet(&sheet);

    assert!(recorder.stack.is_empty());
    assert_eq!(recorder.style_rules, 1);
    assert_eq!(recorder.declarations, 2);
    assert_eq!(recorder.colors, 1);
}

struct RenameAndRecolor;

impl<'a> VisitMut<'a> for RenameAndRecolor {
    fn visit_selector_component(&mut self, component: &mut SelectorComponent<'a>) {
        if let SelectorComponent::Class(name) = component {
            *name = "renamed";
        }
        walk_mut::walk_selector_component(self, component);
    }

    fn visit_rgba(&mut self, color: &mut RGBA) {
        color.red = 1;
        walk_mut::walk_rgba(self, color);
    }
}

#[test]
fn mutable_visitor_can_transform_typed_nodes() {
    let allocator = Allocator::new();
    let mut sheet = rocketcss_parser::parse(
        ".before { color: red }",
        &allocator,
        rocketcss_parser::ParserOptions::default(),
    )
    .unwrap();

    RenameAndRecolor.visit_style_sheet(&mut sheet);

    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    assert!(matches!(
        rule.selectors[0][0],
        SelectorComponent::Class("renamed")
    ));
    assert!(matches!(
        rule.declarations.declarations[0],
        Declaration::Color(ref color)
            if matches!(**color, CssColor::Rgba(RGBA { red: 1, green: 0, blue: 0, alpha: 255 }))
    ));
}
