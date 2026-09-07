use super::*;

keyword_values! {
    BaselinePosition,
    ContentDistribution,
    OverflowPosition,
    ContentPosition,
    SelfPosition,
    LegacyJustify,
}

fn write_overflow_position<'ghost, PrinterT: PrinterTrait>(
    overflow: &Option<OverflowPosition>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if let Some(overflow) = overflow {
        overflow.to_css(dest, cx)?;
        dest.write_char(' ')?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for AlignContent {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::BaselinePosition(value) => {
                value.to_css(dest, _cx)?;
                dest.write_str(" baseline")
            }
            Self::ContentDistribution(value) => value.to_css(dest, _cx),
            Self::ContentPosition { overflow, value } => {
                write_overflow_position(overflow, dest, _cx)?;
                value.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for JustifyContent {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::ContentDistribution(value) => value.to_css(dest, _cx),
            Self::ContentPosition { overflow, value } => {
                write_overflow_position(overflow, dest, _cx)?;
                value.to_css(dest, _cx)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest, _cx)?;
                dest.write_str(if matches!(self, Self::Left { .. }) {
                    "left"
                } else {
                    "right"
                })
            }
        }
    }
}

macro_rules! self_alignment {
    ($ty:ty, $dest:ident, $($extra:pat => $body:expr),* $(,)?) => {
        impl<'ghost> ToCss<'ghost> for $ty {
            fn to_css<PrinterT: PrinterTrait>(
                &self,
                $dest: &mut PrinterT,
                _cx: &ToCssContext<'_, '_, 'ghost>,
            ) -> fmt::Result {
                match self {
                    Self::Normal => $dest.write_str("normal"),
                    Self::Stretch => $dest.write_str("stretch"),
                    Self::BaselinePosition(value) => {
                        value.to_css($dest, _cx)?;
                        $dest.write_str(" baseline")
                    }
                    Self::SelfPosition { overflow, value } => {
                        write_overflow_position(overflow, $dest, _cx)?;
                        value.to_css($dest, _cx)
                    }
                    $($extra => $body,)*
                }
            }
        }
    };
}

self_alignment!(AlignSelf, dest, Self::Auto => dest.write_str("auto"));
self_alignment!(AlignItems, dest,);

impl<'ghost> ToCss<'ghost> for JustifySelf {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Normal => dest.write_str("normal"),
            Self::Stretch => dest.write_str("stretch"),
            Self::BaselinePosition(value) => {
                value.to_css(dest, _cx)?;
                dest.write_str(" baseline")
            }
            Self::SelfPosition { overflow, value } => {
                write_overflow_position(overflow, dest, _cx)?;
                value.to_css(dest, _cx)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest, _cx)?;
                dest.write_str(if matches!(self, Self::Left { .. }) {
                    "left"
                } else {
                    "right"
                })
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for JustifyItems {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Stretch => dest.write_str("stretch"),
            Self::BaselinePosition(value) => {
                value.to_css(dest, _cx)?;
                dest.write_str(" baseline")
            }
            Self::SelfPosition { overflow, value } => {
                write_overflow_position(overflow, dest, _cx)?;
                value.to_css(dest, _cx)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest, _cx)?;
                dest.write_str(if matches!(self, Self::Left { .. }) {
                    "left"
                } else {
                    "right"
                })
            }
            Self::Legacy(value) => {
                dest.write_str("legacy ")?;
                value.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for GapValue<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
        }
    }
}
