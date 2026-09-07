use super::*;

use crate::{AstNodeClone, AstNodeStorage, ExtraData, NodeKind, NodePayload};

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum CssColor<'a> {
    CurrentColor,
    #[visit(skip)]
    Known(KnownColor),
    Rgba(RGBA),
    Function(NodeId<'a, Function<'a>>),
    Lab(NodeId<'a, LABColor>),
    Predefined(NodeId<'a, PredefinedColor>),
    Float(NodeId<'a, FloatColor>),
    LightDark(NodeId<'a, LightDark<'a>>),
    System(SystemColor),
}

impl_inline_node!(CssColor<'ast>, 0x0002_0001);

impl<'ast> AstNodeClone<'ast> for CssColor<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::CurrentColor => Self::CurrentColor,
            Self::Known(value) => Self::Known(value),
            Self::Rgba(value) => Self::Rgba(value),
            Self::Function(value) => Self::Function(context.clone_encoded_node(value)),
            Self::Lab(value) => Self::Lab(context.clone_encoded_node(value)),
            Self::Predefined(value) => Self::Predefined(context.clone_encoded_node(value)),
            Self::Float(value) => Self::Float(context.clone_encoded_node(value)),
            Self::LightDark(value) => Self::LightDark(context.clone_encoded_node(value)),
            Self::System(value) => Self::System(value),
        }
    }
}

macro_rules! define_known_colors {
    ($($name:literal => $variant:ident($red:literal, $green:literal, $blue:literal, $alpha:literal),)+) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum KnownColor {
            $($variant,)+
        }

        impl KnownColor {
            #[inline]
            pub fn from_name(name: &str) -> Option<Self> {
                match_ignore_ascii_case!(
                    name,
                    $($name => Some(Self::$variant),)+
                    _ => None,
                )
            }

            #[inline]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)+
                }
            }

            #[inline]
            pub const fn rgba(self) -> RGBA {
                match self {
                    $(Self::$variant => RGBA {
                        red: $red,
                        green: $green,
                        blue: $blue,
                        alpha: $alpha,
                    },)+
                }
            }
        }
    };
}

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

