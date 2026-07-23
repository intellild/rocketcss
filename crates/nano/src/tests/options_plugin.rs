use super::*;

use crate::MinifyStats;
use crate::context;
use crate::properties;
use rocketcss_ast::{PropertyId, VendorPrefix};

#[test]
fn option_operations_are_explicit() {
    let options = MinifyOptions::default();
    assert!(options.is_enabled(
        Options::NORMALIZE_VALUES | Options::NORMALIZE_WHITESPACE,
        OptionsOp::And,
    ));
    assert!(options.is_enabled(
        Options::NORMALIZE_VALUES | Options::NORMALIZE_URLS,
        OptionsOp::Any,
    ));
    assert!(options.is_enabled(Options::NORMALIZE_URLS, OptionsOp::None));
}

#[test]
fn property_context_dispatches_by_property_id() {
    let animation = PropertyId::Animation(VendorPrefix::WEBKIT);
    assert_eq!(
        properties::value_context(&animation, true, true).property,
        context::PropertyContext::Animation
    );
    assert_eq!(
        properties::value_context(&animation, false, true).property,
        context::PropertyContext::TimingFunction
    );

    let border = PropertyId::Border;
    assert_eq!(
        properties::value_context(&border, true, true).property,
        context::PropertyContext::Border
    );
    assert_eq!(
        properties::value_context(&border, false, true).property,
        context::PropertyContext::Generic
    );

    let columns = PropertyId::from_name("CoLuMnS");
    assert_eq!(columns, PropertyId::Columns(VendorPrefix::NONE));
    assert_eq!(
        properties::value_context(&columns, true, true).property,
        context::PropertyContext::Columns
    );
    assert_eq!(
        properties::value_context(&columns, false, true).property,
        context::PropertyContext::Generic
    );

    let prefixed_animation = PropertyId::from_name("-WebKit-ANIMATION");
    assert_eq!(
        prefixed_animation,
        PropertyId::Animation(VendorPrefix::WEBKIT)
    );
    assert_eq!(
        prefixed_animation
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "-webkit-animation"
    );
    assert_eq!(
        properties::value_context(&prefixed_animation, true, true).property,
        context::PropertyContext::Animation
    );
    assert_eq!(
        properties::value_context(&prefixed_animation, false, true).property,
        context::PropertyContext::TimingFunction
    );
}

#[test]
fn plugin_exposes_local_normalization_stats() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            "a{width:16px;width:16px}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let mut plugins = Plugins::new();
        plugins.add(MinifyPlugin::default());
        let mut plugin_context = PluginContext::new(&allocator, &mut token);
        plugins.run(&mut stylesheet, &mut plugin_context).unwrap();
        let stats = plugin_context.get::<MinifyStats>().unwrap();
        assert_eq!(stats.values_normalized, 2);
        assert_eq!(stats.declarations_removed, 1);
    });
}
