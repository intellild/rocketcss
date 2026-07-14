use crate::*;

#[derive(Debug, PartialEq)]
pub enum ParsedComponent<'a> {
    Length(Box<'a, Length<'a>>),
    Number(f32),
    Percentage(f32),
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    String(&'a str),
    Color(Box<'a, CssColor<'a>>),
    Image(Box<'a, Image<'a>>),
    Url(Box<'a, Url<'a>>),
    Integer(i32),
    Angle(Box<'a, Angle>),
    Time(Box<'a, Time>),
    Resolution(Box<'a, Resolution>),
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

#[derive(Debug, PartialEq)]
pub enum Multiplier {
    None,
    Space,
    Comma,
}

#[derive(Debug, PartialEq)]
pub enum SyntaxString<'a> {
    Components(Vec<'a, SyntaxComponent<'a>>),
    Universal,
}

#[derive(CssKeyword, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct UnparsedProperty<'a> {
    pub property_id: Box<'a, PropertyId<'a>>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct CustomProperty<'a> {
    pub name: Box<'a, CustomPropertyName<'a>>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct PropertyRule<'a> {
    pub inherits: bool,
    pub initial_value: Option<Box<'a, ParsedComponent<'a>>>,
    pub span: Span,
    pub name: &'a str,
    pub syntax: Box<'a, SyntaxString<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct SyntaxComponent<'a> {
    pub kind: Box<'a, SyntaxComponentKind<'a>>,
    pub multiplier: Multiplier,
}
