use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    mem::discriminant,
    ptr::NonNull,
};

use rocketcss_allocator::{Allocator, bit_vec::BitVec, hash_map::HashMap, vec::Vec};
use rocketcss_ast::{
    AnimationName, CSSWideKeyword, CssColor, CssRule, CustomProperty, Declaration,
    DeclarationBlock, DimensionPercentage, EnvironmentVariable, FontFaceProperty, FontFamily,
    Function, FunctionReplacement, KeyframeSelector, KeyframesName, LengthUnit, LengthValue,
    Margin, NamespaceConstraint, Padding, ParsedCaseSensitivity, PropertyId, PseudoClass,
    PseudoElement, SelectorComponent, Size, StyleRule, StyleRuleOutput, StyleRuleOutputDeclaration,
    StyleSheet, Token, TokenOrValue, Unit, UnknownAtRule, UnparsedProperty, Variable, VendorPrefix,
};
use rocketcss_visitor::{VisitMut, walk_mut};
use rustc_hash::FxHasher;

use crate::{
    BrowserHackTarget, Minify, MinifyContext, Options, OptionsOp, context::ValueContextFlags,
};

const SHORT_IDENTIFIERS: [&str; 52] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
];

impl Minify for StyleSheet<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        crate::minify_style_sheet(self, cx);
    }
}

pub(crate) fn reduce_keyframe_identifiers<'a>(
    stylesheet: &mut StyleSheet<'a>,
    cx: &mut MinifyContext,
) {
    if cx.is_enabled(Options::REDUCE_KEYFRAME_IDENTIFIERS, OptionsOp::None) {
        return;
    }
    let allocator = stylesheet.rules.bump();
    let mut references = HashMap::new_in(allocator);
    KeyframeReferenceCollector {
        references: &mut references,
    }
    .visit_style_sheet(stylesheet);
    let mut names: HashMap<'a, &'a str, &'a str> = HashMap::new_in(allocator);
    let mut next = 0;
    collect_keyframe_identifiers(&stylesheet.rules, &references, &mut names, &mut next);
    if names.is_empty() {
        return;
    }
    KeyframeIdentifierReducer { names: &names }.visit_style_sheet(stylesheet);
}

pub(crate) fn reduce_counter_style_identifiers<'a>(
    stylesheet: &mut StyleSheet<'a>,
    cx: &mut MinifyContext,
) {
    if cx.is_enabled(Options::REDUCE_COUNTER_STYLE_IDENTIFIERS, OptionsOp::None) {
        return;
    }
    let allocator = stylesheet.rules.bump();
    let mut references = HashMap::new_in(allocator);
    CounterStyleReferenceCollector {
        references: &mut references,
    }
    .visit_style_sheet(stylesheet);
    let mut names = HashMap::new_in(allocator);
    let mut next = 0;
    collect_counter_style_identifiers(&stylesheet.rules, &references, &mut names, &mut next);
    if names.is_empty() {
        return;
    }
    CounterStyleIdentifierReducer { names: &names }.visit_style_sheet(stylesheet);
}

pub(crate) fn reduce_counter_identifiers<'a>(
    stylesheet: &mut StyleSheet<'a>,
    cx: &mut MinifyContext,
) {
    if cx.is_enabled(Options::REDUCE_COUNTER_IDENTIFIERS, OptionsOp::None) {
        return;
    }
    let allocator = stylesheet.rules.bump();
    let mut occurrences = HashMap::new_in(allocator);
    CounterOccurrenceCollector {
        occurrences: &mut occurrences,
    }
    .visit_style_sheet(stylesheet);
    let mut names = HashMap::new_in(allocator);
    let mut next = 0;
    CounterIdentifierCollector {
        occurrences: &occurrences,
        names: &mut names,
        next: &mut next,
    }
    .visit_style_sheet(stylesheet);
    if names.is_empty() {
        return;
    }
    CounterIdentifierReducer { names: &names }.visit_style_sheet(stylesheet);
}

pub(crate) fn discard_unused_definitions<'a>(
    stylesheet: &mut StyleSheet<'a>,
    cx: &mut MinifyContext,
) {
    let options = cx.options();
    if options.is_enabled(
        Options::DISCARD_UNUSED_KEYFRAMES
            | Options::DISCARD_UNUSED_COUNTER_STYLES
            | Options::DISCARD_UNUSED_FONT_FACES
            | Options::DISCARD_UNUSED_NAMESPACES,
        OptionsOp::None,
    ) {
        return;
    }
    let allocator = stylesheet.rules.bump();
    let mut keyframes = HashMap::new_in(allocator);
    if options.is_enabled(Options::DISCARD_UNUSED_KEYFRAMES, OptionsOp::Any) {
        KeyframeReferenceCollector {
            references: &mut keyframes,
        }
        .visit_style_sheet(stylesheet);
    }
    let mut counter_styles = HashMap::new_in(allocator);
    if options.is_enabled(Options::DISCARD_UNUSED_COUNTER_STYLES, OptionsOp::Any) {
        CounterStyleReferenceCollector {
            references: &mut counter_styles,
        }
        .visit_style_sheet(stylesheet);
    }
    let mut font_families = HashMap::new_in(allocator);
    if options.is_enabled(Options::DISCARD_UNUSED_FONT_FACES, OptionsOp::Any) {
        FontReferenceCollector {
            references: &mut font_families,
        }
        .visit_style_sheet(stylesheet);
    }
    let mut namespace_prefixes = HashMap::new_in(allocator);
    let mut uses_any_namespace = false;
    if options.is_enabled(Options::DISCARD_UNUSED_NAMESPACES, OptionsOp::Any) {
        NamespaceReferenceCollector {
            prefixes: &mut namespace_prefixes,
            uses_any: &mut uses_any_namespace,
        }
        .visit_style_sheet(stylesheet);
    }
    UnusedDefinitionDiscarder {
        counter_styles: &counter_styles,
        font_families: &font_families,
        keyframes: &keyframes,
        namespace_prefixes: &namespace_prefixes,
        options,
        uses_any_namespace,
    }
    .visit_style_sheet(stylesheet);
}

pub(crate) fn merge_identical_identifier_rules<'a>(
    stylesheet: &mut StyleSheet<'a>,
    cx: &mut MinifyContext,
) {
    if cx.is_enabled(Options::MERGE_IDENTICAL_IDENTIFIERS, OptionsOp::None) {
        return;
    }
    let allocator = stylesheet.rules.bump();
    let mut keyframe_candidates = HashMap::new_in(allocator);
    let mut counter_style_candidates = HashMap::new_in(allocator);
    merge_identifier_rule_list(
        &mut stylesheet.rules,
        &mut keyframe_candidates,
        &mut counter_style_candidates,
    );

    let mut live_keyframes = HashMap::new_in(allocator);
    let mut live_counter_styles = HashMap::new_in(allocator);
    LiveIdentifierCollector {
        counter_styles: &mut live_counter_styles,
        keyframes: &mut live_keyframes,
    }
    .visit_style_sheet(stylesheet);

    let mut keyframe_names = HashMap::new_in(allocator);
    for (old, replacement) in keyframe_candidates.iter() {
        if let Some(replacement) = *replacement
            && !live_keyframes.contains_key(old)
        {
            keyframe_names.insert(*old, replacement);
        }
    }
    let mut counter_style_names = HashMap::new_in(allocator);
    for (old, replacement) in counter_style_candidates.iter() {
        if let Some(replacement) = *replacement
            && !live_counter_styles.contains_key(old)
        {
            counter_style_names.insert(*old, replacement);
        }
    }
    if !keyframe_names.is_empty() {
        KeyframeIdentifierReducer {
            names: &keyframe_names,
        }
        .visit_style_sheet(stylesheet);
    }
    if !counter_style_names.is_empty() {
        CounterStyleIdentifierReducer {
            names: &counter_style_names,
        }
        .visit_style_sheet(stylesheet);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum IdentifierRuleBucket {
    Keyframes { count: usize, prefix: u8 },
    CounterStyle { count: usize },
}

fn merge_identifier_rule_list<'a>(
    rules: &mut Vec<'a, CssRule<'a>>,
    keyframe_candidates: &mut HashMap<'a, &'a str, Option<&'a str>>,
    counter_style_candidates: &mut HashMap<'a, &'a str, Option<&'a str>>,
) {
    for rule in rules.iter_mut() {
        match rule {
            CssRule::Style(rule) => merge_identifier_rule_list(
                &mut rule.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            CssRule::Media(rule) => merge_identifier_rule_list(
                &mut rule.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            CssRule::Supports(rule) => merge_identifier_rule_list(
                &mut rule.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            CssRule::MozDocument(rule) => merge_identifier_rule_list(
                &mut rule.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            CssRule::Nesting(rule) => merge_identifier_rule_list(
                &mut rule.style.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            CssRule::LayerBlock(rule) => merge_identifier_rule_list(
                &mut rule.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            CssRule::Container(rule) => merge_identifier_rule_list(
                &mut rule.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            CssRule::Scope(rule) => merge_identifier_rule_list(
                &mut rule.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            CssRule::StartingStyle(rule) => merge_identifier_rule_list(
                &mut rule.rules,
                keyframe_candidates,
                counter_style_candidates,
            ),
            _ => {}
        }
    }

    let allocator = rules.bump();
    let mut bucket_heads: HashMap<'a, IdentifierRuleBucket, usize> = HashMap::new_in(allocator);
    let mut next_in_bucket = allocator.vec();
    next_in_bucket.resize(rules.len(), usize::MAX);
    for index in (0..rules.len()).rev() {
        let Some(bucket) = identifier_rule_bucket(&rules[index]) else {
            continue;
        };
        let mut candidate = bucket_heads.get(&bucket).copied();
        let mut replacement = None;
        while let Some(candidate_index) = candidate {
            if identifier_rule_bodies_equal(&rules[index], &rules[candidate_index]) {
                replacement = identifier_rule_name(&rules[candidate_index]);
                break;
            }
            let next = next_in_bucket[candidate_index];
            candidate = (next != usize::MAX).then_some(next);
        }
        if let Some(replacement) = replacement {
            if let Some(original) = identifier_rule_name(&rules[index])
                && original != replacement
            {
                match &rules[index] {
                    CssRule::Keyframes(_) => {
                        record_identifier_merge(keyframe_candidates, original, replacement)
                    }
                    CssRule::CounterStyle(_) => {
                        record_identifier_merge(counter_style_candidates, original, replacement)
                    }
                    _ => unreachable!(),
                }
            }
            rules[index] = CssRule::Ignored;
        } else if let Some(head) = bucket_heads.insert(bucket, index) {
            next_in_bucket[index] = head;
        }
    }
}

fn identifier_rule_bucket(rule: &CssRule<'_>) -> Option<IdentifierRuleBucket> {
    match rule {
        CssRule::Keyframes(rule) => Some(IdentifierRuleBucket::Keyframes {
            count: rule.keyframes.len(),
            prefix: rule.vendor_prefix.bits(),
        }),
        CssRule::CounterStyle(rule) => Some(IdentifierRuleBucket::CounterStyle {
            count: rule.declarations.output_len(),
        }),
        _ => None,
    }
}

fn identifier_rule_bodies_equal(left: &CssRule<'_>, right: &CssRule<'_>) -> bool {
    match (left, right) {
        (CssRule::Keyframes(left), CssRule::Keyframes(right)) => {
            left.vendor_prefix == right.vendor_prefix && left.keyframes == right.keyframes
        }
        (CssRule::CounterStyle(left), CssRule::CounterStyle(right)) => {
            left.declarations == right.declarations
        }
        _ => false,
    }
}

fn identifier_rule_name<'a>(rule: &CssRule<'a>) -> Option<&'a str> {
    match rule {
        CssRule::Keyframes(rule) => Some(match &*rule.name {
            KeyframesName::Ident(name) | KeyframesName::Custom(name) => *name,
        }),
        CssRule::CounterStyle(rule) => Some(rule.name),
        _ => None,
    }
}

fn record_identifier_merge<'a>(
    candidates: &mut HashMap<'a, &'a str, Option<&'a str>>,
    original: &'a str,
    replacement: &'a str,
) {
    match candidates.get(original).copied() {
        None => {
            candidates.insert(original, Some(replacement));
        }
        Some(Some(previous)) if previous != replacement => {
            candidates.insert(original, None);
        }
        _ => {}
    }
}

struct LiveIdentifierCollector<'map, 'a> {
    counter_styles: &'map mut HashMap<'a, &'a str, ()>,
    keyframes: &'map mut HashMap<'a, &'a str, ()>,
}

impl<'a> VisitMut<'a> for LiveIdentifierCollector<'_, 'a> {
    fn visit_keyframes_rule(&mut self, node: &mut rocketcss_ast::KeyframesRule<'a>) {
        let name = match &*node.name {
            KeyframesName::Ident(name) | KeyframesName::Custom(name) => *name,
        };
        self.keyframes.insert(name, ());
        walk_mut::walk_keyframes_rule(self, node);
    }

    fn visit_counter_style_rule(&mut self, node: &mut rocketcss_ast::CounterStyleRule<'a>) {
        self.counter_styles.insert(node.name, ());
        walk_mut::walk_counter_style_rule(self, node);
    }
}

struct FontReferenceCollector<'map, 'a> {
    references: &'map mut HashMap<'a, &'a str, ()>,
}

impl<'a> VisitMut<'a> for FontReferenceCollector<'_, 'a> {
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        match node {
            Declaration::FontFamily(families) => {
                for family in families.iter() {
                    collect_typed_font_family(family, self.references);
                }
            }
            Declaration::Font(font) => {
                for family in font.family.iter() {
                    collect_typed_font_family(family, self.references);
                }
            }
            Declaration::Unparsed(value)
                if value.property_id.name().eq_ignore_ascii_case("font")
                    || value.property_id.name().eq_ignore_ascii_case("font-family") =>
            {
                collect_quoted_font_tokens(&value.value, self.references);
            }
            _ => {}
        }
        walk_mut::walk_declaration(self, node);
    }
}

fn collect_typed_font_family<'a>(
    family: &FontFamily<'a>,
    references: &mut HashMap<'a, &'a str, ()>,
) {
    if let FontFamily::FamilyName(name) = family {
        references.insert(name.0, ());
    }
}

fn collect_quoted_font_tokens<'a>(
    values: &[TokenOrValue<'a>],
    references: &mut HashMap<'a, &'a str, ()>,
) {
    for value in values {
        match value {
            TokenOrValue::Token(token) => match &**token {
                Token::String(name) | Token::UnquotedFont(name) => {
                    references.insert(*name, ());
                }
                _ => {}
            },
            TokenOrValue::Function(function) => {
                collect_quoted_font_tokens(&function.arguments, references);
            }
            _ => {}
        }
    }
}

struct NamespaceReferenceCollector<'map, 'flag, 'a> {
    prefixes: &'map mut HashMap<'a, &'a str, ()>,
    uses_any: &'flag mut bool,
}

impl<'a> VisitMut<'a> for NamespaceReferenceCollector<'_, '_, 'a> {
    fn visit_selector_component(&mut self, node: &mut SelectorComponent<'a>) {
        match node {
            SelectorComponent::ExplicitAnyNamespace => *self.uses_any = true,
            SelectorComponent::Namespace { prefix, .. } => {
                self.prefixes.insert(*prefix, ());
            }
            SelectorComponent::AttributeOther(attribute) => {
                if let Some(NamespaceConstraint::Specific { prefix, .. }) = &attribute.namespace {
                    self.prefixes.insert(*prefix, ());
                }
            }
            _ => {}
        }
        walk_mut::walk_selector_component(self, node);
    }
}

struct UnusedDefinitionDiscarder<'map, 'a> {
    counter_styles: &'map HashMap<'a, &'a str, ()>,
    font_families: &'map HashMap<'a, &'a str, ()>,
    keyframes: &'map HashMap<'a, &'a str, ()>,
    namespace_prefixes: &'map HashMap<'a, &'a str, ()>,
    options: crate::MinifyOptions,
    uses_any_namespace: bool,
}

impl<'a> VisitMut<'a> for UnusedDefinitionDiscarder<'_, 'a> {
    fn visit_css_rule(&mut self, node: &mut CssRule<'a>) {
        let remove = match node {
            CssRule::Keyframes(rule)
                if self
                    .options
                    .is_enabled(Options::DISCARD_UNUSED_KEYFRAMES, OptionsOp::Any) =>
            {
                let name = match &*rule.name {
                    KeyframesName::Ident(name) | KeyframesName::Custom(name) => *name,
                };
                !self.keyframes.contains_key(name)
            }
            CssRule::CounterStyle(rule)
                if self
                    .options
                    .is_enabled(Options::DISCARD_UNUSED_COUNTER_STYLES, OptionsOp::Any) =>
            {
                !self.counter_styles.contains_key(rule.name)
            }
            CssRule::FontFace(rule)
                if self
                    .options
                    .is_enabled(Options::DISCARD_UNUSED_FONT_FACES, OptionsOp::Any) =>
            {
                font_face_family(rule).is_none_or(|family| {
                    !self
                        .font_families
                        .keys()
                        .any(|reference| reference.eq_ignore_ascii_case(family))
                })
            }
            CssRule::Namespace(rule)
                if self
                    .options
                    .is_enabled(Options::DISCARD_UNUSED_NAMESPACES, OptionsOp::Any) =>
            {
                rule.prefix.is_some_and(|prefix| {
                    !self.uses_any_namespace && !self.namespace_prefixes.contains_key(prefix)
                })
            }
            _ => false,
        };
        if remove {
            *node = CssRule::Ignored;
            return;
        }
        if let CssRule::FontFace(rule) = node
            && self
                .options
                .is_enabled(Options::DISCARD_UNUSED_FONT_FACES, OptionsOp::Any)
        {
            unquote_font_face_family(rule);
        }
        walk_mut::walk_css_rule(self, node);
    }
}

fn font_face_family<'a>(rule: &rocketcss_ast::FontFaceRule<'a>) -> Option<&'a str> {
    for property in rule.properties.iter() {
        match property {
            FontFaceProperty::FontFamily(value) => {
                if let FontFamily::FamilyName(name) = &**value {
                    return Some(name.0);
                }
            }
            FontFaceProperty::Custom(property)
                if matches!(&*property.name,
                    rocketcss_ast::CustomPropertyName::Custom(name)
                    | rocketcss_ast::CustomPropertyName::Unknown(name)
                    if name.eq_ignore_ascii_case("font-family")) =>
            {
                for value in property.value.iter() {
                    if let TokenOrValue::Token(token) = value
                        && let Token::String(name) | Token::UnquotedFont(name) = &**token
                    {
                        return Some(*name);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn unquote_font_face_family(rule: &mut rocketcss_ast::FontFaceRule<'_>) {
    for property in rule.properties.iter_mut() {
        let FontFaceProperty::Custom(property) = property else {
            continue;
        };
        if !matches!(&*property.name,
            rocketcss_ast::CustomPropertyName::Custom(name)
            | rocketcss_ast::CustomPropertyName::Unknown(name)
            if name.eq_ignore_ascii_case("font-family"))
        {
            continue;
        }
        for value in property.value.iter_mut() {
            let TokenOrValue::Token(token) = value else {
                continue;
            };
            if let Token::String(name) = &**token
                && crate::token::can_unquote_font(name)
            {
                let name = *name;
                **token = Token::UnquotedFont(name);
            }
        }
    }
}

pub(crate) fn minify_font_face_property(
    property: &mut FontFaceProperty<'_>,
    cx: &mut MinifyContext,
) {
    if let FontFaceProperty::FontWeight(value) = property {
        let rocketcss_ast::Size2D(first, second) = &mut **value;
        for weight in [first, second] {
            let weight: &mut rocketcss_ast::FontWeight<'_> = weight;
            if let rocketcss_ast::FontWeight::Absolute(absolute) = weight {
                let absolute: &mut rocketcss_ast::AbsoluteFontWeight = absolute;
                if matches!(absolute, rocketcss_ast::AbsoluteFontWeight::Normal) {
                    *absolute = rocketcss_ast::AbsoluteFontWeight::Weight(400.0);
                    cx.record_value_normalized();
                }
            }
        }
        return;
    }
    let FontFaceProperty::Custom(property) = property else {
        return;
    };
    let name = match &*property.name {
        rocketcss_ast::CustomPropertyName::Custom(name)
        | rocketcss_ast::CustomPropertyName::Unknown(name) => *name,
    };
    if name.eq_ignore_ascii_case("font-family") {
        for value in &mut property.value {
            let TokenOrValue::Token(token) = value else {
                continue;
            };
            if let Token::String(name) = &**token
                && crate::token::can_unquote_font(name)
            {
                let name = *name;
                **token = Token::UnquotedFont(name);
                cx.record_value_normalized();
            }
        }
    } else if name.eq_ignore_ascii_case("font-weight")
        && let [TokenOrValue::Token(token)] = property.value.as_mut_slice()
        && matches!(&**token, Token::Ident(value) if value.eq_ignore_ascii_case("normal"))
    {
        **token = Token::Number(400.0);
        cx.record_value_normalized();
    }
}

pub(crate) fn reduce_grid_identifiers<'a>(stylesheet: &mut StyleSheet<'a>, cx: &mut MinifyContext) {
    if cx.is_enabled(Options::REDUCE_GRID_IDENTIFIERS, OptionsOp::None) {
        return;
    }
    let allocator = stylesheet.rules.bump();
    let mut names = HashMap::new_in(allocator);
    let mut next = 0;
    GridIdentifierCollector {
        names: &mut names,
        next: &mut next,
    }
    .visit_style_sheet(stylesheet);
    GridIdentifierReducer {
        allocator,
        names: &names,
    }
    .visit_style_sheet(stylesheet);
}

struct GridIdentifierCollector<'map, 'next, 'a> {
    names: &'map mut HashMap<'a, &'a str, &'a str>,
    next: &'next mut usize,
}

impl<'a> VisitMut<'a> for GridIdentifierCollector<'_, '_, 'a> {
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        if let Declaration::Unparsed(value) = node {
            match grid_template_property(value.property_id.name()) {
                Some(GridTemplateProperty::Areas) => {
                    collect_grid_definitions(&value.value, true, false, self.names, self.next)
                }
                Some(GridTemplateProperty::Template) => {
                    collect_grid_definitions(&value.value, true, true, self.names, self.next)
                }
                Some(GridTemplateProperty::TrackList) => {
                    collect_grid_definitions(&value.value, false, true, self.names, self.next)
                }
                None => {}
            }
        }
        walk_mut::walk_declaration(self, node);
    }
}

#[derive(Clone, Copy)]
enum GridTemplateProperty {
    Areas,
    Template,
    TrackList,
}

fn grid_template_property(name: &str) -> Option<GridTemplateProperty> {
    if name.eq_ignore_ascii_case("grid-template-areas") {
        Some(GridTemplateProperty::Areas)
    } else if name.eq_ignore_ascii_case("grid-template") {
        Some(GridTemplateProperty::Template)
    } else if name.eq_ignore_ascii_case("grid-template-columns")
        || name.eq_ignore_ascii_case("grid-template-rows")
    {
        Some(GridTemplateProperty::TrackList)
    } else {
        None
    }
}

fn collect_grid_definitions<'a>(
    values: &[TokenOrValue<'a>],
    collect_areas: bool,
    collect_line_names: bool,
    names: &mut HashMap<'a, &'a str, &'a str>,
    next: &mut usize,
) {
    let mut inside_line_names = false;
    for value in values {
        match value {
            TokenOrValue::Token(token) => match &**token {
                Token::SquareBracketBlock if collect_line_names => inside_line_names = true,
                Token::CloseSquareBracket if collect_line_names => inside_line_names = false,
                Token::Ident(name) if inside_line_names => {
                    collect_grid_identifier(name, names, next);
                }
                Token::String(value) if collect_areas => {
                    for name in value.split_whitespace() {
                        if !is_grid_area_hole(name) {
                            collect_grid_identifier(name, names, next);
                        }
                    }
                }
                _ => {}
            },
            TokenOrValue::Function(function) if collect_line_names => {
                collect_grid_definitions(&function.arguments, false, true, names, next)
            }
            _ => {}
        }
    }
}

fn collect_grid_identifier<'a>(
    name: &'a str,
    names: &mut HashMap<'a, &'a str, &'a str>,
    next: &mut usize,
) {
    if names.contains_key(name) {
        return;
    }
    let Some(short) = SHORT_IDENTIFIERS.get(*next).copied() else {
        return;
    };
    names.insert(name, short);
    *next += 1;
}

fn is_grid_area_hole(name: &str) -> bool {
    !name.is_empty() && name.bytes().all(|byte| byte == b'.')
}

struct GridIdentifierReducer<'map, 'a> {
    allocator: &'a Allocator,
    names: &'map HashMap<'a, &'a str, &'a str>,
}

impl<'a> VisitMut<'a> for GridIdentifierReducer<'_, 'a> {
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        if let Declaration::Unparsed(value) = node {
            match grid_template_property(value.property_id.name()) {
                Some(GridTemplateProperty::Areas) => reduce_grid_definitions(
                    &mut value.value,
                    true,
                    false,
                    self.names,
                    self.allocator,
                ),
                Some(GridTemplateProperty::Template) => reduce_grid_definitions(
                    &mut value.value,
                    true,
                    true,
                    self.names,
                    self.allocator,
                ),
                Some(GridTemplateProperty::TrackList) => reduce_grid_definitions(
                    &mut value.value,
                    false,
                    true,
                    self.names,
                    self.allocator,
                ),
                None if is_grid_identifier_reference_property(value.property_id.name()) => {
                    reduce_identifier_tokens_recursive(&mut value.value, self.names);
                }
                None => {}
            }
        }
        walk_mut::walk_declaration(self, node);
    }
}

fn is_grid_identifier_reference_property(name: &str) -> bool {
    [
        "grid-area",
        "grid-column",
        "grid-row",
        "grid-column-start",
        "grid-column-end",
        "grid-row-start",
        "grid-row-end",
    ]
    .iter()
    .any(|property| name.eq_ignore_ascii_case(property))
}

fn reduce_grid_definitions<'a>(
    values: &mut [TokenOrValue<'a>],
    reduce_areas: bool,
    reduce_line_names: bool,
    names: &HashMap<'a, &'a str, &'a str>,
    allocator: &'a Allocator,
) {
    let mut inside_line_names = false;
    for value in values {
        match value {
            TokenOrValue::Token(token) => match &mut **token {
                Token::SquareBracketBlock if reduce_line_names => inside_line_names = true,
                Token::CloseSquareBracket if reduce_line_names => inside_line_names = false,
                Token::Ident(name) if inside_line_names => {
                    if let Some(replacement) = names.get(*name).copied() {
                        *name = replacement;
                    }
                }
                Token::String(value) if reduce_areas => {
                    reduce_grid_area_string(value, names, allocator);
                }
                _ => {}
            },
            TokenOrValue::Function(function) if reduce_line_names => {
                reduce_grid_definitions(&mut function.arguments, false, true, names, allocator)
            }
            _ => {}
        }
    }
}

fn reduce_grid_area_string<'a>(
    value: &mut &'a str,
    names: &HashMap<'a, &'a str, &'a str>,
    allocator: &'a Allocator,
) {
    let mut result = String::with_capacity(value.len());
    for (index, name) in value.split_whitespace().enumerate() {
        if index != 0 {
            result.push(' ');
        }
        if is_grid_area_hole(name) {
            result.push('.');
        } else {
            result.push_str(names.get(name).copied().unwrap_or(name));
        }
    }
    if result != *value {
        *value = allocator.alloc_str(&result);
    }
}

fn reduce_identifier_tokens_recursive<'a>(
    values: &mut [TokenOrValue<'a>],
    names: &HashMap<'a, &'a str, &'a str>,
) {
    for value in values {
        match value {
            TokenOrValue::Token(token) => {
                if let Token::Ident(name) = &mut **token
                    && let Some(replacement) = names.get(*name).copied()
                {
                    *name = replacement;
                }
            }
            TokenOrValue::Function(function) => {
                reduce_identifier_tokens_recursive(&mut function.arguments, names);
            }
            _ => {}
        }
    }
}

struct CounterOccurrenceCollector<'map, 'a> {
    occurrences: &'map mut HashMap<'a, &'a str, u32>,
}

impl<'a> VisitMut<'a> for CounterOccurrenceCollector<'_, 'a> {
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        if let Declaration::Unparsed(value) = node
            && is_counter_definition_property(value.property_id.name())
        {
            count_counter_identifiers(&value.value, self.occurrences);
        }
        walk_mut::walk_declaration(self, node);
    }

    fn visit_function(&mut self, node: &mut Function<'a>) {
        if ["counter", "counters", "reversed"]
            .iter()
            .any(|name| node.name.eq_ignore_ascii_case(name))
            && let Some(name) = first_identifier(&node.arguments)
        {
            *self.occurrences.entry(name).or_default() += 1;
        }
        walk_mut::walk_function(self, node);
    }
}

