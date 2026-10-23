use crate::prelude::*;

fn write_space_separated<'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost>>(
    values: &[&T],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

fn write_comma_separated<'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost>>(
    values: &[T],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

fn write_pair<'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost> + PartialEq>(
    first: &T,
    second: &T,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    first.to_css(dest, cx)?;
    if first != second {
        dest.write_char(' ')?;
        second.to_css(dest, cx)?;
    }
    Ok(())
}

fn write_four<'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost> + PartialEq>(
    top: &T,
    right: &T,
    bottom: &T,
    left: &T,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    top.to_css(dest, cx)?;
    if top == right && top == bottom && top == left {
        return Ok(());
    }
    dest.write_char(' ')?;
    right.to_css(dest, cx)?;
    if top == bottom && right == left {
        return Ok(());
    }
    dest.write_char(' ')?;
    bottom.to_css(dest, cx)?;
    if right != left {
        dest.write_char(' ')?;
        left.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ast, 'ghost> ToCss<'ghost> for Position<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.x.to_css(dest, _cx)?;
        if !matches!(data.y.data(_cx.ast()), PositionComponentData::Center)
            || matches!(
                data.x.data(_cx.ast()),
                PositionComponentData::Side {
                    offset: Some(_),
                    ..
                }
            )
        {
            dest.write_char(' ')?;
            data.y.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for WebKitGradientPoint<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.x.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.y.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for WebKitColorStop<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if data.position == 0.0 {
            dest.write_str("from(")?;
            data.color.to_css(dest, _cx)?;
        } else if data.position == 1.0 {
            dest.write_str("to(")?;
            data.color.to_css(dest, _cx)?;
        } else {
            dest.write_str("color-stop(")?;
            serialize_number(data.position, dest)?;
            dest.delim(Delimiter::Comma)?;
            data.color.to_css(dest, _cx)?;
        }
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ImageSet<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.vendor_prefix.to_css(dest, _cx)?;
        dest.write_str("image-set(")?;
        write_comma_separated(data.options, dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ImageSetOption<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.image.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.resolution.to_css(dest, _cx)?;
        if let Some(file_type) = data.file_type {
            dest.write_str(" type(")?;
            serialize_string(file_type, dest)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BackgroundPosition<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.x.to_css(dest, _cx)?;
        if !matches!(data.y.data(_cx.ast()), PositionComponentData::Center)
            || matches!(
                data.x.data(_cx.ast()),
                PositionComponentData::Side {
                    offset: Some(_),
                    ..
                }
            )
        {
            dest.write_char(' ')?;
            data.y.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BackgroundRepeat<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        match (&data.x, &data.y) {
            (BackgroundRepeatKeyword::Repeat, BackgroundRepeatKeyword::NoRepeat) => {
                dest.write_str("repeat-x")
            }
            (BackgroundRepeatKeyword::NoRepeat, BackgroundRepeatKeyword::Repeat) => {
                dest.write_str("repeat-y")
            }
            _ => write_pair(&data.x, &data.y, dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Background<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if matches!(data.image.data(_cx.ast()), ImageData::None)
            && is_zero_background_position(_cx.ast(), data.position)
            && matches!(
                data.size.data(_cx.ast()),
                BackgroundSizeData::Explicit { height, width }
                    if matches!(height.data(_cx.ast()), LengthPercentageOrAutoData::Auto)
                        && matches!(width.data(_cx.ast()), LengthPercentageOrAutoData::Auto)
            )
            && matches!(
                _cx.ast().data(data.repeat),
                BackgroundRepeatData {
                    x: BackgroundRepeatKeyword::Repeat,
                    y: BackgroundRepeatKeyword::Repeat,
                }
            )
            && data.attachment == BackgroundAttachment::Scroll
            && data.origin == BackgroundOrigin::PaddingBox
            && data.clip == BackgroundClip::BorderBox
        {
            return data.color.to_css(dest, _cx);
        }

        data.image.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.position.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.size.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.repeat.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.attachment.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.origin.to_css(dest, _cx)?;
        if data.clip
            != match &data.origin {
                BackgroundOrigin::BorderBox => BackgroundClip::BorderBox,
                BackgroundOrigin::PaddingBox => BackgroundClip::PaddingBox,
                BackgroundOrigin::ContentBox => BackgroundClip::ContentBox,
            }
        {
            dest.write_char(' ')?;
            data.clip.to_css(dest, _cx)?;
        }
        dest.write_char(' ')?;
        data.color.to_css(dest, _cx)
    }
}

fn is_zero_position_components<'ast, Sx: Copy, Sy: Copy>(
    ast: &Ast<'ast>,
    x: PositionComponent<'ast, Sx>,
    y: PositionComponent<'ast, Sy>,
) -> bool {
    fn is_zero<'ast, S: Copy>(ast: &Ast<'ast>, component: PositionComponent<'ast, S>) -> bool {
        matches!(
            component.data(ast),
            PositionComponentData::Length(value)
                if matches!(value.data(ast), DimensionPercentageData::Percentage(0.0) | DimensionPercentageData::Zero)
        )
    }

    is_zero(ast, x) && is_zero(ast, y)
}

fn is_zero_background_position<'ast>(ast: &Ast<'ast>, position: BackgroundPosition<'ast>) -> bool {
    is_zero_position_components(ast, position.x(ast), position.y(ast))
}

fn is_zero_position<'ast>(ast: &Ast<'ast>, position: Position<'ast>) -> bool {
    is_zero_position_components(ast, position.x(ast), position.y(ast))
}

impl<'ast, 'ghost> ToCss<'ghost> for BoxShadow<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if data.inset {
            dest.write_str("inset ")?;
        }
        write_space_separated(
            &[&data.x_offset, &data.y_offset, &data.blur, &data.spread],
            dest,
            _cx,
        )?;
        dest.write_char(' ')?;
        data.color.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for AspectRatio<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if data.auto {
            dest.write_str("auto")?;
            if data.ratio.is_some() {
                dest.write_char(' ')?;
            }
        }
        data.ratio.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Overflow<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.x, &data.y, dest, _cx)
    }
}

