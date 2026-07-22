use crate::prelude::*;

fn zero_position<'i, S>(
    allocator: &'i Allocator,
) -> rocketcss_allocator::boxed::Box<'i, PositionComponent<'i, S>> {
    allocator.boxed(PositionComponent::Length(
        allocator.boxed(DimensionPercentage::Percentage(0.0)),
    ))
}

impl<'i> Parse<'i> for Background<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let color = CssColor::parse(input)?;

        Ok(Self {
            attachment: BackgroundAttachment::Scroll,
            clip: BackgroundClip::BorderBox,
            color: allocator.boxed(color),
            image: allocator.boxed(Image::None),
            origin: BackgroundOrigin::PaddingBox,
            position: allocator.boxed(BackgroundPosition {
                x: zero_position(allocator),
                y: zero_position(allocator),
            }),
            repeat: BackgroundRepeat {
                x: BackgroundRepeatKeyword::Repeat,
                y: BackgroundRepeatKeyword::Repeat,
            },
            size: allocator.boxed(BackgroundSize::Explicit {
                height: allocator.boxed(LengthPercentageOrAuto::Auto),
                width: allocator.boxed(LengthPercentageOrAuto::Auto),
            }),
        })
    }
}
