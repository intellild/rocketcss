use super::*;

#[test]
#[ignore]
fn preserves_three_dimensional_zero_translation() {
    assert_eq!(
        run("a{transform:translate3d(0,0,0)}"),
        "a{transform:translateZ(0)}"
    );
}

#[test]
#[ignore]
fn preserves_individual_transform_properties_as_independent_declarations() {
    assert_eq!(
        run(".foo{scale:1.5;translate:1rem;transform:skew(-25deg)}"),
        ".foo{scale:1.5;translate:1rem;transform:skew(-25deg)}"
    );
    assert_eq!(
        run(".bar{transform:skew(-25deg);scale:1.5;translate:1rem}"),
        ".bar{transform:skew(-25deg);scale:1.5;translate:1rem}"
    );
    assert_eq!(
        run(
            ".bar{transition:scale .3s linear;transform:skew(10deg);scale:1.5;translate:1em}.bar:hover{scale:2}"
        ),
        ".bar{transition:scale .3s linear;transform:skew(10deg);scale:1.5;translate:1em}.bar:hover{scale:2}"
    );
}
