use crate::*;

use crate::{
    AstNodeClone, AstNodeStorage, ExtraData, ExtraDataClone, ExtraDataCompact, NodeKind,
    NodePayload,
};

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Image<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
    Gradient(NodeId<'a, Gradient<'a>>),
    ImageSet(NodeId<'a, ImageSet<'a>>),
}

impl_inline_node!(Image<'ast>, 0x0003_0001);

impl<'ast> AstNodeClone<'ast> for Image<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Gradient(value) => Self::Gradient(context.clone_encoded_node(value)),
            Self::ImageSet(value) => Self::ImageSet(context.clone_encoded_node(value)),
        }
    }
}

impl_inline_extra!(Image<'ast>);

impl<'ast> ExtraDataClone<'ast> for Image<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_in_context(context)
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Gradient<'a> {
    Linear {
        direction: LineDirection,
        items: Vec<'a, NodeId<'a, GradientItem<'a, LengthValue>>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingLinear {
        direction: LineDirection,
        items: Vec<'a, NodeId<'a, GradientItem<'a, LengthValue>>>,
        vendor_prefix: VendorPrefix,
    },
    Radial {
        items: Vec<'a, NodeId<'a, GradientItem<'a, LengthValue>>>,
        position: NodeId<'a, Position<'a>>,
        shape: NodeId<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingRadial {
        items: Vec<'a, NodeId<'a, GradientItem<'a, LengthValue>>>,
        position: NodeId<'a, Position<'a>>,
        shape: NodeId<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    Conic {
        angle: Angle,
        items: Vec<'a, NodeId<'a, GradientItem<'a, Angle>>>,
        position: NodeId<'a, Position<'a>>,
    },
    RepeatingConic {
        angle: Angle,
        items: Vec<'a, NodeId<'a, GradientItem<'a, Angle>>>,
        position: NodeId<'a, Position<'a>>,
    },
    WebKitGradient(NodeId<'a, WebKitGradient<'a>>),
}

// Flatten nested angle variants so the native header remains 16 bytes.
#[repr(u8)]
#[derive(Clone, Copy)]
enum LineDirectionSlot {
    Deg(f32),
    Rad(f32),
    Grad(f32),
    Turn(f32),
    Horizontal(HorizontalPositionKeyword),
    Vertical(VerticalPositionKeyword),
    Corner {
        horizontal: HorizontalPositionKeyword,
        vertical: VerticalPositionKeyword,
    },
}

impl From<LineDirection> for LineDirectionSlot {
    fn from(value: LineDirection) -> Self {
        match value {
            LineDirection::Angle(Angle::Deg(value)) => Self::Deg(value),
            LineDirection::Angle(Angle::Rad(value)) => Self::Rad(value),
            LineDirection::Angle(Angle::Grad(value)) => Self::Grad(value),
            LineDirection::Angle(Angle::Turn(value)) => Self::Turn(value),
            LineDirection::Horizontal(value) => Self::Horizontal(value),
            LineDirection::Vertical(value) => Self::Vertical(value),
            LineDirection::Corner {
                horizontal,
                vertical,
            } => Self::Corner {
                horizontal,
                vertical,
            },
        }
    }
}
impl From<LineDirectionSlot> for LineDirection {
    fn from(value: LineDirectionSlot) -> Self {
        match value {
            LineDirectionSlot::Deg(value) => Self::Angle(Angle::Deg(value)),
            LineDirectionSlot::Rad(value) => Self::Angle(Angle::Rad(value)),
            LineDirectionSlot::Grad(value) => Self::Angle(Angle::Grad(value)),
            LineDirectionSlot::Turn(value) => Self::Angle(Angle::Turn(value)),
            LineDirectionSlot::Horizontal(value) => Self::Horizontal(value),
            LineDirectionSlot::Vertical(value) => Self::Vertical(value),
            LineDirectionSlot::Corner {
                horizontal,
                vertical,
            } => Self::Corner {
                horizontal,
                vertical,
            },
        }
    }
}

#[derive(Clone, Copy)]
enum GradientAngleUnit {
    Deg,
    Rad,
    Grad,
    Turn,
}

impl GradientAngleUnit {
    fn split(angle: Angle) -> (Self, f32) {
        match angle {
            Angle::Deg(value) => (Self::Deg, value),
            Angle::Rad(value) => (Self::Rad, value),
            Angle::Grad(value) => (Self::Grad, value),
            Angle::Turn(value) => (Self::Turn, value),
        }
    }
    fn angle(self, value: f32) -> Angle {
        match self {
            Self::Deg => Angle::Deg(value),
            Self::Rad => Angle::Rad(value),
            Self::Grad => Angle::Grad(value),
            Self::Turn => Angle::Turn(value),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum GradientData<'ast> {
    Linear {
        repeating: bool,
        vendor_prefix: VendorPrefix,
        direction: LineDirectionSlot,
    },
    Radial {
        repeating: bool,
        vendor_prefix: VendorPrefix,
        position: NodeId<'ast, Position<'ast>>,
        shape: NodeId<'ast, EndingShape<'ast>>,
    },
    Conic {
        repeating: bool,
        unit: GradientAngleUnit,
        value: f32,
        position: NodeId<'ast, Position<'ast>>,
    },
    WebKitGradient(NodeId<'ast, WebKitGradient<'ast>>),
}

#[derive(Clone, Copy)]
struct GradientHeader<'ast> {
    data: GradientData<'ast>,
    extra: u32,
}

const _: () = {
    assert!(std::mem::size_of::<LineDirectionSlot>() == 8);
    assert!(std::mem::size_of::<GradientHeader<'_>>() == 16);
};

pub use gradient_access::{GradientItemsRead, GradientRead};
mod gradient_access {
    use super::*;
    pub struct GradientItemsRead<'context, 'storage, 'id, D: DimensionValue> {
        context: &'context AstContext<'storage>,
        extra: u32,
        marker: std::marker::PhantomData<NodeId<'id, GradientItem<'id, D>>>,
    }
    impl<'id, D: DimensionValue> GradientItemsRead<'_, '_, 'id, D> {
        pub fn items(&self) -> Vec<'id, NodeId<'id, GradientItem<'id, D>>> {
            // SAFETY: the matching gradient variant writes this typed range.
            unsafe { self.context.extra_slot(self.extra as usize).read_value() }
        }
    }
    pub enum GradientRead<'context, 'storage, 'id> {
        Linear {
            repeating: bool,
            vendor_prefix: VendorPrefix,
            direction: LineDirection,
            items: GradientItemsRead<'context, 'storage, 'id, LengthValue>,
        },
        Radial {
            repeating: bool,
            vendor_prefix: VendorPrefix,
            position: NodeId<'id, Position<'id>>,
            shape: NodeId<'id, EndingShape<'id>>,
            items: GradientItemsRead<'context, 'storage, 'id, LengthValue>,
        },
        Conic {
            repeating: bool,
            angle: Angle,
            position: NodeId<'id, Position<'id>>,
            items: GradientItemsRead<'context, 'storage, 'id, Angle>,
        },
        WebKitGradient(NodeId<'id, WebKitGradient<'id>>),
    }
    impl<'storage> AstContext<'storage> {
        pub fn gradient<'id>(
            &self,
            id: NodeId<'id, Gradient<'id>>,
        ) -> GradientRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the kind before reading its native header.
            let header: GradientHeader<'id> = unsafe { self.node_payload(id).read_value() };
            match header.data {
                GradientData::Linear {
                    repeating,
                    vendor_prefix,
                    direction,
                } => GradientRead::Linear {
                    repeating,
                    vendor_prefix,
                    direction: direction.into(),
                    items: GradientItemsRead {
                        context: self,
                        extra: header.extra,
                        marker: std::marker::PhantomData,
                    },
                },
                GradientData::Radial {
                    repeating,
                    vendor_prefix,
                    position,
                    shape,
                } => GradientRead::Radial {
                    repeating,
                    vendor_prefix,
                    position,
                    shape,
                    items: GradientItemsRead {
                        context: self,
                        extra: header.extra,
                        marker: std::marker::PhantomData,
                    },
                },
                GradientData::Conic {
                    repeating,
                    unit,
                    value,
                    position,
                } => GradientRead::Conic {
                    repeating,
                    angle: unit.angle(value),
                    position,
                    items: GradientItemsRead {
                        context: self,
                        extra: header.extra,
                        marker: std::marker::PhantomData,
                    },
                },
                GradientData::WebKitGradient(value) => GradientRead::WebKitGradient(value),
            }
        }
    }
}

