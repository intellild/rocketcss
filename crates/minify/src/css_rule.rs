use rs_css_ast::{CssRule, DeclarationBlock};

pub(crate) fn declaration_blocks_equal(
    left: &DeclarationBlock<'_>,
    right: &DeclarationBlock<'_>,
) -> bool {
    left.declarations == right.declarations
        && left.declarations_importance == right.declarations_importance
}

pub(crate) fn same_rule(left: &CssRule<'_>, right: &CssRule<'_>) -> bool {
    match (left, right) {
        (CssRule::Style(left), CssRule::Style(right)) => {
            left.vendor_prefix == right.vendor_prefix
                && left.selectors == right.selectors
                && left.rules.is_empty()
                && right.rules.is_empty()
                && declaration_blocks_equal(&left.declarations, &right.declarations)
        }
        (CssRule::Unknown(left), CssRule::Unknown(right)) => {
            left.name == right.name && left.prelude == right.prelude && left.block == right.block
        }
        (CssRule::Ignored, CssRule::Ignored) => true,
        _ => false,
    }
}

pub(crate) fn is_overridden_by(left: &CssRule<'_>, right: &CssRule<'_>) -> bool {
    match (left, right) {
        (CssRule::Keyframes(left), CssRule::Keyframes(right)) => {
            left.name == right.name && left.vendor_prefix == right.vendor_prefix
        }
        (CssRule::CounterStyle(left), CssRule::CounterStyle(right)) => left.name == right.name,
        (CssRule::Property(left), CssRule::Property(right)) => left.name == right.name,
        (CssRule::FontPaletteValues(left), CssRule::FontPaletteValues(right)) => {
            left.name == right.name
        }
        _ => false,
    }
}

pub(crate) fn is_empty(rule: &CssRule<'_>) -> bool {
    match rule {
        CssRule::Style(rule) => rule.declarations.is_empty() && rule.rules.is_empty(),
        CssRule::Media(rule) => rule.rules.is_empty(),
        CssRule::Keyframes(rule) => rule.keyframes.is_empty(),
        CssRule::FontFace(rule) => rule.properties.is_empty(),
        CssRule::FontPaletteValues(rule) => rule.properties.is_empty(),
        CssRule::FontFeatureValues(rule) => rule.rules.is_empty(),
        CssRule::Page(rule) => rule.declarations.is_empty() && rule.rules.is_empty(),
        CssRule::Supports(rule) => rule.rules.is_empty(),
        CssRule::CounterStyle(rule) => rule.declarations.is_empty(),
        CssRule::MozDocument(rule) => rule.rules.is_empty(),
        CssRule::Nesting(rule) => rule.style.declarations.is_empty() && rule.style.rules.is_empty(),
        CssRule::NestedDeclarations(rule) => rule.declarations.is_empty(),
        CssRule::Viewport(rule) => rule.declarations.is_empty(),
        CssRule::LayerStatement(rule) => rule.names.is_empty(),
        CssRule::LayerBlock(rule) => rule.name.is_none() && rule.rules.is_empty(),
        CssRule::Container(rule) => rule.rules.is_empty(),
        CssRule::Scope(rule) => rule.rules.is_empty(),
        CssRule::StartingStyle(rule) => rule.rules.is_empty(),
        CssRule::ViewTransition(rule) => rule.properties.is_empty(),
        CssRule::PositionTry(rule) => rule.declarations.is_empty(),
        CssRule::Unknown(rule) => rule.block.as_ref().is_some_and(|block| block.is_empty()),
        CssRule::Ignored => true,
        CssRule::Import(_)
        | CssRule::Namespace(_)
        | CssRule::CustomMedia(_)
        | CssRule::Property(_)
        | CssRule::Custom(_) => false,
    }
}
