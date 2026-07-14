use rocketcss_visitor::prelude::*;

#[derive(Default)]
struct Recorder {
    stack: std::vec::Vec<AstType>,
    style_sheets: usize,
    style_rules: usize,
    selector_lists: usize,
    declarations: usize,
    colors: usize,
}

impl<'a> Visitor<'a> for Recorder {
    fn enter_node(&mut self, kind: AstType) {
        self.stack.push(kind);
    }

    fn leave_node(&mut self, kind: AstType) {
        assert_eq!(self.stack.pop(), Some(kind));
    }

    fn visit_style_sheet(&mut self, sheet: &StyleSheet<'a>) {
        self.style_sheets += 1;
        sheet.visit_children(self);
    }

    fn visit_style_rule(&mut self, rule: &StyleRule<'a>) {
        self.style_rules += 1;
        rule.visit_children(self);
    }

    fn visit_selector_list(&mut self, selectors: &SelectorList<'a>) {
        self.selector_lists += 1;
        self.visit_selector_list_children(selectors);
    }

    fn visit_declaration(&mut self, declaration: &Declaration<'a>) {
        self.declarations += 1;
        declaration.visit_children(self);
    }

    fn visit_css_color(&mut self, color: &CssColor<'a>) {
        self.colors += 1;
        color.visit_children(self);
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

    sheet.visit(&mut recorder);

    assert!(recorder.stack.is_empty());
    assert_eq!(recorder.style_sheets, 1);
    assert_eq!(recorder.style_rules, 1);
    assert_eq!(recorder.selector_lists, 1);
    assert_eq!(recorder.declarations, 2);
    assert_eq!(recorder.colors, 1);
}

struct RenameAndRecolor;

impl<'a> VisitorMut<'a> for RenameAndRecolor {
    fn visit_selector_component(&mut self, component: &mut SelectorComponent<'a>) {
        if let SelectorComponent::Class(name) = component {
            *name = "renamed";
        }
        component.visit_mut_children(self);
    }

    fn visit_rgba(&mut self, color: &mut RGBA) {
        color.red = 1;
        color.visit_mut_children(self);
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

    sheet.visit_mut(&mut RenameAndRecolor);

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
