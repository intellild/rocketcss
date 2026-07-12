#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, VisitNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_token_or_value<'a, VisitorT>(visitor: &mut VisitorT, node: &TokenOrValue<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TokenOrValue);
    match node {
        TokenOrValue::Token(field_0) => {
            visitor.visit_token((field_0).as_ref());
        }
        TokenOrValue::Color(field_0) => {
            visitor.visit_css_color((field_0).as_ref());
        }
        TokenOrValue::UnresolvedColor(field_0) => {
            visitor.visit_unresolved_color((field_0).as_ref());
        }
        TokenOrValue::Url(field_0) => {
            visitor.visit_url((field_0).as_ref());
        }
        TokenOrValue::Var(field_0) => {
            visitor.visit_variable((field_0).as_ref());
        }
        TokenOrValue::Env(field_0) => {
            visitor.visit_environment_variable((field_0).as_ref());
        }
        TokenOrValue::Function(field_0) => {
            visitor.visit_function((field_0).as_ref());
        }
        TokenOrValue::Length(field_0) => {
            visitor.visit_length_value((field_0).as_ref());
        }
        TokenOrValue::Angle(field_0) => {
            visitor.visit_angle((field_0).as_ref());
        }
        TokenOrValue::Time(field_0) => {
            visitor.visit_time((field_0).as_ref());
        }
        TokenOrValue::Resolution(field_0) => {
            visitor.visit_resolution((field_0).as_ref());
        }
        TokenOrValue::DashedIdent(field_0) => {
            visitor.visit_str(field_0);
        }
        TokenOrValue::AnimationName(field_0) => {
            visitor.visit_animation_name((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::TokenOrValue);
}
pub fn walk_unit<'a, VisitorT>(visitor: &mut VisitorT, node: &Unit)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Unit);
    match node {
        Unit::Length(field_0) => {
            visitor.visit_length_unit(field_0);
        }
        Unit::Deg => {}
        Unit::Rad => {}
        Unit::Grad => {}
        Unit::Turn => {}
        Unit::Seconds => {}
        Unit::Milliseconds => {}
        Unit::Hertz => {}
        Unit::Kilohertz => {}
        Unit::Dpi => {}
        Unit::Dpcm => {}
        Unit::Dppx => {}
        Unit::ResolutionX => {}
        Unit::Flex => {}
    }
    visitor.leave_node(AstType::Unit);
}
pub fn walk_token<'a, VisitorT>(visitor: &mut VisitorT, node: &Token<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Token);
    match node {
        Token::Ident(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::AtKeyword(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::Hash(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::IdHash(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::MinifiedHash(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::String(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::UnquotedFont(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::UnquotedUrl(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::Delim(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::Number(field_0) => {}
        Token::Percentage(field_0) => {}
        Token::Dimension { unit, value } => {
            visitor.visit_unit(unit);
        }
        Token::UnknownDimension { unit, value } => {
            visitor.visit_str(unit);
        }
        Token::WhiteSpace(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::Comment(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::Colon => {}
        Token::Semicolon => {}
        Token::Comma => {}
        Token::IncludeMatch => {}
        Token::DashMatch => {}
        Token::PrefixMatch => {}
        Token::SuffixMatch => {}
        Token::SubstringMatch => {}
        Token::Cdo => {}
        Token::Cdc => {}
        Token::Function(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::ParenthesisBlock => {}
        Token::SquareBracketBlock => {}
        Token::CurlyBracketBlock => {}
        Token::BadUrl(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::BadString(field_0) => {
            visitor.visit_str(field_0);
        }
        Token::CloseParenthesis => {}
        Token::CloseSquareBracket => {}
        Token::CloseCurlyBracket => {}
    }
    visitor.leave_node(AstType::Token);
}
pub fn walk_specifier<'a, VisitorT>(visitor: &mut VisitorT, node: &Specifier<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Specifier);
    match node {
        Specifier::Global => {}
        Specifier::File(field_0) => {
            visitor.visit_str(field_0);
        }
        Specifier::SourceIndex(field_0) => {}
    }
    visitor.leave_node(AstType::Specifier);
}
pub fn walk_animation_name<'a, VisitorT>(visitor: &mut VisitorT, node: &AnimationName<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::AnimationName);
    match node {
        AnimationName::None => {}
        AnimationName::Ident(field_0) => {
            visitor.visit_str(field_0);
        }
        AnimationName::String(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::AnimationName);
}
pub fn walk_environment_variable_name<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &EnvironmentVariableName<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::EnvironmentVariableName);
    match node {
        EnvironmentVariableName::UA(field_0) => {
            visitor.visit_ua_environment_variable(field_0);
        }
        EnvironmentVariableName::Custom(field_0) => {
            visitor.visit_dashed_ident_reference((field_0).as_ref());
        }
        EnvironmentVariableName::Unknown(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::EnvironmentVariableName);
}
pub fn walk_ua_environment_variable<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &UAEnvironmentVariable,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::UAEnvironmentVariable);
    match node {
        UAEnvironmentVariable::SafeAreaInsetTop => {}
        UAEnvironmentVariable::SafeAreaInsetRight => {}
        UAEnvironmentVariable::SafeAreaInsetBottom => {}
        UAEnvironmentVariable::SafeAreaInsetLeft => {}
        UAEnvironmentVariable::ViewportSegmentWidth => {}
        UAEnvironmentVariable::ViewportSegmentHeight => {}
        UAEnvironmentVariable::ViewportSegmentTop => {}
        UAEnvironmentVariable::ViewportSegmentLeft => {}
        UAEnvironmentVariable::ViewportSegmentBottom => {}
        UAEnvironmentVariable::ViewportSegmentRight => {}
    }
    visitor.leave_node(AstType::UAEnvironmentVariable);
}
