use super::*;

use crate::{AstNodeClone, AstNodeStorage, ExtraData, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
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

// Fixed payload layout for `CssColor`:
//
// byte 0      variant
// byte 1      KnownColor/SystemColor discriminant when applicable
// bytes 2..4  reserved
// bytes 4..8  RGBA bytes or nested NodeId index
// bytes 8..16 reserved
impl<'ast> AstNodeStorage<'ast> for CssColor<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0002_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let data = read_u32(&bytes, 4);
        match bytes[0] {
            0 => Self::CurrentColor,
            1 => Self::Known(KnownColor::from_discriminant(bytes[1])),
            2 => Self::Rgba(RGBA {
                red: bytes[4],
                green: bytes[5],
                blue: bytes[6],
                alpha: bytes[7],
            }),
            3 => Self::Function(context.encoded_node_id_at(data as usize)),
            4 => Self::Lab(context.encoded_node_id_at(data as usize)),
            5 => Self::Predefined(context.encoded_node_id_at(data as usize)),
            6 => Self::Float(context.encoded_node_id_at(data as usize)),
            7 => Self::LightDark(context.encoded_node_id_at(data as usize)),
            8 => Self::System(SystemColor::from_discriminant(bytes[1])),
            _ => panic!("invalid encoded CssColor variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_css_color(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_css_color(self)
    }
}

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

fn encode_css_color(value: CssColor<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        CssColor::CurrentColor => bytes[0] = 0,
        CssColor::Known(value) => {
            bytes[0] = 1;
            bytes[1] = value as u8;
        }
        CssColor::Rgba(value) => {
            bytes[0] = 2;
            bytes[4..8].copy_from_slice(&[value.red, value.green, value.blue, value.alpha]);
        }
        CssColor::Function(value) => write_node_id(&mut bytes, 3, value),
        CssColor::Lab(value) => write_node_id(&mut bytes, 4, value),
        CssColor::Predefined(value) => write_node_id(&mut bytes, 5, value),
        CssColor::Float(value) => write_node_id(&mut bytes, 6, value),
        CssColor::LightDark(value) => write_node_id(&mut bytes, 7, value),
        CssColor::System(value) => {
            bytes[0] = 8;
            bytes[1] = value as u8;
        }
    }
    NodePayload::inline(&bytes)
}

macro_rules! define_known_colors {
    ($($name:literal => $variant:ident($red:literal, $green:literal, $blue:literal, $alpha:literal),)+) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum KnownColor {
            $($variant,)+
        }

        impl KnownColor {
            const ALL: &'static [Self] = &[$(Self::$variant,)+];

            fn from_discriminant(value: u8) -> Self {
                Self::ALL
                    .get(value as usize)
                    .copied()
                    .expect("invalid encoded KnownColor")
            }

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

// Fixed payload layout for four-component floating-point colors:
//
// byte 0       variant
// bytes 1..4   reserved
// bytes 4..8   first component
// bytes 8..12  second component
// bytes 12..16 first extra slot
//
// extra + 0    third component
// extra + 1    fourth component
impl AstNodeStorage<'_> for LABColor {
    const KIND: NodeKind = NodeKind::new(0x0002_0002);

    fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
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

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        let (tag, values) = match self {
            Self::Lab { a, alpha, b, l } => (0, [a, alpha, b, l]),
            Self::Lch { alpha, c, h, l } => (1, [alpha, c, h, l]),
            Self::Oklab { a, alpha, b, l } => (2, [a, alpha, b, l]),
            Self::Oklch { alpha, c, h, l } => (3, [alpha, c, h, l]),
        };
        encode_float_color_payload(tag, values, Some(current.extra_start()), context)
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

impl AstNodeStorage<'_> for PredefinedColor {
    const KIND: NodeKind = NodeKind::new(0x0002_0003);

    fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
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

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        let (tag, values) = encode_predefined_color(self);
        encode_float_color_payload(tag, values, Some(current.extra_start()), context)
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

impl AstNodeStorage<'_> for FloatColor {
    const KIND: NodeKind = NodeKind::new(0x0002_0004);

    fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
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

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        let (tag, values) = match self {
            Self::Rgb { alpha, b, g, r } => (0, [alpha, b, g, r]),
            Self::Hsl { alpha, h, l, s } => (1, [alpha, h, l, s]),
            Self::Hwb { alpha, b, h, w } => (2, [alpha, b, h, w]),
        };
        encode_float_color_payload(tag, values, Some(current.extra_start()), context)
    }
}

