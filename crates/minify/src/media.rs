use rs_css_ast::MediaList;

use crate::MinifyContext;

pub(crate) fn minify_media_list<'a>(media: &mut MediaList<'a>, context: &mut MinifyContext<'a>) {
    if context.options().discard_duplicates {
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
