use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum Transform<'a> {
    Translate(
        (
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, LengthPercentage<'a>>,
        ),
    ),
    TranslateX(NodeId<'a, LengthPercentage<'a>>),
    TranslateY(NodeId<'a, LengthPercentage<'a>>),
    TranslateZ(NodeId<'a, Length<'a>>),
    Translate3d(
        (
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, Length<'a>>,
        ),
    ),
    Scale((NumberOrPercentage, NumberOrPercentage)),
    ScaleX(NumberOrPercentage),
    ScaleY(NumberOrPercentage),
    ScaleZ(NumberOrPercentage),
    Scale3d((NumberOrPercentage, NumberOrPercentage, NumberOrPercentage)),
    Rotate(Angle),
    RotateX(Angle),
    RotateY(Angle),
    RotateZ(Angle),
    Rotate3d((f32, f32, f32, Angle)),
    Skew((Angle, Angle)),
    SkewX(Angle),
    SkewY(Angle),
    Perspective(NodeId<'a, Length<'a>>),
    Matrix(NodeId<'a, MatrixForFloat>),
    Matrix3d(NodeId<'a, Matrix3DForFloat>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TransformStyle {
    Flat,
    Preserve3d,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TransformBox {
    ContentBox,
    BorderBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Perspective<'a> {
    None,
    Length(NodeId<'a, Length<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Translate<'a> {
    None,
    Xyz {
        x: NodeId<'a, LengthPercentage<'a>>,
        y: NodeId<'a, LengthPercentage<'a>>,
        z: NodeId<'a, Length<'a>>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum Scale {
    None,
    Xyz {
        x: NumberOrPercentage,
        y: NumberOrPercentage,
        z: NumberOrPercentage,
    },
}
