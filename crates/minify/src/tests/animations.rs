use super::*;

#[test]
fn preserves_partial_animation_shorthand_with_infinite_iteration_count() {
    assert_eq!(run(".foo{animation:infinite}"), ".foo{animation:infinite}");
}

#[test]
fn avoids_expanding_transition_shorthand_and_property() {
    assert_eq!(
        run(
            ".foo{transition:all .2s cubic-bezier(.4,0,.2,1);transition-property:height,width,transform,max-width,left,right,top,bottom,box-shadow}"
        ),
        ".foo{transition:all .2s cubic-bezier(.4,0,.2,1);transition-property:height,width,transform,max-width,left,right,top,bottom,box-shadow}"
    );
}

#[test]
fn preserves_cascade_sensitive_declaration_order() {
    assert_eq!(
        run(".item{animation:fade both;animation-timeline:scroll(root block)}"),
        ".item{animation:fade both;animation-timeline:scroll(root block)}"
    );
    assert_eq!(
        run(
            ".header{height:1px;height:var(--header-height);block-size:auto;block-size:calc-size(auto)}"
        ),
        ".header{height:1px;height:var(--header-height);block-size:auto;block-size:calc-size(auto)}"
    );
    assert_eq!(
        run(
            ".foo{animation:linear foo;animation-timeline:view();animation-range:entry-crossing 1% exit-crossing 100%}"
        ),
        ".foo{animation:foo linear;animation-timeline:view();animation-range:entry-crossing 1% exit-crossing 100%}"
    );
}

#[test]
fn preserves_scroll_driven_animation_duration_auto_semantics() {
    const SOURCE: &str = ".overflowContainer{animation:--keyframes-top-scroll-border step-end,--keyframes-bottom-scroll-border step-end reverse;animation-timeline:scroll(self)}";
    let output = run(SOURCE);
    assert!(output.contains("animation:--keyframes-top-scroll-border step-end,--keyframes-bottom-scroll-border step-end reverse"));
    assert!(output.contains("animation-timeline:scroll(self)"));
    assert!(!output.contains("animation-duration"));
}
