use super::*;

use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Visit)]
pub enum TokenOrValue<'a> {
    Token(std::boxed::Box<Token<'a>>),
    Color(std::boxed::Box<CssColor<'a>>),
    UnresolvedColor(std::boxed::Box<UnresolvedColor<'a>>),
    Url(std::boxed::Box<Url<'a>>),
    Var(std::boxed::Box<Variable<'a>>),
    Env(std::boxed::Box<EnvironmentVariable<'a>>),
    Function(std::boxed::Box<Function<'a>>),
    Length(LengthValue),
    Angle(Angle),
    Time(Time),
    Resolution(Resolution),
    DashedIdent(Atom<'a>),
    AnimationName(std::boxed::Box<AnimationName<'a>>),
}

impl Eq for TokenOrValue<'_> {}

impl Hash for TokenOrValue<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        if let Self::Token(token) = self {
            token.hash(state);
        }
        // Values in selectors are rare, and fully hashing them would pull
        // floating-point hashing through much of the AST. Equal values still
        // share this hash; collisions are resolved by structural equality.
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
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

#[derive(Clone, Debug, PartialEq, Visit)]
pub enum Token<'a> {
    Ident(Atom<'a>),
    AtKeyword(Atom<'a>),
    Hash(Atom<'a>),
    IdHash(Atom<'a>),
    /// A hexadecimal color hash normalized during minification.
    MinifiedHash(Atom<'a>),
    String(Atom<'a>),
    /// A quoted font family that can be serialized as identifiers in place.
    UnquotedFont(Atom<'a>),
    UnquotedUrl(Atom<'a>),
    Delim(&'a str),
    Number(f32),
    Percentage(f32),
    Dimension {
        unit: Unit,
        value: f32,
    },
    UnknownDimension {
        unit: Atom<'a>,
        value: f32,
    },
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

impl Eq for Token<'_> {}

impl Hash for Token<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Delim(value) | Self::WhiteSpace(value) | Self::Comment(value) => {
                value.hash(state)
            }
            Self::Ident(value)
            | Self::AtKeyword(value)
            | Self::Hash(value)
            | Self::IdHash(value)
            | Self::MinifiedHash(value)
            | Self::String(value)
            | Self::UnquotedFont(value)
            | Self::UnquotedUrl(value)
            | Self::Function(value)
            | Self::BadUrl(value)
            | Self::BadString(value) => value.hash(state),
            Self::Number(value) | Self::Percentage(value) => hash_float(*value, state),
            Self::Dimension { unit, value } => {
                unit.hash(state);
                hash_float(*value, state);
            }
            Self::UnknownDimension { unit, value } => {
                unit.hash(state);
                hash_float(*value, state);
            }
            Self::Colon
            | Self::Semicolon
            | Self::Comma
            | Self::IncludeMatch
            | Self::DashMatch
            | Self::PrefixMatch
            | Self::SuffixMatch
            | Self::SubstringMatch
            | Self::Cdo
            | Self::Cdc
            | Self::ParenthesisBlock
            | Self::SquareBracketBlock
            | Self::CurlyBracketBlock
            | Self::CloseParenthesis
            | Self::CloseSquareBracket
            | Self::CloseCurlyBracket => {}
        }
    }
}

#[inline]
fn hash_float<H: Hasher>(value: f32, state: &mut H) {
    // PartialEq considers both signed zero representations equal.
    if value == 0.0 {
        0_u32.hash(state);
    } else {
        value.to_bits().hash(state);
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum Specifier<'a> {
    Global,
    File(&'a str),
    SourceIndex(u32),
}

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationName<'a> {
    None,
    Ident(Atom<'a>),
    String(Atom<'a>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum EnvironmentVariableName<'a> {
    UA(UAEnvironmentVariable),
    Custom(std::boxed::Box<DashedIdentReference<'a>>),
    Unknown(Atom<'a>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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
