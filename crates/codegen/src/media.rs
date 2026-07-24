use crate::prelude::*;

impl ToCss for MediaList<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if self.media_queries.is_empty() {
            return dest.write_str("not all");
        }
        for (index, query) in self.media_queries.iter().enumerate() {
            if index > 0 {
                dest.delim(Delimiter::Comma)?;
            }
            query.to_css(dest)?;
        }
        Ok(())
    }
}

impl ToCss for MediaQuery<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if let Some(condition) = &self.condition
            && let MediaCondition::Unknown(tokens) = condition
        {
            if matches!(self.qualifier, Some(Qualifier::Not))
                && matches!(self.media_type, MediaType::All)
                && matches!(
                    tokens.iter().find(|value| {
                        !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
                    }),
                    Some(TokenOrValue::Token(token)) if matches!(**token, Token::ParenthesisBlock)
                )
            {
                dest.write_str("not ")?;
                return crate::token::write_token_list_trimmed(tokens, dest);
            }

            if let Some(qualifier) = &self.qualifier {
                qualifier.to_css(dest)?;
                dest.write_char(' ')?;
            }
            let wrote_type = !matches!(self.media_type, MediaType::All);
            if wrote_type || self.qualifier.is_some() {
                self.media_type.to_css(dest)?;
                dest.write_char(' ')?;
            }
            return crate::token::write_token_list_trimmed(tokens, dest);
        }

        if let Some(qualifier) = &self.qualifier {
            qualifier.to_css(dest)?;
            dest.write_char(' ')?;
        }

        let has_type = !matches!(self.media_type, MediaType::All);
        match &self.media_type {
            MediaType::All if self.qualifier.is_some() || self.condition.is_none() => {
                dest.write_str("all")?
            }
            MediaType::All => {}
            value => value.to_css(dest)?,
        }

        if let Some(condition) = &self.condition {
            if has_type || self.qualifier.is_some() {
                dest.write_str(" and ")?;
            }
            let needs_parens = (has_type || self.qualifier.is_some())
                && matches!(
                    *condition,
                    MediaCondition::Operation {
                        operator: Operator::Or,
                        ..
                    }
                );
            if needs_parens {
                dest.write_char('(')?;
            }
            condition.to_css(dest)?;
            if needs_parens {
                dest.write_char(')')?;
            }
        }
        Ok(())
    }
}

impl ToCss for MediaCondition<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_media_condition(self, None, dest)
    }
}

fn write_media_condition<PrinterT: PrinterTrait>(
    condition: &MediaCondition<'_>,
    parent: Option<&Operator>,
    dest: &mut PrinterT,
) -> fmt::Result {
    match condition {
        MediaCondition::Feature(value) => value.to_css(dest),
        MediaCondition::Not(value) => {
            let wrap_not = parent.is_some();
            if wrap_not {
                dest.write_char('(')?;
            }
            dest.write_str("not ")?;
            let needs_parens = matches!(**value, MediaCondition::Operation { .. });
            if needs_parens {
                dest.write_char('(')?;
            }
            write_media_condition(value, None, dest)?;
            if needs_parens {
                dest.write_char(')')?;
            }
            if wrap_not {
                dest.write_char(')')?;
            }
            Ok(())
        }
        MediaCondition::Operation {
            conditions,
            operator,
        } => {
            let needs_parens = parent.is_some_and(|parent| parent != operator);
            if needs_parens {
                dest.write_char('(')?;
            }
            for (index, condition) in conditions.iter().enumerate() {
                if index > 0 {
                    dest.write_str(match operator {
                        Operator::And => " and ",
                        Operator::Or => " or ",
                    })?;
                }
                write_media_condition(condition, Some(operator), dest)?;
            }
            if needs_parens {
                dest.write_char(')')?;
            }
            Ok(())
        }
        MediaCondition::Unknown(values) => crate::token::write_token_list(values, dest),
    }
}

impl<FeatureId: ToCss> ToCss for QueryFeature<'_, FeatureId> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_char('(')?;
        match self {
            Self::Plain { name, value } => {
                name.to_css(dest)?;
                dest.delim(Delimiter::Colon)?;
                value.to_css(dest)?;
            }
            Self::Boolean { name } => name.to_css(dest)?,
            Self::Range {
                name,
                operator,
                value,
            } => {
                name.to_css(dest)?;
                operator.to_css(dest)?;
                value.to_css(dest)?;
            }
            Self::Interval {
                end,
                end_operator,
                name,
                start,
                start_operator,
            } => {
                start.to_css(dest)?;
                start_operator.to_css(dest)?;
                name.to_css(dest)?;
                end_operator.to_css(dest)?;
                end.to_css(dest)?;
            }
        }
        dest.write_char(')')
    }
}

impl<FeatureId: ToCss> ToCss for MediaFeatureName<'_, FeatureId> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Standard(value) => value.to_css(dest),
            Self::Custom(value) => {
                dest.write_str("--")?;
                serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
            }
            Self::Unknown(value) => serialize_identifier(value, dest),
        }
    }
}

impl ToCss for MediaFeatureId {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("media feature names are static keywords"),
        )
    }
}

impl ToCss for MediaFeatureValue<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Integer(value) => serialize_int(*value, dest),
            Self::Boolean(value) => dest.write_char(if *value { '1' } else { '0' }),
            Self::Resolution(value) => value.to_css(dest),
            Self::Ratio(value) => value.to_css(dest),
            Self::Ident(value) => serialize_identifier(value, dest),
            Self::Env(value) => value.to_css(dest),
        }
    }
}

impl ToCss for MediaFeatureComparison {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.whitespace()?;
        dest.write_str(match self {
            Self::Equal => "=",
            Self::GreaterThan => ">",
            Self::GreaterThanEqual => ">=",
            Self::LessThan => "<",
            Self::LessThanEqual => "<=",
        })?;
        dest.whitespace()
    }
}

impl ToCss for Operator {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("operators are static keywords"))
    }
}

impl ToCss for MediaType<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Custom(value) => serialize_identifier(value, dest),
            value => dest.write_str(
                value
                    .as_css_str()
                    .expect("custom media type handled separately"),
            ),
        }
    }
}

impl ToCss for Qualifier {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("qualifiers are static keywords"))
    }
}

impl ToCss for SupportsCondition<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Not(value) => {
                dest.write_str("not ")?;
                let needs_parens = matches!(**value, Self::And(_) | Self::Or(_));
                if needs_parens {
                    dest.write_char('(')?;
                }
                value.to_css(dest)?;
                if needs_parens {
                    dest.write_char(')')?;
                }
                Ok(())
            }
            Self::And(values) | Self::Or(values) => {
                let operator = if matches!(self, Self::And(_)) {
                    " and "
                } else {
                    " or "
                };
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_str(operator)?;
                    }
                    let needs_parens = matches!(
                        (self, value),
                        (Self::And(_), Self::Or(_)) | (Self::Or(_), Self::And(_))
                    );
                    if needs_parens {
                        dest.write_char('(')?;
                    }
                    value.to_css(dest)?;
                    if needs_parens {
                        dest.write_char(')')?;
                    }
                }
                Ok(())
            }
            Self::Declaration { property_id, value } => {
                dest.write_char('(')?;
                property_id.to_css(dest)?;
                dest.delim(Delimiter::Colon)?;
                dest.write_str(value)?;
                dest.write_char(')')
            }
            Self::Selector(value) => {
                dest.write_str("selector(")?;
                dest.write_str(value)?;
                dest.write_char(')')
            }
            Self::Unknown(value) => dest.write_str(value),
        }
    }
}
