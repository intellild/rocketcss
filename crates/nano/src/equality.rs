use std::hash::{Hash, Hasher};

use rocketcss_ast::{
    AstContext, ConcreteRuleId, CssRulePayload, Declaration, EnvironmentVariableName, Function,
    Mask, TokenOrValue, UnresolvedColor,
};
use rocketcss_codegen::{Printer, PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::GhostToken;
use rustc_hash::FxHasher;

/// Compares the serialized value graph while resolving every stored node through its context.
pub(crate) fn css_values_are_equal<T>(ast: &AstContext<'_>, left: &T, right: &T) -> bool
where
    T: for<'ghost> ToCss<'ghost> + PartialEq,
{
    GhostToken::scope(|token| {
        let cx = ToCssContext::with_ast(&token, ast);
        rocketcss_codegen::css_values_are_equal(left, right, &cx)
    })
}

pub(crate) fn css_value_serialization<T>(ast: &AstContext<'_>, value: &T) -> Option<String>
where
    T: for<'ghost> ToCss<'ghost>,
{
    GhostToken::scope(|token| {
        value
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::with_ast(&token, ast),
            )
            .ok()
    })
}

pub(crate) fn css_value_matches_serialization<T>(
    ast: &AstContext<'_>,
    expected: &str,
    value: &T,
) -> bool
where
    T: for<'ghost> ToCss<'ghost>,
{
    GhostToken::scope(|token| {
        rocketcss_codegen::css_value_matches_serialization(
            expected,
            value,
            &ToCssContext::with_ast(&token, ast),
        )
    })
}

/// Compares declaration value graphs without erasing authored shorthand structure.
///
/// Compact CSS is a useful deep comparison for NodeId-backed values, but it is not by itself an
/// AST equality relation: distinct mask origin/clip pairs can serialize to the same one-value
/// shorthand. Keep those outer fields exact and use serialization only for their stored children.
pub(crate) fn declarations_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: &Declaration<'ast>,
    right: &Declaration<'ast>,
) -> bool {
    css_values_are_equal(ast, left, right)
        && declarations_with_equal_css_are_equal(ast, left, right)
}

/// Returns an exact structural result when the declaration's stored graph is composed entirely of
/// the high-frequency token nodes handled here. `None` keeps uncommon typed graphs on the
/// serialization fallback rather than weakening their equality semantics.
pub(crate) fn known_declaration_structural_equality<'ast>(
    ast: &AstContext<'ast>,
    left: &Declaration<'ast>,
    right: &Declaration<'ast>,
) -> Option<bool> {
    match (left, right) {
        (
            Declaration::CSSWide(left_id, left_value),
            Declaration::CSSWide(right_id, right_value),
        ) => Some(left_value == right_value && ast.nodes_eq(*left_id, *right_id)),
        (Declaration::Unparsed(left), Declaration::Unparsed(right)) => {
            let left = ast.resolve_node(*left);
            let right = ast.resolve_node(*right);
            if left.reason != right.reason
                || left.raw_value.map(|value| ast.str(value))
                    != right.raw_value.map(|value| ast.str(value))
                || !ast.nodes_eq(left.property_id, right.property_id)
            {
                return Some(false);
            }
            token_lists_are_equal(ast, ast.vec_iter(left.value), ast.vec_iter(right.value))
        }
        (Declaration::Custom(left), Declaration::Custom(right)) => {
            let left = ast.resolve_node(*left);
            let right = ast.resolve_node(*right);
            if !ast.nodes_eq(left.name, right.name) {
                return Some(false);
            }
            token_lists_are_equal(ast, ast.vec_iter(left.value), ast.vec_iter(right.value))
        }
        (Declaration::CSSWide(..) | Declaration::Unparsed(_) | Declaration::Custom(_), _)
        | (_, Declaration::CSSWide(..) | Declaration::Unparsed(_) | Declaration::Custom(_)) => {
            Some(false)
        }
        _ => None,
    }
}

fn token_lists_are_equal<'ast, I, J>(ast: &AstContext<'ast>, left: I, right: J) -> Option<bool>
where
    I: ExactSizeIterator<Item = TokenOrValue<'ast>>,
    J: ExactSizeIterator<Item = TokenOrValue<'ast>>,
{
    if left.len() != right.len() {
        return Some(false);
    }
    for (left, right) in left.zip(right) {
        match token_or_value_equality(ast, &left, &right) {
            Some(true) => {}
            result => return result,
        }
    }
    Some(true)
}