impl AstNodeClone<'_> for FloatColor {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_float_color_payload(
    tag: u8,
    values: [f32; 4],
    existing_extra: Option<usize>,
    context: &mut AstContext<'_>,
) -> NodePayload {
    let extra_values = [
        ExtraData::from_u64(values[2].to_bits() as u64),
        ExtraData::from_u64(values[3].to_bits() as u64),
    ];
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, extra_values[0]);
            context.set_extra_slot(extra + 1, extra_values[1]);
            extra
        }
        None => context.alloc_extra_slots(extra_values),
    };
    let mut inline = [0; NodePayload::PARTIAL_INLINE_BYTES];
    inline[0] = tag;
    inline[4..8].copy_from_slice(&values[0].to_bits().to_le_bytes());
    inline[8..12].copy_from_slice(&values[1].to_bits().to_le_bytes());
    NodePayload::with_extra(&inline, extra)
}

fn decode_float_color_payload(payload: NodePayload, context: &AstContext<'_>) -> (u8, [f32; 4]) {
    let bytes = payload.bytes();
    let extra = payload.extra_start();
    (
        bytes[0],
        [
            f32::from_bits(read_u32(&bytes, 4)),
            f32::from_bits(read_u32(&bytes, 8)),
            f32::from_bits(context.extra_slot(extra).as_u64() as u32),
            f32::from_bits(context.extra_slot(extra + 1).as_u64() as u32),
        ],
    )
}

#[derive(Debug, PartialEq, Visit)]
pub struct LightDark<'a> {
    pub dark: NodeId<'a, CssColor<'a>>,
    pub light: NodeId<'a, CssColor<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for LightDark<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0002_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            dark: context.encoded_node_id_at(read_u32(&bytes, 0) as usize),
            light: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_light_dark(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_light_dark(self)
    }
}

impl<'ast> AstNodeClone<'ast> for LightDark<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            dark: context.clone_encoded_node(self.dark),
            light: context.clone_encoded_node(self.light),
        }
    }
}

