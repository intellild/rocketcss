//! CSS syntax tree data structures ported from lightningcss.
//!
//! This crate intentionally contains data definitions only. Parsing, printing,
//! transformation, and minification logic stays outside of the AST crate.

#![allow(non_camel_case_types)]

use rocketcss_allocator::prelude::*;

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
    fn declaration_block_links_survive_vec_reallocation() {
        let allocator = Allocator::new();
        let mut first = allocator.pinned(DeclarationBlock::new(&allocator));
        let mut second = allocator.pinned(DeclarationBlock::new(&allocator));
        let first_ptr = first.as_ref().get_ref() as *const DeclarationBlock<'_>;
        let second_ptr = second.as_ref().get_ref() as *const DeclarationBlock<'_>;

        second.as_mut().link_previous(first.as_mut());

        let mut blocks = allocator.vec();
        blocks.push(first);
        blocks.push(second);
        for _ in 0..64 {
            blocks.push(allocator.pinned(DeclarationBlock::new(&allocator)));
        }

        assert_eq!(blocks[1].first() as *const DeclarationBlock<'_>, first_ptr);
        assert_eq!(
            blocks[0].next().unwrap() as *const DeclarationBlock<'_>,
            second_ptr
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
}