fn count_counter_identifiers<'a>(
    values: &[TokenOrValue<'a>],
    occurrences: &mut HashMap<'a, &'a str, u32>,
) {
    for value in values {
        if let TokenOrValue::Token(token) = value
            && let Token::Ident(name) = &**token
            && is_counter_identifier(name)
        {
            *occurrences.entry(*name).or_default() += 1;
        }
    }
}

fn first_identifier<'a>(values: &[TokenOrValue<'a>]) -> Option<&'a str> {
    values.iter().find_map(|value| match value {
        TokenOrValue::Token(token) => match &**token {
            Token::Ident(name) => Some(*name),
            _ => None,
        },
        _ => None,
    })
}

struct CounterIdentifierCollector<'map, 'next, 'a> {
    occurrences: &'map HashMap<'a, &'a str, u32>,
    names: &'map mut HashMap<'a, &'a str, &'a str>,
    next: &'next mut usize,
}

impl<'a> VisitMut<'a> for CounterIdentifierCollector<'_, '_, 'a> {
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        if let Declaration::Unparsed(value) = node
            && is_counter_definition_property(value.property_id.name())
        {
            collect_counter_definition_tokens(
                &value.value,
                self.occurrences,
                self.names,
                self.next,
            );
        }
        walk_mut::walk_declaration(self, node);
    }
}

fn is_counter_definition_property(name: &str) -> bool {
    ["counter-increment", "counter-reset", "counter-set"]
        .iter()
        .any(|property| name.eq_ignore_ascii_case(property))
}

fn collect_counter_definition_tokens<'a>(
    values: &[TokenOrValue<'a>],
    occurrences: &HashMap<'a, &'a str, u32>,
    names: &mut HashMap<'a, &'a str, &'a str>,
    next: &mut usize,
) {
    for value in values {
        match value {
            TokenOrValue::Token(token) => {
                if let Token::Ident(name) = &**token
                    && is_counter_identifier(name)
                    && occurrences.get(name).copied().unwrap_or(0) > 1
                    && !names.contains_key(name)
                    && let Some(short) = SHORT_IDENTIFIERS.get(*next)
                {
                    names.insert(*name, *short);
                    *next += 1;
                }
            }
            TokenOrValue::Function(function) if function.name.eq_ignore_ascii_case("reversed") => {
                collect_counter_definition_tokens(&function.arguments, occurrences, names, next);
            }
            _ => {}
        }
    }
}

fn is_counter_identifier(name: &str) -> bool {
    ![
        "none",
        "initial",
        "inherit",
        "unset",
        "revert",
        "revert-layer",
    ]
    .iter()
    .any(|keyword| name.eq_ignore_ascii_case(keyword))
}

struct CounterIdentifierReducer<'map, 'a> {
    names: &'map HashMap<'a, &'a str, &'a str>,
}

impl<'a> VisitMut<'a> for CounterIdentifierReducer<'_, 'a> {
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        if let Declaration::Unparsed(value) = node
            && is_counter_definition_property(value.property_id.name())
        {
            reduce_identifier_tokens(&mut value.value, self.names);
        }
        walk_mut::walk_declaration(self, node);
    }

    fn visit_function(&mut self, node: &mut Function<'a>) {
        if ["counter", "counters", "reversed"]
            .iter()
            .any(|name| node.name.eq_ignore_ascii_case(name))
        {
            reduce_identifier_tokens(&mut node.arguments, self.names);
        }
        walk_mut::walk_function(self, node);
    }
}

fn collect_counter_style_identifiers<'a>(
    rules: &[CssRule<'a>],
    references: &HashMap<'a, &'a str, ()>,
    names: &mut HashMap<'a, &'a str, &'a str>,
    next: &mut usize,
) {
    for rule in rules {
        if let CssRule::CounterStyle(rule) = rule
            && references.contains_key(rule.name)
            && !names.contains_key(rule.name)
            && let Some(short) = SHORT_IDENTIFIERS.get(*next)
        {
            names.insert(rule.name, *short);
            *next += 1;
        }
        match rule {
            CssRule::Style(rule) => {
                collect_counter_style_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Media(rule) => {
                collect_counter_style_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Supports(rule) => {
                collect_counter_style_identifiers(&rule.rules, references, names, next)
            }
            CssRule::MozDocument(rule) => {
                collect_counter_style_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Nesting(rule) => {
                collect_counter_style_identifiers(&rule.style.rules, references, names, next)
            }
            CssRule::LayerBlock(rule) => {
                collect_counter_style_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Container(rule) => {
                collect_counter_style_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Scope(rule) => {
                collect_counter_style_identifiers(&rule.rules, references, names, next)
            }
            CssRule::StartingStyle(rule) => {
                collect_counter_style_identifiers(&rule.rules, references, names, next)
            }
            _ => {}
        }
    }
}

struct CounterStyleReferenceCollector<'map, 'a> {
    references: &'map mut HashMap<'a, &'a str, ()>,
}

impl<'a> VisitMut<'a> for CounterStyleReferenceCollector<'_, 'a> {
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        if let Declaration::Unparsed(value) = node
            && is_counter_style_reference_property(value.property_id.name())
        {
            collect_identifier_tokens(&value.value, self.references);
        }
        walk_mut::walk_declaration(self, node);
    }
}

fn is_counter_style_reference_property(name: &str) -> bool {
    ["list-style", "list-style-type", "system"]
        .iter()
        .any(|property| name.eq_ignore_ascii_case(property))
}

struct CounterStyleIdentifierReducer<'map, 'a> {
    names: &'map HashMap<'a, &'a str, &'a str>,
}

impl<'a> VisitMut<'a> for CounterStyleIdentifierReducer<'_, 'a> {
    fn visit_counter_style_rule(&mut self, node: &mut rocketcss_ast::CounterStyleRule<'a>) {
        if let Some(replacement) = self.names.get(node.name).copied() {
            node.name = replacement;
        }
        walk_mut::walk_counter_style_rule(self, node);
    }

    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        if let Declaration::Unparsed(value) = node
            && is_counter_style_reference_property(value.property_id.name())
        {
            reduce_identifier_tokens(&mut value.value, self.names);
        }
        walk_mut::walk_declaration(self, node);
    }
}

fn collect_keyframe_identifiers<'a>(
    rules: &[CssRule<'a>],
    references: &HashMap<'a, &'a str, ()>,
    names: &mut HashMap<'a, &'a str, &'a str>,
    next: &mut usize,
) {
    for rule in rules {
        if let CssRule::Keyframes(rule) = rule {
            let original = match &*rule.name {
                KeyframesName::Ident(name) | KeyframesName::Custom(name) => *name,
            };
            if references.contains_key(original)
                && !names.contains_key(original)
                && let Some(short) = SHORT_IDENTIFIERS.get(*next)
            {
                names.insert(original, *short);
                *next += 1;
            }
        }
        match rule {
            CssRule::Style(rule) => {
                collect_keyframe_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Media(rule) => {
                collect_keyframe_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Supports(rule) => {
                collect_keyframe_identifiers(&rule.rules, references, names, next)
            }
            CssRule::MozDocument(rule) => {
                collect_keyframe_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Nesting(rule) => {
                collect_keyframe_identifiers(&rule.style.rules, references, names, next)
            }
            CssRule::LayerBlock(rule) => {
                collect_keyframe_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Container(rule) => {
                collect_keyframe_identifiers(&rule.rules, references, names, next)
            }
            CssRule::Scope(rule) => {
                collect_keyframe_identifiers(&rule.rules, references, names, next)
            }
            CssRule::StartingStyle(rule) => {
                collect_keyframe_identifiers(&rule.rules, references, names, next)
            }
            _ => {}
        }
    }
}

struct KeyframeReferenceCollector<'map, 'a> {
    references: &'map mut HashMap<'a, &'a str, ()>,
}

impl<'a> VisitMut<'a> for KeyframeReferenceCollector<'_, 'a> {
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        match node {
            Declaration::AnimationName(names, _) => {
                for name in names.iter() {
                    if let AnimationName::Ident(name) | AnimationName::String(name) = name {
                        self.references.insert(*name, ());
                    }
                }
            }
            Declaration::Animation(animations, _) => {
                for animation in animations.iter() {
                    if let AnimationName::Ident(name) | AnimationName::String(name) =
                        &*animation.name
                    {
                        self.references.insert(*name, ());
                    }
                }
            }
            Declaration::Unparsed(value) if is_animation_property(value.property_id.name()) => {
                collect_identifier_tokens(&value.value, self.references);
            }
            _ => {}
        }
        walk_mut::walk_declaration(self, node);
    }
}

fn is_animation_property(name: &str) -> bool {
    let unprefixed = name
        .strip_prefix('-')
        .and_then(|name| name.split_once('-').map(|(_, name)| name))
        .unwrap_or(name);
    unprefixed.eq_ignore_ascii_case("animation")
        || unprefixed.eq_ignore_ascii_case("animation-name")
}

fn collect_identifier_tokens<'a>(
    values: &[TokenOrValue<'a>],
    references: &mut HashMap<'a, &'a str, ()>,
) {
    for value in values {
        match value {
            TokenOrValue::Token(token) => {
                if let Token::Ident(name) = &**token {
                    references.insert(*name, ());
                }
            }
            TokenOrValue::AnimationName(name) => {
                if let AnimationName::Ident(name) | AnimationName::String(name) = &**name {
                    references.insert(*name, ());
                }
            }
            _ => {}
        }
    }
}

struct KeyframeIdentifierReducer<'map, 'a> {
    names: &'map HashMap<'a, &'a str, &'a str>,
}

impl<'a> VisitMut<'a> for KeyframeIdentifierReducer<'_, 'a> {
    fn visit_keyframes_rule(&mut self, node: &mut rocketcss_ast::KeyframesRule<'a>) {
        let original = match &*node.name {
            KeyframesName::Ident(name) | KeyframesName::Custom(name) => *name,
        };
        if let Some(replacement) = self.names.get(original).copied() {
            *node.name = KeyframesName::Ident(replacement);
        }
        walk_mut::walk_keyframes_rule(self, node);
    }

    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        match node {
            Declaration::AnimationName(names, _) => {
                for name in names.iter_mut() {
                    reduce_animation_name(name, self.names);
                }
            }
            Declaration::Animation(animations, _) => {
                for animation in animations.iter_mut() {
                    reduce_animation_name(&mut animation.name, self.names);
                }
            }
            Declaration::Unparsed(value) if is_animation_property(value.property_id.name()) => {
                reduce_identifier_tokens(&mut value.value, self.names);
            }
            _ => {}
        }
        walk_mut::walk_declaration(self, node);
    }
}

fn reduce_identifier_tokens<'a>(
    values: &mut [TokenOrValue<'a>],
    names: &HashMap<'a, &'a str, &'a str>,
) {
    for value in values {
        match value {
            TokenOrValue::Token(token) => {
                if let Token::Ident(name) = &mut **token
                    && let Some(replacement) = names.get(*name).copied()
                {
                    *name = replacement;
                }
            }
            TokenOrValue::AnimationName(name) => reduce_animation_name(name, names),
            _ => {}
        }
    }
}

fn reduce_animation_name<'a>(name: &mut AnimationName<'a>, names: &HashMap<'a, &'a str, &'a str>) {
    let original = match name {
        AnimationName::Ident(name) | AnimationName::String(name) => *name,
        AnimationName::None => return,
    };
    if let Some(replacement) = names.get(original).copied() {
        *name = AnimationName::Ident(replacement);
    }
}

impl Minify for KeyframeSelector<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
            return;
        }

        let changed = match self {
            KeyframeSelector::From => {
                *self = KeyframeSelector::Percentage(0.0);
                true
            }
            KeyframeSelector::Percentage(value) if *value == 1.0 => {
                *self = KeyframeSelector::To;
                true
            }
            _ => false,
        };
        if changed {
            cx.record_value_normalized();
        }
    }
}

impl Minify for Declaration<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if cx.is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any) {
            let size = match self {
                Declaration::Width(value)
                | Declaration::Height(value)
                | Declaration::MinWidth(value)
                | Declaration::MinHeight(value)
                | Declaration::BlockSize(value)
                | Declaration::InlineSize(value)
                | Declaration::MinBlockSize(value)
                | Declaration::MinInlineSize(value) => Some(&mut **value),
                _ => None,
            };
            if size.is_some_and(minify_size_percentage_zero) {
                cx.record_value_normalized();
            }
        }
        if let Declaration::Opacity(value) = self {
            let clamped = value.clamp(0.0, 1.0);
            if *value != clamped {
                *value = clamped;
                cx.record_value_normalized();
            }
        }
        if cx.is_enabled(Options::REDUCE_TO_INITIAL, OptionsOp::None) {
            return;
        }
        if let Declaration::BackgroundColor(color) = self
            && matches!(&**color, CssColor::Rgba(value) if value.red == 0 && value.green == 0 && value.blue == 0 && value.alpha == 0)
        {
            **color = CssColor::Initial;
            cx.record_value_normalized();
        }
    }
}

fn minify_size_percentage_zero(size: &mut Size<'_>) -> bool {
    let Size::LengthPercentage(value) = size else {
        return false;
    };
    if matches!(&**value, DimensionPercentage::Percentage(0.0)) {
        **value = DimensionPercentage::Zero;
        true
    } else {
        false
    }
}

impl Minify for UnparsedProperty<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.value.minify(cx);
        if self
            .property_id
            .name()
            .eq_ignore_ascii_case("shape-image-threshold")
            && let [TokenOrValue::Token(token)] = self.value.as_mut_slice()
            && let Token::Number(value) = &mut **token
        {
            let clamped = value.clamp(0.0, 1.0);
            if *value != clamped {
                *value = clamped;
                cx.record_value_normalized();
            }
        }
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
            return;
        }
        if self.property_id.name().eq_ignore_ascii_case("columns") {
            *self.property_id = PropertyId::from_name("columns");
            canonicalize_auto(&mut self.value);
        }
        if self.property_id.name().eq_ignore_ascii_case("columns")
            && minify_columns(&mut self.value)
        {
            *self.property_id = PropertyId::from_name("column-count");
            cx.record_value_normalized();
        }
        let property_name = self.property_id.name();
        if property_name
            .get(..6)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("border"))
        {
            let variable_count = self
                .value
                .iter()
                .filter(|value| token_or_value_contains_variable(value))
                .count();
            if cx.is_enabled(Options::ORDER_BORDER_VALUES_WITH_VARIABLES, OptionsOp::Any)
                && variable_count == 1
                && is_border_shorthand(property_name)
                && order_border_with_trailing_variable(&mut self.value)
            {
                cx.record_value_normalized();
            }
            if variable_count < 2 {
                canonicalize_border_keywords(&mut self.value);
                if is_border_shorthand(property_name) {
                    canonicalize_full_border_keywords(&mut self.value);
                }
            }
            if property_name.eq_ignore_ascii_case("border") {
                minify_default_border(&mut self.value, cx);
            }
        }
        if cx.is_enabled(Options::REDUCE_TO_INITIAL, OptionsOp::Any)
            && is_multi_token_initial_value(self.property_id.name(), &self.value)
        {
            let TokenOrValue::Token(token) = &mut self.value[0] else {
                return;
            };
            **token = Token::Ident("initial");
            self.value.truncate(1);
            cx.record_value_normalized();
            return;
        }
        if self.value.len() != 1 {
            return;
        }
        let allocator = self.value.bump();
        let TokenOrValue::Token(token) = &mut self.value[0] else {
            return;
        };
        let Token::Ident(value) = &**token else {
            return;
        };
        if cx.is_enabled(Options::REDUCE_TO_INITIAL, OptionsOp::Any)
            && crate::properties::is_initial_value(self.property_id.name(), value)
        {
            **token = Token::Ident("initial");
            cx.record_value_normalized();
            return;
        }
        if !value.eq_ignore_ascii_case("initial") {
            return;
        }
        if cx.is_enabled(Options::PRESERVE_MERGED_BOX_INITIAL, OptionsOp::Any)
            && is_margin_or_padding_longhand(self.property_id.name())
        {
            return;
        }
        if matches!(
            self.property_id.name(),
            "background-position" | "mask-position"
        ) {
            **token = Token::Number(0.0);
            self.value
                .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
            self.value
                .push(TokenOrValue::Token(allocator.boxed(Token::Number(0.0))));
            cx.record_value_normalized();
            return;
        }
        let Some(replacement) = crate::properties::initial_value(self.property_id.name()) else {
            return;
        };
        **token = match replacement {
            "0" | "0.0" => Token::Number(0.0),
            "1" => Token::Number(1.0),
            "2" => Token::Number(2.0),
            "8" => Token::Number(8.0),
            "0%" => Token::Percentage(0.0),
            "100%" => Token::Percentage(1.0),
            "0px" => Token::Dimension {
                unit: Unit::Length(LengthUnit::Px),
                value: 0.0,
            },
            "0s" => Token::Dimension {
                unit: Unit::Seconds,
                value: 0.0,
            },
            "1dppx" => Token::Dimension {
                unit: Unit::Dppx,
                value: 1.0,
            },
            _ => Token::Ident(replacement),
        };
        cx.record_value_normalized();
        self.value[0].minify(cx);
        self.value.minify(cx);
    }
}

fn is_border_shorthand(name: &str) -> bool {
    [
        "border",
        "border-top",
        "border-right",
        "border-bottom",
        "border-left",
    ]
    .into_iter()
    .any(|property| name.eq_ignore_ascii_case(property))
}

fn order_border_with_trailing_variable(value: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let mut items = [usize::MAX; 3];
    let mut count = 0;
    for (index, item) in value.iter().enumerate() {
        if !is_token_whitespace(item) {
            if count == 3 {
                return false;
            }
            items[count] = index;
            count += 1;
        }
    }
    if count != 3 || !token_or_value_contains_variable(&value[items[2]]) {
        return false;
    }
    let width = border_component(&value[items[0]]) == Some(BorderComponent::Width)
        || border_component(&value[items[1]]) == Some(BorderComponent::Width);
    let style = border_component(&value[items[0]]) == Some(BorderComponent::Style)
        || border_component(&value[items[1]]) == Some(BorderComponent::Style);
    if !width || !style || border_component(&value[items[0]]) != Some(BorderComponent::Style) {
        return false;
    }
    value.swap(items[0], items[1]);
    true
}

fn canonicalize_border_keywords(value: &mut Vec<'_, TokenOrValue<'_>>) {
    for value in value {
        let TokenOrValue::Token(token) = value else {
            continue;
        };
        let Token::Ident(keyword) = &**token else {
            continue;
        };
        let Some(canonical) = canonical_border_keyword(keyword) else {
            continue;
        };
        **token = Token::Ident(canonical);
    }
}

fn canonical_border_keyword(value: &str) -> Option<&'static str> {
    [
        "currentcolor",
        "transparent",
        "red",
        "blue",
        "green",
        "black",
        "white",
    ]
    .into_iter()
    .find(|keyword| value.eq_ignore_ascii_case(keyword))
}

fn minify_default_border(value: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let [first, space_1, second, space_2, third] = value.as_slice() else {
        return;
    };
    if !matches!(space_1, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
        || !matches!(space_2, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
        || !token_ident(first).is_some_and(|value| value.eq_ignore_ascii_case("medium"))
        || !token_ident(second).is_some_and(|value| value.eq_ignore_ascii_case("none"))
        || !token_ident(third).is_some_and(|value| value.eq_ignore_ascii_case("currentcolor"))
    {
        return;
    }
    let TokenOrValue::Token(token) = &mut value[2] else {
        unreachable!()
    };
    **token = Token::Ident("none");
    value.swap(0, 2);
    value.truncate(1);
    cx.record_value_normalized();
}

fn minify_columns(value: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let [first, space, second] = value.as_slice() else {
        return false;
    };
    if !matches!(space, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_))) {
        return false;
    }
    let number_index = if matches!(first, TokenOrValue::Token(token) if matches!(**token, Token::Number(_)))
        && matches!(second, TokenOrValue::Token(token) if matches!(&**token, Token::Ident(value) if value.eq_ignore_ascii_case("auto")))
    {
        0
    } else if matches!(second, TokenOrValue::Token(token) if matches!(**token, Token::Number(_)))
        && matches!(first, TokenOrValue::Token(token) if matches!(&**token, Token::Ident(value) if value.eq_ignore_ascii_case("auto")))
    {
        2
    } else {
        return false;
    };
    if number_index == 2 {
        value.swap(0, 2);
    }
    value.truncate(1);
    true
}

fn is_multi_token_initial_value(property: &str, value: &[TokenOrValue<'_>]) -> bool {
    let expected: &[&str] = match property {
        "background-size" => &["auto", "auto"],
        "font-synthesis" => &["weight", "style", "small-caps", "position"],
        "text-emphasis-position" => &["over", "right"],
        "transform-origin" => &["50%", "50%", "0"],
        _ => return false,
    };
    let mut actual = value.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
    });
    expected.iter().all(|expected| {
        actual
            .next()
            .is_some_and(|value| token_matches(value, expected))
    }) && actual.next().is_none()
}

fn token_matches(value: &TokenOrValue<'_>, expected: &str) -> bool {
    let TokenOrValue::Token(token) = value else {
        return false;
    };
    match (&**token, expected) {
        (Token::Ident(value), expected) => value.eq_ignore_ascii_case(expected),
        (Token::Number(value), "0") => *value == 0.0,
        (Token::Percentage(value), "50%") => *value == 0.5,
        _ => false,
    }
}

impl Minify for CustomProperty<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.value.minify(cx);
    }
}

impl Minify for Function<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if cx
            .value_context
            .is_enabled(ValueContextFlags::SKIP_VALUE_TRANSFORMS)
        {
            return;
        }
        if let Some(canonical) = ["rgb", "rgba", "hsl", "hsla", "hwb"]
            .into_iter()
            .find(|candidate| self.name.eq_ignore_ascii_case(candidate))
        {
            self.name = canonical;
            canonicalize_nested_variable_functions(&mut self.arguments);
        }
        let is_gradient = is_gradient_function(self.name);
        let gradient_contains_variable =
            is_gradient && self.arguments.iter().any(token_or_value_contains_variable);
        if gradient_contains_variable {
            rollback_gradient_color_replacements(&mut self.arguments);
        }
        let preserve_space_after_comma = cx
            .value_context
            .is_enabled(ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA);
        cx.value_context.set_enabled(
            ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA,
            cx.is_enabled(Options::PRESERVE_VARIABLE_FALLBACK_SPACE, OptionsOp::Any)
                && ["var", "env", "constant"]
                    .iter()
                    .any(|name| self.name.eq_ignore_ascii_case(name)),
        );
        self.arguments.minify(cx);
        cx.value_context.set_enabled(
            ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA,
            preserve_space_after_comma,
        );
        if is_gradient
            && !gradient_contains_variable
            && (minify_gradient_direction(&mut self.arguments)
                | minify_gradient_stops(&mut self.arguments))
        {
            cx.record_value_normalized();
        }
        if cx
            .value_context
            .is_enabled(ValueContextFlags::MINIFY_COLORS)
            && let Some(color) =
                minify_rgb_function(self, cx).or_else(|| minify_hsl_function(self, cx))
        {
            self.replacement = Some(color);
            cx.record_value_normalized();
            return;
        }
        if self.name.eq_ignore_ascii_case("calc") {
            if let Some(linear) = calc_linear_expression(&self.arguments)
                .map(|linear| linear.round(cx.options().calc_precision))
                && linear.write_to(self)
            {
                cx.record_value_normalized();
                if self.replacement.is_some() {
                    return;
                }
            }
            if remove_redundant_calc_parentheses(&mut self.arguments) {
                cx.record_value_normalized();
            }
            if minify_flat_calc_operations(&mut self.arguments) {
                cx.record_value_normalized();
            }
            if let Some(value) = simple_calc_value(&self.arguments) {
                self.replacement = Some(value);
                self.arguments.clear();
                cx.record_value_normalized();
                return;
            }
        }
        if self.name.eq_ignore_ascii_case("url") {
            if cx.is_enabled(Options::NORMALIZE_URLS, OptionsOp::Any) {
                self.name = "url";
                let allocator = self.arguments.bump();
                if let [TokenOrValue::Token(token)] = self.arguments.as_mut_slice()
                    && let Token::String(value) = &mut **token
                {
                    if let Some(normalized) = normalize_url_text(value) {
                        *value = allocator.alloc_str(&normalized);
                        cx.record_value_normalized();
                    }
                    self.unquoted_url = !value
                        .get(..5)
                        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("data:"))
                        && can_unquote_url(value);
                }
            } else if matches!(self.arguments.as_slice(), [TokenOrValue::Token(token)]
                if matches!(&**token, Token::String(value)
                    if !value.get(..5).is_some_and(|prefix| prefix.eq_ignore_ascii_case("data:"))
                        && can_unquote_url(value)))
            {
                self.unquoted_url = true;
                cx.record_value_normalized();
            }
        }
        if cx.value_context.property == crate::context::PropertyContext::Transform
            && minify_transform_function(self)
        {
            cx.record_value_normalized();
        }
        if !matches!(
            cx.value_context.property,
            crate::context::PropertyContext::TimingFunction
                | crate::context::PropertyContext::Animation
                | crate::context::PropertyContext::Transition
        ) {
            return;
        }

        let replacement = if self.name.eq_ignore_ascii_case("cubic-bezier") {
            minify_cubic_bezier(&self.arguments)
        } else if self.name.eq_ignore_ascii_case("steps") {
            minify_steps(&mut self.arguments)
        } else {
            None
        };
        if let Some(replacement) = replacement {
            self.name = replacement;
            self.arguments.clear();
            self.is_identifier = true;
            cx.record_value_normalized();
        }
    }
}

fn canonicalize_nested_variable_functions(arguments: &mut Vec<'_, TokenOrValue<'_>>) {
    for argument in arguments {
        let TokenOrValue::Function(function) = argument else {
            continue;
        };
        if function.name.eq_ignore_ascii_case("var") {
            function.name = "var";
        } else if function.name.eq_ignore_ascii_case("env") {
            function.name = "env";
        }
        canonicalize_nested_variable_functions(&mut function.arguments);
    }
}

fn rollback_gradient_color_replacements(arguments: &mut Vec<'_, TokenOrValue<'_>>) {
    for argument in arguments {
        let TokenOrValue::Function(function) = argument else {
            continue;
        };
        if matches!(
            function.replacement,
            Some(
                FunctionReplacement::Rgb { .. }
                    | FunctionReplacement::Rgba { .. }
                    | FunctionReplacement::GrayAlpha { .. }
            )
        ) {
            function.replacement = None;
        }
    }
}

pub(crate) fn is_gradient_function(name: &str) -> bool {
    let name = name
        .strip_prefix('-')
        .and_then(|name| name.split_once('-').map(|(_, name)| name))
        .unwrap_or(name);
    [
        "linear-gradient",
        "repeating-linear-gradient",
        "radial-gradient",
        "repeating-radial-gradient",
        "conic-gradient",
        "repeating-conic-gradient",
    ]
    .iter()
    .any(|candidate| name.eq_ignore_ascii_case(candidate))
}

fn minify_gradient_direction(arguments: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let end = arguments
        .iter()
        .position(
            |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        )
        .unwrap_or(arguments.len());
    let mut items = arguments[..end]
        .iter()
        .enumerate()
        .filter(|(_, value)| !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_))));
    let Some((to_index, to)) = items.next() else {
        return false;
    };
    let Some((direction_index, direction)) = items.next() else {
        return false;
    };
    if items.next().is_some()
        || !matches!(to, TokenOrValue::Token(token) if matches!(&**token, Token::Ident(value) if value.eq_ignore_ascii_case("to")))
    {
        return false;
    }
    let Some(degrees) = (match direction {
        TokenOrValue::Token(token) => match &**token {
            Token::Ident(value) if value.eq_ignore_ascii_case("top") => Some(0.0),
            Token::Ident(value) if value.eq_ignore_ascii_case("right") => Some(90.0),
            Token::Ident(value) if value.eq_ignore_ascii_case("bottom") => Some(180.0),
            Token::Ident(value) if value.eq_ignore_ascii_case("left") => Some(270.0),
            _ => None,
        },
        _ => None,
    }) else {
        return false;
    };
    let TokenOrValue::Token(token) = &mut arguments[to_index] else {
        return false;
    };
    **token = Token::Dimension {
        unit: Unit::Deg,
        value: degrees,
    };
    arguments.drain(to_index + 1..=direction_index);
    true
}

fn minify_gradient_stops(arguments: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let mut changed = false;
    if let Some((color_index, position_index)) = first_gradient_stop(arguments)
        && is_zero_gradient_position(&arguments[position_index])
    {
        if let TokenOrValue::Function(function) = &mut arguments[color_index]
            && matches!(
                function.replacement,
                Some(FunctionReplacement::Rgba { alpha: 0.0, .. })
            )
        {
            function.name = "transparent";
            function.arguments.clear();
            function.replacement = None;
            function.is_identifier = true;
        }
        arguments.drain(color_index + 1..=position_index);
        changed = true;
    }
    if let Some((color_index, position_index)) = last_gradient_stop(arguments)
        && is_full_gradient_position(&arguments[position_index])
    {
        arguments.drain(color_index + 1..=position_index);
        changed = true;
    }
    changed | clamp_gradient_stop_positions(arguments)
}

