use super::*;

use rocketcss_allocator::{boxed::Box, vec::Vec};

#[derive(Debug, PartialEq, Visit)]
pub enum CssColor<'a> {
    CurrentColor,
    Known(KnownColor),
    Rgba(RGBA),
    Lab(Box<'a, LABColor>),
    Predefined(Box<'a, PredefinedColor>),
    Float(Box<'a, FloatColor>),
    LightDark(Box<'a, LightDark<'a>>),
    System(SystemColor),
}

macro_rules! define_known_colors {
    ($($name:literal => $variant:ident($red:literal, $green:literal, $blue:literal, $alpha:literal),)+) => {
        impl KnownColor {
            $(
                #[allow(non_upper_case_globals)]
                pub const $variant: Self = Self(RGBA {
                    red: $red,
                    green: $green,
                    blue: $blue,
                    alpha: $alpha,
                });
            )+

            #[inline]
            pub fn from_name(name: &str) -> Option<Self> {
                match_ignore_ascii_case!(
                    name,
                    $($name => Some(Self::$variant),)+
                    _ => None,
                )
            }

            #[inline]
            pub const fn rgba(self) -> RGBA {
                self.0
            }
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
pub struct KnownColor(RGBA);

define_known_colors! {
    "transparent" => Transparent(0, 0, 0, 0),
    "black" => Black(0, 0, 0, 255),
    "silver" => Silver(192, 192, 192, 255),
    "gray" => Gray(128, 128, 128, 255),
    "white" => White(255, 255, 255, 255),
    "maroon" => Maroon(128, 0, 0, 255),
    "red" => Red(255, 0, 0, 255),
    "purple" => Purple(128, 0, 128, 255),
    "fuchsia" => Fuchsia(255, 0, 255, 255),
    "green" => Green(0, 128, 0, 255),
    "lime" => Lime(0, 255, 0, 255),
    "olive" => Olive(128, 128, 0, 255),
    "yellow" => Yellow(255, 255, 0, 255),
    "navy" => Navy(0, 0, 128, 255),
    "blue" => Blue(0, 0, 255, 255),
    "teal" => Teal(0, 128, 128, 255),
    "aqua" => Aqua(0, 255, 255, 255),
    "aliceblue" => Aliceblue(240, 248, 255, 255),
    "antiquewhite" => Antiquewhite(250, 235, 215, 255),
    "aquamarine" => Aquamarine(127, 255, 212, 255),
    "azure" => Azure(240, 255, 255, 255),
    "beige" => Beige(245, 245, 220, 255),
    "bisque" => Bisque(255, 228, 196, 255),
    "blanchedalmond" => Blanchedalmond(255, 235, 205, 255),
    "blueviolet" => Blueviolet(138, 43, 226, 255),
    "brown" => Brown(165, 42, 42, 255),
    "burlywood" => Burlywood(222, 184, 135, 255),
    "cadetblue" => Cadetblue(95, 158, 160, 255),
    "chartreuse" => Chartreuse(127, 255, 0, 255),
    "chocolate" => Chocolate(210, 105, 30, 255),
    "coral" => Coral(255, 127, 80, 255),
    "cornflowerblue" => Cornflowerblue(100, 149, 237, 255),
    "cornsilk" => Cornsilk(255, 248, 220, 255),
    "crimson" => Crimson(220, 20, 60, 255),
    "cyan" => Cyan(0, 255, 255, 255),
    "darkblue" => Darkblue(0, 0, 139, 255),
    "darkcyan" => Darkcyan(0, 139, 139, 255),
    "darkgoldenrod" => Darkgoldenrod(184, 134, 11, 255),
    "darkgray" => Darkgray(169, 169, 169, 255),
    "darkgreen" => Darkgreen(0, 100, 0, 255),
    "darkgrey" => Darkgrey(169, 169, 169, 255),
    "darkkhaki" => Darkkhaki(189, 183, 107, 255),
    "darkmagenta" => Darkmagenta(139, 0, 139, 255),
    "darkolivegreen" => Darkolivegreen(85, 107, 47, 255),
    "darkorange" => Darkorange(255, 140, 0, 255),
    "darkorchid" => Darkorchid(153, 50, 204, 255),
    "darkred" => Darkred(139, 0, 0, 255),
    "darksalmon" => Darksalmon(233, 150, 122, 255),
    "darkseagreen" => Darkseagreen(143, 188, 143, 255),
    "darkslateblue" => Darkslateblue(72, 61, 139, 255),
    "darkslategray" => Darkslategray(47, 79, 79, 255),
    "darkslategrey" => Darkslategrey(47, 79, 79, 255),
    "darkturquoise" => Darkturquoise(0, 206, 209, 255),
    "darkviolet" => Darkviolet(148, 0, 211, 255),
    "deeppink" => Deeppink(255, 20, 147, 255),
    "deepskyblue" => Deepskyblue(0, 191, 255, 255),
    "dimgray" => Dimgray(105, 105, 105, 255),
    "dimgrey" => Dimgrey(105, 105, 105, 255),
    "dodgerblue" => Dodgerblue(30, 144, 255, 255),
    "firebrick" => Firebrick(178, 34, 34, 255),
    "floralwhite" => Floralwhite(255, 250, 240, 255),
    "forestgreen" => Forestgreen(34, 139, 34, 255),
    "gainsboro" => Gainsboro(220, 220, 220, 255),
    "ghostwhite" => Ghostwhite(248, 248, 255, 255),
    "gold" => Gold(255, 215, 0, 255),
    "goldenrod" => Goldenrod(218, 165, 32, 255),
    "greenyellow" => Greenyellow(173, 255, 47, 255),
    "grey" => Grey(128, 128, 128, 255),
    "honeydew" => Honeydew(240, 255, 240, 255),
    "hotpink" => Hotpink(255, 105, 180, 255),
    "indianred" => Indianred(205, 92, 92, 255),
    "indigo" => Indigo(75, 0, 130, 255),
    "ivory" => Ivory(255, 255, 240, 255),
    "khaki" => Khaki(240, 230, 140, 255),
    "lavender" => Lavender(230, 230, 250, 255),
    "lavenderblush" => Lavenderblush(255, 240, 245, 255),
    "lawngreen" => Lawngreen(124, 252, 0, 255),
    "lemonchiffon" => Lemonchiffon(255, 250, 205, 255),
    "lightblue" => Lightblue(173, 216, 230, 255),
    "lightcoral" => Lightcoral(240, 128, 128, 255),
    "lightcyan" => Lightcyan(224, 255, 255, 255),
    "lightgoldenrodyellow" => Lightgoldenrodyellow(250, 250, 210, 255),
    "lightgray" => Lightgray(211, 211, 211, 255),
    "lightgreen" => Lightgreen(144, 238, 144, 255),
    "lightgrey" => Lightgrey(211, 211, 211, 255),
    "lightpink" => Lightpink(255, 182, 193, 255),
    "lightsalmon" => Lightsalmon(255, 160, 122, 255),
    "lightseagreen" => Lightseagreen(32, 178, 170, 255),
    "lightskyblue" => Lightskyblue(135, 206, 250, 255),
    "lightslategray" => Lightslategray(119, 136, 153, 255),
    "lightslategrey" => Lightslategrey(119, 136, 153, 255),
    "lightsteelblue" => Lightsteelblue(176, 196, 222, 255),
    "lightyellow" => Lightyellow(255, 255, 224, 255),
    "limegreen" => Limegreen(50, 205, 50, 255),
    "linen" => Linen(250, 240, 230, 255),
    "magenta" => Magenta(255, 0, 255, 255),
    "mediumaquamarine" => Mediumaquamarine(102, 205, 170, 255),
    "mediumblue" => Mediumblue(0, 0, 205, 255),
    "mediumorchid" => Mediumorchid(186, 85, 211, 255),
    "mediumpurple" => Mediumpurple(147, 112, 219, 255),
    "mediumseagreen" => Mediumseagreen(60, 179, 113, 255),
    "mediumslateblue" => Mediumslateblue(123, 104, 238, 255),
    "mediumspringgreen" => Mediumspringgreen(0, 250, 154, 255),
    "mediumturquoise" => Mediumturquoise(72, 209, 204, 255),
    "mediumvioletred" => Mediumvioletred(199, 21, 133, 255),
    "midnightblue" => Midnightblue(25, 25, 112, 255),
    "mintcream" => Mintcream(245, 255, 250, 255),
    "mistyrose" => Mistyrose(255, 228, 225, 255),
    "moccasin" => Moccasin(255, 228, 181, 255),
    "navajowhite" => Navajowhite(255, 222, 173, 255),
    "oldlace" => Oldlace(253, 245, 230, 255),
    "olivedrab" => Olivedrab(107, 142, 35, 255),
    "orange" => Orange(255, 165, 0, 255),
    "orangered" => Orangered(255, 69, 0, 255),
    "orchid" => Orchid(218, 112, 214, 255),
    "palegoldenrod" => Palegoldenrod(238, 232, 170, 255),
    "palegreen" => Palegreen(152, 251, 152, 255),
    "paleturquoise" => Paleturquoise(175, 238, 238, 255),
    "palevioletred" => Palevioletred(219, 112, 147, 255),
    "papayawhip" => Papayawhip(255, 239, 213, 255),
    "peachpuff" => Peachpuff(255, 218, 185, 255),
    "peru" => Peru(205, 133, 63, 255),
    "pink" => Pink(255, 192, 203, 255),
    "plum" => Plum(221, 160, 221, 255),
    "powderblue" => Powderblue(176, 224, 230, 255),
    "rebeccapurple" => Rebeccapurple(102, 51, 153, 255),
    "rosybrown" => Rosybrown(188, 143, 143, 255),
    "royalblue" => Royalblue(65, 105, 225, 255),
    "saddlebrown" => Saddlebrown(139, 69, 19, 255),
    "salmon" => Salmon(250, 128, 114, 255),
    "sandybrown" => Sandybrown(244, 164, 96, 255),
    "seagreen" => Seagreen(46, 139, 87, 255),
    "seashell" => Seashell(255, 245, 238, 255),
    "sienna" => Sienna(160, 82, 45, 255),
    "skyblue" => Skyblue(135, 206, 235, 255),
    "slateblue" => Slateblue(106, 90, 205, 255),
    "slategray" => Slategray(112, 128, 144, 255),
    "slategrey" => Slategrey(112, 128, 144, 255),
    "snow" => Snow(255, 250, 250, 255),
    "springgreen" => Springgreen(0, 255, 127, 255),
    "steelblue" => Steelblue(70, 130, 180, 255),
    "tan" => Tan(210, 180, 140, 255),
    "thistle" => Thistle(216, 191, 216, 255),
    "tomato" => Tomato(255, 99, 71, 255),
    "turquoise" => Turquoise(64, 224, 208, 255),
    "violet" => Violet(238, 130, 238, 255),
    "wheat" => Wheat(245, 222, 179, 255),
    "whitesmoke" => Whitesmoke(245, 245, 245, 255),
    "yellowgreen" => Yellowgreen(154, 205, 50, 255),
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
