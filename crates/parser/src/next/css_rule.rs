//! Single-pass rule body scanning for nested-rule disambiguation.

use super::{prelude::*, properties::CssWideValueHint};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuleBodyDelimiter {
    Semicolon,
    CurlyBracket,
}

pub(crate) struct RuleBodyScan<'i> {
    pub(crate) delimiter: Option<RuleBodyDelimiter>,
    css_wide_candidate: Option<&'i str>,
}

impl<'i> RuleBodyScan<'i> {
    pub(crate) fn css_wide_hint(&self) -> CssWideValueHint<'i> {
        self.css_wide_candidate
            .map_or(CssWideValueHint::NotCssWide, CssWideValueHint::Candidate)
    }
}

fn drain_rule_body(input: &mut Compiler<'_>) {
    while input.next().is_ok() {}
}

fn scan_single_ident_value<'i>(input: &mut Compiler<'i>) -> Option<&'i str> {
    let ident = match input.next() {
        Ok(OldToken::Ident(value)) => *value,
        Ok(_) => {
            drain_rule_body(input);
            return None;
        }
        Err(_) => return None,
    };

    match input.next() {
        Err(_) => Some(ident),
        Ok(OldToken::Delim("!")) => {
            let is_important = matches!(
                input.next(),
                Ok(OldToken::Ident(value)) if value.eq_ignore_ascii_case("important")
            );
            let is_exhausted = input.next().is_err();
            if is_important && is_exhausted {
                Some(ident)
            } else {
                drain_rule_body(input);
                None
            }
        }
        Ok(_) => {
            drain_rule_body(input);
            None
        }
    }
}

pub(crate) fn scan_rule_body<'i>(
    input: &mut Compiler<'i>,
    scan_css_wide: bool,
) -> RuleBodyScan<'i> {
    let state = input.state();
    let mut css_wide_candidate = None;
    let _: Result<(), ParseError<'_, ()>> = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| {
            if scan_css_wide {
                css_wide_candidate = scan_single_ident_value(input);
            } else {
                drain_rule_body(input);
            }
            Ok(())
        },
    );
    let delimiter = match input.next() {
        Ok(OldToken::Semicolon) => Some(RuleBodyDelimiter::Semicolon),
        Ok(OldToken::CurlyBracketBlock) => Some(RuleBodyDelimiter::CurlyBracket),
        _ => None,
    };
    let css_wide_candidate = css_wide_candidate.filter(|_| !input.saw_comments_since(&state));
    input.reset(&state);
    RuleBodyScan {
        delimiter,
        css_wide_candidate,
    }
}
