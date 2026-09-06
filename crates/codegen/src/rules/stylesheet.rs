mod compilation;

use super::*;
use crate::token::{write_dashed_ident, write_token_list};

impl<'ghost> ToCss<'ghost> for Url<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let url = _cx.ast_context().str(self.url);
        dest.write_str("url(")?;
        if !dest.prettify() && can_write_unquoted_url(url) {
            write_unquoted_url(url, dest)?;
        } else {
            serialize_string(url, dest)?;
        }
        dest.write_char(')')
    }
}

fn can_write_unquoted_url(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(|character| {
            character.is_whitespace()
                || character.is_control()
                || matches!(character, '(' | ')' | '\\')
        })
}

pub(crate) fn write_unquoted_url<PrinterT: PrinterTrait>(
    value: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    let mut start = 0;
    for (index, character) in value.char_indices() {
        let replacement = match character {
            '"' => "%22",
            '\'' => "%27",
            _ => continue,
        };
        dest.write_str(&value[start..index])?;
        dest.write_str(replacement)?;
        start = index + character.len_utf8();
    }
    dest.write_str(&value[start..])
}

impl<'ghost> ToCss<'ghost> for Function<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: rocketcss_ast::AstNodeStorage<'id>,
    {
        write_stored_function(id, dest, cx).map(|_| ())
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if let Some(replacement) = self.replacement {
            return write_function_replacement(replacement, dest, cx);
        }
        write_function(
            self.name(),
            self.arguments,
            self.kind(),
            self.is_identifier(),
            self.is_unquoted_url(),
            dest,
            cx,
        )
    }
}

/// Returns whether this replacement needs separation from a following value.
/// Keep this restricted to the existing RGB/hash/transparent boundary rule.
pub(crate) fn write_stored_function<'ghost, PrinterT: PrinterTrait>(
    id: NodeId<'_, Function<'_>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> Result<bool, fmt::Error> {
    let function = cx.ast_context().function(id);
    if let Some(replacement) = function.replacement() {
        write_function_replacement(replacement, dest, cx)?;
        return Ok(matches!(
            replacement,
            FunctionReplacement::Rgb { .. }
                | FunctionReplacement::Rgba { alpha: 0.0, .. }
                | FunctionReplacement::Rgba { use_hex: true, .. }
        ));
    }
    write_function(
        function.name(),
        function.arguments(),
        function.kind(),
        function.is_identifier(),
        function.is_unquoted_url(),
        dest,
        cx,
    )?;
    Ok(false)
}

fn write_function_replacement<'ghost, PrinterT: PrinterTrait>(
    replacement: FunctionReplacement,
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    match replacement {
        FunctionReplacement::GrayAlpha { alpha, lightness } => {
            dest.write_str("hsla(0,0%,")?;
            serialize_number(lightness * 100.0, dest)?;
            dest.write_str("%,")?;
            serialize_number(alpha, dest)?;
            dest.write_char(')')
        }
        FunctionReplacement::Number(value) => serialize_number(value, dest),
        FunctionReplacement::Dimension { unit, value } => {
            serialize_dimension(value, &unit, dest, _cx)
        }
        FunctionReplacement::Percentage(value) => {
            serialize_number(value * 100.0, dest)?;
            dest.write_char('%')
        }
        FunctionReplacement::Rgb { blue, green, red } => write_minified_rgb(red, green, blue, dest),
        FunctionReplacement::Rgba {
            alpha,
            blue,
            green,
            red,
            use_hex,
        } => write_minified_rgba(red, green, blue, alpha, use_hex, dest),
    }
}

fn write_function<'ast, 'ghost, PrinterT: PrinterTrait>(
    name: rocketcss_common::AstStr<'ast>,
    arguments: AstVec<'ast, TokenOrValue<'ast>>,
    kind: KnownFunction,
    is_identifier: bool,
    is_unquoted_url: bool,
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    serialize_identifier(_cx.ast_context().str(name), dest)?;
    if is_identifier {
        return Ok(());
    }
    dest.write_char('(')?;
    let mut values = _cx.ast_context().vec_iter(arguments);
    if is_unquoted_url {
        let Some(TokenOrValue::Token(token)) = values.next() else {
            unreachable!("unquoted URL functions retain one string token")
        };
        assert!(
            values.next().is_none(),
            "unquoted URL functions retain one string token"
        );
        let Token::String(value) = _cx.ast_context().resolve_node(token) else {
            unreachable!("unquoted URL functions retain one string token")
        };
        write_unquoted_url(_cx.ast_context().str(value), dest)?;
        return dest.write_char(')');
    }
    write_token_list(values, dest, _cx)?;
    if kind.is_variable() && crate::token::token_list_ends_with_comma(arguments, _cx) {
        dest.write_char(' ')?;
    }
    dest.write_char(')')
}

