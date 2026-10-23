use crate::prelude::*;

fn is_whitespace<'ast>(ast: &Ast<'ast>, value: &TokenOrValue<'ast>) -> bool {
    matches!(
        value.data(ast),
        TokenOrValueData::Token(token) if matches!(token.data(ast), TokenData::WhiteSpace(_))
    )
}

impl<'ast, 'ghost> ToCss<'ghost> for MediaList<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let media_queries = self.media_queries(_cx.ast());
        if media_queries.is_empty() {
            return dest.write_str("not all");
        }
        for (index, query) in media_queries.iter().enumerate() {
            if index > 0 {
                dest.delim(Delimiter::Comma)?;
            }
            query.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for MediaQuery<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast();
        let data = ast.data(*self);
        if let Some(condition) = data.condition
            && let MediaConditionData::Unknown(tokens) = condition.data(ast)
        {
            if matches!(data.qualifier, Some(Qualifier::Not))
                && matches!(data.media_type, MediaType::All)
                && matches!(
                    tokens.iter().find(|value| !is_whitespace(ast, value)),
                    Some(value)
                        if matches!(
                            value.data(ast),
                            TokenOrValueData::Token(token)
                                if matches!(token.data(ast), TokenData::ParenthesisBlock)
                        )
                )
            {
                dest.write_str("not ")?;
                return crate::token::write_token_list_trimmed(tokens, dest, _cx);
            }

            if let Some(qualifier) = data.qualifier {
                qualifier.to_css(dest, _cx)?;
                dest.write_char(' ')?;
            }
            let wrote_type = !matches!(data.media_type, MediaType::All);
            if wrote_type || data.qualifier.is_some() {
                data.media_type.to_css(dest, _cx)?;
                dest.write_char(' ')?;
            }
            return crate::token::write_token_list_trimmed(tokens, dest, _cx);
        }

        if let Some(qualifier) = data.qualifier {
            qualifier.to_css(dest, _cx)?;
            dest.write_char(' ')?;
        }

        let has_type = !matches!(data.media_type, MediaType::All);
        match data.media_type {
            MediaType::All if data.qualifier.is_some() || data.condition.is_none() => {
                dest.write_str("all")?
            }
            MediaType::All => {}
            value => value.to_css(dest, _cx)?,
        }

        if let Some(condition) = data.condition {
            if has_type || data.qualifier.is_some() {
                dest.write_str(" and ")?;
            }
            let needs_parens = (has_type || data.qualifier.is_some())
                && matches!(
                    condition.data(ast),
                    MediaConditionData::Operation {
                        operator: Operator::Or,
                        ..
                    }
                );
            if needs_parens {
                dest.write_char('(')?;
            }
            condition.to_css(dest, _cx)?;
            if needs_parens {
                dest.write_char(')')?;
            }
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for MediaCondition<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        write_media_condition(*self, None, dest, _cx)
    }
}

fn write_media_condition<'ast, 'ghost, PrinterT: PrinterTrait>(
    condition: MediaCondition<'ast>,
    parent: Option<&Operator>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast();
    match condition.data(ast) {
        MediaConditionData::Feature(value) => value.to_css(dest, cx),
        MediaConditionData::Not(value) => {
            let wrap_not = parent.is_some();
            if wrap_not {
                dest.write_char('(')?;
            }
            dest.write_str("not ")?;
            let needs_parens = matches!(value.data(ast), MediaConditionData::Operation { .. });
            if needs_parens {
                dest.write_char('(')?;
            }
            write_media_condition(value, None, dest, cx)?;
            if needs_parens {
                dest.write_char(')')?;
            }
            if wrap_not {
                dest.write_char(')')?;
            }
            Ok(())
        }
        MediaConditionData::Operation {
            conditions,
            operator,
        } => {
            let needs_parens = parent.is_some_and(|parent| *parent != operator);
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
                write_media_condition(*condition, Some(&operator), dest, cx)?;
            }
            if needs_parens {
                dest.write_char(')')?;
            }
            Ok(())
        }
        MediaConditionData::Unknown(values) => crate::token::write_token_list(values, dest, cx),
    }
}

