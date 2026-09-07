use rocketcss_ast::{PropertyId, PropertySupport};
use std::{collections::BTreeSet, fs, path::Path};

macro_rules! property_names {
    (
        $(
            $(#[$meta:meta])*
            $name:literal: $property:ident($value:ty $(, $vp:ty)?)
                [$strategy:ident $( : $($strategy_args:tt)+)?],
        )+
    ) => {
        &[$($name),+]
    };
}

const ROCKETCSS_PROPERTY_NAMES: &[&str] = rocketcss_ast::for_each_property!(property_names);

// These are Stylo-only declarations that are either Gecko/Servo internals,
// generated helper sections, or newer specifications absent from the local
// Lightning CSS property inventory. This is a test-only audit allowlist, not
// a compiler property registry. A new Stylo-only name must be reviewed and
// added here with the same explicit out-of-scope rationale before the audit
// can pass.
const STYLO_OUT_OF_SCOPE: &[&str] = &[
    "-moz-box-align",
    "-moz-box-collapse",
    "-moz-box-direction",
    "-moz-box-flex",
    "-moz-box-ordinal-group",
    "-moz-box-orient",
    "-moz-box-pack",
    "-moz-context-properties",
    "-moz-control-character-visibility",
    "-moz-default-appearance",
    "-moz-float-edge",
    "-moz-force-broken-image-icon",
    "-moz-image-decoding",
    "-moz-inert",
    "-moz-math-variant",
    "-moz-min-font-size-ratio",
    "-moz-orient",
    "-moz-osx-font-smoothing",
    "-moz-subtree-hidden-only-visually",
    "-moz-text-size-adjust",
    "-moz-theme",
    "-moz-top-layer",
    "-moz-user-focus",
    "-moz-window-dragging",
    "-moz-window-input-region-margin",
    "-moz-window-opacity",
    "-moz-window-shadow",
    "-moz-window-transform",
    "-servo-top-layer",
    "-webkit-line-clamp",
    "-webkit-text-fill-color",
    "-webkit-text-security",
    "-webkit-text-stroke",
    "-webkit-text-stroke-color",
    "-webkit-text-stroke-width",
    "-x-lang",
    "-x-span",
    "-x-text-scale",
    "alignment-baseline",
    "anchor-name",
    "anchor-scope",
    "background-blend-mode",
    "baseline-shift",
    "baseline-source",
    "border-collapse",
    "break-after",
    "break-before",
    "break-inside",
    "caption-side",
    "clip",
    "column-fill",
    "column-rule-color",
    "column-rule-style",
    "column-rule-width",
    "column-span",
    "contain",
    "contain-intrinsic-block-size",
    "contain-intrinsic-height",
    "contain-intrinsic-inline-size",
    "contain-intrinsic-size",
    "contain-intrinsic-width",
    "content-visibility",
    "corner-block-end-shape",
    "corner-block-start-shape",
    "corner-bottom-left-shape",
    "corner-bottom-right-shape",
    "corner-bottom-shape",
    "corner-end-end-shape",
    "corner-end-start-shape",
    "corner-inline-end-shape",
    "corner-inline-start-shape",
    "corner-left-shape",
    "corner-right-shape",
    "corner-shape",
    "corner-start-end-shape",
    "corner-start-start-shape",
    "corner-top-left-shape",
    "corner-top-right-shape",
    "corner-top-shape",
    "counter-increment",
    "counter-reset",
    "counter-set",
    "cx",
    "cy",
    "d",
    "dominant-baseline",
    "empty-cells",
    "field-sizing",
    "flood-color",
    "flood-opacity",
    "font-feature-settings",
    "font-kerning",
    "font-language-override",
    "font-optical-sizing",
    "font-size-adjust",
    "font-synthesis",
    "font-synthesis-position",
    "font-synthesis-small-caps",
    "font-synthesis-style",
    "font-synthesis-weight",
    "font-variant",
    "font-variant-alternates",
    "font-variant-east-asian",
    "font-variant-emoji",
    "font-variant-ligatures",
    "font-variant-numeric",
    "font-variant-position",
    "font-variation-settings",
    "forced-color-adjust",
    "hyphenate-character",
    "hyphenate-limit-chars",
    "image-orientation",
    "ime-mode",
    "initial-letter",
    "isolation",
    "lighting-color",
    "link-parameters",
    "masonry-auto-flow",
    "math-depth",
    "math-shift",
    "math-style",
    "offset",
    "offset-anchor",
    "offset-distance",
    "offset-path",
    "offset-position",
    "offset-rotate",
    "outline-offset",
    "overflow-anchor",
    "overflow-block",
    "overflow-clip-margin",
    "overflow-inline",
    "overscroll-behavior",
    "overscroll-behavior-block",
    "overscroll-behavior-inline",
    "overscroll-behavior-x",
    "overscroll-behavior-y",
    "page",
    "page-break-after",
    "page-break-before",
    "page-break-inside",
    "page-orientation",
    "paint-order",
    "position-anchor",
    "position-area",
    "position-try",
    "position-try-fallbacks",
    "position-try-order",
    "position-visibility",
    "quotes",
    "r",
    "ruby-align",
    "ruby-position",
    "rx",
    "ry",
    "scroll-snap-align",
    "scroll-snap-stop",
    "scroll-snap-type",
    "scroll-timeline",
    "scroll-timeline-axis",
    "scroll-timeline-name",
    "scrollbar-gutter",
    "scrollbar-width",
    "shape-image-threshold",
    "shape-margin",
    "shape-outside",
    "size",
    "stop-color",
    "stop-opacity",
    "table-layout",
    "text-anchor",
    "text-autospace",
    "text-box",
    "text-box-edge",
    "text-box-trim",
    "text-combine-upright",
    "text-decoration-inset",
    "text-orientation",
    "text-underline-offset",
    "text-underline-position",
    "text-wrap",
    "text-wrap-mode",
    "text-wrap-style",
    "timeline-scope",
    "transition-behavior",
    "vector-effect",
    "view-timeline",
    "view-timeline-axis",
    "view-timeline-inset",
    "view-timeline-name",
    "white-space-collapse",
    "will-change",
    "writing-mode",
    "x",
    "y",
    "zoom",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Classification {
    Typed,
    Unsupported,
    OutOfScope,
}

fn repository_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

fn read_upstream(relative_path: &str) -> Option<String> {
    let path = repository_root().join(relative_path);
    match fs::read_to_string(&path) {
        Ok(source) => Some(source),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!(
                "skipping upstream inventory audit; missing {}",
                path.display()
            );
            None
        }
        Err(error) => panic!("failed to read {}: {error}", path.display()),
    }
}

fn lightning_names(source: &str) -> BTreeSet<String> {
    let start = source
        .find("define_properties! {")
        .expect("Lightning CSS property metadata marker");
    let source = &source[start..source[start..].find("\n}\n\nimpl").unwrap() + start];
    quoted_property_names(source)
}

fn quoted_property_names(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            let line = line.strip_prefix('"')?;
            let end = line.find('"')?;
            let name = &line[..end];
            line[end + 1..]
                .trim_start()
                .starts_with(':')
                .then_some(name)
        })
        .map(str::to_owned)
        .collect()
}

