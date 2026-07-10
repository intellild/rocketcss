use crate::prelude::*;

impl ToCss for CssColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::CurrentColor => dest.write_str("currentColor"),
            Self::Rgba(value) => value.to_css(dest),
            Self::Lab(value) => value.to_css(dest),
            Self::Predefined(value) => value.to_css(dest),
            Self::Float(value) => value.to_css(dest),
            Self::LightDark(value) => value.to_css(dest),
            Self::System(value) => value.to_css(dest),
        }
    }
}

impl ToCss for RGBA {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if self.alpha == u8::MAX {
            let value =
                (u32::from(self.red) << 16) | (u32::from(self.green) << 8) | u32::from(self.blue);
            if let Some(name) = short_color_name(value) {
                return dest.write_str(name);
            }
            if [self.red, self.green, self.blue]
                .iter()
                .all(|value| value >> 4 == value & 0x0f)
            {
                write!(
                    dest,
                    "#{:x}{:x}{:x}",
                    self.red & 0x0f,
                    self.green & 0x0f,
                    self.blue & 0x0f
                )
            } else {
                write!(dest, "#{value:06x}")
            }
        } else if [self.red, self.green, self.blue, self.alpha]
            .iter()
            .all(|value| value >> 4 == value & 0x0f)
        {
            write!(
                dest,
                "#{:x}{:x}{:x}{:x}",
                self.red & 0x0f,
                self.green & 0x0f,
                self.blue & 0x0f,
                self.alpha & 0x0f
            )
        } else {
            let value = (u32::from(self.red) << 24)
                | (u32::from(self.green) << 16)
                | (u32::from(self.blue) << 8)
                | u32::from(self.alpha);
            write!(dest, "#{value:08x}")
        }
    }
}

fn short_color_name(value: u32) -> Option<&'static str> {
    Some(match value {
        0x000080 => "navy",
        0x008000 => "green",
        0x008080 => "teal",
        0x4b0082 => "indigo",
        0x800000 => "maroon",
        0x800080 => "purple",
        0x808000 => "olive",
        0x808080 => "gray",
        0xa0522d => "sienna",
        0xa52a2a => "brown",
        0xc0c0c0 => "silver",
        0xcd853f => "peru",
        0xd2b48c => "tan",
        0xda70d6 => "orchid",
        0xdda0dd => "plum",
        0xee82ee => "violet",
        0xf0e68c => "khaki",
        0xf0ffff => "azure",
        0xf5deb3 => "wheat",
        0xf5f5dc => "beige",
        0xfa8072 => "salmon",
        0xfaf0e6 => "linen",
        0xff0000 => "red",
        0xff6347 => "tomato",
        0xff7f50 => "coral",
        0xffa500 => "orange",
        0xffc0cb => "pink",
        0xffd700 => "gold",
        0xffe4c4 => "bisque",
        0xfffafa => "snow",
        0xfffff0 => "ivory",
        _ => return None,
    })
}

fn write_components<PrinterT: PrinterTrait>(
    name: &str,
    first: f32,
    second: f32,
    third: f32,
    alpha: f32,
    first_is_percentage: bool,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_str(name)?;
    dest.write_char('(')?;
    serialize_number(first * if first_is_percentage { 100.0 } else { 1.0 }, dest)?;
    if first_is_percentage {
        dest.write_char('%')?;
    }
    dest.write_char(' ')?;
    serialize_number(second, dest)?;
    dest.write_char(' ')?;
    serialize_number(third, dest)?;
    if alpha != 1.0 {
        dest.write_str(" / ")?;
        serialize_number(alpha, dest)?;
    }
    dest.write_char(')')
}

impl ToCss for LABColor {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Lab { a, alpha, b, l } => {
                write_components("lab", *l / 100.0, *a, *b, *alpha, true, dest)
            }
            Self::Lch { alpha, c, h, l } => {
                write_components("lch", *l / 100.0, *c, *h, *alpha, true, dest)
            }
            Self::Oklab { a, alpha, b, l } => {
                write_components("oklab", *l, *a, *b, *alpha, true, dest)
            }
            Self::Oklch { alpha, c, h, l } => {
                write_components("oklch", *l, *c, *h, *alpha, true, dest)
            }
        }
    }
}

impl ToCss for PredefinedColor {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let (space, first, second, third, alpha) = match self {
            Self::Srgb { alpha, b, g, r } => ("srgb", *r, *g, *b, *alpha),
            Self::SrgbLinear { alpha, b, g, r } => ("srgb-linear", *r, *g, *b, *alpha),
            Self::DisplayP3 { alpha, b, g, r } => ("display-p3", *r, *g, *b, *alpha),
            Self::A98Rgb { alpha, b, g, r } => ("a98-rgb", *r, *g, *b, *alpha),
            Self::ProphotoRgb { alpha, b, g, r } => ("prophoto-rgb", *r, *g, *b, *alpha),
            Self::Rec2020 { alpha, b, g, r } => ("rec2020", *r, *g, *b, *alpha),
            Self::XyzD50 { alpha, x, y, z } => ("xyz-d50", *x, *y, *z, *alpha),
            Self::XyzD65 { alpha, x, y, z } => ("xyz-d65", *x, *y, *z, *alpha),
        };
        dest.write_str("color(")?;
        dest.write_str(space)?;
        for value in [first, second, third] {
            dest.write_char(' ')?;
            serialize_number(value, dest)?;
        }
        if alpha != 1.0 {
            dest.write_str(" / ")?;
            serialize_number(alpha, dest)?;
        }
        dest.write_char(')')
    }
}

