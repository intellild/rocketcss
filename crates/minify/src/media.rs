use rs_css_ast::MediaList;

use crate::MinifyContext;

pub(crate) fn minify_media_list(media: &mut MediaList<'_>, context: &mut MinifyContext) {
    if context.options().deduplicate_lists {
        let before = media.media_queries.len();
        let mut index = 0;
        while index < media.media_queries.len() {
            if media.media_queries[..index]
                .iter()
                .any(|query| query == &media.media_queries[index])
            {
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