// Four-component colors use a native header plus one packed f32-pair slot.
unsafe impl AstNodeStorage<'_> for LABColor {
    const KIND: NodeKind = NodeKind::new(0x0002_0002);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
        let (tag, values) = decode_float_color_payload(payload, context);
        match tag {
            0 => Self::Lab {
                a: values[0],
                alpha: values[1],
                b: values[2],
                l: values[3],
            },
            1 => Self::Lch {
                alpha: values[0],
                c: values[1],
                h: values[2],
                l: values[3],
            },
            2 => Self::Oklab {
                a: values[0],
                alpha: values[1],
                b: values[2],
                l: values[3],
            },
            3 => Self::Oklch {
                alpha: values[0],
                c: values[1],
                h: values[2],
                l: values[3],
            },
            _ => panic!("invalid encoded LABColor variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'_>) -> NodePayload {
        let (tag, values) = match self {
            Self::Lab { a, alpha, b, l } => (0, [a, alpha, b, l]),
            Self::Lch { alpha, c, h, l } => (1, [alpha, c, h, l]),
            Self::Oklab { a, alpha, b, l } => (2, [a, alpha, b, l]),
            Self::Oklch { alpha, c, h, l } => (3, [alpha, c, h, l]),
        };
        encode_float_color_payload(tag, values, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'_>,
    ) -> NodePayload {
        let (tag, values) = match self {
            Self::Lab { a, alpha, b, l } => (0, [a, alpha, b, l]),
            Self::Lch { alpha, c, h, l } => (1, [alpha, c, h, l]),
            Self::Oklab { a, alpha, b, l } => (2, [a, alpha, b, l]),
            Self::Oklch { alpha, c, h, l } => (3, [alpha, c, h, l]),
        };
        encode_float_color_payload(
            tag,
            values,
            Some(unsafe { current.read_value::<FloatColorHeader>() }.extra as usize),
            context,
        )
    }
}

impl AstNodeClone<'_> for LABColor {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
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

pub use color_access::{FloatColorRead, LabColorRead, PredefinedColorRead};

// A serialization view, not a persistent AST node or visitor target.
mod color_access {
    use super::*;

    pub struct PredefinedColorRead<'context, 'storage> {
        context: &'context AstContext<'storage>,
        header: FloatColorHeader,
    }

    impl PredefinedColorRead<'_, '_> {
        pub fn space_name(&self) -> &'static str {
            match self.header.tag {
                0 => "srgb",
                1 => "srgb-linear",
                2 => "display-p3",
                3 => "a98-rgb",
                4 => "prophoto-rgb",
                5 => "rec2020",
                6 => "xyz-d50",
                7 => "xyz-d65",
                _ => unreachable!("invalid stored predefined color space"),
            }
        }

        /// Raw authored components in CSS order, followed by alpha.
        pub fn components(&self) -> ([f32; 3], f32) {
            // SAFETY: this color kind stores the final two floats in one slot.
            let tail: [f32; 2] = unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            };
            let [alpha, second] = self.header.values;
            let components = if self.header.tag < 6 {
                [tail[1], tail[0], second]
            } else {
                [second, tail[0], tail[1]]
            };
            (components, alpha)
        }
    }

    pub struct LabColorRead<'context, 'storage> {
        context: &'context AstContext<'storage>,
        header: FloatColorHeader,
    }

    impl LabColorRead<'_, '_> {
        pub fn space_name(&self) -> &'static str {
            match self.header.tag {
                0 => "lab",
                1 => "lch",
                2 => "oklab",
                3 => "oklch",
                _ => unreachable!("invalid stored LAB color space"),
            }
        }

        pub fn has_cie_lightness(&self) -> bool {
            self.header.tag < 2
        }

        pub fn components(&self) -> ([f32; 3], f32) {
            let [first, second, third, lightness] = self.header.components(self.context);
            if self.header.tag == 0 || self.header.tag == 2 {
                ([lightness, first, third], second)
            } else {
                ([lightness, second, third], first)
            }
        }
    }

    pub struct FloatColorRead<'context, 'storage> {
        context: &'context AstContext<'storage>,
        header: FloatColorHeader,
    }

    impl FloatColorRead<'_, '_> {
        pub fn space_name(&self) -> &'static str {
            match self.header.tag {
                0 => "rgb",
                1 => "hsl",
                2 => "hwb",
                _ => unreachable!("invalid stored float color space"),
            }
        }

        pub fn is_rgb(&self) -> bool {
            self.header.tag == 0
        }

        pub fn components(&self) -> ([f32; 3], f32) {
            let [alpha, first, second, third] = self.header.components(self.context);
            let components = match self.header.tag {
                0 => [third, second, first],
                1 => [first, third, second],
                2 => [second, third, first],
                _ => unreachable!("invalid stored float color space"),
            };
            (components, alpha)
        }
    }

    impl<'storage> AstContext<'storage> {
        pub fn lab_color(&self, id: NodeId<'_, LABColor>) -> LabColorRead<'_, 'storage> {
            // SAFETY: the typed node validates the native color header's kind.
            LabColorRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }

        pub fn float_color(&self, id: NodeId<'_, FloatColor>) -> FloatColorRead<'_, 'storage> {
            // SAFETY: the typed node validates the native color header's kind.
            FloatColorRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }

        pub fn predefined_color(
            &self,
            id: NodeId<'_, PredefinedColor>,
        ) -> PredefinedColorRead<'_, 'storage> {
            // SAFETY: node_payload checks the owning kind before this typed read.
            PredefinedColorRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

unsafe impl AstNodeStorage<'_> for PredefinedColor {
    const KIND: NodeKind = NodeKind::new(0x0002_0003);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
        let (tag, values) = decode_float_color_payload(payload, context);
        match tag {
            0 => Self::Srgb {
                alpha: values[0],
                b: values[1],
                g: values[2],
                r: values[3],
            },
            1 => Self::SrgbLinear {
                alpha: values[0],
                b: values[1],
                g: values[2],
                r: values[3],
            },
            2 => Self::DisplayP3 {
                alpha: values[0],
                b: values[1],
                g: values[2],
                r: values[3],
            },
            3 => Self::A98Rgb {
                alpha: values[0],
                b: values[1],
                g: values[2],
                r: values[3],
            },
            4 => Self::ProphotoRgb {
                alpha: values[0],
                b: values[1],
                g: values[2],
                r: values[3],
            },
            5 => Self::Rec2020 {
                alpha: values[0],
                b: values[1],
                g: values[2],
                r: values[3],
            },
            6 => Self::XyzD50 {
                alpha: values[0],
                x: values[1],
                y: values[2],
                z: values[3],
            },
            7 => Self::XyzD65 {
                alpha: values[0],
                x: values[1],
                y: values[2],
                z: values[3],
            },
            _ => panic!("invalid encoded PredefinedColor variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'_>) -> NodePayload {
        let (tag, values) = encode_predefined_color(self);
        encode_float_color_payload(tag, values, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'_>,
    ) -> NodePayload {
        let (tag, values) = encode_predefined_color(self);
        encode_float_color_payload(
            tag,
            values,
            Some(unsafe { current.read_value::<FloatColorHeader>() }.extra as usize),
            context,
        )
    }
}

impl AstNodeClone<'_> for PredefinedColor {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_predefined_color(value: PredefinedColor) -> (u8, [f32; 4]) {
    match value {
        PredefinedColor::Srgb { alpha, b, g, r } => (0, [alpha, b, g, r]),
        PredefinedColor::SrgbLinear { alpha, b, g, r } => (1, [alpha, b, g, r]),
        PredefinedColor::DisplayP3 { alpha, b, g, r } => (2, [alpha, b, g, r]),
        PredefinedColor::A98Rgb { alpha, b, g, r } => (3, [alpha, b, g, r]),
        PredefinedColor::ProphotoRgb { alpha, b, g, r } => (4, [alpha, b, g, r]),
        PredefinedColor::Rec2020 { alpha, b, g, r } => (5, [alpha, b, g, r]),
        PredefinedColor::XyzD50 { alpha, x, y, z } => (6, [alpha, x, y, z]),
        PredefinedColor::XyzD65 { alpha, x, y, z } => (7, [alpha, x, y, z]),
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum FloatColor {
    Rgb { alpha: f32, b: f32, g: f32, r: f32 },
    Hsl { alpha: f32, h: f32, l: f32, s: f32 },
    Hwb { alpha: f32, b: f32, h: f32, w: f32 },
}

unsafe impl AstNodeStorage<'_> for FloatColor {
    const KIND: NodeKind = NodeKind::new(0x0002_0004);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
        let (tag, values) = decode_float_color_payload(payload, context);
        match tag {
            0 => Self::Rgb {
                alpha: values[0],
                b: values[1],
                g: values[2],
                r: values[3],
            },
            1 => Self::Hsl {
                alpha: values[0],
                h: values[1],
                l: values[2],
                s: values[3],
            },
            2 => Self::Hwb {
                alpha: values[0],
                b: values[1],
                h: values[2],
                w: values[3],
            },
            _ => panic!("invalid encoded FloatColor variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'_>) -> NodePayload {
        let (tag, values) = match self {
            Self::Rgb { alpha, b, g, r } => (0, [alpha, b, g, r]),
            Self::Hsl { alpha, h, l, s } => (1, [alpha, h, l, s]),
            Self::Hwb { alpha, b, h, w } => (2, [alpha, b, h, w]),
        };
        encode_float_color_payload(tag, values, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'_>,
    ) -> NodePayload {
        let (tag, values) = match self {
            Self::Rgb { alpha, b, g, r } => (0, [alpha, b, g, r]),
            Self::Hsl { alpha, h, l, s } => (1, [alpha, h, l, s]),
            Self::Hwb { alpha, b, h, w } => (2, [alpha, b, h, w]),
        };
        encode_float_color_payload(
            tag,
            values,
            Some(unsafe { current.read_value::<FloatColorHeader>() }.extra as usize),
            context,
        )
    }
}

impl AstNodeClone<'_> for FloatColor {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Clone, Copy)]
struct FloatColorHeader {
    tag: u8,
    values: [f32; 2],
    extra: u32,
}

impl FloatColorHeader {
    fn components(self, context: &AstContext<'_>) -> [f32; 4] {
        // SAFETY: color headers refer to a native pair of trailing floats.
        let tail: [f32; 2] = unsafe { context.extra_slot(self.extra as usize).read_value() };
        [self.values[0], self.values[1], tail[0], tail[1]]
    }
}

fn encode_float_color_payload(
    tag: u8,
    values: [f32; 4],
    existing_extra: Option<usize>,
    context: &mut AstContext<'_>,
) -> NodePayload {
    let tail = ExtraData::from_value([values[2], values[3]]);
    let extra = match existing_extra {
        Some(index) => {
            context.set_extra_slot(index, tail);
            index
        }
        None => context.alloc_extra_slots([tail]),
    };
    NodePayload::from_value(FloatColorHeader {
        tag,
        values: [values[0], values[1]],
        extra: u32::try_from(extra).expect("extra index exceeds u32"),
    })
}

fn decode_float_color_payload(payload: NodePayload, context: &AstContext<'_>) -> (u8, [f32; 4]) {
    // SAFETY: the three four-component color kinds publish this header and an f32 pair.
    let header: FloatColorHeader = unsafe { payload.read_value() };
    (header.tag, header.components(context))
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub struct LightDark<'a> {
    pub dark: NodeId<'a, CssColor<'a>>,
    pub light: NodeId<'a, CssColor<'a>>,
}

impl_inline_node!(LightDark<'ast>, 0x0002_0005);

impl<'ast> AstNodeClone<'ast> for LightDark<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            dark: context.clone_encoded_node(self.dark),
            light: context.clone_encoded_node(self.light),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
#[repr(u8)]
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

#[derive(Clone, Copy)]
enum UnresolvedColorData<'ast> {
    Rgb { b: f32, g: f32 },
    Hsl { h: f32, l: f32 },
    LightDark { dark: Vec<'ast, TokenOrValue<'ast>> },
}
#[derive(Clone, Copy)]
struct UnresolvedColorHeader<'ast> {
    data: UnresolvedColorData<'ast>,
    extra: u32,
}

pub use unresolved_access::{UnresolvedColorRead, UnresolvedComponentsRead, UnresolvedLightRead};

// Borrowed serialization views are not persistent AST or visitor targets.
mod unresolved_access {
    use super::*;

    pub enum UnresolvedColorRead<'context, 'storage, 'id> {
        Rgb {
            b: f32,
            g: f32,
            tail: UnresolvedComponentsRead<'context, 'storage, 'id>,
        },
        Hsl {
            h: f32,
            l: f32,
            tail: UnresolvedComponentsRead<'context, 'storage, 'id>,
        },
        LightDark {
            dark: Vec<'id, TokenOrValue<'id>>,
            light: UnresolvedLightRead<'context, 'storage, 'id>,
        },
    }

    pub struct UnresolvedComponentsRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        extra: usize,
        marker: std::marker::PhantomData<&'id ()>,
    }

    impl<'id> UnresolvedComponentsRead<'_, '_, 'id> {
        /// The remaining RGB red or HSL saturation component.
        pub fn scalar(&self) -> f32 {
            // SAFETY: only RGB/HSL headers construct this view; their first slot is f32.
            unsafe { self.context.extra_slot(self.extra).read_value() }
        }

        pub fn alpha(&self) -> Vec<'id, TokenOrValue<'id>> {
            // SAFETY: RGB/HSL store an alpha token range in their second slot.
            unsafe { self.context.extra_slot(self.extra + 1).read_value() }
        }
    }

    pub struct UnresolvedLightRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        extra: usize,
        marker: std::marker::PhantomData<&'id ()>,
    }

    impl<'id> UnresolvedLightRead<'_, '_, 'id> {
        pub fn tokens(&self) -> Vec<'id, TokenOrValue<'id>> {
            // SAFETY: only LightDark constructs this view, with a range in its first slot.
            unsafe { self.context.extra_slot(self.extra).read_value() }
        }
    }

    impl<'storage> AstContext<'storage> {
        pub fn unresolved_color<'id>(
            &self,
            id: NodeId<'id, UnresolvedColor<'id>>,
        ) -> UnresolvedColorRead<'_, 'storage, 'id> {
            // SAFETY: the checked node kind owns this native header and overflow layout.
            let header: UnresolvedColorHeader<'id> = unsafe { self.node_payload(id).read_value() };
            let extra = header.extra as usize;
            match header.data {
                UnresolvedColorData::Rgb { b, g } => UnresolvedColorRead::Rgb {
                    b,
                    g,
                    tail: UnresolvedComponentsRead {
                        context: self,
                        extra,
                        marker: std::marker::PhantomData,
                    },
                },
                UnresolvedColorData::Hsl { h, l } => UnresolvedColorRead::Hsl {
                    h,
                    l,
                    tail: UnresolvedComponentsRead {
                        context: self,
                        extra,
                        marker: std::marker::PhantomData,
                    },
                },
                UnresolvedColorData::LightDark { dark } => UnresolvedColorRead::LightDark {
                    dark,
                    light: UnresolvedLightRead {
                        context: self,
                        extra,
                        marker: std::marker::PhantomData,
                    },
                },
            }
        }
    }
}

