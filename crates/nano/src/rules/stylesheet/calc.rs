use super::*;

pub(super) fn simple_calc_value(values: &[TokenOrValue<'_>]) -> Option<FunctionReplacement> {
    let mut values = values.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
    });
    let left = calc_value(values.next()?)?;
    let Some(operator) = values.next() else {
        return Some(unitless_calc_zero(left));
    };
    let right = calc_value(values.next()?)?;
    if values.next().is_some() {
        return None;
    }
    let TokenOrValue::Token(operator) = operator else {
        return None;
    };
    let Token::Delim(operator) = &**operator else {
        return None;
    };
    calculate_values(left, operator, right).map(unitless_calc_zero)
}

const MAX_CALC_TERMS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CalcTermKind {
    Number,
    Percentage,
    Dimension(Unit),
}

#[derive(Clone, Copy, Debug)]
struct CalcTerm {
    explicit_zero: bool,
    kind: CalcTermKind,
    value: f32,
}

impl CalcTerm {
    const EMPTY: Self = Self {
        explicit_zero: false,
        kind: CalcTermKind::Number,
        value: 0.0,
    };
}

#[derive(Clone, Copy, Debug)]
pub(super) struct CalcLinear {
    terms: [CalcTerm; MAX_CALC_TERMS],
    len: usize,
}

impl CalcLinear {
    const fn empty() -> Self {
        Self {
            terms: [CalcTerm::EMPTY; MAX_CALC_TERMS],
            len: 0,
        }
    }

    fn from_value(value: FunctionReplacement) -> Option<Self> {
        let (kind, value) = match value {
            FunctionReplacement::Number(value) => (CalcTermKind::Number, value),
            FunctionReplacement::Percentage(value) => (CalcTermKind::Percentage, value),
            FunctionReplacement::Dimension { unit, value } => {
                (CalcTermKind::Dimension(unit), value)
            }
            _ => return None,
        };
        let mut result = Self::empty();
        result.terms[0] = CalcTerm {
            explicit_zero: value == 0.0,
            kind,
            value,
        };
        result.len = 1;
        Some(result)
    }

    fn add(mut self, right: Self, sign: f32) -> Option<Self> {
        for right in right.terms[..right.len].iter().copied() {
            if let Some(left) = self.terms[..self.len]
                .iter_mut()
                .find(|left| left.kind == right.kind)
            {
                left.value += right.value * sign;
                left.explicit_zero &= right.explicit_zero;
                continue;
            }
            if self.len == MAX_CALC_TERMS {
                return None;
            }
            self.terms[self.len] = CalcTerm {
                explicit_zero: right.explicit_zero,
                kind: right.kind,
                value: right.value * sign,
            };
            self.len += 1;
        }
        Some(self)
    }

    fn scale(mut self, factor: f32) -> Self {
        for term in &mut self.terms[..self.len] {
            term.value *= factor;
        }
        self
    }

    pub(super) fn round(mut self, precision: Option<u8>) -> Self {
        let Some(precision) = precision else {
            return self;
        };
        let factor = 10_f64.powi(i32::from(precision));
        for term in &mut self.terms[..self.len] {
            term.value = ((f64::from(term.value) * factor).round() / factor) as f32;
        }
        self
    }

    fn scalar(self) -> Option<f32> {
        (self.len == 1 && self.terms[0].kind == CalcTermKind::Number).then_some(self.terms[0].value)
    }

    fn compact_cancelled_terms(&mut self) {
        let mut target = 0;
        for source in 0..self.len {
            let term = self.terms[source];
            if term.value == 0.0 && !term.explicit_zero {
                continue;
            }
            self.terms[target] = term;
            target += 1;
        }
        self.len = target;
    }

    fn replacement(self) -> Option<FunctionReplacement> {
        if self.len == 0 {
            return Some(FunctionReplacement::Number(0.0));
        }
        if self.len != 1 {
            return None;
        }
        Some(match self.terms[0] {
            CalcTerm {
                kind: CalcTermKind::Number,
                value,
                ..
            } => FunctionReplacement::Number(value),
            CalcTerm {
                kind: CalcTermKind::Percentage,
                value,
                ..
            } => FunctionReplacement::Percentage(value),
            CalcTerm {
                kind: CalcTermKind::Dimension(unit),
                value,
                ..
            } => FunctionReplacement::Dimension { unit, value },
        })
    }

