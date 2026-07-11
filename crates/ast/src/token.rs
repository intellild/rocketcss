use super::*;

use rocketcss_allocator::boxed::Box;

#[derive(Debug, PartialEq)]
pub enum TokenOrValue<'a> {
    Token(Box<'a, Token<'a>>),
    Color(Box<'a, CssColor<'a>>),
    UnresolvedColor(Box<'a, UnresolvedColor<'a>>),
    Url(Box<'a, Url<'a>>),
    Var(Box<'a, Variable<'a>>),
    Env(Box<'a, EnvironmentVariable<'a>>),
    Function(Box<'a, Function<'a>>),
    Length(Box<'a, LengthValue>),
    Angle(Box<'a, Angle>),
    Time(Box<'a, Time>),
    Resolution(Box<'a, Resolution>),
    DashedIdent(&'a str),
    AnimationName(Box<'a, AnimationName<'a>>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Length(LengthUnit),
    Deg,
    Rad,
    Grad,
    Turn,
    Seconds,
    Milliseconds,
    Hertz,
    Kilohertz,
    Dpi,
    Dpcm,
    Dppx,
    ResolutionX,
    Flex,
}

impl Unit {
    pub const fn length(self) -> Option<LengthUnit> {
        match self {
            Self::Length(unit) => Some(unit),
            _ => None,
        }
    }

    pub const fn is_length(self) -> bool {
        matches!(self, Self::Length(_))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    Ident(&'a str),
    AtKeyword(&'a str),
    Hash(&'a str),
    IdHash(&'a str),
    String(&'a str),
    UnquotedUrl(&'a str),
    Delim(&'a str),
    Number(f32),
    Percentage(f32),
    Dimension { unit: Unit, value: f32 },
    UnknownDimension { unit: &'a str, value: f32 },
    WhiteSpace(&'a str),
    Comment(&'a str),
    Colon,
    Semicolon,
    Comma,
    IncludeMatch,
    DashMatch,
    PrefixMatch,
    SuffixMatch,
    SubstringMatch,
    Cdo,
    Cdc,
    Function(&'a str),
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl(&'a str),
    BadString(&'a str),
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}

#[derive(Debug, PartialEq)]
pub enum Specifier<'a> {
    Global,
    File(&'a str),
    SourceIndex(u32),
}

#[derive(Debug, PartialEq)]
pub enum AnimationName<'a> {
    None,
    Ident(&'a str),
    String(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum EnvironmentVariableName<'a> {
    UA(UAEnvironmentVariable),
    Custom(Box<'a, DashedIdentReference<'a>>),
    Unknown(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum UAEnvironmentVariable {
    SafeAreaInsetTop,
    SafeAreaInsetRight,
    SafeAreaInsetBottom,
    SafeAreaInsetLeft,
    ViewportSegmentWidth,
    ViewportSegmentHeight,
    ViewportSegmentTop,
    ViewportSegmentLeft,
    ViewportSegmentBottom,
    ViewportSegmentRight,
}