fn write_minified_rgba<PrinterT: PrinterTrait>(
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32,
    use_hex: bool,
    dest: &mut PrinterT,
) -> fmt::Result {
    if alpha == 0.0 {
        return dest.write_str("#0000");
    }
    if !use_hex {
        dest.write_str("rgba(")?;
        serialize_int(red, dest)?;
        dest.write_char(',')?;
        serialize_int(green, dest)?;
        dest.write_char(',')?;
        serialize_int(blue, dest)?;
        dest.write_char(',')?;
        serialize_number(alpha, dest)?;
        return dest.write_char(')');
    }
    let alpha = (alpha * 255.0).round() as u8;
    let rgba = u32::from_be_bytes([red, green, blue, alpha]);
    dest.write_char('#')?;
    let values = [red, green, blue, alpha];
    if values.iter().all(|value| value >> 4 == value & 15) {
        let rgba = ((rgba >> 12) & 0xf000)
            | ((rgba >> 8) & 0x0f00)
            | ((rgba >> 4) & 0x00f0)
            | (rgba & 0x000f);
        serialize_hex(rgba, 4, false, dest)
    } else {
        serialize_hex(rgba, 8, false, dest)
    }
}

fn write_minified_rgb<PrinterT: PrinterTrait>(
    red: u8,
    green: u8,
    blue: u8,
    dest: &mut PrinterT,
) -> fmt::Result {
    if (red, green, blue) == (255, 0, 0) {
        return dest.write_str("red");
    }
    let rgb = u32::from_be_bytes([0, red, green, blue]);
    dest.write_char('#')?;
    if red >> 4 == red & 15 && green >> 4 == green & 15 && blue >> 4 == blue & 15 {
        let rgb = ((rgb >> 12) & 0x0f00) | ((rgb >> 8) & 0x00f0) | ((rgb >> 4) & 0x000f);
        serialize_hex(rgb, 3, false, dest)
    } else {
        serialize_hex(rgb, 6, false, dest)
    }
}

impl<'ghost> ToCss<'ghost> for Variable<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("var(")?;
        self.name.to_css(dest, _cx)?;
        if let Some(fallback) = &self.fallback {
            write_variable_fallback(*fallback, dest, _cx)?;
        }
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for EnvironmentVariable<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let value = cx.ast_context().environment_variable(id);
        write_environment_variable(
            value.name(),
            || value.indices(),
            || value.fallback(),
            dest,
            cx,
        )
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_environment_variable(self.name, || self.indices, || self.fallback, dest, cx)
    }
}

fn write_environment_variable<'ast, 'ghost, PrinterT: PrinterTrait>(
    name: EnvironmentVariableName<'ast>,
    indices: impl FnOnce() -> AstVec<'ast, i32>,
    fallback: impl FnOnce() -> Option<AstVec<'ast, TokenOrValue<'ast>>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("env(")?;
    name.to_css(dest, cx)?;
    for index in cx.ast_context().vec_iter(indices()) {
        dest.write_char(' ')?;
        serialize_int(index, dest)?;
    }
    if let Some(fallback) = fallback() {
        write_variable_fallback(fallback, dest, cx)?;
    }
    dest.write_char(')')
}