// SAFETY: this kind always stores GradientHeader. Each variant writes its typed
// item range before publishing the header; the WebKit variant never reads it.
unsafe impl<'ast> AstNodeStorage<'ast> for Gradient<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_000e);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: GradientHeader<'ast> = unsafe { payload.read_value() };
        match header.data {
            GradientData::Linear {
                repeating,
                vendor_prefix,
                direction,
            } => {
                let items = unsafe { context.extra_slot(header.extra as usize).read_value() };
                let direction = direction.into();
                if repeating {
                    Self::RepeatingLinear {
                        direction,
                        items,
                        vendor_prefix,
                    }
                } else {
                    Self::Linear {
                        direction,
                        items,
                        vendor_prefix,
                    }
                }
            }
            GradientData::Radial {
                repeating,
                vendor_prefix,
                position,
                shape,
            } => {
                let items = unsafe { context.extra_slot(header.extra as usize).read_value() };
                if repeating {
                    Self::RepeatingRadial {
                        items,
                        position,
                        shape,
                        vendor_prefix,
                    }
                } else {
                    Self::Radial {
                        items,
                        position,
                        shape,
                        vendor_prefix,
                    }
                }
            }
            GradientData::Conic {
                repeating,
                unit,
                value,
                position,
            } => {
                let items = unsafe { context.extra_slot(header.extra as usize).read_value() };
                let angle = unit.angle(value);
                if repeating {
                    Self::RepeatingConic {
                        angle,
                        items,
                        position,
                    }
                } else {
                    Self::Conic {
                        angle,
                        items,
                        position,
                    }
                }
            }
            GradientData::WebKitGradient(value) => Self::WebKitGradient(value),
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        store_gradient(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: GradientHeader<'ast> = unsafe { current.read_value() };
        store_gradient(self, Some(header.extra as usize), context)
    }
}

impl<'ast> AstNodeClone<'ast> for Gradient<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Linear {
                direction,
                items,
                vendor_prefix,
            } => Self::Linear {
                direction,
                items: context.clone_encoded_vec(items),
                vendor_prefix,
            },
            Self::RepeatingLinear {
                direction,
                items,
                vendor_prefix,
            } => Self::RepeatingLinear {
                direction,
                items: context.clone_encoded_vec(items),
                vendor_prefix,
            },
            Self::Radial {
                items,
                position,
                shape,
                vendor_prefix,
            } => Self::Radial {
                items: context.clone_encoded_vec(items),
                position: context.clone_encoded_node(position),
                shape: context.clone_encoded_node(shape),
                vendor_prefix,
            },
            Self::RepeatingRadial {
                items,
                position,
                shape,
                vendor_prefix,
            } => Self::RepeatingRadial {
                items: context.clone_encoded_vec(items),
                position: context.clone_encoded_node(position),
                shape: context.clone_encoded_node(shape),
                vendor_prefix,
            },
            Self::Conic {
                angle,
                items,
                position,
            } => Self::Conic {
                angle,
                items: context.clone_encoded_vec(items),
                position: context.clone_encoded_node(position),
            },
            Self::RepeatingConic {
                angle,
                items,
                position,
            } => Self::RepeatingConic {
                angle,
                items: context.clone_encoded_vec(items),
                position: context.clone_encoded_node(position),
            },
            Self::WebKitGradient(value) => Self::WebKitGradient(context.clone_encoded_node(value)),
        }
    }
}

