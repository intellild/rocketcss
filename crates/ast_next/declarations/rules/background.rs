#[ast]
pub struct Position<'a> {
    pub x: Box<'a, PositionComponent<'a, HorizontalPositionKeyword>>,
    pub y: Box<'a, PositionComponent<'a, VerticalPositionKeyword>>,
}

#[ast]
pub struct WebKitGradientPoint {
    pub x: WebKitGradientPointComponent<HorizontalPositionKeyword>,
    pub y: WebKitGradientPointComponent<VerticalPositionKeyword>,
}

#[ast]
pub struct WebKitColorStop<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub position: f32,
}

#[ast]
pub struct ImageSet<'a> {
    pub options: Vec<'a, ImageSetOption<'a>>,
    pub vendor_prefix: VendorPrefix,
}

#[ast]
pub struct ImageSetOption<'a> {
    pub file_type: Option<&'a str>,
    pub image: Box<'a, Image<'a>>,
    pub resolution: Resolution,
}

#[ast]
pub struct BackgroundPosition<'a> {
    pub x: Box<'a, PositionComponent<'a, HorizontalPositionKeyword>>,
    pub y: Box<'a, PositionComponent<'a, VerticalPositionKeyword>>,
}

#[ast]
pub struct BackgroundRepeat {
    pub x: BackgroundRepeatKeyword,
    pub y: BackgroundRepeatKeyword,
}

#[ast]
pub struct Background<'a> {
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub color: Box<'a, CssColor<'a>>,
    pub image: Box<'a, Image<'a>>,
    pub origin: BackgroundOrigin,
    pub position: Box<'a, BackgroundPosition<'a>>,
    pub repeat: BackgroundRepeat,
    pub size: Box<'a, BackgroundSize<'a>>,
}

#[ast]
pub struct BoxShadow<'a> {
    pub blur: Box<'a, Length<'a>>,
    pub color: Box<'a, CssColor<'a>>,
    pub inset: bool,
    pub spread: Box<'a, Length<'a>>,
    pub x_offset: Box<'a, Length<'a>>,
    pub y_offset: Box<'a, Length<'a>>,
}
