use rocketcss_ast::{MediaList, VisitMutContext};

use crate::{MinifyContext, Options, OptionsOp};

pub(crate) fn minify_media_list<'ast>(
    media: &mut MediaList<'ast>,
    context: &mut MinifyContext<'_>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    if context.is_enabled(Options::DEDUPLICATE_LISTS, OptionsOp::Any) {
        let mut changed = false;
        ast.rewrite_vec(&mut media.media_queries, |media_queries, ast| {
            let before = media_queries.len();
            let mut index = 0;
            while index < media_queries.len() {
                if media_queries[..index].iter().any(|query| {
                    crate::equality::css_values_are_equal(
                        ast.ast_context(),
                        &ast.ast_context().resolve_node(*query),
                        &ast.ast_context().resolve_node(media_queries[index]),
                    )
                }) {
                    media_queries.remove(index);
                } else {
                    index += 1;
                }
            }
            changed = media_queries.len() != before;
        });
        if changed {
            context.record_value_normalized();
        }
    }
}