fn store_gradient<'ast>(
    value: Gradient<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let (data, items) = match value {
        Gradient::Linear {
            direction,
            items,
            vendor_prefix,
        } => (
            GradientData::Linear {
                repeating: false,
                vendor_prefix,
                direction: direction.into(),
            },
            ExtraData::from_value(items),
        ),
        Gradient::RepeatingLinear {
            direction,
            items,
            vendor_prefix,
        } => (
            GradientData::Linear {
                repeating: true,
                vendor_prefix,
                direction: direction.into(),
            },
            ExtraData::from_value(items),
        ),
        Gradient::Radial {
            items,
            position,
            shape,
            vendor_prefix,
        } => (
            GradientData::Radial {
                repeating: false,
                vendor_prefix,
                position,
                shape,
            },
            ExtraData::from_value(items),
        ),
        Gradient::RepeatingRadial {
            items,
            position,
            shape,
            vendor_prefix,
        } => (
            GradientData::Radial {
                repeating: true,
                vendor_prefix,
                position,
                shape,
            },
            ExtraData::from_value(items),
        ),
        Gradient::Conic {
            angle,
            items,
            position,
        } => {
            let (unit, value) = GradientAngleUnit::split(angle);
            (
                GradientData::Conic {
                    repeating: false,
                    unit,
                    value,
                    position,
                },
                ExtraData::from_value(items),
            )
        }
        Gradient::RepeatingConic {
            angle,
            items,
            position,
        } => {
            let (unit, value) = GradientAngleUnit::split(angle);
            (
                GradientData::Conic {
                    repeating: true,
                    unit,
                    value,
                    position,
                },
                ExtraData::from_value(items),
            )
        }
        Gradient::WebKitGradient(value) => {
            (GradientData::WebKitGradient(value), ExtraData::default())
        }
    };
    let extra = match existing_extra {
        Some(index) => {
            context.set_extra_slot(index, items);
            index
        }
        None => context.alloc_extra_slots([items]),
    };
    NodePayload::from_value(GradientHeader {
        data,
        extra: u32::try_from(extra).expect("extra index exceeds u32"),
    })
}

#[derive(Debug, PartialEq, Visit)]
pub enum WebKitGradient<'a> {
    Linear {
        from: NodeId<'a, WebKitGradientPoint>,
        to: NodeId<'a, WebKitGradientPoint>,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
    Radial {
        from: NodeId<'a, WebKitGradientPoint>,
        start_radius: f32,
        to: NodeId<'a, WebKitGradientPoint>,
        end_radius: f32,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
}

// The header fits one payload; radii and the stop range each occupy one slot.
#[derive(Clone, Copy)]
struct WebKitGradientHeader<'ast> {
    from: NodeId<'ast, WebKitGradientPoint>,
    to: NodeId<'ast, WebKitGradientPoint>,
    extra: u32,
    radial: bool,
}

pub use webkit_gradient_access::WebKitGradientRead;
mod webkit_gradient_access {
    use super::*;
    pub struct WebKitGradientRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: WebKitGradientHeader<'id>,
    }
    impl<'id> WebKitGradientRead<'_, '_, 'id> {
        pub fn from(&self) -> NodeId<'id, WebKitGradientPoint> {
            self.header.from
        }
        pub fn to(&self) -> NodeId<'id, WebKitGradientPoint> {
            self.header.to
        }
        pub fn radii(&self) -> Option<[f32; 2]> {
            self.header.radial.then(|| {
                // SAFETY: the first extra slot is written as native [f32; 2].
                unsafe {
                    self.context
                        .extra_slot(self.header.extra as usize)
                        .read_value()
                }
            })
        }
        pub fn stops(&self) -> Vec<'id, WebKitColorStop<'id>> {
            // SAFETY: the second extra slot is written as the native stop range.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 1)
                    .read_value()
            }
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn webkit_gradient<'id>(
            &self,
            id: NodeId<'id, WebKitGradient<'id>>,
        ) -> WebKitGradientRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the kind before the native header read.
            WebKitGradientRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: this kind always stores WebKitGradientHeader, followed by typed radii/range slots.
unsafe impl<'ast> AstNodeStorage<'ast> for WebKitGradient<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_000f);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: WebKitGradientHeader<'ast> = unsafe { payload.read_value() };
        let WebKitGradientHeader {
            from,
            to,
            extra,
            radial,
        } = header;
        let stops = unsafe { context.extra_slot(extra as usize + 1).read_value() };
        if radial {
            let [start_radius, end_radius]: [f32; 2] =
                unsafe { context.extra_slot(extra as usize).read_value() };
            Self::Radial {
                from,
                start_radius,
                to,
                end_radius,
                stops,
            }
        } else {
            Self::Linear { from, to, stops }
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        store_webkit_gradient(self, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: WebKitGradientHeader<'ast> = unsafe { current.read_value() };
        store_webkit_gradient(self, Some(header.extra as usize), context)
    }
}

