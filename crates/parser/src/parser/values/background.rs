use crate::prelude::*;

fn zero_position<S>() -> std::boxed::Box<PositionComponent<S>> {
    std::boxed::Box::new(PositionComponent::Length(std::boxed::Box::new(
        DimensionPercentage::Percentage(0.0),
    )))
}

impl<'i> Parse<'i> for Background<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let color = CssColor::parse(input)?;

        Ok(Self {
            attachment: BackgroundAttachment::Scroll,
            clip: BackgroundClip::BorderBox,
            color: std::boxed::Box::new(color),
            image: std::boxed::Box::new(Image::None),
            origin: BackgroundOrigin::PaddingBox,
            position: std::boxed::Box::new(BackgroundPosition {
                x: zero_position(),
                y: zero_position(),
            }),
            repeat: BackgroundRepeat {
                x: BackgroundRepeatKeyword::Repeat,
                y: BackgroundRepeatKeyword::Repeat,
            },
            size: std::boxed::Box::new(BackgroundSize::Explicit {
                height: std::boxed::Box::new(LengthPercentageOrAuto::Auto),
                width: std::boxed::Box::new(LengthPercentageOrAuto::Auto),
            }),
        })
    }
}