fn first_gradient_stop(arguments: &[TokenOrValue<'_>]) -> Option<(usize, usize)> {
    let mut start = 0;
    loop {
        let end = next_comma(arguments, start);
        if !is_gradient_prelude(arguments, start, end) {
            return gradient_stop(arguments, start, end);
        }
        if end == arguments.len() {
            return None;
        }
        start = end + 1;
    }
}

fn last_gradient_stop(arguments: &[TokenOrValue<'_>]) -> Option<(usize, usize)> {
    let start = arguments
        .iter()
        .rposition(
            |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        )
        .map_or(0, |index| index + 1);
    gradient_stop(arguments, start, arguments.len())
}

fn is_gradient_prelude(arguments: &[TokenOrValue<'_>], start: usize, end: usize) -> bool {
    let Some(first) = arguments[start..end].iter().find(|value| {
        !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
    }) else {
        return true;
    };
    match first {
        TokenOrValue::Angle(_) => true,
        TokenOrValue::Token(token) => match &**token {
            Token::Number(_) | Token::Percentage(_) => true,
            Token::Dimension { unit, .. } => !unit.is_length(),
            Token::Ident(value) => [
                "at",
                "to",
                "center",
                "circle",
                "ellipse",
                "closest-side",
                "closest-corner",
                "farthest-side",
                "farthest-corner",
                "contain",
                "cover",
            ]
            .iter()
            .any(|keyword| value.eq_ignore_ascii_case(keyword)),
            _ => false,
        },
        _ => false,
    }
}

fn next_comma(arguments: &[TokenOrValue<'_>], start: usize) -> usize {
    arguments[start..]
        .iter()
        .position(
            |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        )
        .map_or(arguments.len(), |index| start + index)
}

fn gradient_stop(
    arguments: &[TokenOrValue<'_>],
    start: usize,
    end: usize,
) -> Option<(usize, usize)> {
    let mut items = arguments[start..end]
        .iter()
        .enumerate()
        .filter(|(_, value)| !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_))))
        .map(|(index, _)| start + index);
    let color = items.next()?;
    let position = items.next()?;
    if items.next().is_some()
        || !is_color_value(&arguments[color])
        || gradient_position(&arguments[position]).is_none()
    {
        return None;
    }
    Some((color, position))
}

#[derive(Clone, Copy)]
enum GradientPosition {
    Number(f32),
    Percentage(f32),
    Length(LengthUnit, f32),
}

fn gradient_position(value: &TokenOrValue<'_>) -> Option<GradientPosition> {
    match value {
        TokenOrValue::Length(value) => Some(GradientPosition::Length(value.unit, value.value)),
        TokenOrValue::Function(function) => match function.replacement {
            Some(FunctionReplacement::Number(value)) => Some(GradientPosition::Number(value)),
            Some(FunctionReplacement::Percentage(value)) => {
                Some(GradientPosition::Percentage(value))
            }
            Some(FunctionReplacement::Dimension {
                unit: Unit::Length(unit),
                value,
            }) => Some(GradientPosition::Length(unit, value)),
            _ => None,
        },
        TokenOrValue::Token(token) => match **token {
            Token::Number(value) => Some(GradientPosition::Number(value)),
            Token::Percentage(value) => Some(GradientPosition::Percentage(value)),
            Token::Dimension {
                unit: Unit::Length(unit),
                value,
            } => Some(GradientPosition::Length(unit, value)),
            _ => None,
        },
        _ => None,
    }
}

fn clamp_gradient_stop_positions(arguments: &mut [TokenOrValue<'_>]) -> bool {
    let mut start = 0;
    let mut previous = None;
    let mut changed = false;
    loop {
        let end = next_comma(arguments, start);
        if let Some((_, position_index)) = gradient_stop(arguments, start, end) {
            let current = gradient_position(&arguments[position_index])
                .expect("gradient_stop validates its position");
            if previous.is_some_and(|previous| gradient_position_lte(current, previous)) {
                set_gradient_position_zero(&mut arguments[position_index]);
                changed = true;
            } else {
                previous = Some(current);
            }
        }
        if end == arguments.len() {
            return changed;
        }
        start = end + 1;
    }
}

fn gradient_position_lte(left: GradientPosition, right: GradientPosition) -> bool {
    match (left, right) {
        (GradientPosition::Number(left), GradientPosition::Number(right))
        | (GradientPosition::Percentage(left), GradientPosition::Percentage(right)) => {
            left <= right
        }
        (
            GradientPosition::Length(left_unit, left),
            GradientPosition::Length(right_unit, right),
        ) if left_unit == right_unit => left <= right,
        (GradientPosition::Number(0.0), _) => true,
        _ => false,
    }
}

fn set_gradient_position_zero(value: &mut TokenOrValue<'_>) {
    match value {
        TokenOrValue::Length(value) => value.value = 0.0,
        TokenOrValue::Function(function) => {
            function.arguments.clear();
            function.replacement = Some(FunctionReplacement::Number(0.0));
        }
        TokenOrValue::Token(token) => **token = Token::Number(0.0),
        _ => {}
    }
}

fn is_zero_gradient_position(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token)
        if matches!(**token, Token::Number(0.0) | Token::Percentage(0.0)))
}

fn is_full_gradient_position(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Percentage(1.0)))
}

fn is_color_value(value: &TokenOrValue<'_>) -> bool {
    matches!(
        value,
        TokenOrValue::Color(_) | TokenOrValue::UnresolvedColor(_)
    ) || matches!(value, TokenOrValue::Function(function)
            if ["rgb", "rgba", "hsl", "hsla", "hwb", "lab", "lch", "color"]
                .iter()
                .any(|name| function.name.eq_ignore_ascii_case(name)))
        || matches!(value, TokenOrValue::Token(token)
            if matches!(**token, Token::Ident(_) | Token::Hash(_) | Token::IdHash(_) | Token::MinifiedHash(_)))
}

fn minify_hsl_function(function: &Function<'_>, cx: &MinifyContext) -> Option<FunctionReplacement> {
    let is_hsl = function.name.eq_ignore_ascii_case("hsl");
    let is_hsla = function.name.eq_ignore_ascii_case("hsla");
    if !is_hsl && !is_hsla {
        return None;
    }
    let mut components = function.arguments.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token)
            if matches!(**token, Token::WhiteSpace(_) | Token::Comma | Token::Delim("/")))
    });
    let hue = color_number(components.next()?)?;
    let saturation = color_percentage(components.next()?)?;
    let lightness = color_percentage(components.next()?)?;
    let alpha = match components.next() {
        Some(value) => color_alpha(value)?,
        None if is_hsl => 1.0,
        None => return None,
    };
    if components.next().is_some() {
        return None;
    }
    let hue = hue.rem_euclid(360.0) / 60.0;
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = chroma * (1.0 - (hue.rem_euclid(2.0) - 1.0).abs());
    let (red, green, blue) = match hue as u8 {
        0 => (chroma, x, 0.0),
        1 => (x, chroma, 0.0),
        2 => (0.0, chroma, x),
        3 => (0.0, x, chroma),
        4 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };
    let match_value = lightness - chroma / 2.0;
    let red = ((red + match_value) * 255.0).round() as u8;
    let green = ((green + match_value) * 255.0).round() as u8;
    let blue = ((blue + match_value) * 255.0).round() as u8;
    Some(if alpha == 1.0 {
        FunctionReplacement::Rgb { red, green, blue }
    } else if red == green && green == blue && red > 0 && (lightness * 100.0).fract() == 0.0 {
        FunctionReplacement::GrayAlpha {
            alpha: (alpha * 1000.0).round() / 1000.0,
            lightness,
        }
    } else {
        FunctionReplacement::Rgba {
            alpha,
            red,
            green,
            blue,
            use_hex: cx.is_enabled(Options::USE_HEX_ALPHA_COLORS, OptionsOp::Any),
        }
    })
}

fn color_number(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    let Token::Number(value) = **token else {
        return None;
    };
    Some(value)
}

fn color_percentage(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Percentage(value) => Some(value),
        Token::Number(0.0) => Some(0.0),
        _ => None,
    }
}

fn minify_rgb_function(function: &Function<'_>, cx: &MinifyContext) -> Option<FunctionReplacement> {
    let is_rgb = function.name.eq_ignore_ascii_case("rgb");
    let is_rgba = function.name.eq_ignore_ascii_case("rgba");
    if !is_rgb && !is_rgba {
        return None;
    }
    let mut components = function.arguments.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token)
            if matches!(**token, Token::WhiteSpace(_) | Token::Comma | Token::Delim("/")))
    });
    let (red, red_percentage, red_normalized) = color_component(components.next()?)?;
    let (green, green_percentage, green_normalized) = color_component(components.next()?)?;
    let (blue, blue_percentage, blue_normalized) = color_component(components.next()?)?;
    let uses_percentage = red_percentage.or(green_percentage).or(blue_percentage);
    if [red_percentage, green_percentage, blue_percentage]
        .into_iter()
        .flatten()
        .any(|component| Some(component) != uses_percentage)
    {
        return None;
    }
    let alpha = match components.next() {
        Some(value) => color_alpha(value)?,
        None if is_rgb => 1.0,
        None => return None,
    };
    if components.next().is_some() {
        return None;
    }
    if alpha != 1.0 {
        let lightness = (red_normalized + green_normalized + blue_normalized) / 3.0;
        return Some(
            if red == green && green == blue && red > 0 && (lightness * 100.0).fract() == 0.0 {
                FunctionReplacement::GrayAlpha { alpha, lightness }
            } else {
                FunctionReplacement::Rgba {
                    alpha,
                    red,
                    green,
                    blue,
                    use_hex: cx.is_enabled(Options::USE_HEX_ALPHA_COLORS, OptionsOp::Any),
                }
            },
        );
    }
    Some(FunctionReplacement::Rgb { blue, green, red })
}

fn color_alpha(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Number(value) => Some(value),
        Token::Percentage(value) => Some(value),
        _ => None,
    }
}

fn color_component(value: &TokenOrValue<'_>) -> Option<(u8, Option<bool>, f32)> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    let (value, percentage, normalized) = match **token {
        Token::Number(value) if (0.0..=255.0).contains(&value) => {
            (value, (value != 0.0).then_some(false), value / 255.0)
        }
        Token::Percentage(value) if (0.0..=1.0).contains(&value) => {
            (value * 255.0, (value != 0.0).then_some(true), value)
        }
        _ => return None,
    };
    Some((value.round() as u8, percentage, normalized))
}

fn minify_transform_function(function: &mut Function<'_>) -> bool {
    if function.name.eq_ignore_ascii_case("rotatez") && function.arguments.len() == 1 {
        function.name = "rotate";
        return true;
    }
    if function.name.eq_ignore_ascii_case("matrix3d") {
        let values = &function.arguments;
        if values.len() == 31
            && number_at(values, 4) == Some(0.0)
            && number_at(values, 6) == Some(0.0)
            && number_at(values, 12) == Some(0.0)
            && number_at(values, 14) == Some(0.0)
            && number_at(values, 16) == Some(0.0)
            && number_at(values, 18) == Some(0.0)
            && number_at(values, 20) == Some(1.0)
            && number_at(values, 22) == Some(0.0)
            && number_at(values, 28) == Some(0.0)
            && number_at(values, 30) == Some(1.0)
        {
            function.name = "matrix";
            compact_arguments(
                &mut function.arguments,
                &[0, 1, 2, 3, 8, 9, 10, 11, 24, 25, 26],
            );
            return true;
        }
        return false;
    }
    if function.name.eq_ignore_ascii_case("rotate3d") && function.arguments.len() == 7 {
        function.name = match (
            number_at(&function.arguments, 0),
            number_at(&function.arguments, 2),
            number_at(&function.arguments, 4),
        ) {
            (Some(1.0), Some(0.0), Some(0.0)) => "rotateX",
            (Some(0.0), Some(1.0), Some(0.0)) => "rotateY",
            (Some(0.0), Some(0.0), Some(1.0)) => "rotate",
            _ => return false,
        };
        compact_arguments(&mut function.arguments, &[6]);
        return true;
    }
    if function.name.eq_ignore_ascii_case("scale") && function.arguments.len() == 3 {
        if function.arguments[0] == function.arguments[2]
            && !is_empty_variable_function(&function.arguments[0])
        {
            function.arguments.truncate(1);
            return true;
        }
        let first = number_at(&function.arguments, 0);
        let second = number_at(&function.arguments, 2);
        if first == second && first.is_some() {
            function.arguments.truncate(1);
            return true;
        }
        if second == Some(1.0) {
            function.name = "scaleX";
            function.arguments.truncate(1);
            return true;
        }
        if first == Some(1.0) {
            function.name = "scaleY";
            compact_arguments(&mut function.arguments, &[2]);
            return true;
        }
        return false;
    }
    if function.name.eq_ignore_ascii_case("scale3d") && function.arguments.len() == 5 {
        let values = [
            number_at(&function.arguments, 0),
            number_at(&function.arguments, 2),
            number_at(&function.arguments, 4),
        ];
        let (name, index) = if values[1] == Some(1.0) && values[2] == Some(1.0) {
            ("scaleX", 0)
        } else if values[0] == Some(1.0) && values[2] == Some(1.0) {
            ("scaleY", 2)
        } else if values[0] == Some(1.0) && values[1] == Some(1.0) {
            ("scaleZ", 4)
        } else {
            return false;
        };
        function.name = name;
        compact_arguments(&mut function.arguments, &[index]);
        return true;
    }
    if function.name.eq_ignore_ascii_case("translate") && function.arguments.len() == 3 {
        if number_at(&function.arguments, 2) == Some(0.0) {
            function.arguments.truncate(1);
            return true;
        }
        if number_at(&function.arguments, 0) == Some(0.0) {
            function.name = "translateY";
            compact_arguments(&mut function.arguments, &[2]);
            return true;
        }
        return false;
    }
    if function.name.eq_ignore_ascii_case("translate3d")
        && function.arguments.len() == 5
        && number_at(&function.arguments, 0) == Some(0.0)
        && number_at(&function.arguments, 2) == Some(0.0)
    {
        function.name = "translateZ";
        compact_arguments(&mut function.arguments, &[4]);
        return true;
    }
    false
}

fn is_empty_variable_function(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Function(function)
        if function.arguments.is_empty()
            && ["var", "env", "constant"]
                .iter()
                .any(|name| function.name.eq_ignore_ascii_case(name)))
}

fn simple_calc_value(values: &[TokenOrValue<'_>]) -> Option<FunctionReplacement> {
    let mut values = values.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
    });
    let left = calc_value(values.next()?)?;
    let Some(operator) = values.next() else {
        return Some(unitless_calc_zero(left));
    };
    let right = calc_value(values.next()?)?;
    if values.next().is_some() {
        return None;
    }
    let TokenOrValue::Token(operator) = operator else {
        return None;
    };
    let Token::Delim(operator) = &**operator else {
        return None;
    };
    calculate_values(left, operator, right).map(unitless_calc_zero)
}

const MAX_CALC_TERMS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CalcTermKind {
    Number,
    Percentage,
    Dimension(Unit),
}

#[derive(Clone, Copy, Debug)]
struct CalcTerm {
    explicit_zero: bool,
    kind: CalcTermKind,
    value: f32,
}

impl CalcTerm {
    const EMPTY: Self = Self {
        explicit_zero: false,
        kind: CalcTermKind::Number,
        value: 0.0,
    };
}

#[derive(Clone, Copy, Debug)]
struct CalcLinear {
    terms: [CalcTerm; MAX_CALC_TERMS],
    len: usize,
}

impl CalcLinear {
    const fn empty() -> Self {
        Self {
            terms: [CalcTerm::EMPTY; MAX_CALC_TERMS],
            len: 0,
        }
    }

    fn from_value(value: FunctionReplacement) -> Option<Self> {
        let (kind, value) = match value {
            FunctionReplacement::Number(value) => (CalcTermKind::Number, value),
            FunctionReplacement::Percentage(value) => (CalcTermKind::Percentage, value),
            FunctionReplacement::Dimension { unit, value } => {
                (CalcTermKind::Dimension(unit), value)
            }
            _ => return None,
        };
        let mut result = Self::empty();
        result.terms[0] = CalcTerm {
            explicit_zero: value == 0.0,
            kind,
            value,
        };
        result.len = 1;
        Some(result)
    }

    fn add(mut self, right: Self, sign: f32) -> Option<Self> {
        for right in right.terms[..right.len].iter().copied() {
            if let Some(left) = self.terms[..self.len]
                .iter_mut()
                .find(|left| left.kind == right.kind)
            {
                left.value += right.value * sign;
                left.explicit_zero &= right.explicit_zero;
                continue;
            }
            if self.len == MAX_CALC_TERMS {
                return None;
            }
            self.terms[self.len] = CalcTerm {
                explicit_zero: right.explicit_zero,
                kind: right.kind,
                value: right.value * sign,
            };
            self.len += 1;
        }
        Some(self)
    }

    fn scale(mut self, factor: f32) -> Self {
        for term in &mut self.terms[..self.len] {
            term.value *= factor;
        }
        self
    }

    fn round(mut self, precision: Option<u8>) -> Self {
        let Some(precision) = precision else {
            return self;
        };
        let factor = 10_f64.powi(i32::from(precision));
        for term in &mut self.terms[..self.len] {
            term.value = ((f64::from(term.value) * factor).round() / factor) as f32;
        }
        self
    }

    fn scalar(self) -> Option<f32> {
        (self.len == 1 && self.terms[0].kind == CalcTermKind::Number).then_some(self.terms[0].value)
    }

    fn compact_cancelled_terms(&mut self) {
        let mut target = 0;
        for source in 0..self.len {
            let term = self.terms[source];
            if term.value == 0.0 && !term.explicit_zero {
                continue;
            }
            self.terms[target] = term;
            target += 1;
        }
        self.len = target;
    }

    fn replacement(self) -> Option<FunctionReplacement> {
        if self.len == 0 {
            return Some(FunctionReplacement::Number(0.0));
        }
        if self.len != 1 {
            return None;
        }
        Some(match self.terms[0] {
            CalcTerm {
                kind: CalcTermKind::Number,
                value,
                ..
            } => FunctionReplacement::Number(value),
            CalcTerm {
                kind: CalcTermKind::Percentage,
                value,
                ..
            } => FunctionReplacement::Percentage(value),
            CalcTerm {
                kind: CalcTermKind::Dimension(unit),
                value,
                ..
            } => FunctionReplacement::Dimension { unit, value },
        })
    }

    fn write_to(mut self, function: &mut Function<'_>) -> bool {
        self.compact_cancelled_terms();
        if let Some(replacement) = self.replacement() {
            function.replacement = Some(unitless_calc_zero(replacement));
            function.arguments.clear();
            return true;
        }

        let required = 1 + (self.len - 1) * 4;
        if function
            .arguments
            .iter()
            .filter(|value| matches!(value, TokenOrValue::Token(_)))
            .count()
            < required
        {
            return false;
        }
        for target in 0..required {
            if matches!(function.arguments[target], TokenOrValue::Token(_)) {
                continue;
            }
            let Some(source) = function.arguments[target + 1..]
                .iter()
                .position(|value| matches!(value, TokenOrValue::Token(_)))
                .map(|source| target + 1 + source)
            else {
                return false;
            };
            function.arguments.swap(target, source);
        }

        let mut output = 0;
        for (index, term) in self.terms[..self.len].iter().copied().enumerate() {
            if index != 0 {
                set_calc_token(&mut function.arguments[output], Token::WhiteSpace(" "));
                output += 1;
                set_calc_token(
                    &mut function.arguments[output],
                    Token::Delim(if term.value < 0.0 { "-" } else { "+" }),
                );
                output += 1;
                set_calc_token(&mut function.arguments[output], Token::WhiteSpace(" "));
                output += 1;
            }
            let value = if index == 0 {
                term.value
            } else {
                term.value.abs()
            };
            let token = match term.kind {
                CalcTermKind::Number => Token::Number(value),
                CalcTermKind::Percentage => Token::Percentage(value),
                CalcTermKind::Dimension(unit) => Token::Dimension { unit, value },
            };
            set_calc_token(&mut function.arguments[output], token);
            output += 1;
        }
        function.arguments.truncate(required);
        true
    }
}

fn set_calc_token<'a>(value: &mut TokenOrValue<'a>, token_value: Token<'a>) {
    let TokenOrValue::Token(token) = value else {
        unreachable!("calc output slots were normalized to tokens")
    };
    **token = token_value;
}

fn calc_linear_expression(values: &[TokenOrValue<'_>]) -> Option<CalcLinear> {
    let mut parser = CalcLinearParser { index: 0, values };
    let mut result = parser.expression(false)?;
    parser.skip_whitespace();
    if parser.index != values.len() {
        return None;
    }
    result.compact_cancelled_terms();
    Some(result)
}

struct CalcLinearParser<'values, 'arena> {
    index: usize,
    values: &'values [TokenOrValue<'arena>],
}

impl CalcLinearParser<'_, '_> {
    fn expression(&mut self, nested: bool) -> Option<CalcLinear> {
        let mut result = self.term()?;
        loop {
            self.skip_whitespace();
            if nested && self.is_close_parenthesis() {
                break;
            }
            let Some(operator) = self.operator(&["+", "-"]) else {
                break;
            };
            let right = self.term()?;
            result = result.add(right, if operator == "+" { 1.0 } else { -1.0 })?;
        }
        Some(result)
    }

    fn term(&mut self) -> Option<CalcLinear> {
        let mut result = self.factor()?;
        loop {
            self.skip_whitespace();
            let Some(operator) = self.operator(&["*", "/"]) else {
                break;
            };
            let right = self.factor()?;
            result = match operator {
                "*" => {
                    if let Some(scalar) = result.scalar() {
                        right.scale(scalar)
                    } else {
                        result.scale(right.scalar()?)
                    }
                }
                "/" => {
                    let divisor = right.scalar()?;
                    if divisor == 0.0 {
                        return None;
                    }
                    result.scale(1.0 / divisor)
                }
                _ => unreachable!(),
            };
        }
        Some(result)
    }

    fn factor(&mut self) -> Option<CalcLinear> {
        self.skip_whitespace();
        let mut sign = 1.0;
        while let Some(operator) = self.operator(&["+", "-"]) {
            if operator == "-" {
                sign = -sign;
            }
            self.skip_whitespace();
        }
        let value = self.values.get(self.index)?;
        let mut result = match value {
            TokenOrValue::Token(token) if matches!(**token, Token::ParenthesisBlock) => {
                self.index += 1;
                let result = self.expression(true)?;
                self.skip_whitespace();
                if !self.is_close_parenthesis() {
                    return None;
                }
                self.index += 1;
                result
            }
            TokenOrValue::Function(function) if function.name.eq_ignore_ascii_case("calc") => {
                self.index += 1;
                if let Some(replacement) = function.replacement {
                    CalcLinear::from_value(replacement)?
                } else {
                    calc_linear_expression(&function.arguments)?
                }
            }
            value => {
                self.index += 1;
                CalcLinear::from_value(calc_value(value)?)?
            }
        };
        if sign < 0.0 {
            result = result.scale(-1.0);
        }
        Some(result)
    }

    fn operator<'operator>(
        &mut self,
        allowed: &'operator [&'operator str],
    ) -> Option<&'operator str> {
        let TokenOrValue::Token(token) = self.values.get(self.index)? else {
            return None;
        };
        let Token::Delim(operator) = &**token else {
            return None;
        };
        let operator = allowed
            .iter()
            .copied()
            .find(|allowed| operator == allowed)?;
        self.index += 1;
        Some(operator)
    }

    fn is_close_parenthesis(&self) -> bool {
        matches!(self.values.get(self.index), Some(TokenOrValue::Token(token)) if matches!(**token, Token::CloseParenthesis))
    }

    fn skip_whitespace(&mut self) {
        while self.values.get(self.index).is_some_and(is_calc_whitespace) {
            self.index += 1;
        }
    }
}

fn unitless_calc_zero(value: FunctionReplacement) -> FunctionReplacement {
    match value {
        FunctionReplacement::Dimension { value: 0.0, .. }
        | FunctionReplacement::Percentage(0.0) => FunctionReplacement::Number(0.0),
        value => value,
    }
}

fn minify_flat_calc_operations(values: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let mut changed = false;
    loop {
        let mut reduced = false;
        for operator_index in 0..values.len() {
            let TokenOrValue::Token(operator) = &values[operator_index] else {
                continue;
            };
            let Token::Delim(operator) = &**operator else {
                continue;
            };
            if !matches!(*operator, "*" | "/") {
                continue;
            }
            let Some(left_index) = values[..operator_index]
                .iter()
                .rposition(|value| !is_calc_whitespace(value))
            else {
                continue;
            };
            let Some(right_index) = values[operator_index + 1..]
                .iter()
                .position(|value| !is_calc_whitespace(value))
                .map(|index| operator_index + 1 + index)
            else {
                continue;
            };
            let Some(result) = calc_value(&values[left_index])
                .zip(calc_value(&values[right_index]))
                .and_then(|(left, right)| calculate_values(left, operator, right))
            else {
                continue;
            };
            if !set_calc_value(&mut values[left_index], result) {
                continue;
            }
            values.drain(left_index + 1..=right_index);
            reduced = true;
            changed = true;
            break;
        }
        if !reduced {
            break;
        }
    }

    loop {
        let mut reduced = false;
        for operator_index in 0..values.len() {
            let TokenOrValue::Token(operator) = &values[operator_index] else {
                continue;
            };
            let Token::Delim(operator) = &**operator else {
                continue;
            };
            if !matches!(*operator, "+" | "-") {
                continue;
            }
            let Some(left_index) = values[..operator_index]
                .iter()
                .rposition(|value| !is_calc_whitespace(value))
            else {
                continue;
            };
            let Some(right_index) = values[operator_index + 1..]
                .iter()
                .position(|value| !is_calc_whitespace(value))
                .map(|index| operator_index + 1 + index)
            else {
                continue;
            };
            if !calc_value(&values[right_index]).is_some_and(calc_value_is_zero) {
                continue;
            }
            values.drain(left_index + 1..=right_index);
            reduced = true;
            changed = true;
            break;
        }
        if !reduced {
            return changed;
        }
    }
}

fn calc_value_is_zero(value: FunctionReplacement) -> bool {
    matches!(
        value,
        FunctionReplacement::Number(0.0)
            | FunctionReplacement::Dimension { value: 0.0, .. }
            | FunctionReplacement::Percentage(0.0)
    )
}

fn is_calc_whitespace(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_) | Token::Comment(_)))
}

fn set_calc_value(value: &mut TokenOrValue<'_>, result: FunctionReplacement) -> bool {
    match value {
        TokenOrValue::Token(token) => {
            **token = match result {
                FunctionReplacement::Number(value) => Token::Number(value),
                FunctionReplacement::Dimension { unit, value } => Token::Dimension { unit, value },
                FunctionReplacement::Percentage(value) => Token::Percentage(value),
                _ => return false,
            };
            true
        }
        TokenOrValue::Function(function) => {
            function.arguments.clear();
            function.replacement = Some(result);
            true
        }
        TokenOrValue::Length(value) => match result {
            FunctionReplacement::Dimension {
                unit: Unit::Length(unit),
                value: result,
            } if unit == value.unit => {
                value.value = result;
                true
            }
            FunctionReplacement::Number(0.0) => {
                value.value = 0.0;
                true
            }
            _ => false,
        },
        _ => false,
    }
}

fn remove_redundant_calc_parentheses(values: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let Some(open) = values.iter().position(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::ParenthesisBlock))
    }) else {
        return false;
    };
    let Some(close) = values[open + 1..]
        .iter()
        .position(|value| {
            matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::CloseParenthesis))
        })
        .map(|index| open + 1 + index)
    else {
        return false;
    };
    let previous = values[..open]
        .iter()
        .rev()
        .find(|value| !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_))));
    let preceded_by_addition = previous.is_none_or(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::Delim("+") | Token::Delim("-")))
    });
    let contains_addition = values[open + 1..close].iter().any(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::Delim("+") | Token::Delim("-")))
    });
    if !preceded_by_addition || contains_addition {
        return false;
    }
    let _ = values.remove(close);
    let _ = values.remove(open);
    true
}

fn calc_value(value: &TokenOrValue<'_>) -> Option<FunctionReplacement> {
    match value {
        TokenOrValue::Token(token) => match **token {
            Token::Number(value) => Some(FunctionReplacement::Number(value)),
            Token::Dimension { unit, value } => {
                Some(FunctionReplacement::Dimension { unit, value })
            }
            Token::Percentage(value) => Some(FunctionReplacement::Percentage(value)),
            _ => None,
        },
        TokenOrValue::Length(value) => Some(FunctionReplacement::Dimension {
            unit: Unit::Length(value.unit),
            value: value.value,
        }),
        TokenOrValue::Function(function) => function.replacement,
        _ => None,
    }
}