    pub(super) fn write_to(mut self, function: &mut Function<'_>) -> bool {
        self.compact_cancelled_terms();
        if let Some(replacement) = self.replacement() {
            function.replacement = Some(unitless_calc_zero(replacement));
            function.arguments.clear();
            return true;
        }

        let required = 1 + (self.len - 1) * 4;
        if function
            .arguments
            .iter()
            .filter(|value| matches!(value, TokenOrValue::Token(_)))
            .count()
            < required
        {
            return false;
        }
        for target in 0..required {
            if matches!(function.arguments[target], TokenOrValue::Token(_)) {
                continue;
            }
            let Some(source) = function.arguments[target + 1..]
                .iter()
                .position(|value| matches!(value, TokenOrValue::Token(_)))
                .map(|source| target + 1 + source)
            else {
                return false;
            };
            function.arguments.swap(target, source);
        }

        let mut output = 0;
        for (index, term) in self.terms[..self.len].iter().copied().enumerate() {
            if index != 0 {
                set_calc_token(&mut function.arguments[output], Token::WhiteSpace(" "));
                output += 1;
                set_calc_token(
                    &mut function.arguments[output],
                    Token::Delim(if term.value < 0.0 { "-" } else { "+" }),
                );
                output += 1;
                set_calc_token(&mut function.arguments[output], Token::WhiteSpace(" "));
                output += 1;
            }
            let value = if index == 0 {
                term.value
            } else {
                term.value.abs()
            };
            let token = match term.kind {
                CalcTermKind::Number => Token::Number(value),
                CalcTermKind::Percentage => Token::Percentage(value),
                CalcTermKind::Dimension(unit) => Token::Dimension { unit, value },
            };
            set_calc_token(&mut function.arguments[output], token);
            output += 1;
        }
        function.arguments.truncate(required);
        true
    }
}

fn set_calc_token<'a>(value: &mut TokenOrValue<'a>, token_value: Token<'a>) {
    let TokenOrValue::Token(token) = value else {
        unreachable!("calc output slots were normalized to tokens")
    };
    **token = token_value;
}

pub(super) fn calc_linear_expression(values: &[TokenOrValue<'_>]) -> Option<CalcLinear> {
    let mut parser = CalcLinearParser { index: 0, values };
    let mut result = parser.expression(false)?;
    parser.skip_whitespace();
    if parser.index != values.len() {
        return None;
    }
    result.compact_cancelled_terms();
    Some(result)
}

struct CalcLinearParser<'values, 'arena> {
    index: usize,
    values: &'values [TokenOrValue<'arena>],
}

impl CalcLinearParser<'_, '_> {
    fn expression(&mut self, nested: bool) -> Option<CalcLinear> {
        let mut result = self.term()?;
        loop {
            self.skip_whitespace();
            if nested && self.is_close_parenthesis() {
                break;
            }
            let Some(operator) = self.operator(&["+", "-"]) else {
                break;
            };
            let right = self.term()?;
            result = result.add(right, if operator == "+" { 1.0 } else { -1.0 })?;
        }
        Some(result)
    }

    fn term(&mut self) -> Option<CalcLinear> {
        let mut result = self.factor()?;
        loop {
            self.skip_whitespace();
            let Some(operator) = self.operator(&["*", "/"]) else {
                break;
            };
            let right = self.factor()?;
            result = match operator {
                "*" => {
                    if let Some(scalar) = result.scalar() {
                        right.scale(scalar)
                    } else {
                        result.scale(right.scalar()?)
                    }
                }
                "/" => {
                    let divisor = right.scalar()?;
                    if divisor == 0.0 {
                        return None;
                    }
                    result.scale(1.0 / divisor)
                }
                _ => unreachable!(),
            };
        }
        Some(result)
    }

