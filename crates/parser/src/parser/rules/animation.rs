use crate::prelude::*;

impl<'i> Parse<'i> for Transition<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut property = None;
        let mut duration = None;
        let mut timing_function = None;
        let mut delay = None;

        while !input.is_exhausted() {
            if duration.is_none()
                && let Ok(value) = input.try_parse(Time::parse)
            {
                duration = Some(value);
                continue;
            }
            if timing_function.is_none()
                && let Ok(value) = input.try_parse(EasingFunction::parse)
            {
                timing_function = Some(value);
                continue;
            }
            if delay.is_none()
                && let Ok(value) = input.try_parse(Time::parse)
            {
                delay = Some(value);
                continue;
            }
            if property.is_none()
                && let Ok(name) = input.try_parse(Compiler::expect_ident)
            {
                property = Some(store_node(
                    PropertyId::from_name(name, input.ast_context_mut()),
                    input,
                ));
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        Ok(Self {
            delay: delay.unwrap_or(Time::Seconds(0.0)),
            duration: duration.unwrap_or(Time::Seconds(0.0)),
            property: property.unwrap_or_else(|| store_node(PropertyId::All, input)),
            timing_function: store_node(timing_function.unwrap_or(EasingFunction::Ease), input),
        })
    }
}

pub(crate) fn parse_transition_property_list<'i>(
    input: &mut Compiler<'i>,
) -> Result<Vec<'i, PropertyId<'i>>, ParseError<'i, ParserError<'i>>> {
    let mut values = input.allocator().vec();
    loop {
        let name = input.expect_ident()?;
        values.push(PropertyId::from_name(name, input.ast_context_mut()));
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    Ok(values)
}

impl<'i> Parse<'i> for Animation<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let mut components = allocator.vec();
        let mut duration_claimed = false;
        let mut timing_function_claimed = false;
        let mut delay_claimed = false;
        let mut iteration_count_claimed = false;
        let mut direction_claimed = false;
        let mut fill_mode_claimed = false;
        let mut play_state_claimed = false;
        let mut name_claimed = false;

        // Component classes are claimed in a fixed order with the keyframes
        // name as the last resort, mirroring lightningcss and stylo. The first
        // <time> is the duration and the second the delay; a keyword whose
        // class is already claimed (e.g. `ease 1s linear`) falls through to
        // the name. Components are kept in authored order so round-tripping
        // is lossless; animation-timeline is never parsed from the shorthand.
        while !input.is_exhausted() {
            if !duration_claimed && let Ok(value) = input.try_parse(Time::parse) {
                duration_claimed = true;
                components.push(AnimationComponent::Duration(value));
                continue;
            }
            if !timing_function_claimed && let Ok(value) = input.try_parse(EasingFunction::parse) {
                timing_function_claimed = true;
                components.push(AnimationComponent::TimingFunction(store_node(value, input)));
                continue;
            }
            if !delay_claimed && let Ok(value) = input.try_parse(Time::parse) {
                delay_claimed = true;
                components.push(AnimationComponent::Delay(value));
                continue;
            }
            if !iteration_count_claimed
                && let Ok(value) = input.try_parse(AnimationIterationCount::parse)
            {
                iteration_count_claimed = true;
                components.push(AnimationComponent::IterationCount(value));
                continue;
            }
            if !direction_claimed && let Ok(value) = input.try_parse(AnimationDirection::parse) {
                direction_claimed = true;
                components.push(AnimationComponent::Direction(value));
                continue;
            }
            if !fill_mode_claimed && let Ok(value) = input.try_parse(AnimationFillMode::parse) {
                fill_mode_claimed = true;
                components.push(AnimationComponent::FillMode(value));
                continue;
            }
            if !play_state_claimed && let Ok(value) = input.try_parse(AnimationPlayState::parse) {
                play_state_claimed = true;
                components.push(AnimationComponent::PlayState(value));
                continue;
            }
            if !name_claimed && let Ok(value) = input.try_parse(AnimationName::parse) {
                name_claimed = true;
                components.push(AnimationComponent::Name(store_node(value, input)));
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        if components.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self {
            components: store_vec(components, input),
        })
    }
}

pub(crate) fn parse_animation_list<'i>(
    input: &mut Compiler<'i>,
) -> Result<Vec<'i, Animation<'i>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let mut values = allocator.vec();
    loop {
        values.push(input.parse_until_before(Delimiter::Comma, Animation::parse)?);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    Ok(values)
}
