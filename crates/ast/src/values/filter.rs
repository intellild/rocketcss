use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FilterList<'a> {
    None,
    Filters(Vec<'a, Filter<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Filter<'a> {
    Blur(NodeId<'a, Length<'a>>),
    Brightness(NumberOrPercentage),
    Contrast(NumberOrPercentage),
    Grayscale(NumberOrPercentage),
    HueRotate(Angle),
    Invert(NumberOrPercentage),
    Opacity(NumberOrPercentage),
    Saturate(NumberOrPercentage),
    Sepia(NumberOrPercentage),
    DropShadow(NodeId<'a, DropShadow<'a>>),
    Url(NodeId<'a, Url<'a>>),
}
