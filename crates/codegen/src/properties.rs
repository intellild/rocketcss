use crate::prelude::*;

impl<'ghost> ToCss<'ghost> for VendorPrefix {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
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

impl<'ghost, T: ToCss<'ghost>> ToCss<'ghost> for CSSWideOr<T> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Value(value) => value.to_css(dest, _cx),
            Self::CSSWide(keyword) => keyword.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for PropertyId<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        self.vendor_prefix().to_css(dest, _cx)?;
        match self {
            Self::Custom(value) => serialize_name(value, dest),
            _ => dest.write_str(self.name()),
        }
    }
}

impl<'ghost> ToCss<'ghost> for BlendMode {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("blend modes are static keywords"))
    }
}

impl<'ghost> ToCss<'ghost> for f32 {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        serialize_number(*self, dest)
    }
}

impl<'ghost> ToCss<'ghost> for i32 {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        serialize_int(*self, dest)
    }
}

impl<'ghost> ToCss<'ghost> for u16 {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        serialize_int(*self, dest)
    }
}

macro_rules! comma_vec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'a, 'ghost> ToCss<'ghost> for rocketcss_allocator::vec::Vec<'a, $ty> {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, 'ghost>) -> fmt::Result {
                    for (index, value) in self.iter().enumerate() {
                        if index > 0 {
                            dest.delim(Delimiter::Comma)?;
                        }
                        value.to_css(dest, _cx)?;
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

impl<'a, 'ghost> ToCss<'ghost> for rocketcss_allocator::vec::Vec<'a, FontFamily<'a>> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        let mut first = true;
        for family in self.iter().filter(|family| !family.is_tombstone()) {
            if !first {
                dest.delim(Delimiter::Comma)?;
            }
            family.to_css(dest, _cx)?;
            first = false;
        }
        Ok(())
    }
}

macro_rules! space_vec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'a, 'ghost> ToCss<'ghost> for rocketcss_allocator::vec::Vec<'a, $ty> {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, 'ghost>) -> fmt::Result {
                    for (index, value) in self.iter().enumerate() {
                        if index > 0 {
                            dest.write_char(' ')?;
                        }
                        value.to_css(dest, _cx)?;
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
        impl<'ghost> ToCss<'ghost> for Declaration<'_> {
            fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, 'ghost>) -> fmt::Result {
                if self.is_tombstone() {
                    return Ok(());
                }
                self.vendor_prefix().to_css(dest, _cx)?;
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
                        declaration_value_pattern!(Self::$property, value$(, _prefix: $vp)?) => value.to_css(dest, _cx),
                    )+
                    Self::Unparsed(value) => value.to_css(dest, _cx),
                    Self::Custom(value) => value.to_css(dest, _cx),
                    Self::Tombstone => Ok(()),
                }
            }
        }
    };
}

rocketcss_ast::for_each_property!(impl_declaration_to_css);
