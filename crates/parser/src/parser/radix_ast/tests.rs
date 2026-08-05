use super::*;
use crate::parser::ReplayCounters;

fn selector_representative(
    _stylesheet: &StyleSheet<'_>,
    key: &CssEffectiveKey<'_>,
) -> Option<SelectorPathId> {
    key.selector_path()
}

fn context_representative<'ast>(
    stylesheet: &StyleSheet<'ast>,
    key: &CssEffectiveKey<'ast>,
) -> Option<CssRuleId<'ast>> {
    let (_, value) = stylesheet.context_path_record(key.context_path()?)?;
    stylesheet.context_value_representative(value)
}

fn effective_key_for<'ast>(stylesheet: &StyleSheet<'ast>, rule: CssRuleId<'ast>) -> EffectiveKeyId {
    let block = stylesheet
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("the test rule owns a declaration block");
    stylesheet
        .declaration_block(block)
        .expect("the test block remains resolvable")
        .effective_key()
}

fn selector_path_for_rule<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rule: CssRuleId<'ast>,
) -> Option<SelectorPathId> {
    stylesheet
        .effective_key(effective_key_for(stylesheet, rule))
        .and_then(|key| key.selector_path())
}

fn selector_value_for_rule<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rule: CssRuleId<'ast>,
) -> SelectorValueId {
    match stylesheet.rule(rule).unwrap().payload() {
        CssRule::Style(payload) => payload.selector_value,
        CssRule::Nesting(payload) => payload.selector_value,
        _ => panic!("the test rule must own a selector"),
    }
}

fn effective_key_seed<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rule: CssRuleId<'ast>,
) -> CssEffectiveKey<'ast> {
    let block = stylesheet.rule(rule).unwrap().declaration_block().unwrap();
    *stylesheet
        .effective_key(stylesheet.declaration_block(block).unwrap().effective_key())
        .unwrap()
}

fn declaration_ranges(stylesheet: &StyleSheet<'_>) -> std::vec::Vec<(u32, u32)> {
    let mut start = 0;
    stylesheet
        .declaration_blocks_in_source_order()
        .map(|(_, block)| {
            let len = block.declarations().len();
            let range = (start, len);
            start += len;
            range
        })
        .collect()
}

