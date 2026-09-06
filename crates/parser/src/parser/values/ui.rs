use crate::prelude::*;

keyword_parse!(
    PointerEvents,
    "auto" => Self::Auto,
    "none" => Self::None,
    "visiblepainted" => Self::VisiblePainted,
    "visiblefill" => Self::VisibleFill,
    "visiblestroke" => Self::VisibleStroke,
    "visible" => Self::Visible,
    "painted" => Self::Painted,
    "fill" => Self::Fill,
    "stroke" => Self::Stroke,
    "all" => Self::All,
);
keyword_parse!(
    Float,
    "none" => Self::None,
    "left" => Self::Left,
    "right" => Self::Right,
    "inline-start" => Self::InlineStart,
    "inline-end" => Self::InlineEnd,
);
keyword_parse!(
    Clear,
    "none" => Self::None,
    "left" => Self::Left,
    "right" => Self::Right,
    "both" => Self::Both,
    "inline-start" => Self::InlineStart,
    "inline-end" => Self::InlineEnd,
);
keyword_parse!(
    TouchAction,
    "auto" => Self::Auto,
    "none" => Self::None,
    "manipulation" => Self::Manipulation,
    "pan-x" => Self::PanX,
    "pan-y" => Self::PanY,
    "pan-left" => Self::PanLeft,
    "pan-right" => Self::PanRight,
    "pan-up" => Self::PanUp,
    "pan-down" => Self::PanDown,
    "pinch-zoom" => Self::PinchZoom,
);
keyword_parse!(ScrollBehavior, "auto" => Self::Auto, "smooth" => Self::Smooth,);
keyword_parse!(
    UserSelect,
    "auto" => Self::Auto,
    "text" => Self::Text,
    "none" => Self::None,
    "contain" => Self::Contain,
    "all" => Self::All,
);

impl<'i> Parse<'i> for ColorOrAuto<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        Ok(Self::Color(parse_css_color(input)?))
    }
}

impl<'i> Parse<'i> for ScrollbarColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            input.expect_exhausted()?;
            return Ok(Self::Auto);
        }
        let first = parse_css_color(input)?;
        let second = parse_css_color(input)?;
        input.expect_exhausted()?;
        Ok(Self::Colors(first, second))
    }
}

keyword_parse!(
    Resize,
    "none" => Self::None,
    "both" => Self::Both,
    "horizontal" => Self::Horizontal,
    "vertical" => Self::Vertical,
    "block" => Self::Block,
    "inline" => Self::Inline,
);
