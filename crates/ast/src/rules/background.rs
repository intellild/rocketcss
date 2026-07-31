use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Position {
    pub x: std::boxed::Box<PositionComponent<HorizontalPositionKeyword>>,
    pub y: std::boxed::Box<PositionComponent<VerticalPositionKeyword>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct WebKitGradientPoint {
    pub x: WebKitGradientPointComponent<HorizontalPositionKeyword>,
    pub y: WebKitGradientPointComponent<VerticalPositionKeyword>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct WebKitColorStop<'a> {
    pub color: std::boxed::Box<CssColor<'a>>,
    pub position: f32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ImageSet<'a> {
    pub options: std::vec::Vec<ImageSetOption<'a>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ImageSetOption<'a> {
    pub file_type: Option<&'a str>,
    pub image: std::boxed::Box<Image<'a>>,
    pub resolution: Resolution,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BackgroundPosition {
    pub x: std::boxed::Box<PositionComponent<HorizontalPositionKeyword>>,
    pub y: std::boxed::Box<PositionComponent<VerticalPositionKeyword>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BackgroundRepeat {
    pub x: BackgroundRepeatKeyword,
    pub y: BackgroundRepeatKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Background<'a> {
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub color: std::boxed::Box<CssColor<'a>>,
    pub image: std::boxed::Box<Image<'a>>,
    pub origin: BackgroundOrigin,
    pub position: std::boxed::Box<BackgroundPosition>,
    pub repeat: BackgroundRepeat,
    pub size: std::boxed::Box<BackgroundSize>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BoxShadow<'a> {
    pub blur: std::boxed::Box<Length>,
    pub color: std::boxed::Box<CssColor<'a>>,
    pub inset: bool,
    pub spread: std::boxed::Box<Length>,
    pub x_offset: std::boxed::Box<Length>,
    pub y_offset: std::boxed::Box<Length>,
}
