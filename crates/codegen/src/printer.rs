use std::fmt::{self, Write};

use rocketcss_common::GhostToken;

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

/// Shared context used while serializing an AST.
#[derive(Clone, Copy)]
pub struct ToCssContext<'token, 'ast, 'ghost> {
    token: &'token GhostToken<'ghost>,
    ast: Option<&'token rocketcss_ast::Compilation<'ast>>,
}

impl<'token, 'ast, 'ghost> ToCssContext<'token, 'ast, 'ghost> {
    #[inline]
    pub const fn new(token: &'token GhostToken<'ghost>) -> Self {
        Self { token, ast: None }
    }

    /// Creates a serialization context backed by the owner of all node IDs.
    #[inline]
    pub const fn with_ast(
        token: &'token GhostToken<'ghost>,
        ast: &'token rocketcss_ast::Compilation<'ast>,
    ) -> Self {
        Self {
            token,
            ast: Some(ast),
        }
    }

    #[inline]
    pub const fn token(&self) -> &'token GhostToken<'ghost> {
        self.token
    }

    /// Returns the AST context used to resolve typed node IDs.
    #[inline]
    pub fn ast_context(&self) -> &'token rocketcss_ast::Compilation<'ast> {
        self.ast
            .expect("serializing a NodeId requires its AstContext")
    }
}

/// Serializes a syntax-tree node as CSS.
pub trait ToCss<'ghost> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result;

    #[inline]
    fn to_css_string(
        &self,
        options: PrinterOptions,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> Result<String, fmt::Error> {
        let mut output = String::new();
        self.to_css(&mut Printer::new(&mut output, options), cx)?;
        Ok(output)
    }
}

/// Compares two complete serialized value graphs while resolving NodeIds through `cx`.
/// A structural fast path avoids serialization for payloads whose nested identities already
/// match; the fallback allocates only the left serialization and streams the right side into it.
#[doc(hidden)]
pub fn css_values_are_equal<'ghost, T>(
    left: &T,
    right: &T,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> bool
where
    T: ToCss<'ghost> + PartialEq,
{
    if left == right {
        return true;
    }
    let options = PrinterOptions { prettify: false };
    let Ok(expected) = left.to_css_string(options, cx) else {
        return false;
    };
    css_value_matches_serialization(&expected, right, cx)
}

/// Streams `value` against an existing compact CSS serialization without allocating another
/// output buffer.
#[doc(hidden)]
pub fn css_value_matches_serialization<'ghost, T>(
    expected: &str,
    value: &T,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> bool
where
    T: ToCss<'ghost>,
{
    let options = PrinterOptions { prettify: false };
    let mut comparison = CssComparison::new(expected);
    let result = value.to_css(&mut Printer::new(&mut comparison, options), cx);
    result.is_ok() && comparison.is_complete()
}

struct CssComparison<'a> {
    expected: &'a str,
    offset: usize,
}

impl<'a> CssComparison<'a> {
    #[inline]
    const fn new(expected: &'a str) -> Self {
        Self {
            expected,
            offset: 0,
        }
    }

    #[inline]
    fn is_complete(&self) -> bool {
        self.offset == self.expected.len()
    }
}

impl Write for CssComparison<'_> {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        let end = self.offset.checked_add(value.len()).ok_or(fmt::Error)?;
        if self.expected.get(self.offset..end) != Some(value) {
            return Err(fmt::Error);
        }
        self.offset = end;
        Ok(())
    }
}

impl<'ast, 'ghost, T: ToCss<'ghost>> ToCss<'ghost> for rocketcss_ast::NodeId<'ast, T> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        cx.ast_context().resolve_node(*self).to_css(dest, cx)
    }
}

impl<'ghost, T: ToCss<'ghost> + ?Sized> ToCss<'ghost> for &T {
    #[inline]
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        (*self).to_css(dest, cx)
    }
}

impl<'ghost, T: ToCss<'ghost>> ToCss<'ghost> for Option<T> {
    #[inline]
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if let Some(value) = self {
            value.to_css(dest, cx)?;
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

pub(crate) fn serialize_dimension<'ghost, UnitT: ToCss<'ghost>, PrinterT: PrinterTrait>(
    value: f32,
    unit: &UnitT,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    serialize_number(value, dest)?;
    unit.to_css(dest, cx)
}
