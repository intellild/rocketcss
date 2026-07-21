use super::*;

#[test]
fn merges_column_longhands_and_folds_overrides() {
    assert_eq!(
        run("h1{column-width:12em;column-count:auto}"),
        "h1{columns:12em}"
    );
    assert_eq!(
        run("h1{column-count:3;column-width:6em}"),
        "h1{columns:6em 3}"
    );
    assert_eq!(
        run("h1{columns:12em auto;column-width:11em}"),
        "h1{columns:11em}"
    );
    assert_eq!(
        run("h1{column-width:12em;column-count:auto;columns:12em}"),
        "h1{columns:12em}"
    );
}

#[test]
fn merges_case_insensitive_and_css_wide_column_values() {
    assert_eq!(
        run("h1{COLUMN-WIDTH:6em;COLUMN-COUNT:3}"),
        "h1{columns:6em 3}"
    );
    assert_eq!(
        run("h1{COLUMN-WIDTH:INHERIT;COLUMN-COUNT:INHERIT}"),
        "h1{columns:inherit}"
    );
    assert_eq!(
        run("h1{column-width:inherit;column-count:initial}"),
        "h1{column-width:inherit;column-count:initial}"
    );
    assert_eq!(
        run("h1{-webkit-column-width:INHERIT;-webkit-column-count:INHERIT}"),
        "h1{-webkit-columns:inherit}"
    );
}

#[test]
fn preserves_column_fallbacks_hacks_and_importance() {
    assert_eq!(
        run("h1{column-width:12em;column-width:var(--variable)}"),
        "h1{column-width:12em;column-width:var(--variable)}"
    );
    assert_eq!(
        run("h1{column-width:12em;_column-count:auto}"),
        "h1{column-width:12em;_column-count:auto}"
    );
    assert_eq!(
        run("h1{column-width:12em!important;column-count:auto}"),
        "h1{column-width:12em !important;column-count:auto}"
    );
}
