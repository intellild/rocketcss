use super::*;

use rs_css_allocator::{boxed::Box, vec::Vec};

#[derive(Debug, PartialEq)]
pub enum CssColor<'a> {
    CurrentColor(Box<'a, CurrentColor>),
    RGBColor(Box<'a, RGBColor>),
    LABColor(Box<'a, LABColor>),
    PredefinedColor(Box<'a, PredefinedColor>),
    FloatColor(Box<'a, FloatColor>),
    LightDark(Box<'a, LightDark<'a>>),
    SystemColor(Box<'a, SystemColor>),
}

#[derive(Debug, PartialEq)]
pub struct CurrentColor;

#[derive(Debug, PartialEq)]
pub struct RGBColor {
    pub alpha: f64,
    pub b: f64,
    pub g: f64,
    pub r: f64,
}

#[derive(Debug, PartialEq)]
pub enum LABColor {
    Lab { a: f64, alpha: f64, b: f64, l: f64 },
    Lch { alpha: f64, c: f64, h: f64, l: f64 },
    Oklab { a: f64, alpha: f64, b: f64, l: f64 },
    Oklch { alpha: f64, c: f64, h: f64, l: f64 },
}

#[derive(Debug, PartialEq)]
pub enum PredefinedColor {
    Srgb { alpha: f64, b: f64, g: f64, r: f64 },
    SrgbLinear { alpha: f64, b: f64, g: f64, r: f64 },
    DisplayP3 { alpha: f64, b: f64, g: f64, r: f64 },
    A98Rgb { alpha: f64, b: f64, g: f64, r: f64 },
    ProphotoRgb { alpha: f64, b: f64, g: f64, r: f64 },
    Rec2020 { alpha: f64, b: f64, g: f64, r: f64 },
    XyzD50 { alpha: f64, x: f64, y: f64, z: f64 },
    XyzD65 { alpha: f64, x: f64, y: f64, z: f64 },
}

#[derive(Debug, PartialEq)]
pub enum FloatColor {
    Rgb { alpha: f64, b: f64, g: f64, r: f64 },
    Hsl { alpha: f64, h: f64, l: f64, s: f64 },
    Hwb { alpha: f64, b: f64, h: f64, w: f64 },
}

#[derive(Debug, PartialEq)]
pub struct LightDark<'a> {
    pub dark: Box<'a, CssColor<'a>>,
    pub light: Box<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum UnresolvedColor<'a> {
    Rgb {
        alpha: Vec<'a, Box<'a, TokenOrValue<'a>>>,
        b: f64,
        g: f64,
        r: f64,
    },
    Hsl {
        alpha: Vec<'a, Box<'a, TokenOrValue<'a>>>,
        h: f64,
        l: f64,
        s: f64,
    },
    LightDark {
        dark: Vec<'a, Box<'a, TokenOrValue<'a>>>,
        light: Vec<'a, Box<'a, TokenOrValue<'a>>>,
    },
}
