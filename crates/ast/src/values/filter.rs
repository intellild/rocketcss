use crate::*;

#[derive(Debug, PartialEq)]
pub enum FilterList<'a> {
    None,
    Filters(Vec<'a, Filter<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Filter<'a> {
    Blur(Box<'a, Length<'a>>),
    Brightness(Box<'a, NumberOrPercentage>),
    Contrast(Box<'a, NumberOrPercentage>),
    Grayscale(Box<'a, NumberOrPercentage>),
    HueRotate(Box<'a, Angle>),
    Invert(Box<'a, NumberOrPercentage>),
    Opacity(Box<'a, NumberOrPercentage>),
    Saturate(Box<'a, NumberOrPercentage>),
    Sepia(Box<'a, NumberOrPercentage>),
    DropShadow(Box<'a, DropShadow<'a>>),
    Url(Box<'a, Url<'a>>),
}
