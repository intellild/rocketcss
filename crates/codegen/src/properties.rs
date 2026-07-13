use crate::prelude::*;

impl ToCss for VendorPrefix {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if self.contains(Self::WEBKIT) {
            dest.write_str("-webkit-")
        } else if self.contains(Self::MOZ) {
            dest.write_str("-moz-")
        } else if self.contains(Self::MS) {
            dest.write_str("-ms-")
        } else if self.contains(Self::O) {
            dest.write_str("-o-")
        } else {
            Ok(())
        }
    }
}

impl ToCss for PropertyId<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.vendor_prefix().to_css(dest)?;
        match self {
            Self::Custom(value) => serialize_name(value, dest),
            _ => dest.write_str(self.name()),
        }
    }
}

impl ToCss for BlendMode {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_debug_keyword(self, dest)
    }
}

impl ToCss for f32 {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_number(*self, dest)
    }
}

impl ToCss for i32 {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_int(*self, dest)
    }
}

impl ToCss for u16 {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_int(*self, dest)
    }
}

macro_rules! comma_vec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'a> ToCss for rocketcss_allocator::vec::Vec<'a, $ty> {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
                    for (index, value) in self.iter().enumerate() {
                        if index > 0 {
                            dest.delim(Delimiter::Comma)?;
                        }
                        value.to_css(dest)?;
                    }
                    Ok(())
                }
            }
        )+
    };
}

comma_vec! {
    Image<'a>,
    PositionComponent<'a, HorizontalPositionKeyword>,
    PositionComponent<'a, VerticalPositionKeyword>,
    BackgroundPosition<'a>,
    BackgroundSize<'a>,
    BackgroundRepeat,
    BackgroundAttachment,
    BackgroundClip,
    BackgroundOrigin,
    Background<'a>,
    BoxShadow<'a>,
    FontFamily<'a>,
    PropertyId<'a>,
    Time,
    EasingFunction,
    Transition<'a>,
    AnimationName<'a>,
    AnimationIterationCount,
    AnimationDirection,
    AnimationPlayState,
    AnimationFillMode,
    AnimationComposition,
    AnimationTimeline<'a>,
    AnimationAttachmentRange<'a>,
    AnimationRange<'a>,
    Animation<'a>,
    TextShadow<'a>,
    MaskMode,
    Position<'a>,
    MaskClip,
    GeometryBox,
    MaskComposite,
    Mask<'a>,
    WebKitMaskComposite,
    WebKitMaskSourceType,
}

macro_rules! space_vec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'a> ToCss for rocketcss_allocator::vec::Vec<'a, $ty> {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
                    for (index, value) in self.iter().enumerate() {
                        if index > 0 {
                            dest.write_char(' ')?;
                        }
                        value.to_css(dest)?;
                    }
                    Ok(())
                }
            }
        )+
    };
}

space_vec! { TrackSize<'a>, Transform<'a> }

macro_rules! declaration_value_pattern {
    ($name:path, $value:ident) => {
        $name($value)
    };
    ($name:path, $value:ident, $prefix:ident: $vendor_prefix:ty) => {
        $name($value, $prefix)
    };
}

macro_rules! impl_declaration_to_css {
    (
        $(
            $(#[$meta:meta])*
            $name:literal: $property:ident($value:ty $(, $vp:ty)?),
        )+
    ) => {
        impl ToCss for Declaration<'_> {
            fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
                self.vendor_prefix().to_css(dest)?;
                match self {
                    Self::Custom(_) => serialize_name(self.name(), dest)?,
                    Self::Unparsed(value)
                        if matches!(&*value.property_id, PropertyId::Custom(_)) =>
                    {
                        serialize_name(self.name(), dest)?;
                    }
                    _ => dest.write_str(self.name())?,
                }
                if matches!(self, Self::Custom(_)) {
                    dest.write_char(':')?;
                } else {
                    dest.delim(Delimiter::Colon)?;
                }
                match self {
                    $(
                        $(#[$meta])*
                        declaration_value_pattern!(Self::$property, value$(, _prefix: $vp)?) => value.to_css(dest),
                    )+
                    Self::All(value) => value.to_css(dest),
                    Self::Unparsed(value) => value.to_css(dest),
                    Self::Custom(value) => value.to_css(dest),
                }
            }
        }
    };
}

rocketcss_ast::for_each_property!(impl_declaration_to_css);
