use super::*;

impl<'ghost> ToCss<'ghost> for AspectRatio {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.auto {
            dest.write_str("auto")?;
            if self.ratio.is_some() {
                dest.write_char(' ')?;
            }
        }
        self.ratio.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Overflow {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_pair(&self.x, &self.y, dest, _cx)
    }
}

logical_pair! {
    InsetBlock<'_>, block_start, block_end;
    InsetInline<'_>, inline_start, inline_end;
    MarginBlock<'_>, block_start, block_end;
    MarginInline<'_>, inline_start, inline_end;
    PaddingBlock<'_>, block_start, block_end;
    PaddingInline<'_>, inline_start, inline_end;
    ScrollMarginBlock<'_>, block_start, block_end;
    ScrollMarginInline<'_>, inline_start, inline_end;
    ScrollPaddingBlock<'_>, block_start, block_end;
    ScrollPaddingInline<'_>, inline_start, inline_end;
}

physical_four! {
    Inset<'_>;
    Margin<'_>;
    Padding<'_>;
    ScrollMargin<'_>;
    ScrollPadding<'_>;
}

impl<'ghost> ToCss<'ghost> for FlexFlow {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.direction.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.wrap.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Flex<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        let basis = ast.resolve_node(self.basis);
        if self.grow == 0.0 && self.shrink == 0.0 && matches!(basis, LengthPercentageOrAuto::Auto) {
            return dest.write_str("none");
        }
        if self.grow == 1.0 && self.shrink == 1.0 && matches!(basis, LengthPercentageOrAuto::Auto) {
            return dest.write_str("auto");
        }

        serialize_number(self.grow, dest)?;
        let basis_is_zero = matches!(
            basis,
            LengthPercentageOrAuto::LengthPercentage(value)
                if matches!(ast.resolve_node(value), LengthPercentage::Zero)
        );
        let basis_is_auto = matches!(basis, LengthPercentageOrAuto::Auto);
        if self.shrink != 1.0 || (!basis_is_zero && !basis_is_auto) {
            dest.write_char(' ')?;
            serialize_number(self.shrink, dest)?;
        }
        if !basis_is_zero {
            dest.write_char(' ')?;
            basis.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for Gap<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_node_pair(&self.row, &self.column, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ColumnRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        debug_assert!(self.width.is_some() || self.style.is_some() || self.color.is_some());
        let mut wrote_value = false;
        if let Some(width) = &self.width {
            width.to_css(dest, _cx)?;
            wrote_value = true;
        }
        if let Some(style) = &self.style {
            if wrote_value {
                dest.write_char(' ')?;
            }
            style.to_css(dest, _cx)?;
            wrote_value = true;
        }
        if let Some(color) = &self.color {
            if wrote_value {
                dest.write_char(' ')?;
            }
            color.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for ColumnWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ColumnCount {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Columns<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let width_is_auto = matches!(&self.width, ColumnWidth::Auto);
        let count_is_auto = matches!(self.count, ColumnCount::Auto);
        if width_is_auto && count_is_auto {
            return dest.write_str("auto");
        }
        if !width_is_auto {
            self.width.to_css(dest, _cx)?;
        }
        if !count_is_auto {
            if !width_is_auto {
                dest.write_char(' ')?;
            }
            self.count.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for TrackRepeat<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let repeat = cx.ast_context().track_repeat(id);
        write_track_repeat(
            repeat.count(),
            || (repeat.line_names(), repeat.track_sizes()),
            dest,
            cx,
        )
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_track_repeat(self.count, || (self.line_names, self.track_sizes), dest, cx)
    }
}

fn write_track_repeat<'id, 'ghost, PrinterT: PrinterTrait>(
    count: RepeatCount,
    lists: impl FnOnce() -> (
        AstVec<'id, AstVec<'id, AstStr<'id>>>,
        AstVec<'id, NodeId<'id, TrackSize<'id>>>,
    ),
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("repeat(")?;
    count.to_css(dest, _cx)?;
    dest.delim(Delimiter::Comma)?;
    let (line_names, track_sizes) = lists();
    let track_count = _cx.ast_context().vec_len(track_sizes);
    for (index, track_size) in _cx.ast_context().vec_iter(track_sizes).enumerate() {
        if let Some(names) = _cx.ast_context().vec_get(line_names, index)
            && !names.is_empty()
        {
            crate::values::grid::write_line_names(
                _cx.ast_context()
                    .vec_iter(names)
                    .map(|name| _cx.ast_context().str(name)),
                dest,
            )?;
            dest.write_char(' ')?;
        }
        track_size.to_css(dest, _cx)?;
        if index + 1 < track_count {
            dest.write_char(' ')?;
        }
    }
    if let Some(names) = _cx.ast_context().vec_get(line_names, track_count)
        && !names.is_empty()
    {
        dest.write_char(' ')?;
        crate::values::grid::write_line_names(
            _cx.ast_context()
                .vec_iter(names)
                .map(|name| _cx.ast_context().str(name)),
            dest,
        )?;
    }
    dest.write_char(')')
}

impl<'ghost> ToCss<'ghost> for GridAutoFlow {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.direction.to_css(dest, _cx)?;
        if self.dense {
            dest.write_str(" dense")?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for GridTemplate<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.rows.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.columns.to_css(dest, _cx)?;
        if !matches!(
            _cx.ast_context().resolve_node(self.areas),
            GridTemplateAreas::None
        ) {
            dest.write_char(' ')?;
            self.areas.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

fn write_track_sizes<'ghost, PrinterT, I>(
    values: I,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    PrinterT: PrinterTrait,
    I: IntoIterator,
    I::Item: ToCss<'ghost>,
{
    for (index, value) in values.into_iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for Grid<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let grid = cx.ast_context().grid(id);
        write_grid(
            grid.rows(),
            grid.columns(),
            grid.areas(),
            || (grid.auto_flow(), grid.auto_rows(), grid.auto_columns()),
            dest,
            cx,
        )
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_grid(
            self.rows,
            self.columns,
            self.areas,
            || (self.auto_flow, self.auto_rows, self.auto_columns),
            dest,
            cx,
        )
    }
}

fn write_grid<'id, 'ghost, PrinterT: PrinterTrait>(
    rows: NodeId<'id, TrackSizing<'id>>,
    columns: NodeId<'id, TrackSizing<'id>>,
    areas: NodeId<'id, GridTemplateAreas<'id>>,
    auto: impl FnOnce() -> (
        GridAutoFlow,
        AstVec<'id, NodeId<'id, TrackSize<'id>>>,
        AstVec<'id, NodeId<'id, TrackSize<'id>>>,
    ),
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    rows.to_css(dest, _cx)?;
    dest.write_str(" / ")?;
    columns.to_css(dest, _cx)?;
    dest.write_str(" auto-flow ")?;
    let (auto_flow, auto_rows, auto_columns) = auto();
    auto_flow.to_css(dest, _cx)?;
    if !auto_rows.is_empty() {
        dest.write_char(' ')?;
        write_track_sizes(_cx.ast_context().vec_iter(auto_rows), dest, _cx)?;
    }
    if !auto_columns.is_empty() {
        dest.write_str(" / ")?;
        write_track_sizes(_cx.ast_context().vec_iter(auto_columns), dest, _cx)?;
    }
    let areas = _cx.ast_context().resolve_node(areas);
    if !matches!(areas, GridTemplateAreas::None) {
        dest.write_char(' ')?;
        areas.to_css(dest, _cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for GridArea<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.row_start.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.column_start.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.row_end.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.column_end.to_css(dest, _cx)
    }
}
