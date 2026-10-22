//! Font value parsing into the flattened AST.

use super::{
    prelude::*,
    values::collect_tokens,
};

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
        let span = DUMMY_SP;
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
            let data = match ident.to_ascii_lowercase().as_str() {
                "xx-small" => FontSizeData::Absolute(AbsoluteFontSize::XxSmall),
                "x-small" => FontSizeData::Absolute(AbsoluteFontSize::XSmall),
                "small" => FontSizeData::Absolute(AbsoluteFontSize::Small),
                "medium" => FontSizeData::Absolute(AbsoluteFontSize::Medium),
                "large" => FontSizeData::Absolute(AbsoluteFontSize::Large),
                "x-large" => FontSizeData::Absolute(AbsoluteFontSize::XLarge),
                "xx-large" => FontSizeData::Absolute(AbsoluteFontSize::XxLarge),
                "xxx-large" => FontSizeData::Absolute(AbsoluteFontSize::XxxLarge),
                "smaller" => FontSizeData::Relative(RelativeFontSize::Smaller),
                "larger" => FontSizeData::Relative(RelativeFontSize::Larger),
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            };
            return Ok(input.ast_mut().alloc_font_size(data, span));
        }
        let value = LengthPercentage::parse(input)?;
        Ok(input
            .ast_mut()
            .alloc_font_size(FontSizeData::Length(value), span))
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
        let span = DUMMY_SP;
        if input
            .try_parse(|input| input.expect_ident_matching("normal"))
            .is_ok()
        {
            return Ok(input
                .ast_mut()
                .alloc_line_height(LineHeightData::Normal, span));
        }
        if let Ok(value) = input.try_parse(Compiler::expect_number) {
            return Ok(input
                .ast_mut()
                .alloc_line_height(LineHeightData::Number(value), span));
        }
        let value = LengthPercentage::parse(input)?;
        Ok(input
            .ast_mut()
            .alloc_line_height(LineHeightData::Length(value), span))
    }
}

impl<'i> Parse<'i> for FontFamily<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = DUMMY_SP;
        if let Ok(name) = input.try_parse(Compiler::expect_string) {
            input.expect_exhausted()?;
            return Ok(input
                .ast_mut()
                .alloc_font_family(FontFamilyData::Custom(name), span));
        }

        let first = input.expect_ident()?;
        if input.is_exhausted() {
            return Ok(FontFamily::from_name(first, input.ast_mut(), span));
        }
        if FontFamily::is_generic_name(first) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        let mut name = std::string::String::from(first);
        while !input.is_exhausted() {
            let part = input.expect_ident()?;
            if FontFamily::is_generic_name(part) {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            name.push(' ');
            name.push_str(part);
        }
        let name = input.allocator().alloc_str(&name);
        Ok(input
            .ast_mut()
            .alloc_font_family(FontFamilyData::Custom(name), span))
    }
}

pub(crate) fn parse_font_family_list<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
) -> Result<&'i [FontFamily<'i>], ParseError<'i, ParserError<'i>>> {
    let mut families = Vec::new();
    loop {
        let family = input.parse_until_before(Delimiter::Comma, |input| {
            if let Ok(family) = input.try_parse(FontFamily::parse) {
                return Ok(family);
            }
            let tokens = collect_tokens(input, depth + 1)?;
            Ok(input
                .ast_mut()
                .alloc_font_family(FontFamilyData::Unparsed(tokens), DUMMY_SP))
        })?;
        families.push(family);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    Ok(input.ast().alloc_slice(&families))
}