fn calculate_values(
    left: FunctionReplacement,
    operator: &str,
    right: FunctionReplacement,
) -> Option<FunctionReplacement> {
    use FunctionReplacement::{Dimension, Number, Percentage};
    match (left, operator, right) {
        (Number(left), "+", Number(right)) => Some(Number(left + right)),
        (Number(left), "-", Number(right)) => Some(Number(left - right)),
        (Number(left), "*", Number(right)) => Some(Number(left * right)),
        (Number(left), "/", Number(right)) if right != 0.0 => Some(Number(left / right)),
        (
            Dimension {
                unit: left_unit,
                value: left,
            },
            "+",
            Dimension {
                unit: right_unit,
                value: right,
            },
        ) if left_unit == right_unit => Some(Dimension {
            unit: left_unit,
            value: left + right,
        }),
        (
            Dimension {
                unit: left_unit,
                value: left,
            },
            "-",
            Dimension {
                unit: right_unit,
                value: right,
            },
        ) if left_unit == right_unit => Some(Dimension {
            unit: left_unit,
            value: left - right,
        }),
        (Dimension { unit, value }, "*", Number(number))
        | (Number(number), "*", Dimension { unit, value }) => Some(Dimension {
            unit,
            value: value * number,
        }),
        (Dimension { unit, value }, "/", Number(number)) if number != 0.0 => Some(Dimension {
            unit,
            value: value / number,
        }),
        (Percentage(left), "+", Percentage(right)) => Some(Percentage(left + right)),
        (Percentage(left), "-", Percentage(right)) => Some(Percentage(left - right)),
        (Percentage(value), "*", Number(number)) | (Number(number), "*", Percentage(value)) => {
            Some(Percentage(value * number))
        }
        (Percentage(value), "/", Number(number)) if number != 0.0 => {
            Some(Percentage(value / number))
        }
        _ => None,
    }
}

fn number_at(values: &[TokenOrValue<'_>], index: usize) -> Option<f32> {
    values.get(index).and_then(token_number)
}

fn compact_arguments(
    arguments: &mut rocketcss_allocator::vec::Vec<'_, TokenOrValue<'_>>,
    indices: &[usize],
) {
    for (target, &source) in indices.iter().enumerate() {
        if target != source {
            arguments.swap(target, source);
        }
    }
    arguments.truncate(indices.len());
}

fn can_unquote_url(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(|character| {
            character.is_whitespace()
                || character.is_control()
                || matches!(character, '(' | ')' | '\\')
        })
}

pub(crate) fn normalize_url_text(value: &str) -> Option<std::string::String> {
    let trimmed = value.trim();
    if trimmed
        .get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("data:"))
    {
        return (trimmed != value).then(|| trimmed.to_owned());
    }

    let suffix_start = trimmed.find(['?', '#']).unwrap_or(trimmed.len());
    let (base, suffix) = trimmed.split_at(suffix_start);
    let (authority, path) = split_url_authority(base);
    let authority = normalize_url_authority(authority);
    let path = normalize_url_path(path);
    let mut normalized = std::string::String::with_capacity(trimmed.len());
    normalized.push_str(&authority);
    normalized.push_str(&path);
    normalized.push_str(suffix);
    (normalized != value).then_some(normalized)
}

fn split_url_authority(value: &str) -> (&str, &str) {
    let authority_start = if let Some(scheme) = value.find("://") {
        scheme + 3
    } else if value.starts_with("//") {
        2
    } else {
        return ("", value);
    };
    let Some(path_start) = value[authority_start..].find('/') else {
        return (value, "");
    };
    let path_start = authority_start + path_start + 1;
    (&value[..path_start], &value[path_start..])
}

fn normalize_url_authority(value: &str) -> std::string::String {
    if value.is_empty() {
        return std::string::String::new();
    }
    let without_slash = value.strip_suffix('/').unwrap_or(value);
    let default_port = if without_slash
        .get(..7)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("http://"))
        || without_slash.starts_with("//")
    {
        Some(":80")
    } else if without_slash
        .get(..8)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("https://"))
    {
        Some(":443")
    } else {
        None
    };
    let Some(port) = default_port.filter(|port| without_slash.ends_with(port)) else {
        return value.to_owned();
    };
    let mut normalized = std::string::String::with_capacity(value.len() - port.len());
    normalized.push_str(&without_slash[..without_slash.len() - port.len()]);
    if value.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

fn normalize_url_path(value: &str) -> std::string::String {
    if value.is_empty() {
        return std::string::String::new();
    }
    let leading_slash = value.starts_with('/');
    let trailing_slash = value.ends_with('/');
    let mut segments = std::vec::Vec::new();
    for segment in value.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if segments.last().is_some_and(|segment| *segment != "..") {
                    segments.pop();
                } else if !leading_slash {
                    segments.push(segment);
                }
            }
            _ => segments.push(segment),
        }
    }
    let mut normalized = std::string::String::with_capacity(value.len());
    if leading_slash {
        normalized.push('/');
    }
    for (index, segment) in segments.iter().enumerate() {
        if index != 0 {
            normalized.push('/');
        }
        normalized.push_str(segment);
    }
    if trailing_slash && !normalized.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

fn minify_cubic_bezier(arguments: &[TokenOrValue<'_>]) -> Option<&'static str> {
    let [a, comma_1, b, comma_2, c, comma_3, d] = arguments else {
        return None;
    };
    if !is_comma(comma_1) || !is_comma(comma_2) || !is_comma(comma_3) {
        return None;
    }
    match (
        token_number(a)?,
        token_number(b)?,
        token_number(c)?,
        token_number(d)?,
    ) {
        (0.25, 0.1, 0.25, 1.0) => Some("ease"),
        (0.0, 0.0, 1.0, 1.0) => Some("linear"),
        (0.42, 0.0, 1.0, 1.0) => Some("ease-in"),
        (0.0, 0.0, 0.58, 1.0) => Some("ease-out"),
        (0.42, 0.0, 0.58, 1.0) => Some("ease-in-out"),
        _ => None,
    }
}

fn minify_steps(
    arguments: &mut rocketcss_allocator::vec::Vec<'_, TokenOrValue<'_>>,
) -> Option<&'static str> {
    let [count, comma, position] = arguments.as_slice() else {
        return None;
    };
    if !is_comma(comma) {
        return None;
    }
    let position = token_ident(position)?;
    let is_start =
        position.eq_ignore_ascii_case("start") || position.eq_ignore_ascii_case("jump-start");
    let is_end = position.eq_ignore_ascii_case("end") || position.eq_ignore_ascii_case("jump-end");
    if token_number(count) == Some(1.0) {
        if is_start {
            return Some("step-start");
        }
        if is_end {
            return Some("step-end");
        }
    }
    if is_end {
        arguments.truncate(1);
    }
    None
}

fn token_number(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Number(value) => Some(value),
        Token::Dimension { value, .. } | Token::UnknownDimension { value, .. } => Some(value),
        _ => None,
    }
}

fn token_ident<'a>(value: &'a TokenOrValue<'a>) -> Option<&'a str> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Ident(value) => Some(value),
        _ => None,
    }
}

fn is_comma(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma))
}

impl Minify for Variable<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(cx);
        }
    }
}

impl Minify for EnvironmentVariable<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(cx);
        }
    }
}

impl Minify for UnknownAtRule<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.prelude.minify(cx);
        if let Some(block) = &mut self.block {
            block.minify(cx);
        }
    }
}

/// Coalesces adjacent equivalent conditional rules before their contents are
/// minified. This is the fast path used by the local Lightning CSS branch: a
/// run of conditional blocks is appended once and recursively processed once,
/// rather than repeatedly minifying a growing accumulated rule list.
pub(crate) fn coalesce_conditional_rules<'a>(
    rules: &mut Vec<'a, CssRule<'a>>,
    cx: &mut MinifyContext,
) {
    let mut previous_live = None;
    for index in 0..rules.len() {
        if matches!(rules[index], CssRule::Ignored) {
            continue;
        }

        let merge = previous_live.is_some_and(|previous_index| {
            match (&rules[previous_index], &rules[index]) {
                (CssRule::Media(left), CssRule::Media(right)) => left.query == right.query,
                (CssRule::Supports(left), CssRule::Supports(right)) => {
                    left.condition == right.condition
                }
                (CssRule::Container(left), CssRule::Container(right)) => {
                    left.name == right.name && left.condition == right.condition
                }
                _ => false,
            }
        });

        if !merge {
            previous_live = Some(index);
            continue;
        }

        let previous_index = previous_live.expect("checked above");
        let (before, current_and_after) = rules.split_at_mut(index);
        let current = std::mem::replace(&mut current_and_after[0], CssRule::Ignored);
        match (&mut before[previous_index], current) {
            (CssRule::Media(left), CssRule::Media(mut right)) => {
                left.rules.append(&mut right.rules);
            }
            (CssRule::Supports(left), CssRule::Supports(mut right)) => {
                left.rules.append(&mut right.rules);
            }
            (CssRule::Container(left), CssRule::Container(mut right)) => {
                left.rules.append(&mut right.rules);
            }
            _ => unreachable!("conditional rule kind changed after comparison"),
        }
        cx.record_conditional_rule_merged();
    }

    for rule in rules.iter_mut() {
        match rule {
            CssRule::Style(rule) => coalesce_conditional_rules(&mut rule.rules, cx),
            CssRule::Media(rule) => coalesce_conditional_rules(&mut rule.rules, cx),
            CssRule::Supports(rule) => coalesce_conditional_rules(&mut rule.rules, cx),
            CssRule::MozDocument(rule) => coalesce_conditional_rules(&mut rule.rules, cx),
            CssRule::Nesting(rule) => {
                coalesce_conditional_rules(&mut rule.style.rules, cx);
            }
            CssRule::LayerBlock(rule) => coalesce_conditional_rules(&mut rule.rules, cx),
            CssRule::Container(rule) => coalesce_conditional_rules(&mut rule.rules, cx),
            CssRule::Scope(rule) => coalesce_conditional_rules(&mut rule.rules, cx),
            CssRule::StartingStyle(rule) => coalesce_conditional_rules(&mut rule.rules, cx),
            _ => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DeclarationKey<'a> {
    name: &'a str,
    vendor_prefix: VendorPrefix,
    important: bool,
}

#[derive(Clone, Copy)]
struct DeclarationLocation<'a> {
    block: NonNull<DeclarationBlock<'a>>,
    index: usize,
}

/// Merges adjacent compatible style rules within one rule list. Every at-rule
/// ends the current IR segment, so block links never cross a conditional or
/// other at-rule boundary. Nested rule lists are minified in their own scope.
pub(crate) fn minify_rule_list<'a>(rules: &mut Vec<'a, CssRule<'a>>, cx: &mut MinifyContext) {
    for rule in rules.iter_mut() {
        match rule {
            CssRule::Style(rule) => minify_rule_list(&mut rule.rules, cx),
            CssRule::Media(rule) => minify_rule_list(&mut rule.rules, cx),
            CssRule::Supports(rule) => minify_rule_list(&mut rule.rules, cx),
            CssRule::MozDocument(rule) => minify_rule_list(&mut rule.rules, cx),
            CssRule::Nesting(rule) => minify_rule_list(&mut rule.style.rules, cx),
            CssRule::LayerBlock(rule) => minify_rule_list(&mut rule.rules, cx),
            CssRule::Container(rule) => minify_rule_list(&mut rule.rules, cx),
            CssRule::Scope(rule) => minify_rule_list(&mut rule.rules, cx),
            CssRule::StartingStyle(rule) => minify_rule_list(&mut rule.rules, cx),
            _ => {}
        }
    }

    discard_overridden_keyframes(rules, cx);

    for rule in rules.iter_mut() {
        if let CssRule::Style(rule) = rule {
            deduplicate_declarations(&mut rule.declarations, cx);
        }
    }

    factor_style_output_runs(rules, cx);

    if !has_mergeable_style_run(rules, cx) {
        discard_empty_rules(rules, cx);
        deduplicate_style_rules(rules, cx);
        return;
    }

    let allocator = rules.bump();
    let mut declarations = HashMap::new_in(allocator);
    let mut previous_style = None;

    for index in 0..rules.len() {
        if matches!(rules[index], CssRule::Ignored) {
            continue;
        }

        if !matches!(rules[index], CssRule::Style(_)) {
            // All at-rules are barriers in the first IR design. Dedicated
            // optimizations such as the conditional fast path run separately.
            previous_style = None;
            declarations.clear();
            continue;
        }

        if matches!(&rules[index], CssRule::Style(rule) if rule.output_ir().is_some()) {
            previous_style = None;
            declarations.clear();
            continue;
        }

        let merge_kind = previous_style.and_then(|previous_index| {
            let CssRule::Style(previous) = &rules[previous_index] else {
                return None;
            };
            let CssRule::Style(current) = &rules[index] else {
                return None;
            };
            style_rule_merge_kind(previous, current, cx)
        });

        if let Some(merge_kind) = merge_kind {
            let previous_index = previous_style.expect("checked above");
            let (before, current_and_after) = rules.split_at_mut(index);
            if matches!(
                merge_kind,
                StyleRuleMergeKind::PreviousDeclarationsAreSubset
                    | StyleRuleMergeKind::CurrentDeclarationsAreSubset
            ) {
                let CssRule::Style(previous) = &mut before[previous_index] else {
                    unreachable!("style merge candidate changed kind")
                };
                let CssRule::Style(current) = &mut current_and_after[0] else {
                    unreachable!("current style rule changed kind")
                };
                match merge_kind {
                    StyleRuleMergeKind::PreviousDeclarationsAreSubset => {
                        let selector_ir = merged_selector_ir(previous, current, allocator);
                        // SAFETY: source selector vectors are stable and no
                        // longer mutated after the rule-list pass begins.
                        unsafe { previous.set_selector_ir(selector_ir) };
                        tombstone_matching_declarations(
                            &mut current.declarations,
                            &previous.declarations,
                            cx,
                        );
                    }
                    StyleRuleMergeKind::CurrentDeclarationsAreSubset => {
                        let selector_ir = merged_selector_ir(current, previous, allocator);
                        // SAFETY: source selector vectors are stable and no
                        // longer mutated after the rule-list pass begins.
                        unsafe { current.set_selector_ir(selector_ir) };
                        tombstone_matching_declarations(
                            &mut previous.declarations,
                            &current.declarations,
                            cx,
                        );
                    }
                    _ => unreachable!(),
                }
                declarations.clear();
                process_declarations(&mut current.declarations, &mut declarations, cx);
                cx.record_style_rule_merged();
                previous_style = Some(index);
                continue;
            }
            let previous_rule = std::mem::replace(&mut before[previous_index], CssRule::Ignored);
            let CssRule::Style(mut previous) = previous_rule else {
                unreachable!("style merge candidate changed kind")
            };
            let CssRule::Style(current) = &mut current_and_after[0] else {
                unreachable!("current style rule changed kind")
            };

            match merge_kind {
                StyleRuleMergeKind::SameSelectors => {
                    let previous_block = NonNull::from(previous.declarations.as_mut());
                    // SAFETY: declaration blocks are arena boxed and never move. The
                    // previous rule is the live tail of this adjacent merge chain.
                    unsafe { current.declarations.link_previous(previous_block) };
                    process_declarations(&mut current.declarations, &mut declarations, cx);
                }
                StyleRuleMergeKind::SameDeclarations => {
                    std::mem::swap(&mut previous.declarations, &mut current.declarations);
                    let selector_ir = merged_selector_ir(current, &previous, allocator);
                    // SAFETY: all pointers refer to selector nodes in arena
                    // vectors that are no longer mutated after this rule pass.
                    unsafe { current.set_selector_ir(selector_ir) };
                    declarations.clear();
                    process_declarations(&mut current.declarations, &mut declarations, cx);
                }
                StyleRuleMergeKind::PreviousDeclarationsAreSubset
                | StyleRuleMergeKind::CurrentDeclarationsAreSubset => unreachable!(),
            }
            cx.record_style_rule_merged();
            previous_style = Some(index);
            continue;
        }

        declarations.clear();
        let CssRule::Style(current) = &mut rules[index] else {
            unreachable!()
        };
        process_declarations(&mut current.declarations, &mut declarations, cx);
        previous_style = current.rules.is_empty().then_some(index);
    }
    discard_empty_rules(rules, cx);
    deduplicate_style_rules(rules, cx);
}

fn factor_style_output_runs<'a>(rules: &mut Vec<'a, CssRule<'a>>, cx: &mut MinifyContext) {
    if cx.is_enabled(Options::MERGE_STYLE_RULES, OptionsOp::None) {
        return;
    }
    let allocator = rules.bump();
    let mut run = allocator.vec();
    let mut run_vendor = None;
    let mut run_selector_vendor = None;

    for index in 0..=rules.len() {
        let candidate = rules.get(index).and_then(|rule| {
            let CssRule::Style(rule) = rule else {
                return None;
            };
            if !rule.rules.is_empty()
                || rule.output_ir().is_some()
                || !style_rule_selectors_are_compatible(rule, rule, cx)
            {
                return None;
            }
            Some((rule.vendor_prefix, style_rule_selector_vendor(rule)?))
        });

        if let Some((vendor, selector_vendor)) = candidate
            && (run.is_empty()
                || run_vendor == Some(vendor) && run_selector_vendor == Some(selector_vendor))
        {
            run.push(index);
            run_vendor = Some(vendor);
            run_selector_vendor = Some(selector_vendor);
            continue;
        }

        factor_style_output_run(rules, &run, allocator, cx);
        run.clear();
        run_vendor = None;
        run_selector_vendor = None;
        if let Some((vendor, selector_vendor)) = candidate {
            run.push(index);
            run_vendor = Some(vendor);
            run_selector_vendor = Some(selector_vendor);
        }
    }
}

fn factor_style_output_run<'a>(
    rules: &mut [CssRule<'a>],
    indices: &[usize],
    allocator: &'a Allocator,
    cx: &mut MinifyContext,
) {
    if indices.len() < 2 {
        return;
    }
    if indices.windows(2).any(|indices| {
        let (CssRule::Style(left), CssRule::Style(right)) =
            (&rules[indices[0]], &rules[indices[1]])
        else {
            unreachable!()
        };
        left.selectors == right.selectors
    }) {
        return;
    }
    let mut groups = allocator.vec();
    for &index in indices {
        let CssRule::Style(rule) = &rules[index] else {
            unreachable!("style output run contains only style rules")
        };
        let mut selectors = allocator.vec();
        if let Some(selector_ir) = rule.selector_ir() {
            selectors.extend(selector_ir.iter().copied());
        } else {
            selectors.extend(rule.selectors.iter().map(NonNull::from));
        }
        let mut declarations = allocator.vec();
        declarations.extend(OutputDeclarations::new(&rule.declarations).map(
            |(declaration, important)| StyleRuleOutputDeclaration {
                declaration: NonNull::from(declaration),
                important,
            },
        ));
        if !declarations.is_empty() {
            groups.push(StyleRuleOutput {
                declarations,
                selectors,
            });
        }
    }

    let mut transformed = false;
    let mut index = 0;
    while index + 1 < groups.len() {
        let Some(replacement) = factor_output_pair(&groups[index], &groups[index + 1], allocator)
        else {
            index += 1;
            continue;
        };
        groups.remove(index + 1);
        groups.remove(index);
        let replacement_len = replacement.len();
        for (offset, group) in replacement.into_iter().enumerate() {
            groups.insert(index + offset, group);
        }
        transformed = true;
        if index > 0 {
            index -= 1;
        } else if replacement_len == 0 {
            break;
        }
    }

    if merge_nonadjacent_output_groups(&mut groups, allocator) {
        transformed = true;
    }
    if !transformed || groups.is_empty() {
        return;
    }

    let anchor = indices[0];
    let CssRule::Style(anchor_rule) = &mut rules[anchor] else {
        unreachable!()
    };
    // SAFETY: selectors and declarations are arena-backed and no longer
    // mutated after this output IR is installed.
    unsafe { anchor_rule.set_output_ir(groups) };
    for &index in &indices[1..] {
        if !matches!(rules[index], CssRule::Ignored) {
            rules[index] = CssRule::Ignored;
            cx.record_style_rule_merged();
        }
    }
}

fn factor_output_pair<'a>(
    left: &StyleRuleOutput<'a>,
    right: &StyleRuleOutput<'a>,
    allocator: &'a Allocator,
) -> Option<Vec<'a, StyleRuleOutput<'a>>> {
    let mut left_to_right = allocator.vec();
    left_to_right.resize(left.declarations.len(), usize::MAX);
    let mut right_to_left = allocator.vec();
    right_to_left.resize(right.declarations.len(), usize::MAX);
    let mut right_matched = BitVec::new(allocator);
    for _ in 0..right.declarations.len() {
        right_matched.push(false);
    }

    for (left_index, left_declaration) in left.declarations.iter().enumerate() {
        for (right_index, right_declaration) in right.declarations.iter().enumerate() {
            if right_matched.is_set(right_index)
                || !output_declarations_equal(*left_declaration, *right_declaration)
            {
                continue;
            }
            left_to_right[left_index] = right_index;
            right_to_left[right_index] = left_index;
            right_matched.set(right_index, true);
            break;
        }
    }

    let mut common = BitVec::new(allocator);
    for _ in 0..left.declarations.len() {
        common.push(false);
    }
    for (index, right_index) in left_to_right.iter().copied().enumerate() {
        if right_index != usize::MAX {
            common.set(index, true);
        }
    }
    if !common.iter().any(|value| value) {
        return None;
    }

    loop {
        let mut changed = false;
        'reversed: for first in 0..left.declarations.len() {
            if !common.is_set(first) {
                continue;
            }
            for second in first + 1..left.declarations.len() {
                if !common.is_set(second)
                    || left_to_right[first] < left_to_right[second]
                    || !output_declarations_conflict(
                        left.declarations[first],
                        left.declarations[second],
                    )
                {
                    continue;
                }
                let first_score = first + right.declarations.len() - 1 - left_to_right[first];
                let second_score = second + right.declarations.len() - 1 - left_to_right[second];
                common.set(
                    if first_score > second_score {
                        second
                    } else {
                        first
                    },
                    false,
                );
                changed = true;
                break 'reversed;
            }
        }
        if changed {
            continue;
        }

        'boundary: for left_index in 0..left.declarations.len() {
            if !common.is_set(left_index) {
                continue;
            }
            let right_index = left_to_right[left_index];
            let crosses_left_conflict = (left_index + 1..left.declarations.len()).any(|other| {
                !common.is_set(other)
                    && output_declarations_conflict(
                        left.declarations[left_index],
                        left.declarations[other],
                    )
            });
            let crosses_right_conflict = (0..right_index).any(|other| {
                let matching_left = right_to_left[other];
                (matching_left == usize::MAX || !common.is_set(matching_left))
                    && output_declarations_conflict(
                        left.declarations[left_index],
                        right.declarations[other],
                    )
            });
            if crosses_left_conflict || crosses_right_conflict {
                common.set(left_index, false);
                changed = true;
                break 'boundary;
            }
        }
        if !changed {
            break;
        }
    }

    let common_count = common.iter().filter(|value| *value).count();
    if common_count == 0 {
        return None;
    }
    let left_unique_count = left.declarations.len() - common_count;
    let right_unique_count = right.declarations.len() - common_count;
    let saved = left
        .declarations
        .iter()
        .enumerate()
        .filter(|(index, _)| common.is_set(*index))
        .map(|(_, declaration)| output_declaration_weight(*declaration))
        .sum::<usize>();
    let selector_cost = if left_unique_count == 0 {
        0
    } else {
        output_selector_weight(&left.selectors)
    } + if right_unique_count == 0 {
        0
    } else {
        output_selector_weight(&right.selectors)
    };
    let output_count =
        usize::from(left_unique_count != 0) + 1 + usize::from(right_unique_count != 0);
    let structural_cost = output_count.saturating_sub(2) * 2;
    let largest_duplicated_selector = [
        (left_unique_count != 0).then(|| output_selector_weight(&left.selectors)),
        (right_unique_count != 0).then(|| output_selector_weight(&right.selectors)),
    ]
    .into_iter()
    .flatten()
    .max()
    .unwrap_or(0);
    let partial_intersection = left_unique_count != 0 && right_unique_count != 0;
    if saved <= selector_cost + structural_cost
        || partial_intersection && largest_duplicated_selector * 2 > saved
        || partial_intersection
            && left
                .selectors
                .iter()
                .chain(right.selectors.iter())
                .any(|selector| {
                    // SAFETY: output selector pointers refer to stable arena storage.
                    unsafe { selector.as_ref() }
                        .iter()
                        .any(|component| matches!(component, SelectorComponent::PseudoElement(_)))
                })
    {
        return None;
    }

    let mut output = allocator.vec();
    if left_unique_count != 0 {
        output.push(StyleRuleOutput {
            declarations: copy_output_declarations(left, allocator, |index| !common.is_set(index)),
            selectors: copy_output_selectors(&left.selectors, allocator),
        });
    }
    output.push(StyleRuleOutput {
        declarations: copy_output_declarations(left, allocator, |index| common.is_set(index)),
        selectors: merge_output_selectors(&left.selectors, &right.selectors, allocator),
    });
    if right_unique_count != 0 {
        output.push(StyleRuleOutput {
            declarations: copy_output_declarations(right, allocator, |index| {
                let matching_left = right_to_left[index];
                matching_left == usize::MAX || !common.is_set(matching_left)
            }),
            selectors: copy_output_selectors(&right.selectors, allocator),
        });
    }
    Some(output)
}

fn copy_output_declarations<'a>(
    group: &StyleRuleOutput<'a>,
    allocator: &'a Allocator,
    mut include: impl FnMut(usize) -> bool,
) -> Vec<'a, StyleRuleOutputDeclaration<'a>> {
    let mut declarations = allocator.vec();
    declarations.extend(
        group
            .declarations
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(index, declaration)| include(index).then_some(declaration)),
    );
    declarations
}

fn copy_output_selectors<'a>(
    selectors: &[NonNull<rocketcss_ast::Selector<'a>>],
    allocator: &'a Allocator,
) -> Vec<'a, NonNull<rocketcss_ast::Selector<'a>>> {
    let mut output = allocator.vec();
    output.extend(selectors.iter().copied());
    output
}

fn merge_output_selectors<'a>(
    left: &[NonNull<rocketcss_ast::Selector<'a>>],
    right: &[NonNull<rocketcss_ast::Selector<'a>>],
    allocator: &'a Allocator,
) -> Vec<'a, NonNull<rocketcss_ast::Selector<'a>>> {
    let mut selectors = copy_output_selectors(left, allocator);
    selectors.extend(right.iter().copied());
    selectors.sort_unstable_by(|left, right| {
        // SAFETY: output IR points into stable arena selector vectors.
        crate::selector::compare_selectors(unsafe { left.as_ref() }, unsafe { right.as_ref() })
    });
    let mut write = 0;
    for read in 0..selectors.len() {
        let duplicate = write != 0 && {
            // SAFETY: output IR points into stable arena selector vectors.
            unsafe { selectors[write - 1].as_ref() == selectors[read].as_ref() }
        };
        if !duplicate {
            selectors.swap(write, read);
            write += 1;
        }
    }
    selectors.truncate(write);
    selectors
}

fn merge_nonadjacent_output_groups<'a>(
    groups: &mut Vec<'a, StyleRuleOutput<'a>>,
    allocator: &'a Allocator,
) -> bool {
    let mut changed = false;
    let mut left = 0;
    while left + 2 < groups.len() {
        let mut right = left + 2;
        while right < groups.len() {
            if !output_group_declarations_equal(&groups[left], &groups[right])
                || groups[left + 1..right].iter().any(|between| {
                    groups[left].declarations.iter().any(|left| {
                        between
                            .declarations
                            .iter()
                            .any(|right| output_declarations_conflict(*left, *right))
                    })
                })
            {
                right += 1;
                continue;
            }
            let selectors = merge_output_selectors(
                &groups[left].selectors,
                &groups[right].selectors,
                allocator,
            );
            groups[left].selectors = selectors;
            groups.remove(right);
            changed = true;
        }
        left += 1;
    }
    changed
}

fn output_group_declarations_equal(
    left: &StyleRuleOutput<'_>,
    right: &StyleRuleOutput<'_>,
) -> bool {
    left.declarations.len() == right.declarations.len()
        && left
            .declarations
            .iter()
            .zip(right.declarations.iter())
            .all(|(left, right)| output_declarations_equal(*left, *right))
}

fn output_declarations_equal(
    left: StyleRuleOutputDeclaration<'_>,
    right: StyleRuleOutputDeclaration<'_>,
) -> bool {
    left.important == right.important && {
        // SAFETY: output declaration pointers refer to stable arena storage.
        unsafe { left.declaration.as_ref() == right.declaration.as_ref() }
    }
}

fn output_declarations_conflict(
    left: StyleRuleOutputDeclaration<'_>,
    right: StyleRuleOutputDeclaration<'_>,
) -> bool {
    // SAFETY: output declaration pointers refer to stable arena storage.
    properties_conflict(
        unsafe { left.declaration.as_ref() }.name(),
        unsafe { right.declaration.as_ref() }.name(),
    )
}

