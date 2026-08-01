use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FilterList<'a> {
    None,
    Filters(Vec<'a, Filter<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Filter<'a> {
    Blur(Box<'a, Length<'a>>),
    Brightness(NumberOrPercentage),
    Contrast(NumberOrPercentage),
    Grayscale(NumberOrPercentage),
    HueRotate(Angle),
    Invert(NumberOrPercentage),
    Opacity(NumberOrPercentage),
    Saturate(NumberOrPercentage),
    Sepia(NumberOrPercentage),
    DropShadow(Box<'a, DropShadow<'a>>),
    Url(Box<'a, Url<'a>>),
}
