use std::fmt::{self, Write};

/// Options controlling CSS serialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrinterOptions {
    /// Emit optional whitespace, indentation, and line breaks.
    pub prettify: bool,
}

impl Default for PrinterOptions {
    fn default() -> Self {
        Self { prettify: true }
    }
}

/// A delimiter and its surrounding whitespace behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter {
    /// `,`, followed by optional whitespace.
    Comma,
    /// `:`, followed by optional whitespace.
    Colon,
    /// `>`, surrounded by optional whitespace.
    ChildCombinator,
    /// `+`, surrounded by optional whitespace.
    NextSiblingCombinator,
    /// `~`, surrounded by optional whitespace.
    LaterSiblingCombinator,
}

impl Delimiter {
    #[inline]
    const fn value(self) -> char {
        match self {
            Self::Comma => ',',
            Self::Colon => ':',
            Self::ChildCombinator => '>',
            Self::NextSiblingCombinator => '+',
            Self::LaterSiblingCombinator => '~',
        }
    }

    #[inline]
    const fn whitespace_before(self) -> bool {
        matches!(
            self,
            Self::ChildCombinator | Self::NextSiblingCombinator | Self::LaterSiblingCombinator
        )
    }
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
    pub fn prettify(&self) -> bool {
        self.options.prettify
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
        if self.options.prettify {
            self.write_char(' ')
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn delim(&mut self, delimiter: Delimiter) -> fmt::Result {
        if self.options.prettify {
            if delimiter.whitespace_before() {
                self.write_char(' ')?;
            }
            self.write_char(delimiter.value())?;
            self.write_char(' ')
        } else {
            self.write_char(delimiter.value())
        }
    }

    #[inline]
    pub fn new_line(&mut self) -> fmt::Result {
        if !self.options.prettify {
            return Ok(());
        }

        self.write_char('\n')?;
        for _ in 0..self.state.indent {
            self.write_char(' ')?;
        }
        Ok(())
    }

    #[inline]
    pub fn blank_line(&mut self) -> fmt::Result {
        if !self.options.prettify {
            return Ok(());
        }

        self.write_char('\n')?;
        self.new_line()
    }

    #[inline]
    pub fn semicolon(&mut self, required: bool) -> fmt::Result {
        if required || self.options.prettify {
            self.write_char(';')
        } else {
            Ok(())
        }
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
    fn prettify(&self) -> bool {
        self.options().prettify
    }

    #[inline]
    fn whitespace(&mut self) -> fmt::Result {
        if self.prettify() {
            self.write_char(' ')
        } else {
            Ok(())
        }
    }

    #[inline]
    fn delim(&mut self, delimiter: Delimiter) -> fmt::Result {
        if self.prettify() {
            if delimiter.whitespace_before() {
                self.write_char(' ')?;
            }
            self.write_char(delimiter.value())?;
            self.write_char(' ')
        } else {
            self.write_char(delimiter.value())
        }
    }

    #[inline]
    fn new_line(&mut self) -> fmt::Result {
        if !self.prettify() {
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
    fn blank_line(&mut self) -> fmt::Result {
        if !self.prettify() {
            return Ok(());
        }

        self.write_char('\n')?;
        self.new_line()
    }

    #[inline]
    fn semicolon(&mut self, required: bool) -> fmt::Result {
        if required || self.prettify() {
            self.write_char(';')
        } else {
            Ok(())
        }
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
    // Percentages and unit conversions can introduce a tiny f32 error (for
    // example, `30%` is stored as `0.3` and multiplied back to `30.000002`).
    // Snap values that are extremely close to a non-zero integer without
    // erasing genuinely small fractional values.
    let rounded = value.round();
    let value = if rounded != 0.0 && (value - rounded).abs() < 0.000_01 {
        rounded
    } else {
        value
    };
    let mut buffer = zmij::Buffer::new();
    let output = buffer.format(value);
    let output = output.strip_suffix(".0").unwrap_or(output);
    if value != 0.0 && value.abs() < 1.0 {
        if value.is_sign_negative() {
            dest.write_char('-')?;
            dest.write_str(output.trim_start_matches('-').trim_start_matches('0'))
        } else {
            dest.write_str(output.trim_start_matches('0'))
        }
    } else {
        dest.write_str(output)
    }
}

#[inline]
pub(crate) fn serialize_int<IntegerT: itoa::Integer, PrinterT: PrinterTrait>(
    value: IntegerT,
    dest: &mut PrinterT,
) -> fmt::Result {
    let mut buffer = itoa::Buffer::new();
    dest.write_str(buffer.format(value))
}

pub(crate) fn serialize_hex<PrinterT: PrinterTrait>(
    mut value: u32,
    min_digits: usize,
    uppercase: bool,
    dest: &mut PrinterT,
) -> fmt::Result {
    const LOWER: &[u8; 16] = b"0123456789abcdef";
    const UPPER: &[u8; 16] = b"0123456789ABCDEF";

    debug_assert!(min_digits <= 8);
    let digits = if uppercase { UPPER } else { LOWER };
    let mut buffer = [b'0'; 8];
    let mut start = buffer.len();
    loop {
        start -= 1;
        buffer[start] = digits[(value & 0x0f) as usize];
        value >>= 4;
        if value == 0 && buffer.len() - start >= min_digits.max(1) {
            break;
        }
    }

    // SAFETY: `buffer` only contains ASCII hexadecimal digits.
    dest.write_str(unsafe { std::str::from_utf8_unchecked(&buffer[start..]) })
}

pub(crate) fn serialize_dimension<UnitT: ToCss, PrinterT: PrinterTrait>(
    value: f32,
    unit: &UnitT,
    dest: &mut PrinterT,
) -> fmt::Result {
    serialize_number(value, dest)?;
    unit.to_css(dest)
}

pub(crate) fn serialize_debug_keyword<T: fmt::Debug, PrinterT: PrinterTrait>(
    value: &T,
    dest: &mut PrinterT,
) -> fmt::Result {
    let mut writer = DebugKeywordWriter::new(dest);
    fmt::write(&mut writer, format_args!("{value:?}"))?;
    writer.finish()
}

struct DebugKeywordWriter<'a, PrinterT> {
    dest: &'a mut PrinterT,
    pending: Option<char>,
    previous: Option<char>,
    index: usize,
}

impl<'a, PrinterT: PrinterTrait> DebugKeywordWriter<'a, PrinterT> {
    #[inline]
    fn new(dest: &'a mut PrinterT) -> Self {
        Self {
            dest,
            pending: None,
            previous: None,
            index: 0,
        }
    }

    fn flush_pending(&mut self, next: Option<char>) -> fmt::Result {
        let Some(character) = self.pending.take() else {
            return Ok(());
        };
        if character == '_' && next.is_none() {
            return Ok(());
        }
        if character.is_ascii_uppercase()
            && self.index > 0
            && (self
                .previous
                .is_some_and(|previous| previous.is_ascii_lowercase() || previous.is_ascii_digit())
                || next.is_some_and(|next| next.is_ascii_lowercase()))
        {
            self.dest.write_char('-')?;
        }
        self.dest.write_char(character.to_ascii_lowercase())?;
        self.previous = Some(character);
        self.index += 1;
        Ok(())
    }

    fn finish(mut self) -> fmt::Result {
        self.flush_pending(None)
    }
}

impl<PrinterT: PrinterTrait> Write for DebugKeywordWriter<'_, PrinterT> {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        for character in value.chars() {
            self.flush_pending(Some(character))?;
            self.pending = Some(character);
        }
        Ok(())
    }
}

impl<'a, T: ToCss> ToCss for rocketcss_allocator::boxed::Box<'a, T> {
    #[inline]
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        (**self).to_css(dest)
    }
}
