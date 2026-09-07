use super::*;

keyword_values! {
    AutoFlowDirection,
}

pub(crate) fn write_line_names<PrinterT, I>(names: I, dest: &mut PrinterT) -> fmt::Result
where
    PrinterT: PrinterTrait,
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut names = names.into_iter().peekable();
    if names.peek().is_none() {
        return Ok(());
    }
    dest.write_char('[')?;
    for (index, name) in names.enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        serialize_identifier(name.as_ref(), dest)?;
    }
    dest.write_char(']')
}

impl<'ghost> ToCss<'ghost> for TrackSizing<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        match cx.ast_context().track_sizing(id) {
            None => dest.write_str("none"),
            Some(list) => write_track_list(list.items(), list.line_names(), dest, cx),
        }
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::TrackList { items, line_names } => {
                write_track_list(*items, *line_names, dest, cx)
            }
        }
    }
}

fn write_track_list<'id, 'ghost, PrinterT: PrinterTrait>(
    items: AstVec<'id, NodeId<'id, TrackListItem<'id>>>,
    line_names: AstVec<'id, AstVec<'id, AstStr<'id>>>,
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let mut wrote_value = false;
    let item_count = _cx.ast_context().vec_len(items);
    for (index, item) in _cx.ast_context().vec_iter(items).enumerate() {
        if let Some(names) = _cx.ast_context().vec_get(line_names, index)
            && !names.is_empty()
        {
            if wrote_value {
                dest.write_char(' ')?;
            }
            write_line_names(
                _cx.ast_context()
                    .vec_iter(names)
                    .map(|name| _cx.ast_context().str(name)),
                dest,
            )?;
            wrote_value = true;
        }
        if wrote_value {
            dest.write_char(' ')?;
        }
        item.to_css(dest, _cx)?;
        wrote_value = true;
    }
    if let Some(names) = _cx.ast_context().vec_get(line_names, item_count)
        && !names.is_empty()
    {
        if wrote_value {
            dest.write_char(' ')?;
        }
        write_line_names(
            _cx.ast_context()
                .vec_iter(names)
                .map(|name| _cx.ast_context().str(name)),
            dest,
        )?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for TrackListItem<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::TrackSize(value) => value.to_css(dest, _cx),
            Self::TrackRepeat(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TrackSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::TrackBreadth(value) => value.to_css(dest, _cx),
            Self::MinMax { max, min } => {
                dest.write_str("minmax(")?;
                min.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                max.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::FitContent(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for TrackBreadth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest, _cx),
            Self::Flex(value) => serialize_dimension(*value, &Unit::Flex, dest, _cx),
            Self::MinContent => dest.write_str("min-content"),
            Self::MaxContent => dest.write_str("max-content"),
            Self::Auto => dest.write_str("auto"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for RepeatCount {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::AutoFill => dest.write_str("auto-fill"),
            Self::AutoFit => dest.write_str("auto-fit"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for GridTemplateAreas<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Areas { areas, columns } => {
                let columns = *columns as usize;
                if columns == 0 {
                    return Ok(());
                }
                let mut output = String::new();
                let count = _cx.ast_context().vec_len(*areas);
                for (index, value) in _cx.ast_context().vec_iter(*areas).enumerate() {
                    if index % columns > 0 {
                        output.push(' ');
                    }
                    output.push_str(value.map(|name| _cx.ast_context().str(name)).unwrap_or("."));
                    if (index + 1) % columns == 0 || index + 1 == count {
                        if index >= columns {
                            dest.write_char(' ')?;
                        }
                        serialize_string(&output, dest)?;
                        output.clear();
                    }
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for GridLine<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Area { name } => serialize_identifier(_cx.ast_context().str(*name), dest),
            Self::Line { index, name } => {
                if *index != 0 {
                    serialize_int(*index, dest)?;
                    if name.is_some() {
                        dest.write_char(' ')?;
                    }
                }
                if let Some(name) = name {
                    serialize_identifier(_cx.ast_context().str(*name), dest)?;
                }
                Ok(())
            }
            Self::Span { index, name } => {
                dest.write_str("span")?;
                if *index != 0 {
                    dest.write_char(' ')?;
                    serialize_int(*index, dest)?;
                }
                if let Some(name) = name {
                    dest.write_char(' ')?;
                    serialize_identifier(_cx.ast_context().str(*name), dest)?;
                }
                Ok(())
            }
        }
    }
}
