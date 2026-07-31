use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FilterList<'a> {
    None,
    Filters(std::vec::Vec<Filter<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Filter<'a> {
    Blur(std::boxed::Box<Length>),
    Brightness(NumberOrPercentage),
    Contrast(NumberOrPercentage),
    Grayscale(NumberOrPercentage),
    HueRotate(Angle),
    Invert(NumberOrPercentage),
    Opacity(NumberOrPercentage),
    Saturate(NumberOrPercentage),
    Sepia(NumberOrPercentage),
    DropShadow(std::boxed::Box<DropShadow<'a>>),
    Url(std::boxed::Box<Url<'a>>),
}
