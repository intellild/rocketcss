use rocketcss_ast::{Compilation, MediaList};

use crate::{MinifyContext, Options, OptionsOp};

pub(crate) fn minify_media_list(
    media: &mut MediaList<'_>,
    context: &mut MinifyContext<'_>,
    ast: &Compilation<'_>,
) {
    if context.is_enabled(Options::DEDUPLICATE_LISTS, OptionsOp::Any) {
        let before = media.media_queries.len();
        let mut index = 0;
        while index < media.media_queries.len() {
            if media.media_queries[..index].iter().any(|query| {
                crate::equality::css_values_are_equal(
                    ast,
                    ast.resolve_node(*query),
                    ast.resolve_node(media.media_queries[index]),
                )
            }) {
                media.media_queries.remove(index);
            } else {
                index += 1;
            }
        }
        if media.media_queries.len() != before {
            context.record_value_normalized();
        }
    }
}
