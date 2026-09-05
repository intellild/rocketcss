use crate::prelude::*;

impl<'i> Parse<'i> for NumberOrPercentage {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        match input.next()?.clone() {
            ValueToken::Number(value) => Ok(Self::Number(value)),
            ValueToken::Percentage(value) => Ok(Self::Percentage(value)),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for Transform<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let function = input.expect_function()?;
        input.parse_nested_block(|input| match_ignore_ascii_case!(
            function,
            "translate" => parse_translate(input),
            "translatex" => Ok(Self::TranslateX(store_node(LengthPercentage::parse(input)?, input))),
            "translatey" => Ok(Self::TranslateY(store_node(LengthPercentage::parse(input)?, input))),
            "translatez" => Ok(Self::TranslateZ(store_node(Length::parse(input)?, input))),
            "translate3d" => {
                let x = store_node(LengthPercentage::parse(input)?, input);
                expect_comma_or_space_separator(input)?;
                let y = store_node(LengthPercentage::parse(input)?, input);
                expect_comma_or_space_separator(input)?;
                let z = store_node(Length::parse(input)?, input);
                input.expect_exhausted()?;
                Ok(Self::Translate3d((x, y, z)))
            },
            "scale" => parse_scale(input),
            "scalex" => Ok(Self::ScaleX(NumberOrPercentage::parse(input)?)),
            "scaley" => Ok(Self::ScaleY(NumberOrPercentage::parse(input)?)),
            "scalez" => Ok(Self::ScaleZ(NumberOrPercentage::parse(input)?)),
            "scale3d" => {
                let x = NumberOrPercentage::parse(input)?;
                expect_comma_or_space_separator(input)?;
                let y = NumberOrPercentage::parse(input)?;
                expect_comma_or_space_separator(input)?;
                let z = NumberOrPercentage::parse(input)?;
                input.expect_exhausted()?;
                Ok(Self::Scale3d((x, y, z)))
            },
            "rotate" => Ok(Self::Rotate(Angle::parse(input)?)),
            "rotatex" => Ok(Self::RotateX(Angle::parse(input)?)),
            "rotatey" => Ok(Self::RotateY(Angle::parse(input)?)),
            "rotatez" => Ok(Self::RotateZ(Angle::parse(input)?)),
            "rotate3d" => {
                let x = input.expect_number()?;
                expect_comma_or_space_separator(input)?;
                let y = input.expect_number()?;
                expect_comma_or_space_separator(input)?;
                let z = input.expect_number()?;
                expect_comma_or_space_separator(input)?;
                let angle = Angle::parse(input)?;
                input.expect_exhausted()?;
                Ok(Self::Rotate3d((x, y, z, angle)))
            },
            "skew" => {
                let x = Angle::parse(input)?;
                let y = if input.is_exhausted() {
                    Angle::Deg(0.0)
                } else {
                    expect_comma_or_space_separator(input)?;
                    Angle::parse(input)?
                };
                input.expect_exhausted()?;
                Ok(Self::Skew((x, y)))
            },
            "skewx" => Ok(Self::SkewX(Angle::parse(input)?)),
            "skewy" => Ok(Self::SkewY(Angle::parse(input)?)),
            "perspective" => Ok(Self::Perspective(store_node(Length::parse(input)?, input))),
            "matrix" => Ok(Self::Matrix(store_node(parse_matrix(input)?, input))),
            "matrix3d" => Ok(Self::Matrix3d(store_node(parse_matrix3d(input)?, input))),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        ))
    }
}

pub(crate) fn parse_transform_list<'i>(
    input: &mut Compiler<'i>,
) -> Result<Vec<'i, NodeId<'i, Transform<'i>>>, ParseError<'i, ParserError<'i>>> {
    let mut values = input.allocator().vec();
    while !input.is_exhausted() {
        let value = Transform::parse(input)?;
        values.push(store_node(value, input));
    }
    Ok(values)
}

fn parse_translate<'i>(
    input: &mut Compiler<'i>,
) -> Result<Transform<'i>, ParseError<'i, ParserError<'i>>> {
    let x = store_node(LengthPercentage::parse(input)?, input);
    let y = if input.is_exhausted() {
        store_node(LengthPercentage::Zero, input)
    } else {
        expect_comma_or_space_separator(input)?;
        store_node(LengthPercentage::parse(input)?, input)
    };
    input.expect_exhausted()?;
    Ok(Transform::Translate((x, y)))
}

