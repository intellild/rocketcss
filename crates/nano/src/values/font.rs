use rocketcss_ast::{AbsoluteFontWeight, FontFamily, FontWeight};

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl Minify for FontWeight {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if !cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::Any) {
            return;
        }
        let weight = match self {
            Self::Absolute(AbsoluteFontWeight::Normal) => 400.0,
            Self::Absolute(AbsoluteFontWeight::Bold) => 700.0,
            _ => return,
        };
        *self = Self::Absolute(AbsoluteFontWeight::Weight(weight));
        cx.record_value_normalized();
    }
}

pub(crate) fn minify_font_families(
    families: &mut [FontFamily<'_>],
    cx: &mut MinifyContext<'_>,
    ast: &rocketcss_ast::AstContext<'_>,
) {
    for family in families.iter_mut() {
        if matches!(family, FontFamily::Unparsed(_)) {
            *family = FontFamily::Tombstone;
            cx.record_value_normalized();
        }
    }

    if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::Any)
        && let Some(generic) = families.iter().position(FontFamily::is_generic)
        && families[..generic]
            .iter()
            .any(|family| !family.is_tombstone())
        && families[generic + 1..]
            .iter()
            .any(|family| !family.is_tombstone())
    {
        for family in &mut families[generic + 1..] {
            if !family.is_tombstone() {
                *family = FontFamily::Tombstone;
                cx.record_value_normalized();
            }
        }
    }

    if cx.is_enabled(Options::DEDUPLICATE_LISTS, OptionsOp::None) {
        return;
    }

    for current in 1..families.len() {
        if families[current].is_tombstone() {
            continue;
        }
        let duplicate = families[..current]
            .iter()
            .filter(|previous| !previous.is_tombstone())
            .any(|previous| equivalent(previous, &families[current], ast));
        if duplicate {
            families[current] = FontFamily::Tombstone;
            cx.record_value_normalized();
        }
    }
}

fn equivalent(
    left: &FontFamily<'_>,
    right: &FontFamily<'_>,
    ast: &rocketcss_ast::AstContext<'_>,
) -> bool {
    match (left, right) {
        (FontFamily::Custom(left), FontFamily::Custom(right)) => {
            ast.str(*left).eq_ignore_ascii_case(ast.str(*right))
        }
        _ => left == right,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_ast::{AstContext, DUMMY_SP};
    use rocketcss_common::Allocator;

    #[test]
    fn custom_family_comparison_resolves_ranges_without_erasing_variants() {
        let allocator = Allocator::new();
        let mut ast =
            AstContext::with_source_in(&allocator, "Inter Inter INTER", Default::default());
        let first = ast.string_pool().source_range(0, 5);
        let second = ast.string_pool().source_range(6, 11);
        let upper = ast.string_pool().source_range(12, 17);
        let extra = ast.add_str("Inter");
        assert_ne!(first, second);
        assert_ne!(first, extra);
        let a = ast.alloc_node(FontFamily::Custom(first), DUMMY_SP);
        let b = ast.alloc_node(FontFamily::Custom(second), DUMMY_SP);
        let c = ast.alloc_node(FontFamily::Custom(upper), DUMMY_SP);
        assert!(ast.nodes_eq(a, b));
        // Structural string equality remains case-sensitive; family deduplication is not.
        assert!(!ast.nodes_eq(a, c));
        let serif = FontFamily::Custom(ast.add_str("serif"));
        let upper_non_ascii = FontFamily::Custom(ast.add_str("Ä"));
        let lower_non_ascii = FontFamily::Custom(ast.add_str("ä"));
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        let interned = ast.string_pool().len();
        for _ in 0..3 {
            for range in [second, upper, extra] {
                assert!(equivalent(
                    &FontFamily::Custom(first),
                    &FontFamily::Custom(range),
                    &ast
                ));
            }
            assert!(!equivalent(&serif, &FontFamily::Serif, &ast));
            assert!(!equivalent(&upper_non_ascii, &lower_non_ascii, &ast));
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
        assert_eq!(ast.string_pool().len(), interned);
    }
}
