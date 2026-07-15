use rocketcss_ast::MediaList;

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl Minify for MediaList<'_> {
    fn minify<'cx>(&mut self, context: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if context.is_enabled(Options::DEDUPLICATE_LISTS, OptionsOp::Any) {
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
