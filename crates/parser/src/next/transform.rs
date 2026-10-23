//! Transform parsing into the flattened AST.

use super::prelude::*;

impl<'i> Parse<'i> for NumberOrPercentage {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        match input.next()?.clone() {
            OldToken::Number(value) => Ok(Self::Number(value)),
            OldToken::Percentage(value) => Ok(Self::Percentage(value)),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for Transform<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
        let function = input.expect_function()?;
        input.parse_nested_block(|input| match_ignore_ascii_case!(
            function,
            "translate" => parse_translate(input),
            "translatex" => {
                let value = LengthPercentage::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::TranslateX(value), span))
            },
            "translatey" => {
                let value = LengthPercentage::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::TranslateY(value), span))
            },
            "translatez" => {
                let value = Length::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::TranslateZ(value), span))
            },
            "translate3d" => {
                let x = LengthPercentage::parse(input)?;
                expect_comma_or_space_separator(input)?;
                let y = LengthPercentage::parse(input)?;
                expect_comma_or_space_separator(input)?;
                let z = Length::parse(input)?;
                input.expect_exhausted()?;
                Ok(input.ast_mut().alloc_transform(TransformData::Translate3d((x, y, z)), span))
            },
            "scale" => parse_scale(input),
            "scalex" => {
                let value = NumberOrPercentage::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::ScaleX(value), span))
            },
            "scaley" => {
                let value = NumberOrPercentage::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::ScaleY(value), span))
            },
            "scalez" => {
                let value = NumberOrPercentage::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::ScaleZ(value), span))
            },
            "scale3d" => {
                let x = NumberOrPercentage::parse(input)?;
                expect_comma_or_space_separator(input)?;
                let y = NumberOrPercentage::parse(input)?;
                expect_comma_or_space_separator(input)?;
                let z = NumberOrPercentage::parse(input)?;
                input.expect_exhausted()?;
                Ok(input.ast_mut().alloc_transform(TransformData::Scale3d((x, y, z)), span))
            },
            "rotate" => {
                let value = Angle::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::Rotate(value), span))
            },
            "rotatex" => {
                let value = Angle::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::RotateX(value), span))
            },
            "rotatey" => {
                let value = Angle::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::RotateY(value), span))
            },
            "rotatez" => {
                let value = Angle::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::RotateZ(value), span))
            },
            "rotate3d" => {
                let x = input.expect_number()?;
                expect_comma_or_space_separator(input)?;
                let y = input.expect_number()?;
                expect_comma_or_space_separator(input)?;
                let z = input.expect_number()?;
                expect_comma_or_space_separator(input)?;
                let angle = Angle::parse(input)?;
                input.expect_exhausted()?;
                Ok(input.ast_mut().alloc_transform(TransformData::Rotate3d((x, y, z, angle)), span))
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
                Ok(input.ast_mut().alloc_transform(TransformData::Skew((x, y)), span))
            },
            "skewx" => {
                let value = Angle::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::SkewX(value), span))
            },
            "skewy" => {
                let value = Angle::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::SkewY(value), span))
            },
            "perspective" => {
                let value = Length::parse(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::Perspective(value), span))
            },
            "matrix" => {
                let value = parse_matrix(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::Matrix(value), span))
            },
            "matrix3d" => {
                let value = parse_matrix3d(input)?;
                Ok(input.ast_mut().alloc_transform(TransformData::Matrix3d(value), span))
            },
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        ))
    }
}

pub(crate) fn parse_transform_list<'i>(
    input: &mut Compiler<'i>,
) -> Result<&'i [Transform<'i>], ParseError<'i, ParserError<'i>>> {
    let mut values = Vec::new();
    while !input.is_exhausted() {
        values.push(Transform::parse(input)?);
    }
    Ok(input.ast().alloc_slice(&values))
}

fn zero_length_percentage<'i>(input: &mut Compiler<'i>) -> LengthPercentage<'i> {
    with_token_span!(input, |span| input.ast_mut().alloc_dimension_percentage(
        DimensionPercentageData::<LengthValue>::Zero,
        span
    ))
}

