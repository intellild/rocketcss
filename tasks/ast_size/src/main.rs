use std::{
    marker::PhantomData,
    mem::{align_of, size_of},
};

use rocketcss_ast::*;
use rocketcss_ast::{
    CssRulePayload, DeclarationBlockRecord, DeclarationPayload, DeclarationRecord, KeyframePayload,
    NestedDeclarationsPayload, PageMarginPayload, PageRulePayload, PositionTryRulePayload,
    RuleRecord, ViewportRulePayload,
};
use rocketcss_common::vec::Vec as ArenaVec;

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
            $name:literal: $property:ident($value:ty $(, $vendor_prefix:ty)?)
                $([$strategy:ident $( : $($strategy_args:tt)+)?])?,
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
    assert!(
        size_of::<DeclarationRecord<'static, DeclarationPayload<'static>>>() <= 56,
        "direct declaration links must not grow the production declaration record"
    );
    assert!(
        size_of::<DeclarationBlockRecord<'static, CssRulePayload<'static>>>() <= 28,
        "direct declaration links must not grow the production declaration block record"
    );
    println!("{:<56} {:>4} {:>5}", "type", "size", "align");
    println!("{}", "-".repeat(68));
    print_sizes!(
        Atom<'static>,
        rocketcss_common::AstStr<'static>,
        Option<rocketcss_common::AstStr<'static>>,
        Option<AstVec<'static, rocketcss_common::AstStr<'static>>>,
        NodePayload,
        ExtraData,
        NodeId<'static, Token<'static>>,
        rocketcss_common::boxed::Box<'static, u8>,
        ArenaVec<'static, u8>,
        AstVec<'static, u8>,
        RuleRecord<'static, CssRulePayload<'static>>,
        DeclarationBlockRecord<'static, CssRulePayload<'static>>,
        DeclarationRecord<'static, DeclarationPayload<'static>>,
        DeclarationRecord<'static, u8>,
        ScopedDeclarationHandle<'static, 'static>,
        Declaration<'static>,
        PropertyId<'static>,
        TokenOrValue<'static>,
        DashedIdent<'static>,
        AnimationName<'static>,
        Specifier<'static>,
        Variable<'static>,
        Token<'static>,
        Length<'static>,
        LengthValue,
        CssColor<'static>,
        MediaList<'static>,
        MediaQuery<'static>,
        MediaCondition<'static>,
        Selector<'static>,
        SelectorComponent<'static>,
        LABColor,
        PredefinedColor,
        FloatColor,
        UnresolvedColor<'static>,
        QueryFeature<'static, MediaFeatureId>,
        QueryFeature<'static, ContainerSizeFeatureId>,
        QueryFeature<'static, ScrollStateFeatureId>,
        Transition<'static>,
        ImageSetOption<'static>,
        BoxShadow<'static>,
        UnparsedProperty<'static>,
        Function<'static>,
        TextShadow<'static>,
        MatrixForFloat,
        Matrix3DForFloat,
        AttrSelector<'static>,
        EasingFunction,
        Gradient<'static>,
        WebKitGradient<'static>,
        ParsedComponent<'static>,
        AnimationComponent<'static>,
        AnimationTimeline<'static>,
        Filter<'static>,
        Transform<'static>,
        KeyframeSelector,
        WebKitGradientPoint,
        PageSelector<'static>,
        LengthPercentage<'static>,
        AnglePercentage<'static>,
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
        NestedDeclarationsPayload,
        ViewportRulePayload,
        PositionTryRulePayload<'static>,
        PageRulePayload<'static>,
        PageMarginPayload,
        KeyframePayload<'static>,
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
