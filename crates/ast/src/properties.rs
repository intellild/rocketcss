use super::*;

use bitflags::bitflags;

macro_rules! property_id_pattern {
    ($name:path) => {
        $name
    };
    ($name:path, $vendor_prefix:ty) => {
        $name(_)
    };
}

macro_rules! property_id_prefix_pattern {
    ($name:path, $binding:ident) => {
        $name
    };
    ($name:path, $vendor_prefix:ty, $binding:ident) => {
        $name($binding)
    };
}

macro_rules! property_id_prefix {
    () => {
        VendorPrefix::NONE
    };
    ($prefix:ident: $vendor_prefix:ty) => {
        *$prefix
    };
}

macro_rules! property_id_with_vendor_prefix {
    ($name:path, $prefix:expr) => {
        None
    };
    ($name:path, $prefix:expr, $vendor_prefix:ty) => {
        Some($name($prefix))
    };
}

macro_rules! declaration_pattern {
    ($name:path, $value:ident) => {
        $name($value)
    };
    ($name:path, $value:ident, $binding:ident: $vendor_prefix:ty) => {
        $name($value, $binding)
    };
}

macro_rules! declaration_prefix {
    () => {
        VendorPrefix::NONE
    };
    ($binding:ident: $vendor_prefix:ty) => {
        *$binding
    };
}

macro_rules! declaration_property_id {
    ($name:path) => {
        $name
    };
    ($name:path, $binding:ident: $vendor_prefix:ty) => {
        $name(*$binding)
    };
}

macro_rules! define_properties {
    (
        $(
            $(#[$meta:meta])*
            $name:literal: $property:ident($value:ty $(, $vp:ty)?),
        )+
    ) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Visit)]
        pub enum PropertyId<'a> {
            $(
                $(#[$meta])*
                $property$(($vp))?,
            )+
            Unparsed,
            Custom(Atom<'a>),
        }

        // Generated from the same source as `PropertyId`, so compact lookup IDs
        // cannot drift from the known property variants. This enum is only used
        // to assign compile-time discriminants and is never stored in the AST.
        #[repr(u32)]
        enum KnownPropertyDiscriminant {
            $($property,)+
        }

        #[derive(Debug, PartialEq, Visit)]
        pub enum Declaration<'a> {
            $(
                $(#[$meta])*
                $property($value $(, $vp)?),
            )+
            /// A CSS-wide keyword shared by every known property grammar.
            CSSWide(std::boxed::Box<PropertyId<'a>>, CSSWideKeyword),
            Unparsed(std::boxed::Box<UnparsedProperty<'a>>),
            Custom(std::boxed::Box<CustomProperty<'a>>),
            /// Tombstone for a declaration removed by an in-place transform.
            Tombstone,
        }

        impl<'a> PropertyId<'a> {
            /// Resolves a property name while retaining unknown names for lossless parsing.
            pub fn from_name(name: Atom<'a>) -> Self {
                let property_id = match_ignore_ascii_case!(
                    &name,
                    $($name => Some(Self::$property$( (<$vp>::default()) )?),)+
                    _ => None,
                );
                if let Some(property_id) = property_id {
                    return property_id;
                }

                if let Some((prefix, unprefixed_name)) = VendorPrefix::split_from_name(&name) {
                    let property_id = match_ignore_ascii_case!(
                        unprefixed_name,
                        $($name => property_id_with_vendor_prefix!(Self::$property, prefix$(, $vp)?),)+
                        _ => None,
                    );
                    if let Some(property_id) = property_id {
                        return property_id;
                    }
                }

                Self::Custom(name)
            }

            /// Returns the canonical CSS property name.
            pub fn name(&self) -> &str {
                match self {
                    $(property_id_pattern!(Self::$property$(, $vp)?) => $name,)+
                    Self::Unparsed => "",
                    Self::Custom(name) => name,
                }
            }

            /// Returns the vendor prefix associated with this property identifier.
            pub fn vendor_prefix(&self) -> VendorPrefix {
                match self {
                    $(property_id_prefix_pattern!(Self::$property$(, $vp)?, prefix) => property_id_prefix!($(prefix: $vp)?),)+
                    Self::Unparsed | Self::Custom(_) => VendorPrefix::NONE,
                }
            }

            /// Returns the compact discriminant of a known property.
            ///
            /// The value is intended for in-memory lookup tables and is not a stable
            /// serialization format.
            #[inline]
            pub fn known_id(&self) -> Option<u32> {
                self.known_id_and_prefix().map(|(id, _)| id)
            }

            /// Returns the compact discriminant and vendor prefix of a known property.
            ///
            /// The value is intended for in-memory lookup tables and is not a stable
            /// serialization format.
            #[inline]
            pub fn known_id_and_prefix(&self) -> Option<(u32, VendorPrefix)> {
                match self {
                    $(property_id_prefix_pattern!(Self::$property$(, $vp)?, prefix) => Some((KnownPropertyDiscriminant::$property as u32, property_id_prefix!($(prefix: $vp)?))),)+
                    Self::Unparsed | Self::Custom(_) => None,
                }
            }
        }

        impl<'a> Declaration<'a> {
            /// Returns the compact discriminant and vendor prefix of a known declaration.
            ///
            /// This performs one typed dispatch without constructing a `PropertyId`.
            #[inline]
            pub fn known_id_and_prefix(&self) -> Option<(u32, VendorPrefix)> {
                match self {
                    $(declaration_pattern!(Self::$property, _value$(, vendor_prefix: $vp)?) => Some((KnownPropertyDiscriminant::$property as u32, declaration_prefix!($(vendor_prefix: $vp)?))),)+
                    Self::CSSWide(property_id, _) => property_id.known_id_and_prefix(),
                    Self::Unparsed(value) => value.property_id.known_id_and_prefix(),
                    Self::Custom(_) | Self::Tombstone => None,
                }
            }

            /// Returns the typed identity of this declaration.
            #[inline]
            pub fn property_id(&self) -> Option<PropertyId<'a>> {
                match self {
                    $(declaration_pattern!(Self::$property, _value$(, vendor_prefix: $vp)?) => Some(declaration_property_id!(PropertyId::$property$(, vendor_prefix: $vp)?)),)+
                    Self::CSSWide(property_id, _) => Some((**property_id).clone()),
                    Self::Unparsed(value) => Some((*value.property_id).clone()),
                    Self::Custom(value) => Some(PropertyId::Custom(match &*value.name {
                        CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name.clone(),
                    })),
                    Self::Tombstone => None,
                }
            }

            /// Returns the canonical CSS property name.
            pub fn name(&self) -> &str {
                match self {
                    $(Self::$property(..) => $name,)+
                    Self::CSSWide(property_id, _) => property_id.name(),
                    Self::Unparsed(value) => value.property_id.name(),
                    Self::Custom(value) => match &*value.name {
                        CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
                    },
                    Self::Tombstone => "",
                }
            }

            /// Returns the vendor prefix associated with this declaration.
            pub fn vendor_prefix(&self) -> VendorPrefix {
                match self {
                    $(declaration_pattern!(Self::$property, _value$(, vendor_prefix: $vp)?) => declaration_prefix!($(vendor_prefix: $vp)?),)+
                    Self::CSSWide(property_id, _) => property_id.vendor_prefix(),
                    Self::Unparsed(value) => value.property_id.vendor_prefix(),
                    Self::Custom(_) | Self::Tombstone => VendorPrefix::NONE,
                }
            }

            /// Returns whether this declaration slot is an in-place tombstone.
            #[inline]
            pub fn is_tombstone(&self) -> bool {
                matches!(self, Self::Tombstone)
            }
        }

        impl EqIgnoringTombstones for Declaration<'_> {
            fn eq_ignoring_tombstones(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::FontFamily(left), Self::FontFamily(right)) => {
                        left.eq_ignoring_tombstones(right)
                    }
                    _ => self == other,
                }
            }
        }
    };
}

