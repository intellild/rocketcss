use std::hash::{Hash, Hasher};

use rocketcss_ast::{Declaration, PropertyId};
use rocketcss_common::{Allocator, prelude::HashMap};
use rustc_hash::FxHasher;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum CompactPropertyKey {
    Known(u32),
    Custom(u32),
}

#[derive(Debug)]
pub(super) struct DeclarationIrClassifier<'arena, 'ast> {
    custom_property_ids: HashMap<'arena, &'ast str, u32>,
}

impl<'arena, 'ast> DeclarationIrClassifier<'arena, 'ast> {
    pub(super) fn with_capacity_in(capacity: usize, allocator: &'arena Allocator) -> Self {
        Self {
            custom_property_ids: HashMap::with_capacity_in(capacity, allocator),
        }
    }

    pub(super) fn property_key(
        &mut self,
        declaration: &Declaration<'ast>,
        important: bool,
    ) -> Option<CompactPropertyKey> {
        if matches!(
            declaration,
            Declaration::Unparsed(_) | Declaration::Tombstone
        ) {
            return None;
        }
        if let Some(key) = CompactPropertyKey::known(declaration, important) {
            return Some(key);
        }
        let PropertyId::Custom(name) = declaration.property_id()? else {
            return None;
        };
        let id = self.intern_custom_property(name);
        let key = id
            .checked_mul(2)
            .and_then(|id| id.checked_add(u32::from(important)))
            .expect("custom property count exceeds compact key capacity");
        Some(CompactPropertyKey::Custom(key))
    }

    pub(super) fn movement_domain(
        &mut self,
        declaration: &Declaration<'ast>,
    ) -> Option<MovementDomain> {
        let property_id = declaration.property_id()?;
        if let PropertyId::Custom(name) = property_id {
            return Some(MovementDomain::Custom(self.intern_custom_property(name)));
        }
        movement_domain(property_id)
    }

    fn intern_custom_property(&mut self, name: &'ast str) -> u32 {
        let vacant_id = u32::try_from(self.custom_property_ids.len())
            .expect("custom property count exceeds u32::MAX");
        *self.custom_property_ids.entry(name).or_insert(vacant_id)
    }
}

impl CompactPropertyKey {
    const IMPORTANT_MASK: u32 = 1;
    const VENDOR_PREFIX_SHIFT: u32 = 1;
    const PROPERTY_ID_SHIFT: u32 = 6;

    fn known(declaration: &Declaration<'_>, important: bool) -> Option<Self> {
        if let Some((property_id, vendor_prefix)) = declaration.known_id_and_prefix() {
            let vendor_prefix = u32::from(vendor_prefix.bits());
            debug_assert!(property_id <= u32::MAX >> Self::PROPERTY_ID_SHIFT);
            debug_assert_eq!(vendor_prefix & !0b1_1111, 0);
            return Some(Self::Known(
                (property_id << Self::PROPERTY_ID_SHIFT)
                    | (vendor_prefix << Self::VENDOR_PREFIX_SHIFT)
                    | u32::from(important),
            ));
        }
        None
    }