fn encode_light_dark(value: LightDark<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_u32(&mut bytes, 0, node_index(value.dark));
    write_u32(&mut bytes, 4, node_index(value.light));
    NodePayload::inline(&bytes)
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

impl SystemColor {
    const ALL: &'static [Self] = &[
        Self::Accentcolor,
        Self::Accentcolortext,
        Self::Activetext,
        Self::Buttonborder,
        Self::Buttonface,
        Self::Buttontext,
        Self::Canvas,
        Self::Canvastext,
        Self::Field,
        Self::Fieldtext,
        Self::Graytext,
        Self::Highlight,
        Self::Highlighttext,
        Self::Linktext,
        Self::Mark,
        Self::Marktext,
        Self::Selecteditem,
        Self::Selecteditemtext,
        Self::Visitedtext,
        Self::Activeborder,
        Self::Activecaption,
        Self::Appworkspace,
        Self::Background,
        Self::Buttonhighlight,
        Self::Buttonshadow,
        Self::Captiontext,
        Self::Inactiveborder,
        Self::Inactivecaption,
        Self::Inactivecaptiontext,
        Self::Infobackground,
        Self::Infotext,
        Self::Menu,
        Self::Menutext,
        Self::Scrollbar,
        Self::Threeddarkshadow,
        Self::Threedface,
        Self::Threedhighlight,
        Self::Threedlightshadow,
        Self::Threedshadow,
        Self::Window,
        Self::Windowframe,
        Self::Windowtext,
    ];

    fn from_discriminant(value: u8) -> Self {
        Self::ALL
            .get(value as usize)
            .copied()
            .expect("invalid encoded SystemColor")
    }
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

// `UnresolvedColor` always owns two overflow slots so same-kind mutation can
// reuse them even when changing variants.
//
// byte 0       variant
// bytes 1..4   reserved
// bytes 4..8   first scalar, or dark range start
// bytes 8..12  second scalar, or dark range end
// bytes 12..16 first extra slot
//
// extra + 0    third scalar, or light range
// extra + 1    alpha range, or reserved
impl<'ast> AstNodeStorage<'ast> for UnresolvedColor<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0002_0006);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let extra = payload.extra_start();
        match bytes[0] {
            0 => Self::Rgb {
                alpha: decode_range(context.extra_slot(extra + 1), context),
                b: f32::from_bits(read_u32(&bytes, 4)),
                g: f32::from_bits(read_u32(&bytes, 8)),
                r: f32::from_bits(context.extra_slot(extra).as_u64() as u32),
            },
            1 => Self::Hsl {
                alpha: decode_range(context.extra_slot(extra + 1), context),
                h: f32::from_bits(read_u32(&bytes, 4)),
                l: f32::from_bits(read_u32(&bytes, 8)),
                s: f32::from_bits(context.extra_slot(extra).as_u64() as u32),
            },
            2 => Self::LightDark {
                dark: context
                    .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
                light: decode_range(context.extra_slot(extra), context),
            },
            _ => panic!("invalid encoded UnresolvedColor variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_unresolved_color(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_unresolved_color(self, Some(current.extra_start()), context)
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

fn encode_unresolved_color<'ast>(
    value: UnresolvedColor<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut inline = [0; NodePayload::PARTIAL_INLINE_BYTES];
    let slots = match value {
        UnresolvedColor::Rgb { alpha, b, g, r } => {
            inline[0] = 0;
            write_u32(&mut inline, 4, b.to_bits());
            write_u32(&mut inline, 8, g.to_bits());
            [ExtraData::from_u64(r.to_bits() as u64), encode_range(alpha)]
        }
        UnresolvedColor::Hsl { alpha, h, l, s } => {
            inline[0] = 1;
            write_u32(&mut inline, 4, h.to_bits());
            write_u32(&mut inline, 8, l.to_bits());
            [ExtraData::from_u64(s.to_bits() as u64), encode_range(alpha)]
        }
        UnresolvedColor::LightDark { dark, light } => {
            inline[0] = 2;
            write_range(&mut inline, 4, dark);
            [encode_range(light), ExtraData::default()]
        }
    };
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, slots[0]);
            context.set_extra_slot(extra + 1, slots[1]);
            extra
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::with_extra(&inline, extra)
}

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn write_node_id<T>(bytes: &mut [u8; NodePayload::INLINE_BYTES], tag: u8, id: NodeId<'_, T>) {
    bytes[0] = tag;
    write_u32(bytes, 4, node_index(id));
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact color field is four bytes"),
    )
}

fn encode_range<T>(range: Vec<'_, T>) -> ExtraData {
    let start = u32::try_from(range.start_index()).expect("AST range start exceeds four bytes");
    let end = u32::try_from(range.end_index()).expect("AST range end exceeds four bytes");
    ExtraData::from_u64((end as u64) << 32 | start as u64)
}

fn decode_range<'ast, T>(data: ExtraData, context: &AstContext<'ast>) -> Vec<'ast, T> {
    context.encoded_vec_range(
        data.as_u64() as u32 as usize,
        (data.as_u64() >> 32) as u32 as usize,
    )
}

fn write_range<T>(bytes: &mut [u8], offset: usize, range: Vec<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(range.start_index()).expect("AST range start exceeds four bytes"),
    );
    write_u32(
        bytes,
        offset + 4,
        u32::try_from(range.end_index()).expect("AST range end exceeds four bytes"),
    );
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, CssColor, DUMMY_SP, FloatColor, Function, KnownColor, LABColor, SystemColor,
        TokenOrValue, UnresolvedColor,
    };

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
        let function = context.alloc_encoded_node(Function::new("color-mix", arguments), DUMMY_SP);
        let color = context.alloc_encoded_node(CssColor::Function(function), DUMMY_SP);
        assert_eq!(context.encoded_node(color), CssColor::Function(function));
    }

    #[test]
    fn four_component_color_codecs_reuse_two_overflow_slots() {
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
        assert_eq!(context.encoded_extra_len(), 2);
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
        assert_eq!(context.encoded_extra_len(), 2);
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
        let alpha = context.alloc_encoded_vec([TokenOrValue::DashedIdent("--alpha")].into_iter());
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

        let dark = context.alloc_encoded_vec([TokenOrValue::DashedIdent("--dark")].into_iter());
        let light = context.alloc_encoded_vec([TokenOrValue::DashedIdent("--light")].into_iter());
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
