mod keyframes;
pub(crate) mod layout;
mod property;
mod stylesheet;

pub(crate) use stylesheet::{DeclarationBlockMinifier, minify_function, normalize_url_text};