fn output_declaration_weight(declaration: StyleRuleOutputDeclaration<'_>) -> usize {
    // SAFETY: output declaration pointers refer to stable arena storage.
    unsafe { declaration.declaration.as_ref() }.name().len()
        + 8
        + usize::from(declaration.important) * 11
}

fn output_selector_weight(selectors: &[NonNull<rocketcss_ast::Selector<'_>>]) -> usize {
    selectors.len().saturating_sub(1)
        + selectors
            .iter()
            .map(|selector| {
                // SAFETY: output selector pointers refer to stable arena storage.
                unsafe { selector.as_ref() }
                    .iter()
                    .map(|component| match component {
                        SelectorComponent::Class(name) | SelectorComponent::Id(name) => {
                            name.len() + 1
                        }
                        SelectorComponent::DefaultNamespace(name) => name.len(),
                        SelectorComponent::LocalName { name, .. } => name.len(),
                        SelectorComponent::Namespace { prefix, url } => {
                            prefix.len() + url.len() + 1
                        }
                        SelectorComponent::AttributeInNoNamespaceExists { local_name, .. } => {
                            local_name.len() + 2
                        }
                        SelectorComponent::AttributeInNoNamespace {
                            local_name, value, ..
                        } => local_name.len() + value.len() + 4,
                        SelectorComponent::AttributeOther(attribute) => {
                            let value = match &attribute.operation {
                                rocketcss_ast::AttrOperation::Exists => 0,
                                rocketcss_ast::AttrOperation::WithValue {
                                    expected_value, ..
                                } => expected_value.len() + 2,
                            };
                            attribute.local_name.len() + value + 2
                        }
                        SelectorComponent::PseudoClass(pseudo) => match &**pseudo {
                            PseudoClass::Custom { name } => name.len() + 1,
                            PseudoClass::CustomFunction { name, .. } => name.len() + 3,
                            _ => 8,
                        },
                        SelectorComponent::PseudoElement(pseudo) => match &**pseudo {
                            PseudoElement::After => 7,
                            PseudoElement::Before => 8,
                            PseudoElement::FirstLine => 12,
                            PseudoElement::FirstLetter => 14,
                            PseudoElement::Custom { name } => name.len() + 2,
                            PseudoElement::CustomFunction { name, .. } => name.len() + 4,
                            _ => 10,
                        },
                        SelectorComponent::Combinator(_) => 1,
                        _ => 3,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
}

pub(crate) fn discard_browser_hacks(rules: &mut Vec<'_, CssRule<'_>>, cx: &mut MinifyContext) {
    let Some(target) = cx.options().browser_hack_target else {
        return;
    };
    for rule in rules.iter_mut() {
        let required_target = match rule {
            CssRule::Style(rule) => {
                for index in 0..rule.declarations.len() {
                    if rule.declarations.is_invalid(index) {
                        continue;
                    }
                    if declaration_browser_hack_target(&rule.declarations.declarations[index])
                        .is_some_and(|required| required != target)
                    {
                        rule.declarations.mark_invalid(index);
                        cx.record_declaration_removed();
                    }
                }
                discard_browser_hacks(&mut rule.rules, cx);
                style_rule_browser_hack_target(rule)
            }
            CssRule::Media(rule) => {
                let required = media_browser_hack_target(&rule.query);
                discard_browser_hacks(&mut rule.rules, cx);
                required
            }
            CssRule::Supports(rule) => {
                discard_browser_hacks(&mut rule.rules, cx);
                None
            }
            CssRule::MozDocument(rule) => {
                discard_browser_hacks(&mut rule.rules, cx);
                None
            }
            CssRule::Nesting(rule) => {
                discard_browser_hacks(&mut rule.style.rules, cx);
                None
            }
            CssRule::LayerBlock(rule) => {
                discard_browser_hacks(&mut rule.rules, cx);
                None
            }
            CssRule::Container(rule) => {
                discard_browser_hacks(&mut rule.rules, cx);
                None
            }
            CssRule::Scope(rule) => {
                discard_browser_hacks(&mut rule.rules, cx);
                None
            }
            CssRule::StartingStyle(rule) => {
                discard_browser_hacks(&mut rule.rules, cx);
                None
            }
            _ => None,
        };
        if required_target.is_some_and(|required| required != target) {
            *rule = CssRule::Ignored;
            cx.record_style_rule_merged();
        }
    }
}

fn declaration_browser_hack_target(declaration: &Declaration<'_>) -> Option<BrowserHackTarget> {
    let name = declaration.name();
    if name.starts_with('_') || name.eq_ignore_ascii_case("-color") {
        return Some(BrowserHackTarget::Ie6);
    }
    let Declaration::Unparsed(value) = declaration else {
        return None;
    };
    if token_list_contains_bang_ie(&value.value) {
        return Some(BrowserHackTarget::Ie6);
    }
    value
        .value
        .iter()
        .any(token_or_value_contains_backslash_nine)
        .then_some(BrowserHackTarget::Ie8)
}

fn token_list_contains_bang_ie(values: &[TokenOrValue<'_>]) -> bool {
    let mut bang = false;
    for value in values {
        let TokenOrValue::Token(token) = value else {
            continue;
        };
        match &**token {
            Token::Delim(value) if *value == "!" => bang = true,
            Token::Ident(value) if bang && value.eq_ignore_ascii_case("ie") => return true,
            Token::WhiteSpace(_) | Token::Comment(_) => {}
            _ => bang = false,
        }
    }
    false
}

fn token_or_value_contains_backslash_nine(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Token(token) => match &**token {
            Token::Ident(value)
            | Token::AtKeyword(value)
            | Token::Delim(value)
            | Token::UnknownDimension { unit: value, .. } => {
                value.contains("\\9") || value.contains('\t')
            }
            Token::WhiteSpace(value) => value.contains('\t'),
            _ => false,
        },
        TokenOrValue::Function(function) => function
            .arguments
            .iter()
            .any(token_or_value_contains_backslash_nine),
        _ => false,
    }
}

fn style_rule_browser_hack_target(rule: &StyleRule<'_>) -> Option<BrowserHackTarget> {
    for selector in rule.selectors.iter() {
        if selector.iter().any(|component| {
            matches!(component, SelectorComponent::LocalName { name, .. }
                if name.ends_with('\\') || name.ends_with(char::is_whitespace))
        }) {
            return Some(BrowserHackTarget::Ie6);
        }
        for components in selector.windows(3) {
            if matches!(components[0], SelectorComponent::ExplicitUniversalType)
                && matches!(
                    components[1],
                    SelectorComponent::Combinator(rocketcss_ast::Combinator::Descendant)
                )
                && matches!(&components[2], SelectorComponent::LocalName { name, .. } if name.eq_ignore_ascii_case("html"))
            {
                return Some(BrowserHackTarget::Ie6);
            }
        }
    }
    None
}

fn media_browser_hack_target(media: &rocketcss_ast::MediaList<'_>) -> Option<BrowserHackTarget> {
    for query in &media.media_queries {
        let rocketcss_ast::MediaType::Custom(value) = &*query.media_type else {
            continue;
        };
        if value.contains('�') && value.contains(',') && value.contains('\t') {
            return Some(BrowserHackTarget::Ie6);
        }
        if contains_ignore_ascii_case(value, "\\0screen\\,screen\\9")
            || contains_ignore_ascii_case(value, "�screen\\,screen\\9")
        {
            return Some(BrowserHackTarget::Ie6);
        }
        if contains_ignore_ascii_case(value, "\\0screen")
            || contains_ignore_ascii_case(value, "�screen")
        {
            return Some(BrowserHackTarget::Ie8);
        }
        if contains_ignore_ascii_case(value, "screen\\9") || value.contains('\t') {
            return Some(BrowserHackTarget::Ie6);
        }
    }
    None
}

fn contains_ignore_ascii_case(value: &str, needle: &str) -> bool {
    value
        .as_bytes()
        .windows(needle.len())
        .any(|window| window.eq_ignore_ascii_case(needle.as_bytes()))
}

fn discard_overridden_keyframes<'a>(rules: &mut Vec<'a, CssRule<'a>>, cx: &mut MinifyContext) {
    if cx.is_enabled(Options::DISCARD_OVERRIDDEN_KEYFRAMES, OptionsOp::None) {
        return;
    }
    let allocator = rules.bump();
    let mut names: HashMap<'a, (&'a str, u8), ()> = HashMap::new_in(allocator);
    for index in (0..rules.len()).rev() {
        let CssRule::Keyframes(rule) = &rules[index] else {
            continue;
        };
        let name = match &*rule.name {
            KeyframesName::Ident(name) | KeyframesName::Custom(name) => *name,
        };
        let key = (name, rule.vendor_prefix.bits());
        if names.insert(key, ()).is_some() {
            rules[index] = CssRule::Ignored;
            cx.record_style_rule_merged();
        }
    }
}

fn merged_selector_ir<'a>(
    target: &mut StyleRule<'a>,
    other: &StyleRule<'a>,
    allocator: &'a rocketcss_allocator::Allocator,
) -> Vec<'a, NonNull<rocketcss_ast::Selector<'a>>> {
    let mut selectors = target.take_selector_ir().unwrap_or_else(|| {
        let mut selectors = allocator.vec();
        selectors.extend(target.selectors.iter().map(NonNull::from));
        selectors
    });
    if let Some(other_ir) = other.selector_ir() {
        selectors.extend(other_ir.iter().copied());
    } else {
        selectors.extend(other.selectors.iter().map(NonNull::from));
    }
    selectors.sort_unstable_by(|left, right| {
        // SAFETY: selector IR pointers refer to stable arena vector elements.
        crate::selector::compare_selectors(unsafe { left.as_ref() }, unsafe { right.as_ref() })
    });
    let mut write = 0;
    for read in 0..selectors.len() {
        let duplicate = write > 0 && {
            // SAFETY: selector IR pointers refer to stable arena vector elements.
            unsafe { selectors[write - 1].as_ref() == selectors[read].as_ref() }
        };
        if !duplicate {
            selectors.swap(write, read);
            write += 1;
        }
    }
    selectors.truncate(write);
    selectors
}

fn deduplicate_style_rules(rules: &mut Vec<'_, CssRule<'_>>, cx: &mut MinifyContext) {
    if cx.is_enabled(Options::DEDUPLICATE_RULES, OptionsOp::None) {
        return;
    }

    let allocator = rules.bump();
    let mut bucket_heads: HashMap<'_, u64, usize> = HashMap::new_in(allocator);
    let mut next_in_bucket = allocator.vec();
    next_in_bucket.resize(rules.len(), usize::MAX);

    for index in (0..rules.len()).rev() {
        let CssRule::Style(current) = &rules[index] else {
            if !matches!(rules[index], CssRule::Ignored) {
                bucket_heads.clear();
            }
            continue;
        };
        if !current.rules.is_empty() {
            bucket_heads.clear();
            continue;
        }

        let bucket = style_rule_bucket(current);
        let mut candidate = bucket_heads.get(&bucket).copied();
        let mut duplicate = false;
        while let Some(candidate_index) = candidate {
            let CssRule::Style(candidate_rule) = &rules[candidate_index] else {
                unreachable!("style-rule IR bucket must point at a live style rule")
            };
            if style_rules_are_identical(current, candidate_rule) {
                duplicate = true;
                break;
            }
            let next = next_in_bucket[candidate_index];
            candidate = (next != usize::MAX).then_some(next);
        }

        if duplicate {
            rules[index] = CssRule::Ignored;
            cx.record_style_rule_merged();
            continue;
        }
        if let Some(head) = bucket_heads.insert(bucket, index) {
            next_in_bucket[index] = head;
        }
    }
}

fn style_rule_bucket(rule: &StyleRule<'_>) -> u64 {
    let mut hasher = FxHasher::default();
    rule.vendor_prefix.bits().hash(&mut hasher);
    rule.selectors.len().hash(&mut hasher);
    for selector in rule.selectors.iter() {
        selector.len().hash(&mut hasher);
        for component in selector.iter() {
            discriminant(component).hash(&mut hasher);
            match component {
                rocketcss_ast::SelectorComponent::DefaultNamespace(value)
                | rocketcss_ast::SelectorComponent::Id(value)
                | rocketcss_ast::SelectorComponent::Class(value) => value.hash(&mut hasher),
                rocketcss_ast::SelectorComponent::Namespace { prefix, url } => {
                    prefix.hash(&mut hasher);
                    url.hash(&mut hasher);
                }
                rocketcss_ast::SelectorComponent::LocalName { name, lower_name } => {
                    name.hash(&mut hasher);
                    lower_name.hash(&mut hasher);
                }
                rocketcss_ast::SelectorComponent::AttributeInNoNamespaceExists {
                    local_name,
                    local_name_lower,
                } => {
                    local_name.hash(&mut hasher);
                    local_name_lower.hash(&mut hasher);
                }
                _ => {}
            }
        }
    }
    rule.declarations.output_len().hash(&mut hasher);
    for (declaration, important) in OutputDeclarations::new(&rule.declarations) {
        declaration.name().hash(&mut hasher);
        declaration.vendor_prefix().bits().hash(&mut hasher);
        important.hash(&mut hasher);
        discriminant(declaration).hash(&mut hasher);
    }
    hasher.finish()
}

fn style_rules_are_identical(left: &StyleRule<'_>, right: &StyleRule<'_>) -> bool {
    left.vendor_prefix == right.vendor_prefix
        && left.selectors == right.selectors
        && left.rules.is_empty()
        && right.rules.is_empty()
        && style_rule_declarations_are_identical(left, right)
}

fn style_rule_declarations_are_identical(left: &StyleRule<'_>, right: &StyleRule<'_>) -> bool {
    left.declarations.output_len() == right.declarations.output_len()
        && OutputDeclarations::new(&left.declarations)
            .eq(OutputDeclarations::new(&right.declarations))
}

struct OutputDeclarations<'block, 'a> {
    block: Option<&'block DeclarationBlock<'a>>,
    index: usize,
}

impl<'block, 'a> OutputDeclarations<'block, 'a> {
    fn new(block: &'block DeclarationBlock<'a>) -> Self {
        Self {
            block: Some(block.first()),
            index: 0,
        }
    }
}

impl<'block, 'a> Iterator for OutputDeclarations<'block, 'a> {
    type Item = (&'block Declaration<'a>, bool);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let block = self.block?;
            while self.index < block.len() {
                let index = self.index;
                self.index += 1;
                if !block.is_invalid(index) {
                    return Some((&block.declarations[index], block.is_important(index)));
                }
            }
            self.block = block.next();
            self.index = 0;
        }
    }
}

fn discard_empty_rules(rules: &mut [CssRule<'_>], cx: &MinifyContext) {
    if cx.is_enabled(Options::DISCARD_EMPTY, OptionsOp::None) {
        return;
    }
    for rule in rules {
        let should_discard = match rule {
            CssRule::Style(style) => {
                style.declarations.is_output_empty()
                    && style
                        .rules
                        .iter()
                        .all(|rule| matches!(rule, CssRule::Ignored))
            }
            CssRule::Media(rule) => rule
                .rules
                .iter()
                .all(|rule| matches!(rule, CssRule::Ignored)),
            CssRule::Supports(rule) => rule
                .rules
                .iter()
                .all(|rule| matches!(rule, CssRule::Ignored)),
            CssRule::MozDocument(rule) => rule
                .rules
                .iter()
                .all(|rule| matches!(rule, CssRule::Ignored)),
            CssRule::Container(rule) => rule
                .rules
                .iter()
                .all(|rule| matches!(rule, CssRule::Ignored)),
            CssRule::Scope(rule) => rule
                .rules
                .iter()
                .all(|rule| matches!(rule, CssRule::Ignored)),
            CssRule::StartingStyle(rule) => rule
                .rules
                .iter()
                .all(|rule| matches!(rule, CssRule::Ignored)),
            CssRule::FontFace(rule) => rule.properties.is_empty(),
            CssRule::FontFeatureValues(rule) => rule.rules.is_empty(),
            CssRule::CounterStyle(rule) => rule.declarations.is_output_empty(),
            CssRule::Keyframes(rule) => {
                cx.is_enabled(Options::DISCARD_EMPTY_KEYFRAMES, OptionsOp::Any)
                    && rule.keyframes.is_empty()
            }
            CssRule::Viewport(rule) => rule.declarations.is_output_empty(),
            CssRule::Unknown(rule) => rule.block.as_ref().is_some_and(|block| block.is_empty()),
            _ => false,
        };
        if should_discard {
            *rule = CssRule::Ignored;
        }
    }
}

fn has_mergeable_style_run(rules: &[CssRule<'_>], cx: &MinifyContext) -> bool {
    let mut previous_style = None;
    for rule in rules {
        match rule {
            CssRule::Ignored => continue,
            CssRule::Style(current) => {
                if previous_style
                    .is_some_and(|previous| style_rule_merge_kind(previous, current, cx).is_some())
                {
                    return true;
                }
                previous_style = current.rules.is_empty().then_some(current.as_ref());
            }
            _ => previous_style = None,
        }
    }
    false
}

#[derive(Clone, Copy)]
enum StyleRuleMergeKind {
    SameSelectors,
    SameDeclarations,
    PreviousDeclarationsAreSubset,
    CurrentDeclarationsAreSubset,
}

fn style_rule_merge_kind(
    previous: &StyleRule<'_>,
    current: &StyleRule<'_>,
    cx: &MinifyContext,
) -> Option<StyleRuleMergeKind> {
    if !previous.rules.is_empty()
        || !current.rules.is_empty()
        || previous.vendor_prefix != current.vendor_prefix
    {
        return None;
    }
    let selectors_are_compatible = style_rule_selectors_are_compatible(previous, current, cx);
    if previous.selectors == current.selectors && selectors_are_compatible {
        return Some(StyleRuleMergeKind::SameSelectors);
    }
    if cx.is_enabled(Options::MERGE_STYLE_RULES, OptionsOp::None) || !selectors_are_compatible {
        return None;
    }
    if style_rule_declarations_are_equivalent(previous, current) {
        return Some(StyleRuleMergeKind::SameDeclarations);
    }
    if declaration_block_is_strict_subset(&previous.declarations, &current.declarations) {
        return Some(StyleRuleMergeKind::PreviousDeclarationsAreSubset);
    }
    if declaration_block_is_strict_subset(&current.declarations, &previous.declarations) {
        return Some(StyleRuleMergeKind::CurrentDeclarationsAreSubset);
    }
    None
}

fn style_rule_declarations_are_equivalent(left: &StyleRule<'_>, right: &StyleRule<'_>) -> bool {
    if left.declarations.output_len() != right.declarations.output_len()
        || !OutputDeclarations::new(&left.declarations)
            .all(|left| OutputDeclarations::new(&right.declarations).any(|right| left == right))
    {
        return false;
    }
    for (first_index, first) in OutputDeclarations::new(&left.declarations).enumerate() {
        for second in OutputDeclarations::new(&left.declarations).skip(first_index + 1) {
            if !properties_conflict(first.0.name(), second.0.name()) {
                continue;
            }
            let right_first = OutputDeclarations::new(&right.declarations)
                .position(|declaration| declaration == first);
            let right_second = OutputDeclarations::new(&right.declarations)
                .position(|declaration| declaration == second);
            if right_first >= right_second {
                return false;
            }
        }
    }
    true
}

fn declaration_block_is_strict_subset(
    subset: &DeclarationBlock<'_>,
    superset: &DeclarationBlock<'_>,
) -> bool {
    if !declaration_block_is_standalone(subset)
        || !declaration_block_is_standalone(superset)
        || subset.output_len() >= superset.output_len()
    {
        return false;
    }
    if !OutputDeclarations::new(subset).all(|declaration| {
        OutputDeclarations::new(superset).any(|candidate| candidate == declaration)
    }) {
        return false;
    }
    OutputDeclarations::new(superset)
        .filter(|candidate| {
            !OutputDeclarations::new(subset).any(|declaration| declaration == *candidate)
        })
        .all(|unmatched| {
            OutputDeclarations::new(subset)
                .all(|common| !properties_conflict(common.0.name(), unmatched.0.name()))
        })
}

fn declaration_block_is_standalone(block: &DeclarationBlock<'_>) -> bool {
    std::ptr::eq(block.first(), block) && block.next().is_none()
}

fn properties_conflict(left: &str, right: &str) -> bool {
    if left == right {
        return true;
    }
    if left == "all" || right == "all" {
        let other = if left == "all" { right } else { left };
        return !matches!(other, "direction" | "unicode-bidi");
    }
    let left = split_property(left);
    let right = split_property(right);
    if left.base.is_none() && right.base.is_none() {
        return true;
    }
    if left.base != right.base && left.base != Some("place") && right.base != Some("place") {
        return false;
    }
    if left.rest_count != right.rest_count {
        return true;
    }
    if left.base == Some("border")
        && left
            .rest
            .split('-')
            .chain(right.rest.split('-'))
            .any(|part| matches!(part, "image" | "width" | "color" | "style"))
    {
        return true;
    }
    if left.base == Some("flex")
        && ((left.rest == "flow" && matches!(right.rest, "wrap" | "direction"))
            || (right.rest == "flow" && matches!(left.rest, "wrap" | "direction")))
    {
        return true;
    }
    left.rest.split('-').eq(right.rest.split('-'))
}

struct PropertyParts<'a> {
    base: Option<&'a str>,
    rest: &'a str,
    rest_count: usize,
}

fn split_property(property: &str) -> PropertyParts<'_> {
    if property.starts_with("--") {
        return PropertyParts {
            base: None,
            rest: property,
            rest_count: 1,
        };
    }
    let property = property
        .strip_prefix('-')
        .and_then(|property| property.split_once('-').map(|(_, property)| property))
        .unwrap_or(property);
    let (base, rest) = property.split_once('-').unwrap_or((property, ""));
    PropertyParts {
        base: Some(base),
        rest,
        rest_count: if rest.is_empty() {
            0
        } else {
            rest.split('-').count()
        },
    }
}

fn tombstone_matching_declarations(
    target: &mut DeclarationBlock<'_>,
    common: &DeclarationBlock<'_>,
    cx: &mut MinifyContext,
) {
    for index in 0..target.len() {
        if target.is_invalid(index) {
            continue;
        }
        let declaration = (&target.declarations[index], target.is_important(index));
        if OutputDeclarations::new(common).any(|candidate| candidate == declaration) {
            target.mark_invalid(index);
            cx.record_declaration_removed();
        }
    }
}

fn style_rule_selectors_are_compatible(
    left: &StyleRule<'_>,
    right: &StyleRule<'_>,
    cx: &MinifyContext,
) -> bool {
    let Some(left_vendor) = style_rule_selector_vendor(left) else {
        return false;
    };
    let Some(right_vendor) = style_rule_selector_vendor(right) else {
        return false;
    };
    if left_vendor != right_vendor {
        return false;
    }
    left.selectors
        .iter()
        .chain(right.selectors.iter())
        .all(|selector| {
            selector.iter().all(|component| {
                selector_component_is_merge_compatible(
                    component,
                    cx.is_enabled(Options::MERGE_PLACEHOLDER_SELECTORS, OptionsOp::Any),
                )
            })
        })
}

fn style_rule_selector_vendor(rule: &StyleRule<'_>) -> Option<u8> {
    let mut vendor = 0;
    for selector in rule.selectors.iter() {
        for component in selector.iter() {
            let component_vendor = selector_component_vendor(component);
            if component_vendor == 0 {
                continue;
            }
            if vendor != 0 && vendor != component_vendor {
                return None;
            }
            vendor = component_vendor;
        }
    }
    Some(vendor)
}

fn selector_component_vendor(component: &SelectorComponent<'_>) -> u8 {
    match component {
        SelectorComponent::PseudoClass(pseudo) => match &**pseudo {
            PseudoClass::Custom { name } | PseudoClass::CustomFunction { name, .. } => {
                selector_name_vendor(name)
            }
            _ => 0,
        },
        SelectorComponent::PseudoElement(pseudo) => match &**pseudo {
            PseudoElement::Selection(prefix)
            | PseudoElement::Placeholder(prefix)
            | PseudoElement::Backdrop(prefix)
            | PseudoElement::FileSelectorButton(prefix) => {
                if *prefix == VendorPrefix::NONE {
                    0
                } else {
                    prefix.bits()
                }
            }
            PseudoElement::Custom { name } | PseudoElement::CustomFunction { name, .. } => {
                selector_name_vendor(name)
            }
            _ => 0,
        },
        _ => 0,
    }
}

fn selector_name_vendor(name: &str) -> u8 {
    if name
        .get(..8)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-webkit-"))
    {
        VendorPrefix::WEBKIT.bits()
    } else if name
        .get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-moz-"))
    {
        VendorPrefix::MOZ.bits()
    } else if name
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-ms-"))
    {
        VendorPrefix::MS.bits()
    } else if name
        .get(..3)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-o-"))
    {
        VendorPrefix::O.bits()
    } else {
        0
    }
}

fn selector_component_is_merge_compatible(
    component: &SelectorComponent<'_>,
    allow_placeholder: bool,
) -> bool {
    match component {
        SelectorComponent::PseudoClass(pseudo) => match &**pseudo {
            PseudoClass::Hover
            | PseudoClass::Active
            | PseudoClass::Focus
            | PseudoClass::Visited
            | PseudoClass::Link
            | PseudoClass::Checked
            | PseudoClass::Disabled
            | PseudoClass::Enabled => true,
            PseudoClass::FocusVisible | PseudoClass::FocusWithin => false,
            PseudoClass::Custom { name } => {
                [
                    "after",
                    "before",
                    "first-child",
                    "first-letter",
                    "first-line",
                    "first-of-type",
                    "invalid",
                    "lang",
                    "last-child",
                    "last-of-type",
                    "not",
                    "nth-child",
                    "nth-last-child",
                    "nth-last-of-type",
                    "nth-of-type",
                    "only-child",
                    "only-of-type",
                    "optional",
                    "required",
                    "target",
                    "valid",
                ]
                .into_iter()
                .any(|supported| name.eq_ignore_ascii_case(supported))
                    || selector_name_vendor(name) != 0
                        && !name.eq_ignore_ascii_case("-ms-input-placeholder")
            }
            PseudoClass::CustomFunction { .. } => false,
            _ => true,
        },
        SelectorComponent::PseudoElement(pseudo) => match &**pseudo {
            PseudoElement::After
            | PseudoElement::Before
            | PseudoElement::FirstLetter
            | PseudoElement::FirstLine
            | PseudoElement::Selection(VendorPrefix::NONE) => true,
            PseudoElement::Placeholder(VendorPrefix::NONE) => allow_placeholder,
            PseudoElement::Custom { name } => {
                name.eq_ignore_ascii_case("selection") || selector_name_vendor(name) != 0
            }
            _ => false,
        },
        SelectorComponent::AttributeInNoNamespace {
            case_sensitivity, ..
        } => !matches!(
            case_sensitivity,
            ParsedCaseSensitivity::AsciiCaseInsensitive
        ),
        SelectorComponent::AttributeOther(_) => false,
        SelectorComponent::Negation(selectors)
        | SelectorComponent::Where(selectors)
        | SelectorComponent::Is(selectors)
        | SelectorComponent::Has(selectors) => selectors.iter().all(|selector| {
            selector.iter().all(|component| {
                selector_component_is_merge_compatible(component, allow_placeholder)
            })
        }),
        SelectorComponent::NthOf { selectors, .. } | SelectorComponent::Any { selectors, .. } => {
            selectors.iter().all(|selector| {
                selector.iter().all(|component| {
                    selector_component_is_merge_compatible(component, allow_placeholder)
                })
            })
        }
        SelectorComponent::Host(_) => false,
        _ => true,
    }
}

fn deduplicate_declarations(block: &mut DeclarationBlock<'_>, cx: &mut MinifyContext) {
    discard_empty_declarations(block, cx);
    prepare_box_initial_values(block, cx);
    discard_overridden_columns(block, cx);
    discard_box_longhands_before_shorthand(block, cx);
    discard_obsolete_prefixed_declarations(block, cx);
    discard_overridden_same_properties(block, cx);
    merge_box_longhands(block, cx);
    for index in 1..block.declarations.len() {
        if block.is_invalid(index) {
            continue;
        }
        let previous = (0..index).rev().find(|&previous| {
            !block.is_invalid(previous)
                && block.is_important(previous) == block.is_important(index)
                && block.declarations[previous].name() == block.declarations[index].name()
                && block.declarations[previous].vendor_prefix()
                    == block.declarations[index].vendor_prefix()
        });
        if let Some(previous) = previous
            && block.declarations[previous] == block.declarations[index]
        {
            block.mark_invalid(previous);
            cx.record_declaration_removed();
        }
    }
    sort_declarations(block, cx);
}

