use super::*;

impl<'ghost> ToCss<'ghost> for Transition<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let transition = cx.ast_context().transition(id);
        write_transition(
            transition.property(),
            transition.duration(),
            transition.timing_function(),
            transition.nonzero_delay(),
            dest,
            cx,
        )
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let delay = (!matches!(self.delay, Time::Seconds(0.0) | Time::Milliseconds(0.0)))
            .then_some(self.delay);
        write_transition(
            self.property,
            self.duration,
            self.timing_function,
            delay,
            dest,
            cx,
        )
    }
}

fn write_transition<'id, 'ghost, PrinterT: PrinterTrait>(
    property: NodeId<'id, PropertyId<'id>>,
    duration: Time,
    timing_function: NodeId<'id, EasingFunction>,
    delay: Option<Time>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    property.to_css(dest, cx)?;
    dest.write_char(' ')?;
    duration.to_css(dest, cx)?;
    let timing_function = cx.ast_context().easing_function(timing_function);
    if !matches!(timing_function, EasingFunctionRead::Ease) {
        dest.write_char(' ')?;
        timing_function.to_css(dest, cx)?;
    }
    if let Some(delay) = delay {
        dest.write_char(' ')?;
        delay.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for ScrollTimeline {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.scroller.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.axis.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ViewTimeline<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.axis.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.inset.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for AnimationRange<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.start.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.end.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Animation<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        // Components print in their stored order: authored order after
        // parsing, canonical order after the ORDER_VALUES minify pass, which
        // also moves a name colliding with a keyword class behind that class.
        let mut seen_classes = 0u8;
        for (index, component) in ast.vec_iter(self.components).enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            // A quoted name colliding with a keyword class must stay quoted
            // unless the class appears before it; unquoted it would reparse
            // into the class slot.
            if let AnimationComponent::Name(name) = component
                && let name = ast.resolve_node(name)
                && let AnimationName::String(value) = name
                && name
                    .keyword_class(ast)
                    .is_some_and(|class| seen_classes & (1 << class as u8) == 0)
            {
                serialize_string(ast.str(value), dest)?;
                continue;
            }
            if let Some(class) = component.keyword_class() {
                seen_classes |= 1 << class as u8;
            }
            component.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for AnimationComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Name(value) => value.to_css(dest, _cx),
            Self::Duration(value) | Self::Delay(value) => value.to_css(dest, _cx),
            Self::TimingFunction(value) => value.to_css(dest, _cx),
            Self::IterationCount(value) => value.to_css(dest, _cx),
            Self::Direction(value) => value.to_css(dest, _cx),
            Self::FillMode(value) => value.to_css(dest, _cx),
            Self::PlayState(value) => value.to_css(dest, _cx),
        }
    }
}