// SAFETY: the header variant selects the typed overflow written with it. Two
// slots are always reserved and reused across RGB/HSL/light-dark transitions.
unsafe impl<'ast> AstNodeStorage<'ast> for UnresolvedColor<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0002_0006);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: UnresolvedColorHeader<'ast> = unsafe { payload.read_value() };
        let extra = header.extra as usize;
        match header.data {
            UnresolvedColorData::Rgb { b, g } => Self::Rgb {
                b,
                g,
                r: unsafe { context.extra_slot(extra).read_value() },
                alpha: unsafe { context.extra_slot(extra + 1).read_value() },
            },
            UnresolvedColorData::Hsl { h, l } => Self::Hsl {
                h,
                l,
                s: unsafe { context.extra_slot(extra).read_value() },
                alpha: unsafe { context.extra_slot(extra + 1).read_value() },
            },
            UnresolvedColorData::LightDark { dark } => Self::LightDark {
                dark,
                light: unsafe { context.extra_slot(extra).read_value() },
            },
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        store_unresolved_color(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: UnresolvedColorHeader<'ast> = unsafe { current.read_value() };
        store_unresolved_color(self, Some(header.extra as usize), context)
    }
}

impl<'ast> AstNodeClone<'ast> for UnresolvedColor<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Rgb { alpha, b, g, r } => Self::Rgb {
                alpha: context.clone_encoded_vec(alpha),
                b,
                g,
                r,
            },
            Self::Hsl { alpha, h, l, s } => Self::Hsl {
                alpha: context.clone_encoded_vec(alpha),
                h,
                l,
                s,
            },
            Self::LightDark { dark, light } => Self::LightDark {
                dark: context.clone_encoded_vec(dark),
                light: context.clone_encoded_vec(light),
            },
        }
    }
}

