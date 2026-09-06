use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ClipPath<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
    Shape {
        reference_box: GeometryBox,
        shape: NodeId<'a, BasicShape<'a>>,
    },
    Box(GeometryBox),
}

impl_inline_node!(ClipPath<'ast>, 0x001c0001);

impl<'ast> AstNodeClone<'ast> for ClipPath<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Shape {
                reference_box,
                shape,
            } => Self::Shape {
                reference_box,
                shape: context.clone_encoded_node(shape),
            },
            value => value,
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum GeometryBox {
    BorderBox,
    PaddingBox,
    ContentBox,
    MarginBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

impl_inline_extra!(GeometryBox);

impl ExtraDataClone<'_> for GeometryBox {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum BasicShape<'a> {
    Inset(NodeId<'a, InsetRect<'a>>),
    Circle(NodeId<'a, CircleShape<'a>>),
    Ellipse(NodeId<'a, EllipseShape<'a>>),
    Polygon(NodeId<'a, Polygon<'a>>),
}

impl_inline_node!(BasicShape<'ast>, 0x001c0002);

impl<'ast> AstNodeClone<'ast> for BasicShape<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Inset(value) => Self::Inset(context.clone_encoded_node(value)),
            Self::Circle(value) => Self::Circle(context.clone_encoded_node(value)),
            Self::Ellipse(value) => Self::Ellipse(context.clone_encoded_node(value)),
            Self::Polygon(value) => Self::Polygon(context.clone_encoded_node(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ShapeRadius<'a> {
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    ClosestSide,
    FarthestSide,
}

impl_inline_node!(ShapeRadius<'ast>, 0x001c0003);

impl<'ast> AstNodeClone<'ast> for ShapeRadius<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
            value => value,
        }
    }
}
