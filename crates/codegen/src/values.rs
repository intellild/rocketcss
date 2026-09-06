use crate::prelude::*;

macro_rules! keyword_values {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'ghost> ToCss<'ghost> for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, 'ghost>) -> fmt::Result {
                    dest.write_str(self.as_css_str().expect("keyword enum has only static variants"))
                }
            }
        )+
    };
}

fn write_function_values<PrinterT: PrinterTrait, F>(
    name: &str,
    dest: &mut PrinterT,
    callback: F,
) -> fmt::Result
where
    F: FnOnce(&mut PrinterT) -> fmt::Result,
{
    dest.write_str(name)?;
    dest.write_char('(')?;
    callback(dest)?;
    dest.write_char(')')
}

fn write_ident_list<PrinterT, I>(values: I, dest: &mut PrinterT) -> fmt::Result
where
    PrinterT: PrinterTrait,
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for (index, value) in values.into_iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        serialize_identifier(value.as_ref(), dest)?;
    }
    Ok(())
}

pub(crate) mod alignment;
pub(crate) mod animation;
pub(crate) mod border;
pub(crate) mod box_model;
pub(crate) mod container;
pub(crate) mod filter;
pub(crate) mod flex;
pub(crate) mod font;
pub(crate) mod grid;
pub(crate) mod image;
pub(crate) mod list;
pub(crate) mod mask;
pub(crate) mod property;
pub(crate) mod shape;
pub(crate) mod svg;
pub(crate) mod text;
pub(crate) mod transform;
pub(crate) mod ui;
pub(crate) mod view_transition;
