use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum Transform<'a> {
    Translate((Box<'a, LengthPercentage<'a>>, Box<'a, LengthPercentage<'a>>)),
    TranslateX(Box<'a, LengthPercentage<'a>>),
    TranslateY(Box<'a, LengthPercentage<'a>>),
    TranslateZ(Box<'a, Length<'a>>),
    Translate3d(
        (
            Box<'a, LengthPercentage<'a>>,
            Box<'a, LengthPercentage<'a>>,
            Box<'a, Length<'a>>,
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
    Perspective(Box<'a, Length<'a>>),
    Matrix(Box<'a, MatrixForFloat>),
    Matrix3d(Box<'a, Matrix3DForFloat>),
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
    Length(Box<'a, Length<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Translate<'a> {
    None,
    Xyz {
        x: Box<'a, LengthPercentage<'a>>,
        y: Box<'a, LengthPercentage<'a>>,
        z: Box<'a, Length<'a>>,
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
