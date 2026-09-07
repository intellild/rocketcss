use crate::prelude::*;

impl<'ghost> ToCss<'ghost> for MediaCondition<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_media_condition(self, None, dest, _cx)
    }
}

fn write_media_condition<'ghost, PrinterT: PrinterTrait>(
    condition: &MediaCondition<'_>,
    parent: Option<&Operator>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    match condition {
        MediaCondition::Feature(value) => value.to_css(dest, cx),
        MediaCondition::Not(value) => {
            let value = cx.ast_context().resolve_node(*value);
            let wrap_not = parent.is_some();
            if wrap_not {
                dest.write_char('(')?;
            }
            dest.write_str("not ")?;
            let needs_parens = matches!(value, MediaCondition::Operation { .. });
            if needs_parens {
                dest.write_char('(')?;
            }
            write_media_condition(&value, None, dest, cx)?;
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
            for (index, condition) in cx.ast_context().vec_iter(*conditions).enumerate() {
                if index > 0 {
                    dest.write_str(match operator {
                        Operator::And => " and ",
                        Operator::Or => " or ",
                    })?;
                }
                write_media_condition(
                    &cx.ast_context().resolve_node(condition),
                    Some(operator),
                    dest,
                    cx,
                )?;
            }
            if needs_parens {
                dest.write_char(')')?;
            }
            Ok(())
        }
        MediaCondition::Unknown(values) => {
            crate::token::write_token_list(cx.ast_context().vec_iter(*values), dest, cx)
        }
    }
}

impl<'ghost, FeatureId: ToCss<'ghost> + QueryFeatureId> ToCss<'ghost>
    for QueryFeature<'_, FeatureId>
{
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let feature = cx.ast_context().query_feature(id);
        write_query_feature(|| feature.name(), feature.predicate(), dest, cx)
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let (name, predicate) = match *self {
            Self::Plain { name, value } => (name, QueryFeaturePredicate::Plain(value)),
            Self::Boolean { name } => (name, QueryFeaturePredicate::Boolean),
            Self::Range {
                name,
                operator,
                value,
            } => (name, QueryFeaturePredicate::Range { operator, value }),
            Self::Interval {
                name,
                start,
                start_operator,
                end,
                end_operator,
            } => (
                name,
                QueryFeaturePredicate::Interval {
                    start,
                    start_operator,
                    end,
                    end_operator,
                },
            ),
        };
        write_query_feature(|| name, predicate, dest, cx)
    }
}

fn write_query_feature<'id, 'ghost, F: ToCss<'ghost>, PrinterT: PrinterTrait>(
    name: impl FnOnce() -> MediaFeatureName<'id, F>,
    predicate: QueryFeaturePredicate<'id>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_char('(')?;
    match predicate {
        QueryFeaturePredicate::Plain(value) => {
            name().to_css(dest, cx)?;
            dest.delim(Delimiter::Colon)?;
            value.to_css(dest, cx)?;
        }
        QueryFeaturePredicate::Boolean => name().to_css(dest, cx)?,
        QueryFeaturePredicate::Range { operator, value } => {
            name().to_css(dest, cx)?;
            operator.to_css(dest, cx)?;
            value.to_css(dest, cx)?;
        }
        QueryFeaturePredicate::Interval {
            start,
            start_operator,
            end,
            end_operator,
        } => {
            start.to_css(dest, cx)?;
            start_operator.to_css(dest, cx)?;
            name().to_css(dest, cx)?;
            end_operator.to_css(dest, cx)?;
            end.to_css(dest, cx)?;
        }
    }
    dest.write_char(')')
}

impl<'ghost, FeatureId: ToCss<'ghost>> ToCss<'ghost> for MediaFeatureName<'_, FeatureId> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Standard(value) => value.to_css(dest, _cx),
            Self::Custom(value) => {
                let value = _cx.ast_context().str(*value);
                dest.write_str("--")?;
                serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
            }
            Self::Unknown(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for MediaFeatureId {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("media feature names are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for MediaFeatureValue<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest, _cx),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Integer(value) => serialize_int(*value, dest),
            Self::Boolean(value) => dest.write_char(if *value { '1' } else { '0' }),
            Self::Resolution(value) => value.to_css(dest, _cx),
            Self::Ratio(value) => value.to_css(dest, _cx),
            Self::Ident(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
            Self::Env(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for MediaFeatureComparison {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("operators are static keywords"))
    }
}

impl<'ghost> ToCss<'ghost> for MediaType<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Custom(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("qualifiers are static keywords"))
    }
}

impl<'ghost> ToCss<'ghost> for SupportsCondition<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Not(value) => {
                dest.write_str("not ")?;
                let value = _cx.ast_context().resolve_node(*value);
                let needs_parens = matches!(value, Self::And(_) | Self::Or(_));
                if needs_parens {
                    dest.write_char('(')?;
                }
                value.to_css(dest, _cx)?;
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
                for (index, value) in _cx.ast_context().vec_iter(*values).enumerate() {
                    if index > 0 {
                        dest.write_str(operator)?;
                    }
                    let resolved = _cx.ast_context().resolve_node(value);
                    let needs_parens = matches!(
                        (self, &resolved),
                        (Self::And(_), Self::Or(_)) | (Self::Or(_), Self::And(_))
                    );
                    if needs_parens {
                        dest.write_char('(')?;
                    }
                    resolved.to_css(dest, _cx)?;
                    if needs_parens {
                        dest.write_char(')')?;
                    }
                }
                Ok(())
            }
            Self::Declaration { property_id, value } => {
                dest.write_char('(')?;
                property_id.to_css(dest, _cx)?;
                dest.delim(Delimiter::Colon)?;
                dest.write_str(_cx.ast_context().str(*value))?;
                dest.write_char(')')
            }
            Self::Selector(value) => {
                dest.write_str("selector(")?;
                dest.write_str(_cx.ast_context().str(*value))?;
                dest.write_char(')')
            }
            Self::Unknown(value) => dest.write_str(_cx.ast_context().str(*value)),
        }
    }
}
