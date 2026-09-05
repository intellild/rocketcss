use crate::prelude::*;

impl<'i> Parse<'i> for Angle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        let ValueToken::Dimension { unit, value } = input.next()?.clone() else {
            return Err(location.new_custom_error(ParserError::InvalidValue));
        };
        match unit {
            Unit::Deg => Ok(Self::Deg(value)),
            Unit::Rad => Ok(Self::Rad(value)),
            Unit::Grad => Ok(Self::Grad(value)),
            Unit::Turn => Ok(Self::Turn(value)),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for FontWeight {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
            return match_ignore_ascii_case!(
                ident,
                "normal" => Ok(Self::Absolute(AbsoluteFontWeight::Normal)),
                "bold" => Ok(Self::Absolute(AbsoluteFontWeight::Bold)),
                "bolder" => Ok(Self::Bolder),
                "lighter" => Ok(Self::Lighter),
                _ => Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }
        let value = input.expect_number()?;
        if !(1.0..=1000.0).contains(&value) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::Absolute(AbsoluteFontWeight::Weight(value)))
    }
}

impl<'i> Parse<'i> for FontSize<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
            return match_ignore_ascii_case!(
                ident,
                "xx-small" => Ok(Self::Absolute(AbsoluteFontSize::XxSmall)),
                "x-small" => Ok(Self::Absolute(AbsoluteFontSize::XSmall)),
                "small" => Ok(Self::Absolute(AbsoluteFontSize::Small)),
                "medium" => Ok(Self::Absolute(AbsoluteFontSize::Medium)),
                "large" => Ok(Self::Absolute(AbsoluteFontSize::Large)),
                "x-large" => Ok(Self::Absolute(AbsoluteFontSize::XLarge)),
                "xx-large" => Ok(Self::Absolute(AbsoluteFontSize::XxLarge)),
                "xxx-large" => Ok(Self::Absolute(AbsoluteFontSize::XxxLarge)),
                "smaller" => Ok(Self::Relative(RelativeFontSize::Smaller)),
                "larger" => Ok(Self::Relative(RelativeFontSize::Larger)),
                _ => Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }
        Ok(Self::Length(store_node(
            LengthPercentage::parse(input)?,
            input,
        )))
    }
}

impl<'i> Parse<'i> for FontStretch {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
            let value = match_ignore_ascii_case!(
                ident,
                "normal" => Some(FontStretchKeyword::Normal),
                "ultra-condensed" => Some(FontStretchKeyword::UltraCondensed),
                "extra-condensed" => Some(FontStretchKeyword::ExtraCondensed),
                "condensed" => Some(FontStretchKeyword::Condensed),
                "semi-condensed" => Some(FontStretchKeyword::SemiCondensed),
                "semi-expanded" => Some(FontStretchKeyword::SemiExpanded),
                "expanded" => Some(FontStretchKeyword::Expanded),
                "extra-expanded" => Some(FontStretchKeyword::ExtraExpanded),
                "ultra-expanded" => Some(FontStretchKeyword::UltraExpanded),
                _ => None,
            );
            return value
                .map(Self::Keyword)
                .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::Percentage(input.expect_percentage()?))
    }
}

impl<'i> Parse<'i> for FontStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "normal" => Ok(Self::Normal),
            "italic" => Ok(Self::Italic),
            "oblique" => Ok(Self::Oblique(
                input.try_parse(Angle::parse).unwrap_or(Angle::Deg(14.0)),
            )),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for FontVariantCaps {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "normal" => Ok(Self::Normal),
            "small-caps" => Ok(Self::SmallCaps),
            "all-small-caps" => Ok(Self::AllSmallCaps),
            "petite-caps" => Ok(Self::PetiteCaps),
            "all-petite-caps" => Ok(Self::AllPetiteCaps),
            "unicase" => Ok(Self::Unicase),
            "titling-caps" => Ok(Self::TitlingCaps),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for LineHeight<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("normal"))
            .is_ok()
        {
            return Ok(Self::Normal);
        }
        if let Ok(value) = input.try_parse(Compiler::expect_number) {
            return Ok(Self::Number(value));
        }
        Ok(Self::Length(store_node(
            LengthPercentage::parse(input)?,
            input,
        )))
    }
}

impl<'i> Parse<'i> for FontFamily<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(name) = input.try_parse(Compiler::expect_string) {
            input.expect_exhausted()?;
            return Ok(Self::Custom(name));
        }

        let allocator = input.allocator();
        let first = input.expect_ident()?;
        if input.is_exhausted() {
            return Ok(Self::from_name(first));
        }
        if !matches!(Self::from_name(first), Self::Custom(_)) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        let mut name = std::string::String::from(first);
        while !input.is_exhausted() {
            let part = input.expect_ident()?;
            if !matches!(Self::from_name(part), Self::Custom(_)) {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            name.push(' ');
            name.push_str(part);
        }
        Ok(Self::Custom(allocator.alloc_str(&name)))
    }
}

pub(crate) fn parse_font_family_list<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
) -> Result<Vec<'i, FontFamily<'i>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let mut families = allocator.vec();
    loop {
        let family = input.parse_until_before(Delimiter::Comma, |input| {
            if let Ok(family) = input.try_parse(FontFamily::parse) {
                return Ok(family);
            }
            super::collect_tokens(input, allocator, depth + 1).map(FontFamily::Unparsed)
        })?;
        families.push(family);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    Ok(families)
}
