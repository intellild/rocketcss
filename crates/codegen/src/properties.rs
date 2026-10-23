use crate::prelude::*;

impl<'ghost> ToCss<'ghost> for VendorPrefix {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.bits & Self::WEBKIT.bits != 0 {
            dest.write_str("-webkit-")
        } else if self.bits & Self::MOZ.bits != 0 {
            dest.write_str("-moz-")
        } else if self.bits & Self::MS.bits != 0 {
            dest.write_str("-ms-")
        } else if self.bits & Self::O.bits != 0 {
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
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("blend modes are static keywords"))
    }
}

impl<'ghost> ToCss<'ghost> for f32 {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        serialize_number(*self, dest)
    }
}

impl<'ghost> ToCss<'ghost> for i32 {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        serialize_int(*self, dest)
    }
}

impl<'ghost> ToCss<'ghost> for u16 {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        serialize_int(*self, dest)
    }
}

macro_rules! comma_slice {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'ast, 'ghost> ToCss<'ghost> for [$ty] {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, '_, 'ghost>) -> fmt::Result {
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

comma_slice! {
    Image<'ast>,
    PositionComponent<'ast, HorizontalPositionKeyword>,
    PositionComponent<'ast, VerticalPositionKeyword>,
    BackgroundPosition<'ast>,
    BackgroundSize<'ast>,
    BackgroundRepeat<'ast>,
    BackgroundAttachment,
    BackgroundClip,
    BackgroundOrigin,
    Background<'ast>,
    BoxShadow<'ast>,
    PropertyId<'ast>,
    Time,
    EasingFunction,
    Transition<'ast>,
    AnimationName<'ast>,
    AnimationIterationCount,
    AnimationDirection,
    AnimationPlayState,
    AnimationFillMode,
    AnimationComposition,
    AnimationTimeline<'ast>,
    AnimationAttachmentRange<'ast>,
    AnimationRange<'ast>,
    Animation<'ast>,
    TextShadow<'ast>,
    MaskMode,
    Position<'ast>,
    MaskClip,
    GeometryBox,
    MaskComposite,
    Mask<'ast>,
    WebKitMaskComposite,
    WebKitMaskSourceType,
}

impl<'ast, 'ghost> ToCss<'ghost> for [FontFamily<'ast>] {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let mut first = true;
        for family in self
            .iter()
            .filter(|family| !matches!(family.data(_cx.ast()), FontFamilyData::Tombstone))
        {
            if !first {
                dest.delim(Delimiter::Comma)?;
            }
            family.to_css(dest, _cx)?;
            first = false;
        }
        Ok(())
    }
}

macro_rules! space_slice {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'ast, 'ghost> ToCss<'ghost> for [$ty] {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, '_, 'ghost>) -> fmt::Result {
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

space_slice! { TrackSize<'ast>, Transform<'ast> }

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
            $name:literal: $property:ident($value:ty $(, $vp:ty)?)
                $([$strategy:ident $( : $($strategy_args:tt)+)?])?,
        )+
    ) => {
        impl<'ast, 'ghost> ToCss<'ghost> for Declaration<'ast> {
            fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, '_, 'ghost>) -> fmt::Result {
                let ast = _cx.ast();
                if self.is_tombstone(ast) {
                    return Ok(());
                }
                self.vendor_prefix(ast).to_css(dest, _cx)?;
                match self.data(ast) {
                    DeclarationData::Custom(_) => serialize_name(self.name(ast), dest)?,
                    DeclarationData::Unparsed(value)
                        if matches!(value.property_id(ast), PropertyId::Custom(_)) =>
                    {
                        serialize_name(self.name(ast), dest)?;
                    }
                    _ => dest.write_str(self.name(ast))?,
                }
                if matches!(self.data(ast), DeclarationData::Custom(_)) {
                    dest.write_char(':')?;
                } else {
                    dest.delim(Delimiter::Colon)?;
                }
                match self.data(ast) {
                    $(
                        $(#[$meta])*
                        declaration_value_pattern!(DeclarationData::$property, value$(, _prefix: $vp)?) => value.to_css(dest, _cx),
                    )+
                    DeclarationData::CSSWide(_, value) => value.to_css(dest, _cx),
                    DeclarationData::Unparsed(value) => value.to_css(dest, _cx),
                    DeclarationData::Custom(value) => value.to_css(dest, _cx),
                    DeclarationData::Tombstone => Ok(()),
                }
            }
        }
    };
}

rocketcss_ast_next::for_each_property!(impl_declaration_to_css);
