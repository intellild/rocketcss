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

/// Returns the old Box-backed structural result when the declaration's stored graph is composed
/// entirely of the high-frequency token nodes handled here. `None` keeps uncommon typed graphs on
/// the serialization fallback rather than weakening their equality semantics.
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
                || left.raw_value != right.raw_value
                || !ast.nodes_eq(left.property_id, right.property_id)
            {
                return Some(false);
            }
            token_lists_are_equal(ast, ast.vec(left.value), ast.vec(right.value))
        }
        (Declaration::Custom(left), Declaration::Custom(right)) => {
            let left = ast.resolve_node(*left);
            let right = ast.resolve_node(*right);
            if !ast.nodes_eq(left.name, right.name) {
                return Some(false);
            }
            token_lists_are_equal(ast, ast.vec(left.value), ast.vec(right.value))
        }
        (Declaration::CSSWide(..) | Declaration::Unparsed(_) | Declaration::Custom(_), _)
        | (_, Declaration::CSSWide(..) | Declaration::Unparsed(_) | Declaration::Custom(_)) => {
            Some(false)
        }
        _ => None,
    }
}

fn token_lists_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: &[TokenOrValue<'ast>],
    right: &[TokenOrValue<'ast>],
) -> Option<bool> {
    if left.len() != right.len() {
        return Some(false);
    }
    for (left, right) in left.iter().zip(right) {
        match token_or_value_equality(ast, left, right) {
            Some(true) => {}
            result => return result,
        }
    }
    Some(true)
}

fn optional_token_lists_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: Option<&[TokenOrValue<'ast>]>,
    right: Option<&[TokenOrValue<'ast>]>,
) -> Option<bool> {
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
                left.fallback.map(|values| ast.vec(values)),
                right.fallback.map(|values| ast.vec(values)),
            )
        }
        (TokenOrValue::Env(left), TokenOrValue::Env(right)) => {
            let left = ast.resolve_node(*left);
            let right = ast.resolve_node(*right);
            if ast.vec(left.indices) != ast.vec(right.indices)
                || !environment_variable_names_are_equal(ast, &left.name, &right.name)
            {
                return Some(false);
            }
            optional_token_lists_are_equal(
                ast,
                left.fallback.map(|values| ast.vec(values)),
                right.fallback.map(|values| ast.vec(values)),
            )
        }
        (TokenOrValue::Function(left), TokenOrValue::Function(right)) => {
            functions_are_equal(ast, ast.resolve_node(*left), ast.resolve_node(*right))
        }
        (TokenOrValue::UnresolvedColor(left), TokenOrValue::UnresolvedColor(right)) => {
            unresolved_colors_are_equal(ast, ast.resolve_node(*left), ast.resolve_node(*right))
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
        (TokenOrValue::DashedIdent(left), TokenOrValue::DashedIdent(right)) => Some(left == right),
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
        _ => left == right,
    }
}

fn functions_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: &Function<'ast>,
    right: &Function<'ast>,
) -> Option<bool> {
    if left.name() != right.name()
        || left.kind() != right.kind()
        || left.replacement != right.replacement
        || left.is_vendor_prefixed() != right.is_vendor_prefixed()
        || left.is_valid_rgb() != right.is_valid_rgb()
        || left.is_identifier() != right.is_identifier()
        || left.is_unquoted_url() != right.is_unquoted_url()
    {
        return Some(false);
    }
    token_lists_are_equal(ast, ast.vec(left.arguments), ast.vec(right.arguments))
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
            token_lists_are_equal(ast, ast.vec(*left_alpha), ast.vec(*right_alpha))
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
            token_lists_are_equal(ast, ast.vec(*left_alpha), ast.vec(*right_alpha))
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
        ) => match token_lists_are_equal(ast, ast.vec(*left_dark), ast.vec(*right_dark)) {
            Some(true) => token_lists_are_equal(ast, ast.vec(*left_light), ast.vec(*right_light)),
            result => result,
        },
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
            left_prefix == right_prefix && masks_are_equal(ast, ast.vec(*left), ast.vec(*right))
        }
        _ => true,
    }
}

fn masks_are_equal<'ast>(
    ast: &AstContext<'ast>,
    left: &[Mask<'ast>],
    right: &[Mask<'ast>],
) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(left, right)| {
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
            ast.node_span(*left) == ast.node_span(*right)
                && ast.resolve_node(*left) == ast.resolve_node(*right)
        }
        (left, right) => css_values_are_equal(ast, left, right),
    }
}

fn stored_values_are_equal<'ast, T>(
    ast: &AstContext<'ast>,
    left: rocketcss_ast::NodeId<'ast, T>,
    right: rocketcss_ast::NodeId<'ast, T>,
) -> bool
where
    T: for<'ghost> ToCss<'ghost> + PartialEq,
{
    css_values_are_equal(ast, ast.resolve_node(left), ast.resolve_node(right))
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
            payload.name.hash(&mut hasher);
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
            left.name == right.name && css_values_are_equal(ast, &left.condition, &right.condition)
        }
        _ => false,
    }
}