    fn factor(&mut self) -> Option<CalcLinear> {
        self.skip_whitespace();
        let mut sign = 1.0;
        while let Some(operator) = self.operator(&["+", "-"]) {
            if operator == "-" {
                sign = -sign;
            }
            self.skip_whitespace();
        }
        let value = self.values.get(self.index)?;
        let mut result = match value {
            TokenOrValue::Token(token) if matches!(**token, Token::ParenthesisBlock) => {
                self.index += 1;
                let result = self.expression(true)?;
                self.skip_whitespace();
                if !self.is_close_parenthesis() {
                    return None;
                }
                self.index += 1;
                result
            }
            TokenOrValue::Function(function)
                if function.kind() == KnownFunction::Calc && !function.is_vendor_prefixed() =>
            {
                self.index += 1;
                if let Some(replacement) = function.replacement {
                    CalcLinear::from_value(replacement)?
                } else {
                    calc_linear_expression(&function.arguments)?
                }
            }
            value => {
                self.index += 1;
                CalcLinear::from_value(calc_value(value)?)?
            }
        };
        if sign < 0.0 {
            result = result.scale(-1.0);
        }
        Some(result)
    }

    fn operator<'operator>(
        &mut self,
        allowed: &'operator [&'operator str],
    ) -> Option<&'operator str> {
        let TokenOrValue::Token(token) = self.values.get(self.index)? else {
            return None;
        };
        let Token::Delim(operator) = &**token else {
            return None;
        };
        let operator = allowed
            .iter()
            .copied()
            .find(|allowed| operator == allowed)?;
        self.index += 1;
        Some(operator)
    }

    fn is_close_parenthesis(&self) -> bool {
        matches!(self.values.get(self.index), Some(TokenOrValue::Token(token)) if matches!(**token, Token::CloseParenthesis))
    }

    fn skip_whitespace(&mut self) {
        while self.values.get(self.index).is_some_and(is_calc_whitespace) {
            self.index += 1;
        }
    }
}

fn unitless_calc_zero(value: FunctionReplacement) -> FunctionReplacement {
    match value {
        FunctionReplacement::Dimension { value: 0.0, .. }
        | FunctionReplacement::Percentage(0.0) => FunctionReplacement::Number(0.0),
        value => value,
    }
}

pub(super) fn minify_flat_calc_operations(values: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let mut changed = false;
    loop {
        let mut reduced = false;
        for operator_index in 0..values.len() {
            let TokenOrValue::Token(operator) = &values[operator_index] else {
                continue;
            };
            let Token::Delim(operator) = &**operator else {
                continue;
            };
            if !matches!(*operator, "*" | "/") {
                continue;
            }
            let Some(left_index) = values[..operator_index]
                .iter()
                .rposition(|value| !is_calc_whitespace(value))
            else {
                continue;
            };
            let Some(right_index) = values[operator_index + 1..]
                .iter()
                .position(|value| !is_calc_whitespace(value))
                .map(|index| operator_index + 1 + index)
            else {
                continue;
            };
            let Some(result) = calc_value(&values[left_index])
                .zip(calc_value(&values[right_index]))
                .and_then(|(left, right)| calculate_values(left, operator, right))
            else {
                continue;
            };
            if !set_calc_value(&mut values[left_index], result) {
                continue;
            }
            values.drain(left_index + 1..=right_index);
            reduced = true;
            changed = true;
            break;
        }
        if !reduced {
            break;
        }
    }

    loop {
        let mut reduced = false;
        for operator_index in 0..values.len() {
            let TokenOrValue::Token(operator) = &values[operator_index] else {
                continue;
            };
            let Token::Delim(operator) = &**operator else {
                continue;
            };
            if !matches!(*operator, "+" | "-") {
                continue;
            }
            let Some(left_index) = values[..operator_index]
                .iter()
                .rposition(|value| !is_calc_whitespace(value))
            else {
                continue;
            };
            let Some(right_index) = values[operator_index + 1..]
                .iter()
                .position(|value| !is_calc_whitespace(value))
                .map(|index| operator_index + 1 + index)
            else {
                continue;
            };
            if !calc_value(&values[right_index]).is_some_and(calc_value_is_zero) {
                continue;
            }
            values.drain(left_index + 1..=right_index);
            reduced = true;
            changed = true;
            break;
        }
        if !reduced {
            return changed;
        }
    }
}

fn calc_value_is_zero(value: FunctionReplacement) -> bool {
    matches!(
        value,
        FunctionReplacement::Number(0.0)
            | FunctionReplacement::Dimension { value: 0.0, .. }
            | FunctionReplacement::Percentage(0.0)
    )
}

fn is_calc_whitespace(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_) | Token::Comment(_)))
}