impl<'ast> AstNodeClone<'ast> for WebKitGradient<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Linear { from, to, stops } => Self::Linear {
                from: context.clone_encoded_node(from),
                to: context.clone_encoded_node(to),
                stops: context.clone_encoded_vec(stops),
            },
            Self::Radial {
                from,
                start_radius,
                to,
                end_radius,
                stops,
            } => Self::Radial {
                from: context.clone_encoded_node(from),
                start_radius,
                to: context.clone_encoded_node(to),
                end_radius,
                stops: context.clone_encoded_vec(stops),
            },
        }
    }
}

fn store_webkit_gradient<'ast>(
    value: WebKitGradient<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let (from, to, radii, stops, radial) = match value {
        WebKitGradient::Linear { from, to, stops } => (from, to, [0.0; 2], stops, false),
        WebKitGradient::Radial {
            from,
            start_radius,
            to,
            end_radius,
            stops,
        } => (from, to, [start_radius, end_radius], stops, true),
    };
    let fields = [ExtraData::from_value(radii), ExtraData::from_value(stops)];
    let extra = match existing_extra {
        Some(index) => {
            for (offset, field) in fields.into_iter().enumerate() {
                context.set_extra_slot(index + offset, field);
            }
            index
        }
        None => context.alloc_extra_slots(fields),
    };
    NodePayload::from_value(WebKitGradientHeader {
        from,
        to,
        extra: u32::try_from(extra).expect("extra index exceeds u32"),
        radial,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum LineDirection {
    Angle(Angle),
    Horizontal(HorizontalPositionKeyword),
    Vertical(VerticalPositionKeyword),
    Corner {
        horizontal: HorizontalPositionKeyword,
        vertical: VerticalPositionKeyword,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum VerticalPositionKeyword {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum GradientItem<'a, D: DimensionValue> {
    ColorStop {
        color: NodeId<'a, CssColor<'a>>,
        position: Option<NodeId<'a, DimensionPercentage<'a, D>>>,
    },
    Hint(NodeId<'a, DimensionPercentage<'a, D>>),
}

// SAFETY: each supported dimension has a distinct kind; all child handles fit inline.
unsafe impl<'ast, D: DimensionValue> AstNodeStorage<'ast> for GradientItem<'ast, D> {
    const KIND: NodeKind = D::GRADIENT_ITEM_KIND;
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast, D: DimensionValue> AstNodeClone<'ast> for GradientItem<'ast, D> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::ColorStop { color, position } => Self::ColorStop {
                color: context.clone_encoded_node(color),
                position: position.map(|value| context.clone_encoded_node(value)),
            },
            Self::Hint(value) => Self::Hint(context.clone_encoded_node(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum DimensionPercentage<'a, D: DimensionValue> {
    Dimension(D),
    Percentage(f32),
    /// A unitless zero produced by target-aware minification.
    Zero,
    Calc(NodeId<'a, Calc<'a, DimensionPercentage<'a, D>>>),
}

pub type LengthPercentage<'a> = DimensionPercentage<'a, LengthValue>;
pub type AnglePercentage<'a> = DimensionPercentage<'a, Angle>;

#[doc(hidden)]
pub trait DimensionValue: Copy {
    const NODE_KIND: NodeKind;
    const CALC_KIND: NodeKind;
    const GRADIENT_ITEM_KIND: NodeKind;
    const MATH_FUNCTION_KIND: NodeKind;
}

impl DimensionValue for LengthValue {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0002);
    const CALC_KIND: NodeKind = NodeKind::new(0x0018_0002);
    const GRADIENT_ITEM_KIND: NodeKind = NodeKind::new(0x0003_000c);
    const MATH_FUNCTION_KIND: NodeKind = NodeKind::new(0x0019_0002);
}

impl DimensionValue for Angle {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0003);
    const CALC_KIND: NodeKind = NodeKind::new(0x0018_0003);
    const GRADIENT_ITEM_KIND: NodeKind = NodeKind::new(0x0003_000d);
    const MATH_FUNCTION_KIND: NodeKind = NodeKind::new(0x0019_0003);
}

impl<D: DimensionValue> crate::length::CalcValueCodec for DimensionPercentage<'_, D> {
    const CALC_KIND: NodeKind = D::CALC_KIND;
    const MATH_FUNCTION_KIND: NodeKind = D::MATH_FUNCTION_KIND;
}

// Nodes and lists preserve the same native dimension enum. Capacity checks
// at typed slot access reject generic instances larger than the target slot.
// SAFETY: each dimension kind stores and reads the same native enum type.
unsafe impl<'ast, D: DimensionValue> AstNodeStorage<'ast> for DimensionPercentage<'ast, D> {
    const KIND: NodeKind = D::NODE_KIND;
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast, D: DimensionValue> AstNodeClone<'ast> for DimensionPercentage<'ast, D> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Dimension(value) => Self::Dimension(value),
            Self::Percentage(value) => Self::Percentage(value),
            Self::Zero => Self::Zero,
            Self::Calc(value) => Self::Calc(context.clone_encoded_node(value)),
        }
    }
}

// SAFETY: typed lists publish and read the same native Copy dimension enum.
unsafe impl<'ast, D: DimensionValue> ExtraDataCompact<'ast> for DimensionPercentage<'ast, D> {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        unsafe { data.read_value() }
    }
}

impl<'ast, D: DimensionValue> ExtraDataClone<'ast> for DimensionPercentage<'ast, D> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_in_context(context)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum PositionComponent<'a, S> {
    Center,
    Length(NodeId<'a, LengthPercentage<'a>>),
    Side {
        side: S,
        offset: Option<NodeId<'a, LengthPercentage<'a>>>,
    },
}

trait PositionSide: Copy {
    const NODE_KIND: NodeKind;
}

impl PositionSide for HorizontalPositionKeyword {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0004);
}

impl PositionSide for VerticalPositionKeyword {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0005);
}