macro_rules! logical_pair {
    ($($ty:ident, $first:ident, $second:ident);+ $(;)?) => {
        $(
            impl<'ast, 'ghost> ToCss<'ghost> for $ty<'ast> {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, '_, 'ghost>) -> fmt::Result {
                    let data = _cx.ast().data(*self);
                    write_pair(&data.$first, &data.$second, dest, _cx)
                }
            }
        )+
    };
}

logical_pair! {
    InsetBlock, block_start, block_end;
    InsetInline, inline_start, inline_end;
    MarginBlock, block_start, block_end;
    MarginInline, inline_start, inline_end;
    PaddingBlock, block_start, block_end;
    PaddingInline, inline_start, inline_end;
    ScrollMarginBlock, block_start, block_end;
    ScrollMarginInline, inline_start, inline_end;
    ScrollPaddingBlock, block_start, block_end;
    ScrollPaddingInline, inline_start, inline_end;
}

macro_rules! physical_four {
    ($($ty:ident);+ $(;)?) => {
        $(
            impl<'ast, 'ghost> ToCss<'ghost> for $ty<'ast> {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, '_, 'ghost>) -> fmt::Result {
                    let data = _cx.ast().data(*self);
                    write_four(&data.top, &data.right, &data.bottom, &data.left, dest, _cx)
                }
            }
        )+
    };
}

