use std::fmt::{self, Write};

/// Options controlling CSS serialization.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrinterOptions {
    /// Omit optional whitespace and line breaks.
    pub minify: bool,
}

/// Source-map-independent formatting state shared by printer implementations.
#[derive(Debug, Default)]
pub struct PrinterState {
    indent: usize,
    in_calc: bool,
}

/// Destination and formatting state used by [`ToCss`] implementations.
pub struct Printer<'a, W> {
    dest: &'a mut W,
    options: PrinterOptions,
    state: PrinterState,
}

impl<'a, W: Write> Printer<'a, W> {
    #[inline]
    pub fn new(dest: &'a mut W, options: PrinterOptions) -> Self {
        Self {
            dest,
            options,
            state: PrinterState::default(),
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
        for _ in 0..self.state.indent {
            self.write_char(' ')?;
        }
        Ok(())
    }

    #[inline]
    pub fn indent(&mut self) {
        self.state.indent += 2;
    }

    #[inline]
    pub fn dedent(&mut self) {
        self.state.indent -= 2;
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

mod private {
    pub trait Sealed {}
}

/// Source-map-independent interface used by CSS serialization implementations.
///
/// This trait is sealed so the codegen crate can evolve its concrete writer and
/// source-map backends without exposing those implementation details to AST
/// implementations.
pub trait PrinterTrait: Write + private::Sealed + Sized {
    fn options(&self) -> PrinterOptions;
    fn state(&self) -> &PrinterState;
    fn state_mut(&mut self) -> &mut PrinterState;

    #[inline]
    fn minify(&self) -> bool {
        self.options().minify
    }

    #[inline]
    fn whitespace(&mut self) -> fmt::Result {
        if self.minify() {
            Ok(())
        } else {
            self.write_char(' ')
        }
    }

    #[inline]
    fn delim(&mut self, value: char, whitespace_before: bool) -> fmt::Result {
        if whitespace_before {
            self.whitespace()?;
        }
        self.write_char(value)?;
        self.whitespace()
    }

    fn newline(&mut self) -> fmt::Result {
        if self.minify() {
            return Ok(());
        }

        self.write_char('\n')?;
        let indent = self.state().indent;
        for _ in 0..indent {
            self.write_char(' ')?;
        }
        Ok(())
    }

    #[inline]
    fn indent(&mut self) {
        self.state_mut().indent += 2;
    }

    #[inline]
    fn dedent(&mut self) {
        self.state_mut().indent -= 2;
    }

    #[inline]
    fn in_calc(&self) -> bool {
        self.state().in_calc
    }

    fn with_calc<T>(&mut self, callback: impl FnOnce(&mut Self) -> T) -> T {
        let previous = self.state().in_calc;
        self.state_mut().in_calc = true;
        let result = callback(self);
        self.state_mut().in_calc = previous;
        result
    }

    #[inline]
    fn write_ident(&mut self, ident: &str) -> fmt::Result {
        cssparser::serialize_identifier(ident, self)
    }

    #[inline]
    fn write_name(&mut self, name: &str) -> fmt::Result {
        cssparser::serialize_name(name, self)
    }

    #[inline]
    fn write_string(&mut self, value: &str) -> fmt::Result {
        cssparser::serialize_string(value, self)
    }
}

impl<W: Write> private::Sealed for Printer<'_, W> {}

impl<W: Write> PrinterTrait for Printer<'_, W> {
    #[inline]
    fn options(&self) -> PrinterOptions {
        self.options
    }

    #[inline]
    fn state(&self) -> &PrinterState {
        &self.state
    }

    #[inline]
    fn state_mut(&mut self) -> &mut PrinterState {
        &mut self.state
    }
}

/// Serializes a syntax-tree node as CSS.
pub trait ToCss {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result;

    #[inline]
    fn to_css_string(&self, options: PrinterOptions) -> Result<String, fmt::Error> {
        let mut output = String::new();
        self.to_css(&mut Printer::new(&mut output, options))?;
        Ok(output)
    }
}

impl<T: ToCss + ?Sized> ToCss for &T {
    #[inline]
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        (*self).to_css(dest)
    }
}

impl<T: ToCss> ToCss for Option<T> {
    #[inline]
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if let Some(value) = self {
            value.to_css(dest)?;
        }
        Ok(())
    }
}

pub(crate) fn serialize_number<PrinterT: PrinterTrait>(
    value: f32,
    dest: &mut PrinterT,
) -> fmt::Result {
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

pub(crate) fn serialize_dimension<PrinterT: PrinterTrait>(
    value: f32,
    unit: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    serialize_number(value, dest)?;
    dest.write_str(unit)
}

pub(crate) fn serialize_debug_keyword<T: fmt::Debug, PrinterT: PrinterTrait>(
    value: &T,
    dest: &mut PrinterT,
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
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        (**self).to_css(dest)
    }
}
