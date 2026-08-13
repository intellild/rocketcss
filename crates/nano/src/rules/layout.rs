use rocketcss_ast::{Declaration, Margin, Padding, PropertyId};
use rocketcss_common::boxed::Box;

pub(crate) const ALL_BOX_SIDES: u8 = 0b1111;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum BoxFamily {
    Margin,
    Padding,
}

impl BoxFamily {
    pub(crate) const COUNT: usize = 2;

    #[inline]
    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BoxProperty {
    Shorthand(BoxFamily),
    Longhand(BoxFamily, usize),
    Barrier(BoxFamily),
    BarrierAll,
}

impl BoxProperty {
    #[inline]
    pub(crate) const fn family(self) -> Option<BoxFamily> {
        match self {
            Self::Shorthand(family) | Self::Longhand(family, _) | Self::Barrier(family) => {
                Some(family)
            }
            Self::BarrierAll => None,
        }
    }
}

/// Classifies only relationships that are safe to reason about without layout
/// direction, compatibility, or fallback information.
#[inline]
pub(crate) fn typed_box_property(declaration: &Declaration<'_>) -> Option<BoxProperty> {
    Some(match declaration {
        Declaration::Margin(..) => BoxProperty::Shorthand(BoxFamily::Margin),
        Declaration::MarginTop(..) => BoxProperty::Longhand(BoxFamily::Margin, 0),
        Declaration::MarginRight(..) => BoxProperty::Longhand(BoxFamily::Margin, 1),
        Declaration::MarginBottom(..) => BoxProperty::Longhand(BoxFamily::Margin, 2),
        Declaration::MarginLeft(..) => BoxProperty::Longhand(BoxFamily::Margin, 3),
        Declaration::Padding(..) => BoxProperty::Shorthand(BoxFamily::Padding),
        Declaration::PaddingTop(..) => BoxProperty::Longhand(BoxFamily::Padding, 0),
        Declaration::PaddingRight(..) => BoxProperty::Longhand(BoxFamily::Padding, 1),
        Declaration::PaddingBottom(..) => BoxProperty::Longhand(BoxFamily::Padding, 2),
        Declaration::PaddingLeft(..) => BoxProperty::Longhand(BoxFamily::Padding, 3),
        _ => return None,
    })
}

#[inline]
pub(crate) fn box_property(declaration: &Declaration<'_>) -> Option<BoxProperty> {
    if let Some(property) = typed_box_property(declaration) {
        return Some(property);
    }
    let property_id = match declaration {
        Declaration::All(..) => return Some(BoxProperty::BarrierAll),
        Declaration::CSSWide(property_id, _) => &**property_id,
        Declaration::Unparsed(value) => &*value.property_id,
        _ => return None,
    };
    match property_id {
        PropertyId::Margin => Some(BoxProperty::Shorthand(BoxFamily::Margin)),
        PropertyId::MarginTop => Some(BoxProperty::Longhand(BoxFamily::Margin, 0)),
        PropertyId::MarginRight => Some(BoxProperty::Longhand(BoxFamily::Margin, 1)),
        PropertyId::MarginBottom => Some(BoxProperty::Longhand(BoxFamily::Margin, 2)),
        PropertyId::MarginLeft => Some(BoxProperty::Longhand(BoxFamily::Margin, 3)),
        PropertyId::MarginBlockStart
        | PropertyId::MarginBlockEnd
        | PropertyId::MarginInlineStart
        | PropertyId::MarginInlineEnd
        | PropertyId::MarginBlock
        | PropertyId::MarginInline => Some(BoxProperty::Barrier(BoxFamily::Margin)),
        PropertyId::Padding => Some(BoxProperty::Shorthand(BoxFamily::Padding)),
        PropertyId::PaddingTop => Some(BoxProperty::Longhand(BoxFamily::Padding, 0)),
        PropertyId::PaddingRight => Some(BoxProperty::Longhand(BoxFamily::Padding, 1)),
        PropertyId::PaddingBottom => Some(BoxProperty::Longhand(BoxFamily::Padding, 2)),
        PropertyId::PaddingLeft => Some(BoxProperty::Longhand(BoxFamily::Padding, 3)),
        PropertyId::PaddingBlockStart
        | PropertyId::PaddingBlockEnd
        | PropertyId::PaddingInlineStart
        | PropertyId::PaddingInlineEnd
        | PropertyId::PaddingBlock
        | PropertyId::PaddingInline => Some(BoxProperty::Barrier(BoxFamily::Padding)),
        PropertyId::All => Some(BoxProperty::BarrierAll),
        _ => None,
    }
}

pub(crate) fn materialize_box_longhands<'ast>(
    declaration: Declaration<'ast>,
    family: BoxFamily,
    live_effects: u8,
) -> Option<std::vec::Vec<Declaration<'ast>>> {
    if live_effects == 0 || live_effects & !ALL_BOX_SIDES != 0 {
        return None;
    }
    let mut replacements = std::vec::Vec::with_capacity(live_effects.count_ones() as usize);
    match (family, declaration) {
        (BoxFamily::Margin, Declaration::Margin(value)) => {
            let Margin {
                top,
                right,
                bottom,
                left,
            } = Box::into_inner(value);
            for (side, value) in [top, right, bottom, left].into_iter().enumerate() {
                if live_effects & (1 << side) == 0 {
                    continue;
                }
                replacements.push(match side {
                    0 => Declaration::MarginTop(value),
                    1 => Declaration::MarginRight(value),
                    2 => Declaration::MarginBottom(value),
                    3 => Declaration::MarginLeft(value),
                    _ => unreachable!("a box has exactly four sides"),
                });
            }
        }
        (BoxFamily::Padding, Declaration::Padding(value)) => {
            let Padding {
                top,
                right,
                bottom,
                left,
            } = Box::into_inner(value);
            for (side, value) in [top, right, bottom, left].into_iter().enumerate() {
                if live_effects & (1 << side) == 0 {
                    continue;
                }
                replacements.push(match side {
                    0 => Declaration::PaddingTop(value),
                    1 => Declaration::PaddingRight(value),
                    2 => Declaration::PaddingBottom(value),
                    3 => Declaration::PaddingLeft(value),
                    _ => unreachable!("a box has exactly four sides"),
                });
            }
        }
        _ => return None,
    }
    Some(replacements)
}