fn optional_token_lists_are_equal<'ast, I, J>(
    ast: &AstContext<'ast>,
    left: Option<I>,
    right: Option<J>,
) -> Option<bool>
where
    I: ExactSizeIterator<Item = TokenOrValue<'ast>>,
    J: ExactSizeIterator<Item = TokenOrValue<'ast>>,
{
    match (left, right) {
        (Some(left), Some(right)) => token_lists_are_equal(ast, left, right),
        (None, None) => Some(true),
        _ => Some(false),
    }
}

fn token_or_value_equality<'ast>(
    ast: &AstContext<'ast>,
    left: &TokenOrValue<'ast>,
    right: &TokenOrValue<'ast>,
) -> Option<bool> {
    match (left, right) {
        (TokenOrValue::Token(left), TokenOrValue::Token(right)) => {
            Some(ast.nodes_eq(*left, *right))
        }
        (TokenOrValue::Url(left), TokenOrValue::Url(right)) => {
            Some(ast.node_span(*left) == ast.node_span(*right) && ast.nodes_eq(*left, *right))
        }
        (TokenOrValue::Var(left), TokenOrValue::Var(right)) => {
            let left = ast.resolve_node(*left);
            let right = ast.resolve_node(*right);
            if !ast.nodes_eq(left.name, right.name) {
                return Some(false);
            }
            optional_token_lists_are_equal(
                ast,
                left.fallback.map(|values| ast.vec_iter(values)),
                right.fallback.map(|values| ast.vec_iter(values)),
            )
        }
        (TokenOrValue::Env(left), TokenOrValue::Env(right)) => {
            let left = ast.resolve_node(*left);
            let right = ast.resolve_node(*right);
            if !ast.vec_iter(left.indices).eq(ast.vec_iter(right.indices))
                || !environment_variable_names_are_equal(ast, &left.name, &right.name)
            {
                return Some(false);
            }
            optional_token_lists_are_equal(
                ast,
                left.fallback.map(|values| ast.vec_iter(values)),
                right.fallback.map(|values| ast.vec_iter(values)),
            )
        }
        (TokenOrValue::Function(left), TokenOrValue::Function(right)) => {
            functions_are_equal(ast, &ast.resolve_node(*left), &ast.resolve_node(*right))
        }
        (TokenOrValue::UnresolvedColor(left), TokenOrValue::UnresolvedColor(right)) => {
            unresolved_colors_are_equal(ast, &ast.resolve_node(*left), &ast.resolve_node(*right))
        }
        (TokenOrValue::Color(left), TokenOrValue::Color(right)) => {
            ast.nodes_eq(*left, *right).then_some(true)
        }
        (TokenOrValue::AnimationName(left), TokenOrValue::AnimationName(right)) => {
            Some(ast.nodes_eq(*left, *right))
        }
        (TokenOrValue::Length(left), TokenOrValue::Length(right)) => Some(left == right),
        (TokenOrValue::Angle(left), TokenOrValue::Angle(right)) => Some(left == right),
        (TokenOrValue::Time(left), TokenOrValue::Time(right)) => Some(left == right),
        (TokenOrValue::Resolution(left), TokenOrValue::Resolution(right)) => Some(left == right),
        (TokenOrValue::DashedIdent(left), TokenOrValue::DashedIdent(right)) => {
            Some(ast.nodes_eq(*left, *right))
        }
        _ if std::mem::discriminant(left) != std::mem::discriminant(right) => Some(false),
        _ => None,
    }
}

fn environment_variable_names_are_equal(
    ast: &AstContext<'_>,
    left: &EnvironmentVariableName<'_>,
    right: &EnvironmentVariableName<'_>,
) -> bool {
    match (left, right) {
        (EnvironmentVariableName::Custom(left), EnvironmentVariableName::Custom(right)) => {
            ast.nodes_eq(*left, *right)
        }
        (EnvironmentVariableName::Unknown(left), EnvironmentVariableName::Unknown(right)) => {
            ast.str(*left) == ast.str(*right)
        }
        _ => left == right,
    }
}

