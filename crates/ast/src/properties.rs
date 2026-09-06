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
    (node : $value:ty) => {
        PropertyParserStrategy::Node
    };
    (comma_separated : $value:ty) => {
        PropertyParserStrategy::CommaSeparated
    };
    (comma_separated_node : $value:ty) => {
        PropertyParserStrategy::CommaSeparatedNode
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
            Custom(AstStr<'a>),
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
            CSSWide(NodeId<'a, PropertyId<'a>>, CSSWideKeyword),
            Unparsed(NodeId<'a, UnparsedProperty<'a>>),
            Custom(NodeId<'a, CustomProperty<'a>>),
            /// Tombstone for a declaration removed by an in-place transform.
            Tombstone,
        }

        impl<'a> PropertyId<'a> {
            /// Resolves a property name while retaining unknown names for lossless parsing.
            #[inline]
            pub fn from_name(name: &str, context: &mut AstContext<'a>) -> Self {
                Self::from_known_name(name).unwrap_or_else(|| Self::Custom(context.add_str(name)))
            }

            /// Resolves known metadata without constructing or storing unknown text.
            pub fn from_known_name(name: &str) -> Option<Self> {
                // Dashed names belong to the custom-property namespace.
                if name.starts_with("--") { return None; }
                let property_id = match_ignore_ascii_case!(
                    name,
                    $($name => Some(Self::$property$( (<$vp>::default()) )?),)+
                    _ => None,
                );
                if let Some(property_id) = property_id {
                    return Some(property_id);
                }

                if let Some((prefix, unprefixed_name)) = VendorPrefix::split_from_name(name) {
                    let property_id = match_ignore_ascii_case!(
                        unprefixed_name,
                        $($name => property_id_with_vendor_prefix!(Self::$property, prefix$(, $vp)?),)+
                        _ => None,
                    );
                    if let Some(property_id) = property_id {
                        return Some(property_id);
                    }
                }

                None
            }

            /// Returns a static metadata name without borrowing a string pool.
            pub fn known_name(&self) -> Option<&'static str> {
                match self {
                    $(property_id_pattern!(Self::$property$(, $vp)?) => Some($name),)+
                    Self::Unparsed => Some(""),
                    Self::Custom(_) => None,
                }
            }

            /// Returns the canonical metadata name or lossless unknown text.
            pub fn name<'cx>(&self, context: &'cx AstContext<'_>) -> &'cx str {
                match self {
                    Self::Custom(name) => context.str(*name),
                    _ => self.known_name().expect("non-custom property has a metadata name"),
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

        unsafe impl<'ast> AstNodeStorage<'ast> for PropertyId<'ast> {
            const KIND: NodeKind = NodeKind::new(0x0007_0001);
            #[inline]
            unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
                unsafe { payload.read_value() }
            }
            #[inline]
            fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
                NodePayload::from_value(self)
            }
            #[inline]
            unsafe fn encode_existing(self, _current: NodePayload, _context: &mut AstContext<'ast>) -> NodePayload {
                NodePayload::from_value(self)
            }
            fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
                match (self, other) {
                    (Self::Custom(left), Self::Custom(right)) => left == right || context.str(*left) == context.str(*right),
                    _ => self == other,
                }
            }
        }

        impl<'ast> AstNodeClone<'ast> for PropertyId<'ast> {
            fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
                self
            }
        }

        impl<'a> Declaration<'a> {
            /// Returns the compact discriminant and vendor prefix of a known declaration.
            ///
            /// This performs one typed dispatch without constructing a `PropertyId`.
            #[inline]
            pub fn known_id_and_prefix(&self, ast: &AstContext<'_>) -> Option<(u32, VendorPrefix)> {
                match self {
                    $(declaration_pattern!(Self::$property, _value$(, vendor_prefix: $vp)?) => Some((KnownPropertyDiscriminant::$property as u32, declaration_prefix!($(vendor_prefix: $vp)?))),)+
                    Self::CSSWide(property_id, _) => ast.resolve_node(*property_id).known_id_and_prefix(),
                    Self::Unparsed(value) => ast.resolve_node(ast.unparsed_property(*value).property_id()).known_id_and_prefix(),
                    Self::Custom(_) | Self::Tombstone => None,
                }
            }

            /// Returns the typed identity of this declaration.
            #[inline]
            pub fn property_id(&self, ast: &AstContext<'_>) -> Option<PropertyId<'a>> {
                match self {
                    $(declaration_pattern!(Self::$property, _value$(, vendor_prefix: $vp)?) => Some(declaration_property_id!(PropertyId::$property$(, vendor_prefix: $vp)?)),)+
                    Self::CSSWide(property_id, _) => Some(ast.resolve_node(*property_id)),
                    Self::Unparsed(value) => Some(ast.resolve_node(ast.unparsed_property(*value).property_id())),
                    Self::Custom(value) => Some(PropertyId::Custom(match ast.resolve_node(ast.resolve_node(*value).name) {
                        CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
                    })),
                    Self::Tombstone => None,
                }
            }

            /// Returns the canonical CSS property name.
            pub fn name<'cx>(&self, ast: &'cx AstContext<'_>) -> &'cx str {
                match self {
                    $(Self::$property(..) => $name,)+
                    Self::CSSWide(property_id, _) => ast.resolve_node(*property_id).name(ast),
                    Self::Unparsed(value) => ast.resolve_node(ast.unparsed_property(*value).property_id()).name(ast),
                    Self::Custom(value) => match ast.resolve_node(ast.resolve_node(*value).name) {
                        CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => ast.str(name),
                    },
                    Self::Tombstone => "",
                }
            }

            /// Returns the vendor prefix associated with this declaration.
            pub fn vendor_prefix(&self, ast: &AstContext<'_>) -> VendorPrefix {
                match self {
                    $(declaration_pattern!(Self::$property, _value$(, vendor_prefix: $vp)?) => declaration_prefix!($(vendor_prefix: $vp)?),)+
                    Self::CSSWide(property_id, _) => ast.resolve_node(*property_id).vendor_prefix(),
                    Self::Unparsed(value) => ast.resolve_node(ast.unparsed_property(*value).property_id()).vendor_prefix(),
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
            fn eq_ignoring_tombstones(&self, other: &Self, ast: &AstContext<'_>) -> bool {
                match (self, other) {
                    (Self::FontFamily(left), Self::FontFamily(right)) => {
                        left.eq_ignoring_tombstones(right, ast)
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
    Node,
    CommaSeparated,
    CommaSeparatedNode,
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
            | Self::Node
            | Self::CommaSeparated
            | Self::CommaSeparatedNode
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
    "background-color": BackgroundColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "background-image": BackgroundImage(Vec<'a, Image<'a>>) [comma_separated: Image<'i>],
    "background-position-x": BackgroundPositionX(Vec<'a, PositionComponent<'a, HorizontalPositionKeyword>>) [comma_separated: PositionComponent<'i, HorizontalPositionKeyword>],
    "background-position-y": BackgroundPositionY(Vec<'a, PositionComponent<'a, VerticalPositionKeyword>>) [comma_separated: PositionComponent<'i, VerticalPositionKeyword>],
    "background-position": BackgroundPosition(Vec<'a, BackgroundPosition<'a>>) [comma_separated: BackgroundPosition<'i>],
    "background-size": BackgroundSize(Vec<'a, NodeId<'a, BackgroundSize<'a>>>) [comma_separated_node: BackgroundSize<'i>],
    "background-repeat": BackgroundRepeat(Vec<'a, BackgroundRepeat>) [comma_separated: BackgroundRepeat],
    "background-attachment": BackgroundAttachment(Vec<'a, BackgroundAttachment>) [comma_separated: BackgroundAttachment],
    "background-clip": BackgroundClip(Vec<'a, BackgroundClip>, VendorPrefix) [comma_separated: BackgroundClip],
    "background-origin": BackgroundOrigin(Vec<'a, BackgroundOrigin>) [comma_separated: BackgroundOrigin],
    "background": Background(Vec<'a, NodeId<'a, Background<'a>>>) [custom: parse_background],
    "box-shadow": BoxShadow(Vec<'a, NodeId<'a, BoxShadow<'a>>>, VendorPrefix) [unsupported],
    "opacity": Opacity(f32) [custom: parse_opacity],
    "color": Color(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "display": Display(Display) [parse: Display],
    "visibility": Visibility(Visibility) [parse: Visibility],
    "width": Width(NodeId<'a, Size<'a>>) [boxed: Size<'i>],
    "height": Height(NodeId<'a, Size<'a>>) [boxed: Size<'i>],
    "min-width": MinWidth(NodeId<'a, Size<'a>>) [boxed: Size<'i>],
    "min-height": MinHeight(NodeId<'a, Size<'a>>) [boxed: Size<'i>],
    "max-width": MaxWidth(NodeId<'a, MaxSize<'a>>) [boxed: MaxSize<'i>],
    "max-height": MaxHeight(NodeId<'a, MaxSize<'a>>) [boxed: MaxSize<'i>],
    "block-size": BlockSize(NodeId<'a, Size<'a>>) [boxed: Size<'i>],
    "inline-size": InlineSize(NodeId<'a, Size<'a>>) [boxed: Size<'i>],
    "min-block-size": MinBlockSize(NodeId<'a, Size<'a>>) [boxed: Size<'i>],
    "min-inline-size": MinInlineSize(NodeId<'a, Size<'a>>) [boxed: Size<'i>],
    "max-block-size": MaxBlockSize(NodeId<'a, MaxSize<'a>>) [boxed: MaxSize<'i>],
    "max-inline-size": MaxInlineSize(NodeId<'a, MaxSize<'a>>) [boxed: MaxSize<'i>],
    "box-sizing": BoxSizing(BoxSizing, VendorPrefix) [parse: BoxSizing],
    "aspect-ratio": AspectRatio(AspectRatio) [parse: AspectRatio],
    "overflow": Overflow(Overflow) [parse: Overflow],
    "overflow-x": OverflowX(OverflowKeyword) [parse: OverflowKeyword],
    "overflow-y": OverflowY(OverflowKeyword) [parse: OverflowKeyword],
    "text-overflow": TextOverflow(TextOverflow, VendorPrefix) [parse: TextOverflow],
    "object-fit": ObjectFit(ObjectFit) [parse: ObjectFit],
    "object-position": ObjectPosition(NodeId<'a, Position<'a>>) [boxed: Position<'i>],
    "scrollbar-color": ScrollbarColor(ScrollbarColor<'a>) [parse: ScrollbarColor<'i>],
    "position": Position(NodeId<'a, PositionProperty>) [boxed: PositionProperty],
    "top": Top(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "bottom": Bottom(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "left": Left(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "right": Right(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-block-start": InsetBlockStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-block-end": InsetBlockEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-inline-start": InsetInlineStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-inline-end": InsetInlineEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "inset-block": InsetBlock(NodeId<'a, InsetBlock<'a>>) [two_value: InsetBlock<'i>],
    "inset-inline": InsetInline(NodeId<'a, InsetInline<'a>>) [two_value: InsetInline<'i>],
    "inset": Inset(NodeId<'a, Inset<'a>>) [rect: Inset<'i>],
    "border-spacing": BorderSpacing(NodeId<'a, Size2D<'a, Length<'a>>>) [two_value: Size2D<'i, Length<'i>>],
    "border-top-color": BorderTopColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "border-bottom-color": BorderBottomColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "border-left-color": BorderLeftColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "border-right-color": BorderRightColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "border-block-start-color": BorderBlockStartColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "border-block-end-color": BorderBlockEndColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "border-inline-start-color": BorderInlineStartColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "border-inline-end-color": BorderInlineEndColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "border-top-style": BorderTopStyle(LineStyle) [parse: LineStyle],
    "border-bottom-style": BorderBottomStyle(LineStyle) [parse: LineStyle],
    "border-left-style": BorderLeftStyle(LineStyle) [parse: LineStyle],
    "border-right-style": BorderRightStyle(LineStyle) [parse: LineStyle],
    "border-block-start-style": BorderBlockStartStyle(LineStyle) [parse: LineStyle],
    "border-block-end-style": BorderBlockEndStyle(LineStyle) [parse: LineStyle],
    "border-inline-start-style": BorderInlineStartStyle(LineStyle) [parse: LineStyle],
    "border-inline-end-style": BorderInlineEndStyle(LineStyle) [parse: LineStyle],
    "border-top-width": BorderTopWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-bottom-width": BorderBottomWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-left-width": BorderLeftWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-right-width": BorderRightWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-block-start-width": BorderBlockStartWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-block-end-width": BorderBlockEndWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-inline-start-width": BorderInlineStartWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-inline-end-width": BorderInlineEndWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "border-top-left-radius": BorderTopLeftRadius(NodeId<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-top-right-radius": BorderTopRightRadius(NodeId<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-bottom-left-radius": BorderBottomLeftRadius(NodeId<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-bottom-right-radius": BorderBottomRightRadius(NodeId<'a, Size2D<'a, LengthPercentage<'a>>>, VendorPrefix) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-start-start-radius": BorderStartStartRadius(NodeId<'a, Size2D<'a, LengthPercentage<'a>>>) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-start-end-radius": BorderStartEndRadius(NodeId<'a, Size2D<'a, LengthPercentage<'a>>>) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-end-start-radius": BorderEndStartRadius(NodeId<'a, Size2D<'a, LengthPercentage<'a>>>) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-end-end-radius": BorderEndEndRadius(NodeId<'a, Size2D<'a, LengthPercentage<'a>>>) [two_value: Size2D<'i, LengthPercentage<'i>>],
    "border-radius": BorderRadius(NodeId<'a, BorderRadius<'a>>, VendorPrefix) [rect: BorderRadius<'i>],
    "border-image-source": BorderImageSource(NodeId<'a, Image<'a>>) [boxed: Image<'i>],
    "border-image-outset": BorderImageOutset(NodeId<'a, Rect<'a, LengthOrNumber<'a>>>) [unsupported],
    "border-image-repeat": BorderImageRepeat(BorderImageRepeat) [unsupported],
    "border-image-width": BorderImageWidth(NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>) [unsupported],
    "border-image-slice": BorderImageSlice(NodeId<'a, BorderImageSlice<'a>>) [unsupported],
    "border-image": BorderImage(NodeId<'a, BorderImage<'a>>, VendorPrefix) [unsupported],
    "border-color": BorderColor(NodeId<'a, BorderColor<'a>>) [rect: BorderColor<'i>],
    "border-style": BorderStyle(NodeId<'a, BorderStyle>) [rect: BorderStyle],
    "border-width": BorderWidth(NodeId<'a, BorderWidth<'a>>) [rect: BorderWidth<'i>],
    "border-block-color": BorderBlockColor(NodeId<'a, BorderBlockColor<'a>>) [two_value: BorderBlockColor<'i>],
    "border-block-style": BorderBlockStyle(NodeId<'a, BorderBlockStyle>) [two_value: BorderBlockStyle],
    "border-block-width": BorderBlockWidth(NodeId<'a, BorderBlockWidth<'a>>) [two_value: BorderBlockWidth<'i>],
    "border-inline-color": BorderInlineColor(NodeId<'a, BorderInlineColor<'a>>) [two_value: BorderInlineColor<'i>],
    "border-inline-style": BorderInlineStyle(NodeId<'a, BorderInlineStyle>) [two_value: BorderInlineStyle],
    "border-inline-width": BorderInlineWidth(NodeId<'a, BorderInlineWidth<'a>>) [two_value: BorderInlineWidth<'i>],
    "border": Border(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-top": BorderTop(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-bottom": BorderBottom(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-left": BorderLeft(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-right": BorderRight(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-block": BorderBlock(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-block-start": BorderBlockStart(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-block-end": BorderBlockEnd(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-inline": BorderInline(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-inline-start": BorderInlineStart(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "border-inline-end": BorderInlineEnd(NodeId<'a, GenericBorder<'a, LineStyle>>) [boxed: GenericBorder<'i, LineStyle>],
    "outline": Outline(NodeId<'a, GenericBorder<'a, OutlineStyle>>) [boxed: GenericBorder<'i, OutlineStyle>],
    "outline-color": OutlineColor(NodeId<'a, CssColor<'a>>) [node: CssColor<'i>],
    "outline-style": OutlineStyle(OutlineStyle) [parse: OutlineStyle],
    "outline-width": OutlineWidth(NodeId<'a, BorderSideWidth<'a>>) [boxed: BorderSideWidth<'i>],
    "flex-direction": FlexDirection(FlexDirection, VendorPrefix) [parse: FlexDirection],
    "flex-wrap": FlexWrap(FlexWrap, VendorPrefix) [parse: FlexWrap],
    "flex-flow": FlexFlow(NodeId<'a, FlexFlow>, VendorPrefix) [whitespace_separated: FlexFlow],
    "flex-grow": FlexGrow(f32, VendorPrefix) [custom: parse_flex_grow],
    "flex-shrink": FlexShrink(f32, VendorPrefix) [custom: parse_flex_shrink],
    "flex-basis": FlexBasis(NodeId<'a, LengthPercentageOrAuto<'a>>, VendorPrefix) [boxed: LengthPercentageOrAuto<'i>],
    "flex": Flex(NodeId<'a, Flex<'a>>, VendorPrefix) [boxed: Flex<'i>],
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
    "row-gap": RowGap(NodeId<'a, GapValue<'a>>) [boxed: GapValue<'i>],
    "column-gap": ColumnGap(NodeId<'a, GapValue<'a>>) [boxed: GapValue<'i>],
    "gap": Gap(NodeId<'a, Gap<'a>>) [two_value: Gap<'i>],
    "column-rule": ColumnRule(NodeId<'a, ColumnRule<'a>>, VendorPrefix) [boxed: ColumnRule<'i>],
    "column-width": ColumnWidth(CSSWideOr<ColumnWidth<'a>>, VendorPrefix) [css_wide: ColumnWidth<'i>],
    "column-count": ColumnCount(CSSWideOr<ColumnCount>, VendorPrefix) [css_wide: ColumnCount],
    "columns": Columns(CSSWideOr<NodeId<'a, Columns<'a>>>, VendorPrefix) [css_wide_boxed: Columns<'i>],
    "grid-column-gap": GridColumnGap(NodeId<'a, GapValue<'a>>) [boxed: GapValue<'i>],
    "grid-row-gap": GridRowGap(NodeId<'a, GapValue<'a>>) [boxed: GapValue<'i>],
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
    "flex-preferred-size": FlexPreferredSize(NodeId<'a, LengthPercentageOrAuto<'a>>, VendorPrefix) [boxed: LengthPercentageOrAuto<'i>],
    "grid-template-columns": GridTemplateColumns(NodeId<'a, TrackSizing<'a>>) [unsupported],
    "grid-template-rows": GridTemplateRows(NodeId<'a, TrackSizing<'a>>) [unsupported],
    "grid-auto-columns": GridAutoColumns(Vec<'a, NodeId<'a, TrackSize<'a>>>) [unsupported],
    "grid-auto-rows": GridAutoRows(Vec<'a, NodeId<'a, TrackSize<'a>>>) [unsupported],
    "grid-auto-flow": GridAutoFlow(GridAutoFlow) [unsupported],
    "grid-template-areas": GridTemplateAreas(NodeId<'a, GridTemplateAreas<'a>>) [unsupported],
    "grid-template": GridTemplate(NodeId<'a, GridTemplate<'a>>) [unsupported],
    "grid": Grid(NodeId<'a, Grid<'a>>) [unsupported],
    "grid-row-start": GridRowStart(NodeId<'a, GridLine<'a>>) [unsupported],
    "grid-row-end": GridRowEnd(NodeId<'a, GridLine<'a>>) [unsupported],
    "grid-column-start": GridColumnStart(NodeId<'a, GridLine<'a>>) [unsupported],
    "grid-column-end": GridColumnEnd(NodeId<'a, GridLine<'a>>) [unsupported],
    "grid-row": GridRow(NodeId<'a, GridRow<'a>>) [unsupported],
    "grid-column": GridColumn(NodeId<'a, GridColumn<'a>>) [unsupported],
    "grid-area": GridArea(NodeId<'a, GridArea<'a>>) [unsupported],
    "margin-top": MarginTop(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-bottom": MarginBottom(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-left": MarginLeft(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-right": MarginRight(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-block-start": MarginBlockStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-block-end": MarginBlockEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-inline-start": MarginInlineStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-inline-end": MarginInlineEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "margin-block": MarginBlock(NodeId<'a, MarginBlock<'a>>) [two_value: MarginBlock<'i>],
    "margin-inline": MarginInline(NodeId<'a, MarginInline<'a>>) [two_value: MarginInline<'i>],
    "margin": Margin(NodeId<'a, Margin<'a>>) [rect: Margin<'i>],
    "padding-top": PaddingTop(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-bottom": PaddingBottom(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-left": PaddingLeft(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-right": PaddingRight(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-block-start": PaddingBlockStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-block-end": PaddingBlockEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-inline-start": PaddingInlineStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-inline-end": PaddingInlineEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "padding-block": PaddingBlock(NodeId<'a, PaddingBlock<'a>>) [two_value: PaddingBlock<'i>],
    "padding-inline": PaddingInline(NodeId<'a, PaddingInline<'a>>) [two_value: PaddingInline<'i>],
    "padding": Padding(NodeId<'a, Padding<'a>>) [rect: Padding<'i>],
    "scroll-margin-top": ScrollMarginTop(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-bottom": ScrollMarginBottom(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-left": ScrollMarginLeft(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-right": ScrollMarginRight(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-block-start": ScrollMarginBlockStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-block-end": ScrollMarginBlockEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-inline-start": ScrollMarginInlineStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-inline-end": ScrollMarginInlineEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-margin-block": ScrollMarginBlock(NodeId<'a, ScrollMarginBlock<'a>>) [two_value: ScrollMarginBlock<'i>],
    "scroll-margin-inline": ScrollMarginInline(NodeId<'a, ScrollMarginInline<'a>>) [two_value: ScrollMarginInline<'i>],
    "scroll-margin": ScrollMargin(NodeId<'a, ScrollMargin<'a>>) [rect: ScrollMargin<'i>],
    "scroll-padding-top": ScrollPaddingTop(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-bottom": ScrollPaddingBottom(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-left": ScrollPaddingLeft(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-right": ScrollPaddingRight(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-block-start": ScrollPaddingBlockStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-block-end": ScrollPaddingBlockEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-inline-start": ScrollPaddingInlineStart(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-inline-end": ScrollPaddingInlineEnd(NodeId<'a, LengthPercentageOrAuto<'a>>) [boxed: LengthPercentageOrAuto<'i>],
    "scroll-padding-block": ScrollPaddingBlock(NodeId<'a, ScrollPaddingBlock<'a>>) [two_value: ScrollPaddingBlock<'i>],
    "scroll-padding-inline": ScrollPaddingInline(NodeId<'a, ScrollPaddingInline<'a>>) [two_value: ScrollPaddingInline<'i>],
    "scroll-padding": ScrollPadding(NodeId<'a, ScrollPadding<'a>>) [rect: ScrollPadding<'i>],
    "font-weight": FontWeight(FontWeight) [parse: FontWeight],
    "font-size": FontSize(NodeId<'a, FontSize<'a>>) [boxed: FontSize<'i>],
    "font-stretch": FontStretch(FontStretch) [parse: FontStretch],
    "font-family": FontFamily(Vec<'a, NodeId<'a, FontFamily<'a>>>) [custom: parse_font_family],
    "font-style": FontStyle(FontStyle) [parse: FontStyle],
    "font-variant-caps": FontVariantCaps(FontVariantCaps) [parse: FontVariantCaps],
    "line-height": LineHeight(NodeId<'a, LineHeight<'a>>) [boxed: LineHeight<'i>],
    "font": Font(NodeId<'a, Font<'a>>) [unsupported],
    "vertical-align": VerticalAlign(NodeId<'a, VerticalAlign<'a>>) [unsupported],
    "font-palette": FontPalette(NodeId<'a, DashedIdentReference<'a>>) [unsupported],
    "transition-property": TransitionProperty(Vec<'a, NodeId<'a, PropertyId<'a>>>, VendorPrefix) [custom: parse_transition_property],
    "transition-duration": TransitionDuration(Vec<'a, Time>, VendorPrefix) [custom: parse_transition_duration],
    "transition-delay": TransitionDelay(Vec<'a, Time>, VendorPrefix) [custom: parse_transition_delay],
    "transition-timing-function": TransitionTimingFunction(Vec<'a, NodeId<'a, EasingFunction>>, VendorPrefix) [custom: parse_transition_timing],
    "transition": Transition(Vec<'a, NodeId<'a, Transition<'a>>>, VendorPrefix) [custom: parse_transition],
    "animation-name": AnimationName(Vec<'a, NodeId<'a, AnimationName<'a>>>, VendorPrefix) [custom: parse_animation_name],
    "animation-duration": AnimationDuration(Vec<'a, Time>, VendorPrefix) [custom: parse_animation_duration],
    "animation-timing-function": AnimationTimingFunction(Vec<'a, NodeId<'a, EasingFunction>>, VendorPrefix) [custom: parse_animation_timing],
    "animation-iteration-count": AnimationIterationCount(Vec<'a, AnimationIterationCount>, VendorPrefix) [custom: parse_animation_iteration],
    "animation-direction": AnimationDirection(Vec<'a, AnimationDirection>, VendorPrefix) [custom: parse_animation_direction],
    "animation-play-state": AnimationPlayState(Vec<'a, AnimationPlayState>, VendorPrefix) [custom: parse_animation_play_state],
    "animation-delay": AnimationDelay(Vec<'a, Time>, VendorPrefix) [custom: parse_animation_delay],
    "animation-fill-mode": AnimationFillMode(Vec<'a, AnimationFillMode>, VendorPrefix) [custom: parse_animation_fill],
    "animation-composition": AnimationComposition(Vec<'a, AnimationComposition>) [unsupported],
    "animation-timeline": AnimationTimeline(Vec<'a, NodeId<'a, AnimationTimeline<'a>>>) [unsupported],
    "animation-range-start": AnimationRangeStart(Vec<'a, AnimationRangeStart<'a>>) [unsupported],
    "animation-range-end": AnimationRangeEnd(Vec<'a, AnimationRangeEnd<'a>>) [unsupported],
    "animation-range": AnimationRange(Vec<'a, AnimationRange<'a>>) [unsupported],
    "animation": Animation(Vec<'a, Animation<'a>>, VendorPrefix) [custom: parse_animation],
    "transform": Transform(Vec<'a, NodeId<'a, Transform<'a>>>, VendorPrefix) [custom: parse_transform],
    "transform-origin": TransformOrigin(NodeId<'a, Position<'a>>, VendorPrefix) [boxed: Position<'i>],
    "transform-style": TransformStyle(TransformStyle, VendorPrefix) [parse: TransformStyle],
    "transform-box": TransformBox(TransformBox) [parse: TransformBox],
    "backface-visibility": BackfaceVisibility(BackfaceVisibility, VendorPrefix) [parse: BackfaceVisibility],
    "perspective": Perspective(NodeId<'a, Perspective<'a>>, VendorPrefix) [boxed: Perspective<'i>],
    "perspective-origin": PerspectiveOrigin(NodeId<'a, Position<'a>>, VendorPrefix) [boxed: Position<'i>],
    "translate": Translate(NodeId<'a, Translate<'a>>) [boxed: Translate<'i>],
    "rotate": Rotate(Rotate) [parse: Rotate],
    "scale": Scale(NodeId<'a, Scale>) [boxed: Scale],
    "text-transform": TextTransform(TextTransform) [parse: TextTransform],
    "content": Content(NodeId<'a, Content<'a>>) [unsupported],
    "white-space": WhiteSpace(WhiteSpace) [parse: WhiteSpace],
    "tab-size": TabSize(NodeId<'a, LengthOrNumber<'a>>, VendorPrefix) [unsupported],
    "word-break": WordBreak(WordBreak) [parse: WordBreak],
    "line-break": LineBreak(LineBreak) [parse: LineBreak],
    "hyphens": Hyphens(Hyphens, VendorPrefix) [parse: Hyphens],
    "overflow-wrap": OverflowWrap(OverflowWrap) [parse: OverflowWrap],
    "word-wrap": WordWrap(OverflowWrap) [parse: OverflowWrap],
    "text-align": TextAlign(TextAlign) [parse: TextAlign],
    "text-align-last": TextAlignLast(TextAlignLast, VendorPrefix) [parse: TextAlignLast],
    "text-justify": TextJustify(TextJustify) [parse: TextJustify],
    "word-spacing": WordSpacing(NodeId<'a, Spacing<'a>>) [boxed: Spacing<'i>],
    "letter-spacing": LetterSpacing(NodeId<'a, Spacing<'a>>) [boxed: Spacing<'i>],
    "text-indent": TextIndent(NodeId<'a, TextIndent<'a>>) [boxed: TextIndent<'i>],
    "text-decoration-line": TextDecorationLine(NodeId<'a, TextDecorationLine<'a>>, VendorPrefix) [boxed: TextDecorationLine<'i>],
    "text-decoration-style": TextDecorationStyle(TextDecorationStyle, VendorPrefix) [parse: TextDecorationStyle],
    "text-decoration-color": TextDecorationColor(NodeId<'a, CssColor<'a>>, VendorPrefix) [node: CssColor<'i>],
    "text-decoration-thickness": TextDecorationThickness(NodeId<'a, TextDecorationThickness<'a>>) [boxed: TextDecorationThickness<'i>],
    "text-decoration": TextDecoration(NodeId<'a, TextDecoration<'a>>, VendorPrefix) [unsupported],
    "text-decoration-skip-ink": TextDecorationSkipInk(TextDecorationSkipInk, VendorPrefix) [parse: TextDecorationSkipInk],
    "text-emphasis-style": TextEmphasisStyle(NodeId<'a, TextEmphasisStyle<'a>>, VendorPrefix) [unsupported],
    "text-emphasis-color": TextEmphasisColor(NodeId<'a, CssColor<'a>>, VendorPrefix) [node: CssColor<'i>],
    "text-emphasis": TextEmphasis(NodeId<'a, TextEmphasis<'a>>, VendorPrefix) [unsupported],
    "text-emphasis-position": TextEmphasisPosition(TextEmphasisPosition, VendorPrefix) [unsupported],
    "text-shadow": TextShadow(Vec<'a, NodeId<'a, TextShadow<'a>>>) [unsupported],
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
    "cursor": Cursor(NodeId<'a, Cursor<'a>>) [unsupported],
    "caret-color": CaretColor(NodeId<'a, ColorOrAuto<'a>>) [unsupported],
    "caret-shape": CaretShape(CaretShape) [unsupported],
    "caret": Caret(NodeId<'a, Caret<'a>>) [unsupported],
    "user-select": UserSelect(UserSelect, VendorPrefix) [parse: UserSelect],
    "accent-color": AccentColor(NodeId<'a, ColorOrAuto<'a>>) [boxed: ColorOrAuto<'i>],
    "appearance": Appearance(NodeId<'a, Appearance<'a>>, VendorPrefix) [unsupported],
    "list-style-type": ListStyleType(NodeId<'a, ListStyleType<'a>>) [unsupported],
    "list-style-image": ListStyleImage(NodeId<'a, Image<'a>>) [boxed: Image<'i>],
    "list-style-position": ListStylePosition(ListStylePosition) [unsupported],
    "list-style": ListStyle(NodeId<'a, ListStyle<'a>>) [unsupported],
    "marker-side": MarkerSide(MarkerSide) [unsupported],
    "composes": Composes(NodeId<'a, Composes<'a>>) [unsupported],
    "fill": Fill(NodeId<'a, SVGPaint<'a>>) [boxed: SVGPaint<'i>],
    "fill-rule": FillRule(FillRule) [parse: FillRule],
    "fill-opacity": FillOpacity(f32) [custom: parse_fill_opacity],
    "stroke": Stroke(NodeId<'a, SVGPaint<'a>>) [boxed: SVGPaint<'i>],
    "stroke-opacity": StrokeOpacity(f32) [custom: parse_stroke_opacity],
    "stroke-width": StrokeWidth(NodeId<'a, LengthPercentage<'a>>) [boxed: LengthPercentage<'i>],
    "stroke-linecap": StrokeLinecap(StrokeLinecap) [parse: StrokeLinecap],
    "stroke-linejoin": StrokeLinejoin(StrokeLinejoin) [parse: StrokeLinejoin],
    "stroke-miterlimit": StrokeMiterlimit(f32) [custom: parse_stroke_miterlimit],
    "stroke-dasharray": StrokeDasharray(NodeId<'a, StrokeDasharray<'a>>) [boxed: StrokeDasharray<'i>],
    "stroke-dashoffset": StrokeDashoffset(NodeId<'a, LengthPercentage<'a>>) [boxed: LengthPercentage<'i>],
    "marker-start": MarkerStart(NodeId<'a, Marker<'a>>) [boxed: Marker<'i>],
    "marker-mid": MarkerMid(NodeId<'a, Marker<'a>>) [boxed: Marker<'i>],
    "marker-end": MarkerEnd(NodeId<'a, Marker<'a>>) [boxed: Marker<'i>],
    "marker": Marker(NodeId<'a, Marker<'a>>) [boxed: Marker<'i>],
    "color-interpolation": ColorInterpolation(ColorInterpolation) [parse: ColorInterpolation],
    "color-interpolation-filters": ColorInterpolationFilters(ColorInterpolation) [parse: ColorInterpolation],
    "color-rendering": ColorRendering(ColorRendering) [parse: ColorRendering],
    "shape-rendering": ShapeRendering(ShapeRendering) [parse: ShapeRendering],
    "text-rendering": TextRendering(TextRendering) [parse: TextRendering],
    "image-rendering": ImageRendering(ImageRendering) [parse: ImageRendering],
    "clip-path": ClipPath(NodeId<'a, ClipPath<'a>>, VendorPrefix) [unsupported],
    "clip-rule": ClipRule(FillRule) [unsupported],
    "mask-image": MaskImage(Vec<'a, Image<'a>>, VendorPrefix) [comma_separated: Image<'i>],
    "mask-mode": MaskMode(Vec<'a, MaskMode>) [comma_separated: MaskMode],
    "mask-repeat": MaskRepeat(Vec<'a, BackgroundRepeat>, VendorPrefix) [comma_separated: BackgroundRepeat],
    "mask-position-x": MaskPositionX(Vec<'a, PositionComponent<'a, HorizontalPositionKeyword>>) [comma_separated: PositionComponent<'i, HorizontalPositionKeyword>],
    "mask-position-y": MaskPositionY(Vec<'a, PositionComponent<'a, VerticalPositionKeyword>>) [comma_separated: PositionComponent<'i, VerticalPositionKeyword>],
    "mask-position": MaskPosition(Vec<'a, Position<'a>>, VendorPrefix) [comma_separated: Position<'i>],
    "mask-clip": MaskClip(Vec<'a, MaskClip>, VendorPrefix) [comma_separated: MaskClip],
    "mask-origin": MaskOrigin(Vec<'a, GeometryBox>, VendorPrefix) [comma_separated: GeometryBox],
    "mask-size": MaskSize(Vec<'a, NodeId<'a, BackgroundSize<'a>>>, VendorPrefix) [comma_separated_node: BackgroundSize<'i>],
    "mask-composite": MaskComposite(Vec<'a, MaskComposite>) [comma_separated: MaskComposite],
    "mask-type": MaskType(MaskType) [parse: MaskType],
    "mask": Mask(Vec<'a, NodeId<'a, Mask<'a>>>, VendorPrefix) [comma_separated_node: Mask<'i>],
    "mask-border-source": MaskBorderSource(NodeId<'a, Image<'a>>) [boxed: Image<'i>],
    "mask-border-mode": MaskBorderMode(MaskBorderMode) [unsupported],
    "mask-border-slice": MaskBorderSlice(NodeId<'a, BorderImageSlice<'a>>) [unsupported],
    "mask-border-width": MaskBorderWidth(NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>) [unsupported],
    "mask-border-outset": MaskBorderOutset(NodeId<'a, Rect<'a, LengthOrNumber<'a>>>) [unsupported],
    "mask-border-repeat": MaskBorderRepeat(BorderImageRepeat) [unsupported],
    "mask-border": MaskBorder(NodeId<'a, MaskBorder<'a>>) [unsupported],
    "-webkit-mask-composite": WebKitMaskComposite(Vec<'a, WebKitMaskComposite>) [comma_separated: WebKitMaskComposite],
    "mask-source-type": WebKitMaskSourceType(Vec<'a, WebKitMaskSourceType>, VendorPrefix) [comma_separated: WebKitMaskSourceType],
    "mask-box-image": WebKitMaskBoxImage(NodeId<'a, BorderImage<'a>>, VendorPrefix) [unsupported],
    "mask-box-image-source": WebKitMaskBoxImageSource(NodeId<'a, Image<'a>>, VendorPrefix) [boxed: Image<'i>],
    "mask-box-image-slice": WebKitMaskBoxImageSlice(NodeId<'a, BorderImageSlice<'a>>, VendorPrefix) [unsupported],
    "mask-box-image-width": WebKitMaskBoxImageWidth(NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>, VendorPrefix) [unsupported],
    "mask-box-image-outset": WebKitMaskBoxImageOutset(NodeId<'a, Rect<'a, LengthOrNumber<'a>>>, VendorPrefix) [unsupported],
    "mask-box-image-repeat": WebKitMaskBoxImageRepeat(BorderImageRepeat, VendorPrefix) [unsupported],
    "filter": Filter(NodeId<'a, FilterList<'a>>, VendorPrefix) [unsupported],
    "backdrop-filter": BackdropFilter(NodeId<'a, FilterList<'a>>, VendorPrefix) [unsupported],
    "mix-blend-mode": MixBlendMode(BlendMode) [unsupported],
    "z-index": ZIndex(ZIndex) [parse: ZIndex],
    "container-type": ContainerType(ContainerType) [unsupported],
    "container-name": ContainerName(NodeId<'a, ContainerNameList<'a>>) [unsupported],
    "container": Container(NodeId<'a, Container<'a>>) [unsupported],
    "view-transition-name": ViewTransitionName(NodeId<'a, ViewTransitionName<'a>>) [unsupported],
    "view-transition-class": ViewTransitionClass(NodeId<'a, NoneOrCustomIdentList<'a>>) [unsupported],
    "view-transition-group": ViewTransitionGroup(NodeId<'a, ViewTransitionGroup<'a>>) [unsupported],
    "color-scheme": ColorScheme(ColorScheme) [unsupported],
    "print-color-adjust": PrintColorAdjust(PrintColorAdjust, VendorPrefix) [unsupported],
        }
    };
}

for_each_property!(define_properties);

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_common::Allocator;

    #[test]
    fn property_id_storage_reuses_generated_identity_metadata() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let known = PropertyId::BackgroundClip(VendorPrefix::WEBKIT | VendorPrefix::MOZ);
        let known_id = context.alloc_encoded_node(known, DUMMY_SP);
        assert_eq!(
            context.encoded_node(known_id),
            PropertyId::BackgroundClip(VendorPrefix::WEBKIT | VendorPrefix::MOZ)
        );

        let custom = context.add_str("--theme");
        let custom_id = context.alloc_encoded_node(PropertyId::Custom(custom), DUMMY_SP);
        let cloned = context.clone_encoded_node(custom_id);
        assert_ne!(custom_id, cloned);
        assert_eq!(context.encoded_node(cloned), PropertyId::Custom(custom));

        let custom = context.add_str("x-widget");
        let expected = [
            PropertyId::Width,
            PropertyId::Unparsed,
            PropertyId::Custom(custom),
        ];
        let nodes = expected.map(|value| context.alloc_encoded_node(value, DUMMY_SP));
        let values = context.alloc_encoded_vec(nodes.into_iter());
        assert_eq!(
            context
                .encoded_vec_iter(values)
                .map(|id| context.encoded_node(id))
                .collect::<std::vec::Vec<_>>(),
            expected,
        );
    }

    #[test]
    fn ordinary_property_names_compare_by_content_without_storage_growth() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let left = PropertyId::from_name("FuTuRe-Property", &mut context);
        let right = PropertyId::from_name("FuTuRe-Property", &mut context);
        assert_ne!(left, right);
        assert_eq!(context.string_pool().len(), 0);
        let left = context.alloc_encoded_node(left, DUMMY_SP);
        let right = context.alloc_encoded_node(right, DUMMY_SP);
        assert!(context.nodes_eq(left, right));
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for _ in 0..100 {
            assert_eq!(context.encoded_node(left).name(&context), "FuTuRe-Property");
            context.mutate_encoded_node(left, |_, _| {});
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }

    macro_rules! assert_webkit_round_trip {
        ($name:literal, $property:ident, $vendor_prefix:ty) => {{
            let prefixed_name = concat!("-webkit-", $name);
            assert!(matches!(
                PropertyId::from_known_name(prefixed_name).unwrap(),
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
                let allocator = Allocator::new();
                let mut context = AstContext::new_in(&allocator);
                $(
                    let property_id = PropertyId::from_known_name($name).unwrap();
                    assert!(matches!(
                        property_id,
                        property_id_pattern!(PropertyId::$property $(, $vp)?)
                    ));
                    let node = context.alloc_encoded_node(property_id, DUMMY_SP);
                    assert_eq!(context.encoded_node(node), property_id);
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
                    let uppercase_id = PropertyId::from_known_name(&uppercase_name).unwrap();
                    assert_eq!(property_id.known_id(), uppercase_id.known_id(), "{name}", name = $name);
                )+
            }
        };
    }

    for_each_property!(metadata_tests);
}
