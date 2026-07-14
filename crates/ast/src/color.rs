use super::*;

use rocketcss_allocator::{boxed::Box, vec::Vec};

#[derive(Debug, PartialEq, Visit)]
pub enum CssColor<'a> {
    CurrentColor,
    Rgba(RGBA),
    Lab(Box<'a, LABColor>),
    Predefined(Box<'a, PredefinedColor>),
    Float(Box<'a, FloatColor>),
    LightDark(Box<'a, LightDark<'a>>),
    System(SystemColor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, PartialEq, Visit)]
pub enum LABColor {
    Lab { a: f32, alpha: f32, b: f32, l: f32 },
    Lch { alpha: f32, c: f32, h: f32, l: f32 },
    Oklab { a: f32, alpha: f32, b: f32, l: f32 },
    Oklch { alpha: f32, c: f32, h: f32, l: f32 },
}

#[derive(Debug, PartialEq, Visit)]
pub enum PredefinedColor {
    Srgb { alpha: f32, b: f32, g: f32, r: f32 },
    SrgbLinear { alpha: f32, b: f32, g: f32, r: f32 },
    DisplayP3 { alpha: f32, b: f32, g: f32, r: f32 },
    A98Rgb { alpha: f32, b: f32, g: f32, r: f32 },
    ProphotoRgb { alpha: f32, b: f32, g: f32, r: f32 },
    Rec2020 { alpha: f32, b: f32, g: f32, r: f32 },
    XyzD50 { alpha: f32, x: f32, y: f32, z: f32 },
    XyzD65 { alpha: f32, x: f32, y: f32, z: f32 },
}

#[derive(Debug, PartialEq, Visit)]
pub enum FloatColor {
    Rgb { alpha: f32, b: f32, g: f32, r: f32 },
    Hsl { alpha: f32, h: f32, l: f32, s: f32 },
    Hwb { alpha: f32, b: f32, h: f32, w: f32 },
}

#[derive(Debug, PartialEq, Visit)]
pub struct LightDark<'a> {
    pub dark: Box<'a, CssColor<'a>>,
    pub light: Box<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub enum SystemColor {
    Accentcolor,
    Accentcolortext,
    Activetext,
    Buttonborder,
    Buttonface,
    Buttontext,
    Canvas,
    Canvastext,
    Field,
    Fieldtext,
    Graytext,
    Highlight,
    Highlighttext,
    Linktext,
    Mark,
    Marktext,
    Selecteditem,
    Selecteditemtext,
    Visitedtext,
    Activeborder,
    Activecaption,
    Appworkspace,
    Background,
    Buttonhighlight,
    Buttonshadow,
    Captiontext,
    Inactiveborder,
    Inactivecaption,
    Inactivecaptiontext,
    Infobackground,
    Infotext,
    Menu,
    Menutext,
    Scrollbar,
    Threeddarkshadow,
    Threedface,
    Threedhighlight,
    Threedlightshadow,
    Threedshadow,
    Window,
    Windowframe,
    Windowtext,
}

#[derive(Debug, PartialEq, Visit)]
pub enum UnresolvedColor<'a> {
    Rgb {
        alpha: Vec<'a, TokenOrValue<'a>>,
        b: f32,
        g: f32,
        r: f32,
    },
    Hsl {
        alpha: Vec<'a, TokenOrValue<'a>>,
        h: f32,
        l: f32,
        s: f32,
    },
    LightDark {
        dark: Vec<'a, TokenOrValue<'a>>,
        light: Vec<'a, TokenOrValue<'a>>,
    },
}
