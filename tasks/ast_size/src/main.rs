use std::{
    marker::PhantomData,
    mem::{align_of, size_of},
};

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
        std::boxed::Box<u8>,
        std::vec::Vec<u8>,
        CssRule<'static>,
        StyleRule<'static>,
        DeclarationBlock<'static>,
        Declaration<'static>,
        PropertyId<'static>,
        TokenOrValue<'static>,
        Token<'static>,
        Length,
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
        Transform,
        KeyframeSelector,
        Display,
        FontWeight,
        FontStyle,
        AspectRatio,
        PlaceContent,
        PlaceSelf,
        PlaceItems,
        Columns,
        TrackRepeat<'static>,
        Grid<'static>,
        Background<'static>,
        Mask<'static>,
        BorderImage<'static>,
        EnvironmentVariable<'static>,
        DashedIdentReference<'static>,
        CounterStyleRule<'static>,
        NestedDeclarationsRule,
        ViewportRule,
        PositionTryRule<'static>,
        PageRule<'static>,
        PageMarginRule,
        Keyframe,
        PositionProperty,
        BorderStyle,
        BorderBlockStyle,
        BorderInlineStyle,
        FlexFlow,
        Scale,
        CssColor<'static>,
        Size<'static>,
        MaxSize<'static>,
        LengthPercentageOrAuto,
        BorderSideWidth,
        Size2D<Length>,
        Size2D<LengthPercentage>,
        BorderRadius,
        BorderColor<'static>,
        BorderWidth,
        GenericBorder<'static, LineStyle>,
        GapValue,
        Gap,
        GridLine<'static>,
        GridRow<'static>,
        GridColumn<'static>,
        GridArea<'static>,
        FontSize,
        LineHeight,
        DashedIdentReference<'static>,
        Perspective,
        Translate,
        LengthOrNumber,
        Spacing,
        ColorOrAuto<'static>,
        ListStyleType<'static>,
        SVGPaint<'static>,
        LengthPercentage,
        Marker<'static>,
        ViewTransitionName<'static>,
        ViewTransitionGroup<'static>,
        Appearance<'static>,
        BorderImageSlice,
        Caret<'static>,
        ClipPath<'static>,
        ColumnRule<'static>,
        Composes<'static>,
        Container<'static>,
        ContainerNameList<'static>,
        Cursor<'static>,
        FilterList<'static>,
        Flex,
        Font<'static>,
        Image<'static>,
        Inset,
        ListStyle<'static>,
        MaskBorder<'static>,
        NoneOrCustomIdentList<'static>,
        Position,
        StrokeDasharray,
        TextDecoration<'static>,
        TextDecorationLine,
        TextDecorationThickness,
        TextEmphasis<'static>,
        TextEmphasisStyle<'static>,
        TextIndent,
        TrackSizing<'static>,
        VerticalAlign,
    );
    print_all_property_sizes(PhantomData::<&'static ()>);
}

fn print_all_property_sizes<'a>(_: PhantomData<&'a ()>) {
    rocketcss_ast::for_each_property!(print_property_sizes);
}
