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

macro_rules! define_properties {
    (
        $(
            $(#[$meta:meta])*
            $name:literal: $property:ident($value:ty $(, $vp:ty)?),
        )+
    ) => {
        #[derive(Debug, PartialEq, Visit)]
        pub enum PropertyId<'a> {
            $(
                $(#[$meta])*
                $property$(($vp))?,
            )+
            ColumnRule,
            Columns,
            GridColumnGap,
            GridRowGap,
            All,
            Unparsed,
            Custom(&'a str),
        }

        #[derive(Debug, PartialEq, Visit)]
        pub enum Declaration<'a> {
            $(
                $(#[$meta])*
                $property($value $(, $vp)?),
            )+
            All(CSSWideKeyword),
            Unparsed(Box<'a, UnparsedProperty<'a>>),
            Custom(Box<'a, CustomProperty<'a>>),
            /// Tombstone for a declaration removed by an in-place transform.
            Tombstone,
        }

        impl<'a> PropertyId<'a> {
            /// Resolves a property name while retaining unknown names for lossless parsing.
            pub fn from_name(name: &'a str) -> Self {
                let property_id = match_ignore_ascii_case!(
                    name,
                    "all" => Some(Self::All),
                    $($name => Some(Self::$property$( (<$vp>::default()) )?),)+
                    "column-rule" => Some(Self::ColumnRule),
                    "columns" => Some(Self::Columns),
                    "grid-column-gap" => Some(Self::GridColumnGap),
                    "grid-row-gap" => Some(Self::GridRowGap),
                    _ => None,
                );
                if let Some(property_id) = property_id {
                    return property_id;
                }

                if let Some((prefix, unprefixed_name)) = VendorPrefix::split_from_name(name) {
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
                    Self::ColumnRule => "column-rule",
                    Self::Columns => "columns",
                    Self::GridColumnGap => "grid-column-gap",
                    Self::GridRowGap => "grid-row-gap",
                    Self::All => "all",
                    Self::Unparsed => "",
                    Self::Custom(name) => name,
                }
            }

            /// Returns the vendor prefix associated with this property identifier.
            pub fn vendor_prefix(&self) -> VendorPrefix {
                match self {
                    $(property_id_prefix_pattern!(Self::$property$(, $vp)?, prefix) => property_id_prefix!($(prefix: $vp)?),)+
                    Self::ColumnRule
                    | Self::Columns
                    | Self::GridColumnGap
                    | Self::GridRowGap
                    | Self::All
                    | Self::Unparsed
                    | Self::Custom(_) => VendorPrefix::NONE,
                }
            }
        }

        impl Declaration<'_> {
            /// Returns the canonical CSS property name.
            pub fn name(&self) -> &str {
                match self {
                    $(Self::$property(..) => $name,)+
                    Self::All(_) => "all",
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
                    Self::Unparsed(value) => value.property_id.vendor_prefix(),
                    Self::All(_) | Self::Custom(_) | Self::Tombstone => VendorPrefix::NONE,
                }
            }

            /// Returns whether this declaration slot is an in-place tombstone.
            #[inline]
            pub fn is_tombstone(&self) -> bool {
                matches!(self, Self::Tombstone)
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
    "background-color": BackgroundColor(Box<'a, CssColor<'a>>),
    "background-image": BackgroundImage(Vec<'a, Image<'a>>),
    "background-position-x": BackgroundPositionX(Vec<'a, PositionComponent<'a, HorizontalPositionKeyword>>),
    "background-position-y": BackgroundPositionY(Vec<'a, PositionComponent<'a, VerticalPositionKeyword>>),
    "background-position": BackgroundPosition(Vec<'a, BackgroundPosition<'a>>),
    "background-size": BackgroundSize(Vec<'a, BackgroundSize<'a>>),
    "background-repeat": BackgroundRepeat(Vec<'a, BackgroundRepeat>),
    "background-attachment": BackgroundAttachment(Vec<'a, BackgroundAttachment>),
    "background-clip": BackgroundClip(Vec<'a, BackgroundClip>, VendorPrefix),
    "background-origin": BackgroundOrigin(Vec<'a, BackgroundOrigin>),
    "background": Background(Vec<'a, Background<'a>>),
    "box-shadow": BoxShadow(Vec<'a, BoxShadow<'a>>, VendorPrefix),
    "opacity": Opacity(f32),
    "color": Color(Box<'a, CssColor<'a>>),
    "display": Display(Box<'a, Display<'a>>),
    "visibility": Visibility(Visibility),
    "width": Width(Box<'a, Size<'a>>),
    "height": Height(Box<'a, Size<'a>>),
    "min-width": MinWidth(Box<'a, Size<'a>>),
    "min-height": MinHeight(Box<'a, Size<'a>>),
    "max-width": MaxWidth(Box<'a, MaxSize<'a>>),
    "max-height": MaxHeight(Box<'a, MaxSize<'a>>),
    "block-size": BlockSize(Box<'a, Size<'a>>),
    "inline-size": InlineSize(Box<'a, Size<'a>>),
    "min-block-size": MinBlockSize(Box<'a, Size<'a>>),
    "min-inline-size": MinInlineSize(Box<'a, Size<'a>>),
    "max-block-size": MaxBlockSize(Box<'a, MaxSize<'a>>),
    "max-inline-size": MaxInlineSize(Box<'a, MaxSize<'a>>),
    "box-sizing": BoxSizing(BoxSizing, VendorPrefix),
    "aspect-ratio": AspectRatio(Box<'a, AspectRatio<'a>>),
    "overflow": Overflow(Box<'a, Overflow>),
    "overflow-x": OverflowX(OverflowKeyword),
    "overflow-y": OverflowY(OverflowKeyword),
    "text-overflow": TextOverflow(TextOverflow, VendorPrefix),
    "position": Position(Box<'a, PositionProperty>),
    "top": Top(Box<'a, LengthPercentageOrAuto<'a>>),
    "bottom": Bottom(Box<'a, LengthPercentageOrAuto<'a>>),
    "left": Left(Box<'a, LengthPercentageOrAuto<'a>>),
    "right": Right(Box<'a, LengthPercentageOrAuto<'a>>),
    "inset-block-start": InsetBlockStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "inset-block-end": InsetBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "inset-inline-start": InsetInlineStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "inset-inline-end": InsetInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "inset-block": InsetBlock(Box<'a, InsetBlock<'a>>),
    "inset-inline": InsetInline(Box<'a, InsetInline<'a>>),
    "inset": Inset(Box<'a, Inset<'a>>),
    "border-spacing": BorderSpacing(Box<'a, Size2D<'a, Length<'a>>>),
    "border-top-color": BorderTopColor(Box<'a, CssColor<'a>>),
    "border-bottom-color": BorderBottomColor(Box<'a, CssColor<'a>>),
    "border-left-color": BorderLeftColor(Box<'a, CssColor<'a>>),
    "border-right-color": BorderRightColor(Box<'a, CssColor<'a>>),
    "border-block-start-color": BorderBlockStartColor(Box<'a, CssColor<'a>>),
    "border-block-end-color": BorderBlockEndColor(Box<'a, CssColor<'a>>),
    "border-inline-start-color": BorderInlineStartColor(Box<'a, CssColor<'a>>),
    "border-inline-end-color": BorderInlineEndColor(Box<'a, CssColor<'a>>),
    "border-top-style": BorderTopStyle(LineStyle),
    "border-bottom-style": BorderBottomStyle(LineStyle),
    "border-left-style": BorderLeftStyle(LineStyle),
    "border-right-style": BorderRightStyle(LineStyle),
    "border-block-start-style": BorderBlockStartStyle(LineStyle),
    "border-block-end-style": BorderBlockEndStyle(LineStyle),
    "border-inline-start-style": BorderInlineStartStyle(LineStyle),
    "border-inline-end-style": BorderInlineEndStyle(LineStyle),
    "border-top-width": BorderTopWidth(Box<'a, BorderSideWidth<'a>>),
    "border-bottom-width": BorderBottomWidth(Box<'a, BorderSideWidth<'a>>),
    "border-left-width": BorderLeftWidth(Box<'a, BorderSideWidth<'a>>),
    "border-right-width": BorderRightWidth(Box<'a, BorderSideWidth<'a>>),
    "border-block-start-width": BorderBlockStartWidth(Box<'a, BorderSideWidth<'a>>),
    "border-block-end-width": BorderBlockEndWidth(Box<'a, BorderSideWidth<'a>>),
    "border-inline-start-width": BorderInlineStartWidth(Box<'a, BorderSideWidth<'a>>),
    "border-inline-end-width": BorderInlineEndWidth(Box<'a, BorderSideWidth<'a>>),
    "border-top-left-radius": BorderTopLeftRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix),
    "border-top-right-radius": BorderTopRightRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix),
    "border-bottom-left-radius": BorderBottomLeftRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix),
    "border-bottom-right-radius": BorderBottomRightRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix),
    "border-start-start-radius": BorderStartStartRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>),
    "border-start-end-radius": BorderStartEndRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>),
    "border-end-start-radius": BorderEndStartRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>),
    "border-end-end-radius": BorderEndEndRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>),
    "border-radius": BorderRadius(Box<'a, BorderRadius<'a>>, VendorPrefix),
    "border-image-source": BorderImageSource(Box<'a, Image<'a>>),
    "border-image-outset": BorderImageOutset(Box<'a, Rect<'a, LengthOrNumber<'a>>>),
    "border-image-repeat": BorderImageRepeat(Box<'a, BorderImageRepeat>),
    "border-image-width": BorderImageWidth(Box<'a, Rect<'a, BorderImageSideWidth<'a>>>),
    "border-image-slice": BorderImageSlice(Box<'a, BorderImageSlice<'a>>),
    "border-image": BorderImage(Box<'a, BorderImage<'a>>, VendorPrefix),
    "border-color": BorderColor(Box<'a, BorderColor<'a>>),
    "border-style": BorderStyle(Box<'a, BorderStyle>),
    "border-width": BorderWidth(Box<'a, BorderWidth<'a>>),
    "border-block-color": BorderBlockColor(Box<'a, BorderBlockColor<'a>>),
    "border-block-style": BorderBlockStyle(Box<'a, BorderBlockStyle>),
    "border-block-width": BorderBlockWidth(Box<'a, BorderBlockWidth<'a>>),
    "border-inline-color": BorderInlineColor(Box<'a, BorderInlineColor<'a>>),
    "border-inline-style": BorderInlineStyle(Box<'a, BorderInlineStyle>),
    "border-inline-width": BorderInlineWidth(Box<'a, BorderInlineWidth<'a>>),
    "border": Border(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-top": BorderTop(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-bottom": BorderBottom(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-left": BorderLeft(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-right": BorderRight(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-block": BorderBlock(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-block-start": BorderBlockStart(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-block-end": BorderBlockEnd(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-inline": BorderInline(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-inline-start": BorderInlineStart(Box<'a, GenericBorder<'a, LineStyle>>),
    "border-inline-end": BorderInlineEnd(Box<'a, GenericBorder<'a, LineStyle>>),
    "outline": Outline(Box<'a, GenericBorder<'a, OutlineStyle>>),
    "outline-color": OutlineColor(Box<'a, CssColor<'a>>),
    "outline-style": OutlineStyle(Box<'a, OutlineStyle>),
    "outline-width": OutlineWidth(Box<'a, BorderSideWidth<'a>>),
    "flex-direction": FlexDirection(FlexDirection, VendorPrefix),
    "flex-wrap": FlexWrap(FlexWrap, VendorPrefix),
    "flex-flow": FlexFlow(Box<'a, FlexFlow>, VendorPrefix),
    "flex-grow": FlexGrow(f32, VendorPrefix),
    "flex-shrink": FlexShrink(f32, VendorPrefix),
    "flex-basis": FlexBasis(Box<'a, LengthPercentageOrAuto<'a>>, VendorPrefix),
    "flex": Flex(Box<'a, Flex<'a>>, VendorPrefix),
    "order": Order(f32, VendorPrefix),
    "align-content": AlignContent(Box<'a, AlignContent>, VendorPrefix),
    "justify-content": JustifyContent(Box<'a, JustifyContent>, VendorPrefix),
    "place-content": PlaceContent(Box<'a, PlaceContent<'a>>),
    "align-self": AlignSelf(Box<'a, AlignSelf>, VendorPrefix),
    "justify-self": JustifySelf(Box<'a, JustifySelf>),
    "place-self": PlaceSelf(Box<'a, PlaceSelf<'a>>),
    "align-items": AlignItems(Box<'a, AlignItems>, VendorPrefix),
    "justify-items": JustifyItems(Box<'a, JustifyItems>),
    "place-items": PlaceItems(Box<'a, PlaceItems<'a>>),
    "row-gap": RowGap(Box<'a, GapValue<'a>>),
    "column-gap": ColumnGap(Box<'a, GapValue<'a>>),
    "gap": Gap(Box<'a, Gap<'a>>),
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
    "flex-preferred-size": FlexPreferredSize(Box<'a, LengthPercentageOrAuto<'a>>, VendorPrefix),
    "grid-template-columns": GridTemplateColumns(Box<'a, TrackSizing<'a>>),
    "grid-template-rows": GridTemplateRows(Box<'a, TrackSizing<'a>>),
    "grid-auto-columns": GridAutoColumns(Vec<'a, TrackSize<'a>>),
    "grid-auto-rows": GridAutoRows(Vec<'a, TrackSize<'a>>),
    "grid-auto-flow": GridAutoFlow(Box<'a, GridAutoFlow>),
    "grid-template-areas": GridTemplateAreas(Box<'a, GridTemplateAreas<'a>>),
    "grid-template": GridTemplate(Box<'a, GridTemplate<'a>>),
    "grid": Grid(Box<'a, Grid<'a>>),
    "grid-row-start": GridRowStart(Box<'a, GridLine<'a>>),
    "grid-row-end": GridRowEnd(Box<'a, GridLine<'a>>),
    "grid-column-start": GridColumnStart(Box<'a, GridLine<'a>>),
    "grid-column-end": GridColumnEnd(Box<'a, GridLine<'a>>),
    "grid-row": GridRow(Box<'a, GridRow<'a>>),
    "grid-column": GridColumn(Box<'a, GridColumn<'a>>),
    "grid-area": GridArea(Box<'a, GridArea<'a>>),
    "margin-top": MarginTop(Box<'a, LengthPercentageOrAuto<'a>>),
    "margin-bottom": MarginBottom(Box<'a, LengthPercentageOrAuto<'a>>),
    "margin-left": MarginLeft(Box<'a, LengthPercentageOrAuto<'a>>),
    "margin-right": MarginRight(Box<'a, LengthPercentageOrAuto<'a>>),
    "margin-block-start": MarginBlockStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "margin-block-end": MarginBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "margin-inline-start": MarginInlineStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "margin-inline-end": MarginInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "margin-block": MarginBlock(Box<'a, MarginBlock<'a>>),
    "margin-inline": MarginInline(Box<'a, MarginInline<'a>>),
    "margin": Margin(Box<'a, Margin<'a>>),
    "padding-top": PaddingTop(Box<'a, LengthPercentageOrAuto<'a>>),
    "padding-bottom": PaddingBottom(Box<'a, LengthPercentageOrAuto<'a>>),
    "padding-left": PaddingLeft(Box<'a, LengthPercentageOrAuto<'a>>),
    "padding-right": PaddingRight(Box<'a, LengthPercentageOrAuto<'a>>),
    "padding-block-start": PaddingBlockStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "padding-block-end": PaddingBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "padding-inline-start": PaddingInlineStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "padding-inline-end": PaddingInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "padding-block": PaddingBlock(Box<'a, PaddingBlock<'a>>),
    "padding-inline": PaddingInline(Box<'a, PaddingInline<'a>>),
    "padding": Padding(Box<'a, Padding<'a>>),
    "scroll-margin-top": ScrollMarginTop(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-margin-bottom": ScrollMarginBottom(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-margin-left": ScrollMarginLeft(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-margin-right": ScrollMarginRight(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-margin-block-start": ScrollMarginBlockStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-margin-block-end": ScrollMarginBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-margin-inline-start": ScrollMarginInlineStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-margin-inline-end": ScrollMarginInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-margin-block": ScrollMarginBlock(Box<'a, ScrollMarginBlock<'a>>),
    "scroll-margin-inline": ScrollMarginInline(Box<'a, ScrollMarginInline<'a>>),
    "scroll-margin": ScrollMargin(Box<'a, ScrollMargin<'a>>),
    "scroll-padding-top": ScrollPaddingTop(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-padding-bottom": ScrollPaddingBottom(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-padding-left": ScrollPaddingLeft(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-padding-right": ScrollPaddingRight(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-padding-block-start": ScrollPaddingBlockStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-padding-block-end": ScrollPaddingBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-padding-inline-start": ScrollPaddingInlineStart(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-padding-inline-end": ScrollPaddingInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>),
    "scroll-padding-block": ScrollPaddingBlock(Box<'a, ScrollPaddingBlock<'a>>),
    "scroll-padding-inline": ScrollPaddingInline(Box<'a, ScrollPaddingInline<'a>>),
    "scroll-padding": ScrollPadding(Box<'a, ScrollPadding<'a>>),
    "font-weight": FontWeight(Box<'a, FontWeight<'a>>),
    "font-size": FontSize(Box<'a, FontSize<'a>>),
    "font-stretch": FontStretch(Box<'a, FontStretch>),
    "font-family": FontFamily(Vec<'a, FontFamily<'a>>),
    "font-style": FontStyle(Box<'a, FontStyle<'a>>),
    "font-variant-caps": FontVariantCaps(FontVariantCaps),
    "line-height": LineHeight(Box<'a, LineHeight<'a>>),
    "font": Font(Box<'a, Font<'a>>),
    "vertical-align": VerticalAlign(Box<'a, VerticalAlign<'a>>),
    "font-palette": FontPalette(Box<'a, DashedIdentReference<'a>>),
    "transition-property": TransitionProperty(Vec<'a, PropertyId<'a>>, VendorPrefix),
    "transition-duration": TransitionDuration(Vec<'a, Time>, VendorPrefix),
    "transition-delay": TransitionDelay(Vec<'a, Time>, VendorPrefix),
    "transition-timing-function": TransitionTimingFunction(Vec<'a, EasingFunction>, VendorPrefix),
    "transition": Transition(Vec<'a, Transition<'a>>, VendorPrefix),
    "animation-name": AnimationName(Vec<'a, AnimationName<'a>>, VendorPrefix),
    "animation-duration": AnimationDuration(Vec<'a, Time>, VendorPrefix),
    "animation-timing-function": AnimationTimingFunction(Vec<'a, EasingFunction>, VendorPrefix),
    "animation-iteration-count": AnimationIterationCount(Vec<'a, AnimationIterationCount>, VendorPrefix),
    "animation-direction": AnimationDirection(Vec<'a, AnimationDirection>, VendorPrefix),
    "animation-play-state": AnimationPlayState(Vec<'a, AnimationPlayState>, VendorPrefix),
    "animation-delay": AnimationDelay(Vec<'a, Time>, VendorPrefix),
    "animation-fill-mode": AnimationFillMode(Vec<'a, AnimationFillMode>, VendorPrefix),
    "animation-composition": AnimationComposition(Vec<'a, AnimationComposition>),
    "animation-timeline": AnimationTimeline(Vec<'a, AnimationTimeline<'a>>),
    "animation-range-start": AnimationRangeStart(Vec<'a, AnimationRangeStart<'a>>),
    "animation-range-end": AnimationRangeEnd(Vec<'a, AnimationRangeEnd<'a>>),
    "animation-range": AnimationRange(Vec<'a, AnimationRange<'a>>),
    "animation": Animation(Vec<'a, Animation<'a>>, VendorPrefix),
    "transform": Transform(Vec<'a, Transform<'a>>, VendorPrefix),
    "transform-origin": TransformOrigin(Box<'a, Position<'a>>, VendorPrefix),
    "transform-style": TransformStyle(TransformStyle, VendorPrefix),
    "transform-box": TransformBox(TransformBox),
    "backface-visibility": BackfaceVisibility(BackfaceVisibility, VendorPrefix),
    "perspective": Perspective(Box<'a, Perspective<'a>>, VendorPrefix),
    "perspective-origin": PerspectiveOrigin(Box<'a, Position<'a>>, VendorPrefix),
    "translate": Translate(Box<'a, Translate<'a>>),
    "rotate": Rotate(Box<'a, Rotate<'a>>),
    "scale": Scale(Box<'a, Scale<'a>>),
    "text-transform": TextTransform(Box<'a, TextTransform>),
    "white-space": WhiteSpace(WhiteSpace),
    "tab-size": TabSize(Box<'a, LengthOrNumber<'a>>, VendorPrefix),
    "word-break": WordBreak(WordBreak),
    "line-break": LineBreak(LineBreak),
    "hyphens": Hyphens(Hyphens, VendorPrefix),
    "overflow-wrap": OverflowWrap(OverflowWrap),
    "word-wrap": WordWrap(OverflowWrap),
    "text-align": TextAlign(TextAlign),
    "text-align-last": TextAlignLast(TextAlignLast, VendorPrefix),
    "text-justify": TextJustify(TextJustify),
    "word-spacing": WordSpacing(Box<'a, Spacing<'a>>),
    "letter-spacing": LetterSpacing(Box<'a, Spacing<'a>>),
    "text-indent": TextIndent(Box<'a, TextIndent<'a>>),
    "text-decoration-line": TextDecorationLine(Box<'a, TextDecorationLine<'a>>, VendorPrefix),
    "text-decoration-style": TextDecorationStyle(TextDecorationStyle, VendorPrefix),
    "text-decoration-color": TextDecorationColor(Box<'a, CssColor<'a>>, VendorPrefix),
    "text-decoration-thickness": TextDecorationThickness(Box<'a, TextDecorationThickness<'a>>),
    "text-decoration": TextDecoration(Box<'a, TextDecoration<'a>>, VendorPrefix),
    "text-decoration-skip-ink": TextDecorationSkipInk(TextDecorationSkipInk, VendorPrefix),
    "text-emphasis-style": TextEmphasisStyle(Box<'a, TextEmphasisStyle<'a>>, VendorPrefix),
    "text-emphasis-color": TextEmphasisColor(Box<'a, CssColor<'a>>, VendorPrefix),
    "text-emphasis": TextEmphasis(Box<'a, TextEmphasis<'a>>, VendorPrefix),
    "text-emphasis-position": TextEmphasisPosition(Box<'a, TextEmphasisPosition>, VendorPrefix),
    "text-shadow": TextShadow(Vec<'a, TextShadow<'a>>),
    "text-size-adjust": TextSizeAdjust(Box<'a, TextSizeAdjust>, VendorPrefix),
    "direction": Direction(TextDirection),
    "unicode-bidi": UnicodeBidi(UnicodeBidi),
    "box-decoration-break": BoxDecorationBreak(BoxDecorationBreak, VendorPrefix),
    "resize": Resize(Resize),
    "cursor": Cursor(Box<'a, Cursor<'a>>),
    "caret-color": CaretColor(Box<'a, ColorOrAuto<'a>>),
    "caret-shape": CaretShape(CaretShape),
    "caret": Caret(Box<'a, Caret<'a>>),
    "user-select": UserSelect(UserSelect, VendorPrefix),
    "accent-color": AccentColor(Box<'a, ColorOrAuto<'a>>),
    "appearance": Appearance(Box<'a, Appearance<'a>>, VendorPrefix),
    "list-style-type": ListStyleType(Box<'a, ListStyleType<'a>>),
    "list-style-image": ListStyleImage(Box<'a, Image<'a>>),
    "list-style-position": ListStylePosition(ListStylePosition),
    "list-style": ListStyle(Box<'a, ListStyle<'a>>),
    "marker-side": MarkerSide(MarkerSide),
    "composes": Composes(Box<'a, Composes<'a>>),
    "fill": Fill(Box<'a, SVGPaint<'a>>),
    "fill-rule": FillRule(FillRule),
    "fill-opacity": FillOpacity(f32),
    "stroke": Stroke(Box<'a, SVGPaint<'a>>),
    "stroke-opacity": StrokeOpacity(f32),
    "stroke-width": StrokeWidth(Box<'a, LengthPercentage<'a>>),
    "stroke-linecap": StrokeLinecap(StrokeLinecap),
    "stroke-linejoin": StrokeLinejoin(StrokeLinejoin),
    "stroke-miterlimit": StrokeMiterlimit(f32),
    "stroke-dasharray": StrokeDasharray(Box<'a, StrokeDasharray<'a>>),
    "stroke-dashoffset": StrokeDashoffset(Box<'a, LengthPercentage<'a>>),
    "marker-start": MarkerStart(Box<'a, Marker<'a>>),
    "marker-mid": MarkerMid(Box<'a, Marker<'a>>),
    "marker-end": MarkerEnd(Box<'a, Marker<'a>>),
    "marker": Marker(Box<'a, Marker<'a>>),
    "color-interpolation": ColorInterpolation(ColorInterpolation),
    "color-interpolation-filters": ColorInterpolationFilters(ColorInterpolation),
    "color-rendering": ColorRendering(ColorRendering),
    "shape-rendering": ShapeRendering(ShapeRendering),
    "text-rendering": TextRendering(TextRendering),
    "image-rendering": ImageRendering(ImageRendering),
    "clip-path": ClipPath(Box<'a, ClipPath<'a>>, VendorPrefix),
    "clip-rule": ClipRule(FillRule),
    "mask-image": MaskImage(Vec<'a, Image<'a>>, VendorPrefix),
    "mask-mode": MaskMode(Vec<'a, MaskMode>),
    "mask-repeat": MaskRepeat(Vec<'a, BackgroundRepeat>, VendorPrefix),
    "mask-position-x": MaskPositionX(Vec<'a, PositionComponent<'a, HorizontalPositionKeyword>>),
    "mask-position-y": MaskPositionY(Vec<'a, PositionComponent<'a, VerticalPositionKeyword>>),
    "mask-position": MaskPosition(Vec<'a, Position<'a>>, VendorPrefix),
    "mask-clip": MaskClip(Vec<'a, MaskClip>, VendorPrefix),
    "mask-origin": MaskOrigin(Vec<'a, GeometryBox>, VendorPrefix),
    "mask-size": MaskSize(Vec<'a, BackgroundSize<'a>>, VendorPrefix),
    "mask-composite": MaskComposite(Vec<'a, MaskComposite>),
    "mask-type": MaskType(MaskType),
    "mask": Mask(Vec<'a, Mask<'a>>, VendorPrefix),
    "mask-border-source": MaskBorderSource(Box<'a, Image<'a>>),
    "mask-border-mode": MaskBorderMode(MaskBorderMode),
    "mask-border-slice": MaskBorderSlice(Box<'a, BorderImageSlice<'a>>),
    "mask-border-width": MaskBorderWidth(Box<'a, Rect<'a, BorderImageSideWidth<'a>>>),
    "mask-border-outset": MaskBorderOutset(Box<'a, Rect<'a, LengthOrNumber<'a>>>),
    "mask-border-repeat": MaskBorderRepeat(Box<'a, BorderImageRepeat>),
    "mask-border": MaskBorder(Box<'a, MaskBorder<'a>>),
    "-webkit-mask-composite": WebKitMaskComposite(Vec<'a, WebKitMaskComposite>),
    "mask-source-type": WebKitMaskSourceType(Vec<'a, WebKitMaskSourceType>, VendorPrefix),
    "mask-box-image": WebKitMaskBoxImage(Box<'a, BorderImage<'a>>, VendorPrefix),
    "mask-box-image-source": WebKitMaskBoxImageSource(Box<'a, Image<'a>>, VendorPrefix),
    "mask-box-image-slice": WebKitMaskBoxImageSlice(Box<'a, BorderImageSlice<'a>>, VendorPrefix),
    "mask-box-image-width": WebKitMaskBoxImageWidth(Box<'a, Rect<'a, BorderImageSideWidth<'a>>>, VendorPrefix),
    "mask-box-image-outset": WebKitMaskBoxImageOutset(Box<'a, Rect<'a, LengthOrNumber<'a>>>, VendorPrefix),
    "mask-box-image-repeat": WebKitMaskBoxImageRepeat(Box<'a, BorderImageRepeat>, VendorPrefix),
    "filter": Filter(Box<'a, FilterList<'a>>, VendorPrefix),
    "backdrop-filter": BackdropFilter(Box<'a, FilterList<'a>>, VendorPrefix),
    "mix-blend-mode": MixBlendMode(BlendMode),
    "z-index": ZIndex(Box<'a, ZIndex>),
    "container-type": ContainerType(ContainerType),
    "container-name": ContainerName(Box<'a, ContainerNameList<'a>>),
    "container": Container(Box<'a, Container<'a>>),
    "view-transition-name": ViewTransitionName(Box<'a, ViewTransitionName<'a>>),
    "view-transition-class": ViewTransitionClass(Box<'a, NoneOrCustomIdentList<'a>>),
    "view-transition-group": ViewTransitionGroup(Box<'a, ViewTransitionGroup<'a>>),
    "color-scheme": ColorScheme(Box<'a, ColorScheme>),
    "print-color-adjust": PrintColorAdjust(PrintColorAdjust, VendorPrefix),
        }
    };
}

for_each_property!(define_properties);
