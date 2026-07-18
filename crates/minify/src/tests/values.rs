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