bitflags! {
    /// One or more vendor prefixes attached to a property or rule.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct VendorPrefix: u8 {
        const NONE = 0b0000_0001;
        const WEBKIT = 0b0000_0010;
        const MOZ = 0b0000_0100;
        const MS = 0b0000_1000;
        const O = 0b0001_0000;
    }
}

impl Default for VendorPrefix {
    fn default() -> Self {
        Self::NONE
    }
}

impl VendorPrefix {
    fn split_from_name(name: &str) -> Option<(Self, &str)> {
        [
            (Self::WEBKIT, "-webkit-"),
            (Self::MOZ, "-moz-"),
            (Self::MS, "-ms-"),
            (Self::O, "-o-"),
        ]
        .into_iter()
        .find_map(|(prefix, value)| {
            strip_prefix_ignore_ascii_case(name, value).map(|name| (prefix, name))
        })
    }
}

fn strip_prefix_ignore_ascii_case<'a>(value: &'a str, prefix: &str) -> Option<&'a str> {
    value
        .get(..prefix.len())
        .filter(|value| value.eq_ignore_ascii_case(prefix))?;
    value.get(prefix.len()..)
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    PlusDarker,
    PlusLighter,
}

#[macro_export]
macro_rules! for_each_property {
    ($macro:ident) => {
        $macro! {
    "all": All(CSSWideKeyword),
    "background-color": BackgroundColor(std::boxed::Box<CssColor<'a>>),
    "background-image": BackgroundImage(std::vec::Vec<Image<'a>>),
    "background-position-x": BackgroundPositionX(std::vec::Vec<PositionComponent<HorizontalPositionKeyword>>),
    "background-position-y": BackgroundPositionY(std::vec::Vec<PositionComponent<VerticalPositionKeyword>>),
    "background-position": BackgroundPosition(std::vec::Vec<BackgroundPosition>),
    "background-size": BackgroundSize(std::vec::Vec<BackgroundSize>),
    "background-repeat": BackgroundRepeat(std::vec::Vec<BackgroundRepeat>),
    "background-attachment": BackgroundAttachment(std::vec::Vec<BackgroundAttachment>),
    "background-clip": BackgroundClip(std::vec::Vec<BackgroundClip>, VendorPrefix),
    "background-origin": BackgroundOrigin(std::vec::Vec<BackgroundOrigin>),
    "background": Background(std::vec::Vec<Background<'a>>),
    "box-shadow": BoxShadow(std::vec::Vec<BoxShadow<'a>>, VendorPrefix),
    "opacity": Opacity(f32),
    "color": Color(std::boxed::Box<CssColor<'a>>),
    "display": Display(Display),
    "visibility": Visibility(Visibility),
    "width": Width(std::boxed::Box<Size<'a>>),
    "height": Height(std::boxed::Box<Size<'a>>),
    "min-width": MinWidth(std::boxed::Box<Size<'a>>),
    "min-height": MinHeight(std::boxed::Box<Size<'a>>),
    "max-width": MaxWidth(std::boxed::Box<MaxSize<'a>>),
    "max-height": MaxHeight(std::boxed::Box<MaxSize<'a>>),
    "block-size": BlockSize(std::boxed::Box<Size<'a>>),
    "inline-size": InlineSize(std::boxed::Box<Size<'a>>),
    "min-block-size": MinBlockSize(std::boxed::Box<Size<'a>>),
    "min-inline-size": MinInlineSize(std::boxed::Box<Size<'a>>),
    "max-block-size": MaxBlockSize(std::boxed::Box<MaxSize<'a>>),
    "max-inline-size": MaxInlineSize(std::boxed::Box<MaxSize<'a>>),
    "box-sizing": BoxSizing(BoxSizing, VendorPrefix),
    "aspect-ratio": AspectRatio(AspectRatio),
    "overflow": Overflow(Overflow),
    "overflow-x": OverflowX(OverflowKeyword),
    "overflow-y": OverflowY(OverflowKeyword),
    "text-overflow": TextOverflow(TextOverflow, VendorPrefix),
    "position": Position(std::boxed::Box<PositionProperty>),
    "top": Top(std::boxed::Box<LengthPercentageOrAuto>),
    "bottom": Bottom(std::boxed::Box<LengthPercentageOrAuto>),
    "left": Left(std::boxed::Box<LengthPercentageOrAuto>),
    "right": Right(std::boxed::Box<LengthPercentageOrAuto>),
    "inset-block-start": InsetBlockStart(std::boxed::Box<LengthPercentageOrAuto>),
    "inset-block-end": InsetBlockEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "inset-inline-start": InsetInlineStart(std::boxed::Box<LengthPercentageOrAuto>),
    "inset-inline-end": InsetInlineEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "inset-block": InsetBlock(std::boxed::Box<InsetBlock>),
    "inset-inline": InsetInline(std::boxed::Box<InsetInline>),
    "inset": Inset(std::boxed::Box<Inset>),
    "border-spacing": BorderSpacing(std::boxed::Box<Size2D<Length>>),
    "border-top-color": BorderTopColor(std::boxed::Box<CssColor<'a>>),
    "border-bottom-color": BorderBottomColor(std::boxed::Box<CssColor<'a>>),
    "border-left-color": BorderLeftColor(std::boxed::Box<CssColor<'a>>),
    "border-right-color": BorderRightColor(std::boxed::Box<CssColor<'a>>),
    "border-block-start-color": BorderBlockStartColor(std::boxed::Box<CssColor<'a>>),
    "border-block-end-color": BorderBlockEndColor(std::boxed::Box<CssColor<'a>>),
    "border-inline-start-color": BorderInlineStartColor(std::boxed::Box<CssColor<'a>>),
    "border-inline-end-color": BorderInlineEndColor(std::boxed::Box<CssColor<'a>>),
    "border-top-style": BorderTopStyle(LineStyle),
    "border-bottom-style": BorderBottomStyle(LineStyle),
    "border-left-style": BorderLeftStyle(LineStyle),
    "border-right-style": BorderRightStyle(LineStyle),
    "border-block-start-style": BorderBlockStartStyle(LineStyle),
    "border-block-end-style": BorderBlockEndStyle(LineStyle),
    "border-inline-start-style": BorderInlineStartStyle(LineStyle),
    "border-inline-end-style": BorderInlineEndStyle(LineStyle),
    "border-top-width": BorderTopWidth(std::boxed::Box<BorderSideWidth>),
    "border-bottom-width": BorderBottomWidth(std::boxed::Box<BorderSideWidth>),
    "border-left-width": BorderLeftWidth(std::boxed::Box<BorderSideWidth>),
    "border-right-width": BorderRightWidth(std::boxed::Box<BorderSideWidth>),
    "border-block-start-width": BorderBlockStartWidth(std::boxed::Box<BorderSideWidth>),
    "border-block-end-width": BorderBlockEndWidth(std::boxed::Box<BorderSideWidth>),
    "border-inline-start-width": BorderInlineStartWidth(std::boxed::Box<BorderSideWidth>),
    "border-inline-end-width": BorderInlineEndWidth(std::boxed::Box<BorderSideWidth>),
    "border-top-left-radius": BorderTopLeftRadius(std::boxed::Box<Size2D<LengthPercentage>>, VendorPrefix),
    "border-top-right-radius": BorderTopRightRadius(std::boxed::Box<Size2D<LengthPercentage>>, VendorPrefix),
    "border-bottom-left-radius": BorderBottomLeftRadius(std::boxed::Box<Size2D<LengthPercentage>>, VendorPrefix),
    "border-bottom-right-radius": BorderBottomRightRadius(std::boxed::Box<Size2D<LengthPercentage>>, VendorPrefix),
    "border-start-start-radius": BorderStartStartRadius(std::boxed::Box<Size2D<LengthPercentage>>),
    "border-start-end-radius": BorderStartEndRadius(std::boxed::Box<Size2D<LengthPercentage>>),
    "border-end-start-radius": BorderEndStartRadius(std::boxed::Box<Size2D<LengthPercentage>>),
    "border-end-end-radius": BorderEndEndRadius(std::boxed::Box<Size2D<LengthPercentage>>),
    "border-radius": BorderRadius(std::boxed::Box<BorderRadius>, VendorPrefix),
    "border-image-source": BorderImageSource(std::boxed::Box<Image<'a>>),
    "border-image-outset": BorderImageOutset(std::boxed::Box<Rect<LengthOrNumber>>),
    "border-image-repeat": BorderImageRepeat(BorderImageRepeat),
    "border-image-width": BorderImageWidth(std::boxed::Box<Rect<BorderImageSideWidth>>),
    "border-image-slice": BorderImageSlice(std::boxed::Box<BorderImageSlice>),
    "border-image": BorderImage(std::boxed::Box<BorderImage<'a>>, VendorPrefix),
    "border-color": BorderColor(std::boxed::Box<BorderColor<'a>>),
    "border-style": BorderStyle(std::boxed::Box<BorderStyle>),
    "border-width": BorderWidth(std::boxed::Box<BorderWidth>),
    "border-block-color": BorderBlockColor(std::boxed::Box<BorderBlockColor<'a>>),
    "border-block-style": BorderBlockStyle(std::boxed::Box<BorderBlockStyle>),
    "border-block-width": BorderBlockWidth(std::boxed::Box<BorderBlockWidth>),
    "border-inline-color": BorderInlineColor(std::boxed::Box<BorderInlineColor<'a>>),
    "border-inline-style": BorderInlineStyle(std::boxed::Box<BorderInlineStyle>),
    "border-inline-width": BorderInlineWidth(std::boxed::Box<BorderInlineWidth>),
    "border": Border(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-top": BorderTop(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-bottom": BorderBottom(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-left": BorderLeft(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-right": BorderRight(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-block": BorderBlock(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-block-start": BorderBlockStart(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-block-end": BorderBlockEnd(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-inline": BorderInline(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-inline-start": BorderInlineStart(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "border-inline-end": BorderInlineEnd(std::boxed::Box<GenericBorder<'a, LineStyle>>),
    "outline": Outline(std::boxed::Box<GenericBorder<'a, OutlineStyle>>),
    "outline-color": OutlineColor(std::boxed::Box<CssColor<'a>>),
    "outline-style": OutlineStyle(OutlineStyle),
    "outline-width": OutlineWidth(std::boxed::Box<BorderSideWidth>),
    "flex-direction": FlexDirection(FlexDirection, VendorPrefix),
    "flex-wrap": FlexWrap(FlexWrap, VendorPrefix),
    "flex-flow": FlexFlow(std::boxed::Box<FlexFlow>, VendorPrefix),
    "flex-grow": FlexGrow(f32, VendorPrefix),
    "flex-shrink": FlexShrink(f32, VendorPrefix),
    "flex-basis": FlexBasis(std::boxed::Box<LengthPercentageOrAuto>, VendorPrefix),
    "flex": Flex(std::boxed::Box<Flex>, VendorPrefix),
    "order": Order(f32, VendorPrefix),
    "align-content": AlignContent(AlignContent, VendorPrefix),
    "justify-content": JustifyContent(JustifyContent, VendorPrefix),
    "place-content": PlaceContent(PlaceContent),
    "align-self": AlignSelf(AlignSelf, VendorPrefix),
    "justify-self": JustifySelf(JustifySelf),
    "place-self": PlaceSelf(PlaceSelf),
    "align-items": AlignItems(AlignItems, VendorPrefix),
    "justify-items": JustifyItems(JustifyItems),
    "place-items": PlaceItems(PlaceItems),
    "row-gap": RowGap(std::boxed::Box<GapValue>),
    "column-gap": ColumnGap(std::boxed::Box<GapValue>),
    "gap": Gap(std::boxed::Box<Gap>),
    "column-rule": ColumnRule(std::boxed::Box<ColumnRule<'a>>, VendorPrefix),
    "column-width": ColumnWidth(CSSWideOr<ColumnWidth>, VendorPrefix),
    "column-count": ColumnCount(CSSWideOr<ColumnCount>, VendorPrefix),
    "columns": Columns(CSSWideOr<std::boxed::Box<Columns>>, VendorPrefix),
    "grid-column-gap": GridColumnGap(std::boxed::Box<GapValue>),
    "grid-row-gap": GridRowGap(std::boxed::Box<GapValue>),
    "box-orient": BoxOrient(BoxOrient, VendorPrefix),
    "box-direction": BoxDirection(BoxDirection, VendorPrefix),
    "box-ordinal-group": BoxOrdinalGroup(f32, VendorPrefix),
    "box-align": BoxAlign(BoxAlign, VendorPrefix),
    "box-flex": BoxFlex(f32, VendorPrefix),
    "box-flex-group": BoxFlexGroup(f32, VendorPrefix),
    "box-pack": BoxPack(BoxPack, VendorPrefix),
    "box-lines": BoxLines(BoxLines, VendorPrefix),
    "flex-pack": FlexPack(FlexPack, VendorPrefix),
    "flex-order": FlexOrder(f32, VendorPrefix),
    "flex-align": FlexAlign(BoxAlign, VendorPrefix),
    "flex-item-align": FlexItemAlign(FlexItemAlign, VendorPrefix),
    "flex-line-pack": FlexLinePack(FlexLinePack, VendorPrefix),
    "flex-positive": FlexPositive(f32, VendorPrefix),
    "flex-negative": FlexNegative(f32, VendorPrefix),
    "flex-preferred-size": FlexPreferredSize(std::boxed::Box<LengthPercentageOrAuto>, VendorPrefix),
    "grid-template-columns": GridTemplateColumns(std::boxed::Box<TrackSizing<'a>>),
    "grid-template-rows": GridTemplateRows(std::boxed::Box<TrackSizing<'a>>),
    "grid-auto-columns": GridAutoColumns(std::vec::Vec<TrackSize>),
    "grid-auto-rows": GridAutoRows(std::vec::Vec<TrackSize>),
    "grid-auto-flow": GridAutoFlow(GridAutoFlow),
    "grid-template-areas": GridTemplateAreas(std::boxed::Box<GridTemplateAreas<'a>>),
    "grid-template": GridTemplate(std::boxed::Box<GridTemplate<'a>>),
    "grid": Grid(std::boxed::Box<Grid<'a>>),
    "grid-row-start": GridRowStart(std::boxed::Box<GridLine<'a>>),
    "grid-row-end": GridRowEnd(std::boxed::Box<GridLine<'a>>),
    "grid-column-start": GridColumnStart(std::boxed::Box<GridLine<'a>>),
    "grid-column-end": GridColumnEnd(std::boxed::Box<GridLine<'a>>),
    "grid-row": GridRow(std::boxed::Box<GridRow<'a>>),
    "grid-column": GridColumn(std::boxed::Box<GridColumn<'a>>),
    "grid-area": GridArea(std::boxed::Box<GridArea<'a>>),
    "margin-top": MarginTop(std::boxed::Box<LengthPercentageOrAuto>),
    "margin-bottom": MarginBottom(std::boxed::Box<LengthPercentageOrAuto>),
    "margin-left": MarginLeft(std::boxed::Box<LengthPercentageOrAuto>),
    "margin-right": MarginRight(std::boxed::Box<LengthPercentageOrAuto>),
    "margin-block-start": MarginBlockStart(std::boxed::Box<LengthPercentageOrAuto>),
    "margin-block-end": MarginBlockEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "margin-inline-start": MarginInlineStart(std::boxed::Box<LengthPercentageOrAuto>),
    "margin-inline-end": MarginInlineEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "margin-block": MarginBlock(std::boxed::Box<MarginBlock>),
    "margin-inline": MarginInline(std::boxed::Box<MarginInline>),
    "margin": Margin(std::boxed::Box<Margin>),
    "padding-top": PaddingTop(std::boxed::Box<LengthPercentageOrAuto>),
    "padding-bottom": PaddingBottom(std::boxed::Box<LengthPercentageOrAuto>),
    "padding-left": PaddingLeft(std::boxed::Box<LengthPercentageOrAuto>),
    "padding-right": PaddingRight(std::boxed::Box<LengthPercentageOrAuto>),
    "padding-block-start": PaddingBlockStart(std::boxed::Box<LengthPercentageOrAuto>),
    "padding-block-end": PaddingBlockEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "padding-inline-start": PaddingInlineStart(std::boxed::Box<LengthPercentageOrAuto>),
    "padding-inline-end": PaddingInlineEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "padding-block": PaddingBlock(std::boxed::Box<PaddingBlock>),
    "padding-inline": PaddingInline(std::boxed::Box<PaddingInline>),
    "padding": Padding(std::boxed::Box<Padding>),
    "scroll-margin-top": ScrollMarginTop(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-margin-bottom": ScrollMarginBottom(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-margin-left": ScrollMarginLeft(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-margin-right": ScrollMarginRight(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-margin-block-start": ScrollMarginBlockStart(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-margin-block-end": ScrollMarginBlockEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-margin-inline-start": ScrollMarginInlineStart(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-margin-inline-end": ScrollMarginInlineEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-margin-block": ScrollMarginBlock(std::boxed::Box<ScrollMarginBlock>),
    "scroll-margin-inline": ScrollMarginInline(std::boxed::Box<ScrollMarginInline>),
    "scroll-margin": ScrollMargin(std::boxed::Box<ScrollMargin>),
    "scroll-padding-top": ScrollPaddingTop(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-padding-bottom": ScrollPaddingBottom(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-padding-left": ScrollPaddingLeft(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-padding-right": ScrollPaddingRight(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-padding-block-start": ScrollPaddingBlockStart(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-padding-block-end": ScrollPaddingBlockEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-padding-inline-start": ScrollPaddingInlineStart(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-padding-inline-end": ScrollPaddingInlineEnd(std::boxed::Box<LengthPercentageOrAuto>),
    "scroll-padding-block": ScrollPaddingBlock(std::boxed::Box<ScrollPaddingBlock>),
    "scroll-padding-inline": ScrollPaddingInline(std::boxed::Box<ScrollPaddingInline>),
    "scroll-padding": ScrollPadding(std::boxed::Box<ScrollPadding>),
    "font-weight": FontWeight(FontWeight),
    "font-size": FontSize(std::boxed::Box<FontSize>),
    "font-stretch": FontStretch(FontStretch),
    "font-family": FontFamily(std::vec::Vec<FontFamily<'a>>),
    "font-style": FontStyle(FontStyle),
    "font-variant-caps": FontVariantCaps(FontVariantCaps),
    "line-height": LineHeight(std::boxed::Box<LineHeight>),
    "font": Font(std::boxed::Box<Font<'a>>),
    "vertical-align": VerticalAlign(std::boxed::Box<VerticalAlign>),
    "font-palette": FontPalette(std::boxed::Box<DashedIdentReference<'a>>),
    "transition-property": TransitionProperty(std::vec::Vec<PropertyId<'a>>, VendorPrefix),
    "transition-duration": TransitionDuration(std::vec::Vec<Time>, VendorPrefix),
    "transition-delay": TransitionDelay(std::vec::Vec<Time>, VendorPrefix),
    "transition-timing-function": TransitionTimingFunction(std::vec::Vec<EasingFunction>, VendorPrefix),
    "transition": Transition(std::vec::Vec<Transition<'a>>, VendorPrefix),
    "animation-name": AnimationName(std::vec::Vec<AnimationName<'a>>, VendorPrefix),
    "animation-duration": AnimationDuration(std::vec::Vec<Time>, VendorPrefix),
    "animation-timing-function": AnimationTimingFunction(std::vec::Vec<EasingFunction>, VendorPrefix),
    "animation-iteration-count": AnimationIterationCount(std::vec::Vec<AnimationIterationCount>, VendorPrefix),
    "animation-direction": AnimationDirection(std::vec::Vec<AnimationDirection>, VendorPrefix),
    "animation-play-state": AnimationPlayState(std::vec::Vec<AnimationPlayState>, VendorPrefix),
    "animation-delay": AnimationDelay(std::vec::Vec<Time>, VendorPrefix),
    "animation-fill-mode": AnimationFillMode(std::vec::Vec<AnimationFillMode>, VendorPrefix),
    "animation-composition": AnimationComposition(std::vec::Vec<AnimationComposition>),
    "animation-timeline": AnimationTimeline(std::vec::Vec<AnimationTimeline<'a>>),
    "animation-range-start": AnimationRangeStart(std::vec::Vec<AnimationRangeStart>),
    "animation-range-end": AnimationRangeEnd(std::vec::Vec<AnimationRangeEnd>),
    "animation-range": AnimationRange(std::vec::Vec<AnimationRange>),
    "animation": Animation(std::vec::Vec<Animation<'a>>, VendorPrefix),
    "transform": Transform(std::vec::Vec<Transform>, VendorPrefix),
    "transform-origin": TransformOrigin(std::boxed::Box<Position>, VendorPrefix),
    "transform-style": TransformStyle(TransformStyle, VendorPrefix),
    "transform-box": TransformBox(TransformBox),
    "backface-visibility": BackfaceVisibility(BackfaceVisibility, VendorPrefix),
    "perspective": Perspective(std::boxed::Box<Perspective>, VendorPrefix),
    "perspective-origin": PerspectiveOrigin(std::boxed::Box<Position>, VendorPrefix),
    "translate": Translate(std::boxed::Box<Translate>),
    "rotate": Rotate(Rotate),
    "scale": Scale(std::boxed::Box<Scale>),
    "text-transform": TextTransform(TextTransform),
    "white-space": WhiteSpace(WhiteSpace),
    "tab-size": TabSize(std::boxed::Box<LengthOrNumber>, VendorPrefix),
    "word-break": WordBreak(WordBreak),
    "line-break": LineBreak(LineBreak),
    "hyphens": Hyphens(Hyphens, VendorPrefix),
    "overflow-wrap": OverflowWrap(OverflowWrap),
    "word-wrap": WordWrap(OverflowWrap),
    "text-align": TextAlign(TextAlign),
    "text-align-last": TextAlignLast(TextAlignLast, VendorPrefix),
    "text-justify": TextJustify(TextJustify),
    "word-spacing": WordSpacing(std::boxed::Box<Spacing>),
    "letter-spacing": LetterSpacing(std::boxed::Box<Spacing>),
    "text-indent": TextIndent(std::boxed::Box<TextIndent>),
    "text-decoration-line": TextDecorationLine(std::boxed::Box<TextDecorationLine>, VendorPrefix),
    "text-decoration-style": TextDecorationStyle(TextDecorationStyle, VendorPrefix),
    "text-decoration-color": TextDecorationColor(std::boxed::Box<CssColor<'a>>, VendorPrefix),
    "text-decoration-thickness": TextDecorationThickness(std::boxed::Box<TextDecorationThickness>),
    "text-decoration": TextDecoration(std::boxed::Box<TextDecoration<'a>>, VendorPrefix),
    "text-decoration-skip-ink": TextDecorationSkipInk(TextDecorationSkipInk, VendorPrefix),
    "text-emphasis-style": TextEmphasisStyle(std::boxed::Box<TextEmphasisStyle<'a>>, VendorPrefix),
    "text-emphasis-color": TextEmphasisColor(std::boxed::Box<CssColor<'a>>, VendorPrefix),
    "text-emphasis": TextEmphasis(std::boxed::Box<TextEmphasis<'a>>, VendorPrefix),
    "text-emphasis-position": TextEmphasisPosition(TextEmphasisPosition, VendorPrefix),
    "text-shadow": TextShadow(std::vec::Vec<TextShadow<'a>>),
    "text-size-adjust": TextSizeAdjust(TextSizeAdjust, VendorPrefix),
    "direction": Direction(TextDirection),
    "unicode-bidi": UnicodeBidi(UnicodeBidi),
    "box-decoration-break": BoxDecorationBreak(BoxDecorationBreak, VendorPrefix),
    "resize": Resize(Resize),
    "cursor": Cursor(std::boxed::Box<Cursor<'a>>),
    "caret-color": CaretColor(std::boxed::Box<ColorOrAuto<'a>>),
    "caret-shape": CaretShape(CaretShape),
    "caret": Caret(std::boxed::Box<Caret<'a>>),
    "user-select": UserSelect(UserSelect, VendorPrefix),
    "accent-color": AccentColor(std::boxed::Box<ColorOrAuto<'a>>),
    "appearance": Appearance(std::boxed::Box<Appearance<'a>>, VendorPrefix),
    "list-style-type": ListStyleType(std::boxed::Box<ListStyleType<'a>>),
    "list-style-image": ListStyleImage(std::boxed::Box<Image<'a>>),
    "list-style-position": ListStylePosition(ListStylePosition),
    "list-style": ListStyle(std::boxed::Box<ListStyle<'a>>),
    "marker-side": MarkerSide(MarkerSide),
    "composes": Composes(std::boxed::Box<Composes<'a>>),
    "fill": Fill(std::boxed::Box<SVGPaint<'a>>),
    "fill-rule": FillRule(FillRule),
    "fill-opacity": FillOpacity(f32),
    "stroke": Stroke(std::boxed::Box<SVGPaint<'a>>),
    "stroke-opacity": StrokeOpacity(f32),
    "stroke-width": StrokeWidth(std::boxed::Box<LengthPercentage>),
    "stroke-linecap": StrokeLinecap(StrokeLinecap),
    "stroke-linejoin": StrokeLinejoin(StrokeLinejoin),
    "stroke-miterlimit": StrokeMiterlimit(f32),
    "stroke-dasharray": StrokeDasharray(std::boxed::Box<StrokeDasharray>),
    "stroke-dashoffset": StrokeDashoffset(std::boxed::Box<LengthPercentage>),
    "marker-start": MarkerStart(std::boxed::Box<Marker<'a>>),
    "marker-mid": MarkerMid(std::boxed::Box<Marker<'a>>),
    "marker-end": MarkerEnd(std::boxed::Box<Marker<'a>>),
    "marker": Marker(std::boxed::Box<Marker<'a>>),
    "color-interpolation": ColorInterpolation(ColorInterpolation),
    "color-interpolation-filters": ColorInterpolationFilters(ColorInterpolation),
    "color-rendering": ColorRendering(ColorRendering),
    "shape-rendering": ShapeRendering(ShapeRendering),
    "text-rendering": TextRendering(TextRendering),
    "image-rendering": ImageRendering(ImageRendering),
    "clip-path": ClipPath(std::boxed::Box<ClipPath<'a>>, VendorPrefix),
    "clip-rule": ClipRule(FillRule),
    "mask-image": MaskImage(std::vec::Vec<Image<'a>>, VendorPrefix),
    "mask-mode": MaskMode(std::vec::Vec<MaskMode>),
    "mask-repeat": MaskRepeat(std::vec::Vec<BackgroundRepeat>, VendorPrefix),
    "mask-position-x": MaskPositionX(std::vec::Vec<PositionComponent<HorizontalPositionKeyword>>),
    "mask-position-y": MaskPositionY(std::vec::Vec<PositionComponent<VerticalPositionKeyword>>),
    "mask-position": MaskPosition(std::vec::Vec<Position>, VendorPrefix),
    "mask-clip": MaskClip(std::vec::Vec<MaskClip>, VendorPrefix),
    "mask-origin": MaskOrigin(std::vec::Vec<GeometryBox>, VendorPrefix),
    "mask-size": MaskSize(std::vec::Vec<BackgroundSize>, VendorPrefix),
    "mask-composite": MaskComposite(std::vec::Vec<MaskComposite>),
    "mask-type": MaskType(MaskType),
    "mask": Mask(std::vec::Vec<Mask<'a>>, VendorPrefix),
    "mask-border-source": MaskBorderSource(std::boxed::Box<Image<'a>>),
    "mask-border-mode": MaskBorderMode(MaskBorderMode),
    "mask-border-slice": MaskBorderSlice(std::boxed::Box<BorderImageSlice>),
    "mask-border-width": MaskBorderWidth(std::boxed::Box<Rect<BorderImageSideWidth>>),
    "mask-border-outset": MaskBorderOutset(std::boxed::Box<Rect<LengthOrNumber>>),
    "mask-border-repeat": MaskBorderRepeat(BorderImageRepeat),
    "mask-border": MaskBorder(std::boxed::Box<MaskBorder<'a>>),
    "-webkit-mask-composite": WebKitMaskComposite(std::vec::Vec<WebKitMaskComposite>),
    "mask-source-type": WebKitMaskSourceType(std::vec::Vec<WebKitMaskSourceType>, VendorPrefix),
    "mask-box-image": WebKitMaskBoxImage(std::boxed::Box<BorderImage<'a>>, VendorPrefix),
    "mask-box-image-source": WebKitMaskBoxImageSource(std::boxed::Box<Image<'a>>, VendorPrefix),
    "mask-box-image-slice": WebKitMaskBoxImageSlice(std::boxed::Box<BorderImageSlice>, VendorPrefix),
    "mask-box-image-width": WebKitMaskBoxImageWidth(std::boxed::Box<Rect<BorderImageSideWidth>>, VendorPrefix),
    "mask-box-image-outset": WebKitMaskBoxImageOutset(std::boxed::Box<Rect<LengthOrNumber>>, VendorPrefix),
    "mask-box-image-repeat": WebKitMaskBoxImageRepeat(BorderImageRepeat, VendorPrefix),
    "filter": Filter(std::boxed::Box<FilterList<'a>>, VendorPrefix),
    "backdrop-filter": BackdropFilter(std::boxed::Box<FilterList<'a>>, VendorPrefix),
    "mix-blend-mode": MixBlendMode(BlendMode),
    "z-index": ZIndex(ZIndex),
    "container-type": ContainerType(ContainerType),
    "container-name": ContainerName(std::boxed::Box<ContainerNameList<'a>>),
    "container": Container(std::boxed::Box<Container<'a>>),
    "view-transition-name": ViewTransitionName(std::boxed::Box<ViewTransitionName<'a>>),
    "view-transition-class": ViewTransitionClass(std::boxed::Box<NoneOrCustomIdentList<'a>>),
    "view-transition-group": ViewTransitionGroup(std::boxed::Box<ViewTransitionGroup<'a>>),
    "color-scheme": ColorScheme(ColorScheme),
    "print-color-adjust": PrintColorAdjust(PrintColorAdjust, VendorPrefix),
        }
    };
}

for_each_property!(define_properties);