fn functions_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: &Function<'ast>,
    right: &Function<'ast>,
) -> Option<bool> {
    if ast.str(left.name()) != ast.str(right.name())
        || left.kind() != right.kind()
        || left.replacement != right.replacement
        || left.is_vendor_prefixed() != right.is_vendor_prefixed()
        || left.is_valid_rgb() != right.is_valid_rgb()
        || left.is_identifier() != right.is_identifier()
        || left.is_unquoted_url() != right.is_unquoted_url()
    {
        return Some(false);
    }
    token_lists_are_equal(
        ast,
        ast.vec_iter(left.arguments),
        ast.vec_iter(right.arguments),
    )
}

fn unresolved_colors_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: &UnresolvedColor<'ast>,
    right: &UnresolvedColor<'ast>,
) -> Option<bool> {
    match (left, right) {
        (
            UnresolvedColor::Rgb {
                alpha: left_alpha,
                b: left_b,
                g: left_g,
                r: left_r,
            },
            UnresolvedColor::Rgb {
                alpha: right_alpha,
                b: right_b,
                g: right_g,
                r: right_r,
            },
        ) if left_b == right_b && left_g == right_g && left_r == right_r => {
            token_lists_are_equal(ast, ast.vec_iter(*left_alpha), ast.vec_iter(*right_alpha))
        }
        (
            UnresolvedColor::Hsl {
                alpha: left_alpha,
                h: left_h,
                l: left_l,
                s: left_s,
            },
            UnresolvedColor::Hsl {
                alpha: right_alpha,
                h: right_h,
                l: right_l,
                s: right_s,
            },
        ) if left_h == right_h && left_l == right_l && left_s == right_s => {
            token_lists_are_equal(ast, ast.vec_iter(*left_alpha), ast.vec_iter(*right_alpha))
        }
        (
            UnresolvedColor::LightDark {
                dark: left_dark,
                light: left_light,
            },
            UnresolvedColor::LightDark {
                dark: right_dark,
                light: right_light,
            },
        ) => {
            match token_lists_are_equal(ast, ast.vec_iter(*left_dark), ast.vec_iter(*right_dark)) {
                Some(true) => token_lists_are_equal(
                    ast,
                    ast.vec_iter(*left_light),
                    ast.vec_iter(*right_light),
                ),
                result => result,
            }
        }
        _ => Some(false),
    }
}

/// Applies authored-structure guards after callers have established equal compact CSS.
pub(crate) fn declarations_with_equal_css_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: &Declaration<'ast>,
    right: &Declaration<'ast>,
) -> bool {
    match (left, right) {
        (Declaration::Mask(left, left_prefix), Declaration::Mask(right, right_prefix)) => {
            left_prefix == right_prefix
                && masks_are_equal(ast, ast.vec_iter(*left), ast.vec_iter(*right))
        }
        _ => true,
    }
}

fn masks_are_equal<'ast, I, J>(ast: &AstContext<'ast>, left: I, right: J) -> bool
where
    I: ExactSizeIterator<Item = rocketcss_ast::NodeId<'ast, Mask<'ast>>>,
    J: ExactSizeIterator<Item = rocketcss_ast::NodeId<'ast, Mask<'ast>>>,
{
    left.len() == right.len()
        && left.zip(right).all(|(left, right)| {
            let left = ast.resolve_node(left);
            let right = ast.resolve_node(right);
            left.clip == right.clip
                && left.composite == right.composite
                && left.mode == right.mode
                && left.origin == right.origin
                && left.repeat == right.repeat
                && images_are_equal(ast, left.image, right.image)
                && stored_values_are_equal(ast, left.position, right.position)
                && stored_values_are_equal(ast, left.size, right.size)
        })
}

fn images_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: rocketcss_ast::NodeId<'ast, rocketcss_ast::Image<'ast>>,
    right: rocketcss_ast::NodeId<'ast, rocketcss_ast::Image<'ast>>,
) -> bool {
    match (ast.resolve_node(left), ast.resolve_node(right)) {
        (rocketcss_ast::Image::Url(left), rocketcss_ast::Image::Url(right)) => {
            ast.node_span(left) == ast.node_span(right) && ast.nodes_eq(left, right)
        }
        (left, right) => css_values_are_equal(ast, &left, &right),
    }
}

