use std::{
    marker::PhantomData,
    mem::{align_of, size_of},
};

use rocketcss_allocator::{boxed::Box, vec::Vec};
use rocketcss_ast::*;

macro_rules! print_sizes {
    ($($ty:ty),+ $(,)?) => {
        $(println!(
            "{:<56} {:>4} {:>5}",
            stringify!($ty),
            size_of::<$ty>(),
            align_of::<$ty>(),
        );)+
    };
}

macro_rules! print_property_sizes {
    (
        $(
            $(#[$meta:meta])*
            $name:literal: $property:ident($value:ty $(, $vendor_prefix:ty)?),
        )+
    ) => {
        println!();
        println!("property payloads");
        println!("{:<32} {:<48} {:>4} {:>5}", "property", "type", "size", "align");
        println!("{}", "-".repeat(95));
        $(
            println!(
                "{:<32} {:<48} {:>4} {:>5}",
                $name,
                stringify!($value),
                size_of::<$value>(),
                align_of::<$value>(),
            );
        )+
    };
}

fn main() {
    println!("{:<56} {:>4} {:>5}", "type", "size", "align");
    println!("{}", "-".repeat(68));
    print_sizes!(
        rocketcss_allocator::boxed::Box<'static, u8>,
        rocketcss_allocator::vec::Vec<'static, u8>,
        CssRule<'static, 'static>,
        StyleRule<'static, 'static>,
        DeclarationBlock<'static>,
        Declaration<'static>,
        PropertyId<'static>,
        TokenOrValue<'static>,
        Token<'static>,
        Length<'static>,
        LengthValue,
        CssColor<'static>,
        MediaList<'static>,
        MediaQuery<'static>,
        MediaCondition<'static>,
        Selector<'static>,
        SelectorComponent<'static>,
        ParsedComponent<'static>,
        AnimationComponent<'static>,
        AnimationTimeline<'static>,
        Filter<'static>,
        Transform<'static>,
        KeyframeSelector,
        Display,
        FontWeight,
        FontStyle,
        AspectRatio,
        PlaceContent,
        PlaceSelf,
        PlaceItems,
        Columns<'static>,
        TrackRepeat<'static>,
        Grid<'static>,
        Background<'static>,
        Mask<'static>,
        BorderImage<'static>,
        EnvironmentVariable<'static>,
        DashedIdentReference<'static>,
        CounterStyleRule<'static, 'static>,
        NestedDeclarationsRule<'static, 'static>,
        ViewportRule<'static, 'static>,
        PositionTryRule<'static, 'static>,
        PageRule<'static, 'static>,
        PageMarginRule<'static, 'static>,
        Keyframe<'static, 'static>,
        PositionProperty,
        BorderStyle,
        BorderBlockStyle,
        BorderInlineStyle,
        FlexFlow,
        Scale,
        CssColor<'static>,
        Size<'static>,
        MaxSize<'static>,
        LengthPercentageOrAuto<'static>,
        BorderSideWidth<'static>,
        Size2D<'static, Length<'static>>,
        Size2D<'static, LengthPercentage<'static>>,
        BorderRadius<'static>,
        BorderColor<'static>,
        BorderWidth<'static>,
        GenericBorder<'static, LineStyle>,
        GapValue<'static>,
        Gap<'static>,
        GridLine<'static>,
        GridRow<'static>,
        GridColumn<'static>,
        GridArea<'static>,
        FontSize<'static>,
        LineHeight<'static>,
        DashedIdentReference<'static>,
        Perspective<'static>,
        Translate<'static>,
        LengthOrNumber<'static>,
        Spacing<'static>,
        ColorOrAuto<'static>,
        ListStyleType<'static>,
        SVGPaint<'static>,
        LengthPercentage<'static>,
        Marker<'static>,
        ViewTransitionName<'static>,
        ViewTransitionGroup<'static>,
        Appearance<'static>,
        BorderImageSlice<'static>,
        Caret<'static>,
        ClipPath<'static>,
        ColumnRule<'static>,
        Composes<'static>,
        Container<'static>,
        ContainerNameList<'static>,
        Cursor<'static>,
        FilterList<'static>,
        Flex<'static>,
        Font<'static>,
        Image<'static>,
        Inset<'static>,
        ListStyle<'static>,
        MaskBorder<'static>,
        NoneOrCustomIdentList<'static>,
        Position<'static>,
        StrokeDasharray<'static>,
        TextDecoration<'static>,
        TextDecorationLine<'static>,
        TextDecorationThickness<'static>,
        TextEmphasis<'static>,
        TextEmphasisStyle<'static>,
        TextIndent<'static>,
        TrackSizing<'static>,
        VerticalAlign<'static>,
    );
    print_all_property_sizes(PhantomData::<&'static ()>);
}

fn print_all_property_sizes<'a>(_: PhantomData<&'a ()>) {
    rocketcss_ast::for_each_property!(print_property_sizes);
}