fn discard_obsolete_prefixed_declarations(
    block: &mut DeclarationBlock<'_>,
    cx: &mut MinifyContext,
) {
    if cx.is_enabled(Options::DISCARD_OBSOLETE_PREFIXES, OptionsOp::None) {
        return;
    }
    for current in 0..block.len() {
        if block.is_invalid(current)
            || block.declarations[current].vendor_prefix() != VendorPrefix::NONE
            || !block.declarations[current]
                .name()
                .eq_ignore_ascii_case("box-sizing")
        {
            continue;
        }
        for previous in 0..current {
            let previous_name = block.declarations[previous].name();
            if block.is_invalid(previous)
                || block.is_important(previous) != block.is_important(current)
                || !(block.declarations[previous].vendor_prefix() == VendorPrefix::WEBKIT
                    && previous_name.eq_ignore_ascii_case("box-sizing")
                    || previous_name.eq_ignore_ascii_case("-webkit-box-sizing"))
            {
                continue;
            }
            block.mark_invalid(previous);
            cx.record_declaration_removed();
        }
    }
}

fn sort_declarations(block: &mut DeclarationBlock<'_>, cx: &mut MinifyContext) {
    if cx.is_enabled(Options::SORT_DECLARATIONS, OptionsOp::None) {
        return;
    }
    let mut changed = false;
    loop {
        let mut swapped = false;
        let mut previous = None;
        for current in 0..block.len() {
            if block.is_invalid(current) {
                continue;
            }
            if let Some(previous_index) = previous
                && declaration_sort_order(
                    &block.declarations[previous_index],
                    &block.declarations[current],
                ) == Ordering::Greater
            {
                block.swap(previous_index, current);
                swapped = true;
                changed = true;
            }
            previous = Some(current);
        }
        if !swapped {
            break;
        }
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn declaration_sort_order(left: &Declaration<'_>, right: &Declaration<'_>) -> Ordering {
    let left_name = left.name();
    let right_name = right.name();
    if left_name.eq_ignore_ascii_case("all") {
        return Ordering::Less;
    }
    if right_name.eq_ignore_ascii_case("all") {
        return Ordering::Greater;
    }
    if declaration_is_unsortable(left)
        || declaration_is_unsortable(right)
        || properties_conflict(left_name, right_name)
    {
        return Ordering::Equal;
    }
    left_name.cmp(right_name)
}

fn declaration_is_unsortable(declaration: &Declaration<'_>) -> bool {
    matches!(declaration, Declaration::Custom(_))
        || matches!(declaration, Declaration::Unparsed(value)
            if matches!(&*value.property_id, PropertyId::Custom(_)))
}

pub(crate) fn sort_font_face_properties(
    properties: &mut Vec<'_, FontFaceProperty<'_>>,
    cx: &mut MinifyContext,
) {
    if cx.is_enabled(Options::SORT_DECLARATIONS, OptionsOp::None) {
        return;
    }
    let mut changed = false;
    for current in 1..properties.len() {
        let mut index = current;
        while index != 0
            && font_face_property_name(&properties[index - 1])
                > font_face_property_name(&properties[index])
        {
            properties.swap(index - 1, index);
            index -= 1;
            changed = true;
        }
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn font_face_property_name<'a>(property: &FontFaceProperty<'a>) -> &'a str {
    match property {
        FontFaceProperty::Source(_) => "src",
        FontFaceProperty::FontFamily(_) => "font-family",
        FontFaceProperty::FontStyle(_) => "font-style",
        FontFaceProperty::FontWeight(_) => "font-weight",
        FontFaceProperty::FontStretch(_) => "font-stretch",
        FontFaceProperty::UnicodeRange(_) => "unicode-range",
        FontFaceProperty::Custom(value) => match &*value.name {
            rocketcss_ast::CustomPropertyName::Custom(name)
            | rocketcss_ast::CustomPropertyName::Unknown(name) => name,
        },
    }
}

fn discard_box_longhands_before_shorthand(
    block: &mut DeclarationBlock<'_>,
    cx: &mut MinifyContext,
) {
    for shorthand in 1..block.len() {
        if block.is_invalid(shorthand) {
            continue;
        }
        if declaration_contains_variable(&block.declarations[shorthand]) {
            continue;
        }
        let longhands = match block.declarations[shorthand].name() {
            "margin" => Some(["margin-top", "margin-right", "margin-bottom", "margin-left"]),
            "padding" => Some([
                "padding-top",
                "padding-right",
                "padding-bottom",
                "padding-left",
            ]),
            "border" => None,
            _ => continue,
        };
        for previous in 0..shorthand {
            if block.is_invalid(previous)
                || block.is_important(previous) != block.is_important(shorthand)
                || !longhands.map_or_else(
                    || border_shorthand_overrides(block.declarations[previous].name()),
                    |longhands| {
                        longhands.iter().any(|name| {
                            block.declarations[previous]
                                .name()
                                .eq_ignore_ascii_case(name)
                        })
                    },
                )
            {
                continue;
            }
            block.mark_invalid(previous);
            cx.record_declaration_removed();
        }
    }
}

fn border_shorthand_overrides(name: &str) -> bool {
    let Some(suffix) = name.strip_prefix("border-") else {
        return false;
    };
    suffix.split('-').next().is_some_and(|component| {
        [
            "top", "right", "bottom", "left", "width", "style", "color", "image",
        ]
        .iter()
        .any(|candidate| component.eq_ignore_ascii_case(candidate))
    })
}

fn is_margin_or_padding_longhand(name: &str) -> bool {
    [
        "margin-top",
        "margin-right",
        "margin-bottom",
        "margin-left",
        "padding-top",
        "padding-right",
        "padding-bottom",
        "padding-left",
    ]
    .iter()
    .any(|property| name.eq_ignore_ascii_case(property))
}

fn prepare_box_initial_values(block: &mut DeclarationBlock<'_>, cx: &MinifyContext) {
    if cx.is_enabled(Options::PRESERVE_MERGED_BOX_INITIAL, OptionsOp::None) {
        return;
    }
    let allocator = block.declarations.bump();
    let mut preserve = BitVec::new(allocator);
    for _ in 0..block.len() {
        preserve.push(false);
    }
    for important in [false, true] {
        for (names, shorthand) in [
            (
                ["margin-top", "margin-right", "margin-bottom", "margin-left"],
                "margin",
            ),
            (
                [
                    "padding-top",
                    "padding-right",
                    "padding-bottom",
                    "padding-left",
                ],
                "padding",
            ),
        ] {
            let Some(indices) = box_longhand_indices(block, important, names, shorthand) else {
                continue;
            };
            if indices.iter().all(|&index| {
                matches!(&block.declarations[index], Declaration::Unparsed(value)
                    if matches!(value.value.as_slice(), [TokenOrValue::Token(token)]
                        if matches!(&**token, Token::Ident(keyword)
                            if keyword.eq_ignore_ascii_case("initial"))))
            }) {
                for index in indices {
                    preserve.set(index, true);
                }
            }
        }
    }
    for index in 0..block.len() {
        if preserve.is_set(index)
            || !is_margin_or_padding_longhand(block.declarations[index].name())
        {
            continue;
        }
        if let Declaration::Unparsed(value) = &mut block.declarations[index]
            && let [TokenOrValue::Token(token)] = value.value.as_mut_slice()
            && matches!(&**token, Token::Ident(keyword) if keyword.eq_ignore_ascii_case("initial"))
        {
            **token = Token::Number(0.0);
        }
    }
}

fn discard_overridden_same_properties(block: &mut DeclarationBlock<'_>, cx: &mut MinifyContext) {
    for current in 1..block.len() {
        if block.is_invalid(current) {
            continue;
        }
        let current_name = block.declarations[current].name();
        let current_important = block.is_important(current);
        let current_is_custom = declaration_contains_variable(&block.declarations[current]);
        for previous in (0..current).rev() {
            if block.is_invalid(previous)
                || block.is_important(previous) != current_important
                || block.declarations[previous].name() != current_name
            {
                continue;
            }
            let previous_is_custom = declaration_contains_variable(&block.declarations[previous]);
            if !previous_is_custom && !current_is_custom {
                continue;
            }
            if !previous_is_custom && current_is_custom {
                continue;
            }
            block.mark_invalid(previous);
            cx.record_declaration_removed();
        }
    }
}

fn declaration_contains_variable(declaration: &Declaration<'_>) -> bool {
    matches!(declaration, Declaration::Unparsed(value)
        if value.value.iter().any(token_or_value_contains_variable))
}

fn token_or_value_contains_variable(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Var(_) => true,
        TokenOrValue::Function(function) => {
            function.name.eq_ignore_ascii_case("var")
                || function
                    .arguments
                    .iter()
                    .any(token_or_value_contains_variable)
        }
        _ => false,
    }
}

fn discard_overridden_columns(block: &mut DeclarationBlock<'_>, cx: &mut MinifyContext) {
    for important in [false, true] {
        let Some(last_shorthand) = (0..block.declarations.len()).rev().find(|&index| {
            !block.is_invalid(index)
                && block.is_important(index) == important
                && block.declarations[index]
                    .name()
                    .eq_ignore_ascii_case("columns")
        }) else {
            merge_column_longhands(block, important, 0, cx);
            continue;
        };
        for index in 0..last_shorthand {
            if !block.is_invalid(index)
                && block.is_important(index) == important
                && is_column_property(block.declarations[index].name())
            {
                block.mark_invalid(index);
                cx.record_declaration_removed();
            }
        }
        fold_column_override(block, important, last_shorthand, cx);
        merge_column_longhands(block, important, last_shorthand + 1, cx);
    }
}

fn is_column_property(name: &str) -> bool {
    ["columns", "column-count", "column-width"]
        .iter()
        .any(|property| name.eq_ignore_ascii_case(property))
}

fn merge_column_longhands(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    start: usize,
    cx: &mut MinifyContext,
) {
    let mut width = None;
    let mut count = None;
    for index in start..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        let name = block.declarations[index].name();
        if name.eq_ignore_ascii_case("column-width") {
            width = Some(index);
        } else if name.eq_ignore_ascii_case("column-count") {
            count = Some(index);
        }
    }
    let (Some(width), Some(count)) = (width, count) else {
        return;
    };
    let can_merge = matches!(&block.declarations[width], Declaration::Unparsed(value)
        if value.value.len() == 1 && !declaration_contains_variable(&block.declarations[width]))
        && matches!(&block.declarations[count], Declaration::Unparsed(value)
            if value.value.len() == 1 && !declaration_contains_variable(&block.declarations[count]));
    if !can_merge {
        return;
    }
    let css_wide = match (&block.declarations[width], &block.declarations[count]) {
        (Declaration::Unparsed(width), Declaration::Unparsed(count)) => {
            let width_wide = is_css_wide_value(&width.value[0]);
            let count_wide = is_css_wide_value(&count.value[0]);
            if width_wide || count_wide {
                if width.value != count.value {
                    return;
                }
                true
            } else {
                false
            }
        }
        _ => return,
    };
    let allocator = block.declarations.bump();
    let (width_declaration, count_declaration) = if width < count {
        let (before, after) = block.declarations.split_at_mut(count);
        (&mut before[width], &mut after[0])
    } else {
        let (before, after) = block.declarations.split_at_mut(width);
        (&mut after[0], &mut before[count])
    };
    let (Declaration::Unparsed(width_value), Declaration::Unparsed(count_value)) =
        (width_declaration, count_declaration)
    else {
        return;
    };
    let mut width_part = std::mem::replace(&mut width_value.value, allocator.vec());
    let mut count_part = std::mem::replace(&mut count_value.value, allocator.vec());
    canonicalize_auto(&mut width_part);
    canonicalize_auto(&mut count_part);
    let width_auto = token_ident(&width_part[0]).is_some_and(|value| value == "auto");
    let count_auto = token_ident(&count_part[0]).is_some_and(|value| value == "auto");
    let mut value = if count_auto && !width_auto {
        width_part
    } else if width_auto || css_wide {
        count_part
    } else {
        width_part.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        width_part.append(&mut count_part);
        width_part
    };
    if width_auto && count_auto {
        value.truncate(1);
    }
    let target = width.max(count);
    let Declaration::Unparsed(target_value) = &mut block.declarations[target] else {
        unreachable!()
    };
    *target_value.property_id = PropertyId::from_name("columns");
    target_value.value = value;
    let removed = width.min(count);
    block.mark_invalid(removed);
    cx.record_declaration_removed();
}

fn fold_column_override(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    shorthand: usize,
    cx: &mut MinifyContext,
) {
    let mut override_index = None;
    let mut override_is_width = false;
    for index in (shorthand + 1)..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        let name = block.declarations[index].name();
        if name.eq_ignore_ascii_case("column-width") {
            override_index = Some(index);
            override_is_width = true;
        } else if name.eq_ignore_ascii_case("column-count") {
            override_index = Some(index);
            override_is_width = false;
        }
    }
    let Some(override_index) = override_index else {
        return;
    };
    let can_fold = matches!(&block.declarations[shorthand], Declaration::Unparsed(value)
        if column_other_component_is_auto(&value.value, override_is_width))
        && matches!(&block.declarations[override_index], Declaration::Unparsed(value)
            if value.value.len() == 1 && !declaration_contains_variable(&block.declarations[override_index]));
    if !can_fold {
        return;
    }
    let Declaration::Unparsed(override_value) = &mut block.declarations[override_index] else {
        return;
    };
    *override_value.property_id = PropertyId::from_name("columns");
    canonicalize_auto(&mut override_value.value);
    block.mark_invalid(shorthand);
    cx.record_declaration_removed();
}

fn column_other_component_is_auto(value: &[TokenOrValue<'_>], override_is_width: bool) -> bool {
    let mut parts = value.iter().filter(|value| !is_token_whitespace(value));
    let Some(first) = parts.next() else {
        return false;
    };
    let second = parts.next();
    if parts.next().is_some() {
        return false;
    }
    match second {
        None => {
            let is_number =
                matches!(first, TokenOrValue::Token(token) if matches!(&**token, Token::Number(_)));
            let is_auto =
                token_ident(first).is_some_and(|value| value.eq_ignore_ascii_case("auto"));
            if override_is_width {
                !is_number || is_auto
            } else {
                is_number || is_auto
            }
        }
        Some(second) => {
            let other = if override_is_width {
                if matches!(first, TokenOrValue::Token(token) if matches!(&**token, Token::Number(_)))
                {
                    first
                } else {
                    second
                }
            } else if matches!(first, TokenOrValue::Token(token) if matches!(&**token, Token::Number(_)))
            {
                second
            } else {
                first
            };
            token_ident(other).is_some_and(|value| value.eq_ignore_ascii_case("auto"))
        }
    }
}

fn canonicalize_auto(value: &mut Vec<'_, TokenOrValue<'_>>) {
    for value in value {
        if let TokenOrValue::Token(token) = value
            && let Token::Ident(keyword) = &**token
            && let Some(canonical) = ["auto", "inherit", "initial", "unset", "revert"]
                .into_iter()
                .find(|candidate| keyword.eq_ignore_ascii_case(candidate))
        {
            **token = Token::Ident(canonical);
        }
    }
}

fn merge_box_longhands<'a>(block: &mut DeclarationBlock<'a>, cx: &mut MinifyContext) {
    for important in [false, true] {
        merge_border_longhands(block, important, cx);
        fold_margin_side_overrides(block, important, cx);
        fold_border_component_overrides(
            block,
            important,
            ["margin-top", "margin-right", "margin-bottom", "margin-left"],
            "margin",
            cx,
        );
        merge_unparsed_box_groups(
            block,
            important,
            ["margin-top", "margin-right", "margin-bottom", "margin-left"],
            "margin",
            cx,
        );
        merge_margin_longhands(block, important, cx);
        fold_padding_side_overrides(block, important, cx);
        fold_border_component_overrides(
            block,
            important,
            [
                "padding-top",
                "padding-right",
                "padding-bottom",
                "padding-left",
            ],
            "padding",
            cx,
        );
        merge_unparsed_box_groups(
            block,
            important,
            [
                "padding-top",
                "padding-right",
                "padding-bottom",
                "padding-left",
            ],
            "padding",
            cx,
        );
        merge_padding_longhands(block, important, cx);
    }
}

fn fold_margin_side_overrides(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let mut shorthand = None;
    for index in 0..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        if matches!(block.declarations[index], Declaration::Margin(_)) {
            shorthand = Some(index);
            continue;
        }
        let Some(target) = shorthand else {
            continue;
        };
        let (shorthand_declaration, longhand_declaration) = if target < index {
            let (before, after) = block.declarations.split_at_mut(index);
            (&mut before[target], &mut after[0])
        } else {
            unreachable!("the shorthand is always before its override")
        };
        let Declaration::Margin(value) = shorthand_declaration else {
            continue;
        };
        let folded = match longhand_declaration {
            Declaration::MarginTop(longhand) => {
                std::mem::swap(&mut value.top, longhand);
                true
            }
            Declaration::MarginRight(longhand) => {
                std::mem::swap(&mut value.right, longhand);
                true
            }
            Declaration::MarginBottom(longhand) => {
                std::mem::swap(&mut value.bottom, longhand);
                true
            }
            Declaration::MarginLeft(longhand) => {
                std::mem::swap(&mut value.left, longhand);
                true
            }
            _ => false,
        };
        if folded {
            block.mark_invalid(index);
            cx.record_declaration_removed();
        }
    }
}

fn fold_padding_side_overrides(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let mut shorthand = None;
    for index in 0..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        if matches!(block.declarations[index], Declaration::Padding(_)) {
            shorthand = Some(index);
            continue;
        }
        let Some(target) = shorthand else {
            continue;
        };
        let (shorthand_declaration, longhand_declaration) = if target < index {
            let (before, after) = block.declarations.split_at_mut(index);
            (&mut before[target], &mut after[0])
        } else {
            unreachable!("the shorthand is always before its override")
        };
        let Declaration::Padding(value) = shorthand_declaration else {
            continue;
        };
        let folded = match longhand_declaration {
            Declaration::PaddingTop(longhand) => {
                std::mem::swap(&mut value.top, longhand);
                true
            }
            Declaration::PaddingRight(longhand) => {
                std::mem::swap(&mut value.right, longhand);
                true
            }
            Declaration::PaddingBottom(longhand) => {
                std::mem::swap(&mut value.bottom, longhand);
                true
            }
            Declaration::PaddingLeft(longhand) => {
                std::mem::swap(&mut value.left, longhand);
                true
            }
            _ => false,
        };
        if folded {
            block.mark_invalid(index);
            cx.record_declaration_removed();
        }
    }
}

fn merge_unparsed_box_groups<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    names: [&str; 4],
    shorthand: &'a str,
    cx: &mut MinifyContext,
) {
    let allocator = block.declarations.bump();
    let last_shorthand = (0..block.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == shorthand
    });
    let start = last_shorthand.map_or(0, |index| index + 1);
    let mut candidates = BitVec::new(allocator);
    for index in 0..block.len() {
        candidates.push(
            index >= start
                && !block.is_invalid(index)
                && block.is_important(index) == important
                && names.contains(&block.declarations[index].name())
                && !declaration_contains_variable(&block.declarations[index]),
        );
    }

    while let Some(last) = (0..block.len())
        .rev()
        .find(|&index| candidates.is_set(index))
    {
        let mut indices = [None; 4];
        for index in start..=last {
            if !candidates.is_set(index) {
                continue;
            }
            if let Some(side) = names
                .iter()
                .position(|name| block.declarations[index].name() == *name)
            {
                indices[side] = Some(index);
            }
        }
        let [Some(top), Some(right), Some(bottom), Some(left)] = indices else {
            candidates.set(last, false);
            continue;
        };
        let indices = [top, right, bottom, left];
        if merge_unparsed_box_longhands(block, indices, shorthand, cx) {
            for index in indices {
                candidates.set(index, false);
            }
        } else {
            candidates.set(last, false);
        }
    }
}

fn merge_border_longhands<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    cx: &mut MinifyContext,
) {
    discard_redundant_border_side_overrides(block, important, cx);
    merge_border_side_shorthands(block, important, cx);
    factor_border_side_shorthand_override(block, important, cx);
    fold_border_side_components_around_variable_border(block, important, cx);
    fold_border_shorthand_component_overrides(block, important, cx);
    for (component, output) in [
        ("width", "border-width"),
        ("style", "border-style"),
        ("color", "border-color"),
    ] {
        let names = [
            border_name("top", component),
            border_name("right", component),
            border_name("bottom", component),
            border_name("left", component),
        ];
        fold_majority_border_component_shorthand(block, important, names, output, cx);
        fold_three_equal_border_side_component_overrides(block, important, names, output, cx);
        factor_border_side_component_overrides(block, important, names, output, cx);
        fold_border_component_overrides(block, important, names, output, cx);
        merge_unparsed_border_component_groups(block, important, names, output, cx);
        if let Some(indices) = declaration_indices(block, important, names, output) {
            merge_unparsed_values(block, indices, output, true, cx);
        }
    }
    fold_border_shorthand_component_overrides(block, important, cx);
    fold_border_color_after_width_shorthand(block, important, cx);

    for side in ["top", "right", "bottom", "left"] {
        let output = border_side_name(side);
        let names = [
            border_name(side, "width"),
            border_name(side, "style"),
            border_name(side, "color"),
        ];
        if let Some(indices) = declaration_indices(block, important, names, output) {
            merge_unparsed_values(block, indices, output, false, cx);
        }
        if let Some(indices) = declaration_indices(
            block,
            important,
            [border_name(side, "width"), border_name(side, "style")],
            output,
        ) && can_merge_without_color(block, border_name(side, "color"), important, &indices)
        {
            let target = *indices.iter().max().expect("two side components");
            if merge_unparsed_values(block, indices, output, false, cx) {
                move_border_side_before_earlier_color(
                    block,
                    target,
                    border_name(side, "color"),
                    important,
                );
            }
        }
    }

    if let Some(indices) = declaration_indices(
        block,
        important,
        ["border-width", "border-style", "border-color"],
        "border",
    ) {
        merge_unparsed_values(block, indices, "border", false, cx);
    }
    if let Some(indices) =
        declaration_indices(block, important, ["border-width", "border-style"], "border")
        && can_merge_without_color(block, "border-color", important, &indices)
    {
        merge_unparsed_values(block, indices, "border", false, cx);
    }

    if let Some(indices) = declaration_indices(
        block,
        important,
        ["border-top", "border-right", "border-bottom", "border-left"],
        "border",
    ) {
        merge_equal_unparsed_values(block, indices, "border", cx);
    }
    promote_two_equal_border_sides(block, important, cx);
    merge_common_border_sides(block, important, cx);
    canonicalize_border_side_declaration_order(block, important);
    canonicalize_border_component_declaration_order(block, important);
    canonicalize_standalone_border_style(block);
}

fn fold_border_side_components_around_variable_border(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let Some(border) = (0..block.len()).find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == "border"
            && declaration_contains_variable(&block.declarations[index])
    }) else {
        return;
    };
    let allocator = block.declarations.bump();
    let width = match &block.declarations[border] {
        Declaration::Unparsed(value) => {
            border_component_index(&value.value, BorderComponent::Width)
                .and_then(|index| clone_simple_token_or_value(&value.value[index], allocator))
        }
        _ => None,
    };
    let Some(width) = width else {
        return;
    };
    for side in ["top", "right", "bottom", "left"] {
        let style_name = border_name(side, "style");
        let color_name = border_name(side, "color");
        let width_name = border_name(side, "width");
        let style = (0..border).rev().find(|&index| {
            !block.is_invalid(index)
                && block.is_important(index) == important
                && block.declarations[index].name() == style_name
        });
        let color = (0..border).rev().find(|&index| {
            !block.is_invalid(index)
                && block.is_important(index) == important
                && block.declarations[index].name() == color_name
        });
        let later_width = ((border + 1)..block.len()).find(|&index| {
            !block.is_invalid(index)
                && block.is_important(index) == important
                && block.declarations[index].name() == width_name
        });
        let (Some(style), Some(color), Some(_later_width)) = (style, color, later_width) else {
            continue;
        };
        let can_fold = [style, color].into_iter().all(|index| {
            matches!(&block.declarations[index], Declaration::Unparsed(value)
                if value.value.len() == 1
                    && !value.value.iter().any(token_or_value_contains_variable))
        });
        if !can_fold {
            continue;
        }
        let mut style_value = match &mut block.declarations[style] {
            Declaration::Unparsed(value) => std::mem::replace(&mut value.value, allocator.vec()),
            _ => continue,
        };
        let mut color_value = match &mut block.declarations[color] {
            Declaration::Unparsed(value) => std::mem::replace(&mut value.value, allocator.vec()),
            _ => continue,
        };
        let target = style.max(color);
        let removed = style.min(color);
        let Declaration::Unparsed(target_value) = &mut block.declarations[target] else {
            continue;
        };
        target_value.value.push(
            clone_simple_token_or_value(&width, allocator)
                .unwrap_or_else(|| unreachable!("the border width was validated as cloneable")),
        );
        target_value
            .value
            .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        target_value.value.append(&mut style_value);
        target_value
            .value
            .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        target_value.value.append(&mut color_value);
        *target_value.property_id = PropertyId::from_name(border_side_name(side));
        canonicalize_full_border_keywords(&mut target_value.value);
        block.mark_invalid(removed);
        cx.record_declaration_removed();
        minify_unparsed_declaration(block, target, cx);
    }
}

fn fold_border_color_after_width_shorthand(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let Some(border) = (0..block.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == "border"
    }) else {
        return;
    };
    let Some(width) = ((border + 1)..block.len()).find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == "border-width"
    }) else {
        return;
    };
    let (style_component, has_width_component) = match &block.declarations[border] {
        Declaration::Unparsed(value) => (
            border_component_index(&value.value, BorderComponent::Style),
            border_component_index(&value.value, BorderComponent::Width).is_some(),
        ),
        _ => return,
    };
    if has_width_component {
        return;
    }
    let Some(style_component) = style_component else {
        return;
    };
    let allocator = block.declarations.bump();
    let style = match &block.declarations[border] {
        Declaration::Unparsed(value) => {
            clone_simple_token_or_value(&value.value[style_component], allocator)
        }
        _ => None,
    };
    let Some(style) = style else {
        return;
    };
    let names = [
        "border-top-color",
        "border-right-color",
        "border-bottom-color",
        "border-left-color",
    ];
    let Some(color) = ((width + 1)..block.len()).find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && names.contains(&block.declarations[index].name())
    }) else {
        return;
    };
    let side = names
        .iter()
        .position(|name| block.declarations[color].name() == *name)
        .expect("the side color matched one name");
    let can_fold = matches!(&block.declarations[color], Declaration::Unparsed(value)
        if value.value.len() == 1
            && !value.value.iter().any(token_or_value_contains_variable));
    if !can_fold {
        return;
    }
    let Declaration::Unparsed(color_value) = &mut block.declarations[color] else {
        return;
    };
    let mut old = std::mem::replace(&mut color_value.value, allocator.vec());
    let color_component = old.remove(0);
    color_value.value.push(style);
    color_value
        .value
        .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
    color_value.value.push(color_component);
    *color_value.property_id =
        PropertyId::from_name(["border-top", "border-right", "border-bottom", "border-left"][side]);
    canonicalize_full_border_keywords(&mut color_value.value);
    minify_unparsed_declaration(block, color, cx);
    block.declarations.swap(width, color);
}

fn discard_redundant_border_side_overrides(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let mut border = None;
    for index in 0..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        let name = block.declarations[index].name();
        if name == "border" {
            border = Some(index);
            continue;
        }
        let Some(border_index) = border else {
            continue;
        };
        let redundant =
            if ["border-top", "border-right", "border-bottom", "border-left"].contains(&name) {
                match (
                    &block.declarations[border_index],
                    &block.declarations[index],
                ) {
                    (Declaration::Unparsed(border), Declaration::Unparsed(side)) => {
                        border.value == side.value
                    }
                    _ => false,
                }
            } else {
                let component = if name.ends_with("-width") {
                    Some(BorderComponent::Width)
                } else if name.ends_with("-style") {
                    Some(BorderComponent::Style)
                } else if name.ends_with("-color") {
                    Some(BorderComponent::Color)
                } else {
                    None
                };
                component.is_some_and(|component| {
                    match (
                        &block.declarations[border_index],
                        &block.declarations[index],
                    ) {
                        (Declaration::Unparsed(border), Declaration::Unparsed(side))
                            if side.value.len() == 1 =>
                        {
                            border_component_index(&border.value, component)
                                .is_some_and(|target| border.value[target] == side.value[0])
                        }
                        _ => false,
                    }
                })
            };
        if redundant {
            block.mark_invalid(index);
            cx.record_declaration_removed();
        }
    }
}