fn stored_values_are_equal<'ast, T>(
    ast: &AstContext<'ast>,
    left: rocketcss_ast::NodeId<'ast, T>,
    right: rocketcss_ast::NodeId<'ast, T>,
) -> bool
where
    T: for<'ghost> ToCss<'ghost> + PartialEq + rocketcss_ast::AstNodeStorage<'ast>,
{
    css_values_are_equal(ast, &ast.resolve_node(left), &ast.resolve_node(right))
}

fn css_value_fingerprint<T>(ast: &AstContext<'_>, value: &T) -> u64
where
    T: for<'ghost> ToCss<'ghost> + PartialEq,
{
    GhostToken::scope(|token| {
        let cx = ToCssContext::with_ast(&token, ast);
        let mut hasher = FxHasher::default();
        let mut writer = CssHashWriter(&mut hasher);
        let _ = value.to_css(
            &mut Printer::new(&mut writer, PrinterOptions { prettify: false }),
            &cx,
        );
        hasher.finish()
    })
}

struct CssHashWriter<'a>(&'a mut FxHasher);

impl std::fmt::Write for CssHashWriter<'_> {
    #[inline]
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        self.0.write(value.as_bytes());
        Ok(())
    }
}

pub(crate) fn context_frame_fingerprint<'ast>(
    ast: &AstContext<'ast>,
    rule: ConcreteRuleId<'ast>,
) -> u64 {
    match ast
        .rule(rule)
        .expect("a context representative remains resolvable")
        .payload()
    {
        CssRulePayload::Media(payload) => css_value_fingerprint(ast, &payload.query),
        CssRulePayload::Supports(payload) => css_value_fingerprint(ast, &payload.condition),
        CssRulePayload::Container(payload) => {
            let mut hasher = FxHasher::default();
            payload.name.map(|name| ast.str(name)).hash(&mut hasher);
            css_value_fingerprint(ast, &payload.condition).hash(&mut hasher);
            hasher.finish()
        }
        _ => rule.index() as u64,
    }
}

