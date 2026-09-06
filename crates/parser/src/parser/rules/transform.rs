use crate::prelude::*;

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
