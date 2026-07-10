use std::fmt::{self, Write};

/// Options controlling CSS serialization.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrinterOptions {
    /// Omit optional whitespace and line breaks.
    pub minify: bool,
}

/// Destination and formatting state used by [`ToCss`] implementations.
pub struct Printer<'a, W> {
    dest: &'a mut W,
    options: PrinterOptions,
    indent: usize,
    in_calc: bool,
}

impl<'a, W: Write> Printer<'a, W> {
    #[inline]
    pub fn new(dest: &'a mut W, options: PrinterOptions) -> Self {
        Self {
            dest,
            options,
            indent: 0,
            in_calc: false,
        }
    }

    #[inline]
    pub fn options(&self) -> PrinterOptions {
        self.options
    }

    #[inline]
    pub fn minify(&self) -> bool {
        self.options.minify
    }

    #[inline]
    pub fn write_str(&mut self, value: &str) -> fmt::Result {
        self.dest.write_str(value)
    }

    #[inline]
    pub fn write_char(&mut self, value: char) -> fmt::Result {
        self.dest.write_char(value)
    }

    #[inline]
    pub fn whitespace(&mut self) -> fmt::Result {
        if self.options.minify {
            Ok(())
        } else {
            self.write_char(' ')
        }
    }

    #[inline]
    pub fn delim(&mut self, value: char, whitespace_before: bool) -> fmt::Result {
        if whitespace_before {
            self.whitespace()?;
        }
        self.write_char(value)?;
        self.whitespace()
    }

    pub fn newline(&mut self) -> fmt::Result {
        if self.options.minify {
            return Ok(());
        }

        self.write_char('\n')?;
        for _ in 0..self.indent {
            self.write_char(' ')?;
        }
        Ok(())
    }

    #[inline]
    pub fn indent(&mut self) {
        self.indent += 2;
    }

    #[inline]
    pub fn dedent(&mut self) {
        self.indent -= 2;
    }

    #[inline]
    pub(crate) fn in_calc(&self) -> bool {
        self.in_calc
    }

    pub(crate) fn with_calc<F>(&mut self, callback: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        let previous = self.in_calc;
        self.in_calc = true;
        let result = callback(self);
        self.in_calc = previous;
        result
    }

    #[inline]
    pub fn write_ident(&mut self, ident: &str) -> fmt::Result {
        cssparser::serialize_identifier(ident, self)
    }

    #[inline]
    pub fn write_name(&mut self, name: &str) -> fmt::Result {
        cssparser::serialize_name(name, self)
    }

    #[inline]
    pub fn write_string(&mut self, value: &str) -> fmt::Result {
        cssparser::serialize_string(value, self)
    }
}

impl<W: Write> Write for Printer<'_, W> {
    #[inline]
    fn write_str(&mut self, value: &str) -> fmt::Result {
        self.dest.write_str(value)
    }

    #[inline]
    fn write_char(&mut self, value: char) -> fmt::Result {
        self.dest.write_char(value)
    }
}

/// Serializes a syntax-tree node as CSS.
pub trait ToCss {
    fn to_css<W: Write>(&self, dest: &mut Printer<'_, W>) -> fmt::Result;

    #[inline]
    fn to_css_string(&self, options: PrinterOptions) -> Result<String, fmt::Error> {
        let mut output = String::new();
        self.to_css(&mut Printer::new(&mut output, options))?;
        Ok(output)
    }
}

impl<T: ToCss + ?Sized> ToCss for &T {
    #[inline]
    fn to_css<W: Write>(&self, dest: &mut Printer<'_, W>) -> fmt::Result {
        (*self).to_css(dest)
    }
}

impl<T: ToCss> ToCss for Option<T> {
    #[inline]
    fn to_css<W: Write>(&self, dest: &mut Printer<'_, W>) -> fmt::Result {
        if let Some(value) = self {
            value.to_css(dest)?;
        }
        Ok(())
    }
}

pub(crate) fn serialize_number<W: Write>(value: f32, dest: &mut Printer<'_, W>) -> fmt::Result {
    let output = value.to_string();
    if value != 0.0 && value.abs() < 1.0 {
        if value.is_sign_negative() {
            dest.write_char('-')?;
            dest.write_str(output.trim_start_matches('-').trim_start_matches('0'))
        } else {
            dest.write_str(output.trim_start_matches('0'))
        }
    } else {
        dest.write_str(&output)
    }
}

pub(crate) fn serialize_dimension<W: Write>(
    value: f32,
    unit: &str,
    dest: &mut Printer<'_, W>,
) -> fmt::Result {
    serialize_number(value, dest)?;
    dest.write_str(unit)
}

pub(crate) fn serialize_debug_keyword<T: fmt::Debug, W: Write>(
    value: &T,
    dest: &mut Printer<'_, W>,
) -> fmt::Result {
    let debug = format!("{value:?}");
    let debug = debug.strip_suffix('_').unwrap_or(&debug);
    let characters: Vec<_> = debug.chars().collect();
    for (index, character) in characters.iter().copied().enumerate() {
        if character.is_ascii_uppercase()
            && index > 0
            && (characters[index - 1].is_ascii_lowercase()
                || characters[index - 1].is_ascii_digit()
                || characters
                    .get(index + 1)
                    .is_some_and(char::is_ascii_lowercase))
        {
            dest.write_char('-')?;
        }
        dest.write_char(character.to_ascii_lowercase())?;
    }
    Ok(())
}

impl<'a, T: ToCss> ToCss for rs_css_allocator::boxed::Box<'a, T> {
    #[inline]
    fn to_css<W: Write>(&self, dest: &mut Printer<'_, W>) -> fmt::Result {
        (**self).to_css(dest)
    }
}
