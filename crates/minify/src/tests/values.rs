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
        "a{color:red;transform:rotate(1turn)}"
    );
    assert_eq!(
        run("a{width:-WEBKIT-CALC(3px * 2)}"),
        "a{width:-WEBKIT-CALC(3px*2)}"
    );
}

#[test]
fn orders_duplicate_animation_keywords_as_name_last() {
    // The first value matching a keyword class claims it; a later duplicate is
    // ambiguous and becomes the keyframes name. A colliding name is printed
    // last so the output reparses to the same components.
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
        "a{animation:bounce 1s linear 2s 3 normal none running}"
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
    assert_eq!(run("a{border:solid 0 0 red}"), "a{border:0 0 solid red}");
    // `columns` values the typed parser rejects fall back to token ordering.
    assert_eq!(run("a{columns:inherit 3rem}"), "a{columns:3rem inherit}");
    assert_eq!(run("a{columns:3rem 2 12em}"), "a{columns:3rem 2 12em}");
    assert_eq!(run("a{columns:2px 2px}"), "a{columns:2px 2px}");
}

#[test]
fn leaves_value_order_untouched_when_ordering_is_disabled() {
    let options = MinifyOptions {
        flags: MinifyOptions::default().flags & !Options::ORDER_VALUES,
        ..MinifyOptions::default()
    };
    assert_eq!(
        run_with_options("a{animation:ease 1s var(--easing)}", options),
        "a{animation:ease 1s var(--easing)}"
    );
    assert_eq!(
        run_with_options("a{border:solid 1px red}", options),
        "a{border:solid 1px red}"
    );
}

#[test]
fn parses_animation_shorthand_into_typed_components() {
    assert_eq!(
        run("a{animation:3s ease fade}"),
        "a{animation:fade 3s ease}"
    );
    // Explicit defaults are preserved and printed in canonical order.
    assert_eq!(
        run("a{animation:running none normal 3 1s 2s linear bounce}"),
        "a{animation:bounce 1s linear 2s 3 normal none running}"
    );
    assert_eq!(
        run("a{animation:1s 2s bounce linear,8s 1s shake ease}"),
        "a{animation:bounce 1s linear 2s,shake 8s ease 1s}"
    );
    assert_eq!(
        run("a{-webkit-animation:linear bounce 1s 2s}"),
        "a{-webkit-animation:bounce 1s linear 2s}"
    );
    // Timing functions canonicalize through the typed AST.
    assert_eq!(
        run("a{animation:fade 3s cubic-bezier(0.25,0.1,0.25,1)}"),
        "a{animation:fade 3s ease}"
    );
    assert_eq!(
        run("a{animation:fade 3s steps(1, jump-start)}"),
        "a{animation:fade 3s step-start}"
    );
    assert_eq!(
        run("a{animation:fade 3s steps(10, end)}"),
        "a{animation:fade 3s steps(10)}"
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
        "a{animation:fade 3s ease}"
    );
}
