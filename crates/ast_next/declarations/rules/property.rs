#[ast]
pub enum ParsedComponent<'a> {
    Length(Length<'a>),
    Number(f32),
    Percentage(f32),
    LengthPercentage(LengthPercentage<'a>),
    String(&'a str),
    Color(CssColor<'a>),
    Image(Box<'a, Image<'a>>),
    Url(Url<'a>),
    Integer(i32),
    Angle(Angle),
    Time(Time),
    Resolution(Resolution),
    TransformFunction(Box<'a, Transform<'a>>),
    TransformList(Vec<'a, Transform<'a>>),
    CustomIdent(&'a str),
    Literal(&'a str),
    Repeated {
        components: Vec<'a, ParsedComponent<'a>>,
        multiplier: Multiplier,
    },
    TokenList(Vec<'a, TokenOrValue<'a>>),
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Multiplier {
    None,
    Space,
    Comma,
}

#[ast]
pub enum SyntaxString<'a> {
    Components(Vec<'a, SyntaxComponent<'a>>),
    Universal,
}

#[ast]
pub enum SyntaxComponentKind<'a> {
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
    Literal(&'a str),
}

#[ast]
pub struct UnparsedProperty<'a> {
    pub property_id: Box<'a, PropertyId<'a>>,
    pub reason: UnparsedPropertyReason,
    /// The authored value after removing declaration-level whitespace and
    /// `!important`. This keeps fallback serialization independent from the
    /// lossy numeric and function normalization used by typed tokens.
    pub raw_value: Option<&'a str>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

/// Why a declaration could not be represented by its typed value AST.
///
/// Keeping this decision in the parsed tree lets transforms distinguish
/// unsupported grammar from values whose syntax or semantics are opaque.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[ast]
pub struct CustomProperty<'a> {
    pub name: Box<'a, CustomPropertyName<'a>>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

#[ast]
pub struct SyntaxComponent<'a> {
    pub kind: Box<'a, SyntaxComponentKind<'a>>,
    pub multiplier: Multiplier,
}
