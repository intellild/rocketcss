use rs_css_ast::KeyframeSelector;

/// Normalizes one keyframe selector without inspecting sibling keyframes.
pub(crate) fn minify_keyframe_selector(selector: &mut KeyframeSelector<'_>) -> bool {
    match selector {
        KeyframeSelector::From => {
            *selector = KeyframeSelector::Percentage(0.0);
            true
        }
        KeyframeSelector::Percentage(value) if *value == 1.0 => {
            *selector = KeyframeSelector::To;
            true
        }
        _ => false,
    }
}
