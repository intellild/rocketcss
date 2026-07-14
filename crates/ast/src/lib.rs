//! CSS syntax tree data structures ported from lightningcss.
//!
//! This crate intentionally contains data definitions only. Parsing, printing,
//! transformation, and minification logic stays outside of the AST crate.

#![allow(non_camel_case_types)]

/// Matches a value against ASCII case-insensitive string literal arms.
#[macro_export]
macro_rules! match_ignore_ascii_case {
    (
        $value:expr,
        $($($expected:literal)|+ => $result:expr,)+
        _ => $fallback:expr $(,)?
    ) => {{
        let value = $value;
        $(
            if $(value.eq_ignore_ascii_case($expected))||+ {
                $result
            } else
        )+
        {
            $fallback
        }
    }};
}

use rocketcss_allocator::prelude::*;
pub use rocketcss_macros::CssKeyword;

mod color;
mod css_rule;
mod length;
mod media;
pub mod prelude;
mod properties;
mod rules;
mod selector;
mod span;
mod token;
mod values;

pub use color::*;
pub use css_rule::*;
pub use length::*;
pub use media::*;
pub use properties::*;
pub use rules::*;
pub use selector::*;
pub use span::*;
pub use token::*;
pub use values::*;

#[cfg(target_pointer_width = "64")]
const _: () = {
    use std::mem::size_of;

    assert!(size_of::<VendorPrefix>() == 1);
    assert!(size_of::<CssRule<'_>>() == 16);
    assert!(size_of::<Declaration<'_>>() == 32);
    assert!(size_of::<Token<'_>>() == 24);
    assert!(size_of::<CssColor<'_>>() == 16);
    assert!(size_of::<Length<'_>>() == 16);
};

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_allocator::Allocator;

    #[test]
    fn position_try_rule_uses_span() {
        let allocator = Allocator::new();
        let rule = PositionTryRule {
            span: Span::new(4, 42),
            name: "--fallback",
            declarations: allocator.pinned(DeclarationBlock::new(&allocator)),
        };
        let rule = CssRule::PositionTry(allocator.boxed(rule));

        assert_eq!(rule.span(), Span::new(4, 42));
    }

    #[test]
    fn declaration_block_remains_pinned_when_its_container_grows() {
        let allocator = Allocator::new();
        let first = allocator.pinned(DeclarationBlock::new(&allocator));
        let first_ptr = first.as_ref().get_ref() as *const DeclarationBlock<'_>;
        let mut blocks = allocator.vec();
        blocks.push(first);

        for _ in 0..32 {
            blocks.push(allocator.pinned(DeclarationBlock::new(&allocator)));
        }

        assert_eq!(
            blocks[0].as_ref().get_ref() as *const DeclarationBlock<'_>,
            first_ptr,
        );
    }

    #[test]
    fn selector_uses_typed_lightningcss_components() {
        let allocator = Allocator::new();
        let mut selector = allocator.vec();
        selector.push(SelectorComponent::Nth(NthSelectorData {
            kind: NthType::Child,
            is_function: true,
            a: 2,
            b: 1,
        }));
        selector.push(SelectorComponent::PseudoClass(
            allocator.boxed(PseudoClass::Hover),
        ));

        assert!(matches!(selector[0], SelectorComponent::Nth(_)));
        assert!(matches!(
            selector[1],
            SelectorComponent::PseudoClass(ref value) if matches!(**value, PseudoClass::Hover)
        ));
    }

    #[test]
    fn function_state_is_accessed_through_flags() {
        let allocator = Allocator::new();
        let mut function = Function::new("url", allocator.vec());

        assert!(!function.is_identifier());
        assert!(!function.is_unquoted_url());

        function.set_identifier(true);
        function.set_unquoted_url(true);

        assert!(function.is_identifier());
        assert!(function.is_unquoted_url());
    }

    #[test]
    fn css_keyword_derive_handles_defaults_overrides_and_dynamic_variants() {
        assert_eq!(NthType::LastOfType.as_css_str(), Some("last-of-type"));
        assert_eq!(LengthUnit::Cqmax.as_css_str(), Some("cqmax"));
        assert_eq!(
            MediaFeatureId::WebkitDevicePixelRatio.as_css_str(),
            Some("-webkit-device-pixel-ratio"),
        );
        assert_eq!(Appearance::NonStandard("textfield").as_css_str(), None);
        assert_eq!(FontFormat::String("woff3").as_css_str(), None);
    }
}
