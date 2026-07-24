use rocketcss_visitor::prelude::*;

#[derive(Default)]
struct Recorder {
    stack: std::vec::Vec<AstType>,
    style_sheets: usize,
    style_rules: usize,
    selector_lists: usize,
    declarations: usize,
    colors: usize,
    unknown_at_rules: usize,
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

    fn visit_unknown_at_rule(&mut self, rule: &UnknownAtRule<'a>) {
        self.unknown_at_rules += 1;
        rule.visit_children(self);
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

#[test]
#[ignore]
fn unknown_at_rule_emits_balanced_events_while_preserving_raw_body() {
    let allocator = Allocator::new();
    let sheet = rocketcss_parser::parse(
        "@global { h1 { color: red } }",
        &allocator,
        rocketcss_parser::ParserOptions::default(),
    )
    .unwrap();
    let mut recorder = Recorder::default();

    sheet.visit(&mut recorder);

    assert!(recorder.stack.is_empty());
    assert_eq!(recorder.unknown_at_rules, 1);
    assert_eq!(recorder.style_rules, 0);
}

#[derive(Default)]
struct RuleRecorder {
    css_rules: usize,
    default_at_rules: usize,
}

impl<'a> Visitor<'a> for RuleRecorder {
    fn visit_css_rule(&mut self, rule: &CssRule<'a>) {
        self.css_rules += 1;
        rule.visit_children(self);
    }

    fn visit_default_at_rule(&mut self, rule: &DefaultAtRule) {
        self.default_at_rules += 1;
        rule.visit_children(self);
    }
}

#[test]
#[ignore]
fn css_rule_callback_is_non_generic_and_default_at_rule_is_public() {
    let allocator = Allocator::new();
    let mut sheet = rocketcss_parser::parse(
        "a{}@media print{b{}}",
        &allocator,
        rocketcss_parser::ParserOptions::default(),
    )
    .unwrap();
    sheet
        .rules
        .push(CssRule::Custom(allocator.boxed(DefaultAtRule)));
    let mut recorder = RuleRecorder::default();

    sheet.visit(&mut recorder);

    assert_eq!(recorder.css_rules, 4);
    assert_eq!(recorder.default_at_rules, 1);
}

#[derive(Default)]
struct SelectorRecorder<'a> {
    classes: std::vec::Vec<&'a str>,
    ids: std::vec::Vec<&'a str>,
    child_combinators: usize,
}

impl<'a> Visitor<'a> for SelectorRecorder<'a> {
    fn visit_selector(&mut self, selector: &Selector<'a>) {
        selector.visit_children(self);
    }

    fn visit_selector_component(&mut self, component: &SelectorComponent<'a>) {
        match component {
            SelectorComponent::Class(name) => self.classes.push(name),
            SelectorComponent::Id(name) => self.ids.push(name),
            SelectorComponent::Combinator(Combinator::Child) => self.child_combinators += 1,
            _ => {}
        }
        component.visit_children(self);
    }
}

#[test]
#[ignore]
fn selector_visitor_recurses_into_functional_selector_lists() {
    let allocator = Allocator::new();
    let sheet = rocketcss_parser::parse(
        ".a:is(.b,#c):has(> .d):nth-child(2n of .e,.f){color:red}",
        &allocator,
        rocketcss_parser::ParserOptions::default(),
    )
    .unwrap();
    let mut recorder = SelectorRecorder::default();

    sheet.visit(&mut recorder);

    assert_eq!(recorder.classes, ["a", "b", "d", "e", "f"]);
    assert_eq!(recorder.ids, ["c"]);
    assert_eq!(recorder.child_combinators, 1);
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
            if matches!(
                **color,
                CssColor::Known(color)
                    if color.rgba() == RGBA { red: 1, green: 0, blue: 0, alpha: 255 }
            )
    ));
}

#[derive(Default)]
struct FunctionOrderRecorder {
    events: std::vec::Vec<std::string::String>,
}

impl<'a> VisitorMut<'a> for FunctionOrderRecorder {
    fn visit_function(&mut self, function: &mut Function<'a>) {
        self.events.push(std::format!("{}-before", function.name()));
        function.visit_mut_children(self);
        self.events.push(std::format!("{}-after", function.name()));
    }
}

#[test]
#[ignore]
fn mutable_visitor_controls_nested_function_entry_and_exit_order() {
    let allocator = Allocator::new();
    let mut sheet = rocketcss_parser::parse(
        "a { unknown: outer(inner()) }",
        &allocator,
        rocketcss_parser::ParserOptions::default(),
    )
    .unwrap();
    let mut recorder = FunctionOrderRecorder::default();

    sheet.visit_mut(&mut recorder);

    assert_eq!(
        recorder.events,
        ["outer-before", "inner-before", "inner-after", "outer-after"]
    );
}

struct RemoveUnusedClass;

impl<'a> VisitorMut<'a> for RemoveUnusedClass {
    fn visit_style_rule(&mut self, mut rule: std::pin::Pin<&mut StyleRule<'a>>) {
        for selector in rule.as_mut().selectors_mut().iter_mut() {
            let should_remove = matches!(
                selector,
                Selector::Parsed(components)
                    if components.len() == 1
                        && matches!(components[0], SelectorComponent::Class("unused"))
            );
            if should_remove {
                *selector = Selector::Tombstone;
            }
        }
        rule.visit_mut_children(self);
    }
}

#[test]
#[ignore]
fn mutable_visitor_can_tombstone_direct_style_rule_selectors() {
    let allocator = Allocator::new();
    let mut sheet = rocketcss_parser::parse(
        ".unused,.used{color:red}.unused{color:blue}",
        &allocator,
        rocketcss_parser::ParserOptions::default(),
    )
    .unwrap();

    sheet.visit_mut(&mut RemoveUnusedClass);

    let CssRule::Style(first) = &sheet.rules[0] else {
        panic!("expected first style rule")
    };
    assert!(matches!(first.selectors[0], Selector::Tombstone));
    assert!(matches!(first.selectors[1], Selector::Parsed(_)));

    let CssRule::Style(second) = &sheet.rules[1] else {
        panic!("expected second style rule")
    };
    assert!(second.selectors.iter().all(Selector::is_tombstone));
}