    fn is_important(self) -> bool {
        match self {
            Self::Known(key) | Self::Custom(key) => key & Self::IMPORTANT_MASK != 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct PropertyBloom {
    normal: u64,
    important: u64,
}

impl PropertyBloom {
    pub(super) fn insert(&mut self, key: CompactPropertyKey) {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let bits = (1 << (hash & 63)) | (1 << ((hash >> 6) & 63));
        if key.is_important() {
            self.important |= bits;
        } else {
            self.normal |= bits;
        }
    }

    pub(super) fn may_share_declaration(self, other: Self) -> bool {
        self.normal & other.normal != 0 || self.important & other.important != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MovementDomain {
    Custom(u32),
    Background,
    Border,
    Color,
    Display,
    Font,
    Height,
    Margin(u8),
    Mask,
    Opacity,
    Padding(u8),
    Position,
    TextAlign,
    TextDecoration,
    TextTransform,
    Visibility,
    Width,
}

impl MovementDomain {
    pub(super) fn overlaps(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Custom(left), Self::Custom(right)) => left == right,
            (Self::Margin(left), Self::Margin(right))
            | (Self::Padding(left), Self::Padding(right)) => left & right != 0,
            _ => self == other,
        }
    }
}

fn movement_domain(property_id: PropertyId<'_>) -> Option<MovementDomain> {
    use MovementDomain as Domain;
    use PropertyId as Id;

    Some(match property_id {
        Id::Custom(_) => unreachable!("custom properties are interned by the IR store"),
        Id::BackgroundColor
        | Id::BackgroundImage
        | Id::BackgroundPositionX
        | Id::BackgroundPositionY
        | Id::BackgroundPosition
        | Id::BackgroundSize
        | Id::BackgroundRepeat
        | Id::BackgroundAttachment
        | Id::BackgroundClip(_)
        | Id::BackgroundOrigin
        | Id::Background => Domain::Background,
        Id::BorderTopColor
        | Id::BorderBottomColor
        | Id::BorderLeftColor
        | Id::BorderRightColor
        | Id::BorderTopStyle
        | Id::BorderBottomStyle
        | Id::BorderLeftStyle
        | Id::BorderRightStyle
        | Id::BorderTopWidth
        | Id::BorderBottomWidth
        | Id::BorderLeftWidth
        | Id::BorderRightWidth
        | Id::BorderColor
        | Id::BorderStyle
        | Id::BorderWidth
        | Id::Border
        | Id::BorderTop
        | Id::BorderBottom
        | Id::BorderLeft
        | Id::BorderRight => Domain::Border,
        Id::Color => Domain::Color,
        Id::Display => Domain::Display,
        Id::FontWeight
        | Id::FontSize
        | Id::FontStretch
        | Id::FontFamily
        | Id::FontStyle
        | Id::FontVariantCaps
        | Id::LineHeight
        | Id::Font => Domain::Font,
        Id::Height | Id::MinHeight | Id::MaxHeight => Domain::Height,
        Id::MarginTop => Domain::Margin(0b0001),
        Id::MarginRight => Domain::Margin(0b0010),
        Id::MarginBottom => Domain::Margin(0b0100),
        Id::MarginLeft => Domain::Margin(0b1000),
        Id::Margin => Domain::Margin(0b1111),
        Id::MaskImage(_)
        | Id::MaskMode
        | Id::MaskRepeat(_)
        | Id::MaskPositionX
        | Id::MaskPositionY
        | Id::MaskPosition(_)
        | Id::MaskClip(_)
        | Id::MaskOrigin(_)
        | Id::MaskSize(_)
        | Id::MaskComposite
        | Id::MaskType
        | Id::Mask(_) => Domain::Mask,
        Id::Opacity => Domain::Opacity,
        Id::PaddingTop => Domain::Padding(0b0001),
        Id::PaddingRight => Domain::Padding(0b0010),
        Id::PaddingBottom => Domain::Padding(0b0100),
        Id::PaddingLeft => Domain::Padding(0b1000),
        Id::Padding => Domain::Padding(0b1111),
        Id::Position | Id::Top | Id::Right | Id::Bottom | Id::Left => Domain::Position,
        Id::TextAlign => Domain::TextAlign,
        Id::TextDecorationLine(_)
        | Id::TextDecorationStyle(_)
        | Id::TextDecorationColor(_)
        | Id::TextDecorationThickness
        | Id::TextDecoration(_) => Domain::TextDecoration,
        Id::TextTransform => Domain::TextTransform,
        Id::Visibility => Domain::Visibility,
        Id::Width | Id::MinWidth | Id::MaxWidth => Domain::Width,
        _ => return None,
    })
}
