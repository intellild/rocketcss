//! Compact source positions kept in a context-owned sidecar column.

pub const DUMMY_SP: Span = Span { start: 0, end: 0 };

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[inline]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    #[inline]
    pub const fn is_dummy(self) -> bool {
        self.start == 0 && self.end == 0
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}..{}", self.start, self.end)
    }
}

pub trait GetSpan<'ast>: Copy {
    fn span(self, ast: &crate::Ast<'ast>) -> Span;
}

pub trait SetSpan<'ast>: Copy {
    fn set_span(self, ast: &mut crate::Ast<'ast>, span: Span);
}