// SAFETY: each side type has a distinct node kind and uses its native representation.
unsafe impl<'ast, S: PositionSide> AstNodeStorage<'ast> for PositionComponent<'ast, S> {
    const KIND: NodeKind = S::NODE_KIND;
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast, S: PositionSide> AstNodeClone<'ast> for PositionComponent<'ast, S> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Center => Self::Center,
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            Self::Side { offset, side } => Self::Side {
                offset: offset.map(|value| context.clone_encoded_node(value)),
                side,
            },
        }
    }
}

// SAFETY: typed lists store and read the same native position type.
unsafe impl<'ast, S: PositionSide> ExtraDataCompact<'ast> for PositionComponent<'ast, S> {
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    unsafe fn decode_extra(data: ExtraData) -> Self {
        unsafe { data.read_value() }
    }
}

impl<'ast, S: PositionSide> ExtraDataClone<'ast> for PositionComponent<'ast, S> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_in_context(context)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum EndingShape<'a> {
    Ellipse(NodeId<'a, Ellipse<'a>>),
    Circle(NodeId<'a, Circle<'a>>),
}

impl_inline_node!(EndingShape<'ast>, 0x0003_0006);

impl<'ast> AstNodeClone<'ast> for EndingShape<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Ellipse(value) => Self::Ellipse(context.clone_encoded_node(value)),
            Self::Circle(value) => Self::Circle(context.clone_encoded_node(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Ellipse<'a> {
    Size {
        x: NodeId<'a, LengthPercentage<'a>>,
        y: NodeId<'a, LengthPercentage<'a>>,
    },
    Extent(ShapeExtent),
}

impl_inline_node!(Ellipse<'ast>, 0x0003_0007);

impl<'ast> AstNodeClone<'ast> for Ellipse<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Size { x, y } => Self::Size {
                x: context.clone_encoded_node(x),
                y: context.clone_encoded_node(y),
            },
            Self::Extent(value) => Self::Extent(value),
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum ShapeExtent {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Circle<'a> {
    Radius(NodeId<'a, Length<'a>>),
    Extent(ShapeExtent),
}

impl_inline_node!(Circle<'ast>, 0x0003_0008);

impl<'ast> AstNodeClone<'ast> for Circle<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Radius(value) => Self::Radius(context.clone_encoded_node(value)),
            Self::Extent(value) => Self::Extent(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum WebKitGradientPointComponent<S> {
    Center,
    Number(NumberOrPercentage),
    Side(S),
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}

impl_inline_node!(NumberOrPercentage, 0x0003_000b);

impl AstNodeClone<'_> for NumberOrPercentage {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum BackgroundSize<'a> {
    Explicit {
        height: NodeId<'a, LengthPercentageOrAuto<'a>>,
        width: NodeId<'a, LengthPercentageOrAuto<'a>>,
    },
    Cover,
    Contain,
}

impl_inline_node!(BackgroundSize<'ast>, 0x0003_0009);

impl<'ast> AstNodeClone<'ast> for BackgroundSize<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Explicit { height, width } => Self::Explicit {
                height: context.clone_encoded_node(height),
                width: context.clone_encoded_node(width),
            },
            value => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum LengthPercentageOrAuto<'a> {
    Auto,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
}

impl_inline_node!(LengthPercentageOrAuto<'ast>, 0x0003_000a);

impl<'ast> AstNodeClone<'ast> for LengthPercentageOrAuto<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Auto => Self::Auto,
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
        }
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, BackgroundSize, Circle, CssColor, DUMMY_SP, DimensionPercentage, Ellipse,
        EndingShape, Gradient, GradientItem, HorizontalPositionKeyword, Image,
        LengthPercentageOrAuto, LineDirection, PositionComponent, ShapeExtent, Url, VendorPrefix,
        VerticalPositionKeyword,
    };

    #[test]
    fn native_gradient_switches_all_variants_without_growing_overflow() {
        use crate::{
            Angle, Position, WebKitGradient, WebKitGradientPoint, WebKitGradientPointComponent,
        };
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let x = context.alloc_encoded_node(PositionComponent::Center, DUMMY_SP);
        let y = context.alloc_encoded_node(PositionComponent::Center, DUMMY_SP);
        let position = context.alloc_encoded_node(Position { x, y }, DUMMY_SP);
        let circle =
            context.alloc_encoded_node(Circle::Extent(ShapeExtent::FarthestCorner), DUMMY_SP);
        let shape = context.alloc_encoded_node(EndingShape::Circle(circle), DUMMY_SP);
        let length = context.alloc_encoded_node(DimensionPercentage::Percentage(25.0), DUMMY_SP);
        let length_item = context.alloc_encoded_node(GradientItem::Hint(length), DUMMY_SP);
        let items = context.alloc_encoded_vec([length_item].into_iter());
        let angle =
            context.alloc_encoded_node(DimensionPercentage::<Angle>::Percentage(50.0), DUMMY_SP);
        let angle_item = context.alloc_encoded_node(GradientItem::Hint(angle), DUMMY_SP);
        let angle_items = context.alloc_encoded_vec([angle_item].into_iter());
        let point = context.alloc_encoded_node(
            WebKitGradientPoint {
                x: WebKitGradientPointComponent::Center,
                y: WebKitGradientPointComponent::Center,
            },
            DUMMY_SP,
        );
        let stops = context.alloc_encoded_vec(std::iter::empty());
        let webkit = context.alloc_encoded_node(
            WebKitGradient::Linear {
                from: point,
                to: point,
                stops,
            },
            DUMMY_SP,
        );
        let before = context.encoded_extra_len();
        let node = context.alloc_encoded_node(Gradient::WebKitGradient(webkit), DUMMY_SP);
        let vendor_prefix = VendorPrefix::WEBKIT | VendorPrefix::MOZ;
        for direction in [
            LineDirection::Angle(Angle::Deg(-0.0)),
            LineDirection::Angle(Angle::Rad(1.5)),
            LineDirection::Angle(Angle::Grad(-2.5)),
            LineDirection::Angle(Angle::Turn(0.75)),
            LineDirection::Horizontal(HorizontalPositionKeyword::Left),
            LineDirection::Horizontal(HorizontalPositionKeyword::Right),
            LineDirection::Vertical(VerticalPositionKeyword::Top),
            LineDirection::Vertical(VerticalPositionKeyword::Bottom),
            LineDirection::Corner {
                horizontal: HorizontalPositionKeyword::Right,
                vertical: VerticalPositionKeyword::Top,
            },
        ] {
            for expected in [
                Gradient::Linear {
                    direction,
                    items,
                    vendor_prefix,
                },
                Gradient::RepeatingLinear {
                    direction,
                    items,
                    vendor_prefix,
                },
                Gradient::Radial {
                    items,
                    position,
                    shape,
                    vendor_prefix,
                },
                Gradient::RepeatingRadial {
                    items,
                    position,
                    shape,
                    vendor_prefix,
                },
                Gradient::Conic {
                    angle: Angle::Turn(-0.0),
                    items: angle_items,
                    position,
                },
                Gradient::RepeatingConic {
                    angle: Angle::Grad(42.0),
                    items: angle_items,
                    position,
                },
                Gradient::WebKitGradient(webkit),
            ] {
                context.mutate_encoded_node(node, |value, _| *value = expected);
                assert_eq!(context.encoded_node(node), expected);
                assert_eq!(context.encoded_extra_len(), before + 1);
            }
        }
        let checkpoint = context.node_checkpoint();
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_1234,
        ] {
            let number = f32::from_bits(bits);
            for angle in [
                Angle::Deg(number),
                Angle::Rad(number),
                Angle::Grad(number),
                Angle::Turn(number),
            ] {
                let check_angle = |actual: Angle| {
                    assert_eq!(
                        std::mem::discriminant(&actual),
                        std::mem::discriminant(&angle)
                    );
                    let (Angle::Deg(value)
                    | Angle::Rad(value)
                    | Angle::Grad(value)
                    | Angle::Turn(value)) = actual;
                    assert_eq!(value.to_bits(), bits);
                };
                for repeating in [false, true] {
                    context.mutate_encoded_node(node, |value, _| {
                        *value = if repeating {
                            Gradient::RepeatingConic {
                                angle,
                                items: angle_items,
                                position,
                            }
                        } else {
                            Gradient::Conic {
                                angle,
                                items: angle_items,
                                position,
                            }
                        };
                    });
                    let owned = context.encoded_node(node);
                    assert_eq!(matches!(owned, Gradient::RepeatingConic { .. }), repeating);
                    let (Gradient::Conic {
                        angle: actual,
                        items: actual_items,
                        position: actual_position,
                    }
                    | Gradient::RepeatingConic {
                        angle: actual,
                        items: actual_items,
                        position: actual_position,
                    }) = owned
                    else {
                        panic!("expected conic");
                    };
                    check_angle(actual);
                    assert_eq!((actual_items, actual_position), (angle_items, position));
                    let super::GradientRead::Conic {
                        angle: actual,
                        items: actual_items,
                        position: actual_position,
                        repeating: actual_repeating,
                    } = context.gradient(node)
                    else {
                        panic!("expected conic view");
                    };
                    check_angle(actual);
                    assert_eq!(actual_repeating, repeating);
                    assert_eq!(
                        (actual_items.items(), actual_position),
                        (angle_items, position)
                    );
                    context.mutate_encoded_node(node, |value, _| {
                        *value = if repeating {
                            Gradient::RepeatingLinear {
                                direction: LineDirection::Angle(angle),
                                items,
                                vendor_prefix,
                            }
                        } else {
                            Gradient::Linear {
                                direction: LineDirection::Angle(angle),
                                items,
                                vendor_prefix,
                            }
                        };
                    });
                    let owned = context.encoded_node(node);
                    assert_eq!(matches!(owned, Gradient::RepeatingLinear { .. }), repeating);
                    let (Gradient::Linear {
                        direction: LineDirection::Angle(actual),
                        items: actual_items,
                        vendor_prefix: actual_prefix,
                    }
                    | Gradient::RepeatingLinear {
                        direction: LineDirection::Angle(actual),
                        items: actual_items,
                        vendor_prefix: actual_prefix,
                    }) = owned
                    else {
                        panic!("expected linear angle");
                    };
                    check_angle(actual);
                    assert_eq!((actual_items, actual_prefix), (items, vendor_prefix));
                    let super::GradientRead::Linear {
                        direction: LineDirection::Angle(actual),
                        items: actual_items,
                        vendor_prefix: actual_prefix,
                        repeating: actual_repeating,
                    } = context.gradient(node)
                    else {
                        panic!("expected linear view");
                    };
                    check_angle(actual);
                    assert_eq!(actual_repeating, repeating);
                    assert_eq!(
                        (actual_items.items(), actual_prefix),
                        (items, vendor_prefix)
                    );
                    assert_eq!(context.node_checkpoint(), checkpoint);
                }
            }
        }
    }

    #[test]
    fn native_dimension_nodes_and_lists_preserve_float_bits() {
        use super::DimensionValue;
        use crate::{Angle, Calc, ExtraDataCompact, LengthPercentage, LengthUnit, LengthValue};
        fn check<'ast, D: DimensionValue + 'ast, R: std::fmt::Debug + PartialEq>(
            context: &mut AstContext<'ast>,
            dimension: D,
            describe: impl Fn(D) -> R,
        ) {
            let expected = describe(dimension);
            let value = DimensionPercentage::Dimension(dimension);
            let node = context.alloc_encoded_node(value, DUMMY_SP);
            let slot = value.encode_extra();
            let list_value = unsafe { DimensionPercentage::<D>::decode_extra(slot) };
            for actual in [context.encoded_node(node), list_value] {
                let DimensionPercentage::Dimension(actual) = actual else {
                    panic!("expected dimension")
                };
                assert_eq!(describe(actual), expected);
            }
        }
        assert_eq!(std::mem::size_of::<DimensionPercentage<'_, Angle>>(), 8);
        assert_eq!(
            std::mem::size_of::<DimensionPercentage<'_, LengthValue>>(),
            8
        );
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let calc = context.alloc_encoded_node(Calc::<LengthPercentage>::Number(1.0), DUMMY_SP);
        for value in [LengthPercentage::Zero, LengthPercentage::Calc(calc)] {
            let node = context.alloc_encoded_node(value, DUMMY_SP);
            let slot = value.encode_extra();
            assert_eq!(context.encoded_node(node), value);
            assert_eq!(unsafe { LengthPercentage::decode_extra(slot) }, value);
        }
        for bits in [0, 0x8000_0000, 0x7f80_0000, 0xff80_0000, 0x7fc0_1234] {
            let value = f32::from_bits(bits);
            check(
                &mut context,
                LengthValue {
                    unit: LengthUnit::Cqw,
                    value,
                },
                |value| (value.unit, value.value.to_bits()),
            );
            for dimension in [
                Angle::Deg(value),
                Angle::Rad(value),
                Angle::Grad(value),
                Angle::Turn(value),
            ] {
                check(&mut context, dimension, |angle| {
                    let value = match angle {
                        Angle::Deg(value)
                        | Angle::Rad(value)
                        | Angle::Grad(value)
                        | Angle::Turn(value) => value,
                    };
                    (std::mem::discriminant(&angle), value.to_bits())
                });
            }
            let percentage = DimensionPercentage::<Angle>::Percentage(value);
            let slot = percentage.encode_extra();
            let DimensionPercentage::Percentage(actual) =
                (unsafe { DimensionPercentage::<Angle>::decode_extra(slot) })
            else {
                panic!("expected percentage")
            };
            assert_eq!(actual.to_bits(), bits);
        }
    }

    #[test]
    fn native_position_slots_preserve_side_and_optional_zero_index() {
        use crate::{ExtraDataCompact, NodePayload};
        assert_eq!(
            std::mem::size_of::<PositionComponent<'_, HorizontalPositionKeyword>>(),
            8
        );
        assert_eq!(
            std::mem::size_of::<PositionComponent<'_, VerticalPositionKeyword>>(),
            8
        );
        assert_eq!(std::mem::size_of::<Image<'_>>(), 8);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let offset = context.alloc_encoded_node(DimensionPercentage::Percentage(-0.0), DUMMY_SP);
        assert_eq!(offset.index(), 0);
        for side in [
            HorizontalPositionKeyword::Left,
            HorizontalPositionKeyword::Right,
        ] {
            let values = [
                PositionComponent::Center,
                PositionComponent::Length(offset),
                PositionComponent::Side { side, offset: None },
                PositionComponent::Side {
                    side,
                    offset: Some(offset),
                },
            ];
            let list = context.alloc_encoded_vec(values.into_iter());
            for (index, expected) in values.into_iter().enumerate() {
                assert_eq!(context.encoded_vec_get(list, index), Some(expected));
                let payload = NodePayload::from_value(expected);
                let actual: PositionComponent<'_, HorizontalPositionKeyword> =
                    unsafe { payload.read_value() };
                assert_eq!(actual, expected);
            }
        }
        for side in [
            VerticalPositionKeyword::Top,
            VerticalPositionKeyword::Bottom,
        ] {
            for offset in [None, Some(offset)] {
                let expected = PositionComponent::Side { side, offset };
                let slot = expected.encode_extra();
                let actual =
                    unsafe { PositionComponent::<VerticalPositionKeyword>::decode_extra(slot) };
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn native_webkit_gradient_reuses_overflow_when_switching_variants() {
        use crate::{WebKitGradient, WebKitGradientPoint, WebKitGradientPointComponent};
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let from = context.alloc_encoded_node(
            WebKitGradientPoint {
                x: WebKitGradientPointComponent::Center,
                y: WebKitGradientPointComponent::Center,
            },
            DUMMY_SP,
        );
        let to = context.alloc_encoded_node(
            WebKitGradientPoint {
                x: WebKitGradientPointComponent::Side(HorizontalPositionKeyword::Right),
                y: WebKitGradientPointComponent::Side(VerticalPositionKeyword::Bottom),
            },
            DUMMY_SP,
        );
        let stops = context.alloc_encoded_vec(std::iter::empty());
        let before = context.encoded_extra_len();
        let gradient =
            context.alloc_encoded_node(WebKitGradient::Linear { from, to, stops }, DUMMY_SP);
        assert_eq!(context.encoded_extra_len(), before + 2);
        let checkpoint = context.node_checkpoint();
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_1234,
        ] {
            for position in 0..2 {
                let mut expected = [1.25_f32, -2.5].map(f32::to_bits);
                expected[position] = bits;
                let [first, second] = expected.map(f32::from_bits);
                context.mutate_encoded_node(gradient, |value, _| {
                    *value = WebKitGradient::Radial {
                        from,
                        to,
                        stops,
                        start_radius: first,
                        end_radius: second,
                    }
                });
                let WebKitGradient::Radial {
                    from: actual_from,
                    to: actual_to,
                    stops: actual_stops,
                    start_radius,
                    end_radius,
                } = context.encoded_node(gradient)
                else {
                    panic!("expected radial")
                };
                assert_eq!((actual_from, actual_to, actual_stops), (from, to, stops));
                let view = context.webkit_gradient(gradient);
                assert_eq!(view.from(), from);
                assert_eq!(view.to(), to);
                assert_eq!(view.stops(), stops);
                assert_eq!(view.radii().unwrap().map(f32::to_bits), expected);
                assert_eq!([start_radius, end_radius].map(f32::to_bits), expected);
                context.mutate_encoded_node(gradient, |value, _| {
                    *value = WebKitGradient::Linear { from, to, stops }
                });
                assert_eq!(
                    context.encoded_node(gradient),
                    WebKitGradient::Linear { from, to, stops }
                );
                let view = context.webkit_gradient(gradient);
                assert!(view.radii().is_none());
                assert_eq!(view.from(), from);
                assert_eq!(view.to(), to);
                assert_eq!(view.stops(), stops);
                assert_eq!(context.node_checkpoint(), checkpoint);
            }
        }
    }

    #[test]
    fn gradient_codec_deep_clones_promoted_item_nodes() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let color = context.alloc_encoded_node(CssColor::CurrentColor, DUMMY_SP);
        let position = context.alloc_encoded_node(DimensionPercentage::Percentage(25.0), DUMMY_SP);
        let item = context.alloc_encoded_node(
            GradientItem::ColorStop {
                color,
                position: Some(position),
            },
            DUMMY_SP,
        );
        let items = context.alloc_encoded_vec([item].into_iter());
        let before = context.encoded_extra_len();
        let gradient = context.alloc_encoded_node(
            Gradient::Linear {
                direction: LineDirection::Corner {
                    horizontal: HorizontalPositionKeyword::Right,
                    vertical: VerticalPositionKeyword::Top,
                },
                items,
                vendor_prefix: VendorPrefix::WEBKIT,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 1);
        assert_eq!(
            context.encoded_node(gradient),
            Gradient::Linear {
                direction: LineDirection::Corner {
                    horizontal: HorizontalPositionKeyword::Right,
                    vertical: VerticalPositionKeyword::Top,
                },
                items,
                vendor_prefix: VendorPrefix::WEBKIT,
            }
        );

        let cloned = context.clone_encoded_node(gradient);
        let Gradient::Linear {
            items: cloned_items,
            ..
        } = context.encoded_node(cloned)
        else {
            panic!("expected linear gradient")
        };
        assert_ne!(cloned_items, items);
        assert_ne!(context.encoded_vec_get(cloned_items, 0), Some(item));
    }

    #[test]
    fn image_and_dimension_codecs_round_trip_compact_variants() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("asset.webp");
        let url = context.alloc_encoded_node(Url { url: text }, DUMMY_SP);
        let image = context.alloc_encoded_node(Image::Url(url), DUMMY_SP);
        assert_eq!(context.encoded_node(image), Image::Url(url));

        let length = context.alloc_encoded_node(
            DimensionPercentage::Dimension(crate::LengthValue {
                unit: crate::LengthUnit::Cqw,
                value: 2.5,
            }),
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(length),
            DimensionPercentage::Dimension(crate::LengthValue {
                unit: crate::LengthUnit::Cqw,
                value: 2.5,
            })
        );

        let angle = context.alloc_encoded_node(
            DimensionPercentage::<crate::Angle>::Percentage(33.0),
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(angle),
            DimensionPercentage::<crate::Angle>::Percentage(33.0)
        );
    }

    #[test]
    fn position_and_shape_codecs_preserve_typed_child_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let offset = context.alloc_encoded_node(DimensionPercentage::Percentage(25.0), DUMMY_SP);
        let horizontal = context.alloc_encoded_node(
            PositionComponent::Side {
                offset: Some(offset),
                side: HorizontalPositionKeyword::Right,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(horizontal),
            PositionComponent::Side {
                offset: Some(offset),
                side: HorizontalPositionKeyword::Right,
            }
        );

        let vertical = context.alloc_encoded_node(
            PositionComponent::<VerticalPositionKeyword>::Center,
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(vertical),
            PositionComponent::<VerticalPositionKeyword>::Center
        );

        let ellipse = context.alloc_encoded_node(
            Ellipse::Size {
                x: offset,
                y: offset,
            },
            DUMMY_SP,
        );
        let shape = context.alloc_encoded_node(EndingShape::Ellipse(ellipse), DUMMY_SP);
        assert_eq!(context.encoded_node(shape), EndingShape::Ellipse(ellipse));

        let circle = context.alloc_encoded_node(Circle::Extent(ShapeExtent::ClosestSide), DUMMY_SP);
        assert_eq!(
            context.encoded_node(circle),
            Circle::Extent(ShapeExtent::ClosestSide)
        );
    }

    #[test]
    fn background_size_mutation_reuses_the_same_node_identity() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let auto = context.alloc_encoded_node(LengthPercentageOrAuto::Auto, DUMMY_SP);
        let size = context.alloc_encoded_node(
            BackgroundSize::Explicit {
                height: auto,
                width: auto,
            },
            DUMMY_SP,
        );
        context.mutate_encoded_node(size, |value, _| *value = BackgroundSize::Cover);
        assert_eq!(context.encoded_node(size), BackgroundSize::Cover);
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

impl_inline_extra!(BackgroundAttachment);

impl ExtraDataClone<'_> for BackgroundAttachment {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Border,
    Text,
}

impl_inline_extra!(BackgroundClip);

impl ExtraDataClone<'_> for BackgroundClip {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}

impl_inline_extra!(BackgroundOrigin);

impl ExtraDataClone<'_> for BackgroundOrigin {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}
