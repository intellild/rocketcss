//! CSS syntax tree data structures ported from lightningcss.
//!
//! This crate intentionally contains data definitions only. Parsing, printing,
//! transformation, and minification logic stays outside of the AST crate.

#![allow(non_camel_case_types)]

use rs_css_allocator::{boxed::Box, vec::Vec};

mod common;
mod properties;
mod rules;
mod selector;
mod values;

pub use common::*;
pub use properties::*;
pub use rules::*;
pub use selector::*;
pub use values::*;
