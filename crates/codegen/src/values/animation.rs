use super::*;

keyword_values! {
    StepPosition,
    AnimationDirection,
    AnimationPlayState,
    AnimationFillMode,
    AnimationComposition,
    ScrollAxis,
    Scroller,
    TimelineRangeName,
}

impl<'ghost> ToCss<'ghost> for EasingFunction {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        cx.ast_context().easing_function(id).to_css(dest, cx)
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match *self {
            Self::Linear => dest.write_str("linear"),
            Self::Ease => dest.write_str("ease"),
            Self::EaseIn => dest.write_str("ease-in"),
            Self::EaseOut => dest.write_str("ease-out"),
            Self::EaseInOut => dest.write_str("ease-in-out"),
            Self::CubicBezier { x1, x2, y1, y2 } => write_cubic_bezier([x1, y1, x2, y2], dest),
            Self::Frames(count) => write_frames(count, dest),
            Self::Steps { count, position } => write_steps(count, position, dest, cx),
        }
    }
}
impl<'ghost> ToCss<'ghost> for EasingFunctionRead<'_, '_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            EasingFunctionRead::Linear => dest.write_str("linear"),
            EasingFunctionRead::Ease => dest.write_str("ease"),
            EasingFunctionRead::EaseIn => dest.write_str("ease-in"),
            EasingFunctionRead::EaseOut => dest.write_str("ease-out"),
            EasingFunctionRead::EaseInOut => dest.write_str("ease-in-out"),
            EasingFunctionRead::CubicBezier(value) => write_cubic_bezier(value.coordinates(), dest),
            EasingFunctionRead::Frames(count) => write_frames(*count, dest),
            EasingFunctionRead::Steps { count, position } => {
                write_steps(*count, *position, dest, cx)
            }
        }
    }
}

fn write_cubic_bezier<PrinterT: PrinterTrait>(
    [x1, y1, x2, y2]: [f32; 4],
    dest: &mut PrinterT,
) -> fmt::Result {
    if (x1, y1, x2, y2) == (0.0, 0.0, 1.0, 1.0) {
        return dest.write_str("linear");
    }
    if (x1, y1, x2, y2) == (0.25, 0.1, 0.25, 1.0) {
        return dest.write_str("ease");
    }
    if (x1, y1, x2, y2) == (0.42, 0.0, 1.0, 1.0) {
        return dest.write_str("ease-in");
    }
    if (x1, y1, x2, y2) == (0.0, 0.0, 0.58, 1.0) {
        return dest.write_str("ease-out");
    }
    if (x1, y1, x2, y2) == (0.42, 0.0, 0.58, 1.0) {
        return dest.write_str("ease-in-out");
    }
    dest.write_str("cubic-bezier(")?;
    for (index, value) in [x1, y1, x2, y2].into_iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        serialize_number(value, dest)?;
    }
    dest.write_char(')')
}
fn write_frames<PrinterT: PrinterTrait>(count: i32, dest: &mut PrinterT) -> fmt::Result {
    dest.write_str("frames(")?;
    serialize_int(count, dest)?;
    dest.write_char(')')
}
fn write_steps<'ghost, PrinterT: PrinterTrait>(
    count: i32,
    position: StepPosition,
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if count == 1 {
        match position {
            StepPosition::Start => return dest.write_str("step-start"),
            StepPosition::End => return dest.write_str("step-end"),
            _ => {}
        }
    }
    dest.write_str("steps(")?;
    serialize_int(count, dest)?;
    if !matches!(position, StepPosition::End) {
        dest.delim(Delimiter::Comma)?;
        position.to_css(dest, _cx)?;
    }
    dest.write_char(')')
}

impl<'ghost> ToCss<'ghost> for AnimationIterationCount {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Infinite => dest.write_str("infinite"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for AnimationTimeline<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::None => dest.write_str("none"),
            Self::DashedIdent(value) => {
                let value = _cx.ast_context().str(*value);
                dest.write_str("--")?;
                serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
            }
            Self::Scroll(value) => {
                dest.write_str("scroll(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::View(value) => {
                dest.write_str("view(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for AnimationAttachmentRange<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::TimelineRange { name, offset } => {
                name.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                offset.to_css(dest, _cx)
            }
        }
    }
}
