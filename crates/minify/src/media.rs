use rocketcss_ast::MediaList;

use crate::{Minify, MinifyContext};

impl Minify for MediaList<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if context.options().deduplicate_lists {
            let before = self.media_queries.len();
            let mut index = 0;
            while index < self.media_queries.len() {
                if self.media_queries[..index]
                    .iter()
                    .any(|query| query == &self.media_queries[index])
                {
                    self.media_queries.remove(index);
                } else {
                    index += 1;
                }
            }
            if self.media_queries.len() != before {
                context.record_value_normalized();
            }
        }
    }
}
