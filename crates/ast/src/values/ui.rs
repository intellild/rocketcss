use crate::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ScrollbarColor<'a> {
    Auto,
    Colors(NodeId<'a, CssColor<'a>>, NodeId<'a, CssColor<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum PointerEvents {
    Auto,
    None,
    VisiblePainted,
    VisibleFill,
    VisibleStroke,
    Visible,
    Painted,
    Fill,
    Stroke,
    All,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Float {
    None,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Clear {
    None,
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TouchAction {
    Auto,
    None,
    Manipulation,
    PanX,
    PanY,
    PanLeft,
    PanRight,
    PanUp,
    PanDown,
    PinchZoom,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum CursorKeyword {
    Auto,
    Default,
    None,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ColorOrAuto<'a> {
    Auto,
    Color(NodeId<'a, CssColor<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for ColorOrAuto<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0013_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Auto,
            1 => {
                Self::Color(context.encoded_node_id_at(u32::from_le_bytes(
                    bytes[4..8].try_into().unwrap(),
                ) as usize))
            }
            _ => panic!("invalid encoded ColorOrAuto variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Auto => bytes[0] = 0,
            Self::Color(value) => {
                bytes[0] = 1;
                bytes[4..8].copy_from_slice(
                    &u32::try_from(value.index())
                        .expect("AST node ID exceeds four bytes")
                        .to_le_bytes(),
                );
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ColorOrAuto<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Auto => Self::Auto,
            Self::Color(value) => Self::Color(context.clone_encoded_node(value)),
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum CaretShape {
    Auto,
    Bar,
    Block,
    Underscore,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum UserSelect {
    Auto,
    Text,
    None,
    Contain,
    All,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Appearance<'a> {
    None,
    Auto,
    Textfield,
    MenulistButton,
    Button,
    Checkbox,
    Listbox,
    Menulist,
    Meter,
    ProgressBar,
    PushButton,
    Radio,
    Searchfield,
    SliderHorizontal,
    SquareButton,
    Textarea,
    NonStandard(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for Appearance<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0013_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Auto,
            2 => Self::Textfield,
            3 => Self::MenulistButton,
            4 => Self::Button,
            5 => Self::Checkbox,
            6 => Self::Listbox,
            7 => Self::Menulist,
            8 => Self::Meter,
            9 => Self::ProgressBar,
            10 => Self::PushButton,
            11 => Self::Radio,
            12 => Self::Searchfield,
            13 => Self::SliderHorizontal,
            14 => Self::SquareButton,
            15 => Self::Textarea,
            16 => Self::NonStandard(context.resolve_string(read_u32(&bytes) as u64)),
            _ => panic!("invalid encoded Appearance variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = match self {
            Self::None => 0,
            Self::Auto => 1,
            Self::Textfield => 2,
            Self::MenulistButton => 3,
            Self::Button => 4,
            Self::Checkbox => 5,
            Self::Listbox => 6,
            Self::Menulist => 7,
            Self::Meter => 8,
            Self::ProgressBar => 9,
            Self::PushButton => 10,
            Self::Radio => 11,
            Self::Searchfield => 12,
            Self::SliderHorizontal => 13,
            Self::SquareButton => 14,
            Self::Textarea => 15,
            Self::NonStandard(value) => {
                bytes[4..8].copy_from_slice(&context.store_string(value).to_le_bytes());
                16
            }
        };
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Appearance<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

fn read_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes[4..8].try_into().unwrap())
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum PrintColorAdjust {
    Economy,
    Exact,
}
