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

macro_rules! property_parser_strategy {
    () => {
        PropertyParserStrategy::Unsupported
    };
    (unsupported) => {
        PropertyParserStrategy::Unsupported
    };
    (parse : $value:ty) => {
        PropertyParserStrategy::Parse
    };
    (boxed : $value:ty) => {
        PropertyParserStrategy::Boxed
    };
    (comma_separated : $value:ty) => {
        PropertyParserStrategy::CommaSeparated
    };
    (whitespace_separated : $value:ty) => {
        PropertyParserStrategy::WhitespaceSeparated
    };
    (rect : $value:ty) => {
        PropertyParserStrategy::Rect
    };
    (two_value : $value:ty) => {
        PropertyParserStrategy::TwoValue
    };
    (custom : $adapter:ident) => {
        PropertyParserStrategy::Custom
    };
    (css_wide : $value:ty) => {
        PropertyParserStrategy::CssWide
    };
    (css_wide_boxed : $value:ty) => {
        PropertyParserStrategy::CssWideBoxed
    };
}

macro_rules! define_properties {
    (
        $(
            $(#[$meta:meta])*
            $name:literal: $property:ident($value:ty $(, $vp:ty)?)
                [$strategy:ident $( : $($strategy_args:tt)+)?],
        )+
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
        pub enum PropertyId<'a> {
            $(
                $(#[$meta])*
                $property$(($vp))?,
            )+
            Unparsed,
            Custom(&'a str),
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
            CSSWide(Box<'a, PropertyId<'a>>, CSSWideKeyword),
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
                    $($name => Some(Self::$property$( (<$vp>::default()) )?),)+
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
            pub fn name(&self) -> &'a str {
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

            /// Returns the generated parser strategy for this known property.
            ///
            /// The strategy is metadata rather than a second property
            /// registry. Unknown and sentinel IDs intentionally report
            /// `Unsupported` because they have no typed declaration entry.
            pub fn parser_strategy(&self) -> PropertyParserStrategy {
                match self {
                    $(property_id_prefix_pattern!(Self::$property$(, $vp)?, _prefix) => {
                        property_parser_strategy!($strategy $( : $($strategy_args)+ )?)
                    }),+
                    Self::Unparsed | Self::Custom(_) => PropertyParserStrategy::Unsupported,
                }
            }

            /// Returns the generated support classification for this property.
            pub fn support_classification(&self) -> PropertySupport {
                match self {
                    Self::Custom(_) => PropertySupport::Custom,
                    Self::Unparsed => PropertySupport::UnsupportedGrammar,
                    _ => self.parser_strategy().support_classification(),
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
                    Self::CSSWide(property_id, _) => Some(**property_id),
                    Self::Unparsed(value) => Some(*value.property_id),
                    Self::Custom(value) => Some(PropertyId::Custom(match &*value.name {
                        CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
                    })),
                    Self::Tombstone => None,
                }
            }

            /// Returns the canonical CSS property name.
            pub fn name(&self) -> &'a str {
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

/// The parser construction path generated for a known property.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
pub enum PropertyParserStrategy {
    Parse,
    Boxed,
    CommaSeparated,
    WhitespaceSeparated,
    Rect,
    TwoValue,
    Custom,
    CssWide,
    CssWideBoxed,
    Unsupported,
}

/// Coarse parser support used by audits without duplicating property metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
pub enum PropertySupport {
    Typed,
    UnsupportedGrammar,
    Custom,
}

impl PropertyParserStrategy {
    #[inline]
    pub const fn support_classification(self) -> PropertySupport {
        match self {
            Self::Unsupported => PropertySupport::UnsupportedGrammar,
            Self::Parse
            | Self::Boxed
            | Self::CommaSeparated
            | Self::WhitespaceSeparated
            | Self::Rect
            | Self::TwoValue
            | Self::Custom
            | Self::CssWide
            | Self::CssWideBoxed => PropertySupport::Typed,
        }
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
    "all": All(CSSWideKeyword) [custom: parse_all],
    "background-color": BackgroundColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "background-image": BackgroundImage(Vec<'a, Image<'a>>) [comma_separated: Image<'i>],
    "background-position-x": BackgroundPositionX(Vec<'a, PositionComponent<'a, HorizontalPositionKeyword>>) [comma_separated: PositionComponent<'i, HorizontalPositionKeyword>],
    "background-position-y": BackgroundPositionY(Vec<'a, PositionComponent<'a, VerticalPositionKeyword>>) [comma_separated: PositionComponent<'i, VerticalPositionKeyword>],
    "background-position": BackgroundPosition(Vec<'a, BackgroundPosition<'a>>) [comma_separated: BackgroundPosition<'i>],
    "background-size": BackgroundSize(Vec<'a, BackgroundSize<'a>>) [comma_separated: BackgroundSize<'i>],
    "background-repeat": BackgroundRepeat(Vec<'a, BackgroundRepeat>) [comma_separated: BackgroundRepeat],
    "background-attachment": BackgroundAttachment(Vec<'a, BackgroundAttachment>) [comma_separated: BackgroundAttachment],
    "background-clip": BackgroundClip(Vec<'a, BackgroundClip>, VendorPrefix) [comma_separated: BackgroundClip],
    "background-origin": BackgroundOrigin(Vec<'a, BackgroundOrigin>) [comma_separated: BackgroundOrigin],
    "background": Background(Vec<'a, Background<'a>>) [custom: parse_background],
    "box-shadow": BoxShadow(Vec<'a, BoxShadow<'a>>, VendorPrefix) [unsupported],
    "opacity": Opacity(f32) [custom: parse_opacity],
    "color": Color(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "display": Display(Display) [parse: Display],
    "visibility": Visibility(Visibility) [parse: Visibility],
    "width": Width(Box<'a, Size<'a>>) [boxed: Size<'i>],
    "height": Height(Box<'a, Size<'a>>) [boxed: Size<'i>],
    "min-width": MinWidth(Box<'a, Size<'a>>) [boxed: Size<'i>],
    "min-height": MinHeight(Box<'a, Size<'a>>) [boxed: Size<'i>],
    "max-width": MaxWidth(Box<'a, MaxSize<'a>>) [boxed: MaxSize<'i>],
    "max-height": MaxHeight(Box<'a, MaxSize<'a>>) [boxed: MaxSize<'i>],
    "block-size": BlockSize(Box<'a, Size<'a>>) [boxed: Size<'i>],
    "inline-size": InlineSize(Box<'a, Size<'a>>) [boxed: Size<'i>],
    "min-block-size": MinBlockSize(Box<'a, Size<'a>>) [boxed: Size<'i>],
    "min-inline-size": MinInlineSize(Box<'a, Size<'a>>) [boxed: Size<'i>],
    "max-block-size": MaxBlockSize(Box<'a, MaxSize<'a>>) [boxed: MaxSize<'i>],
    "max-inline-size": MaxInlineSize(Box<'a, MaxSize<'a>>) [boxed: MaxSize<'i>],
    "box-sizing": BoxSizing(BoxSizing, VendorPrefix) [parse: BoxSizing],
    "aspect-ratio": AspectRatio(AspectRatio) [parse: AspectRatio],
    "overflow": Overflow(Overflow) [parse: Overflow],
    "overflow-x": OverflowX(OverflowKeyword) [parse: OverflowKeyword],
    "overflow-y": OverflowY(OverflowKeyword) [parse: OverflowKeyword],
    "text-overflow": TextOverflow(TextOverflow, VendorPrefix) [parse: TextOverflow],
    "object-fit": ObjectFit(ObjectFit) [parse: ObjectFit],
    "object-position": ObjectPosition(Box<'a, Position<'a>>) [boxed: Position<'i>],
    "scrollbar-color": ScrollbarColor(ScrollbarColor<'a>) [parse: ScrollbarColor<'i>],
    "position": Position(Box<'a, PositionProperty>) [boxed: PositionProperty],
    "top": Top(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "bottom": Bottom(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "left": Left(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "right": Right(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-block-start": InsetBlockStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-block-end": InsetBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-inline-start": InsetInlineStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-inline-end": InsetInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-block": InsetBlock(Box<'a, InsetBlock<'a>>) [two_value: InsetBlock<'i>],
    "inset-inline": InsetInline(Box<'a, InsetInline<'a>>) [two_value: InsetInline<'i>],
    "inset": Inset(Box<'a, Inset<'a>>) [rect: Inset<'i>],
    "border-spacing": BorderSpacing(Box<'a, Size2D<'a, Length<'a>>>) [two_value: Size2D<'i, Length<'i>>],
    "border-top-color": BorderTopColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "border-bottom-color": BorderBottomColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "border-left-color": BorderLeftColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "border-right-color": BorderRightColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "border-block-start-color": BorderBlockStartColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "border-block-end-color": BorderBlockEndColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "border-inline-start-color": BorderInlineStartColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "border-inline-end-color": BorderInlineEndColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "border-top-style": BorderTopStyle(LineStyle) [parse: LineStyle],
    "border-bottom-style": BorderBottomStyle(LineStyle) [parse: LineStyle],
    "border-left-style": BorderLeftStyle(LineStyle) [parse: LineStyle],
    "border-right-style": BorderRightStyle(LineStyle) [parse: LineStyle],
    "border-block-start-style": BorderBlockStartStyle(LineStyle) [parse: LineStyle],
    "border-block-end-style": BorderBlockEndStyle(LineStyle) [parse: LineStyle],
    "border-inline-start-style": BorderInlineStartStyle(LineStyle) [parse: LineStyle],
    "border-inline-end-style": BorderInlineEndStyle(LineStyle) [parse: LineStyle],
    "border-top-width": BorderTopWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-bottom-width": BorderBottomWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-left-width": BorderLeftWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-right-width": BorderRightWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-block-start-width": BorderBlockStartWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-block-end-width": BorderBlockEndWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-inline-start-width": BorderInlineStartWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-inline-end-width": BorderInlineEndWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-top-left-radius": BorderTopLeftRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-top-right-radius": BorderTopRightRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-bottom-left-radius": BorderBottomLeftRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-bottom-right-radius": BorderBottomRightRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-start-start-radius": BorderStartStartRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-start-end-radius": BorderStartEndRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-end-start-radius": BorderEndStartRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-end-end-radius": BorderEndEndRadius(Box<'a, Size2D<'a, LengthPercentage<'a>>>) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-radius": BorderRadius(Box<'a, BorderRadius<'a>>, VendorPrefix) [rect: BorderRadius<'i>],
    "border-image-source": BorderImageSource(Box<'a, Image<'a>>) [boxed: Image<'i>],
    "border-image-outset": BorderImageOutset(Box<'a, Rect<'a, LengthOrNumber<'a>>>) [unsupported],
    "border-image-repeat": BorderImageRepeat(BorderImageRepeat) [unsupported],
    "border-image-width": BorderImageWidth(Box<'a, Rect<'a, BorderImageSideWidth<'a>>>) [unsupported],
    "border-image-slice": BorderImageSlice(Box<'a, BorderImageSlice<'a>>) [unsupported],
    "border-image": BorderImage(Box<'a, BorderImage<'a>>, VendorPrefix) [unsupported],
    "border-color": BorderColor(Box<'a, BorderColor<'a>>) [rect: BorderColor<'i>],
    "border-style": BorderStyle(Box<'a, BorderStyle>) [rect: BorderStyle],
    "border-width": BorderWidth(Box<'a, BorderWidth<'a>>) [rect: BorderWidth<'i>],
    "border-block-color": BorderBlockColor(Box<'a, BorderBlockColor<'a>>) [two_value: BorderBlockColor<'i>],
    "border-block-style": BorderBlockStyle(Box<'a, BorderBlockStyle>) [two_value: BorderBlockStyle],
    "border-block-width": BorderBlockWidth(Box<'a, BorderBlockWidth<'a>>) [two_value: BorderBlockWidth<'i>],
    "border-inline-color": BorderInlineColor(Box<'a, BorderInlineColor<'a>>) [two_value: BorderInlineColor<'i>],
    "border-inline-style": BorderInlineStyle(Box<'a, BorderInlineStyle>) [two_value: BorderInlineStyle],
    "border-inline-width": BorderInlineWidth(Box<'a, BorderInlineWidth<'a>>) [two_value: BorderInlineWidth<'i>],
    "border": Border(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-top": BorderTop(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-bottom": BorderBottom(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-left": BorderLeft(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-right": BorderRight(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-block": BorderBlock(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-block-start": BorderBlockStart(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-block-end": BorderBlockEnd(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-inline": BorderInline(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-inline-start": BorderInlineStart(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-inline-end": BorderInlineEnd(Box<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "outline": Outline(Box<'a, GenericBorder<'a, OutlineStyle>>) [boxed: GenericBorder<'i, OutlineStyle>],
    "outline-color": OutlineColor(Box<'a, CssColor<'a>>) [boxed: CssColor<'i>],
    "outline-style": OutlineStyle(OutlineStyle) [parse: OutlineStyle],
    "outline-width": OutlineWidth(Box<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "flex-direction": FlexDirection(FlexDirection, VendorPrefix) [parse: FlexDirection],
    "flex-wrap": FlexWrap(FlexWrap, VendorPrefix) [parse: FlexWrap],
    "flex-flow": FlexFlow(Box<'a, FlexFlow>, VendorPrefix) [whitespace_separated: FlexFlow],
    "flex-grow": FlexGrow(f32, VendorPrefix) [custom: parse_flex_grow],
    "flex-shrink": FlexShrink(f32, VendorPrefix) [custom: parse_flex_shrink],
    "flex-basis": FlexBasis(Box<'a, LengthPercentageOrAuto<'a>>, VendorPrefix) [boxed: LengthPercentageOrAuto<'i>],
    "flex": Flex(Box<'a, Flex<'a>>, VendorPrefix) [boxed: Flex<'i>],
    "order": Order(f32, VendorPrefix) [custom: parse_order],
    "align-content": AlignContent(AlignContent, VendorPrefix) [parse: AlignContent],
    "justify-content": JustifyContent(JustifyContent, VendorPrefix) [parse: JustifyContent],
    "place-content": PlaceContent(PlaceContent) [parse: PlaceContent],
    "align-self": AlignSelf(AlignSelf, VendorPrefix) [parse: AlignSelf],
    "justify-self": JustifySelf(JustifySelf) [parse: JustifySelf],
    "place-self": PlaceSelf(PlaceSelf) [parse: PlaceSelf],
    "align-items": AlignItems(AlignItems, VendorPrefix) [parse: AlignItems],
    "justify-items": JustifyItems(JustifyItems) [parse: JustifyItems],
    "place-items": PlaceItems(PlaceItems) [parse: PlaceItems],
    "row-gap": RowGap(Box<'a, GapValue<'a>>) [boxed: GapValue<'i>],
    "column-gap": ColumnGap(Box<'a, GapValue<'a>>) [boxed: GapValue<'i>],
    "gap": Gap(Box<'a, Gap<'a>>) [two_value: Gap<'i>],
    "column-rule": ColumnRule(Box<'a, ColumnRule<'a>>, VendorPrefix) [boxed: ColumnRule<'i>],
    "column-width": ColumnWidth(CSSWideOr<ColumnWidth<'a>>, VendorPrefix) [css_wide: ColumnWidth<'i>],
    "column-count": ColumnCount(CSSWideOr<ColumnCount>, VendorPrefix) [css_wide: ColumnCount],
    "columns": Columns(CSSWideOr<Box<'a, Columns<'a>>>, VendorPrefix) [css_wide_boxed: Columns<'i>],
    "grid-column-gap": GridColumnGap(Box<'a, GapValue<'a>>) [boxed: GapValue<'i>],
    "grid-row-gap": GridRowGap(Box<'a, GapValue<'a>>) [boxed: GapValue<'i>],
    "box-orient": BoxOrient(BoxOrient, VendorPrefix) [parse: BoxOrient],
    "box-direction": BoxDirection(BoxDirection, VendorPrefix) [parse: BoxDirection],
    "box-ordinal-group": BoxOrdinalGroup(f32, VendorPrefix) [custom: parse_box_ordinal_group],
    "box-align": BoxAlign(BoxAlign, VendorPrefix) [parse: BoxAlign],
    "box-flex": BoxFlex(f32, VendorPrefix) [custom: parse_box_flex],
    "box-flex-group": BoxFlexGroup(f32, VendorPrefix) [custom: parse_box_flex_group],
    "box-pack": BoxPack(BoxPack, VendorPrefix) [parse: BoxPack],
    "box-lines": BoxLines(BoxLines, VendorPrefix) [parse: BoxLines],
    "flex-pack": FlexPack(FlexPack, VendorPrefix) [parse: FlexPack],
    "flex-order": FlexOrder(f32, VendorPrefix) [custom: parse_flex_order],
    "flex-align": FlexAlign(BoxAlign, VendorPrefix) [parse: BoxAlign],
    "flex-item-align": FlexItemAlign(FlexItemAlign, VendorPrefix) [parse: FlexItemAlign],
    "flex-line-pack": FlexLinePack(FlexLinePack, VendorPrefix) [parse: FlexLinePack],
    "flex-positive": FlexPositive(f32, VendorPrefix) [custom: parse_flex_positive],
    "flex-negative": FlexNegative(f32, VendorPrefix) [custom: parse_flex_negative],
    "flex-preferred-size": FlexPreferredSize(Box<'a, LengthPercentageOrAuto<'a>>, VendorPrefix) [boxed: LengthPercentageOrAuto<'i>],
    "grid-template-columns": GridTemplateColumns(Box<'a, TrackSizing<'a>>) [unsupported],
    "grid-template-rows": GridTemplateRows(Box<'a, TrackSizing<'a>>) [unsupported],
    "grid-auto-columns": GridAutoColumns(Vec<'a, TrackSize<'a>>) [unsupported],
    "grid-auto-rows": GridAutoRows(Vec<'a, TrackSize<'a>>) [unsupported],
    "grid-auto-flow": GridAutoFlow(GridAutoFlow) [unsupported],
    "grid-template-areas": GridTemplateAreas(Box<'a, GridTemplateAreas<'a>>) [unsupported],
    "grid-template": GridTemplate(Box<'a, GridTemplate<'a>>) [unsupported],
    "grid": Grid(Box<'a, Grid<'a>>) [unsupported],
    "grid-row-start": GridRowStart(Box<'a, GridLine<'a>>) [unsupported],
    "grid-row-end": GridRowEnd(Box<'a, GridLine<'a>>) [unsupported],
    "grid-column-start": GridColumnStart(Box<'a, GridLine<'a>>) [unsupported],
    "grid-column-end": GridColumnEnd(Box<'a, GridLine<'a>>) [unsupported],
    "grid-row": GridRow(Box<'a, GridRow<'a>>) [unsupported],
    "grid-column": GridColumn(Box<'a, GridColumn<'a>>) [unsupported],
    "grid-area": GridArea(Box<'a, GridArea<'a>>) [unsupported],
    "margin-top": MarginTop(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-bottom": MarginBottom(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-left": MarginLeft(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-right": MarginRight(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-block-start": MarginBlockStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-block-end": MarginBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-inline-start": MarginInlineStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-inline-end": MarginInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-block": MarginBlock(Box<'a, MarginBlock<'a>>) [two_value: MarginBlock<'i>],
    "margin-inline": MarginInline(Box<'a, MarginInline<'a>>) [two_value: MarginInline<'i>],
    "margin": Margin(Box<'a, Margin<'a>>) [rect: Margin<'i>],
    "padding-top": PaddingTop(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-bottom": PaddingBottom(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-left": PaddingLeft(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-right": PaddingRight(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-block-start": PaddingBlockStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-block-end": PaddingBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-inline-start": PaddingInlineStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-inline-end": PaddingInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-block": PaddingBlock(Box<'a, PaddingBlock<'a>>) [two_value: PaddingBlock<'i>],
    "padding-inline": PaddingInline(Box<'a, PaddingInline<'a>>) [two_value: PaddingInline<'i>],
    "padding": Padding(Box<'a, Padding<'a>>) [rect: Padding<'i>],
    "scroll-margin-top": ScrollMarginTop(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-bottom": ScrollMarginBottom(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-left": ScrollMarginLeft(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-right": ScrollMarginRight(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-block-start": ScrollMarginBlockStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-block-end": ScrollMarginBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-inline-start": ScrollMarginInlineStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-inline-end": ScrollMarginInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-block": ScrollMarginBlock(Box<'a, ScrollMarginBlock<'a>>) [two_value: ScrollMarginBlock<'i>],
    "scroll-margin-inline": ScrollMarginInline(Box<'a, ScrollMarginInline<'a>>) [two_value: ScrollMarginInline<'i>],
    "scroll-margin": ScrollMargin(Box<'a, ScrollMargin<'a>>) [rect: ScrollMargin<'i>],
    "scroll-padding-top": ScrollPaddingTop(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-bottom": ScrollPaddingBottom(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-left": ScrollPaddingLeft(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-right": ScrollPaddingRight(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-block-start": ScrollPaddingBlockStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-block-end": ScrollPaddingBlockEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-inline-start": ScrollPaddingInlineStart(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-inline-end": ScrollPaddingInlineEnd(Box<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-block": ScrollPaddingBlock(Box<'a, ScrollPaddingBlock<'a>>) [two_value: ScrollPaddingBlock<'i>],
    "scroll-padding-inline": ScrollPaddingInline(Box<'a, ScrollPaddingInline<'a>>) [two_value: ScrollPaddingInline<'i>],
    "scroll-padding": ScrollPadding(Box<'a, ScrollPadding<'a>>) [rect: ScrollPadding<'i>],
    "font-weight": FontWeight(FontWeight) [parse: FontWeight],
    "font-size": FontSize(Box<'a, FontSize<'a>>) [boxed: FontSize<'i>],
    "font-stretch": FontStretch(FontStretch) [parse: FontStretch],
    "font-family": FontFamily(Vec<'a, FontFamily<'a>>) [custom: parse_font_family],
    "font-style": FontStyle(FontStyle) [parse: FontStyle],
    "font-variant-caps": FontVariantCaps(FontVariantCaps) [parse: FontVariantCaps],
    "line-height": LineHeight(Box<'a, LineHeight<'a>>) [boxed: LineHeight<'i>],
    "font": Font(Box<'a, Font<'a>>) [unsupported],
    "vertical-align": VerticalAlign(Box<'a, VerticalAlign<'a>>) [unsupported],
    "font-palette": FontPalette(Box<'a, DashedIdentReference<'a>>) [unsupported],
    "transition-property": TransitionProperty(Vec<'a, PropertyId<'a>>, VendorPrefix) [custom: parse_transition_property],
    "transition-duration": TransitionDuration(Vec<'a, Time>, VendorPrefix) [custom: parse_transition_duration],
    "transition-delay": TransitionDelay(Vec<'a, Time>, VendorPrefix) [custom: parse_transition_delay],
    "transition-timing-function": TransitionTimingFunction(Vec<'a, EasingFunction>, VendorPrefix) [custom: parse_transition_timing],
    "transition": Transition(Vec<'a, Transition<'a>>, VendorPrefix) [custom: parse_transition],
    "animation-name": AnimationName(Vec<'a, AnimationName<'a>>, VendorPrefix) [custom: parse_animation_name],
    "animation-duration": AnimationDuration(Vec<'a, Time>, VendorPrefix) [custom: parse_animation_duration],
    "animation-timing-function": AnimationTimingFunction(Vec<'a, EasingFunction>, VendorPrefix) [custom: parse_animation_timing],
    "animation-iteration-count": AnimationIterationCount(Vec<'a, AnimationIterationCount>, VendorPrefix) [custom: parse_animation_iteration],
    "animation-direction": AnimationDirection(Vec<'a, AnimationDirection>, VendorPrefix) [custom: parse_animation_direction],
    "animation-play-state": AnimationPlayState(Vec<'a, AnimationPlayState>, VendorPrefix) [custom: parse_animation_play_state],
    "animation-delay": AnimationDelay(Vec<'a, Time>, VendorPrefix) [custom: parse_animation_delay],
    "animation-fill-mode": AnimationFillMode(Vec<'a, AnimationFillMode>, VendorPrefix) [custom: parse_animation_fill],
    "animation-composition": AnimationComposition(Vec<'a, AnimationComposition>) [unsupported],
    "animation-timeline": AnimationTimeline(Vec<'a, AnimationTimeline<'a>>) [unsupported],
    "animation-range-start": AnimationRangeStart(Vec<'a, AnimationRangeStart<'a>>) [unsupported],
    "animation-range-end": AnimationRangeEnd(Vec<'a, AnimationRangeEnd<'a>>) [unsupported],
    "animation-range": AnimationRange(Vec<'a, AnimationRange<'a>>) [unsupported],
    "animation": Animation(Vec<'a, Animation<'a>>, VendorPrefix) [custom: parse_animation],
    "transform": Transform(Vec<'a, Transform<'a>>, VendorPrefix) [custom: parse_transform],
    "transform-origin": TransformOrigin(Box<'a, Position<'a>>, VendorPrefix) [boxed: Position<'i>],
    "transform-style": TransformStyle(TransformStyle, VendorPrefix) [parse: TransformStyle],
    "transform-box": TransformBox(TransformBox) [parse: TransformBox],
    "backface-visibility": BackfaceVisibility(BackfaceVisibility, VendorPrefix) [parse: BackfaceVisibility],
    "perspective": Perspective(Box<'a, Perspective<'a>>, VendorPrefix) [boxed: Perspective<'i>],
    "perspective-origin": PerspectiveOrigin(Box<'a, Position<'a>>, VendorPrefix) [boxed: Position<'i>],
    "translate": Translate(Box<'a, Translate<'a>>) [boxed: Translate<'i>],
    "rotate": Rotate(Rotate) [parse: Rotate],
    "scale": Scale(Box<'a, Scale>) [boxed: Scale],
    "text-transform": TextTransform(TextTransform) [parse: TextTransform],
    "content": Content(Box<'a, Content<'a>>) [unsupported],
    "white-space": WhiteSpace(WhiteSpace) [parse: WhiteSpace],
    "tab-size": TabSize(Box<'a, LengthOrNumber<'a>>, VendorPrefix) [unsupported],
    "word-break": WordBreak(WordBreak) [parse: WordBreak],
    "line-break": LineBreak(LineBreak) [parse: LineBreak],
    "hyphens": Hyphens(Hyphens, VendorPrefix) [parse: Hyphens],
    "overflow-wrap": OverflowWrap(OverflowWrap) [parse: OverflowWrap],
    "word-wrap": WordWrap(OverflowWrap) [parse: OverflowWrap],
    "text-align": TextAlign(TextAlign) [parse: TextAlign],
    "text-align-last": TextAlignLast(TextAlignLast, VendorPrefix) [parse: TextAlignLast],
    "text-justify": TextJustify(TextJustify) [parse: TextJustify],
    "word-spacing": WordSpacing(Box<'a, Spacing<'a>>) [boxed: Spacing<'i>],
    "letter-spacing": LetterSpacing(Box<'a, Spacing<'a>>) [boxed: Spacing<'i>],
    "text-indent": TextIndent(Box<'a, TextIndent<'a>>) [boxed: TextIndent<'i>],
    "text-decoration-line": TextDecorationLine(Box<'a, TextDecorationLine<'a>>, VendorPrefix) [boxed: TextDecorationLine<'i>],
    "text-decoration-style": TextDecorationStyle(TextDecorationStyle, VendorPrefix) [parse: TextDecorationStyle],
    "text-decoration-color": TextDecorationColor(Box<'a, CssColor<'a>>, VendorPrefix) [boxed: CssColor<'i>],
    "text-decoration-thickness": TextDecorationThickness(Box<'a, TextDecorationThickness<'a>>) [boxed: TextDecorationThickness<'i>],
    "text-decoration": TextDecoration(Box<'a, TextDecoration<'a>>, VendorPrefix) [unsupported],
    "text-decoration-skip-ink": TextDecorationSkipInk(TextDecorationSkipInk, VendorPrefix) [parse: TextDecorationSkipInk],
    "text-emphasis-style": TextEmphasisStyle(Box<'a, TextEmphasisStyle<'a>>, VendorPrefix) [unsupported],
    "text-emphasis-color": TextEmphasisColor(Box<'a, CssColor<'a>>, VendorPrefix) [boxed: CssColor<'i>],
    "text-emphasis": TextEmphasis(Box<'a, TextEmphasis<'a>>, VendorPrefix) [unsupported],
    "text-emphasis-position": TextEmphasisPosition(TextEmphasisPosition, VendorPrefix) [unsupported],
    "text-shadow": TextShadow(Vec<'a, TextShadow<'a>>) [unsupported],
    "text-size-adjust": TextSizeAdjust(TextSizeAdjust, VendorPrefix) [parse: TextSizeAdjust],
    "direction": Direction(TextDirection) [parse: TextDirection],
    "unicode-bidi": UnicodeBidi(UnicodeBidi) [parse: UnicodeBidi],
    "box-decoration-break": BoxDecorationBreak(BoxDecorationBreak, VendorPrefix) [parse: BoxDecorationBreak],
    "resize": Resize(Resize) [parse: Resize],
    "pointer-events": PointerEvents(PointerEvents) [parse: PointerEvents],
    "float": Float(Float) [parse: Float],
    "clear": Clear(Clear) [parse: Clear],
    "touch-action": TouchAction(TouchAction) [parse: TouchAction],
    "scroll-behavior": ScrollBehavior(ScrollBehavior) [parse: ScrollBehavior],
    "cursor": Cursor(Box<'a, Cursor<'a>>) [unsupported],
    "caret-color": CaretColor(Box<'a, ColorOrAuto<'a>>) [unsupported],
    "caret-shape": CaretShape(CaretShape) [unsupported],
    "caret": Caret(Box<'a, Caret<'a>>) [unsupported],
    "user-select": UserSelect(UserSelect, VendorPrefix) [parse: UserSelect],
    "accent-color": AccentColor(Box<'a, ColorOrAuto<'a>>) [boxed: ColorOrAuto<'i>],
    "appearance": Appearance(Box<'a, Appearance<'a>>, VendorPrefix) [unsupported],
    "list-style-type": ListStyleType(Box<'a, ListStyleType<'a>>) [unsupported],
    "list-style-image": ListStyleImage(Box<'a, Image<'a>>) [boxed: Image<'i>],
    "list-style-position": ListStylePosition(ListStylePosition) [unsupported],
    "list-style": ListStyle(Box<'a, ListStyle<'a>>) [unsupported],
    "marker-side": MarkerSide(MarkerSide) [unsupported],
    "composes": Composes(Box<'a, Composes<'a>>) [unsupported],
    "fill": Fill(Box<'a, SVGPaint<'a>>) [boxed: SVGPaint<'i>],
    "fill-rule": FillRule(FillRule) [parse: FillRule],
    "fill-opacity": FillOpacity(f32) [custom: parse_fill_opacity],
    "stroke": Stroke(Box<'a, SVGPaint<'a>>) [boxed: SVGPaint<'i>],
    "stroke-opacity": StrokeOpacity(f32) [custom: parse_stroke_opacity],
    "stroke-width": StrokeWidth(Box<'a, LengthPercentage<'a>>) [boxed: LengthPercentage<'i>],
    "stroke-linecap": StrokeLinecap(StrokeLinecap) [parse: StrokeLinecap],
    "stroke-linejoin": StrokeLinejoin(StrokeLinejoin) [parse: StrokeLinejoin],
    "stroke-miterlimit": StrokeMiterlimit(f32) [custom: parse_stroke_miterlimit],
    "stroke-dasharray": StrokeDasharray(Box<'a, StrokeDasharray<'a>>) [boxed: StrokeDasharray<'i>],
    "stroke-dashoffset": StrokeDashoffset(Box<'a, LengthPercentage<'a>>) [boxed: LengthPercentage<'i>],
    "marker-start": MarkerStart(Box<'a, Marker<'a>>) [boxed: Marker<'i>],
    "marker-mid": MarkerMid(Box<'a, Marker<'a>>) [boxed: Marker<'i>],
    "marker-end": MarkerEnd(Box<'a, Marker<'a>>) [boxed: Marker<'i>],
    "marker": Marker(Box<'a, Marker<'a>>) [boxed: Marker<'i>],
    "color-interpolation": ColorInterpolation(ColorInterpolation) [parse: ColorInterpolation],
    "color-interpolation-filters": ColorInterpolationFilters(ColorInterpolation) [parse: ColorInterpolation],
    "color-rendering": ColorRendering(ColorRendering) [parse: ColorRendering],
    "shape-rendering": ShapeRendering(ShapeRendering) [parse: ShapeRendering],
    "text-rendering": TextRendering(TextRendering) [parse: TextRendering],
    "image-rendering": ImageRendering(ImageRendering) [parse: ImageRendering],
    "clip-path": ClipPath(Box<'a, ClipPath<'a>>, VendorPrefix) [unsupported],
    "clip-rule": ClipRule(FillRule) [unsupported],
    "mask-image": MaskImage(Vec<'a, Image<'a>>, VendorPrefix) [comma_separated: Image<'i>],
    "mask-mode": MaskMode(Vec<'a, MaskMode>) [comma_separated: MaskMode],
    "mask-repeat": MaskRepeat(Vec<'a, BackgroundRepeat>, VendorPrefix) [comma_separated: BackgroundRepeat],
    "mask-position-x": MaskPositionX(Vec<'a, PositionComponent<'a, HorizontalPositionKeyword>>) [comma_separated: PositionComponent<'i, HorizontalPositionKeyword>],
    "mask-position-y": MaskPositionY(Vec<'a, PositionComponent<'a, VerticalPositionKeyword>>) [comma_separated: PositionComponent<'i, VerticalPositionKeyword>],
    "mask-position": MaskPosition(Vec<'a, Position<'a>>, VendorPrefix) [comma_separated: Position<'i>],
    "mask-clip": MaskClip(Vec<'a, MaskClip>, VendorPrefix) [comma_separated: MaskClip],
    "mask-origin": MaskOrigin(Vec<'a, GeometryBox>, VendorPrefix) [comma_separated: GeometryBox],
    "mask-size": MaskSize(Vec<'a, BackgroundSize<'a>>, VendorPrefix) [comma_separated: BackgroundSize<'i>],
    "mask-composite": MaskComposite(Vec<'a, MaskComposite>) [comma_separated: MaskComposite],
    "mask-type": MaskType(MaskType) [parse: MaskType],
    "mask": Mask(Vec<'a, Mask<'a>>, VendorPrefix) [comma_separated: Mask<'i>],
    "mask-border-source": MaskBorderSource(Box<'a, Image<'a>>) [boxed: Image<'i>],
    "mask-border-mode": MaskBorderMode(MaskBorderMode) [unsupported],
    "mask-border-slice": MaskBorderSlice(Box<'a, BorderImageSlice<'a>>) [unsupported],
    "mask-border-width": MaskBorderWidth(Box<'a, Rect<'a, BorderImageSideWidth<'a>>>) [unsupported],
    "mask-border-outset": MaskBorderOutset(Box<'a, Rect<'a, LengthOrNumber<'a>>>) [unsupported],
    "mask-border-repeat": MaskBorderRepeat(BorderImageRepeat) [unsupported],
    "mask-border": MaskBorder(Box<'a, MaskBorder<'a>>) [unsupported],
    "-webkit-mask-composite": WebKitMaskComposite(Vec<'a, WebKitMaskComposite>) [comma_separated: WebKitMaskComposite],
    "mask-source-type": WebKitMaskSourceType(Vec<'a, WebKitMaskSourceType>, VendorPrefix) [comma_separated: WebKitMaskSourceType],
    "mask-box-image": WebKitMaskBoxImage(Box<'a, BorderImage<'a>>, VendorPrefix) [unsupported],
    "mask-box-image-source": WebKitMaskBoxImageSource(Box<'a, Image<'a>>, VendorPrefix) [boxed: Image<'i>],
    "mask-box-image-slice": WebKitMaskBoxImageSlice(Box<'a, BorderImageSlice<'a>>, VendorPrefix) [unsupported],
    "mask-box-image-width": WebKitMaskBoxImageWidth(Box<'a, Rect<'a, BorderImageSideWidth<'a>>>, VendorPrefix) [unsupported],
    "mask-box-image-outset": WebKitMaskBoxImageOutset(Box<'a, Rect<'a, LengthOrNumber<'a>>>, VendorPrefix) [unsupported],
    "mask-box-image-repeat": WebKitMaskBoxImageRepeat(BorderImageRepeat, VendorPrefix) [unsupported],
    "filter": Filter(Box<'a, FilterList<'a>>, VendorPrefix) [unsupported],
    "backdrop-filter": BackdropFilter(Box<'a, FilterList<'a>>, VendorPrefix) [unsupported],
    "mix-blend-mode": MixBlendMode(BlendMode) [unsupported],
    "z-index": ZIndex(ZIndex) [parse: ZIndex],
    "container-type": ContainerType(ContainerType) [unsupported],
    "container-name": ContainerName(Box<'a, ContainerNameList<'a>>) [unsupported],
    "container": Container(Box<'a, Container<'a>>) [unsupported],
    "view-transition-name": ViewTransitionName(Box<'a, ViewTransitionName<'a>>) [unsupported],
    "view-transition-class": ViewTransitionClass(Box<'a, NoneOrCustomIdentList<'a>>) [unsupported],
    "view-transition-group": ViewTransitionGroup(Box<'a, ViewTransitionGroup<'a>>) [unsupported],
    "color-scheme": ColorScheme(ColorScheme) [unsupported],
    "print-color-adjust": PrintColorAdjust(PrintColorAdjust, VendorPrefix) [unsupported],
        }
    };
}

for_each_property!(define_properties);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_webkit_round_trip {
        ($name:literal, $property:ident, $vendor_prefix:ty) => {{
            let prefixed_name = concat!("-webkit-", $name);
            assert!(matches!(
                PropertyId::from_name(prefixed_name),
                PropertyId::$property(prefix) if prefix == VendorPrefix::WEBKIT
            ));
        }};
        ($name:literal, $property:ident) => {{
            // Explicitly prefixed aliases, such as `-webkit-mask-composite`,
            // are separate metadata entries and may legitimately make this
            // lookup resolve to another known property.
            let _ = ($name, stringify!($property));
        }};
    }

    macro_rules! metadata_tests {
        (
            $(
                $(#[$meta:meta])*
                $name:literal: $property:ident($value:ty $(, $vp:ty)?)
                    [$strategy:ident $( : $($strategy_args:tt)+)?],
            )+
        ) => {
            #[test]
            fn every_property_entry_has_one_generated_identity_and_strategy() {
                $(
                    let property_id = PropertyId::from_name($name);
                    assert!(matches!(
                        property_id,
                        property_id_pattern!(PropertyId::$property $(, $vp)?)
                    ));
                    assert!(property_id.known_id().is_some(), "{name} has no known ID", name = $name);
                    assert_eq!(
                        property_id.parser_strategy(),
                        property_parser_strategy!($strategy $( : $($strategy_args)+ )?)
                    );
                    assert_eq!(
                        property_id.support_classification(),
                        property_id.parser_strategy().support_classification()
                    );
                    assert_webkit_round_trip!($name, $property $(, $vp)?);

                    let uppercase_name = $name.to_ascii_uppercase();
                    let uppercase_id = PropertyId::from_name(&uppercase_name);
                    assert_eq!(property_id.known_id(), uppercase_id.known_id(), "{name}", name = $name);
                )+
            }
        };
    }

    for_each_property!(metadata_tests);
}
