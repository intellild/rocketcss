use self::stylesheet::{check_depth, recover_declaration};
use super::{
    selector::parse_selector_list,
    values::{
        collect_tokens, matches_ignore_case, remove_important, token_ident, trim_leading_whitespace,
    },
};
use crate::prelude::*;
use rocketcss_ast::PropertyRuleDescriptor;

pub(super) fn parse_single_ident<'i>(
    prelude: &'i str,
    input: &mut Compiler<'i>,
) -> Result<AstStr<'i>, ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |input| {
        let name = input.expect_ident()?;
        input.expect_exhausted()?;
        Ok(input.add_str(name))
    })
}

pub(super) fn at_rule_vendor_prefix(name: &str) -> VendorPrefix {
    if name
        .get(..8)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-webkit-"))
    {
        VendorPrefix::WEBKIT
    } else if name
        .get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-moz-"))
    {
        VendorPrefix::MOZ
    } else if name
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-ms-"))
    {
        VendorPrefix::MS
    } else if name
        .get(..3)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-o-"))
    {
        VendorPrefix::O
    } else {
        VendorPrefix::NONE
    }
}

pub(super) mod at_rule;
pub(super) mod container;
pub(super) mod font;
pub(super) mod keyframes;
pub(super) mod page;
pub(super) mod property;
pub(super) mod stylesheet;
pub(super) mod view_transition;

pub(super) mod background;
pub(super) mod layout;

pub(super) mod border;

pub(super) mod shape;
pub(super) mod text;
pub(super) mod transform;

pub(super) mod animation;
