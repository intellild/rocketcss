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
use rocketcss_common::prelude::*;
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
pub use visit_context::{VisitContext, VisitMutContext};

#[cfg(target_pointer_width = "64")]
const _: () = {
    use std::mem::size_of;

    assert!(size_of::<VendorPrefix>() == 1);
    assert!(size_of::<KnownFunction>() == 1);
    assert!(size_of::<Declaration<'_>>() == 32);
    assert!(size_of::<TokenOrValue<'_>>() == 24);
    assert!(size_of::<Token<'_>>() == 24);
    assert!(size_of::<CssColor<'_>>() == 16);
    assert!(size_of::<Length<'_>>() == 16);
    assert!(size_of::<ParsedComponent<'_>>() == 32);
    assert!(size_of::<AnimationComponent<'_>>() == 16);
    assert!(size_of::<Filter<'_>>() == 16);
    assert!(size_of::<Transform<'_>>() == 32);
    assert!(size_of::<KeyframeSelector>() == 8);
    assert!(size_of::<Display>() == 4);
    assert!(size_of::<PlaceContent>() == 4);
    assert!(size_of::<PlaceSelf>() == 4);
    assert!(size_of::<PlaceItems>() == 4);
};

#[cfg(test)]
#[path = "../tests/unit/lib.rs"]
mod tests;