impl<'ghost> ToCss<'ghost> for DashedIdentReference<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_dashed_ident(_cx.ast_context().str(self.ident), dest)?;
        if let Some(from) = &self.from {
            dest.write_str(" from ")?;
            from.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

fn write_variable_fallback<PrinterT: PrinterTrait>(
    range: AstVec<'_, TokenOrValue<'_>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_>,
) -> fmt::Result {
    let ast = cx.ast_context();
    let mut values = ast.vec_iter(range).peekable();
    dest.write_char(',')?;
    let Some(first) = values.peek() else {
        return dest.write_char(' ');
    };
    if !matches!(first, TokenOrValue::Token(token) if matches!(ast.resolve_node(*token), Token::WhiteSpace(_)))
    {
        dest.whitespace()?;
    }
    write_token_list(values, dest, cx)?;
    if crate::token::token_list_ends_with_comma(range, cx) {
        dest.write_char(' ')?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for ImportRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@import ")?;
        serialize_string(_cx.ast_context().str(self.url), dest)?;
        if let Some(layer) = &self.layer {
            dest.write_str(" layer")?;
            if !layer.is_empty() {
                dest.write_char('(')?;
                write_layer_name(
                    _cx.ast_context()
                        .vec_iter(*layer)
                        .map(|part| _cx.ast_context().str(part)),
                    dest,
                )?;
                dest.write_char(')')?;
            }
        }
        if let Some(supports) = &self.supports {
            dest.write_str(" supports(")?;
            let serialized = supports.to_css_string(dest.options(), _cx)?;
            dest.write_str(
                serialized
                    .strip_prefix('(')
                    .and_then(|value| value.strip_suffix(')'))
                    .unwrap_or(&serialized),
            )?;
            dest.write_char(')')?;
        }
        if let Some(media) = &self.media {
            dest.write_char(' ')?;
            media.to_css(dest, _cx)?;
        }
        dest.write_char(';')
    }
}

fn write_layer_name<PrinterT, I>(name: I, dest: &mut PrinterT) -> fmt::Result
where
    PrinterT: PrinterTrait,
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for (index, part) in name.into_iter().enumerate() {
        if index > 0 {
            dest.write_char('.')?;
        }
        serialize_identifier(part.as_ref(), dest)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for LengthValue {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.value == 0.0 && !dest.in_calc() {
            return dest.write_char('0');
        }
        serialize_dimension(self.value, &self.unit, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for MediaList<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.media_queries.is_empty() {
            return dest.write_str("not all");
        }
        for (index, query) in _cx.ast_context().vec_iter(self.media_queries).enumerate() {
            if index > 0 {
                dest.delim(Delimiter::Comma)?;
            }
            query.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for MediaQuery<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        if let Some(condition) = self.condition
            && let MediaCondition::Unknown(tokens) = ast.resolve_node(condition)
        {
            let mut tokens = ast.vec_iter(tokens).skip_while(|value| {
                matches!(value, TokenOrValue::Token(token) if matches!(ast.resolve_node(*token), Token::WhiteSpace(_)))
            }).peekable();
            if matches!(self.qualifier, Some(Qualifier::Not))
                && matches!(self.media_type, MediaType::All)
                && matches!(
                    tokens.peek(),
                    Some(TokenOrValue::Token(token)) if matches!(ast.resolve_node(*token), Token::ParenthesisBlock)
                )
            {
                dest.write_str("not ")?;
                return crate::token::write_token_list(tokens, dest, _cx);
            }

            if let Some(qualifier) = &self.qualifier {
                qualifier.to_css(dest, _cx)?;
                dest.write_char(' ')?;
            }
            let wrote_type = !matches!(self.media_type, MediaType::All);
            if wrote_type || self.qualifier.is_some() {
                self.media_type.to_css(dest, _cx)?;
                dest.write_char(' ')?;
            }
            return crate::token::write_token_list(tokens, dest, _cx);
        }

        if let Some(qualifier) = &self.qualifier {
            qualifier.to_css(dest, _cx)?;
            dest.write_char(' ')?;
        }

        let has_type = !matches!(self.media_type, MediaType::All);
        match &self.media_type {
            MediaType::All if self.qualifier.is_some() || self.condition.is_none() => {
                dest.write_str("all")?
            }
            MediaType::All => {}
            value => value.to_css(dest, _cx)?,
        }

        if let Some(condition) = self.condition {
            let condition = ast.resolve_node(condition);
            if has_type || self.qualifier.is_some() {
                dest.write_str(" and ")?;
            }
            let needs_parens = (has_type || self.qualifier.is_some())
                && matches!(
                    condition,
                    MediaCondition::Operation {
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
