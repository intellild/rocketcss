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
    DashedIdent(Atom<'a>),
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
    Ident(Atom<'a>),
    AtKeyword(Atom<'a>),
    Hash(Atom<'a>),
    IdHash(Atom<'a>),
    String(Atom<'a>),
    UnquotedUrl(Atom<'a>),
    Delim(Atom<'a>),
    Number(f32),
    Percentage(f32),
    Dimension { unit: Unit, value: f32 },
    UnknownDimension { unit: Atom<'a>, value: f32 },
    WhiteSpace(Atom<'a>),
    Comment(Atom<'a>),
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
    Function(Atom<'a>),
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl(Atom<'a>),
    BadString(Atom<'a>),
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}

#[derive(Debug, PartialEq)]
pub enum Specifier<'a> {
    Global,
    File(Atom<'a>),
    SourceIndex(u32),
}

#[derive(Debug, PartialEq)]
pub enum AnimationName<'a> {
    None,
    Ident(Atom<'a>),
    String(Atom<'a>),
}

#[derive(Debug, PartialEq)]
pub enum EnvironmentVariableName<'a> {
    UA(UAEnvironmentVariable),
    Custom(Box<'a, DashedIdentReference<'a>>),
    Unknown(Atom<'a>),
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