impl ToCss for FloatColor {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Rgb { alpha, b, g, r } => write_components("rgb", *r, *g, *b, *alpha, true, dest),
            Self::Hsl { alpha, h, l, s } => {
                dest.write_str("hsl(")?;
                serialize_number(*h, dest)?;
                dest.write_char(' ')?;
                serialize_number(*s * 100.0, dest)?;
                dest.write_str("% ")?;
                serialize_number(*l * 100.0, dest)?;
                dest.write_char('%')?;
                if *alpha != 1.0 {
                    dest.write_str(" / ")?;
                    serialize_number(*alpha, dest)?;
                }
                dest.write_char(')')
            }
            Self::Hwb { alpha, b, h, w } => {
                dest.write_str("hwb(")?;
                serialize_number(*h, dest)?;
                dest.write_char(' ')?;
                serialize_number(*w * 100.0, dest)?;
                dest.write_str("% ")?;
                serialize_number(*b * 100.0, dest)?;
                dest.write_char('%')?;
                if *alpha != 1.0 {
                    dest.write_str(" / ")?;
                    serialize_number(*alpha, dest)?;
                }
                dest.write_char(')')
            }
        }
    }
}

impl ToCss for LightDark<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("light-dark(")?;
        self.light.to_css(dest)?;
        dest.delim(',', false)?;
        self.dark.to_css(dest)?;
        dest.write_char(')')
    }
}

impl ToCss for SystemColor {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(match self {
            Self::Accentcolor => "AccentColor",
            Self::Accentcolortext => "AccentColorText",
            Self::Activetext => "ActiveText",
            Self::Buttonborder => "ButtonBorder",
            Self::Buttonface => "ButtonFace",
            Self::Buttontext => "ButtonText",
            Self::Canvas => "Canvas",
            Self::Canvastext => "CanvasText",
            Self::Field => "Field",
            Self::Fieldtext => "FieldText",
            Self::Graytext => "GrayText",
            Self::Highlight => "Highlight",
            Self::Highlighttext => "HighlightText",
            Self::Linktext => "LinkText",
            Self::Mark => "Mark",
            Self::Marktext => "MarkText",
            Self::Selecteditem => "SelectedItem",
            Self::Selecteditemtext => "SelectedItemText",
            Self::Visitedtext => "VisitedText",
            Self::Activeborder => "ActiveBorder",
            Self::Activecaption => "ActiveCaption",
            Self::Appworkspace => "AppWorkspace",
            Self::Background => "Background",
            Self::Buttonhighlight => "ButtonHighlight",
            Self::Buttonshadow => "ButtonShadow",
            Self::Captiontext => "CaptionText",
            Self::Inactiveborder => "InactiveBorder",
            Self::Inactivecaption => "InactiveCaption",
            Self::Inactivecaptiontext => "InactiveCaptionText",
            Self::Infobackground => "InfoBackground",
            Self::Infotext => "InfoText",
            Self::Menu => "Menu",
            Self::Menutext => "MenuText",
            Self::Scrollbar => "Scrollbar",
            Self::Threeddarkshadow => "ThreeDDarkShadow",
            Self::Threedface => "ThreeDFace",
            Self::Threedhighlight => "ThreeDHighlight",
            Self::Threedlightshadow => "ThreeDLightShadow",
            Self::Threedshadow => "ThreeDShadow",
            Self::Window => "Window",
            Self::Windowframe => "WindowFrame",
            Self::Windowtext => "WindowText",
        })
    }
}

impl ToCss for UnresolvedColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Rgb { alpha, b, g, r } => {
                dest.write_str("rgb(")?;
                serialize_number(*r, dest)?;
                dest.write_char(' ')?;
                serialize_number(*g, dest)?;
                dest.write_char(' ')?;
                serialize_number(*b, dest)?;
                dest.write_str(" / ")?;
                crate::token::write_token_list(alpha, dest)?;
                dest.write_char(')')
            }
            Self::Hsl { alpha, h, l, s } => {
                dest.write_str("hsl(")?;
                serialize_number(*h, dest)?;
                dest.write_char(' ')?;
                serialize_number(*s * 100.0, dest)?;
                dest.write_str("% ")?;
                serialize_number(*l * 100.0, dest)?;
                dest.write_str("% / ")?;
                crate::token::write_token_list(alpha, dest)?;
                dest.write_char(')')
            }
            Self::LightDark { dark, light } => {
                dest.write_str("light-dark(")?;
                crate::token::write_token_list(light, dest)?;
                dest.delim(',', false)?;
                crate::token::write_token_list(dark, dest)?;
                dest.write_char(')')
            }
        }
    }
}
