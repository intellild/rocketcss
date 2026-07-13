use rocketcss_ast::{MediaCondition, MediaList, MediaType, SupportsCondition, Token, TokenOrValue};

use crate::{Minify, MinifyContext, Options};

impl Minify for MediaList<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        for query in &mut self.media_queries {
            if let Some(condition) = &mut query.condition
                && let MediaCondition::Unknown(tokens) = &mut **condition
            {
                tokens.minify(context);
                minify_ratios(tokens, context);
                if context
                    .options()
                    .is_enabled(Options::NORMALIZE_MEDIA_QUERIES)
                    && matches!(*query.media_type, MediaType::All)
                    && query.qualifier.is_none()
                    && matches!(tokens.first(), Some(TokenOrValue::Token(token))
                        if matches!(&**token, Token::Ident(value) if value.eq_ignore_ascii_case("and")))
                {
                    tokens.remove(0);
                    if matches!(tokens.first(), Some(TokenOrValue::Token(token))
                        if matches!(**token, Token::WhiteSpace(_)))
                    {
                        tokens.remove(0);
                    }
                    context.record_value_normalized();
                }
            }
        }
        if context
            .options()
            .is_enabled(Options::NORMALIZE_MEDIA_QUERIES)
            && self.media_queries.len() == 1
            && matches!(*self.media_queries[0].media_type, MediaType::All)
            && self.media_queries[0].qualifier.is_none()
            && self.media_queries[0].condition.is_none()
        {
            self.media_queries.clear();
            context.record_value_normalized();
        }
        if context.options().is_enabled(Options::DEDUPLICATE_LISTS) {
            let before = self.media_queries.len();
            let mut index = 0;
            while index < self.media_queries.len() {
                if self.media_queries[..index]
                    .iter()
                    .any(|query| query == &self.media_queries[index])
                {
                    self.media_queries.remove(index);
                } else {
                    index += 1;
                }
            }
            if self.media_queries.len() != before {
                context.record_value_normalized();
            }
        }
    }
}

impl Minify for SupportsCondition<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        match self {
            Self::Declaration { value, .. } => {
                let normalized = value.trim();
                if normalized.len() != value.len() {
                    *value = normalized;
                    context.record_value_normalized();
                }
            }
            Self::Unknown(value)
                if value
                    .split_once(':')
                    .is_some_and(|(_, value)| value.starts_with(char::is_whitespace)) =>
            {
                *self = Self::MinifiedUnknown(value);
                context.record_value_normalized();
            }
            _ => {}
        }
    }
}

fn minify_ratios(
    values: &mut rocketcss_allocator::vec::Vec<'_, TokenOrValue<'_>>,
    context: &mut MinifyContext,
) {
    if !context.options().is_enabled(Options::NORMALIZE_VALUES) || values.len() < 5 {
        return;
    }
    for index in 2..values.len() - 2 {
        let is_aspect_ratio = matches!(&values[index - 2], TokenOrValue::Token(token)
            if matches!(&**token, Token::Ident(name)
                if ["aspect-ratio", "min-aspect-ratio", "max-aspect-ratio"]
                    .iter()
                    .any(|candidate| name.eq_ignore_ascii_case(candidate))))
            && matches!(&values[index - 1], TokenOrValue::Token(token)
                if matches!(**token, Token::Colon));
        if !is_aspect_ratio
            || !matches!(&values[index + 1], TokenOrValue::Token(token)
                if matches!(&**token, Token::Delim("/")))
        {
            continue;
        }
        let (left, right) = match (&values[index], &values[index + 2]) {
            (TokenOrValue::Token(left), TokenOrValue::Token(right)) => match (&**left, &**right) {
                (Token::Number(left), Token::Number(right)) if *left > 0.0 && *right > 0.0 => {
                    (*left, *right)
                }
                _ => continue,
            },
            _ => continue,
        };
        let mut scale = 1_u64;
        while scale < 1_000_000
            && (!is_near_integer(left * scale as f32) || !is_near_integer(right * scale as f32))
        {
            scale *= 10;
        }
        let left_integer = (left * scale as f32).round() as u64;
        let right_integer = (right * scale as f32).round() as u64;
        let divisor = gcd(left_integer, right_integer);
        let reduced_left = (left_integer / divisor) as f32;
        let reduced_right = (right_integer / divisor) as f32;
        if reduced_left == left && reduced_right == right {
            continue;
        }
        let TokenOrValue::Token(left_token) = &mut values[index] else {
            unreachable!()
        };
        **left_token = Token::Number(reduced_left);
        let TokenOrValue::Token(right_token) = &mut values[index + 2] else {
            unreachable!()
        };
        **right_token = Token::Number(reduced_right);
        context.record_value_normalized();
    }
}

fn is_near_integer(value: f32) -> bool {
    (value - value.round()).abs() <= f32::EPSILON * value.abs().max(1.0)
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}