fn factor_border_side_shorthand_override(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let allocator = block.declarations.bump();
    let mut border = None;
    for index in 0..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        let name = block.declarations[index].name();
        if name == "border" {
            border = Some(index);
            continue;
        }
        let Some(side) = ["border-top", "border-right", "border-bottom", "border-left"]
            .iter()
            .position(|candidate| name == *candidate)
        else {
            continue;
        };
        let Some(border_index) = border.take() else {
            continue;
        };
        let (Declaration::Unparsed(border_value), Declaration::Unparsed(side_value)) = (
            &block.declarations[border_index],
            &block.declarations[index],
        ) else {
            border = Some(border_index);
            continue;
        };
        if border_value
            .value
            .iter()
            .any(token_or_value_contains_variable)
            || side_value
                .value
                .iter()
                .any(token_or_value_contains_variable)
        {
            border = Some(border_index);
            continue;
        }
        let mut difference = None;
        let mut valid = true;
        for component in [
            BorderComponent::Width,
            BorderComponent::Style,
            BorderComponent::Color,
        ] {
            let Some(border_component) = border_component_index(&border_value.value, component)
            else {
                valid = false;
                break;
            };
            let Some(side_component) = border_component_index(&side_value.value, component) else {
                valid = false;
                break;
            };
            if border_value.value[border_component] != side_value.value[side_component] {
                if difference.is_some() {
                    valid = false;
                    break;
                }
                difference = Some((component, border_component, side_component));
            }
        }
        let Some((component, border_component, side_component)) = difference else {
            border = Some(border_index);
            continue;
        };
        if !valid
            || clone_simple_token_or_value(&border_value.value[border_component], allocator)
                .is_none()
        {
            border = Some(border_index);
            continue;
        }
        let Some(base) = take_border_component(block, border_index, component) else {
            border = Some(border_index);
            continue;
        };
        let first_clone = clone_simple_token_or_value(&base, allocator)
            .expect("the differing border component was validated as cloneable");
        let second_clone = clone_simple_token_or_value(&base, allocator)
            .expect("the differing border component was validated as cloneable");
        let third_clone = clone_simple_token_or_value(&base, allocator)
            .expect("the differing border component was validated as cloneable");
        let Declaration::Unparsed(side_value) = &mut block.declarations[index] else {
            unreachable!()
        };
        let mut old_side = std::mem::replace(&mut side_value.value, allocator.vec());
        let override_component = old_side.remove(side_component);
        let mut values = allocator.vec();
        values.push(base);
        values.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        values.push(first_clone);
        values.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        values.push(second_clone);
        values.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        values.push(third_clone);
        values[side * 2] = override_component;
        *side_value.property_id = PropertyId::from_name(match component {
            BorderComponent::Width => "border-width",
            BorderComponent::Style => "border-style",
            BorderComponent::Color => "border-color",
        });
        side_value.value = values;
        minify_unparsed_declaration(block, index, cx);
        minify_border_declaration(block, border_index, cx);
    }
}

fn fold_majority_border_component_shorthand<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    names: [&'a str; 4],
    shorthand: &'a str,
    cx: &mut MinifyContext,
) -> bool {
    let Some(border) = (0..block.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == "border"
    }) else {
        return false;
    };
    let Some(component_index) = ((border + 1)..block.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == shorthand
    }) else {
        return false;
    };
    if shorthand == "border-color"
        && fold_typed_majority_border_color(block, border, component_index, cx)
    {
        return true;
    }
    let component = match shorthand {
        "border-width" => BorderComponent::Width,
        "border-style" => BorderComponent::Style,
        "border-color" => BorderComponent::Color,
        _ => unreachable!(),
    };
    if matches!(&block.declarations[border], Declaration::Unparsed(value)
        if border_component_index(&value.value, component).is_some())
    {
        return false;
    }
    let Declaration::Unparsed(value) = &block.declarations[component_index] else {
        return false;
    };
    if value.value.iter().any(token_or_value_contains_variable) {
        return false;
    }
    let mut actual = [0; 4];
    let mut count = 0;
    for (index, item) in value.value.iter().enumerate() {
        if !is_token_whitespace(item) {
            if count == 4 {
                return false;
            }
            actual[count] = index;
            count += 1;
        }
    }
    if !(2..=4).contains(&count) {
        return false;
    }
    let semantic = match count {
        2 => [actual[0], actual[1], actual[0], actual[1]],
        3 => [actual[0], actual[1], actual[2], actual[1]],
        4 => actual,
        _ => unreachable!(),
    };
    let mut common_side = None;
    let mut exception_side = None;
    for candidate in 0..4 {
        let equal_count = (0..4)
            .filter(|&side| value.value[semantic[candidate]] == value.value[semantic[side]])
            .count();
        if equal_count == 3 {
            common_side = Some(candidate);
            exception_side = (0..4)
                .find(|&side| value.value[semantic[candidate]] != value.value[semantic[side]]);
            break;
        }
    }
    let (Some(common_side), Some(exception_side)) = (common_side, exception_side) else {
        return false;
    };
    let common_index = semantic[common_side];
    let exception_index = semantic[exception_side];
    if common_index == exception_index {
        return false;
    }
    let allocator = block.declarations.bump();
    let Declaration::Unparsed(value) = &mut block.declarations[component_index] else {
        return false;
    };
    let mut old = std::mem::replace(&mut value.value, allocator.vec());
    let high = common_index.max(exception_index);
    let low = common_index.min(exception_index);
    let high_value = old.remove(high);
    let low_value = old.remove(low);
    let (common, exception) = if common_index > exception_index {
        (high_value, low_value)
    } else {
        (low_value, high_value)
    };
    value.value.push(exception);
    *value.property_id = PropertyId::from_name(names[exception_side]);
    let Declaration::Unparsed(border_value) = &mut block.declarations[border] else {
        return false;
    };
    if !border_value.value.is_empty() {
        border_value
            .value
            .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
    }
    border_value.value.push(common);
    minify_border_declaration(block, border, cx);
    minify_unparsed_declaration(block, component_index, cx);
    true
}

fn fold_typed_majority_border_color(
    block: &mut DeclarationBlock<'_>,
    border: usize,
    component_index: usize,
    cx: &mut MinifyContext,
) -> bool {
    let Declaration::BorderColor(colors) = &block.declarations[component_index] else {
        return false;
    };
    let sides = [&colors.top, &colors.right, &colors.bottom, &colors.left];
    let mut common_side = None;
    let mut exception_side = None;
    for candidate in 0..4 {
        let equal_count = (0..4)
            .filter(|&side| sides[candidate] == sides[side])
            .count();
        if equal_count == 3 {
            common_side = Some(candidate);
            exception_side = (0..4).find(|&side| sides[candidate] != sides[side]);
            break;
        }
    }
    let (Some(common_side), Some(exception_side)) = (common_side, exception_side) else {
        return false;
    };
    let old = std::mem::replace(
        &mut block.declarations[component_index],
        Declaration::All(CSSWideKeyword::Initial),
    );
    let Declaration::BorderColor(colors) = old else {
        unreachable!()
    };
    let colors = rocketcss_allocator::boxed::Box::into_inner(colors);
    let mut sides = [
        Some(colors.top),
        Some(colors.right),
        Some(colors.bottom),
        Some(colors.left),
    ];
    let common = sides[common_side]
        .take()
        .expect("the common color is present");
    let exception = sides[exception_side]
        .take()
        .expect("the exceptional color is present");
    block.declarations[component_index] = match exception_side {
        0 => Declaration::BorderTopColor(exception),
        1 => Declaration::BorderRightColor(exception),
        2 => Declaration::BorderBottomColor(exception),
        3 => Declaration::BorderLeftColor(exception),
        _ => unreachable!(),
    };
    let allocator = block.declarations.bump();
    let Declaration::Unparsed(border_value) = &mut block.declarations[border] else {
        return false;
    };
    if !border_value.value.is_empty() {
        border_value
            .value
            .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
    }
    border_value.value.push(TokenOrValue::Color(common));
    minify_border_declaration(block, border, cx);
    true
}

fn fold_three_equal_border_side_component_overrides<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    names: [&'a str; 4],
    shorthand: &str,
    cx: &mut MinifyContext,
) {
    let Some(border) = (0..block.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == "border"
    }) else {
        return;
    };
    let mut indices = [None; 4];
    for index in (border + 1)..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        if matches!(
            block.declarations[index].name(),
            "border" | "border-width" | "border-style" | "border-color"
        ) {
            return;
        }
        if let Some(side) = names
            .iter()
            .position(|name| block.declarations[index].name() == *name)
        {
            indices[side] = Some(index);
        }
    }
    if indices.iter().flatten().count() != 3 {
        return;
    }
    let mut present = [0; 3];
    let mut present_count = 0;
    for index in indices.iter().flatten().copied() {
        present[present_count] = index;
        present_count += 1;
    }
    debug_assert_eq!(present_count, 3);
    let first = present[0];
    let can_fold = present.iter().all(|&index| {
        matches!(&block.declarations[index], Declaration::Unparsed(value)
        if value.value.len() == 1
            && !declaration_contains_variable(&block.declarations[index])
                && &value.value == match &block.declarations[first] {
                Declaration::Unparsed(first) => &first.value,
                _ => unreachable!(),
            })
    });
    if !can_fold {
        return;
    }
    let component = match shorthand {
        "border-width" => BorderComponent::Width,
        "border-style" => BorderComponent::Style,
        "border-color" => BorderComponent::Color,
        _ => unreachable!(),
    };
    let component_index = match &block.declarations[border] {
        Declaration::Unparsed(value) => border_component_index(&value.value, component),
        _ => None,
    };
    let Some(component_index) = component_index else {
        return;
    };
    let target = *present.iter().max().expect("three overrides");
    let (border_declaration, target_declaration) = if border < target {
        let (before, after) = block.declarations.split_at_mut(target);
        (&mut before[border], &mut after[0])
    } else {
        return;
    };
    let (Declaration::Unparsed(border_value), Declaration::Unparsed(target_value)) =
        (border_declaration, target_declaration)
    else {
        return;
    };
    std::mem::swap(
        &mut border_value.value[component_index],
        &mut target_value.value[0],
    );
    let missing = indices
        .iter()
        .position(Option::is_none)
        .expect("exactly one missing side");
    *target_value.property_id = PropertyId::from_name(names[missing]);
    for index in present {
        if index != target {
            block.mark_invalid(index);
            cx.record_declaration_removed();
        }
    }
    minify_border_declaration(block, border, cx);
}

fn promote_two_equal_border_sides(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let names = ["border-top", "border-right", "border-bottom", "border-left"];
    let Some(border) = (0..block.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == "border"
    }) else {
        return;
    };
    let mut indices = [None; 4];
    for index in (border + 1)..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        if let Some(side) = names
            .iter()
            .position(|name| block.declarations[index].name() == *name)
        {
            indices[side] = Some(index);
        }
    }
    if indices.iter().flatten().count() != 2 {
        return;
    }
    let mut present = [0; 2];
    let mut missing = [0; 2];
    let mut present_count = 0;
    let mut missing_count = 0;
    for (side, index) in indices.into_iter().enumerate() {
        if let Some(index) = index {
            present[present_count] = index;
            present_count += 1;
        } else {
            missing[missing_count] = side;
            missing_count += 1;
        }
    }
    debug_assert_eq!(present_count, 2);
    debug_assert_eq!(missing_count, 2);
    let can_promote = matches!(&block.declarations[border], Declaration::Unparsed(value)
        if value.value.len() == 1
            && clone_simple_token_or_value(&value.value[0], block.declarations.bump()).is_some()
            && !value.value.iter().any(token_or_value_contains_variable))
        && matches!(
            (&block.declarations[present[0]], &block.declarations[present[1]]),
            (Declaration::Unparsed(first), Declaration::Unparsed(second))
                if first.value == second.value
                    && !first.value.iter().any(token_or_value_contains_variable)
        );
    if !can_promote {
        return;
    }
    let allocator = block.declarations.bump();
    let old_border = match &mut block.declarations[border] {
        Declaration::Unparsed(value) => std::mem::replace(&mut value.value, allocator.vec()),
        _ => return,
    };
    let fallback = clone_simple_token_or_value(&old_border[0], allocator)
        .expect("the existing border was validated as cloneable");
    let promoted = match &mut block.declarations[present[0]] {
        Declaration::Unparsed(value) => std::mem::replace(&mut value.value, allocator.vec()),
        _ => return,
    };
    let Declaration::Unparsed(border_value) = &mut block.declarations[border] else {
        return;
    };
    border_value.value = promoted;
    let Declaration::Unparsed(first_exception) = &mut block.declarations[present[0]] else {
        return;
    };
    *first_exception.property_id = PropertyId::from_name(names[missing[0]]);
    first_exception.value = old_border;
    let Declaration::Unparsed(second_exception) = &mut block.declarations[present[1]] else {
        return;
    };
    *second_exception.property_id = PropertyId::from_name(names[missing[1]]);
    second_exception.value.clear();
    second_exception.value.push(fallback);
    minify_border_declaration(block, border, cx);
    minify_unparsed_declaration(block, present[0], cx);
    minify_unparsed_declaration(block, present[1], cx);
}

fn merge_common_border_sides(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let names = ["border-top", "border-right", "border-bottom", "border-left"];
    let Some(indices) = declaration_indices(block, important, names, "border") else {
        return;
    };
    let mut base_side = None;
    for candidate in 0..4 {
        let count = (0..4)
            .filter(|&side| {
                match (
                    &block.declarations[indices[candidate]],
                    &block.declarations[indices[side]],
                ) {
                    (Declaration::Unparsed(candidate), Declaration::Unparsed(value)) => {
                        candidate.value == value.value
                    }
                    _ => false,
                }
            })
            .count();
        let mut exceptions = [usize::MAX; 2];
        let mut exception_count = 0;
        if count == 2 {
            for side in 0..4 {
                let equal = match (
                    &block.declarations[indices[candidate]],
                    &block.declarations[indices[side]],
                ) {
                    (Declaration::Unparsed(candidate), Declaration::Unparsed(value)) => {
                        candidate.value == value.value
                    }
                    _ => false,
                };
                if !equal {
                    exceptions[exception_count] = side;
                    exception_count += 1;
                }
            }
        }
        let unique_pair = count == 2
            && exception_count == 2
            && block.declarations[indices[exceptions[0]]]
                != block.declarations[indices[exceptions[1]]];
        if count >= 3 || unique_pair {
            base_side = Some(candidate);
            break;
        }
    }
    let Some(base_side) = base_side else {
        return;
    };
    let target = *indices.iter().min().expect("four border sides");
    let source = indices[base_side];
    if source != target {
        block.declarations.swap(source, target);
    }
    let Declaration::Unparsed(target_value) = &mut block.declarations[target] else {
        return;
    };
    *target_value.property_id = PropertyId::from_name("border");
    let mut remove = [false; 4];
    for (side, &index) in indices.iter().enumerate() {
        if index == target {
            continue;
        }
        remove[side] = match (&block.declarations[target], &block.declarations[index]) {
            (Declaration::Unparsed(target), Declaration::Unparsed(value)) => {
                target.value == value.value
            }
            _ => false,
        };
    }
    for (side, remove) in remove.into_iter().enumerate() {
        if remove {
            block.mark_invalid(indices[side]);
            cx.record_declaration_removed();
        }
    }
    minify_border_declaration(block, target, cx);
}

fn canonicalize_border_side_declaration_order(block: &mut DeclarationBlock<'_>, important: bool) {
    if !(0..block.len()).any(|index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == "border"
    }) {
        return;
    }
    let desired = ["border-top", "border-right", "border-bottom", "border-left"];
    let mut slots = [usize::MAX; 4];
    let mut slot_count = 0;
    for index in 0..block.len() {
        if !block.is_invalid(index)
            && block.is_important(index) == important
            && desired.contains(&block.declarations[index].name())
        {
            slots[slot_count] = index;
            slot_count += 1;
        }
    }
    slots[..slot_count].sort_unstable();
    let mut slot = 0;
    for name in desired {
        let Some(current) = (0..block.len()).find(|&index| {
            !block.is_invalid(index)
                && block.is_important(index) == important
                && block.declarations[index].name() == name
        }) else {
            continue;
        };
        let target = slots[slot];
        slot += 1;
        if target != current {
            block.declarations.swap(target, current);
        }
    }
}

fn move_border_side_before_earlier_color(
    block: &mut DeclarationBlock<'_>,
    target: usize,
    color_name: &str,
    important: bool,
) {
    let Some(color) = (0..target).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && matches!(block.declarations[index].name(), name if name == color_name || name == "border-color")
    }) else {
        return;
    };
    block.declarations.swap(color, target);
}

fn canonicalize_border_component_declaration_order(
    block: &mut DeclarationBlock<'_>,
    important: bool,
) {
    let desired = ["border-color", "border-style", "border-width"];
    let mut positions = [None; 3];
    for index in 0..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        if let Some(component) = desired
            .iter()
            .position(|name| block.declarations[index].name() == *name)
        {
            positions[component] = Some(index);
        }
    }
    let [Some(color), Some(style), Some(width)] = positions else {
        return;
    };
    if [color, style, width]
        .into_iter()
        .any(|index| declaration_contains_variable(&block.declarations[index]))
        || !matches!(&block.declarations[width], Declaration::Unparsed(value)
        if value.value.iter().filter(|item| !is_token_whitespace(item)).count() > 1)
    {
        return;
    }
    let mut slots = [color, style, width];
    slots.sort_unstable();
    for (slot, name) in slots.into_iter().zip(desired) {
        let Some(current) = (0..block.len()).find(|&index| {
            !block.is_invalid(index)
                && block.is_important(index) == important
                && block.declarations[index].name() == name
        }) else {
            return;
        };
        if slot != current {
            block.declarations.swap(slot, current);
        }
    }
}

fn factor_border_side_component_overrides<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    names: [&str; 4],
    shorthand: &'a str,
    cx: &mut MinifyContext,
) {
    let allocator = block.declarations.bump();
    let mut border = None;
    for index in 0..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        let name = block.declarations[index].name();
        if name == "border" {
            border = Some(index);
            continue;
        }
        if name == shorthand {
            border = None;
            continue;
        }
        let Some(side) = names.iter().position(|candidate| name == *candidate) else {
            continue;
        };
        let Some(border_index) = border.take() else {
            continue;
        };
        let component = match shorthand {
            "border-width" => BorderComponent::Width,
            "border-style" => BorderComponent::Style,
            "border-color" => BorderComponent::Color,
            _ => unreachable!(),
        };
        if component == BorderComponent::Style
            && fold_none_border_style_side_override(block, border_index, index, side, cx)
        {
            continue;
        }
        let next_barrier = ((border_index + 1)..block.len())
            .find(|&candidate| {
                !block.is_invalid(candidate)
                    && block.is_important(candidate) == important
                    && matches!(block.declarations[candidate].name(), "border")
            })
            .unwrap_or(block.len());
        let override_count = ((border_index + 1)..next_barrier)
            .filter(|&candidate| {
                !block.is_invalid(candidate)
                    && block.is_important(candidate) == important
                    && names.contains(&block.declarations[candidate].name())
            })
            .count();
        let has_variable_barrier = ((border_index + 1)..=index)
            .any(|candidate| declaration_contains_variable(&block.declarations[candidate]));
        let has_component_barrier = ((border_index + 1)..index).any(|candidate| {
            matches!(
                block.declarations[candidate].name(),
                "border-width" | "border-style" | "border-color"
            )
        });
        let can_factor = override_count == 1
            && !has_variable_barrier
            && !has_component_barrier
            && (side != 3
                || matches!(
                    (&block.declarations[border_index], component),
                    (Declaration::Unparsed(value), BorderComponent::Width)
                        if border_component_index(&value.value, BorderComponent::Width)
                            .is_some_and(|index| is_zero_border_width(&value.value[index]))
                ))
            && matches!(&block.declarations[index], Declaration::Unparsed(value)
            if value.value.len() == 1
                && is_simple_box_value(&value.value[0])
                && !token_ident(&value.value[0])
                    .is_some_and(|keyword| keyword.eq_ignore_ascii_case("currentcolor"))
                && !value.value.iter().any(token_or_value_contains_variable))
            && matches!(&block.declarations[border_index], Declaration::Unparsed(value)
            if value.value.iter().filter(|item| !is_token_whitespace(item)).count() > 1
                && !value.value.iter().any(token_or_value_contains_variable)
                && border_component_index(&value.value, component)
                    .is_some_and(|component_index| {
                        !token_ident(&value.value[component_index])
                            .is_some_and(|keyword| keyword.eq_ignore_ascii_case("transparent"))
                            && clone_simple_token_or_value(
                                &value.value[component_index], allocator
                            ).is_some()
                    }));
        if !can_factor {
            border = Some(border_index);
            continue;
        }
        let Some(base) = take_border_component(block, border_index, component) else {
            border = Some(border_index);
            continue;
        };
        let Some(first_clone) = clone_simple_token_or_value(&base, allocator) else {
            unreachable!("the component was validated as cloneable")
        };
        let Some(second_clone) = clone_simple_token_or_value(&base, allocator) else {
            unreachable!("the component was validated as cloneable")
        };
        let Some(third_clone) = clone_simple_token_or_value(&base, allocator) else {
            unreachable!("the component was validated as cloneable")
        };
        let Declaration::Unparsed(override_value) = &mut block.declarations[index] else {
            unreachable!("the override was validated as unparsed")
        };
        let mut override_part = std::mem::replace(&mut override_value.value, allocator.vec());
        let override_component = override_part.remove(0);
        let mut values = allocator.vec();
        values.push(base);
        values.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        values.push(first_clone);
        values.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        values.push(second_clone);
        values.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        values.push(third_clone);
        values[side * 2] = override_component;
        *override_value.property_id = PropertyId::from_name(shorthand);
        override_value.value = values;
        if shorthand == "border-style" {
            canonicalize_full_border_keywords(&mut override_value.value);
        }
        minify_unparsed_declaration(block, index, cx);
        minify_border_declaration(block, border_index, cx);
    }
}

fn is_zero_border_width(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Length(value) => value.value == 0.0,
        TokenOrValue::Token(token) => matches!(
            &**token,
            Token::Number(0.0) | Token::Dimension { value: 0.0, .. }
        ),
        _ => false,
    }
}

fn fold_none_border_style_side_override(
    block: &mut DeclarationBlock<'_>,
    border_index: usize,
    override_index: usize,
    side: usize,
    cx: &mut MinifyContext,
) -> bool {
    let allocator = block.declarations.bump();
    let width = match &block.declarations[border_index] {
        Declaration::Unparsed(value)
            if border_component_index(&value.value, BorderComponent::Color).is_none()
                && border_component_index(&value.value, BorderComponent::Style).is_some_and(
                    |index| {
                        token_ident(&value.value[index])
                            .is_some_and(|keyword| keyword.eq_ignore_ascii_case("none"))
                    },
                ) =>
        {
            border_component_index(&value.value, BorderComponent::Width)
                .and_then(|index| clone_simple_token_or_value(&value.value[index], allocator))
        }
        _ => None,
    };
    let Some(width) = width else {
        return false;
    };
    let can_fold = matches!(&block.declarations[override_index], Declaration::Unparsed(value)
        if value.value.len() == 1
            && is_simple_box_value(&value.value[0])
            && !is_css_wide_value(&value.value[0]));
    if !can_fold {
        return false;
    }
    if take_border_component(block, border_index, BorderComponent::Style).is_none() {
        return false;
    }
    let Declaration::Unparsed(override_value) = &mut block.declarations[override_index] else {
        return false;
    };
    let mut old = std::mem::replace(&mut override_value.value, allocator.vec());
    let style = old.remove(0);
    override_value.value.push(width);
    override_value
        .value
        .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
    override_value.value.push(style);
    *override_value.property_id =
        PropertyId::from_name(border_side_name(["top", "right", "bottom", "left"][side]));
    canonicalize_full_border_keywords(&mut override_value.value);
    minify_border_declaration(block, border_index, cx);
    minify_unparsed_declaration(block, override_index, cx);
    true
}

fn take_border_component<'a>(
    block: &mut DeclarationBlock<'a>,
    border_index: usize,
    component: BorderComponent,
) -> Option<TokenOrValue<'a>> {
    let Declaration::Unparsed(border) = &mut block.declarations[border_index] else {
        return None;
    };
    let index = border_component_index(&border.value, component)?;
    let component = border.value.remove(index);
    if index > 0 && is_token_whitespace(&border.value[index - 1]) {
        border.value.remove(index - 1);
    } else if border.value.get(index).is_some_and(is_token_whitespace) {
        border.value.remove(index);
    }
    Some(component)
}

fn fold_border_shorthand_component_overrides(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let mut border = None;
    for index in 0..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        let name = block.declarations[index].name();
        if name == "border" {
            border = Some(index);
            continue;
        }
        let component = match name {
            "border-width" => BorderComponent::Width,
            "border-style" => BorderComponent::Style,
            "border-color" => BorderComponent::Color,
            _ => continue,
        };
        let Some(border_index) = border else {
            continue;
        };
        let Declaration::Unparsed(override_value) = &block.declarations[index] else {
            continue;
        };
        if override_value
            .value
            .iter()
            .any(token_or_value_contains_variable)
        {
            continue;
        }
        let item_count = override_value
            .value
            .iter()
            .filter(|value| !is_token_whitespace(value))
            .count();
        let single_is_css_wide = item_count == 1
            && override_value
                .value
                .iter()
                .find(|value| !is_token_whitespace(value))
                .is_some_and(is_css_wide_value);
        if item_count != 1 || single_is_css_wide {
            if remove_border_component(block, border_index, component) {
                minify_border_declaration(block, border_index, cx);
            }
            continue;
        }
        if fold_single_border_component(block, border_index, index, component) {
            block.mark_invalid(index);
            cx.record_declaration_removed();
            minify_border_declaration(block, border_index, cx);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BorderComponent {
    Width,
    Style,
    Color,
}

fn fold_single_border_component<'a>(
    block: &mut DeclarationBlock<'a>,
    border_index: usize,
    override_index: usize,
    component: BorderComponent,
) -> bool {
    let allocator = block.declarations.bump();
    let border_component = match &block.declarations[border_index] {
        Declaration::Unparsed(value) => border_component_index(&value.value, component),
        _ => None,
    };
    let override_component = match &block.declarations[override_index] {
        Declaration::Unparsed(value) => value
            .value
            .iter()
            .position(|value| !is_token_whitespace(value)),
        _ => None,
    };
    let Some(override_component) = override_component else {
        return false;
    };
    let (border_declaration, override_declaration) = if border_index < override_index {
        let (before, after) = block.declarations.split_at_mut(override_index);
        (&mut before[border_index], &mut after[0])
    } else {
        let (before, after) = block.declarations.split_at_mut(border_index);
        (&mut after[0], &mut before[override_index])
    };
    let (Declaration::Unparsed(border), Declaration::Unparsed(override_value)) =
        (border_declaration, override_declaration)
    else {
        return false;
    };
    let target = border_component.unwrap_or_else(|| {
        border
            .value
            .iter()
            .position(|value| !is_token_whitespace(value))
            .expect("border declaration is non-empty")
    });
    if border_component.is_none() {
        if component == BorderComponent::Color
            && matches!(&border.value[target], TokenOrValue::Token(token)
                if matches!(&**token, Token::Ident(value) if value.eq_ignore_ascii_case("none")))
        {
            std::mem::swap(
                &mut border.value[target],
                &mut override_value.value[override_component],
            );
            return true;
        }
        let replacement = override_value.value.remove(override_component);
        border
            .value
            .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        border.value.push(replacement);
        return true;
    }
    std::mem::swap(
        &mut border.value[target],
        &mut override_value.value[override_component],
    );
    true
}

fn remove_border_component(
    block: &mut DeclarationBlock<'_>,
    border_index: usize,
    component: BorderComponent,
) -> bool {
    let Declaration::Unparsed(border) = &mut block.declarations[border_index] else {
        return false;
    };
    let Some(index) = border_component_index(&border.value, component) else {
        return false;
    };
    let start = if index > 0 && is_token_whitespace(&border.value[index - 1]) {
        index - 1
    } else {
        index
    };
    let end = if start == index && border.value.get(index + 1).is_some_and(is_token_whitespace) {
        index + 1
    } else {
        index
    };
    border.value.drain(start..=end);
    true
}

fn border_component_index(values: &[TokenOrValue<'_>], expected: BorderComponent) -> Option<usize> {
    values.iter().enumerate().find_map(|(index, value)| {
        (!is_token_whitespace(value) && border_component(value) == Some(expected)).then_some(index)
    })
}

fn border_component(value: &TokenOrValue<'_>) -> Option<BorderComponent> {
    match value {
        TokenOrValue::Length(_) => Some(BorderComponent::Width),
        TokenOrValue::Color(_) | TokenOrValue::UnresolvedColor(_) => Some(BorderComponent::Color),
        TokenOrValue::Function(function)
            if ["rgb", "rgba", "hsl", "hsla", "hwb", "lab", "lch", "color"]
                .iter()
                .any(|name| function.name.eq_ignore_ascii_case(name)) =>
        {
            Some(BorderComponent::Color)
        }
        TokenOrValue::Token(token) => match &**token {
            Token::Number(_) | Token::Dimension { .. } => Some(BorderComponent::Width),
            Token::Ident(value)
                if ["thin", "medium", "thick"]
                    .iter()
                    .any(|keyword| value.eq_ignore_ascii_case(keyword)) =>
            {
                Some(BorderComponent::Width)
            }
            Token::Ident(value)
                if [
                    "none", "hidden", "dotted", "dashed", "solid", "double", "groove", "ridge",
                    "inset", "outset",
                ]
                .iter()
                .any(|keyword| value.eq_ignore_ascii_case(keyword)) =>
            {
                Some(BorderComponent::Style)
            }
            Token::Ident(_) | Token::Hash(_) | Token::IdHash(_) | Token::MinifiedHash(_) => {
                Some(BorderComponent::Color)
            }
            _ => None,
        },
        _ => None,
    }
}

fn is_token_whitespace(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
}

fn is_css_wide_value(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token)
        if matches!(&**token, Token::Ident(value)
            if ["initial", "inherit", "unset", "revert", "revert-layer"]
                .iter()
                .any(|keyword| value.eq_ignore_ascii_case(keyword))))
}