fn stylo_names(longhands: &str, shorthands: &str) -> BTreeSet<String> {
    longhands
        .lines()
        .chain(shorthands.lines())
        .filter_map(|line| {
            let line = line.trim();
            let name = line.strip_prefix('[')?.strip_suffix(']')?;
            (!name.is_empty() && !name.contains('.') && !name.starts_with('['))
                .then_some(name.to_owned())
        })
        .collect()
}

fn classify(name: &str, stylo: bool) -> Result<Classification, String> {
    match PropertyId::from_known_name(name)
        .map_or(PropertySupport::Custom, |id| id.support_classification())
    {
        PropertySupport::Typed => Ok(Classification::Typed),
        PropertySupport::UnsupportedGrammar => Ok(Classification::Unsupported),
        PropertySupport::Custom if stylo && STYLO_OUT_OF_SCOPE.contains(&name) => {
            Ok(Classification::OutOfScope)
        }
        PropertySupport::Custom => Err(format!(
            "{name} is missing from RocketCSS metadata and has no explicit audit allowlist reason"
        )),
    }
}

fn report(label: &str, names: &BTreeSet<String>, stylo: bool) {
    let mut typed = 0;
    let mut unsupported = 0;
    let mut out_of_scope = 0;
    for name in names {
        match classify(name, stylo).unwrap_or_else(|error| panic!("{label}: {error}")) {
            Classification::Typed => typed += 1,
            Classification::Unsupported => unsupported += 1,
            Classification::OutOfScope => out_of_scope += 1,
        }
    }
    println!(
        "{label}: total={}, typed={typed}, unsupported={unsupported}, out_of_scope={out_of_scope}",
        names.len()
    );
}

#[test]
fn audits_property_metadata_against_lightning_and_stylo_inventories() {
    let Some(lightning) = read_upstream("../lightningcss/src/properties/mod.rs") else {
        return;
    };
    let Some(stylo_longhands) = read_upstream("../stylo/style/properties/longhands.toml") else {
        return;
    };
    let Some(stylo_shorthands) = read_upstream("../stylo/style/properties/shorthands.toml") else {
        return;
    };

    let rocket_names = ROCKETCSS_PROPERTY_NAMES
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        rocket_names.len(),
        ROCKETCSS_PROPERTY_NAMES.len(),
        "RocketCSS metadata contains duplicate property names"
    );
    for name in &rocket_names {
        assert!(
            PropertyId::from_known_name(name).is_some(),
            "RocketCSS metadata entry {name} does not resolve to a known PropertyId"
        );
    }

    let lightning_names = lightning_names(&lightning);
    let stylo_names = stylo_names(&stylo_longhands, &stylo_shorthands);
    for name in &lightning_names {
        classify(name, false).unwrap_or_else(|error| panic!("Lightning CSS: {error}"));
    }
    for name in &stylo_names {
        classify(name, true).unwrap_or_else(|error| panic!("Stylo: {error}"));
    }

    for name in STYLO_OUT_OF_SCOPE {
        assert!(
            stylo_names.contains(*name),
            "stale Stylo out-of-scope allowlist entry: {name}"
        );
    }

    report("Lightning CSS", &lightning_names, false);
    report("Stylo", &stylo_names, true);
}