fn set_calc_value(value: &mut TokenOrValue<'_>, result: FunctionReplacement) -> bool {
    match value {
        TokenOrValue::Token(token) => {
            **token = match result {
                FunctionReplacement::Number(value) => Token::Number(value),
                FunctionReplacement::Dimension { unit, value } => Token::Dimension { unit, value },
                FunctionReplacement::Percentage(value) => Token::Percentage(value),
                _ => return false,
            };
            true
        }
        TokenOrValue::Function(function) => {
            function.arguments.clear();
            function.replacement = Some(result);
            true
        }
        TokenOrValue::Length(value) => match result {
            FunctionReplacement::Dimension {
                unit: Unit::Length(unit),
                value: result,
            } if unit == value.unit => {
                value.value = result;
                true
            }
            FunctionReplacement::Number(0.0) => {
                value.value = 0.0;
                true
            }
            _ => false,
        },
        _ => false,
    }
}

pub(super) fn remove_redundant_calc_parentheses(values: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let Some(open) = values.iter().position(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::ParenthesisBlock))
    }) else {
        return false;
    };
    let Some(close) = values[open + 1..]
        .iter()
        .position(|value| {
            matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::CloseParenthesis))
        })
        .map(|index| open + 1 + index)
    else {
        return false;
    };
    let previous = values[..open]
        .iter()
        .rev()
        .find(|value| !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_))));
    let preceded_by_addition = previous.is_none_or(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::Delim("+") | Token::Delim("-")))
    });
    let contains_addition = values[open + 1..close].iter().any(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::Delim("+") | Token::Delim("-")))
    });
    if !preceded_by_addition || contains_addition {
        return false;
    }
    let _ = values.remove(close);
    let _ = values.remove(open);
    true
}

fn calc_value(value: &TokenOrValue<'_>) -> Option<FunctionReplacement> {
    match value {
        TokenOrValue::Token(token) => match **token {
            Token::Number(value) => Some(FunctionReplacement::Number(value)),
            Token::Dimension { unit, value } => {
                Some(FunctionReplacement::Dimension { unit, value })
            }
            Token::Percentage(value) => Some(FunctionReplacement::Percentage(value)),
            _ => None,
        },
        TokenOrValue::Length(value) => Some(FunctionReplacement::Dimension {
            unit: Unit::Length(value.unit),
            value: value.value,
        }),
        TokenOrValue::Function(function) => function.replacement,
        _ => None,
    }
}

fn calculate_values(
    left: FunctionReplacement,
    operator: &str,
    right: FunctionReplacement,
) -> Option<FunctionReplacement> {
    use FunctionReplacement::{Dimension, Number, Percentage};
    match (left, operator, right) {
        (Number(left), "+", Number(right)) => Some(Number(left + right)),
        (Number(left), "-", Number(right)) => Some(Number(left - right)),
        (Number(left), "*", Number(right)) => Some(Number(left * right)),
        (Number(left), "/", Number(right)) if right != 0.0 => Some(Number(left / right)),
        (
            Dimension {
                unit: left_unit,
                value: left,
            },
            "+",
            Dimension {
                unit: right_unit,
                value: right,
            },
        ) if left_unit == right_unit => Some(Dimension {
            unit: left_unit,
            value: left + right,
        }),
        (
            Dimension {
                unit: left_unit,
                value: left,
            },
            "-",
            Dimension {
                unit: right_unit,
                value: right,
            },
        ) if left_unit == right_unit => Some(Dimension {
            unit: left_unit,
            value: left - right,
        }),
        (Dimension { unit, value }, "*", Number(number))
        | (Number(number), "*", Dimension { unit, value }) => Some(Dimension {
            unit,
            value: value * number,
        }),
        (Dimension { unit, value }, "/", Number(number)) if number != 0.0 => Some(Dimension {
            unit,
            value: value / number,
        }),
        (Percentage(left), "+", Percentage(right)) => Some(Percentage(left + right)),
        (Percentage(left), "-", Percentage(right)) => Some(Percentage(left - right)),
        (Percentage(value), "*", Number(number)) | (Number(number), "*", Percentage(value)) => {
            Some(Percentage(value * number))
        }
        (Percentage(value), "/", Number(number)) if number != 0.0 => {
            Some(Percentage(value / number))
        }
        _ => None,
    }
}
