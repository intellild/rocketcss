use crate::prelude::*;

macro_rules! keyword_parse {
    ($ty:ty, $($name:literal => $variant:expr),+ $(,)?) => {
        impl<'i> Parse<'i> for $ty {
            fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
                let ident = input.expect_ident()?;
                match_ignore_ascii_case!(
                    ident,
                    $( $name => Ok($variant), )+
                    _ => Err(input.new_custom_error(ParserError::InvalidValue)),
                )
            }
        }
    };
}

keyword_parse!(
    FlexDirection,
    "row" => Self::Row,
    "row-reverse" => Self::RowReverse,
    "column" => Self::Column,
    "column-reverse" => Self::ColumnReverse,
);
keyword_parse!(
    FlexWrap,
    "nowrap" => Self::Nowrap,
    "wrap" => Self::Wrap,
    "wrap-reverse" => Self::WrapReverse,
);
keyword_parse!(
    BoxOrient,
    "horizontal" => Self::Horizontal,
    "vertical" => Self::Vertical,
    "inline-axis" => Self::InlineAxis,
    "block-axis" => Self::BlockAxis,
);
keyword_parse!(BoxDirection, "normal" => Self::Normal, "reverse" => Self::Reverse,);
keyword_parse!(
    BoxAlign,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "baseline" => Self::Baseline,
    "stretch" => Self::Stretch,
);
keyword_parse!(
    BoxPack,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "justify" => Self::Justify,
);
keyword_parse!(BoxLines, "single" => Self::Single, "multiple" => Self::Multiple,);
keyword_parse!(
    FlexPack,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "justify" => Self::Justify,
    "distribute" => Self::Distribute,
);
keyword_parse!(
    FlexItemAlign,
    "auto" => Self::Auto,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "baseline" => Self::Baseline,
    "stretch" => Self::Stretch,
);
keyword_parse!(
    FlexLinePack,
    "start" => Self::Start,
    "end" => Self::End,
    "center" => Self::Center,
    "justify" => Self::Justify,
    "distribute" => Self::Distribute,
    "stretch" => Self::Stretch,
);

impl<'i> Parse<'i> for FlexFlow {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut direction = None;
        let mut wrap = None;
        while !input.is_exhausted() {
            if direction.is_none()
                && let Ok(value) = input.try_parse(FlexDirection::parse)
            {
                direction = Some(value);
                continue;
            }
            if wrap.is_none()
                && let Ok(value) = input.try_parse(FlexWrap::parse)
            {
                wrap = Some(value);
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        if direction.is_none() && wrap.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self {
            direction: direction.unwrap_or(FlexDirection::Row),
            wrap: wrap.unwrap_or(FlexWrap::Nowrap),
        })
    }
}

impl<'i> Parse<'i> for Flex<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            input.expect_exhausted()?;
            return Ok(Self {
                basis: input.allocator().boxed(LengthPercentageOrAuto::Auto),
                grow: 0.0,
                shrink: 0.0,
            });
        }
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            input.expect_exhausted()?;
            return Ok(Self {
                basis: input.allocator().boxed(LengthPercentageOrAuto::Auto),
                grow: 1.0,
                shrink: 1.0,
            });
        }

        let mut grow = None;
        let mut shrink = None;
        let mut basis = None;

        while !input.is_exhausted() {
            if grow.is_none()
                && let Ok(value) = input.try_parse(Compiler::expect_number)
            {
                grow = Some(value);
                continue;
            }
            if grow.is_some()
                && shrink.is_none()
                && let Ok(value) = input.try_parse(Compiler::expect_number)
            {
                shrink = Some(value);
                continue;
            }
            if basis.is_none()
                && let Ok(value) = input.try_parse(LengthPercentageOrAuto::parse)
            {
                basis = Some(value);
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        let grow = grow.unwrap_or(1.0);
        let shrink = shrink.unwrap_or(1.0);
        let basis = basis.unwrap_or_else(|| {
            LengthPercentageOrAuto::LengthPercentage(
                input.allocator().boxed(LengthPercentage::Zero),
            )
        });
        input.expect_exhausted()?;
        Ok(Self {
            basis: input.allocator().boxed(basis),
            grow,
            shrink,
        })
    }
}
