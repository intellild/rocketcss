mod animation;
mod at_rule;
mod background;
mod border;
mod container;
mod font;
mod keyframes;
mod layout;
mod page;
mod property;
mod shape;
mod stylesheet;
mod text;
mod transform;
mod ui;
mod view_transition;

pub use animation::*;
pub use at_rule::*;
pub use background::*;
pub use border::*;
pub use container::*;
pub use font::*;
pub use keyframes::*;
pub use layout::*;
pub use page::*;
pub use property::*;
pub use shape::*;
pub use stylesheet::*;
pub use text::*;
pub use transform::*;
pub use ui::*;
pub use view_transition::*;

use crate::{GetSpan, SetSpan, Span};

macro_rules! impl_spanned {
    ($($ty:ident),+ $(,)?) => {
        $(
            impl GetSpan for $ty<'_> {
                #[inline]
                fn span(&self) -> Span {
                    self.span
                }
            }

            impl SetSpan for $ty<'_> {
                #[inline]
                fn set_span(&mut self, span: Span) {
                    self.span = span;
                }
            }
        )+
    };
}

macro_rules! impl_spanned_ghost {
    ($($ty:ident),+ $(,)?) => {
        $(
            impl GetSpan for $ty<'_, '_> {
                #[inline]
                fn span(&self) -> Span {
                    self.span
                }
            }

            impl SetSpan for $ty<'_, '_> {
                #[inline]
                fn set_span(&mut self, span: Span) {
                    self.span = span;
                }
            }
        )+
    };
}

impl_spanned!(
    Composes,
    FontFaceRule,
    FontPaletteValuesRule,
    FontFeatureValuesRule,
    FontFeatureSubrule,
    CharsetRule,
    NamespaceRule,
    CustomMediaRule,
    LayerStatementRule,
    PropertyRule,
    ViewTransitionRule,
    UnknownAtRule,
    Url,
    ImportRule,
);

impl_spanned_ghost!(
    SupportsRule,
    KeyframesRule,
    PageRule,
    PageMarginRule,
    CounterStyleRule,
    MozDocumentRule,
    NestingRule,
    NestedDeclarationsRule,
    ViewportRule,
    LayerBlockRule,
    ContainerRule,
    ScopeRule,
    StartingStyleRule,
    PositionTryRule,
    MediaRule,
    StyleRule,
);
