use super::*;

#[test]
#[ignore]
fn preserves_authored_image_set_fallbacks_without_generating_duplicates() {
    assert_eq!(
        run("a{background-image:image-set(\"a.png\" 1x)}"),
        "a{background-image:image-set(\"a.png\" 1x)}"
    );
    assert_eq!(
        run(
            "a{background-image:-webkit-image-set(\"a.png\" 1x);background-image:image-set(\"a.png\" 1x)}"
        ),
        "a{background-image:-webkit-image-set(\"a.png\" 1x);background-image:image-set(\"a.png\" 1x)}"
    );
}

#[test]
fn dispatches_known_functions_without_repeated_name_matching() {
    assert_eq!(
        run("a{color:RGB(255 0 0);transform:ROTATEZ(1turn)}"),
        "a{color:red;transform:rotateZ(1turn)}"
    );
    assert_eq!(
        run("a{width:-WEBKIT-CALC(3px * 2)}"),
        "a{width:-WEBKIT-CALC(3px*2)}"
    );
}

#[test]
fn orders_duplicate_animation_keywords_as_name_last() {
    // The first value matching a keyword class claims it; a later duplicate is
    // ambiguous and becomes the keyframes name. Flat semantic fields let
    // codegen emit any required default before the colliding name.
    assert_eq!(
        run("a{animation:none 1s linear 2s both}"),
        "a{animation:1s linear 2s none both}"
    );
    assert_eq!(
        run("a{animation:ease 1s linear}"),
        "a{animation:1s ease linear}"
    );
    assert_eq!(
        run("a{animation:running none normal 3 1s 2s linear bounce}"),
        "a{animation:1s linear 2s 3 bounce}"
    );
}

#[test]
fn preserves_invalid_border_and_columns_values_while_ordering() {
    // Extra components of one class keep their relative order instead of
    // being dropped, so invalid values round-trip unchanged.
    assert_eq!(
        run("a{border:0 0 7px 7px solid black}"),
        "a{border:0 0 7px 7px solid black}"
    );
    assert_eq!(run("a{border:solid 0 0 red}"), "a{border:solid 0 0 red}");
    // Values rejected by a typed grammar are opaque to the minifier.
    assert_eq!(run("a{columns:inherit 3rem}"), "a{columns:inherit 3rem}");
    assert_eq!(run("a{columns:3rem 2 12em}"), "a{columns:3rem 2 12em}");
    assert_eq!(run("a{columns:2px 2px}"), "a{columns:2px 2px}");
}

#[test]
fn preserves_invalid_display_values() {
    assert_eq!(run("a{display:}"), "a{display:}");
    assert_eq!(run("a{display:none flow}"), "a{display:none flow}");
    assert_eq!(
        run("a{display:table-cell flow}"),
        "a{display:table-cell flow}"
    );
}

#[test]
fn round_trips_legacy_single_token_display_values() {
    assert_eq!(
        run("a{display:inline-block}b{display:-webkit-inline-box}c{display:-moz-inline-box}"),
        "a{display:inline-block}b{display:-webkit-inline-box}c{display:-moz-inline-box}"
    );
}

#[test]
fn normalizes_flat_animation_when_ordering_is_disabled() {
    let options = MinifyOptions {
        flags: MinifyOptions::default().flags & !Options::ORDER_VALUES,
        ..MinifyOptions::default()
    };
    // A flat shorthand no longer stores authored order, so typed animation
    // values serialize semantically even when the generic ordering pass is off.
    assert_eq!(
        run_with_options("a{animation:3s ease fade}", options),
        "a{animation:3s fade}"
    );
    assert_eq!(
        run_with_options("a{animation:ease 1s linear}", options),
        "a{animation:1s ease linear}"
    );
    // Unparsed values remain lossless because they never enter the flat AST.
    assert_eq!(
        run_with_options("a{animation:ease 1s var(--easing)}", options),
        "a{animation:ease 1s var(--easing)}"
    );
    assert_eq!(
        run_with_options("a{border:solid 1px red}", options),
        "a{border:1px solid red}"
    );
}

#[test]
fn keeps_comments_in_animation_values_on_the_unparsed_path() {
    // The typed component parsers skip comments, so values containing
    // comments stay unparsed where they are retained.
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let stylesheet = parse(
            "a{animation:bounce /*!wow*/ 1s linear}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        assert!(matches!(
            first_property_declaration(&stylesheet),
            Declaration::Unparsed(_)
        ));
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: true },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a {\n  animation: bounce /*!wow*/ 1s linear;\n}\n"
        );
    });
}

#[test]
fn parses_animation_shorthand_into_flat_fields() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let stylesheet = parse(
            "a{animation:3s ease fade}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let Declaration::Animation(animations, _) = first_property_declaration(&stylesheet) else {
            panic!("animation shorthand should use its typed declaration");
        };
        let animation = &animations[0];
        assert!(matches!(&*animation.name, AnimationName::Ident("fade")));
        assert!(matches!(&animation.duration, Time::Seconds(3.0)));
        assert!(matches!(&*animation.timing_function, EasingFunction::Ease));
        assert!(matches!(
            &animation.iteration_count,
            AnimationIterationCount::Number(1.0)
        ));
        assert!(matches!(&animation.direction, AnimationDirection::Normal));
        assert!(matches!(&animation.play_state, AnimationPlayState::Running));
        assert!(matches!(&animation.delay, Time::Seconds(0.0)));
        assert!(matches!(&animation.fill_mode, AnimationFillMode::None));
        assert!(matches!(&*animation.timeline, AnimationTimeline::Auto));
    });

    assert_eq!(run("a{animation:3s ease fade}"), "a{animation:3s fade}");
    // Explicit defaults collapse into the same semantic fields.
    assert_eq!(
        run("a{animation:running none normal 3 1s 2s linear bounce}"),
        "a{animation:1s linear 2s 3 bounce}"
    );
    assert_eq!(
        run("a{animation:1s 2s bounce linear,8s 1s shake ease}"),
        "a{animation:1s linear 2s bounce,8s 1s shake}"
    );
    assert_eq!(
        run("a{-webkit-animation:linear bounce 1s 2s}"),
        "a{-webkit-animation:1s linear 2s bounce}"
    );
    // Timing functions canonicalize through the typed AST.
    assert_eq!(
        run("a{animation:fade 3s cubic-bezier(0.25,0.1,0.25,1)}"),
        "a{animation:3s fade}"
    );
    assert_eq!(
        run("a{animation:fade 3s steps(1, jump-start)}"),
        "a{animation:3s step-start fade}"
    );
    assert_eq!(
        run("a{animation:fade 3s steps(10, end)}"),
        "a{animation:3s steps(10) fade}"
    );
    // Values the typed grammar cannot represent stay unparsed and unordered.
    assert_eq!(
        run("a{animation:ease 1s var(--easing)}"),
        "a{animation:ease 1s var(--easing)}"
    );
}

#[test]
fn keeps_timing_rank_after_timing_function_minification() {
    // A timing function minified to a keyword keeps its rank, so an already
    // canonical shorthand is not reordered past it.
    assert_eq!(
        run("a{transition:color 3s cubic-bezier(0.25,0.1,0.25,1)}"),
        "a{transition:color 3s ease}"
    );
    assert_eq!(
        run("a{transition:color 3s steps(1, start)}"),
        "a{transition:color 3s step-start}"
    );
    assert_eq!(
        run("a{animation:fade 3s cubic-bezier(0.250,1e-1px,0.250,1)}"),
        "a{animation:fade 3s cubic-bezier(0.250,1e-1px,0.250,1)}"
    );
}
