use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TokenOrValue<'ast>(pub(crate) NodeId<'ast, TokenOrValueData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenOrValueData<'ast> {
    Token(Token<'ast>),
    Color(CssColor<'ast>),
    UnresolvedColor(UnresolvedColor<'ast>),
    Url(Url<'ast>),
    Var(Variable<'ast>),
    Env(EnvironmentVariable<'ast>),
    Function(Function<'ast>),
    Length(LengthValue<'ast>),
    Angle(Angle),
    Time(Time),
    Resolution(Resolution),
    DashedIdent(&'ast str),
    AnimationName(AnimationName<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenOrValueKind {
    Token,
    Color,
    UnresolvedColor,
    Url,
    Var,
    Env,
    Function,
    Length,
    Angle,
    Time,
    Resolution,
    DashedIdent,
    AnimationName,
}
impl<'ast> NodeStorage<'ast> for TokenOrValue<'ast> {
    const KIND: NodeKind = super::kind::KIND_TOKEN_OR_VALUE;
    type Data = TokenOrValueData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TokenOrValueData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TokenOrValueData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TokenOrValue<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TokenOrValueData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TokenOrValueKind {
        match ast.data(self) {
            TokenOrValueData::Token { .. } => TokenOrValueKind::Token,
            TokenOrValueData::Color { .. } => TokenOrValueKind::Color,
            TokenOrValueData::UnresolvedColor { .. } => TokenOrValueKind::UnresolvedColor,
            TokenOrValueData::Url { .. } => TokenOrValueKind::Url,
            TokenOrValueData::Var { .. } => TokenOrValueKind::Var,
            TokenOrValueData::Env { .. } => TokenOrValueKind::Env,
            TokenOrValueData::Function { .. } => TokenOrValueKind::Function,
            TokenOrValueData::Length { .. } => TokenOrValueKind::Length,
            TokenOrValueData::Angle { .. } => TokenOrValueKind::Angle,
            TokenOrValueData::Time { .. } => TokenOrValueKind::Time,
            TokenOrValueData::Resolution { .. } => TokenOrValueKind::Resolution,
            TokenOrValueData::DashedIdent { .. } => TokenOrValueKind::DashedIdent,
            TokenOrValueData::AnimationName { .. } => TokenOrValueKind::AnimationName,
        }
    }
}
impl<'ast> GetSpan<'ast> for TokenOrValue<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TokenOrValue<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_token_or_value(
        &mut self,
        data: TokenOrValueData<'ast>,
        span: Span,
    ) -> TokenOrValue<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Unit<'ast>(pub(crate) NodeId<'ast, UnitData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitData {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnitKind {
    Length,
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
impl<'ast> NodeStorage<'ast> for Unit<'ast> {
    const KIND: NodeKind = super::kind::KIND_UNIT;
    type Data = UnitData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, UnitData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, UnitData>) -> Self {
        Self(id)
    }
}
impl<'ast> Unit<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> UnitData {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> UnitKind {
        match ast.data(self) {
            UnitData::Length { .. } => UnitKind::Length,
            UnitData::Deg { .. } => UnitKind::Deg,
            UnitData::Rad { .. } => UnitKind::Rad,
            UnitData::Grad { .. } => UnitKind::Grad,
            UnitData::Turn { .. } => UnitKind::Turn,
            UnitData::Seconds { .. } => UnitKind::Seconds,
            UnitData::Milliseconds { .. } => UnitKind::Milliseconds,
            UnitData::Hertz { .. } => UnitKind::Hertz,
            UnitData::Kilohertz { .. } => UnitKind::Kilohertz,
            UnitData::Dpi { .. } => UnitKind::Dpi,
            UnitData::Dpcm { .. } => UnitKind::Dpcm,
            UnitData::Dppx { .. } => UnitKind::Dppx,
            UnitData::ResolutionX { .. } => UnitKind::ResolutionX,
            UnitData::Flex { .. } => UnitKind::Flex,
        }
    }
}
impl<'ast> GetSpan<'ast> for Unit<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Unit<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_unit(&mut self, data: UnitData, span: Span) -> Unit<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Token<'ast>(pub(crate) NodeId<'ast, TokenData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenData<'ast> {
    Ident(&'ast str),
    AtKeyword(&'ast str),
    Hash(&'ast str),
    IdHash(&'ast str),
    MinifiedHash(&'ast str),
    String(&'ast str),
    UnquotedFont(&'ast str),
    UnquotedUrl(&'ast str),
    Delim(&'ast str),
    Number(f32),
    Percentage(f32),
    Dimension { unit: Unit<'ast>, value: f32 },
    UnknownDimension { unit: &'ast str, value: f32 },
    WhiteSpace(&'ast str),
    Comment(&'ast str),
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
    Function(&'ast str),
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl(&'ast str),
    BadString(&'ast str),
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Ident,
    AtKeyword,
    Hash,
    IdHash,
    MinifiedHash,
    String,
    UnquotedFont,
    UnquotedUrl,
    Delim,
    Number,
    Percentage,
    Dimension,
    UnknownDimension,
    WhiteSpace,
    Comment,
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
    Function,
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl,
    BadString,
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}
impl<'ast> NodeStorage<'ast> for Token<'ast> {
    const KIND: NodeKind = super::kind::KIND_TOKEN;
    type Data = TokenData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TokenData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TokenData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Token<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TokenData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TokenKind {
        match ast.data(self) {
            TokenData::Ident { .. } => TokenKind::Ident,
            TokenData::AtKeyword { .. } => TokenKind::AtKeyword,
            TokenData::Hash { .. } => TokenKind::Hash,
            TokenData::IdHash { .. } => TokenKind::IdHash,
            TokenData::MinifiedHash { .. } => TokenKind::MinifiedHash,
            TokenData::String { .. } => TokenKind::String,
            TokenData::UnquotedFont { .. } => TokenKind::UnquotedFont,
            TokenData::UnquotedUrl { .. } => TokenKind::UnquotedUrl,
            TokenData::Delim { .. } => TokenKind::Delim,
            TokenData::Number { .. } => TokenKind::Number,
            TokenData::Percentage { .. } => TokenKind::Percentage,
            TokenData::Dimension { .. } => TokenKind::Dimension,
            TokenData::UnknownDimension { .. } => TokenKind::UnknownDimension,
            TokenData::WhiteSpace { .. } => TokenKind::WhiteSpace,
            TokenData::Comment { .. } => TokenKind::Comment,
            TokenData::Colon { .. } => TokenKind::Colon,
            TokenData::Semicolon { .. } => TokenKind::Semicolon,
            TokenData::Comma { .. } => TokenKind::Comma,
            TokenData::IncludeMatch { .. } => TokenKind::IncludeMatch,
            TokenData::DashMatch { .. } => TokenKind::DashMatch,
            TokenData::PrefixMatch { .. } => TokenKind::PrefixMatch,
            TokenData::SuffixMatch { .. } => TokenKind::SuffixMatch,
            TokenData::SubstringMatch { .. } => TokenKind::SubstringMatch,
            TokenData::Cdo { .. } => TokenKind::Cdo,
            TokenData::Cdc { .. } => TokenKind::Cdc,
            TokenData::Function { .. } => TokenKind::Function,
            TokenData::ParenthesisBlock { .. } => TokenKind::ParenthesisBlock,
            TokenData::SquareBracketBlock { .. } => TokenKind::SquareBracketBlock,
            TokenData::CurlyBracketBlock { .. } => TokenKind::CurlyBracketBlock,
            TokenData::BadUrl { .. } => TokenKind::BadUrl,
            TokenData::BadString { .. } => TokenKind::BadString,
            TokenData::CloseParenthesis { .. } => TokenKind::CloseParenthesis,
            TokenData::CloseSquareBracket { .. } => TokenKind::CloseSquareBracket,
            TokenData::CloseCurlyBracket { .. } => TokenKind::CloseCurlyBracket,
        }
    }
}
impl<'ast> GetSpan<'ast> for Token<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Token<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_token(&mut self, data: TokenData<'ast>, span: Span) -> Token<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Specifier<'ast> {
    Global,
    File(&'ast str),
    SourceIndex(u32),
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationName<'ast> {
    None,
    Ident(&'ast str),
    String(&'ast str),
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EnvironmentVariableName<'ast>(
    pub(crate) NodeId<'ast, EnvironmentVariableNameData<'ast>>,
);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnvironmentVariableNameData<'ast> {
    UA(UAEnvironmentVariable),
    Custom(DashedIdentReference<'ast>),
    Unknown(&'ast str),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnvironmentVariableNameKind {
    UA,
    Custom,
    Unknown,
}
impl<'ast> NodeStorage<'ast> for EnvironmentVariableName<'ast> {
    const KIND: NodeKind = super::kind::KIND_ENVIRONMENT_VARIABLE_NAME;
    type Data = EnvironmentVariableNameData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, EnvironmentVariableNameData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, EnvironmentVariableNameData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> EnvironmentVariableName<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> EnvironmentVariableNameData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> EnvironmentVariableNameKind {
        match ast.data(self) {
            EnvironmentVariableNameData::UA { .. } => EnvironmentVariableNameKind::UA,
            EnvironmentVariableNameData::Custom { .. } => EnvironmentVariableNameKind::Custom,
            EnvironmentVariableNameData::Unknown { .. } => EnvironmentVariableNameKind::Unknown,
        }
    }
}
impl<'ast> GetSpan<'ast> for EnvironmentVariableName<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for EnvironmentVariableName<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_environment_variable_name(
        &mut self,
        data: EnvironmentVariableNameData<'ast>,
        span: Span,
    ) -> EnvironmentVariableName<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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
