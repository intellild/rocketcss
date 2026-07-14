#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for TokenOrValue<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_token_or_value(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TokenOrValue);
        let node = self;
        match node {
            TokenOrValue::Token(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Color(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::UnresolvedColor(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Url(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Var(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Env(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Function(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Length(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Angle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Time(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::Resolution(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TokenOrValue::DashedIdent(field_0) => {
                visitor.visit_str(field_0);
            }
            TokenOrValue::AnimationName(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::TokenOrValue);
    }
}
impl<'a> Visit<'a> for Unit {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_unit(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Unit);
        let node = self;
        match node {
            Unit::Length(field_0) => {
                Visit::visit(field_0, visitor);
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
}
impl<'a> Visit<'a> for Token<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_token(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Token);
        let node = self;
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
                Visit::visit(unit, visitor);
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
}
impl<'a> Visit<'a> for Specifier<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_specifier(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Specifier);
        let node = self;
        match node {
            Specifier::Global => {}
            Specifier::File(field_0) => {
                visitor.visit_str(field_0);
            }
            Specifier::SourceIndex(field_0) => {}
        }
        visitor.leave_node(AstType::Specifier);
    }
}
impl<'a> Visit<'a> for AnimationName<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationName);
        let node = self;
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
}
impl<'a> Visit<'a> for EnvironmentVariableName<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_environment_variable_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::EnvironmentVariableName);
        let node = self;
        match node {
            EnvironmentVariableName::UA(field_0) => {
                Visit::visit(field_0, visitor);
            }
            EnvironmentVariableName::Custom(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            EnvironmentVariableName::Unknown(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::EnvironmentVariableName);
    }
}
impl<'a> Visit<'a> for UAEnvironmentVariable {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_ua_environment_variable(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UAEnvironmentVariable);
        let node = self;
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
}
