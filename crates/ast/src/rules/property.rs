use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ParsedComponent<'a> {
    Length(Length),
    Number(f32),
    Percentage(f32),
    LengthPercentage(LengthPercentage),
    String(&'a str),
    Color(CssColor<'a>),
    Image(std::boxed::Box<Image<'a>>),
    Url(Url<'a>),
    Integer(i32),
    Angle(Angle),
    Time(Time),
    Resolution(Resolution),
    TransformFunction(std::boxed::Box<Transform>),
    TransformList(std::vec::Vec<Transform>),
    CustomIdent(&'a str),
    Literal(&'a str),
    Repeated {
        components: std::vec::Vec<ParsedComponent<'a>>,
        multiplier: Multiplier,
    },
    TokenList(std::vec::Vec<TokenOrValue<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Multiplier {
    None,
    Space,
    Comma,
}

#[derive(Debug, PartialEq, Visit)]
pub enum SyntaxString {
    Components(std::vec::Vec<SyntaxComponent>),
    Universal,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum SyntaxComponentKind {
    Length,
    Number,
    Percentage,
    LengthPercentage,
    String,
    Color,
    Image,
    Url,
    Integer,
    Angle,
    Time,
    Resolution,
    TransformFunction,
    TransformList,
    CustomIdent,
    Literal(std::string::String),
}

#[derive(Debug, PartialEq, Visit)]
pub struct UnparsedProperty<'a> {
    pub property_id: std::boxed::Box<PropertyId<'a>>,
    #[visit(skip)]
    pub reason: UnparsedPropertyReason,
    pub value: std::vec::Vec<TokenOrValue<'a>>,
}

/// Why a declaration could not be represented by its typed value AST.
///
/// Keeping this decision in the parsed tree lets transforms distinguish
/// unsupported grammar from values whose syntax or semantics are opaque.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub enum UnparsedPropertyReason {
    /// RocketCSS recognizes the property, but does not implement its grammar yet.
    UnsupportedGrammar,
    /// The property name is unknown, so its value grammar is also unknown.
    UnknownProperty,
    /// A supported grammar contains a function or comment that cannot be
    /// validated without preserving its original token representation.
    OpaqueValue,
    /// The implemented grammar rejected an otherwise tokenizable value.
    InvalidValue,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CustomProperty<'a> {
    pub name: std::boxed::Box<CustomPropertyName<'a>>,
    pub value: std::vec::Vec<TokenOrValue<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PropertyRule<'a> {
    pub inherits: bool,
    pub initial_value: Option<std::boxed::Box<ParsedComponent<'a>>>,
    pub span: Span,
    pub name: Atom<'a>,
    pub syntax: std::boxed::Box<SyntaxString>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct SyntaxComponent {
    pub kind: std::boxed::Box<SyntaxComponentKind>,
    pub multiplier: Multiplier,
}