fn store_unresolved_color<'ast>(
    value: UnresolvedColor<'ast>,
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let (data, slots) = match value {
        UnresolvedColor::Rgb { alpha, b, g, r } => (
            UnresolvedColorData::Rgb { b, g },
            [ExtraData::from_value(r), ExtraData::from_value(alpha)],
        ),
        UnresolvedColor::Hsl { alpha, h, l, s } => (
            UnresolvedColorData::Hsl { h, l },
            [ExtraData::from_value(s), ExtraData::from_value(alpha)],
        ),
        UnresolvedColor::LightDark { dark, light } => (
            UnresolvedColorData::LightDark { dark },
            [ExtraData::from_value(light), ExtraData::default()],
        ),
    };
    let extra = match existing {
        Some(index) => {
            for (offset, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(index + offset, slot);
            }
            index
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::from_value(UnresolvedColorHeader {
        data,
        extra: u32::try_from(extra).expect("extra index exceeds u32"),
    })
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, CssColor, DUMMY_SP, FloatColor, Function, KnownColor, LABColor, SystemColor,
        TokenOrValue, UnresolvedColor,
    };

    #[test]
    fn all_four_component_color_spaces_preserve_channels_and_bits() {
        use crate::PredefinedColor;

        // Fields are listed in authored CSS order, independently of storage order.
        macro_rules! check_spaces {
            ($ty:ident, $access:ident, [$($variant:ident($x:ident, $y:ident, $z:ident) => $space:literal),+ $(,)?]) => {{
                let allocator = Allocator::new();
                let mut ast = AstContext::new_in(&allocator);
                let mut id = None;
                for special in [0, 0x8000_0000, 1, 0x7f7f_ffff, 0x7f80_0000, 0xff80_0000, 0x7fc0_1234] {
                    for position in 0..4 {
                        let mut expected = [1.25_f32, 2.5, 3.75, 0.625].map(f32::to_bits);
                        expected[position] = special;
                        let [first, second, third, opacity] = expected.map(f32::from_bits);
                        for (value, space) in [$(($ty::$variant {
                            $x: first, $y: second, $z: third, alpha: opacity,
                        }, $space)),+] {
                            let node = match id {
                                Some(node) => {
                                    ast.mutate_node(node, |stored, _| *stored = value);
                                    node
                                }
                                None => {
                                    let node = ast.alloc_node(value, DUMMY_SP);
                                    id = Some(node);
                                    node
                                }
                            };
                            assert_eq!(ast.encoded_extra_len(), 1);
                            let checkpoint = ast.node_checkpoint();
                            let (actual_space, actual) = match ast.resolve_node(node) {
                                $($ty::$variant { $x, $y, $z, alpha } =>
                                    ($space, [$x, $y, $z, alpha].map(f32::to_bits))),+
                            };
                            assert_eq!(actual_space, space);
                            assert_eq!(actual, expected, "{space}, channel {position}");
                            let view = ast.$access(node);
                            assert_eq!(view.space_name(), space);
                            let ([a, b, c], alpha) = view.components();
                            assert_eq!([a, b, c, alpha].map(f32::to_bits), expected);
                            assert_eq!(ast.node_checkpoint(), checkpoint);
                            assert_eq!(ast.string_pool().extra_len(), 0);
                        }
                    }
                }
            }};
        }

        check_spaces!(LABColor, lab_color, [
            Lab(l, a, b) => "lab",
            Lch(l, c, h) => "lch",
            Oklab(l, a, b) => "oklab",
            Oklch(l, c, h) => "oklch",
        ]);
        check_spaces!(PredefinedColor, predefined_color, [
            Srgb(r, g, b) => "srgb",
            SrgbLinear(r, g, b) => "srgb-linear",
            DisplayP3(r, g, b) => "display-p3",
            A98Rgb(r, g, b) => "a98-rgb",
            ProphotoRgb(r, g, b) => "prophoto-rgb",
            Rec2020(r, g, b) => "rec2020",
            XyzD50(x, y, z) => "xyz-d50",
            XyzD65(x, y, z) => "xyz-d65",
        ]);
        check_spaces!(FloatColor, float_color, [
            Rgb(r, g, b) => "rgb",
            Hsl(h, s, l) => "hsl",
            Hwb(h, w, b) => "hwb",
        ]);
    }

    #[test]
    fn predefined_color_view_preserves_float_bits() {
        use crate::PredefinedColor;

        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let bits = [0x8000_0000, 0x7fc0_1234, 1, 0x7f80_0000];
        let [first, second, third, alpha] = bits.map(f32::from_bits);
        let id = ast.alloc_node(
            PredefinedColor::Srgb {
                r: first,
                g: second,
                b: third,
                alpha,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for value in [
            PredefinedColor::Srgb {
                r: first,
                g: second,
                b: third,
                alpha,
            },
            PredefinedColor::XyzD65 {
                x: first,
                y: second,
                z: third,
                alpha,
            },
        ] {
            ast.mutate_node(id, |stored, _| *stored = value);
            let (components, alpha) = ast.predefined_color(id).components();
            assert_eq!(components.map(f32::to_bits), [bits[0], bits[1], bits[2]]);
            assert_eq!(alpha.to_bits(), bits[3]);
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    }

    #[test]
    fn unresolved_color_views_preserve_channels_and_distinct_ranges() {
        use crate::UnresolvedColorRead;

        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let child = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let ranges = [0, 1, 2]
            .map(|count| ast.alloc_encoded_vec((0..count).map(|_| TokenOrValue::Color(child))));
        let id = ast.alloc_node(
            UnresolvedColor::LightDark {
                dark: ranges[1],
                light: ranges[2],
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_1234,
        ] {
            for position in 0..3 {
                let mut expected = [1.25_f32, 2.5, 3.75].map(f32::to_bits);
                expected[position] = bits;
                let [first, second, third] = expected.map(f32::from_bits);
                for alpha in ranges {
                    ast.mutate_node(id, |value, _| {
                        *value = UnresolvedColor::Rgb {
                            r: first,
                            g: second,
                            b: third,
                            alpha,
                        }
                    });
                    let UnresolvedColor::Rgb {
                        r,
                        g,
                        b,
                        alpha: actual,
                    } = ast.resolve_node(id)
                    else {
                        panic!("expected RGB");
                    };
                    assert_eq!([r, g, b].map(f32::to_bits), expected);
                    assert_eq!(actual, alpha);
                    let UnresolvedColorRead::Rgb { b, g, tail } = ast.unresolved_color(id) else {
                        panic!("expected RGB view");
                    };
                    assert_eq!([tail.scalar(), g, b].map(f32::to_bits), expected);
                    assert_eq!(tail.alpha(), alpha);

                    ast.mutate_node(id, |value, _| {
                        *value = UnresolvedColor::Hsl {
                            h: first,
                            s: second,
                            l: third,
                            alpha,
                        }
                    });
                    let UnresolvedColor::Hsl {
                        h,
                        s,
                        l,
                        alpha: actual,
                    } = ast.resolve_node(id)
                    else {
                        panic!("expected HSL");
                    };
                    assert_eq!([h, s, l].map(f32::to_bits), expected);
                    assert_eq!(actual, alpha);
                    let UnresolvedColorRead::Hsl { h, l, tail } = ast.unresolved_color(id) else {
                        panic!("expected HSL view");
                    };
                    assert_eq!([h, tail.scalar(), l].map(f32::to_bits), expected);
                    assert_eq!(tail.alpha(), alpha);

                    for light in ranges {
                        ast.mutate_node(id, |value, _| {
                            *value = UnresolvedColor::LightDark { dark: alpha, light }
                        });
                        assert_eq!(
                            ast.resolve_node(id),
                            UnresolvedColor::LightDark { dark: alpha, light }
                        );
                        let UnresolvedColorRead::LightDark {
                            dark,
                            light: actual,
                        } = ast.unresolved_color(id)
                        else {
                            panic!("expected light-dark view");
                        };
                        assert_eq!(dark, alpha);
                        assert_eq!(actual.tokens(), light);
                    }
                    assert_eq!(ast.node_checkpoint(), checkpoint);
                    assert_eq!(ast.string_pool().extra_len(), 0);
                }
            }
        }
    }

    #[test]
    fn native_color_overflow_preserves_float_bits_and_variant_changes() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let color = ast.alloc_node(
            FloatColor::Rgb {
                alpha: 1.0,
                b: 2.0,
                g: 3.0,
                r: 4.0,
            },
            DUMMY_SP,
        );
        let alpha = ast.alloc_encoded_vec(std::iter::empty());
        let unresolved = ast.alloc_node(
            UnresolvedColor::LightDark {
                dark: alpha,
                light: alpha,
            },
            DUMMY_SP,
        );
        assert_eq!(ast.encoded_extra_len(), 3);
        let checkpoint = ast.node_checkpoint();
        for bits in [0x8000_0000, 0x7f80_0000, 0xff80_0000, 0x7fc0_1234] {
            let value = f32::from_bits(bits);
            ast.mutate_node(color, |color, _| {
                *color = FloatColor::Hsl {
                    alpha: value,
                    h: 2.0,
                    l: value,
                    s: 4.0,
                }
            });
            let FloatColor::Hsl { alpha: a, h, l, s } = ast.resolve_node(color) else {
                panic!("expected HSL")
            };
            assert_eq!((a.to_bits(), h, l.to_bits(), s), (bits, 2.0, bits, 4.0));
            ast.mutate_node(unresolved, |color, _| {
                *color = UnresolvedColor::Hsl {
                    alpha,
                    h: 2.0,
                    l: 3.0,
                    s: value,
                }
            });
            let UnresolvedColor::Hsl {
                alpha: actual,
                h,
                l,
                s,
            } = ast.resolve_node(unresolved)
            else {
                panic!("expected unresolved HSL")
            };
            assert_eq!((actual, h, l, s.to_bits()), (alpha, 2.0, 3.0, bits));
            ast.mutate_node(unresolved, |color, _| {
                *color = UnresolvedColor::Rgb {
                    alpha,
                    b: value,
                    g: 3.0,
                    r: 4.0,
                }
            });
            let UnresolvedColor::Rgb {
                alpha: actual,
                b,
                g,
                r,
            } = ast.resolve_node(unresolved)
            else {
                panic!("expected unresolved RGB")
            };
            assert_eq!((actual, b.to_bits(), g, r), (alpha, bits, 3.0, 4.0));
            ast.mutate_node(unresolved, |color, _| {
                *color = UnresolvedColor::LightDark {
                    dark: alpha,
                    light: alpha,
                }
            });
            assert_eq!(
                ast.resolve_node(unresolved),
                UnresolvedColor::LightDark {
                    dark: alpha,
                    light: alpha
                }
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }

    #[test]
    fn css_color_codec_round_trips_inline_and_nested_variants() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);

        let known =
            context.alloc_encoded_node(CssColor::Known(KnownColor::Rebeccapurple), DUMMY_SP);
        assert_eq!(
            context.encoded_node(known),
            CssColor::Known(KnownColor::Rebeccapurple)
        );

        let system =
            context.alloc_encoded_node(CssColor::System(SystemColor::Canvastext), DUMMY_SP);
        assert_eq!(
            context.encoded_node(system),
            CssColor::System(SystemColor::Canvastext)
        );

        let arguments = context.alloc_encoded_vec(std::iter::empty());
        let function = Function::new("color-mix", arguments, &mut context);
        let function = context.alloc_encoded_node(function, DUMMY_SP);
        let color = context.alloc_encoded_node(CssColor::Function(function), DUMMY_SP);
        assert_eq!(context.encoded_node(color), CssColor::Function(function));
    }

    #[test]
    fn four_component_colors_reuse_one_native_overflow_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let id = context.alloc_encoded_node(
            LABColor::Oklch {
                alpha: 0.75,
                c: 0.2,
                h: 120.0,
                l: 0.6,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), 1);
        assert_eq!(
            context.encoded_node(id),
            LABColor::Oklch {
                alpha: 0.75,
                c: 0.2,
                h: 120.0,
                l: 0.6,
            }
        );

        context.mutate_encoded_node(id, |color, _| {
            *color = LABColor::Lab {
                a: -0.1,
                alpha: 1.0,
                b: 0.3,
                l: 0.5,
            };
        });
        assert_eq!(context.encoded_extra_len(), 1);
        assert_eq!(
            context.encoded_node(id),
            LABColor::Lab {
                a: -0.1,
                alpha: 1.0,
                b: 0.3,
                l: 0.5,
            }
        );

        let float = context.alloc_encoded_node(
            FloatColor::Hwb {
                alpha: 0.4,
                b: 0.2,
                h: 30.0,
                w: 0.1,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(float),
            FloatColor::Hwb {
                alpha: 0.4,
                b: 0.2,
                h: 30.0,
                w: 0.1,
            }
        );
    }

    #[test]
    fn unresolved_color_codec_keeps_ranges_in_shared_extra_data() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let ident = context.add_str("--alpha");
        let ident = context.alloc_encoded_node(crate::DashedIdent { value: ident }, DUMMY_SP);
        let alpha = context.alloc_encoded_vec([TokenOrValue::DashedIdent(ident)].into_iter());
        let id = context.alloc_encoded_node(
            UnresolvedColor::Rgb {
                alpha,
                b: 0.3,
                g: 0.2,
                r: 0.1,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), 3);
        assert_eq!(
            context.encoded_node(id),
            UnresolvedColor::Rgb {
                alpha,
                b: 0.3,
                g: 0.2,
                r: 0.1,
            }
        );

        let ident = context.add_str("--dark");
        let ident = context.alloc_encoded_node(crate::DashedIdent { value: ident }, DUMMY_SP);
        let dark = context.alloc_encoded_vec([TokenOrValue::DashedIdent(ident)].into_iter());
        let ident = context.add_str("--light");
        let ident = context.alloc_encoded_node(crate::DashedIdent { value: ident }, DUMMY_SP);
        let light = context.alloc_encoded_vec([TokenOrValue::DashedIdent(ident)].into_iter());
        context.mutate_encoded_node(id, |color, _| {
            *color = UnresolvedColor::LightDark { dark, light };
        });
        assert_eq!(context.encoded_extra_len(), 5);
        assert_eq!(
            context.encoded_node(id),
            UnresolvedColor::LightDark { dark, light }
        );
    }
}
