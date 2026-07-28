use rocketcss_allocator::vec::Vec;
use rocketcss_ast::{CssRule, StyleRule, StyleSheet};

pub(crate) fn walk_style_rules<'walk, 'ast, 'ghost>(
    stylesheet: &'walk StyleSheet<'ast, 'ghost>,
) -> std::vec::Vec<&'walk StyleRule<'ast, 'ghost>> {
    let mut style_rules = std::vec::Vec::new();
    collect_rule_list(&stylesheet.rules, &mut style_rules);
    style_rules
}

fn collect_rule_list<'walk, 'ast, 'ghost>(
    rules: &'walk Vec<'ast, CssRule<'ast, 'ghost>>,
    style_rules: &mut std::vec::Vec<&'walk StyleRule<'ast, 'ghost>>,
) {
    style_rules.reserve(rules.len());
    for rule in rules {
        collect_rule(rule, style_rules);
    }
}

fn collect_rule<'walk, 'ast, 'ghost>(
    rule: &'walk CssRule<'ast, 'ghost>,
    style_rules: &mut std::vec::Vec<&'walk StyleRule<'ast, 'ghost>>,
) {
    match rule {
        CssRule::Media(rule) => collect_rule_list(&rule.rules, style_rules),
        CssRule::Style(rule) => {
            let rule = rule.as_ref().get_ref();
            style_rules.push(rule);
            collect_rule_list(&rule.rules, style_rules);
        }
        CssRule::Supports(rule) => collect_rule_list(&rule.rules, style_rules),
        CssRule::MozDocument(rule) => collect_rule_list(&rule.rules, style_rules),
        CssRule::Nesting(rule) => {
            let rule = rule.style.as_ref().get_ref();
            style_rules.push(rule);
            collect_rule_list(&rule.rules, style_rules);
        }
        CssRule::LayerBlock(rule) => collect_rule_list(&rule.rules, style_rules),
        CssRule::Container(rule) => collect_rule_list(&rule.rules, style_rules),
        CssRule::Scope(rule) => collect_rule_list(&rule.rules, style_rules),
        CssRule::StartingStyle(rule) => collect_rule_list(&rule.rules, style_rules),
        _ => {}
    }
}
