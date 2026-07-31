use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum Transform {
    Translate(
        (
            std::boxed::Box<LengthPercentage>,
            std::boxed::Box<LengthPercentage>,
        ),
    ),
    TranslateX(std::boxed::Box<LengthPercentage>),
    TranslateY(std::boxed::Box<LengthPercentage>),
    TranslateZ(std::boxed::Box<Length>),
    Translate3d(
        (
            std::boxed::Box<LengthPercentage>,
            std::boxed::Box<LengthPercentage>,
            std::boxed::Box<Length>,
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
    Perspective(std::boxed::Box<Length>),
    Matrix(std::boxed::Box<MatrixForFloat>),
    Matrix3d(std::boxed::Box<Matrix3DForFloat>),
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
pub enum Perspective {
    None,
    Length(std::boxed::Box<Length>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Translate {
    None,
    Xyz {
        x: std::boxed::Box<LengthPercentage>,
        y: std::boxed::Box<LengthPercentage>,
        z: std::boxed::Box<Length>,
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