fn parse_scale<'i>(
    input: &mut Compiler<'i>,
) -> Result<Transform<'i>, ParseError<'i, ParserError<'i>>> {
    let x = NumberOrPercentage::parse(input)?;
    let y = if input.is_exhausted() {
        clone_number_or_percentage(&x)
    } else {
        expect_comma_or_space_separator(input)?;
        NumberOrPercentage::parse(input)?
    };
    input.expect_exhausted()?;
    Ok(Transform::Scale((x, y)))
}

fn clone_number_or_percentage(value: &NumberOrPercentage) -> NumberOrPercentage {
    match value {
        NumberOrPercentage::Number(value) => NumberOrPercentage::Number(*value),
        NumberOrPercentage::Percentage(value) => NumberOrPercentage::Percentage(*value),
    }
}

fn expect_comma_or_space_separator<'i>(
    input: &mut Compiler<'i>,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    if input.try_parse(Compiler::expect_comma).is_ok() {
        return Ok(());
    }
    if input.is_exhausted() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok(())
}

fn parse_matrix<'i>(
    input: &mut Compiler<'i>,
) -> Result<MatrixForFloat, ParseError<'i, ParserError<'i>>> {
    let mut values = [0.0; 6];
    let length = values.len();
    for (index, value) in values.iter_mut().enumerate() {
        *value = input.expect_number()?;
        if index + 1 < length {
            input.expect_comma()?;
        }
    }
    input.expect_exhausted()?;
    Ok(MatrixForFloat {
        a: values[0],
        b: values[1],
        c: values[2],
        d: values[3],
        e: values[4],
        f: values[5],
    })
}

fn parse_matrix3d<'i>(
    input: &mut Compiler<'i>,
) -> Result<Matrix3DForFloat, ParseError<'i, ParserError<'i>>> {
    let mut values = [0.0; 16];
    let length = values.len();
    for (index, value) in values.iter_mut().enumerate() {
        *value = input.expect_number()?;
        if index + 1 < length {
            input.expect_comma()?;
        }
    }
    input.expect_exhausted()?;
    Ok(Matrix3DForFloat {
        m11: values[0],
        m12: values[1],
        m13: values[2],
        m14: values[3],
        m21: values[4],
        m22: values[5],
        m23: values[6],
        m24: values[7],
        m31: values[8],
        m32: values[9],
        m33: values[10],
        m34: values[11],
        m41: values[12],
        m42: values[13],
        m43: values[14],
        m44: values[15],
    })
}

impl<'i> Parse<'i> for TransformStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "flat" => Ok(Self::Flat),
            "preserve-3d" => Ok(Self::Preserve3d),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for TransformBox {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "content-box" => Ok(Self::ContentBox),
            "border-box" => Ok(Self::BorderBox),
            "fill-box" => Ok(Self::FillBox),
            "stroke-box" => Ok(Self::StrokeBox),
            "view-box" => Ok(Self::ViewBox),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for BackfaceVisibility {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "visible" => Ok(Self::Visible),
            "hidden" => Ok(Self::Hidden),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for Perspective<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(Self::None);
        }
        Ok(Self::Length(store_node(Length::parse(input)?, input)))
    }
}

impl<'i> Parse<'i> for Translate<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(Self::None);
        }
        let x = store_node(LengthPercentage::parse(input)?, input);
        let y = if input.is_exhausted() {
            store_node(LengthPercentage::Zero, input)
        } else {
            expect_comma_or_space_separator(input)?;
            store_node(LengthPercentage::parse(input)?, input)
        };
        let z = if input.is_exhausted() {
            store_node(
                Length::Value(LengthValue {
                    unit: LengthUnit::Px,
                    value: 0.0,
                }),
                input,
            )
        } else {
            expect_comma_or_space_separator(input)?;
            store_node(Length::parse(input)?, input)
        };
        input.expect_exhausted()?;
        Ok(Self::Xyz { x, y, z })
    }
}

impl<'i> Parse<'i> for Scale {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(Self::None);
        }
        let x = NumberOrPercentage::parse(input)?;
        let y = if input.is_exhausted() {
            clone_number_or_percentage(&x)
        } else {
            expect_comma_or_space_separator(input)?;
            NumberOrPercentage::parse(input)?
        };
        let z = if input.is_exhausted() {
            NumberOrPercentage::Number(1.0)
        } else {
            expect_comma_or_space_separator(input)?;
            NumberOrPercentage::parse(input)?
        };
        input.expect_exhausted()?;
        Ok(Self::Xyz { x, y, z })
    }
}

impl<'i> Parse<'i> for Rotate {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let angle = Angle::parse(input)?;
        input.expect_exhausted()?;
        Ok(Self {
            angle,
            x: 0.0,
            y: 0.0,
            z: 1.0,
        })
    }
}
