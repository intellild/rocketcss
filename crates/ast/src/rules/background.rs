use crate::*;

#[derive(Debug, PartialEq)]
pub struct Position<'a> {
    pub x: Box<'a, PositionComponent<'a, HorizontalPositionKeyword>>,
    pub y: Box<'a, PositionComponent<'a, VerticalPositionKeyword>>,
}

#[derive(Debug, PartialEq)]
pub struct WebKitGradientPoint<'a> {
    pub x: Box<'a, WebKitGradientPointComponent<'a, HorizontalPositionKeyword>>,
    pub y: Box<'a, WebKitGradientPointComponent<'a, VerticalPositionKeyword>>,
}

#[derive(Debug, PartialEq)]
pub struct WebKitColorStop<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub position: f32,
}

#[derive(Debug, PartialEq)]
pub struct ImageSet<'a> {
    pub options: Vec<'a, ImageSetOption<'a>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq)]
pub struct ImageSetOption<'a> {
    pub file_type: Option<&'a str>,
    pub image: Box<'a, Image<'a>>,
    pub resolution: Box<'a, Resolution>,
}

#[derive(Debug, PartialEq)]
pub struct BackgroundPosition<'a> {
    pub x: Box<'a, PositionComponent<'a, HorizontalPositionKeyword>>,
    pub y: Box<'a, PositionComponent<'a, VerticalPositionKeyword>>,
}

#[derive(Debug, PartialEq)]
pub struct BackgroundRepeat {
    pub x: BackgroundRepeatKeyword,
    pub y: BackgroundRepeatKeyword,
}

#[derive(Debug, PartialEq)]
pub struct Background<'a> {
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub color: Box<'a, CssColor<'a>>,
    pub image: Box<'a, Image<'a>>,
    pub origin: BackgroundOrigin,
    pub position: Box<'a, BackgroundPosition<'a>>,
    pub repeat: Box<'a, BackgroundRepeat>,
    pub size: Box<'a, BackgroundSize<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BoxShadow<'a> {
    pub blur: Box<'a, Length<'a>>,
    pub color: Box<'a, CssColor<'a>>,
    pub inset: bool,
    pub spread: Box<'a, Length<'a>>,
    pub x_offset: Box<'a, Length<'a>>,
    pub y_offset: Box<'a, Length<'a>>,
}