fn minify_border_declaration(
    block: &mut DeclarationBlock<'_>,
    index: usize,
    cx: &mut MinifyContext,
) {
    let Declaration::Unparsed(value) = &mut block.declarations[index] else {
        return;
    };
    let previous = cx.value_context;
    cx.value_context = crate::properties::value_context(
        &value.property_id,
        cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
        cx.is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
    );
    canonicalize_border_keywords(&mut value.value);
    canonicalize_full_border_keywords(&mut value.value);
    value.value.minify(cx);
    cx.value_context = previous;
}

fn fold_border_component_overrides(
    block: &mut DeclarationBlock<'_>,
    important: bool,
    names: [&str; 4],
    shorthand: &str,
    cx: &mut MinifyContext,
) {
    let mut shorthand_index = None;
    for index in 0..block.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        let name = block.declarations[index].name();
        if name == shorthand {
            shorthand_index = Some(index);
            continue;
        }
        let Some(side) = names.iter().position(|candidate| name == *candidate) else {
            continue;
        };
        let Some(target) = shorthand_index else {
            continue;
        };
        if fold_box_side_value(block, target, index, side, true) {
            block.mark_invalid(index);
            cx.record_declaration_removed();
            if shorthand == "border-width"
                && let Declaration::Unparsed(value) = &mut block.declarations[target]
            {
                for item in &mut value.value {
                    if let TokenOrValue::Token(token) = item
                        && matches!(&**token, Token::Ident(keyword) if keyword.eq_ignore_ascii_case("none"))
                    {
                        **token = Token::Number(0.0);
                    }
                }
            }
            minify_unparsed_declaration(block, target, cx);
        }
    }
}

fn minify_unparsed_declaration(
    block: &mut DeclarationBlock<'_>,
    index: usize,
    cx: &mut MinifyContext,
) {
    let Declaration::Unparsed(value) = &mut block.declarations[index] else {
        return;
    };
    let previous = cx.value_context;
    cx.value_context = crate::properties::value_context(
        &value.property_id,
        cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
        cx.is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
    );
    value.value.minify(cx);
    cx.value_context = previous;
}

fn fold_box_side_value(
    block: &mut DeclarationBlock<'_>,
    shorthand: usize,
    longhand: usize,
    side: usize,
    expand_shared: bool,
) -> bool {
    let allocator = block.declarations.bump();
    let (shorthand_declaration, longhand_declaration) = if shorthand < longhand {
        let (before, after) = block.declarations.split_at_mut(longhand);
        (&mut before[shorthand], &mut after[0])
    } else {
        let (before, after) = block.declarations.split_at_mut(shorthand);
        (&mut after[0], &mut before[longhand])
    };
    let (Declaration::Unparsed(shorthand_value), Declaration::Unparsed(longhand_value)) =
        (shorthand_declaration, longhand_declaration)
    else {
        return false;
    };
    if longhand_value.value.len() != 1
        || is_css_wide_value(&longhand_value.value[0])
        || (expand_shared && !is_simple_box_value(&longhand_value.value[0]))
        || longhand_value
            .value
            .iter()
            .any(token_or_value_contains_variable)
        || shorthand_value
            .value
            .iter()
            .any(token_or_value_contains_variable)
    {
        return false;
    }
    let component_count = shorthand_value.value.len().div_ceil(2);
    if !(1..=4).contains(&component_count)
        || shorthand_value.value.len() != component_count * 2 - 1
        || shorthand_value.value.iter().enumerate().any(|(index, value)| {
            index % 2 == 1
                && !matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::WhiteSpace(_)))
        })
    {
        return false;
    }
    let component = match (component_count, side) {
        (1, _) => 0,
        (2, 0 | 2) => 0,
        (2, 1 | 3) => 1,
        (3, 0) => 0,
        (3, 1 | 3) => 1,
        (3, 2) => 2,
        (4, side) => side,
        _ => unreachable!(),
    };
    let value_index = component * 2;
    if shorthand_value.value[value_index] == longhand_value.value[0] {
        return true;
    }
    let component_is_unique = match component_count {
        1 | 2 => false,
        3 => side == 0 || side == 2,
        4 => true,
        _ => unreachable!(),
    };
    if !component_is_unique && !expand_shared {
        return false;
    }
    let mut target_index = value_index;
    if component_count < 4 && !component_is_unique {
        let additions = match component_count {
            1 => [
                clone_simple_token_or_value(&shorthand_value.value[0], allocator),
                clone_simple_token_or_value(&shorthand_value.value[0], allocator),
                clone_simple_token_or_value(&shorthand_value.value[0], allocator),
            ],
            2 => [
                clone_simple_token_or_value(&shorthand_value.value[0], allocator),
                clone_simple_token_or_value(&shorthand_value.value[2], allocator),
                None,
            ],
            3 => [
                clone_simple_token_or_value(&shorthand_value.value[2], allocator),
                None,
                None,
            ],
            _ => unreachable!(),
        };
        if additions.iter().flatten().count() != 4 - component_count {
            return false;
        }
        for addition in additions.into_iter().flatten() {
            shorthand_value
                .value
                .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
            shorthand_value.value.push(addition);
        }
        debug_assert_eq!(shorthand_value.value.len(), 7);
        target_index = side * 2;
    }
    std::mem::swap(
        &mut shorthand_value.value[target_index],
        &mut longhand_value.value[0],
    );
    true
}

fn clone_simple_token_or_value<'a>(
    value: &TokenOrValue<'a>,
    allocator: &'a Allocator,
) -> Option<TokenOrValue<'a>> {
    match value {
        TokenOrValue::Token(token)
            if matches!(
                &**token,
                Token::Ident(_)
                    | Token::Hash(_)
                    | Token::IdHash(_)
                    | Token::MinifiedHash(_)
                    | Token::Number(_)
                    | Token::Percentage(_)
                    | Token::Dimension { .. }
            ) =>
        {
            Some(TokenOrValue::Token(allocator.boxed((**token).clone())))
        }
        TokenOrValue::Length(length) => Some(TokenOrValue::Length(allocator.boxed(LengthValue {
            unit: length.unit,
            value: length.value,
        }))),
        TokenOrValue::Color(color) => match &**color {
            CssColor::Rgba(value) => {
                Some(TokenOrValue::Color(allocator.boxed(CssColor::Rgba(*value))))
            }
            _ => None,
        },
        TokenOrValue::DashedIdent(value) => Some(TokenOrValue::DashedIdent(value)),
        _ => None,
    }
}

fn is_simple_box_value(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Length(_) | TokenOrValue::DashedIdent(_) => true,
        TokenOrValue::Color(color) => matches!(&**color, CssColor::Rgba(_)),
        TokenOrValue::Token(token) => matches!(
            &**token,
            Token::Ident(_)
                | Token::Hash(_)
                | Token::IdHash(_)
                | Token::MinifiedHash(_)
                | Token::Number(_)
                | Token::Percentage(_)
                | Token::Dimension { .. }
        ),
        _ => false,
    }
}

fn merge_unparsed_border_component_groups<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    names: [&str; 4],
    shorthand: &'a str,
    cx: &mut MinifyContext,
) {
    let allocator = block.declarations.bump();
    let last_shorthand = (0..block.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == shorthand
    });
    let start = last_shorthand.map_or(0, |index| index + 1);
    let mut candidates = BitVec::new(allocator);
    for index in 0..block.len() {
        candidates.push(
            index >= start
                && !block.is_invalid(index)
                && block.is_important(index) == important
                && names.contains(&block.declarations[index].name()),
        );
    }

    while let Some(last) = (0..block.len())
        .rev()
        .find(|&index| candidates.is_set(index))
    {
        let mut indices = [None; 4];
        for index in start..=last {
            if !candidates.is_set(index) {
                continue;
            }
            if let Some(side) = names
                .iter()
                .position(|name| block.declarations[index].name() == *name)
            {
                indices[side] = Some(index);
            }
        }
        let [Some(top), Some(right), Some(bottom), Some(left)] = indices else {
            candidates.set(last, false);
            continue;
        };
        let indices = [top, right, bottom, left];
        if merge_unparsed_values(block, indices, shorthand, true, cx) {
            for index in indices {
                candidates.set(index, false);
            }
        } else {
            candidates.set(last, false);
        }
    }
}

fn merge_border_side_shorthands<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let Some(indices) = declaration_indices(
        block,
        important,
        ["border-top", "border-right", "border-bottom", "border-left"],
        "border",
    ) else {
        return;
    };
    if indices.iter().any(|&index| {
        !matches!(&block.declarations[index], Declaration::Unparsed(value)
            if matches!(value.value.as_slice(),
                [_, TokenOrValue::Token(first_space), _, TokenOrValue::Token(second_space), _]
                    if matches!(**first_space, Token::WhiteSpace(_))
                        && matches!(**second_space, Token::WhiteSpace(_))))
    }) {
        return;
    }

    let allocator = block.declarations.bump();
    let [top, right, bottom, left] = indices.map(|index| {
        let Declaration::Unparsed(value) = &mut block.declarations[index] else {
            unreachable!()
        };
        let mut values = std::mem::replace(&mut value.value, allocator.vec());
        let width = values.remove(0);
        let first_space = values.remove(0);
        let style = values.remove(0);
        let second_space = values.remove(0);
        let color = values.remove(0);
        (values, width, first_space, style, second_space, color)
    });
    let (mut width_values, top_width, top_first_space, top_style, top_second_space, top_color) =
        top;
    let (
        mut style_values,
        right_width,
        right_first_space,
        right_style,
        right_second_space,
        right_color,
    ) = right;
    let (
        mut color_values,
        bottom_width,
        bottom_first_space,
        bottom_style,
        bottom_second_space,
        bottom_color,
    ) = bottom;
    let (_unused_values, left_width, left_first_space, left_style, left_second_space, left_color) =
        left;

    width_values.push(top_width);
    width_values.push(top_first_space);
    width_values.push(right_width);
    width_values.push(right_first_space);
    width_values.push(bottom_width);
    width_values.push(bottom_first_space);
    width_values.push(left_width);

    style_values.push(top_style);
    style_values.push(top_second_space);
    style_values.push(right_style);
    style_values.push(right_second_space);
    style_values.push(bottom_style);
    style_values.push(bottom_second_space);
    style_values.push(left_style);

    color_values.push(top_color);
    color_values.push(left_first_space);
    color_values.push(right_color);
    color_values.push(left_second_space);
    color_values.push(bottom_color);
    color_values.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
    color_values.push(left_color);

    let mut targets = indices;
    targets.sort_unstable();
    for (target, name, values) in [
        (targets[0], "border-color", color_values),
        (targets[1], "border-style", style_values),
        (targets[2], "border-width", width_values),
    ] {
        let Declaration::Unparsed(target_value) = &mut block.declarations[target] else {
            unreachable!()
        };
        *target_value.property_id = PropertyId::from_name(name);
        target_value.value = values;
        let previous = cx.value_context;
        cx.value_context = crate::properties::value_context(
            &target_value.property_id,
            cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
            cx.is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
        );
        target_value.value.minify(cx);
        cx.value_context = previous;
    }
    block.mark_invalid(targets[3]);
    cx.record_declaration_removed();
}

fn can_merge_without_color(
    block: &DeclarationBlock<'_>,
    color_name: &str,
    important: bool,
    component_indices: &[usize],
) -> bool {
    let target = *component_indices.iter().max().expect("components");
    (0..block.declarations.len()).all(|index| {
        block.is_invalid(index)
            || block.declarations[index].name() != color_name
            || (block.is_important(index) == important && index > target)
    })
}

fn canonicalize_standalone_border_style(block: &mut DeclarationBlock<'_>) {
    if block
        .declarations
        .iter()
        .enumerate()
        .any(|(index, declaration)| {
            !block.is_invalid(index)
                && declaration
                    .name()
                    .strip_prefix('_')
                    .and_then(|name| name.get(.."border-".len()))
                    .is_some_and(|name| name.eq_ignore_ascii_case("border-"))
        })
    {
        return;
    }
    for index in 0..block.declarations.len() {
        if block.is_invalid(index) || block.declarations[index].name() != "border-style" {
            continue;
        }
        let important = block.is_important(index);
        let mut has_related_component = false;
        let mut has_different_importance = false;
        for related in 0..block.declarations.len() {
            if block.is_invalid(related)
                || !matches!(
                    block.declarations[related].name(),
                    "border-width" | "border-color"
                )
            {
                continue;
            }
            has_related_component = true;
            has_different_importance |= block.is_important(related) != important;
        }
        if !has_related_component || has_different_importance {
            continue;
        }
        if let Declaration::Unparsed(value) = &mut block.declarations[index] {
            canonicalize_full_border_keywords(&mut value.value);
        }
    }
}

fn border_name(side: &str, component: &str) -> &'static str {
    match (side, component) {
        ("top", "width") => "border-top-width",
        ("right", "width") => "border-right-width",
        ("bottom", "width") => "border-bottom-width",
        ("left", "width") => "border-left-width",
        ("top", "style") => "border-top-style",
        ("right", "style") => "border-right-style",
        ("bottom", "style") => "border-bottom-style",
        ("left", "style") => "border-left-style",
        ("top", "color") => "border-top-color",
        ("right", "color") => "border-right-color",
        ("bottom", "color") => "border-bottom-color",
        ("left", "color") => "border-left-color",
        _ => unreachable!(),
    }
}

fn border_side_name(side: &str) -> &'static str {
    match side {
        "top" => "border-top",
        "right" => "border-right",
        "bottom" => "border-bottom",
        "left" => "border-left",
        _ => unreachable!(),
    }
}

fn declaration_indices<const N: usize>(
    block: &DeclarationBlock<'_>,
    important: bool,
    names: [&str; N],
    shorthand: &str,
) -> Option<[usize; N]> {
    let last_shorthand = (0..block.declarations.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == shorthand
    });
    let mut indices = [None; N];
    for index in last_shorthand.map_or(0, |index| index + 1)..block.declarations.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        if let Some(position) = names
            .iter()
            .position(|name| block.declarations[index].name() == *name)
        {
            indices[position] = Some(index);
        }
    }
    let mut output = [0; N];
    for (target, index) in output.iter_mut().zip(indices) {
        *target = index?;
    }
    Some(output)
}

fn merge_unparsed_values<'a, const N: usize>(
    block: &mut DeclarationBlock<'a>,
    indices: [usize; N],
    shorthand: &'a str,
    minify_as_box: bool,
    cx: &mut MinifyContext,
) -> bool {
    if indices.iter().any(|&index| {
        !matches!(&block.declarations[index], Declaration::Unparsed(value) if value.value.len() == 1)
    }) {
        return false;
    }
    let all_equal = indices.windows(2).all(|pair| {
        let (Declaration::Unparsed(left), Declaration::Unparsed(right)) =
            (&block.declarations[pair[0]], &block.declarations[pair[1]])
        else {
            unreachable!()
        };
        left.value == right.value
    });
    let has_complex_value = indices.iter().any(|&index| {
        matches!(&block.declarations[index], Declaration::Unparsed(value)
            if matches!(value.value.as_slice(), [TokenOrValue::Var(_) | TokenOrValue::Env(_) | TokenOrValue::Function(_)]))
    });
    if has_complex_value && !all_equal {
        return false;
    }
    let css_wide = indices.iter().any(|&index| {
        matches!(&block.declarations[index], Declaration::Unparsed(value)
            if matches!(value.value.as_slice(), [TokenOrValue::Token(token)]
                if matches!(&**token, Token::Ident(value)
                    if ["initial", "inherit", "unset", "revert", "revert-layer"]
                        .iter().any(|keyword| value.eq_ignore_ascii_case(keyword)))))
    });
    if css_wide && !all_equal {
        return false;
    }

    let allocator = block.declarations.bump();
    let mut parts = indices.map(|index| {
        let Declaration::Unparsed(value) = &mut block.declarations[index] else {
            unreachable!()
        };
        std::mem::replace(&mut value.value, allocator.vec())
    });
    let mut value = std::mem::replace(&mut parts[0], allocator.vec());
    if !(css_wide && all_equal) {
        for part in &mut parts[1..] {
            value.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
            value.append(part);
        }
    }
    let target = *indices.iter().max().expect("non-empty declaration group");
    let Declaration::Unparsed(target_value) = &mut block.declarations[target] else {
        unreachable!()
    };
    *target_value.property_id = PropertyId::from_name(shorthand);
    target_value.value = value;
    if matches!(
        shorthand,
        "border" | "border-top" | "border-right" | "border-bottom" | "border-left"
    ) {
        canonicalize_full_border_keywords(&mut target_value.value);
    }
    if minify_as_box {
        let previous = cx.value_context;
        cx.value_context = crate::properties::value_context(
            &target_value.property_id,
            cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
            cx.is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
        );
        target_value.value.minify(cx);
        cx.value_context = previous;
    }
    mark_merged_indices(block, &indices, target, cx);
    true
}

fn canonicalize_full_border_keywords(value: &mut Vec<'_, TokenOrValue<'_>>) {
    for value in value {
        let TokenOrValue::Token(token) = value else {
            continue;
        };
        let Token::Ident(keyword) = &**token else {
            continue;
        };
        let canonical = [
            "none", "hidden", "inset", "groove", "outset", "ridge", "dotted", "dashed", "solid",
            "double", "thin", "medium", "thick",
        ]
        .into_iter()
        .find(|candidate| keyword.eq_ignore_ascii_case(candidate))
        .or_else(|| canonical_border_keyword(keyword));
        if let Some(canonical) = canonical {
            **token = Token::Ident(canonical);
        }
    }
}

fn merge_equal_unparsed_values<'a, const N: usize>(
    block: &mut DeclarationBlock<'a>,
    indices: [usize; N],
    shorthand: &'a str,
    cx: &mut MinifyContext,
) -> bool {
    let mut values = indices
        .iter()
        .filter_map(|&index| match &block.declarations[index] {
            Declaration::Unparsed(value) => Some(&value.value),
            _ => None,
        });
    let Some(first) = values.next() else {
        return false;
    };
    if !values.all(|value| value == first) {
        return false;
    }
    let target = *indices.iter().max().expect("non-empty declaration group");
    let Declaration::Unparsed(target_value) = &mut block.declarations[target] else {
        return false;
    };
    *target_value.property_id = PropertyId::from_name(shorthand);
    mark_merged_indices(block, &indices, target, cx);
    true
}

fn mark_merged_indices(
    block: &mut DeclarationBlock<'_>,
    indices: &[usize],
    target: usize,
    cx: &mut MinifyContext,
) {
    for &index in indices {
        if index != target {
            block.mark_invalid(index);
            cx.record_declaration_removed();
        }
    }
}

fn merge_margin_longhands<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let Some([top, right, bottom, left]) = box_longhand_indices(
        block,
        important,
        ["margin-top", "margin-right", "margin-bottom", "margin-left"],
        "margin",
    ) else {
        return;
    };
    if merge_unparsed_box_longhands(block, [top, right, bottom, left], "margin", cx) {
        return;
    }
    if !matches!(block.declarations[top], Declaration::MarginTop(_))
        || !matches!(block.declarations[right], Declaration::MarginRight(_))
        || !matches!(block.declarations[bottom], Declaration::MarginBottom(_))
        || !matches!(block.declarations[left], Declaration::MarginLeft(_))
    {
        return;
    }
    let allocator = block.declarations.bump();
    let top_value = match std::mem::replace(
        &mut block.declarations[top],
        Declaration::All(CSSWideKeyword::Initial),
    ) {
        Declaration::MarginTop(value) => value,
        _ => return,
    };
    let right_value = match std::mem::replace(
        &mut block.declarations[right],
        Declaration::All(CSSWideKeyword::Initial),
    ) {
        Declaration::MarginRight(value) => value,
        _ => return,
    };
    let bottom_value = match std::mem::replace(
        &mut block.declarations[bottom],
        Declaration::All(CSSWideKeyword::Initial),
    ) {
        Declaration::MarginBottom(value) => value,
        _ => return,
    };
    let left_value = match std::mem::replace(
        &mut block.declarations[left],
        Declaration::All(CSSWideKeyword::Initial),
    ) {
        Declaration::MarginLeft(value) => value,
        _ => return,
    };
    let target = *[top, right, bottom, left]
        .iter()
        .max()
        .expect("four longhands");
    block.declarations[target] = Declaration::Margin(allocator.boxed(Margin {
        top: top_value,
        right: right_value,
        bottom: bottom_value,
        left: left_value,
    }));
    mark_merged_longhands(block, [top, right, bottom, left], target, cx);
}

fn merge_padding_longhands<'a>(
    block: &mut DeclarationBlock<'a>,
    important: bool,
    cx: &mut MinifyContext,
) {
    let Some([top, right, bottom, left]) = box_longhand_indices(
        block,
        important,
        [
            "padding-top",
            "padding-right",
            "padding-bottom",
            "padding-left",
        ],
        "padding",
    ) else {
        return;
    };
    if merge_unparsed_box_longhands(block, [top, right, bottom, left], "padding", cx) {
        return;
    }
    if !matches!(block.declarations[top], Declaration::PaddingTop(_))
        || !matches!(block.declarations[right], Declaration::PaddingRight(_))
        || !matches!(block.declarations[bottom], Declaration::PaddingBottom(_))
        || !matches!(block.declarations[left], Declaration::PaddingLeft(_))
    {
        return;
    }
    let allocator = block.declarations.bump();
    let top_value = match std::mem::replace(
        &mut block.declarations[top],
        Declaration::All(CSSWideKeyword::Initial),
    ) {
        Declaration::PaddingTop(value) => value,
        _ => return,
    };
    let right_value = match std::mem::replace(
        &mut block.declarations[right],
        Declaration::All(CSSWideKeyword::Initial),
    ) {
        Declaration::PaddingRight(value) => value,
        _ => return,
    };
    let bottom_value = match std::mem::replace(
        &mut block.declarations[bottom],
        Declaration::All(CSSWideKeyword::Initial),
    ) {
        Declaration::PaddingBottom(value) => value,
        _ => return,
    };
    let left_value = match std::mem::replace(
        &mut block.declarations[left],
        Declaration::All(CSSWideKeyword::Initial),
    ) {
        Declaration::PaddingLeft(value) => value,
        _ => return,
    };
    let target = *[top, right, bottom, left]
        .iter()
        .max()
        .expect("four longhands");
    block.declarations[target] = Declaration::Padding(allocator.boxed(Padding {
        top: top_value,
        right: right_value,
        bottom: bottom_value,
        left: left_value,
    }));
    mark_merged_longhands(block, [top, right, bottom, left], target, cx);
}

fn merge_unparsed_box_longhands<'a>(
    block: &mut DeclarationBlock<'a>,
    indices: [usize; 4],
    shorthand: &'a str,
    cx: &mut MinifyContext,
) -> bool {
    if indices.iter().any(|&index| {
        !matches!(&block.declarations[index], Declaration::Unparsed(value) if value.value.len() == 1)
    }) {
        return false;
    }
    let custom_count = indices
        .iter()
        .filter(|&&index| declaration_contains_variable(&block.declarations[index]))
        .count();
    if custom_count != 0 && custom_count != indices.len() {
        return false;
    }
    let first_value = match &block.declarations[indices[0]] {
        Declaration::Unparsed(value) => &value.value,
        _ => return false,
    };
    let all_equal = indices[1..].iter().all(|&index| {
        matches!(&block.declarations[index], Declaration::Unparsed(value) if value.value == *first_value)
    });
    let preserve_initial = cx.is_enabled(Options::PRESERVE_MERGED_BOX_INITIAL, OptionsOp::Any)
        && all_equal
        && matches!(first_value.as_slice(), [TokenOrValue::Token(token)]
            if matches!(&**token, Token::Ident(keyword)
                if keyword.eq_ignore_ascii_case("initial")));
    if !all_equal
        && indices.iter().any(|&index| {
            matches!(&block.declarations[index], Declaration::Unparsed(value)
                if matches!(value.value.as_slice(), [TokenOrValue::Token(token)]
                    if matches!(&**token, Token::Ident(keyword)
                        if ["inherit", "initial", "unset", "revert"]
                            .iter().any(|value| keyword.eq_ignore_ascii_case(value)))))
        })
    {
        return false;
    }

    let allocator = block.declarations.bump();
    let mut sides = indices.map(|index| {
        let Declaration::Unparsed(value) = &mut block.declarations[index] else {
            unreachable!("validated above")
        };
        std::mem::replace(&mut value.value, allocator.vec())
    });
    let mut value = std::mem::replace(&mut sides[0], allocator.vec());
    for side in &mut sides[1..] {
        value.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
        value.append(side);
    }

    let target = *indices.iter().max().expect("four longhands");
    let Declaration::Unparsed(target_value) = &mut block.declarations[target] else {
        unreachable!("validated above")
    };
    *target_value.property_id = PropertyId::from_name(shorthand);
    target_value.value = value;
    if preserve_initial {
        target_value.value.truncate(1);
    }

    if !preserve_initial {
        let previous = cx.value_context;
        cx.value_context = crate::properties::value_context(
            &target_value.property_id,
            cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
            cx.is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
        );
        target_value.minify(cx);
        cx.value_context = previous;
    }
    mark_merged_longhands(block, indices, target, cx);
    true
}

fn box_longhand_indices(
    block: &DeclarationBlock<'_>,
    important: bool,
    names: [&str; 4],
    shorthand: &str,
) -> Option<[usize; 4]> {
    let last_shorthand = (0..block.declarations.len()).rev().find(|&index| {
        !block.is_invalid(index)
            && block.is_important(index) == important
            && block.declarations[index].name() == shorthand
    });
    let mut indices = [None; 4];
    for index in last_shorthand.map_or(0, |index| index + 1)..block.declarations.len() {
        if block.is_invalid(index) || block.is_important(index) != important {
            continue;
        }
        if let Some(side) = names
            .iter()
            .position(|name| block.declarations[index].name() == *name)
        {
            indices[side] = Some(index);
        }
    }
    Some([indices[0]?, indices[1]?, indices[2]?, indices[3]?])
}

fn mark_merged_longhands(
    block: &mut DeclarationBlock<'_>,
    indices: [usize; 4],
    target: usize,
    cx: &mut MinifyContext,
) {
    for index in indices {
        if index != target {
            block.mark_invalid(index);
            cx.record_declaration_removed();
        }
    }
}

fn process_declarations<'a>(
    block: &mut DeclarationBlock<'a>,
    declarations: &mut HashMap<'a, DeclarationKey<'a>, DeclarationLocation<'a>>,
    cx: &mut MinifyContext,
) {
    discard_empty_declarations(block, cx);
    merge_box_longhands(block, cx);
    let block_pointer = NonNull::from(&mut *block);
    for index in 0..block.declarations.len() {
        if block.is_invalid(index) {
            continue;
        }
        let declaration = &block.declarations[index];
        let key = DeclarationKey {
            name: declaration.name(),
            vendor_prefix: declaration.vendor_prefix(),
            important: block.is_important(index),
        };

        if let Some(mut previous) = declarations.get(&key).copied() {
            // SAFETY: all locations refer to arena-boxed declaration blocks.
            // No declaration vectors are resized or reordered by this pass,
            // so the stored logical index remains valid.
            let duplicate = unsafe {
                previous.block.as_ref().declarations[previous.index]
                    == block_pointer.as_ref().declarations[index]
            };
            if duplicate {
                if cx.is_enabled(Options::KEEP_LATER_DUPLICATE_DECLARATIONS, OptionsOp::Any) {
                    // Keep the later declaration and tombstone the earlier
                    // one. Besides matching cascade order, this lets the IR
                    // map point at the live tail declaration block.
                    unsafe { previous.block.as_mut().mark_invalid(previous.index) };
                } else {
                    block.mark_invalid(index);
                    cx.record_declaration_removed();
                    continue;
                }
                cx.record_declaration_removed();
            }
        }

        declarations.insert(
            key,
            DeclarationLocation {
                block: block_pointer,
                index,
            },
        );
    }
}

fn discard_empty_declarations(block: &mut DeclarationBlock<'_>, cx: &mut MinifyContext) {
    for index in 0..block.declarations.len() {
        if block.is_invalid(index) {
            continue;
        }
        if matches!(&block.declarations[index], rocketcss_ast::Declaration::Unparsed(value) if value.value.is_empty())
        {
            block.mark_invalid(index);
            cx.record_declaration_removed();
        }
    }
}
