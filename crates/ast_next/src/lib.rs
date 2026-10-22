//! Flattened CSS syntax tree.
//!
//! Every syntax node is a four-byte [`NodeId`] handle. Node data lives in the
//! owning [`Ast`] context, which stores spans, kinds, and payloads in parallel
//! columns. Non-node values such as keywords and operators remain plain Rust
//! values stored inline in node data.

mod compilation;
mod context;
mod impls;
mod impls_values;
mod known_colors;
mod properties_impl;
mod span;
mod storage;

pub use compilation::Compilation;
pub use context::Ast;
pub use span::{DUMMY_SP, GetSpan, SetSpan, Span};
pub use storage::{NodeId, NodeKind, NodeStorage};

pub(crate) use storage::NodePayload;

pub use rocketcss_common::Atom;
pub use rocketcss_macros::CssKeyword;

/// Matches a value against ASCII case-insensitive string literal arms.
#[macro_export]
macro_rules! match_ignore_ascii_case {
    (
        $value:expr,
        $($($expected:literal)|+ => $result:expr,)+
        _ => $fallback:expr $(,)?
    ) => {{
        let value = $value;
        $(
            if $(value.eq_ignore_ascii_case($expected))||+ {
                $result
            } else
        )+
        {
            $fallback
        }
    }};
}

mod generated;
pub use generated::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_common::Allocator;

    #[test]
    fn allocates_inline_and_spilled_nodes_with_stable_handles() {
        let allocator = Allocator::new();
        let mut ast = Ast::new_in(&allocator);

        let url = ast.alloc_url(UrlData { url: "a.css" }, DUMMY_SP);
        assert_eq!(url.url(&ast), "a.css");
        assert_eq!(url.span(&ast), DUMMY_SP);

        let function = ast.alloc_function(
            FunctionData {
                arguments: &[],
                flags: FunctionFlags::default(),
                kind: KnownFunction::default(),
                name: "calc",
                replacement: None,
            },
            Span::new(1, 5),
        );
        assert_eq!(function.name(&ast), "calc");
        assert!(function.arguments(&ast).is_empty());
        assert_eq!(function.span(&ast), Span::new(1, 5));
    }

    #[test]
    fn builds_a_stylesheet_rule_tree() {
        let allocator = Allocator::new();
        let mut ast = Ast::new_in(&allocator);

        let declarations = ast.alloc_slice::<DeclarationPayload<'_>>(&[]);
        let block = ast.alloc_declaration_block(
            DeclarationBlockData { declarations },
            DUMMY_SP,
        );
        let selectors = ast.alloc_slice::<Selector<'_>>(&[]);
        let style = ast.alloc_style_rule(
            StyleRuleData {
                selectors,
                vendor_prefix: VendorPrefix::default(),
            },
            DUMMY_SP,
        );
        let payload = ast.alloc_css_rule_payload(CssRulePayloadData::Style(style), DUMMY_SP);
        let children = ast.alloc_slice::<CssRule<'_>>(&[]);
        let rule = ast.alloc_css_rule(
            CssRuleData {
                payload,
                declarations: Some(block),
                children,
            },
            DUMMY_SP,
        );
        let rules = ast.alloc_slice(&[rule]);
        let style_sheet = ast.alloc_style_sheet(StyleSheetData { rules }, DUMMY_SP);
        let compilation = Compilation::new(ast, style_sheet);

        assert_eq!(compilation.style_sheet().rules(compilation.ast()).len(), 1);
        let rule = compilation.style_sheet().rules(compilation.ast())[0];
        assert!(rule.declarations(compilation.ast()).is_some());
        assert!(rule
            .payload(compilation.ast())
            .owns_property_declarations(compilation.ast()));
    }
}