physical_four! {
    Inset;
    Margin;
    Padding;
    ScrollMargin;
    ScrollPadding;
    BorderColor;
    BorderWidth;
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderStyle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_four(&data.top, &data.right, &data.bottom, &data.left, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderRadius<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        let ast = _cx.ast();
        let horizontal = [
            data.top_left.field_0(ast),
            data.top_right.field_0(ast),
            data.bottom_right.field_0(ast),
            data.bottom_left.field_0(ast),
        ];
        write_four(
            &horizontal[0],
            &horizontal[1],
            &horizontal[2],
            &horizontal[3],
            dest,
            _cx,
        )?;
        let vertical = [
            data.top_left.field_1(ast),
            data.top_right.field_1(ast),
            data.bottom_right.field_1(ast),
            data.bottom_left.field_1(ast),
        ];
        if horizontal != vertical {
            dest.write_str(" / ")?;
            write_four(
                &vertical[0],
                &vertical[1],
                &vertical[2],
                &vertical[3],
                dest,
                _cx,
            )?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderImageRepeat<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.horizontal, &data.vertical, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderImageSlice<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.offsets.to_css(dest, _cx)?;
        if data.fill {
            dest.write_str(" fill")?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderImage<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.source.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.slice.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.width.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.outset.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.repeat.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderBlockColor<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.start, &data.end, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderBlockStyle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.start, &data.end, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderBlockWidth<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.start, &data.end, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderInlineColor<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.start, &data.end, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderInlineStyle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.start, &data.end, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderInlineWidth<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.start, &data.end, dest, _cx)
    }
}

impl<'ast, 'ghost, S: ToCss<'ghost> + Copy> ToCss<'ghost> for GenericBorder<'ast, S> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.width.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.color.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for FlexFlow<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.direction.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.wrap.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Flex<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if data.grow == 0.0
            && data.shrink == 0.0
            && matches!(data.basis.data(_cx.ast()), LengthPercentageOrAutoData::Auto)
        {
            return dest.write_str("none");
        }
        if data.grow == 1.0
            && data.shrink == 1.0
            && matches!(data.basis.data(_cx.ast()), LengthPercentageOrAutoData::Auto)
        {
            return dest.write_str("auto");
        }

        serialize_number(data.grow, dest)?;
        let basis_is_zero = matches!(
            data.basis.data(_cx.ast()),
            LengthPercentageOrAutoData::LengthPercentage(value)
                if matches!(value.data(_cx.ast()), DimensionPercentageData::Zero)
        );
        let basis_is_auto = matches!(data.basis.data(_cx.ast()), LengthPercentageOrAutoData::Auto);
        if data.shrink != 1.0 || (!basis_is_zero && !basis_is_auto) {
            dest.write_char(' ')?;
            serialize_number(data.shrink, dest)?;
        }
        if !basis_is_zero {
            dest.write_char(' ')?;
            data.basis.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

macro_rules! place_pair {
    ($($ty:ident);+ $(;)?) => {
        $(
            impl<'ast, 'ghost> ToCss<'ghost> for $ty<'ast> {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, '_, 'ghost>) -> fmt::Result {
                    let data = _cx.ast().data(*self);
                    data.align.to_css(dest, _cx)?;
                    dest.write_char(' ')?;
                    data.justify.to_css(dest, _cx)
                }
            }
        )+
    };
}

place_pair! { PlaceContent; PlaceSelf; PlaceItems }

impl<'ast, 'ghost> ToCss<'ghost> for Gap<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_pair(&data.row, &data.column, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ColumnRule<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        debug_assert!(data.width.is_some() || data.style.is_some() || data.color.is_some());
        let mut wrote_value = false;
        if let Some(width) = &data.width {
            width.to_css(dest, _cx)?;
            wrote_value = true;
        }
        if let Some(style) = &data.style {
            if wrote_value {
                dest.write_char(' ')?;
            }
            style.to_css(dest, _cx)?;
            wrote_value = true;
        }
        if let Some(color) = &data.color {
            if wrote_value {
                dest.write_char(' ')?;
            }
            color.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ColumnWidth<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ColumnWidthData::Auto => dest.write_str("auto"),
            ColumnWidthData::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ColumnCount {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Columns<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        let width_is_auto = matches!(data.width.data(_cx.ast()), ColumnWidthData::Auto);
        let count_is_auto = matches!(data.count, ColumnCount::Auto);
        if width_is_auto && count_is_auto {
            return dest.write_str("auto");
        }
        if !width_is_auto {
            data.width.to_css(dest, _cx)?;
        }
        if !count_is_auto {
            if !width_is_auto {
                dest.write_char(' ')?;
            }
            data.count.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TrackRepeat<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("repeat(")?;
        data.count.to_css(dest, _cx)?;
        dest.delim(Delimiter::Comma)?;
        for (index, track_size) in data.track_sizes.iter().enumerate() {
            if let Some(names) = data.line_names.get(index)
                && !names.is_empty()
            {
                crate::values::write_line_names(names, dest)?;
                dest.write_char(' ')?;
            }
            track_size.to_css(dest, _cx)?;
            if index + 1 < data.track_sizes.len() {
                dest.write_char(' ')?;
            }
        }
        if let Some(names) = data.line_names.get(data.track_sizes.len())
            && !names.is_empty()
        {
            dest.write_char(' ')?;
            crate::values::write_line_names(names, dest)?;
        }
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for GridAutoFlow<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.direction.to_css(dest, _cx)?;
        if data.dense {
            dest.write_str(" dense")?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for GridTemplate<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.rows.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.columns.to_css(dest, _cx)?;
        if !matches!(data.areas.data(_cx.ast()), GridTemplateAreasData::None) {
            dest.write_char(' ')?;
            data.areas.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

fn write_track_sizes<'ghost, PrinterT: PrinterTrait>(
    values: &[TrackSize<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ast, 'ghost> ToCss<'ghost> for Grid<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.rows.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.columns.to_css(dest, _cx)?;
        dest.write_str(" auto-flow ")?;
        data.auto_flow.to_css(dest, _cx)?;
        if !data.auto_rows.is_empty() {
            dest.write_char(' ')?;
            write_track_sizes(data.auto_rows, dest, _cx)?;
        }
        if !data.auto_columns.is_empty() {
            dest.write_str(" / ")?;
            write_track_sizes(data.auto_columns, dest, _cx)?;
        }
        if !matches!(data.areas.data(_cx.ast()), GridTemplateAreasData::None) {
            dest.write_char(' ')?;
            data.areas.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

macro_rules! grid_pair {
    ($($ty:ident);+ $(;)?) => {
        $(
            impl<'ast, 'ghost> ToCss<'ghost> for $ty<'ast> {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, '_, 'ghost>) -> fmt::Result {
                    let data = _cx.ast().data(*self);
                    data.start.to_css(dest, _cx)?;
                    dest.write_str(" / ")?;
                    data.end.to_css(dest, _cx)
                }
            }
        )+
    };
}

grid_pair! { GridRow; GridColumn }

impl<'ast, 'ghost> ToCss<'ghost> for GridArea<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.row_start.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.column_start.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.row_end.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.column_end.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Font<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.variant_caps.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.weight.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.stretch.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.size.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.line_height.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        write_comma_separated(data.family, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Transition<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.property.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.duration.to_css(dest, _cx)?;
        if !matches!(data.timing_function, EasingFunction::Ease) {
            dest.write_char(' ')?;
            data.timing_function.to_css(dest, _cx)?;
        }
        if !matches!(data.delay, Time::Seconds(0.0) | Time::Milliseconds(0.0)) {
            dest.write_char(' ')?;
            data.delay.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ScrollTimeline<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.scroller.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.axis.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ViewTimeline<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.axis.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.inset.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for AnimationRange<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.start.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.end.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Animation<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        // Components print in their stored order: authored order after
        // parsing, canonical order after the ORDER_VALUES minify pass, which
        // also moves a name colliding with a keyword class behind that class.
        for (index, component) in data.components.iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            // A quoted name colliding with a keyword class must stay quoted
            // unless the class appears before it; unquoted it would reparse
            // into the class slot.
            if let AnimationComponentData::Name(name) = component.data(_cx.ast())
                && let AnimationName::String(value) = name
                && name.keyword_class().is_some_and(|class| {
                    !data.components[..index]
                        .iter()
                        .any(|component| component.keyword_class(_cx.ast()) == Some(class))
                })
            {
                serialize_string(value, dest)?;
                continue;
            }
            component.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for AnimationComponent<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            AnimationComponentData::Name(value) => value.to_css(dest, _cx),
            AnimationComponentData::Duration(value) | AnimationComponentData::Delay(value) => {
                value.to_css(dest, _cx)
            }
            AnimationComponentData::TimingFunction(value) => value.to_css(dest, _cx),
            AnimationComponentData::IterationCount(value) => value.to_css(dest, _cx),
            AnimationComponentData::Direction(value) => value.to_css(dest, _cx),
            AnimationComponentData::FillMode(value) => value.to_css(dest, _cx),
            AnimationComponentData::PlayState(value) => value.to_css(dest, _cx),
        }
    }
}

fn write_numbers<PrinterT: PrinterTrait>(values: &[f32], dest: &mut PrinterT) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        serialize_number(*value, dest)?;
    }
    Ok(())
}

impl<'ast, 'ghost> ToCss<'ghost> for MatrixForFloat<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("matrix(")?;
        write_numbers(&[data.a, data.b, data.c, data.d, data.e, data.f], dest)?;
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Matrix3DForFloat<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("matrix3d(")?;
        write_numbers(
            &[
                data.m11, data.m12, data.m13, data.m14, data.m21, data.m22, data.m23, data.m24,
                data.m31, data.m32, data.m33, data.m34, data.m41, data.m42, data.m43, data.m44,
            ],
            dest,
        )?;
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Rotate<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if data.x == 0.0 && data.y == 0.0 && data.z == 1.0 {
            return data.angle.to_css(dest, _cx);
        }
        write_numbers(&[data.x, data.y, data.z], dest)?;
        dest.write_char(' ')?;
        data.angle.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextTransform<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.case.to_css(dest, _cx)?;
        if data.full_width {
            dest.write_str(" full-width")?;
        }
        if data.full_size_kana {
            dest.write_str(" full-size-kana")?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextIndent<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.value.to_css(dest, _cx)?;
        if data.hanging {
            dest.write_str(" hanging")?;
        }
        if data.each_line {
            dest.write_str(" each-line")?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextDecoration<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.line.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.thickness.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.color.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextEmphasis<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.color.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextEmphasisPosition<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.vertical.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.horizontal.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextShadow<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_space_separated(
            &[&data.x_offset, &data.y_offset, &data.blur, &data.spread],
            dest,
            _cx,
        )?;
        dest.write_char(' ')?;
        data.color.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Cursor<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        for image in data.images {
            image.to_css(dest, _cx)?;
            dest.delim(Delimiter::Comma)?;
        }
        data.keyword.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for CursorImage<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.url.to_css(dest, _cx)?;
        if let Some((x, y)) = data.hotspot {
            dest.write_char(' ')?;
            serialize_number(x, dest)?;
            dest.write_char(' ')?;
            serialize_number(y, dest)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Caret<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.color.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.shape.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ListStyle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.position.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.image.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.list_style_type.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Composes<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        for (index, name) in data.names.iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            serialize_identifier(name, dest)?;
        }
        if let Some(from) = &data.from {
            dest.write_str(" from ")?;
            from.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for InsetRect<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("inset(")?;
        data.rect.to_css(dest, _cx)?;
        dest.write_str(" round ")?;
        data.radius.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for CircleShape<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("circle(")?;
        data.radius.to_css(dest, _cx)?;
        dest.write_str(" at ")?;
        data.position.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for EllipseShape<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("ellipse(")?;
        data.radius_x.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.radius_y.to_css(dest, _cx)?;
        dest.write_str(" at ")?;
        data.position.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Polygon<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("polygon(")?;
        data.fill_rule.to_css(dest, _cx)?;
        dest.delim(Delimiter::Comma)?;
        write_comma_separated(data.points, dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Point<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.x.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.y.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Mask<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.image.to_css(dest, _cx)?;

        let default_size = matches!(
            data.size.data(_cx.ast()),
            BackgroundSizeData::Explicit { height, width }
                if matches!(height.data(_cx.ast()), LengthPercentageOrAutoData::Auto)
                    && matches!(width.data(_cx.ast()), LengthPercentageOrAutoData::Auto)
        );
        if !is_zero_position(_cx.ast(), data.position) || !default_size {
            dest.write_char(' ')?;
            write_mask_position(data.position, dest, _cx)?;
            if !default_size {
                if dest.prettify() {
                    dest.write_str(" / ")?;
                } else {
                    dest.write_char('/')?;
                }
                write_mask_size(data.size, dest, _cx)?;
            }
        }

        if _cx.ast().data(data.repeat)
            != (BackgroundRepeatData {
                x: BackgroundRepeatKeyword::Repeat,
                y: BackgroundRepeatKeyword::Repeat,
            })
        {
            dest.write_char(' ')?;
            data.repeat.to_css(dest, _cx)?;
        }

        let clip_matches_origin = matches!(
            (&data.clip, &data.origin),
            (MaskClip::GeometryBox(clip), origin) if clip == origin
        );
        if data.origin != GeometryBox::BorderBox || !clip_matches_origin {
            dest.write_char(' ')?;
            data.origin.to_css(dest, _cx)?;
            if !clip_matches_origin {
                dest.write_char(' ')?;
                data.clip.to_css(dest, _cx)?;
            }
        }

        if data.composite != MaskComposite::Add {
            dest.write_char(' ')?;
            data.composite.to_css(dest, _cx)?;
        }
        if data.mode != MaskMode::MatchSource {
            dest.write_char(' ')?;
            data.mode.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

fn write_mask_position<'ast, 'ghost, PrinterT: PrinterTrait>(
    value: Position<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast();
    write_mask_horizontal_position(value.x(ast), dest, cx)?;
    if !matches!(value.y(ast).data(ast), PositionComponentData::Center)
        || matches!(
            value.x(ast).data(ast),
            PositionComponentData::Side {
                offset: Some(_),
                ..
            }
        )
    {
        dest.write_char(' ')?;
        write_mask_vertical_position(value.y(ast), dest, cx)?;
    }
    Ok(())
}

fn write_mask_size<'ast, 'ghost, PrinterT: PrinterTrait>(
    value: BackgroundSize<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast();
    if let BackgroundSizeData::Explicit { height, width } = value.data(ast)
        && matches!(height.data(ast), LengthPercentageOrAutoData::Auto)
    {
        return width.to_css(dest, cx);
    }
    value.to_css(dest, cx)
}

fn write_mask_horizontal_position<'ast, 'ghost, PrinterT: PrinterTrait>(
    value: PositionComponent<'ast, HorizontalPositionKeyword>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    if !dest.prettify()
        && let PositionComponentData::Side { offset: None, side } = value.data(cx.ast())
    {
        return dest.write_str(match side {
            HorizontalPositionKeyword::Left => "0",
            HorizontalPositionKeyword::Right => "100%",
        });
    }
    value.to_css(dest, cx)
}

fn write_mask_vertical_position<'ast, 'ghost, PrinterT: PrinterTrait>(
    value: PositionComponent<'ast, VerticalPositionKeyword>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    if !dest.prettify()
        && let PositionComponentData::Side { offset: None, side } = value.data(cx.ast())
    {
        return dest.write_str(match side {
            VerticalPositionKeyword::Top => "0",
            VerticalPositionKeyword::Bottom => "100%",
        });
    }
    value.to_css(dest, cx)
}

impl<'ast, 'ghost> ToCss<'ghost> for MaskBorder<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.source.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.slice.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.width.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.outset.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.repeat.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        data.mode.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for DropShadow<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_space_separated(&[&data.x_offset, &data.y_offset, &data.blur], dest, _cx)?;
        dest.write_char(' ')?;
        data.color.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Container<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.name.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        data.container_type.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ColorScheme<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if data.only {
            dest.write_str("only ")?;
        }
        match (data.light, data.dark) {
            (true, true) => dest.write_str("light dark"),
            (true, false) => dest.write_str("light"),
            (false, true) => dest.write_str("dark"),
            (false, false) => dest.write_str("normal"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for UnparsedProperty<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if let Some(raw_value) = data.raw_value {
            dest.write_str(raw_value)
        } else {
            crate::token::write_unparsed_token_list(data.value, dest, _cx)
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for CustomProperty<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        crate::token::write_token_list(data.value, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for FamilyName<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        crate::values::font::write_custom_font_family(data.field_0, dest)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for KeyframeSelector<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            KeyframeSelectorData::Percentage(value) => {
                serialize_number(value * 100.0, dest)?;
                dest.write_char('%')
            }
            KeyframeSelectorData::From => dest.write_str("from"),
            KeyframeSelectorData::To => dest.write_str("to"),
            KeyframeSelectorData::TimelineRangePercentage(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for KeyframesName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Ident(value) => serialize_identifier(value, dest),
            Self::Custom(value) => serialize_string(value, dest),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for FontFaceProperty<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            FontFacePropertyData::Source(values) => write_comma_separated(values, dest, _cx),
            FontFacePropertyData::FontFamily(value) => value.to_css(dest, _cx),
            FontFacePropertyData::FontStyle(value) => value.to_css(dest, _cx),
            FontFacePropertyData::FontWeight(value) => value.to_css(dest, _cx),
            FontFacePropertyData::FontStretch(value) => value.to_css(dest, _cx),
            FontFacePropertyData::UnicodeRange(values) => write_comma_separated(values, dest, _cx),
            FontFacePropertyData::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for PropertyRuleDescriptor<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            PropertyRuleDescriptorData::Syntax(value) => {
                dest.write_str("syntax")?;
                dest.delim(Delimiter::Colon)?;
                let syntax = value.to_css_string(dest.options(), _cx)?;
                serialize_string(&syntax, dest)
            }
            PropertyRuleDescriptorData::Inherits(value) => {
                dest.write_str("inherits")?;
                dest.delim(Delimiter::Colon)?;
                dest.write_str(if value { "true" } else { "false" })
            }
            PropertyRuleDescriptorData::InitialValue(value) => {
                dest.write_str("initial-value")?;
                dest.delim(Delimiter::Colon)?;
                value.to_css(dest, _cx)
            }
            PropertyRuleDescriptorData::Unknown(value) => {
                let name = match value.name(_cx.ast()) {
                    CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
                };
                serialize_identifier(name, dest)?;
                dest.delim(Delimiter::Colon)?;
                value.to_css(dest, _cx)
            }
        }
    }
}

pub(crate) trait NamedProperty<'ast> {
    fn css_name(self, ast: &Ast<'ast>) -> &'ast str;
}

impl<'ast> NamedProperty<'ast> for FontFaceProperty<'ast> {
    fn css_name(self, ast: &Ast<'ast>) -> &'ast str {
        match self.data(ast) {
            FontFacePropertyData::Source(_) => "src",
            FontFacePropertyData::FontFamily(_) => "font-family",
            FontFacePropertyData::FontStyle(_) => "font-style",
            FontFacePropertyData::FontWeight(_) => "font-weight",
            FontFacePropertyData::FontStretch(_) => "font-stretch",
            FontFacePropertyData::UnicodeRange(_) => "unicode-range",
            FontFacePropertyData::Custom(value) => match value.name(ast) {
                CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
            },
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Source<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SourceData::Url(value) => value.to_css(dest, _cx),
            SourceData::Local(value) => {
                dest.write_str("local(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontFormat<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let value = match self {
            Self::String(value) => value,
            value => value
                .as_css_str()
                .expect("custom font format handled separately"),
        };
        serialize_string(value, dest)
    }
}

impl<'ghost> ToCss<'ghost> for FontTechnology {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("font technologies are static keywords"),
        )
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for FontFaceStyle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            FontFaceStyleData::Normal => dest.write_str("normal"),
            FontFaceStyleData::Italic => dest.write_str("italic"),
            FontFaceStyleData::Oblique(value) => {
                dest.write_str("oblique")?;
                let ast = _cx.ast();
                let is_default = matches!(
                    (value.field_0(ast), value.field_1(ast)),
                    (Angle::Deg(first), Angle::Deg(second)) if first == 14.0 && second == 14.0
                );
                if !is_default {
                    dest.write_char(' ')?;
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for FontPaletteValuesProperty<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            FontPaletteValuesPropertyData::FontFamily(value) => value.to_css(dest, _cx),
            FontPaletteValuesPropertyData::BasePalette(value) => value.to_css(dest, _cx),
            FontPaletteValuesPropertyData::OverrideColors(values) => {
                write_comma_separated(values, dest, _cx)
            }
            FontPaletteValuesPropertyData::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast> NamedProperty<'ast> for FontPaletteValuesProperty<'ast> {
    fn css_name(self, ast: &Ast<'ast>) -> &'ast str {
        match self.data(ast) {
            FontPaletteValuesPropertyData::FontFamily(_) => "font-family",
            FontPaletteValuesPropertyData::BasePalette(_) => "base-palette",
            FontPaletteValuesPropertyData::OverrideColors(_) => "override-colors",
            FontPaletteValuesPropertyData::Custom(value) => match value.name(ast) {
                CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
            },
        }
    }
}

impl<'ghost> ToCss<'ghost> for BasePalette {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Light => dest.write_str("light"),
            Self::Dark => dest.write_str("dark"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontFeatureSubruleType {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("font feature subrule types are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for PageMarginBox {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("page margin boxes are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for PagePseudoClass {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("page pseudo classes are static keywords"),
        )
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ParsedComponent<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ParsedComponentData::Length(value) => value.to_css(dest, _cx),
            ParsedComponentData::Number(value) => serialize_number(value, dest),
            ParsedComponentData::Percentage(value) => {
                serialize_number(value * 100.0, dest)?;
                dest.write_char('%')
            }
            ParsedComponentData::LengthPercentage(value) => value.to_css(dest, _cx),
            ParsedComponentData::String(value) => serialize_string(value, dest),
            ParsedComponentData::Color(value) => value.to_css(dest, _cx),
            ParsedComponentData::Image(value) => value.to_css(dest, _cx),
            ParsedComponentData::Url(value) => value.to_css(dest, _cx),
            ParsedComponentData::Integer(value) => serialize_int(value, dest),
            ParsedComponentData::Angle(value) => value.to_css(dest, _cx),
            ParsedComponentData::Time(value) => value.to_css(dest, _cx),
            ParsedComponentData::Resolution(value) => value.to_css(dest, _cx),
            ParsedComponentData::TransformFunction(value) => value.to_css(dest, _cx),
            ParsedComponentData::TransformList(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
            ParsedComponentData::CustomIdent(value) => serialize_identifier(value, dest),
            ParsedComponentData::Literal(value) => dest.write_str(value),
            ParsedComponentData::Repeated {
                components,
                multiplier,
            } => {
                let delimiter = match multiplier {
                    Multiplier::None => "",
                    Multiplier::Space => " ",
                    Multiplier::Comma => ", ",
                };
                for (index, value) in components.iter().enumerate() {
                    if index > 0 {
                        dest.write_str(delimiter)?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
            ParsedComponentData::TokenList(values) => {
                crate::token::write_token_list(values, dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Multiplier {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(match self {
            Self::None => "",
            Self::Space => "+",
            Self::Comma => "#",
        })
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for SyntaxString<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SyntaxStringData::Universal => dest.write_char('*'),
            SyntaxStringData::Components(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_str(" | ")?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for SyntaxComponentKind<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SyntaxComponentKindData::Literal(value) => dest.write_str(value),
            SyntaxComponentKindData::Length => dest.write_str("<length>"),
            SyntaxComponentKindData::Number => dest.write_str("<number>"),
            SyntaxComponentKindData::Percentage => dest.write_str("<percentage>"),
            SyntaxComponentKindData::LengthPercentage => dest.write_str("<length-percentage>"),
            SyntaxComponentKindData::String => dest.write_str("<string>"),
            SyntaxComponentKindData::Color => dest.write_str("<color>"),
            SyntaxComponentKindData::Image => dest.write_str("<image>"),
            SyntaxComponentKindData::Url => dest.write_str("<url>"),
            SyntaxComponentKindData::Integer => dest.write_str("<integer>"),
            SyntaxComponentKindData::Angle => dest.write_str("<angle>"),
            SyntaxComponentKindData::Time => dest.write_str("<time>"),
            SyntaxComponentKindData::Resolution => dest.write_str("<resolution>"),
            SyntaxComponentKindData::TransformFunction => dest.write_str("<transform-function>"),
            SyntaxComponentKindData::TransformList => dest.write_str("<transform-list>"),
            SyntaxComponentKindData::CustomIdent => dest.write_str("<custom-ident>"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for SyntaxComponent<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.kind.to_css(dest, _cx)?;
        data.multiplier.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ContainerSizeFeatureId {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(match self {
            ContainerSizeFeatureId::Width => "width",
            ContainerSizeFeatureId::Height => "height",
            ContainerSizeFeatureId::InlineSize => "inline-size",
            ContainerSizeFeatureId::BlockSize => "block-size",
            ContainerSizeFeatureId::AspectRatio => "aspect-ratio",
            ContainerSizeFeatureId::Orientation => "orientation",
        })
    }
}

impl<'ghost> ToCss<'ghost> for ScrollStateFeatureId {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("scroll state features are static keywords"),
        )
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ContainerCondition<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ContainerConditionData::Feature(value) => value.to_css(dest, _cx),
            ContainerConditionData::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            ContainerConditionData::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in conditions.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
            ContainerConditionData::Style(value) => {
                dest.write_str("style(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            ContainerConditionData::ScrollState(value) => {
                dest.write_str("scroll-state(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            ContainerConditionData::Unknown(values) => {
                crate::token::write_token_list(values, dest, _cx)
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for StyleQuery<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            StyleQueryData::Declaration(value) => value.to_css(dest, _cx),
            StyleQueryData::Property(value) => value.to_css(dest, _cx),
            StyleQueryData::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            StyleQueryData::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in conditions.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ScrollStateQuery<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ScrollStateQueryData::Feature(value) => value.to_css(dest, _cx),
            ScrollStateQueryData::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            ScrollStateQueryData::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in conditions.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ViewTransitionProperty<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ViewTransitionPropertyData::Navigation(value) => value.to_css(dest, _cx),
            ViewTransitionPropertyData::Types(value) => value.to_css(dest, _cx),
            ViewTransitionPropertyData::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast> NamedProperty<'ast> for ViewTransitionProperty<'ast> {
    fn css_name(self, ast: &Ast<'ast>) -> &'ast str {
        match self.data(ast) {
            ViewTransitionPropertyData::Navigation(_) => "navigation",
            ViewTransitionPropertyData::Types(_) => "types",
            ViewTransitionPropertyData::Custom(value) => match value.name(ast) {
                CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
            },
        }
    }
}

impl<'ghost> ToCss<'ghost> for Navigation {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("navigation values are static keywords"),
        )
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ImportRule<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("@import ")?;
        serialize_string(data.url, dest)?;
        if let Some(layer) = &data.layer {
            dest.write_str(" layer")?;
            if !layer.is_empty() {
                dest.write_char('(')?;
                write_layer_name(layer, dest)?;
                dest.write_char(')')?;
            }
        }
        if let Some(supports) = &data.supports {
            dest.write_str(" supports(")?;
            let serialized = supports.to_css_string(dest.options(), _cx)?;
            dest.write_str(
                serialized
                    .strip_prefix('(')
                    .and_then(|value| value.strip_suffix(')'))
                    .unwrap_or(&serialized),
            )?;
            dest.write_char(')')?;
        }
        if let Some(media) = &data.media {
            dest.write_char(' ')?;
            media.to_css(dest, _cx)?;
        }
        dest.write_char(';')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TimelineRangePercentage<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.name.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        serialize_number(data.percentage * 100.0, dest)?;
        dest.write_char('%')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for UrlSource<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.url.to_css(dest, _cx)?;
        if let Some(format) = &data.format {
            dest.write_str(" format(")?;
            format.to_css(dest, _cx)?;
            dest.write_char(')')?;
        }
        if !data.tech.is_empty() {
            dest.write_str(" tech(")?;
            write_comma_separated(data.tech, dest, _cx)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for UnicodeRange<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        for wildcard_digits in 1..=6 {
            let bits = wildcard_digits * 4;
            let mask = (1_u32 << bits) - 1;
            if data.start & mask == 0 && data.end == data.start | mask {
                dest.write_str("U+")?;
                serialize_hex(data.start >> bits, 1, true, dest)?;
                for _ in 0..wildcard_digits {
                    dest.write_char('?')?;
                }
                return Ok(());
            }
        }
        dest.write_str("U+")?;
        serialize_hex(data.start, 1, true, dest)?;
        if data.start != data.end {
            dest.write_char('-')?;
            serialize_hex(data.end, 1, true, dest)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for OverrideColors<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        serialize_int(data.index, dest)?;
        dest.write_char(' ')?;
        data.color.to_css(dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for FontFeatureDeclaration<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        serialize_identifier(data.name, dest)?;
        dest.delim(Delimiter::Colon)?;
        for (index, value) in data.values.iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            serialize_int(*value, dest)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for PageSelector<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if let Some(name) = data.name {
            serialize_identifier(name, dest)?;
        }
        for pseudo_class in data.pseudo_classes {
            dest.write_char(':')?;
            pseudo_class.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for CharsetRule<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("@charset ")?;
        serialize_string(data.encoding, dest)?;
        dest.write_char(';')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for NamespaceRule<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("@namespace ")?;
        if let Some(prefix) = data.prefix {
            serialize_identifier(prefix, dest)?;
            dest.write_char(' ')?;
        }
        serialize_string(data.url, dest)?;
        dest.write_char(';')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for CustomMediaRule<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("@custom-media ")?;
        dest.write_str("--")?;
        serialize_name(data.name.strip_prefix("--").unwrap_or(data.name), dest)?;
        dest.write_char(' ')?;
        data.query.to_css(dest, _cx)?;
        dest.write_char(';')
    }
}

fn write_layer_name<PrinterT: PrinterTrait>(name: &[&str], dest: &mut PrinterT) -> fmt::Result {
    for (index, part) in name.iter().enumerate() {
        if index > 0 {
            dest.write_char('.')?;
        }
        serialize_identifier(part, dest)?;
    }
    Ok(())
}