fn zero_length<'i>(input: &mut Compiler<'i>) -> Length<'i> {
    let span = token_span(input);
    let value = input.ast_mut().alloc_length_value(
        LengthValueData {
            unit: LengthUnit::Px,
            value: 0.0,
        },
        span,
    );
    input.ast_mut().alloc_length(LengthData::Value(value), span)
}

fn parse_translate<'i>(
    input: &mut Compiler<'i>,
) -> Result<Transform<'i>, ParseError<'i, ParserError<'i>>> {
    let x = LengthPercentage::parse(input)?;
    let y = if input.is_exhausted() {
        zero_length_percentage(input)
    } else {
        expect_comma_or_space_separator(input)?;
        LengthPercentage::parse(input)?
    };
    input.expect_exhausted()?;
    Ok(with_token_span!(input, |span| input
        .ast_mut()
        .alloc_transform(TransformData::Translate((x, y)), span)))
}

fn parse_scale<'i>(
    input: &mut Compiler<'i>,
) -> Result<Transform<'i>, ParseError<'i, ParserError<'i>>> {
    let x = NumberOrPercentage::parse(input)?;
    let y = if input.is_exhausted() {
        x
    } else {
        expect_comma_or_space_separator(input)?;
        NumberOrPercentage::parse(input)?
    };
    input.expect_exhausted()?;
    Ok(with_token_span!(input, |span| input
        .ast_mut()
        .alloc_transform(TransformData::Scale((x, y)), span)))
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
) -> Result<MatrixForFloat<'i>, ParseError<'i, ParserError<'i>>> {
    let mut values = [0.0; 6];
    let length = values.len();
    for (index, value) in values.iter_mut().enumerate() {
        *value = input.expect_number()?;
        if index + 1 < length {
            input.expect_comma()?;
        }
    }
    input.expect_exhausted()?;
    Ok(with_token_span!(input, |span| input
        .ast_mut()
        .alloc_matrix_for_float(
            MatrixForFloatData {
                a: values[0],
                b: values[1],
                c: values[2],
                d: values[3],
                e: values[4],
                f: values[5],
            },
            span,
        )))
}

fn parse_matrix3d<'i>(
    input: &mut Compiler<'i>,
) -> Result<Matrix3DForFloat<'i>, ParseError<'i, ParserError<'i>>> {
    let mut values = [0.0; 16];
    let length = values.len();
    for (index, value) in values.iter_mut().enumerate() {
        *value = input.expect_number()?;
        if index + 1 < length {
            input.expect_comma()?;
        }
    }
    input.expect_exhausted()?;
    Ok(with_token_span!(input, |span| input
        .ast_mut()
        .alloc_matrix_3_d_for_float(
            Matrix3DForFloatData {
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
            },
            span,
        )))
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
        let span = token_span(input);
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(input
                .ast_mut()
                .alloc_perspective(PerspectiveData::None, span));
        }
        let value = Length::parse(input)?;
        Ok(input
            .ast_mut()
            .alloc_perspective(PerspectiveData::Length(value), span))
    }
}

impl<'i> Parse<'i> for Translate<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(input.ast_mut().alloc_translate(TranslateData::None, span));
        }
        let x = LengthPercentage::parse(input)?;
        let y = if input.is_exhausted() {
            zero_length_percentage(input)
        } else {
            expect_comma_or_space_separator(input)?;
            LengthPercentage::parse(input)?
        };
        let z = if input.is_exhausted() {
            zero_length(input)
        } else {
            expect_comma_or_space_separator(input)?;
            Length::parse(input)?
        };
        input.expect_exhausted()?;
        Ok(input
            .ast_mut()
            .alloc_translate(TranslateData::Xyz { x, y, z }, span))
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
            x
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

impl<'i> Parse<'i> for Rotate<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let angle = Angle::parse(input)?;
        input.expect_exhausted()?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_rotate(
                RotateData {
                    angle,
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                span,
            )))
    }
}