fn payload_kind(payload: &CssRule<'_>) -> &'static str {
    match payload {
        CssRule::Style(_) => "style",
        CssRule::Media(_) => "media",
        CssRule::Supports(_) => "supports",
        CssRule::StartingStyle(_) => "starting-style",
        CssRule::LayerStatement(_) => "layer-statement",
        CssRule::LayerBlock(_) => "layer-block",
        CssRule::Container(_) => "container",
        CssRule::Scope(_) => "scope",
        CssRule::MozDocument(_) => "moz-document",
        CssRule::Unknown(_) => "unknown",
        CssRule::CounterStyle(_) => "counter-style",
        CssRule::Viewport(_) => "viewport",
        CssRule::PositionTry(_) => "position-try",
        CssRule::FontFace(_) => "font-face",
        CssRule::FontPaletteValues(_) => "font-palette-values",
        CssRule::ViewTransition(_) => "view-transition",
        CssRule::Import(_) => "import",
        CssRule::Charset(_) => "charset",
        CssRule::Namespace(_) => "namespace",
        CssRule::CustomMedia(_) => "custom-media",
        CssRule::Keyframes(_) => "keyframes",
        CssRule::Keyframe(_) => "keyframe",
        CssRule::Page(_) => "page",
        CssRule::PageMargin(_) => "page-margin",
        CssRule::PageDeclarations(_) => "page-declarations",
        CssRule::Nesting(_) => "nesting",
        CssRule::FontFeatureValues(_) => "font-feature-values",
        CssRule::FontFeatureSubrule(_) => "font-feature-subrule",
        CssRule::Property(_) => "property",
        CssRule::NestedDeclarations(_) => "declarations",
    }
}

#[test]
fn allocates_nested_rules_and_blocks_in_lexical_order() {
    let allocator = Allocator::new();
    let mut compiler = Compiler::new(&allocator);
    let stylesheet = compiler
        .parse_stylesheet(
            "a{--before:0;& b{--nested:1}--after:2}c{color:red}",
            ParserOptions::default(),
        )
        .unwrap();

    let rules = stylesheet
        .rules_in_source_order()
        .map(|(id, rule)| {
            let kind = payload_kind(rule.payload());
            (id.primary_index(), kind)
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules,
        [
            (0, "style"),
            (1, "style"),
            (2, "declarations"),
            (3, "style")
        ]
    );

    let blocks = stylesheet
        .declaration_blocks_in_source_order()
        .map(|(id, block)| (id.primary_index(), block.owner(), block.declarations()))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(blocks.len(), 4);
    assert_eq!(blocks[0].0, 0);
    assert_eq!(
        (blocks[0].2.start_id().primary_index(), blocks[0].2.len()),
        (0, 1)
    );
    assert_eq!(blocks[1].0, 1);
    assert_eq!(
        (blocks[1].2.start_id().primary_index(), blocks[1].2.len()),
        (1, 1)
    );
    assert_eq!(blocks[2].0, 2);
    assert_eq!(
        (blocks[2].2.start_id().primary_index(), blocks[2].2.len()),
        (2, 1)
    );
    assert_eq!(blocks[3].0, 3);
    assert_eq!(
        (blocks[3].2.start_id().primary_index(), blocks[3].2.len()),
        (3, 1)
    );
    assert_eq!(
        stylesheet
            .declarations_in_source_order()
            .map(|(id, _)| id.primary_index())
            .collect::<std::vec::Vec<_>>(),
        [0, 1, 2, 3]
    );

    let root_ids = stylesheet
        .root_rules()
        .map(|(id, _)| id.primary_index())
        .collect::<std::vec::Vec<_>>();
    assert_eq!(root_ids, [0, 3]);
    let outer_id = stylesheet.root_rules().next().unwrap().0;
    let child_ids = stylesheet
        .nested_rules(outer_id)
        .unwrap()
        .map(|(id, _)| id.primary_index())
        .collect::<std::vec::Vec<_>>();
    assert_eq!(child_ids, [1, 2]);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn parser_interns_exact_effective_keys_and_isolates_opaque_contexts() {
    let allocator = Allocator::new();
    let source = "a{x:1}b{x:2}a{x:3}@media (width:1px){a{x:4}}@media (width:2px){a{x:5}}@media (width:1px){a{x:6}}@scope (.root){a{x:7}}@scope (.root){a{x:8}}@layer theme{a{x:9}}@layer theme{a{x:10}}";
    let stylesheet = Compiler::new(&allocator)
        .parse_stylesheet(source, ParserOptions::default())
        .unwrap();
    let styles = stylesheet
        .rules_in_source_order()
        .filter_map(|(id, rule)| matches!(rule.payload(), CssRule::Style(_)).then_some(id))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(styles.len(), 10);

    assert_eq!(
        effective_key_for(&stylesheet, styles[0]),
        effective_key_for(&stylesheet, styles[2])
    );
    assert_ne!(
        effective_key_for(&stylesheet, styles[0]),
        effective_key_for(&stylesheet, styles[1])
    );
    assert_eq!(
        effective_key_for(&stylesheet, styles[3]),
        effective_key_for(&stylesheet, styles[5])
    );
    assert_ne!(
        effective_key_for(&stylesheet, styles[3]),
        effective_key_for(&stylesheet, styles[4])
    );
    assert_ne!(
        effective_key_for(&stylesheet, styles[6]),
        effective_key_for(&stylesheet, styles[7])
    );
    assert_ne!(
        effective_key_for(&stylesheet, styles[8]),
        effective_key_for(&stylesheet, styles[9])
    );

    let key = *stylesheet
        .effective_key(effective_key_for(&stylesheet, styles[0]))
        .unwrap();
    assert_eq!(key.origin(), CascadeOrigin::Author);
    assert_eq!(key.cascade_phase(), CascadePhase::AuthorNormalAndImportant);
    assert_eq!(key.history_segment(), CssHistorySegment::StyleCascade);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn wrapper_order_and_multiplicity_are_part_of_the_effective_key() {
    let allocator = Allocator::new();
    let source = "@media print{@supports (display:grid){a{x:1}}}@supports (display:grid){@media print{a{x:2}}}@media print{@media print{a{x:3}}}@media print{a{x:4}}";
    let stylesheet = Compiler::new(&allocator)
        .parse_stylesheet(source, ParserOptions::default())
        .unwrap();
    let styles = stylesheet
        .rules_in_source_order()
        .filter_map(|(id, rule)| matches!(rule.payload(), CssRule::Style(_)).then_some(id))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(styles.len(), 4);
    let keys = styles
        .into_iter()
        .map(|rule| effective_key_for(&stylesheet, rule))
        .collect::<std::vec::Vec<_>>();
    assert_ne!(keys[0], keys[1]);
    assert_ne!(keys[2], keys[3]);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn replacing_a_selector_updates_only_its_inherited_effective_key_subtree() {
    let allocator = Allocator::new();
    let mut stylesheet = Compiler::new(&allocator)
        .parse_stylesheet(
            "a{x:1;& c{x:2}}b{x:3}a{x:4;& c{x:5}}",
            ParserOptions::default(),
        )
        .unwrap();
    let styles = stylesheet
        .rules_in_source_order()
        .filter_map(|(id, rule)| matches!(rule.payload(), CssRule::Style(_)).then_some(id))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(styles.len(), 5);

    let first_outer_key = effective_key_for(&stylesheet, styles[0]);
    let first_inner_key = effective_key_for(&stylesheet, styles[1]);
    let second_outer_key = effective_key_for(&stylesheet, styles[3]);
    let second_inner_key = effective_key_for(&stylesheet, styles[4]);
    assert_eq!(first_outer_key, second_outer_key);
    assert_eq!(first_inner_key, second_inner_key);

    let replacement = selector_value_for_rule(&stylesheet, styles[2]);
    assert!(
        stylesheet
            .replace_rule_selector_value(styles[0], replacement)
            .unwrap()
    );

    assert_eq!(selector_value_for_rule(&stylesheet, styles[0]), replacement);
    assert_eq!(
        effective_key_for(&stylesheet, styles[0]),
        effective_key_for(&stylesheet, styles[2])
    );
    assert_ne!(
        effective_key_for(&stylesheet, styles[0]),
        effective_key_for(&stylesheet, styles[3])
    );
    assert_ne!(
        effective_key_for(&stylesheet, styles[1]),
        effective_key_for(&stylesheet, styles[4])
    );
    assert_eq!(effective_key_for(&stylesheet, styles[3]), second_outer_key);
    assert_eq!(effective_key_for(&stylesheet, styles[4]), second_inner_key);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn radix_style_subset_preserves_semantic_blocks() {
    let allocator = Allocator::new();
    let source = "a{color:red;& b{margin:0;padding:1px}color:blue}c{display:block}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();

    assert_eq!(declaration_ranges(&radix), [(0, 1), (1, 2), (3, 1), (4, 1)]);
    assert!(
        radix
            .declarations_in_source_order()
            .all(|(_, declaration)| !declaration.is_important())
    );
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn inserts_a_direct_sibling_after_the_previous_subtree() {
    let allocator = Allocator::new();
    let mut stylesheet = Compiler::new(&allocator)
        .parse_stylesheet(
            "a{& b{color:red}color:blue}c{display:block}",
            ParserOptions::default(),
        )
        .unwrap();
    let (outer, following) = {
        let mut root_rules = stylesheet.root_rules();
        (root_rules.next().unwrap().0, root_rules.next().unwrap().0)
    };
    let nested = stylesheet
        .nested_rules(outer)
        .unwrap()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();

    let inserted = stylesheet
        .insert_rule_after(
            outer,
            CssRule::NestedDeclarations(NestedDeclarationsRule { span: DUMMY_SP }),
        )
        .unwrap();

    assert_eq!(
        stylesheet
            .root_rules()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [outer, inserted.id, following]
    );
    assert_eq!(
        stylesheet
            .rules_in_source_order()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [outer]
            .into_iter()
            .chain(nested)
            .chain([inserted.id, following])
            .collect::<std::vec::Vec<_>>()
    );
    assert_eq!(
        stylesheet
            .sibling_rules(outer)
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [outer, inserted.id, following]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn local_relabel_repairs_topology_and_effective_key_seeds() {
    let allocator = Allocator::new();
    let mut stylesheet = Compiler::new(&allocator)
        .parse_stylesheet("a{}b{}", ParserOptions::default())
        .unwrap();
    let outer = stylesheet.root_rules().next().unwrap().0;
    let first = stylesheet
        .insert_rule_after(
            outer,
            CssRule::NestedDeclarations(NestedDeclarationsRule { span: DUMMY_SP }),
        )
        .unwrap();
    let key = stylesheet
        .append_effective_key(CssEffectiveContext::isolated(first.id))
        .unwrap();
    let mut tracked = first.id;
    let mut relabeled = false;

    for _ in 0..16 {
        let result = stylesheet
            .insert_rule_after(
                outer,
                CssRule::NestedDeclarations(NestedDeclarationsRule { span: DUMMY_SP }),
            )
            .unwrap();
        if let Some(remap) = result.remaps.iter().find(|remap| remap.old == tracked) {
            tracked = remap.new;
            relabeled = true;
        }
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    assert!(relabeled);
    assert_eq!(
        stylesheet.effective_key(key).unwrap().history_segment(),
        CssHistorySegment::Isolated(tracked)
    );
    assert!(stylesheet.rule(tracked).is_some());
}

#[test]
fn insertion_after_retirement_preserves_live_source_order() {
    let allocator = Allocator::new();
    let mut stylesheet = Compiler::new(&allocator)
        .parse_stylesheet(
            "a{color:red}b{color:blue}c{color:green}",
            ParserOptions::default(),
        )
        .unwrap();
    let (first, retired, last) = {
        let mut rules = stylesheet.root_rules();
        (
            rules.next().unwrap().0,
            rules.next().unwrap().0,
            rules.next().unwrap().0,
        )
    };
    stylesheet.retire_rule(retired).unwrap();
    assert!(!stylesheet.rule(retired).unwrap().is_live());
    assert_eq!(
        stylesheet
            .root_rules()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [first, last]
    );

    let inserted = stylesheet
        .insert_rule_after(
            first,
            CssRule::NestedDeclarations(NestedDeclarationsRule { span: DUMMY_SP }),
        )
        .unwrap()
        .id;

    assert_eq!(
        stylesheet
            .rules_in_source_order()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [first, inserted, last]
    );
    assert_eq!(
        stylesheet
            .root_rules()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [first, inserted, last]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn top_level_media_uses_preorder_and_direct_child_topology() {
    let allocator = Allocator::new();
    let stylesheet = Compiler::new(&allocator)
        .parse_stylesheet(
            "@media screen{a{color:red}}b{display:block}",
            ParserOptions::default(),
        )
        .unwrap();
    let ids = stylesheet
        .rules_in_source_order()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        ids.iter()
            .map(|id| id.primary_index())
            .collect::<std::vec::Vec<_>>(),
        [0, 1, 2]
    );
    assert!(matches!(
        stylesheet.rule(ids[0]).unwrap().payload(),
        CssRule::Media(_)
    ));
    assert!(matches!(
        stylesheet.rule(ids[1]).unwrap().payload(),
        CssRule::Style(_)
    ));
    assert_eq!(
        stylesheet
            .root_rules()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [ids[0], ids[2]]
    );
    assert_eq!(
        stylesheet
            .nested_rules(ids[0])
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [ids[1]]
    );
    let style_block = stylesheet
        .rule(ids[1])
        .unwrap()
        .declaration_block()
        .unwrap();
    let seed = stylesheet
        .effective_key(
            stylesheet
                .declaration_block(style_block)
                .unwrap()
                .effective_key(),
        )
        .unwrap();
    assert_eq!(
        selector_representative(&stylesheet, seed),
        selector_path_for_rule(&stylesheet, ids[1])
    );
    assert_eq!(context_representative(&stylesheet, seed), Some(ids[0]));
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn media_inside_style_splits_ranges_and_preserves_context() {
    let allocator = Allocator::new();
    let source = "a{color:red;@media (width>1px){color:blue;& b{margin:0}padding:1px}color:green}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let rules = radix
        .rules_in_source_order()
        .map(|(id, rule)| {
            let kind = payload_kind(rule.payload());
            (id, kind)
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<std::vec::Vec<_>>(),
        [
            "style",
            "media",
            "declarations",
            "style",
            "declarations",
            "declarations"
        ]
    );
    let [outer, media, media_before, nested, media_after, outer_after] = rules.as_slice() else {
        unreachable!()
    };

    let outer_seed = effective_key_seed(&radix, outer.0);
    let media_before_seed = effective_key_seed(&radix, media_before.0);
    let nested_seed = effective_key_seed(&radix, nested.0);
    let media_after_seed = effective_key_seed(&radix, media_after.0);
    let outer_after_seed = effective_key_seed(&radix, outer_after.0);
    assert_eq!(context_representative(&radix, &outer_seed), None);
    assert_eq!(
        selector_representative(&radix, &outer_seed),
        selector_path_for_rule(&radix, outer.0)
    );
    assert_eq!(
        selector_representative(&radix, &media_before_seed),
        selector_path_for_rule(&radix, outer.0)
    );
    assert_eq!(
        context_representative(&radix, &media_before_seed),
        Some(media.0)
    );
    assert_eq!(media_after_seed, media_before_seed);
    assert_eq!(
        selector_representative(&radix, &nested_seed),
        selector_path_for_rule(&radix, nested.0)
    );
    assert_eq!(context_representative(&radix, &nested_seed), Some(media.0));
    assert_eq!(outer_after_seed, outer_seed);

    assert_eq!(
        declaration_ranges(&radix),
        [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)]
    );
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn supports_wrappers_share_group_topology_without_losing_style_context() {
    let allocator = Allocator::new();
    let source = "@supports (display:grid){a{color:red}}b{@supports (color:oklch(0 0 0)){color:blue}color:green}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let rules = radix
        .rules_in_source_order()
        .map(|(id, rule)| {
            let kind = payload_kind(rule.payload());
            (id, kind)
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<std::vec::Vec<_>>(),
        [
            "supports",
            "style",
            "style",
            "supports",
            "declarations",
            "declarations"
        ]
    );

    let top_supports = rules[0].0;
    let top_style = rules[1].0;
    let outer_style = rules[2].0;
    let nested_supports = rules[3].0;
    let nested_declarations = rules[4].0;
    let trailing_declarations = rules[5].0;
    assert_eq!(radix.rule(top_style).unwrap().parent(), Some(top_supports));
    assert_eq!(
        radix.rule(nested_supports).unwrap().parent(),
        Some(outer_style)
    );

    assert_eq!(
        context_representative(&radix, &effective_key_seed(&radix, top_style)),
        Some(top_supports)
    );
    assert_eq!(
        selector_representative(&radix, &effective_key_seed(&radix, nested_declarations)),
        selector_path_for_rule(&radix, outer_style)
    );
    assert_eq!(
        context_representative(&radix, &effective_key_seed(&radix, nested_declarations)),
        Some(nested_supports)
    );
    assert_eq!(
        context_representative(&radix, &effective_key_seed(&radix, trailing_declarations)),
        None
    );

    assert_eq!(declaration_ranges(&radix), [(0, 1), (1, 0), (1, 1), (2, 1)]);
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn starting_style_uses_explicit_source_order_segments() {
    let allocator = Allocator::new();
    let source =
        "@starting-style{a{opacity:0}}b{@starting-style{opacity:0;&:hover{opacity:.5}}opacity:1}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let kinds = radix
        .rules_in_source_order()
        .map(|(_, rule)| payload_kind(rule.payload()))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        kinds,
        [
            "starting-style",
            "style",
            "style",
            "starting-style",
            "declarations",
            "style",
            "declarations"
        ]
    );

    assert_eq!(
        declaration_ranges(&radix),
        [(0, 1), (1, 0), (1, 1), (2, 1), (3, 1)]
    );
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn layer_statement_and_blocks_keep_distinct_topology() {
    let allocator = Allocator::new();
    let source =
        "@layer reset,theme;@layer app{a{color:red}}b{@layer nested{color:blue}color:green}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let rules = radix
        .rules_in_source_order()
        .map(|(id, rule)| {
            let kind = payload_kind(rule.payload());
            (id, kind)
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<std::vec::Vec<_>>(),
        [
            "layer-statement",
            "layer-block",
            "style",
            "style",
            "layer-block",
            "declarations",
            "declarations"
        ]
    );
    assert!(!radix.has_nested_rules(rules[0].0).unwrap());
    assert!(radix.has_nested_rules(rules[1].0).unwrap());
    assert_eq!(radix.rule(rules[2].0).unwrap().parent(), Some(rules[1].0));
    assert_eq!(radix.rule(rules[4].0).unwrap().parent(), Some(rules[3].0));

    let nested_block = radix.rule(rules[5].0).unwrap().declaration_block().unwrap();
    let nested_key = radix
        .effective_key(
            radix
                .declaration_block(nested_block)
                .unwrap()
                .effective_key(),
        )
        .unwrap();
    assert_eq!(
        selector_representative(&radix, nested_key),
        selector_path_for_rule(&radix, rules[3].0)
    );
    assert_eq!(context_representative(&radix, nested_key), None);
    assert_eq!(
        radix
            .layer_context_record(nested_key.layer().unwrap())
            .unwrap()
            .1,
        rules[4].0
    );

    assert_eq!(declaration_ranges(&radix), [(0, 1), (1, 0), (1, 1), (2, 1)]);
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn nested_group_wrappers_preserve_the_full_parent_context_chain() {
    let allocator = Allocator::new();
    let source = "@container card (width>1px){@scope (.card) to (.end){a{color:red}}}b{@container style(--theme:dark){@scope (&){color:blue}}color:green}@-moz-document url-prefix(){c{display:block}}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let rules = radix
        .rules_in_source_order()
        .map(|(id, rule)| {
            let kind = payload_kind(rule.payload());
            (id, kind)
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<std::vec::Vec<_>>(),
        [
            "container",
            "scope",
            "style",
            "style",
            "container",
            "scope",
            "declarations",
            "declarations",
            "moz-document",
            "style"
        ]
    );

    let outer_style = rules[3].0;
    let container = rules[4].0;
    let scope = rules[5].0;
    let scoped_declarations = rules[6].0;
    assert_eq!(radix.rule(container).unwrap().parent(), Some(outer_style));
    assert_eq!(radix.rule(scope).unwrap().parent(), Some(container));
    assert_eq!(
        radix.rule(scoped_declarations).unwrap().parent(),
        Some(scope)
    );
    let block = radix
        .rule(scoped_declarations)
        .unwrap()
        .declaration_block()
        .unwrap();
    let seed = radix
        .effective_key(radix.declaration_block(block).unwrap().effective_key())
        .unwrap();
    assert_eq!(
        selector_representative(&radix, seed),
        selector_path_for_rule(&radix, outer_style)
    );
    assert_eq!(context_representative(&radix, seed), Some(scope));

    assert_eq!(
        declaration_ranges(&radix),
        [(0, 1), (1, 0), (1, 1), (2, 1), (3, 1)]
    );
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn unknown_at_rules_remain_opaque_and_lossless() {
    let allocator = Allocator::new();
    let source = "@foo screen and (x:y);a{color:red;@bar one{two:3;nested(x)}color:blue}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let ids = radix
        .rules_in_source_order()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    assert_eq!(ids.len(), 4);

    let CssRule::Unknown(radix_top) = radix.rule(ids[0]).unwrap().payload() else {
        unreachable!()
    };
    assert_eq!(radix_top.name, "foo");
    assert!(!radix_top.prelude.is_empty());
    assert!(radix_top.block.is_none());
    assert!(!radix.has_nested_rules(ids[0]).unwrap());

    let CssRule::Unknown(radix_nested) = radix.rule(ids[2]).unwrap().payload() else {
        unreachable!()
    };
    assert_eq!(radix_nested.name, "bar");
    assert!(!radix_nested.prelude.is_empty());
    assert!(
        radix_nested
            .block
            .as_ref()
            .is_some_and(|block| !block.is_empty())
    );
    assert!(!radix.has_nested_rules(ids[2]).unwrap());
    assert_eq!(radix.rule(ids[2]).unwrap().parent(), Some(ids[1]));

    assert_eq!(declaration_ranges(&radix), [(0, 1), (1, 1)]);
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn declaration_owner_rules_share_one_lexical_property_tape() {
    let allocator = Allocator::new();
    let source = "a{color:red}@counter-style marker{system:cyclic;symbols:'x'}@viewport{width:device-width}@position-try --fallback{top:0;left:1px}b{color:blue}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let kinds = radix
        .rules_in_source_order()
        .map(|(_, rule)| payload_kind(rule.payload()))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        kinds,
        [
            "style",
            "counter-style",
            "viewport",
            "position-try",
            "style"
        ]
    );
    assert_eq!(
        radix
            .declarations_in_source_order()
            .map(|(id, _)| id.primary_index())
            .collect::<std::vec::Vec<_>>(),
        [0, 1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        declaration_ranges(&radix),
        [(0, 1), (1, 2), (3, 1), (4, 2), (6, 1)]
    );

    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn declaration_owner_at_rules_are_not_reclassified_inside_style_rules() {
    let allocator = Allocator::new();
    let result = Compiler::new(&allocator).parse_stylesheet(
        "a{@counter-style marker{system:cyclic}}",
        ParserOptions {
            error_recovery: false,
            ..ParserOptions::default()
        },
    );
    assert!(result.is_err());
}

#[test]
fn font_face_descriptors_are_typed_occurrences_in_the_global_tape() {
    let allocator = Allocator::new();
    let source = "a{color:red}@font-face{font-family:Demo;src:url(demo.woff2);unicode-range:U+0-7F}b{color:blue}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let ids = radix
        .rules_in_source_order()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    assert_eq!(ids.len(), 3);
    assert!(matches!(
        radix.rule(ids[1]).unwrap().payload(),
        CssRule::FontFace(_)
    ));
    assert_eq!(declaration_ranges(&radix), [(0, 1), (1, 3), (4, 1)]);

    let block = radix.rule(ids[1]).unwrap().declaration_block().unwrap();
    let descriptors = radix
        .declarations_in_block(block)
        .unwrap()
        .collect::<std::vec::Vec<_>>();
    assert_eq!(descriptors.len(), 3);
    for record in descriptors {
        assert!(matches!(record.payload(), CssDeclaration::FontFace(_)));
        assert!(!record.is_important());
    }
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn palette_and_view_transition_descriptors_keep_typed_source_order() {
    let allocator = Allocator::new();
    let source = "a{color:red}@font-palette-values --theme{font-family:Demo;base-palette:1;override-colors:0 red}@view-transition{navigation:auto;types:foo bar}b{color:blue}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let ids = radix
        .rules_in_source_order()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    assert_eq!(ids.len(), 4);
    assert!(matches!(
        radix.rule(ids[1]).unwrap().payload(),
        CssRule::FontPaletteValues(_)
    ));
    assert!(matches!(
        radix.rule(ids[2]).unwrap().payload(),
        CssRule::ViewTransition(_)
    ));
    assert_eq!(declaration_ranges(&radix), [(0, 1), (1, 3), (4, 2), (6, 1)]);

    let palette_block = radix.rule(ids[1]).unwrap().declaration_block().unwrap();
    let palette = radix
        .declarations_in_block(palette_block)
        .unwrap()
        .collect::<std::vec::Vec<_>>();
    assert_eq!(palette.len(), 3);
    for record in palette {
        assert!(matches!(
            record.payload(),
            CssDeclaration::FontPaletteValues(_)
        ));
        assert!(!record.is_important());
    }

    let view_transition_block = radix.rule(ids[2]).unwrap().declaration_block().unwrap();
    let view_transition = radix
        .declarations_in_block(view_transition_block)
        .unwrap()
        .collect::<std::vec::Vec<_>>();
    assert_eq!(view_transition.len(), 2);
    for record in view_transition {
        assert!(matches!(
            record.payload(),
            CssDeclaration::ViewTransition(_)
        ));
        assert!(!record.is_important());
    }
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn top_level_statements_preserve_payloads_and_ordering_state() {
    let allocator = Allocator::new();
    let source = "@charset 'UTF-8';@layer base;@import url(a.css) layer(theme) screen;@namespace svg url(http://www.w3.org/2000/svg);@custom-media --narrow (width < 30em);a{}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let ids = radix
        .rules_in_source_order()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    assert_eq!(ids.len(), 6);

    let CssRule::Charset(radix_charset) = radix.rule(ids[0]).unwrap().payload() else {
        unreachable!()
    };
    assert_eq!(radix_charset.encoding, "UTF-8");

    let CssRule::LayerStatement(radix_layer) = radix.rule(ids[1]).unwrap().payload() else {
        unreachable!()
    };
    assert_eq!(radix_layer.names.as_slice(), [&["base"][..]]);

    let CssRule::Import(radix_import) = radix.rule(ids[2]).unwrap().payload() else {
        unreachable!()
    };
    assert_eq!(radix_import.url, "a.css");
    assert_eq!(radix_import.layer.as_deref(), Some(&["theme"][..]));

    let CssRule::Namespace(radix_namespace) = radix.rule(ids[3]).unwrap().payload() else {
        unreachable!()
    };
    assert_eq!(radix_namespace.prefix, Some("svg"));
    assert_eq!(radix_namespace.url, "http://www.w3.org/2000/svg");

    let CssRule::CustomMedia(radix_custom_media) = radix.rule(ids[4]).unwrap().payload() else {
        unreachable!()
    };
    assert_eq!(radix_custom_media.name, "--narrow");
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn radix_top_level_state_rejects_late_and_nested_statements() {
    let allocator = Allocator::new();
    let strict = ParserOptions {
        error_recovery: false,
        ..ParserOptions::default()
    };
    assert!(
        Compiler::new(&allocator)
            .parse_stylesheet("a{}@import 'late.css';", strict)
            .is_err()
    );
    assert!(
        Compiler::new(&allocator)
            .parse_stylesheet("@media screen{@namespace svg url(x);}", strict)
            .is_err()
    );
}

#[test]
fn keyframe_syntax_positions_are_explicit_child_rules() {
    let allocator = Allocator::new();
    let source = "a{color:red}@-webkit-keyframes fade{from{opacity:0}bogus selector{bad:1}50%,to{opacity:1;transform:none}}b{color:blue}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let rules = radix
        .rules_in_source_order()
        .map(|(id, rule)| (id, payload_kind(rule.payload())))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<std::vec::Vec<_>>(),
        ["style", "keyframes", "keyframe", "keyframe", "style"]
    );
    let wrapper = rules[1].0;
    let frames = radix
        .nested_rules(wrapper)
        .unwrap()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    assert_eq!(frames, [rules[2].0, rules[3].0]);

    let CssRule::Keyframes(radix_keyframes) = radix.rule(wrapper).unwrap().payload() else {
        unreachable!()
    };
    assert!(matches!(
        *radix_keyframes.name,
        KeyframesName::Ident("fade")
    ));
    assert_eq!(radix_keyframes.vendor_prefix, VendorPrefix::WEBKIT);
    let CssRule::Keyframe(first_frame) = radix.rule(frames[0]).unwrap().payload() else {
        unreachable!()
    };
    assert!(matches!(
        first_frame.selectors.as_slice(),
        [KeyframeSelector::From]
    ));
    let CssRule::Keyframe(second_frame) = radix.rule(frames[1]).unwrap().payload() else {
        unreachable!()
    };
    assert!(matches!(
        second_frame.selectors.as_slice(),
        [KeyframeSelector::Percentage(0.5), KeyframeSelector::To]
    ));
    assert_eq!(declaration_ranges(&radix), [(0, 1), (1, 1), (2, 2), (4, 1)]);
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn page_margin_rules_split_parent_declaration_ranges() {
    let allocator = Allocator::new();
    let source = "a{color:red}@page invoice:left{size:A4;@top-left{content:'x'}margin:0;@bottom-right{content:counter(page)}color:red}b{}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let rules = radix
        .rules_in_source_order()
        .map(|(id, rule)| (id, payload_kind(rule.payload())))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<std::vec::Vec<_>>(),
        [
            "style",
            "page",
            "page-margin",
            "page-declarations",
            "page-margin",
            "page-declarations",
            "style"
        ]
    );
    assert_eq!(
        declaration_ranges(&radix),
        [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 0)]
    );

    let page = rules[1].0;
    let CssRule::Page(radix_page) = radix.rule(page).unwrap().payload() else {
        unreachable!()
    };
    assert!(matches!(
        radix_page.selectors.as_slice(),
        [PageSelector { name: Some("invoice"), pseudo_classes }]
            if matches!(pseudo_classes.as_slice(), [PagePseudoClass::Left])
    ));
    let page_children = radix
        .nested_rules(page)
        .unwrap()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        page_children,
        [rules[2].0, rules[3].0, rules[4].0, rules[5].0]
    );

    let mut radix_page_declarations = std::vec::Vec::new();
    for owner in [page, rules[3].0, rules[5].0] {
        let block = radix.rule(owner).unwrap().declaration_block().unwrap();
        radix_page_declarations.extend(radix.declarations_in_block(block).unwrap());
    }
    assert_eq!(radix_page_declarations.len(), 3);
    assert!(
        radix_page_declarations
            .iter()
            .all(|declaration| !declaration.is_important())
    );

    for (margin, expected_box) in [
        (rules[2].0, PageMarginBox::TopLeft),
        (rules[4].0, PageMarginBox::BottomRight),
    ] {
        let CssRule::PageMargin(radix_margin) = radix.rule(margin).unwrap().payload() else {
            unreachable!()
        };
        assert_eq!(radix_margin.margin_box, expected_box);
        let radix_block = radix.rule(margin).unwrap().declaration_block().unwrap();
        let declarations = radix
            .declarations_in_block(radix_block)
            .unwrap()
            .collect::<std::vec::Vec<_>>();
        assert_eq!(declarations.len(), 1);
        assert!(!declarations[0].is_important());
    }
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn nest_rule_has_its_own_selector_owner_and_inherited_context() {
    let allocator = Allocator::new();
    let source = "a{@nest & .b{color:red;@media (width>1px){color:green}}color:blue}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let rules = radix
        .rules_in_source_order()
        .map(|(id, rule)| (id, payload_kind(rule.payload())))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<std::vec::Vec<_>>(),
        ["style", "nesting", "media", "declarations", "declarations"]
    );
    let outer = rules[0].0;
    let nesting = rules[1].0;
    let media = rules[2].0;
    let media_declarations = rules[3].0;
    assert_eq!(radix.rule(nesting).unwrap().parent(), Some(outer));
    assert_eq!(radix.rule(media).unwrap().parent(), Some(nesting));
    assert_eq!(
        radix.rule(media_declarations).unwrap().parent(),
        Some(media)
    );

    let nesting_block = radix.rule(nesting).unwrap().declaration_block().unwrap();
    let nesting_seed = radix
        .effective_key(
            radix
                .declaration_block(nesting_block)
                .unwrap()
                .effective_key(),
        )
        .unwrap();
    assert_eq!(
        selector_representative(&radix, nesting_seed),
        selector_path_for_rule(&radix, nesting)
    );
    assert_eq!(context_representative(&radix, nesting_seed), None);
    let media_block = radix
        .rule(media_declarations)
        .unwrap()
        .declaration_block()
        .unwrap();
    let media_seed = radix
        .effective_key(
            radix
                .declaration_block(media_block)
                .unwrap()
                .effective_key(),
        )
        .unwrap();
    assert_eq!(
        selector_representative(&radix, media_seed),
        selector_path_for_rule(&radix, nesting)
    );
    assert_eq!(context_representative(&radix, media_seed), Some(media));

    assert_eq!(declaration_ranges(&radix), [(0, 0), (0, 1), (1, 1), (2, 1)]);
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn font_feature_subrules_and_declarations_are_flattened() {
    let allocator = Allocator::new();
    let source = "@font-feature-values 'Demo'{@styleset{nice:1 2;alt:3}@swash{fancy:4}}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let rules = radix
        .rules_in_source_order()
        .map(|(id, rule)| (id, payload_kind(rule.payload())))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        rules
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<std::vec::Vec<_>>(),
        [
            "font-feature-values",
            "font-feature-subrule",
            "font-feature-subrule"
        ]
    );
    let wrapper = rules[0].0;
    assert_eq!(
        radix
            .nested_rules(wrapper)
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [rules[1].0, rules[2].0]
    );
    assert_eq!(declaration_ranges(&radix), [(0, 2), (2, 1)]);

    let CssRule::FontFeatureValues(radix_features) = radix.rule(wrapper).unwrap().payload() else {
        unreachable!()
    };
    assert!(matches!(
        radix_features.name.as_slice(),
        [FamilyName("Demo")]
    ));
    for (subrule, expected_name, expected_len) in [
        (rules[1].0, FontFeatureSubruleType::Styleset, 2),
        (rules[2].0, FontFeatureSubruleType::Swash, 1),
    ] {
        let CssRule::FontFeatureSubrule(radix_subrule) = radix.rule(subrule).unwrap().payload()
        else {
            unreachable!()
        };
        assert_eq!(radix_subrule.name, expected_name);
        let block = radix.rule(subrule).unwrap().declaration_block().unwrap();
        let declarations = radix
            .declarations_in_block(block)
            .unwrap()
            .collect::<std::vec::Vec<_>>();
        assert_eq!(declarations.len(), expected_len);
        for record in declarations {
            assert!(matches!(record.payload(), CssDeclaration::FontFeature(_)));
            assert!(!record.is_important());
        }
    }
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn property_rule_keeps_occurrences_and_points_to_last_effective_descriptors() {
    let allocator = Allocator::new();
    let source = "@property --space{syntax:'<length>';unknown:foo;syntax:'*';inherits:false;initial-value:10px}";
    let options = ParserOptions::default();
    let radix = Compiler::new(&allocator)
        .parse_stylesheet(source, options)
        .unwrap();
    let (rule, record) = radix.rules_in_source_order().next().unwrap();
    let CssRule::Property(property) = record.payload() else {
        unreachable!()
    };
    let block = record.declaration_block().unwrap();
    let declarations = radix
        .declarations_in_block(block)
        .unwrap()
        .collect::<std::vec::Vec<_>>();
    assert_eq!(declarations.len(), 5);
    assert!(matches!(
        declarations[0].payload(),
        CssDeclaration::PropertyRule(PropertyRuleDescriptor::Syntax(_))
    ));
    assert!(matches!(
        declarations[1].payload(),
        CssDeclaration::PropertyRule(PropertyRuleDescriptor::Unknown(_))
    ));
    assert!(matches!(
        declarations[2].payload(),
        CssDeclaration::PropertyRule(PropertyRuleDescriptor::Syntax(_))
    ));
    assert_eq!(property.syntax.unwrap().primary_index(), 2);
    assert_eq!(property.inherits.unwrap().primary_index(), 3);
    assert_eq!(property.initial_value.unwrap().primary_index(), 4);

    assert_eq!(property.name, "--space");
    let CssDeclaration::PropertyRule(PropertyRuleDescriptor::Syntax(syntax)) = radix
        .declaration(property.syntax.unwrap())
        .unwrap()
        .payload()
    else {
        unreachable!()
    };
    assert!(matches!(&**syntax, SyntaxString::Universal));
    let CssDeclaration::PropertyRule(PropertyRuleDescriptor::Inherits(inherits)) = radix
        .declaration(property.inherits.unwrap())
        .unwrap()
        .payload()
    else {
        unreachable!()
    };
    assert!(!*inherits);
    let CssDeclaration::PropertyRule(PropertyRuleDescriptor::InitialValue(initial)) = radix
        .declaration(property.initial_value.unwrap())
        .unwrap()
        .payload()
    else {
        unreachable!()
    };
    assert!(matches!(&**initial, ParsedComponent::TokenList(values) if !values.is_empty()));
    assert_eq!(radix.rule(rule).unwrap().parent(), None);
    assert_eq!(radix.validate_ast(), Ok(()));
}

#[test]
fn non_universal_property_still_requires_an_initial_value() {
    let allocator = Allocator::new();
    let strict = ParserOptions {
        error_recovery: false,
        ..ParserOptions::default()
    };
    assert!(
        Compiler::new(&allocator)
            .parse_stylesheet(
                "@property --space{syntax:'<length>';inherits:false}",
                strict,
            )
            .is_err()
    );
}

fn property_declarations<'comp, 'ast>(
    stylesheet: &'comp StyleSheet<'ast>,
) -> std::vec::Vec<&'comp Declaration<'ast>> {
    stylesheet
        .declarations_in_source_order()
        .filter_map(|(_, declaration)| declaration.payload().as_property())
        .collect()
}

fn parse_with_replay_counters<'a>(
    allocator: &'a Allocator,
    source: &'a str,
) -> (StyleSheet<'a>, ReplayCounters) {
    let mut compiler = Compiler::new(allocator);
    let stylesheet = compiler
        .parse_stylesheet(source, ParserOptions::default())
        .unwrap();
    let counters = compiler.replay_counters();
    (stylesheet, counters)
}

#[test]
fn replay_typed_success_decodes_without_replaying() {
    let allocator = Allocator::new();
    let (stylesheet, counters) = parse_with_replay_counters(&allocator, "a{width:1px;height:auto}");
    assert!(matches!(
        property_declarations(&stylesheet)[0],
        Declaration::Width(_)
    ));
    assert!(matches!(
        property_declarations(&stylesheet)[1],
        Declaration::Height(_)
    ));
    // The typed parser decodes `1px` and the trailing `;` once each; the
    // terminating `;` is then replayed by the final semicolon expect, so no
    // token is lexed twice.
    assert_eq!(counters.decodes, 2);
    assert_eq!(counters.replay_hits, 1);
    assert_eq!(counters.sync_misses, 0);
    assert_eq!(counters.recorded, 0);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn replay_failed_prefix_is_never_decoded_twice() {
    let allocator = Allocator::new();
    let (stylesheet, counters) = parse_with_replay_counters(&allocator, "a{width:1px,2px}");
    let declarations = property_declarations(&stylesheet);
    assert!(matches!(
        declarations[0],
        Declaration::Unparsed(value) if value.reason == UnparsedPropertyReason::InvalidValue
    ));
    // `1px` and the comma are decoded once by the typed parser and replayed
    // by the fallback; `2px` is decoded once by the fallback.
    assert_eq!(counters.decodes, 3);
    assert_eq!(counters.replay_hits, 2);
    assert_eq!(counters.sync_misses, 0);
    assert_eq!(counters.recorded, 0);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn replay_nested_failure_reuses_the_whole_typed_tape() {
    let allocator = Allocator::new();
    let (stylesheet, counters) =
        parse_with_replay_counters(&allocator, "a{width:calc(1px + var(--x))}");
    let declarations = property_declarations(&stylesheet);
    assert!(matches!(
        declarations[0],
        Declaration::Unparsed(value) if value.reason == UnparsedPropertyReason::OpaqueValue
    ));
    // calc, 1px, whitespace, +, whitespace, var, --x are all decoded by the
    // typed parser and replayed by the fallback; the closing parenthesis is
    // consumed by nested-block recovery without a second decode.
    assert_eq!(counters.decodes, 7);
    assert_eq!(counters.replay_hits, 6);
    assert_eq!(counters.sync_misses, 0);
    assert_eq!(counters.recorded, 0);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn replay_css_wide_candidate_fallback_reuses_the_ident() {
    let allocator = Allocator::new();
    let (stylesheet, counters) = parse_with_replay_counters(&allocator, "a{width:initial 5px}");
    let declarations = property_declarations(&stylesheet);
    assert!(matches!(
        declarations[0],
        Declaration::Unparsed(value) if value.reason == UnparsedPropertyReason::InvalidValue
    ));
    // `initial` is decoded by the CSS-wide attempt and replayed by the
    // fallback; the whitespace and `5px` are decoded only by the fallback.
    assert_eq!(counters.decodes, 3);
    assert_eq!(counters.replay_hits, 1);
    assert_eq!(counters.sync_misses, 0);
    assert_eq!(counters.recorded, 0);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn replay_css_wide_success_is_not_replayed() {
    let allocator = Allocator::new();
    let (stylesheet, counters) = parse_with_replay_counters(&allocator, "a{width:initial}");
    assert!(matches!(
        property_declarations(&stylesheet)[0],
        Declaration::CSSWide(property_id, keyword)
            if **property_id == PropertyId::Width && *keyword == CSSWideKeyword::Initial
    ));
    assert_eq!(counters.decodes, 0);
    assert_eq!(counters.replay_hits, 0);
    assert_eq!(counters.recorded, 0);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn custom_property_never_activates_replay() {
    let allocator = Allocator::new();
    let (stylesheet, counters) = parse_with_replay_counters(&allocator, "a{--x:red}");
    assert!(matches!(
        property_declarations(&stylesheet)[0],
        Declaration::Custom(_)
    ));
    assert_eq!(counters.decodes, 0);
    assert_eq!(counters.replay_hits, 0);
    assert_eq!(counters.recorded, 0);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn unsupported_grammar_property_stays_on_the_inactive_fast_path() {
    let allocator = Allocator::new();
    let (stylesheet, counters) = parse_with_replay_counters(&allocator, "a{cursor:pointer}");
    let declarations = property_declarations(&stylesheet);
    assert!(matches!(
        declarations[0],
        Declaration::Unparsed(value) if value.reason == UnparsedPropertyReason::UnsupportedGrammar
    ));
    assert_eq!(counters.decodes, 0);
    assert_eq!(counters.replay_hits, 0);
    assert_eq!(counters.recorded, 0);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

/// Guards the `stylesheet_capacity` divisor calibration against the
/// benchmark corpora. Any store that outgrows its preallocated capacity forces
/// a geometric reallocation during parse, which is exactly what the estimates
/// exist to avoid.
#[test]
fn capacity_estimates_cover_benchmark_corpora() {
    for source in [
        include_str!("../../../../../tasks/benchmark/files/bootstrap.css"),
        include_str!("../../../../../tasks/benchmark/files/tailwind.css"),
    ] {
        let allocator = Allocator::new();
        let mut compiler = Compiler::new(&allocator);
        let stylesheet = compiler
            .parse_stylesheet(source, ParserOptions::default())
            .unwrap();
        let capacity = stylesheet_capacity(source.len());
        assert!(
            stylesheet.rules_in_source_order().count() <= capacity.rules,
            "rules exceed capacity estimate ({} > {}) for {} bytes",
            stylesheet.rules_in_source_order().count(),
            capacity.rules,
            source.len()
        );
        assert!(
            stylesheet.declaration_block_count() <= capacity.declaration_blocks,
            "declaration blocks exceed capacity estimate ({} > {})",
            stylesheet.declaration_block_count(),
            capacity.declaration_blocks
        );
        assert!(
            stylesheet.declarations_in_source_order().count() <= capacity.declarations,
            "declarations exceed capacity estimate ({} > {})",
            stylesheet.declarations_in_source_order().count(),
            capacity.declarations
        );
        assert!(
            stylesheet.selector_value_count() <= capacity.selectors,
            "selector values exceed capacity estimate ({} > {})",
            stylesheet.selector_value_count(),
            capacity.selectors
        );
        assert!(
            stylesheet.context_value_count() <= capacity.contexts,
            "context values exceed capacity estimate ({} > {})",
            stylesheet.context_value_count(),
            capacity.contexts
        );
    }
}
