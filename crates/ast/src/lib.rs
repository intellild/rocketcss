//! CSS syntax tree data structures ported from lightningcss.
//!
//! Parsing, printing, transformation, and minification logic stays outside of
//! this crate. Typed immutable and mutable traversal is implemented directly by
//! the AST nodes.

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

pub use rocketcss_common::Atom;
pub use rocketcss_macros::{CssKeyword, Visit};

mod color;
mod generated;
mod length;
mod media;
pub mod prelude;
mod properties;
mod rules;
mod selector;
mod span;
mod token;
mod tombstone;
mod values;
mod visit_context;

pub use color::*;
pub use generated::{
    kind::AstType,
    visit::{Visit, Visitor},
    visit_mut::{VisitMut, VisitorMut},
};
pub use length::*;
pub use media::*;
pub use properties::*;
pub use rules::*;
pub use selector::*;
pub use span::*;
pub use token::*;
pub use tombstone::*;
pub use values::*;
pub(crate) use values::{decode_line_style, encode_line_style};
pub use visit_context::{VisitContext, VisitMutContext};

/// Persistent AST list stored as a typed dense range in [`AstContext`].
pub type Vec<'ast, T> = AstVec<'ast, T>;

#[cfg(target_pointer_width = "64")]
const _: () = {
    use std::mem::size_of;

    assert!(size_of::<VendorPrefix>() == 1);
    assert!(size_of::<KnownFunction>() == 1);
    assert!(size_of::<AstVec<'_, u8>>() == 8);
    assert!(size_of::<Declaration<'_>>() == 20);
    assert!(size_of::<TokenOrValue<'_>>() == 24);
    assert!(size_of::<Token<'_>>() == 24);
    assert!(size_of::<CssColor<'_>>() == 8);
    assert!(size_of::<KnownColor>() == 1);
    assert!(size_of::<Length<'_>>() == 8);
    assert!(size_of::<ParsedComponent<'_>>() == 24);
    assert!(size_of::<AnimationComponent<'_>>() == 12);
    assert!(size_of::<Filter<'_>>() == 12);
    assert!(size_of::<Transform<'_>>() == 24);
    assert!(size_of::<KeyframeSelector>() == 8);
    assert!(size_of::<Display>() == 4);
    assert!(size_of::<PlaceContent>() == 4);
    assert!(size_of::<PlaceSelf>() == 4);
    assert!(size_of::<PlaceItems>() == 4);
};

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_common::Allocator;

    #[test]
    fn compares_nodes_while_ignoring_owned_tombstone_slots() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        assert!(FontFamily::Tombstone.eq_ignoring_tombstones(&FontFamily::Tombstone, &ast));
        assert!(!FontFamily::Tombstone.eq_ignoring_tombstones(&FontFamily::Serif, &ast));

        let mut left_families = allocator.vec();
        left_families.push(FontFamily::Custom("A"));
        left_families.push(FontFamily::Tombstone);
        left_families.push(FontFamily::Serif);
        let mut right_families = allocator.vec();
        right_families.push(FontFamily::Custom("A"));
        right_families.push(FontFamily::Serif);

        assert_ne!(left_families, right_families);
        let left_families = ast.alloc_vec(left_families);
        let right_families = ast.alloc_vec(right_families);
        assert!(left_families.eq_ignoring_tombstones(&right_families, &ast));

        let left_declaration = Declaration::FontFamily(left_families);
        let right_declaration = Declaration::FontFamily(right_families);
        assert_ne!(left_declaration, right_declaration);
        assert!(left_declaration.eq_ignoring_tombstones(&right_declaration, &ast));
    }

    #[test]
    fn known_property_ids_use_the_property_discriminant() {
        let width = PropertyId::Width;
        let height = PropertyId::Height;
        let webkit_user_select = PropertyId::UserSelect(VendorPrefix::WEBKIT);
        let moz_user_select = PropertyId::UserSelect(VendorPrefix::MOZ);

        assert_ne!(width.known_id(), height.known_id());
        assert_eq!(webkit_user_select.known_id(), moz_user_select.known_id());
        assert_eq!(
            webkit_user_select.known_id_and_prefix(),
            webkit_user_select
                .known_id()
                .map(|id| (id, VendorPrefix::WEBKIT))
        );
        assert_eq!(
            moz_user_select.known_id_and_prefix(),
            moz_user_select.known_id().map(|id| (id, VendorPrefix::MOZ))
        );
        assert!(PropertyId::All.known_id().is_some());
        assert_eq!(PropertyId::Unparsed.known_id(), None);
        assert_eq!(PropertyId::Custom("unknown").known_id(), None);

        for (name, expected) in [
            ("CoLuMn-RuLe", PropertyId::ColumnRule(VendorPrefix::NONE)),
            ("CoLuMnS", PropertyId::Columns(VendorPrefix::NONE)),
            ("GrId-CoLuMn-GaP", PropertyId::GridColumnGap),
            ("GrId-RoW-GaP", PropertyId::GridRowGap),
        ] {
            let property_id = PropertyId::from_name(name);
            assert_eq!(property_id, expected);
            assert!(property_id.known_id().is_some());
            assert_eq!(property_id.vendor_prefix(), VendorPrefix::NONE);
        }
        assert_eq!(
            PropertyId::from_name("-WeBkIt-CoLuMnS"),
            PropertyId::Columns(VendorPrefix::WEBKIT)
        );
    }

    #[test]
    fn selector_uses_typed_lightningcss_components() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let mut selector = allocator.vec();
        selector.push(SelectorComponent::Nth(NthSelectorData {
            kind: NthType::Child,
            is_function: true,
            a: 2,
            b: 1,
        }));
        selector.push(SelectorComponent::PseudoClass(
            ast.alloc_node_without_span(PseudoClass::Hover),
        ));

        assert!(matches!(selector[0], SelectorComponent::Nth(_)));
        assert!(matches!(
            selector[1],
            SelectorComponent::PseudoClass(value)
                if matches!(ast.resolve_node(value), PseudoClass::Hover)
        ));
    }

    #[test]
    fn function_state_is_accessed_through_flags() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let arguments = ast.alloc_vec(allocator.vec());
        let mut function = Function::new("url", arguments);

        assert_eq!(function.name(), "url");
        assert_eq!(function.kind(), KnownFunction::Url);
        assert!(!function.is_vendor_prefixed());
        assert!(!function.is_identifier());
        assert!(!function.is_unquoted_url());

        function.set_name("VAR");
        function.set_identifier(true);
        function.set_unquoted_url(true);

        assert_eq!(function.name(), "VAR");
        assert_eq!(function.kind(), KnownFunction::Var);
        assert!(!function.is_vendor_prefixed());
        assert!(function.is_identifier());
        assert!(function.is_unquoted_url());
    }

    #[test]
    fn known_function_classifies_case_and_supported_vendor_prefixes() {
        let allocator = Allocator::new();
        assert_eq!(KnownFunction::from_name("RGB"), KnownFunction::Rgb);
        assert_eq!(
            KnownFunction::from_name("-WEBKIT-LINEAR-GRADIENT"),
            KnownFunction::LinearGradient,
        );
        assert_eq!(KnownFunction::from_name("-moz-calc"), KnownFunction::Calc,);
        let mut ast = AstContext::new_in(&allocator);
        let arguments = ast.alloc_vec(allocator.vec());
        let function = Function::new("-moz-calc", arguments);
        assert!(function.is_vendor_prefixed());
        assert_eq!(
            KnownFunction::from_name("custom-function"),
            KnownFunction::Unknown,
        );
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
        assert_eq!(FontFamily::SansSerif.as_css_str(), Some("sans-serif"));
        assert_eq!(FontFamily::Custom("Inter").as_css_str(), None);
    }
}