impl<'ast, 'ghost, FeatureId: ToCss<'ghost> + Copy> ToCss<'ghost>
    for QueryFeature<'ast, FeatureId>
{
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_char('(')?;
        match self.data(_cx.ast()) {
            QueryFeatureData::Plain { name, value } => {
                name.to_css(dest, _cx)?;
                dest.delim(Delimiter::Colon)?;
                value.to_css(dest, _cx)?;
            }
            QueryFeatureData::Boolean { name } => name.to_css(dest, _cx)?,
            QueryFeatureData::Range {
                name,
                operator,
                value,
            } => {
                name.to_css(dest, _cx)?;
                operator.to_css(dest, _cx)?;
                value.to_css(dest, _cx)?;
            }
            QueryFeatureData::Interval {
                end,
                end_operator,
                name,
                start,
                start_operator,
            } => {
                start.to_css(dest, _cx)?;
                start_operator.to_css(dest, _cx)?;
                name.to_css(dest, _cx)?;
                end_operator.to_css(dest, _cx)?;
                end.to_css(dest, _cx)?;
            }
        }
        dest.write_char(')')
    }
}

impl<'ghost, FeatureId: ToCss<'ghost>> ToCss<'ghost> for MediaFeatureName<'_, FeatureId> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Standard(value) => value.to_css(dest, _cx),
            Self::Custom(value) => {
                dest.write_str("--")?;
                serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
            }
            Self::Unknown(value) => serialize_identifier(value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for MediaFeatureId {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("media feature names are static keywords"),
        )
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for MediaFeatureValue<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            MediaFeatureValueData::Length(value) => value.to_css(dest, _cx),
            MediaFeatureValueData::Number(value) => serialize_number(value, dest),
            MediaFeatureValueData::Integer(value) => serialize_int(value, dest),
            MediaFeatureValueData::Boolean(value) => dest.write_char(if value { '1' } else { '0' }),
            MediaFeatureValueData::Resolution(value) => value.to_css(dest, _cx),
            MediaFeatureValueData::Ratio(value) => value.to_css(dest, _cx),
            MediaFeatureValueData::Ident(value) => serialize_identifier(value, dest),
            MediaFeatureValueData::Env(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for MediaFeatureComparison {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
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

impl<'ghost> ToCss<'ghost> for Operator {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("operators are static keywords"))
    }
}

impl<'ghost> ToCss<'ghost> for MediaType<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
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

impl<'ghost> ToCss<'ghost> for Qualifier {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("qualifiers are static keywords"))
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for SupportsCondition<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast();
        match self.data(ast) {
            SupportsConditionData::Not(value) => {
                dest.write_str("not ")?;
                let needs_parens = matches!(
                    value.data(ast),
                    SupportsConditionData::And(_) | SupportsConditionData::Or(_)
                );
                if needs_parens {
                    dest.write_char('(')?;
                }
                value.to_css(dest, _cx)?;
                if needs_parens {
                    dest.write_char(')')?;
                }
                Ok(())
            }
            SupportsConditionData::And(values) | SupportsConditionData::Or(values) => {
                let is_and = matches!(self.data(ast), SupportsConditionData::And(_));
                let operator = if is_and { " and " } else { " or " };
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_str(operator)?;
                    }
                    let needs_parens = matches!(
                        (is_and, value.data(ast)),
                        (true, SupportsConditionData::Or(_))
                            | (false, SupportsConditionData::And(_))
                    );
                    if needs_parens {
                        dest.write_char('(')?;
                    }
                    value.to_css(dest, _cx)?;
                    if needs_parens {
                        dest.write_char(')')?;
                    }
                }
                Ok(())
            }
            SupportsConditionData::Declaration { property_id, value } => {
                dest.write_char('(')?;
                property_id.to_css(dest, _cx)?;
                dest.delim(Delimiter::Colon)?;
                dest.write_str(value)?;
                dest.write_char(')')
            }
            SupportsConditionData::Selector(value) => {
                dest.write_str("selector(")?;
                dest.write_str(value)?;
                dest.write_char(')')
            }
            SupportsConditionData::Unknown(value) => dest.write_str(value),
        }
    }
}