pub(crate) fn context_frames_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: ConcreteRuleId<'ast>,
    right: ConcreteRuleId<'ast>,
) -> bool {
    let left = ast
        .rule(left)
        .expect("a context representative remains resolvable")
        .payload();
    let right = ast
        .rule(right)
        .expect("a context representative remains resolvable")
        .payload();
    match (left, right) {
        (CssRulePayload::Media(left), CssRulePayload::Media(right)) => {
            css_values_are_equal(ast, &left.query, &right.query)
        }
        (CssRulePayload::Supports(left), CssRulePayload::Supports(right)) => {
            css_values_are_equal(ast, &left.condition, &right.condition)
        }
        (CssRulePayload::Container(left), CssRulePayload::Container(right)) => {
            left.name.map(|name| ast.str(name)) == right.name.map(|name| ast.str(name))
                && css_values_are_equal(ast, &left.condition, &right.condition)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::images_are_equal;
    use rocketcss_ast::{AstContext, DUMMY_SP, Image, Span, Url};
    use rocketcss_common::Allocator;

    #[test]
    fn nested_declaration_equality_resolves_text_and_preserves_structure() {
        use super::known_declaration_structural_equality as equal;
        use rocketcss_ast::{
            AstStr, AstVec, Declaration, EnvironmentVariable, EnvironmentVariableName, Function,
            NodeId, PropertyId, Token, TokenOrValue, UnparsedProperty, UnparsedPropertyReason,
        };
        use rocketcss_common::vec::Vec;
        fn graph<'a>(
            ast: &mut AstContext<'a>,
            allocator: &'a Allocator,
            text: AstStr<'a>,
        ) -> (
            Declaration<'a>,
            NodeId<'a, Function<'a>>,
            NodeId<'a, EnvironmentVariable<'a>>,
            AstVec<'a, TokenOrValue<'a>>,
        ) {
            let leaf = ast.alloc_node(Token::Ident(text), DUMMY_SP);
            let fallback = ast.alloc_vec(Vec::from_iter_in([TokenOrValue::Token(leaf)], allocator));
            let indices = ast.alloc_vec(Vec::from_iter_in([1, 2], allocator));
            let env = ast.alloc_node(
                EnvironmentVariable {
                    name: EnvironmentVariableName::Unknown(text),
                    indices,
                    fallback: Some(fallback),
                },
                DUMMY_SP,
            );
            let arguments = ast.alloc_vec(Vec::from_iter_in([TokenOrValue::Env(env)], allocator));
            let function = Function::new("Fn", arguments, ast);
            let function = ast.alloc_node(function, DUMMY_SP);
            let value = ast.alloc_vec(Vec::from_iter_in(
                [TokenOrValue::Function(function)],
                allocator,
            ));
            let property_id = ast.alloc_node(PropertyId::Custom(text), DUMMY_SP);
            let declaration = ast.alloc_node(
                UnparsedProperty {
                    property_id,
                    reason: UnparsedPropertyReason::OpaqueValue,
                    raw_value: Some(text),
                    value,
                },
                DUMMY_SP,
            );
            (Declaration::Unparsed(declaration), function, env, fallback)
        }
        let allocator = Allocator::new();
        let mut ast = AstContext::with_source_in(&allocator, "same same SAME", Default::default());
        let first = ast.string_pool().source_range(0, 4);
        let second = ast.string_pool().source_range(5, 9);
        let uppercase = ast.string_pool().source_range(10, 14);
        let extra = ast.add_str("same");
        assert_ne!(first, second);
        assert_ne!(first, extra);
        let (left, _, _, _) = graph(&mut ast, &allocator, first);
        let (right, function, env, fallback) = graph(&mut ast, &allocator, second);
        let (third, _, _, _) = graph(&mut ast, &allocator, extra);
        let empty = ast.alloc_vec(allocator.vec::<TokenOrValue<'_>>());
        let reversed_indices = ast.alloc_vec(Vec::from_iter_in([2, 1], &allocator));
        let original_indices = ast.resolve_node(env).indices;
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        assert_eq!(equal(&ast, &left, &right), Some(true));
        assert_eq!(equal(&ast, &left, &third), Some(true));
        for changed in [None, Some(empty), Some(fallback)] {
            ast.mutate_node(env, |value, _| value.fallback = changed);
            assert_eq!(equal(&ast, &left, &right), Some(changed == Some(fallback)));
        }
        for indices in [reversed_indices, original_indices] {
            ast.mutate_node(env, |value, _| value.indices = indices);
            assert_eq!(
                equal(&ast, &left, &right),
                Some(indices == original_indices)
            );
        }
        for name in [uppercase, second] {
            ast.mutate_node(env, |value, _| {
                value.name = EnvironmentVariableName::Unknown(name)
            });
            assert_eq!(equal(&ast, &left, &right), Some(name == second));
        }
        for identifier in [true, false] {
            ast.mutate_node(function, |value, _| value.set_identifier(identifier));
            assert_eq!(equal(&ast, &left, &right), Some(!identifier));
        }
        let Declaration::Unparsed(root) = right else {
            unreachable!()
        };
        for raw in [None, Some(AstStr::EMPTY), Some(second)] {
            ast.mutate_node(root, |value, _| value.raw_value = raw);
            assert_eq!(equal(&ast, &left, &right), Some(raw == Some(second)));
        }
        for reason in [
            UnparsedPropertyReason::InvalidValue,
            UnparsedPropertyReason::OpaqueValue,
        ] {
            ast.mutate_node(root, |value, _| value.reason = reason);
            assert_eq!(
                equal(&ast, &left, &right),
                Some(reason == UnparsedPropertyReason::OpaqueValue)
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    }

    #[test]
    fn ordinary_name_lists_compare_contents_and_preserve_order_and_duplicates() {
        use super::css_values_are_equal;
        use rocketcss_ast::{ContainerNameList, NoneOrCustomIdentList};
        let allocator = Allocator::new();
        let mut ast =
            AstContext::with_source_in(&allocator, "alpha alpha beta", Default::default());
        let first = ast.string_pool().source_range(0, 5);
        let second = ast.string_pool().source_range(6, 11);
        let beta = ast.string_pool().source_range(12, 16);
        let extra = ast.add_str("alpha");
        assert_ne!(first, second);
        assert_ne!(first, extra);
        let lists = [
            std::vec![first, beta, first],
            std::vec![second, beta, extra],
            std::vec![first, first, beta],
            std::vec![first, beta],
        ]
        .map(|items| {
            ast.alloc_vec({
                let mut values = allocator.vec();
                values.extend(items);
                values
            })
        });
        let containers =
            lists.map(|names| ast.alloc_node(ContainerNameList::Names(names), DUMMY_SP));
        let transitions =
            lists.map(|names| ast.alloc_node(NoneOrCustomIdentList::Idents(names), DUMMY_SP));
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        let interned = ast.string_pool().len();
        // Distinct list handles need the actual content fallback in Nano equality.
        assert_ne!(lists[0], lists[1]);
        for _ in 0..3 {
            assert!(css_values_are_equal(&ast, &containers[0], &containers[1]));
            assert!(css_values_are_equal(&ast, &transitions[0], &transitions[1]));
            for index in [2, 3] {
                assert!(!css_values_are_equal(
                    &ast,
                    &containers[0],
                    &containers[index]
                ));
                assert!(!css_values_are_equal(
                    &ast,
                    &transitions[0],
                    &transitions[index]
                ));
            }
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
        assert_eq!(ast.string_pool().len(), interned);
    }

    #[test]
    fn context_fingerprints_resolve_source_and_extra_strings() {
        use super::{context_frame_fingerprint, context_frames_are_equal};
        use rocketcss_ast::{
            AstStr, ContainerRulePayload, CssRulePayload, SupportsCondition, SupportsRulePayload,
        };
        let allocator = Allocator::new();
        let mut ast = AstContext::with_source_in(&allocator, "same same", Default::default());
        let first = ast.string_pool().source_range(0, 4);
        let second = ast.string_pool().source_range(5, 9);
        let extra = ast.add_str("same");
        let different = ast.add_str("other");
        assert_ne!(first, second);
        assert_ne!(first, extra);
        let root = ast.stylesheet().root_rules();
        let supports = [first, second, extra, different].map(|value| {
            ast.append_rule(
                root,
                CssRulePayload::Supports(SupportsRulePayload {
                    condition: SupportsCondition::Unknown(value),
                }),
            )
            .unwrap()
        });
        let containers = [
            Some(first),
            Some(second),
            Some(extra),
            Some(different),
            None,
            Some(AstStr::EMPTY),
        ]
        .map(|name| {
            ast.append_rule(
                root,
                CssRulePayload::Container(ContainerRulePayload {
                    name,
                    condition: None,
                }),
            )
            .unwrap()
        });
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        let interned = ast.string_pool().len();
        for _ in 0..3 {
            for rules in [&supports[..], &containers[..]] {
                for index in [1, 2] {
                    assert!(context_frames_are_equal(&ast, rules[0], rules[index]));
                    assert_eq!(
                        context_frame_fingerprint(&ast, rules[0]),
                        context_frame_fingerprint(&ast, rules[index])
                    );
                }
                assert!(!context_frames_are_equal(&ast, rules[0], rules[3]));
            }
            assert!(!context_frames_are_equal(
                &ast,
                containers[4],
                containers[5]
            ));
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), bytes);
            assert_eq!(ast.string_pool().len(), interned);
        }
    }

    #[test]
    fn image_url_equality_resolves_text_but_preserves_source_span() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let first = ast.add_str("icons/é.svg");
        let second = ast.add_str("icons/é.svg");
        let different = ast.add_str("icons/other.svg");
        assert_ne!(first, second);
        let first_url = ast.alloc_node(Url { url: first }, DUMMY_SP);
        let second_url = ast.alloc_node(Url { url: second }, DUMMY_SP);
        let different_url = ast.alloc_node(Url { url: different }, DUMMY_SP);
        let first = ast.alloc_node(Image::Url(first_url), DUMMY_SP);
        let second = ast.alloc_node(Image::Url(second_url), DUMMY_SP);
        let different = ast.alloc_node(Image::Url(different_url), DUMMY_SP);
        assert!(images_are_equal(&ast, first, second));
        assert!(!images_are_equal(&ast, first, different));
        ast.set_node_span(second_url, Span::new(1, 12));
        assert!(!images_are_equal(&ast, first, second));
    }
}
