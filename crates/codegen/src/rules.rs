use crate::prelude::*;

fn write_space_separated<'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost>>(
    values: &[&T],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
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
    cx: &ToCssContext<'_, '_, 'ghost>,
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
    cx: &ToCssContext<'_, '_, 'ghost>,
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
    cx: &ToCssContext<'_, '_, 'ghost>,
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

fn write_node_pair<'ast, 'ghost, PrinterT, T>(
    first: &NodeId<'ast, T>,
    second: &NodeId<'ast, T>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    PrinterT: PrinterTrait,
    T: ToCss<'ghost> + PartialEq,
{
    first.to_css(dest, cx)?;
    if !css_values_are_equal(
        cx.ast_context().resolve_node(*first),
        cx.ast_context().resolve_node(*second),
        cx,
    ) {
        dest.write_char(' ')?;
        second.to_css(dest, cx)?;
    }
    Ok(())
}

fn write_four_nodes<'ast, 'ghost, PrinterT, T>(
    top: &NodeId<'ast, T>,
    right: &NodeId<'ast, T>,
    bottom: &NodeId<'ast, T>,
    left: &NodeId<'ast, T>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    PrinterT: PrinterTrait,
    T: ToCss<'ghost> + PartialEq,
{
    let ast = cx.ast_context();
    let top_value = ast.resolve_node(*top);
    let right_value = ast.resolve_node(*right);
    let bottom_value = ast.resolve_node(*bottom);
    let left_value = ast.resolve_node(*left);
    top.to_css(dest, cx)?;
    if css_values_are_equal(top_value, right_value, cx)
        && css_values_are_equal(top_value, bottom_value, cx)
        && css_values_are_equal(top_value, left_value, cx)
    {
        return Ok(());
    }
    dest.write_char(' ')?;
    right.to_css(dest, cx)?;
    if css_values_are_equal(top_value, bottom_value, cx)
        && css_values_are_equal(right_value, left_value, cx)
    {
        return Ok(());
    }
    dest.write_char(' ')?;
    bottom.to_css(dest, cx)?;
    if !css_values_are_equal(right_value, left_value, cx) {
        dest.write_char(' ')?;
        left.to_css(dest, cx)?;
    }
    Ok(())
}

fn write_color_pair<'ast, 'ghost, PrinterT: PrinterTrait>(
    first: &NodeId<'ast, CssColor<'ast>>,
    second: &NodeId<'ast, CssColor<'ast>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    write_node_pair(first, second, dest, cx)
}

fn write_four_colors<'ast, 'ghost, PrinterT: PrinterTrait>(
    top: &NodeId<'ast, CssColor<'ast>>,
    right: &NodeId<'ast, CssColor<'ast>>,
    bottom: &NodeId<'ast, CssColor<'ast>>,
    left: &NodeId<'ast, CssColor<'ast>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    write_four_nodes(top, right, bottom, left, dest, cx)
}

impl<'ghost> ToCss<'ghost> for Position<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        self.x.to_css(dest, _cx)?;
        if !matches!(ast.resolve_node(self.y), PositionComponent::Center)
            || matches!(
                ast.resolve_node(self.x),
                PositionComponent::Side {
                    offset: Some(_),
                    ..
                }
            )
        {
            dest.write_char(' ')?;
            self.y.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for WebKitGradientPoint {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.x.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.y.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for WebKitColorStop<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.position == 0.0 {
            dest.write_str("from(")?;
            self.color.to_css(dest, _cx)?;
        } else if self.position == 1.0 {
            dest.write_str("to(")?;
            self.color.to_css(dest, _cx)?;
        } else {
            dest.write_str("color-stop(")?;
            serialize_number(self.position, dest)?;
            dest.delim(Delimiter::Comma)?;
            self.color.to_css(dest, _cx)?;
        }
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for ImageSet<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.vendor_prefix.to_css(dest, _cx)?;
        dest.write_str("image-set(")?;
        write_comma_separated(_cx.ast_context().vec(self.options), dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for ImageSetOption<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.image.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.resolution.to_css(dest, _cx)?;
        if let Some(file_type) = self.file_type {
            dest.write_str(" type(")?;
            serialize_string(file_type, dest)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for BackgroundPosition<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        self.x.to_css(dest, _cx)?;
        if !matches!(ast.resolve_node(self.y), PositionComponent::Center)
            || matches!(
                ast.resolve_node(self.x),
                PositionComponent::Side {
                    offset: Some(_),
                    ..
                }
            )
        {
            dest.write_char(' ')?;
            self.y.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for BackgroundRepeat {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match (&self.x, &self.y) {
            (BackgroundRepeatKeyword::Repeat, BackgroundRepeatKeyword::NoRepeat) => {
                dest.write_str("repeat-x")
            }
            (BackgroundRepeatKeyword::NoRepeat, BackgroundRepeatKeyword::Repeat) => {
                dest.write_str("repeat-y")
            }
            _ => write_pair(&self.x, &self.y, dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Background<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        if matches!(ast.resolve_node(self.image), Image::None)
            && is_zero_background_position(&self.position, _cx)
            && matches!(
                ast.resolve_node(self.size),
                BackgroundSize::Explicit { height, width }
                    if matches!(ast.resolve_node(*height), LengthPercentageOrAuto::Auto)
                        && matches!(ast.resolve_node(*width), LengthPercentageOrAuto::Auto)
            )
            && matches!(
                &self.repeat,
                BackgroundRepeat {
                    x: BackgroundRepeatKeyword::Repeat,
                    y: BackgroundRepeatKeyword::Repeat,
                }
            )
            && self.attachment == BackgroundAttachment::Scroll
            && self.origin == BackgroundOrigin::PaddingBox
            && self.clip == BackgroundClip::BorderBox
        {
            return self.color.to_css(dest, _cx);
        }

        self.image.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.position.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.size.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.repeat.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.attachment.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.origin.to_css(dest, _cx)?;
        if self.clip
            != match &self.origin {
                BackgroundOrigin::BorderBox => BackgroundClip::BorderBox,
                BackgroundOrigin::PaddingBox => BackgroundClip::PaddingBox,
                BackgroundOrigin::ContentBox => BackgroundClip::ContentBox,
            }
        {
            dest.write_char(' ')?;
            self.clip.to_css(dest, _cx)?;
        }
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

fn is_zero_position_components<'ast, Sx, Sy>(
    x: &NodeId<'ast, PositionComponent<'ast, Sx>>,
    y: &NodeId<'ast, PositionComponent<'ast, Sy>>,
    cx: &ToCssContext<'_, '_, '_>,
) -> bool {
    fn is_zero(
        component: &PositionComponent<'_, impl Sized>,
        cx: &ToCssContext<'_, '_, '_>,
    ) -> bool {
        matches!(
            component,
            PositionComponent::Length(value)
                if matches!(
                    cx.ast_context().resolve_node(*value),
                    DimensionPercentage::Percentage(0.0) | DimensionPercentage::Zero
                )
        )
    }

    let ast = cx.ast_context();
    is_zero(ast.resolve_node(*x), cx) && is_zero(ast.resolve_node(*y), cx)
}

fn is_zero_background_position(
    position: &NodeId<'_, BackgroundPosition<'_>>,
    cx: &ToCssContext<'_, '_, '_>,
) -> bool {
    let position = cx.ast_context().resolve_node(*position);
    is_zero_position_components(&position.x, &position.y, cx)
}

fn is_zero_position(position: &NodeId<'_, Position<'_>>, cx: &ToCssContext<'_, '_, '_>) -> bool {
    let position = cx.ast_context().resolve_node(*position);
    is_zero_position_components(&position.x, &position.y, cx)
}

impl<'ghost> ToCss<'ghost> for BoxShadow<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.inset {
            dest.write_str("inset ")?;
        }
        write_space_separated(
            &[&self.x_offset, &self.y_offset, &self.blur, &self.spread],
            dest,
            _cx,
        )?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

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

macro_rules! logical_pair {
    ($($ty:ty, $first:ident, $second:ident);+ $(;)?) => {
        $(
            impl<'ghost> ToCss<'ghost> for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, 'ghost>) -> fmt::Result {
                    write_node_pair(&self.$first, &self.$second, dest, _cx)
                }
            }
        )+
    };
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

macro_rules! physical_four {
    ($($ty:ty);+ $(;)?) => {
        $(
            impl<'ghost> ToCss<'ghost> for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, 'ghost>) -> fmt::Result {
                    write_four_nodes(&self.top, &self.right, &self.bottom, &self.left, dest, _cx)
                }
            }
        )+
    };
}

physical_four! {
    Inset<'_>;
    Margin<'_>;
    Padding<'_>;
    ScrollMargin<'_>;
    ScrollPadding<'_>;
    BorderWidth<'_>;
}

impl<'ghost> ToCss<'ghost> for BorderColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_four_colors(&self.top, &self.right, &self.bottom, &self.left, dest, cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_four(&self.top, &self.right, &self.bottom, &self.left, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderRadius<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        let top_left = ast.resolve_node(self.top_left);
        let top_right = ast.resolve_node(self.top_right);
        let bottom_right = ast.resolve_node(self.bottom_right);
        let bottom_left = ast.resolve_node(self.bottom_left);
        write_four_nodes(
            &top_left.0,
            &top_right.0,
            &bottom_right.0,
            &bottom_left.0,
            dest,
            _cx,
        )?;
        if !css_values_are_equal(
            ast.resolve_node(top_left.0),
            ast.resolve_node(top_left.1),
            _cx,
        ) || !css_values_are_equal(
            ast.resolve_node(top_right.0),
            ast.resolve_node(top_right.1),
            _cx,
        ) || !css_values_are_equal(
            ast.resolve_node(bottom_right.0),
            ast.resolve_node(bottom_right.1),
            _cx,
        ) || !css_values_are_equal(
            ast.resolve_node(bottom_left.0),
            ast.resolve_node(bottom_left.1),
            _cx,
        ) {
            dest.write_str(" / ")?;
            write_four_nodes(
                &top_left.1,
                &top_right.1,
                &bottom_right.1,
                &bottom_left.1,
                dest,
                _cx,
            )?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for BorderImageRepeat {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_pair(&self.horizontal, &self.vertical, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderImageSlice<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.offsets.to_css(dest, _cx)?;
        if self.fill {
            dest.write_str(" fill")?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for BorderImage<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.source.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.slice.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.width.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.outset.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.repeat.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderBlockColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_color_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderBlockStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderBlockWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_node_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderInlineColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_color_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderInlineStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderInlineWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_node_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost, S: ToCss<'ghost>> ToCss<'ghost> for GenericBorder<'_, S> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.width.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
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
        if self.grow == 0.0
            && self.shrink == 0.0
            && matches!(ast.resolve_node(self.basis), LengthPercentageOrAuto::Auto)
        {
            return dest.write_str("none");
        }
        if self.grow == 1.0
            && self.shrink == 1.0
            && matches!(ast.resolve_node(self.basis), LengthPercentageOrAuto::Auto)
        {
            return dest.write_str("auto");
        }

        serialize_number(self.grow, dest)?;
        let basis_is_zero = matches!(
            ast.resolve_node(self.basis),
            LengthPercentageOrAuto::LengthPercentage(value)
                if matches!(ast.resolve_node(*value), LengthPercentage::Zero)
        );
        let basis_is_auto = matches!(ast.resolve_node(self.basis), LengthPercentageOrAuto::Auto);
        if self.shrink != 1.0 || (!basis_is_zero && !basis_is_auto) {
            dest.write_char(' ')?;
            serialize_number(self.shrink, dest)?;
        }
        if !basis_is_zero {
            dest.write_char(' ')?;
            self.basis.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

macro_rules! place_pair {
    ($($ty:ty);+ $(;)?) => {
        $(
            impl<'ghost> ToCss<'ghost> for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, 'ghost>) -> fmt::Result {
                    self.align.to_css(dest, _cx)?;
                    dest.write_char(' ')?;
                    self.justify.to_css(dest, _cx)
                }
            }
        )+
    };
}

place_pair! { PlaceContent; PlaceSelf; PlaceItems }

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
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("repeat(")?;
        self.count.to_css(dest, _cx)?;
        dest.delim(Delimiter::Comma)?;
        let track_sizes = _cx.ast_context().vec(self.track_sizes);
        let line_names = _cx.ast_context().vec(self.line_names);
        for (index, track_size) in track_sizes.iter().enumerate() {
            if let Some(names) = line_names.get(index)
                && !names.is_empty()
            {
                crate::values::write_line_names(_cx.ast_context().vec(*names), dest)?;
                dest.write_char(' ')?;
            }
            track_size.to_css(dest, _cx)?;
            if index + 1 < track_sizes.len() {
                dest.write_char(' ')?;
            }
        }
        if let Some(names) = line_names.get(track_sizes.len())
            && !names.is_empty()
        {
            dest.write_char(' ')?;
            crate::values::write_line_names(_cx.ast_context().vec(*names), dest)?;
        }
        dest.write_char(')')
    }
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

fn write_track_sizes<'ghost, PrinterT: PrinterTrait>(
    values: &[TrackSize<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for Grid<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.rows.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.columns.to_css(dest, _cx)?;
        dest.write_str(" auto-flow ")?;
        self.auto_flow.to_css(dest, _cx)?;
        if !self.auto_rows.is_empty() {
            dest.write_char(' ')?;
            write_track_sizes(_cx.ast_context().vec(self.auto_rows), dest, _cx)?;
        }
        if !self.auto_columns.is_empty() {
            dest.write_str(" / ")?;
            write_track_sizes(_cx.ast_context().vec(self.auto_columns), dest, _cx)?;
        }
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

macro_rules! grid_pair {
    ($($ty:ty);+ $(;)?) => {
        $(
            impl<'ghost> ToCss<'ghost> for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, 'ghost>) -> fmt::Result {
                    self.start.to_css(dest, _cx)?;
                    dest.write_str(" / ")?;
                    self.end.to_css(dest, _cx)
                }
            }
        )+
    };
}

grid_pair! { GridRow<'_>; GridColumn<'_> }

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

impl<'ghost> ToCss<'ghost> for Font<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.variant_caps.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.weight.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.stretch.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.size.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.line_height.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        write_comma_separated(_cx.ast_context().vec(self.family), dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Transition<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.property.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.duration.to_css(dest, _cx)?;
        if !matches!(
            _cx.ast_context().resolve_node(self.timing_function),
            EasingFunction::Ease
        ) {
            dest.write_char(' ')?;
            self.timing_function.to_css(dest, _cx)?;
        }
        if !matches!(self.delay, Time::Seconds(0.0) | Time::Milliseconds(0.0)) {
            dest.write_char(' ')?;
            self.delay.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for ScrollTimeline {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.scroller.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.axis.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ViewTimeline<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.axis.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.inset.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for AnimationRange<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.start.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.end.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Animation<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        // Components print in their stored order: authored order after
        // parsing, canonical order after the ORDER_VALUES minify pass, which
        // also moves a name colliding with a keyword class behind that class.
        let components = ast.vec(self.components);
        for (index, component) in components.iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            // A quoted name colliding with a keyword class must stay quoted
            // unless the class appears before it; unquoted it would reparse
            // into the class slot.
            if let AnimationComponent::Name(name) = component
                && let name = ast.resolve_node(*name)
                && let AnimationName::String(value) = name
                && name.keyword_class().is_some_and(|class| {
                    !components[..index]
                        .iter()
                        .any(|component| component.keyword_class() == Some(class))
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

impl<'ghost> ToCss<'ghost> for AnimationComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Name(value) => value.to_css(dest, _cx),
            Self::Duration(value) | Self::Delay(value) => value.to_css(dest, _cx),
            Self::TimingFunction(value) => value.to_css(dest, _cx),
            Self::IterationCount(value) => value.to_css(dest, _cx),
            Self::Direction(value) => value.to_css(dest, _cx),
            Self::FillMode(value) => value.to_css(dest, _cx),
            Self::PlayState(value) => value.to_css(dest, _cx),
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

impl<'ghost> ToCss<'ghost> for MatrixForFloat {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("matrix(")?;
        write_numbers(&[self.a, self.b, self.c, self.d, self.e, self.f], dest)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for Matrix3DForFloat {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("matrix3d(")?;
        write_numbers(
            &[
                self.m11, self.m12, self.m13, self.m14, self.m21, self.m22, self.m23, self.m24,
                self.m31, self.m32, self.m33, self.m34, self.m41, self.m42, self.m43, self.m44,
            ],
            dest,
        )?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for Rotate {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.x == 0.0 && self.y == 0.0 && self.z == 1.0 {
            return self.angle.to_css(dest, _cx);
        }
        write_numbers(&[self.x, self.y, self.z], dest)?;
        dest.write_char(' ')?;
        self.angle.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for TextTransform {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.case.to_css(dest, _cx)?;
        if self.full_width {
            dest.write_str(" full-width")?;
        }
        if self.full_size_kana {
            dest.write_str(" full-size-kana")?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for TextIndent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.value.to_css(dest, _cx)?;
        if self.hanging {
            dest.write_str(" hanging")?;
        }
        if self.each_line {
            dest.write_str(" each-line")?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for TextDecoration<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.line.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.thickness.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for TextEmphasis<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for TextEmphasisPosition {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.vertical.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.horizontal.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for TextShadow<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_space_separated(
            &[&self.x_offset, &self.y_offset, &self.blur, &self.spread],
            dest,
            _cx,
        )?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Cursor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        for image in _cx.ast_context().vec(self.images) {
            image.to_css(dest, _cx)?;
            dest.delim(Delimiter::Comma)?;
        }
        self.keyword.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for CursorImage<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.url.to_css(dest, _cx)?;
        if let Some((x, y)) = self.hotspot {
            dest.write_char(' ')?;
            serialize_number(x, dest)?;
            dest.write_char(' ')?;
            serialize_number(y, dest)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for Caret<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.color.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.shape.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ListStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.position.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.image.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.list_style_type.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Composes<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        for (index, name) in _cx.ast_context().vec(self.names).iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            serialize_identifier(name, dest)?;
        }
        if let Some(from) = &self.from {
            dest.write_str(" from ")?;
            from.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for InsetRect<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("inset(")?;
        self.rect.to_css(dest, _cx)?;
        dest.write_str(" round ")?;
        self.radius.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for CircleShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("circle(")?;
        self.radius.to_css(dest, _cx)?;
        dest.write_str(" at ")?;
        self.position.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for EllipseShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("ellipse(")?;
        self.radius_x.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.radius_y.to_css(dest, _cx)?;
        dest.write_str(" at ")?;
        self.position.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for Polygon<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("polygon(")?;
        self.fill_rule.to_css(dest, _cx)?;
        dest.delim(Delimiter::Comma)?;
        write_comma_separated(_cx.ast_context().vec(self.points), dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for Point<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.x.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.y.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Mask<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        self.image.to_css(dest, _cx)?;

        let default_size = matches!(
            ast.resolve_node(self.size),
            BackgroundSize::Explicit { height, width }
                if matches!(ast.resolve_node(*height), LengthPercentageOrAuto::Auto)
                    && matches!(ast.resolve_node(*width), LengthPercentageOrAuto::Auto)
        );
        if !is_zero_position(&self.position, _cx) || !default_size {
            dest.write_char(' ')?;
            write_mask_position(ast.resolve_node(self.position), dest, _cx)?;
            if !default_size {
                if dest.prettify() {
                    dest.write_str(" / ")?;
                } else {
                    dest.write_char('/')?;
                }
                write_mask_size(ast.resolve_node(self.size), dest, _cx)?;
            }
        }

        if self.repeat
            != (BackgroundRepeat {
                x: BackgroundRepeatKeyword::Repeat,
                y: BackgroundRepeatKeyword::Repeat,
            })
        {
            dest.write_char(' ')?;
            self.repeat.to_css(dest, _cx)?;
        }

        let clip_matches_origin = matches!(
            (&self.clip, &self.origin),
            (MaskClip::GeometryBox(clip), origin) if clip == origin
        );
        if self.origin != GeometryBox::BorderBox || !clip_matches_origin {
            dest.write_char(' ')?;
            self.origin.to_css(dest, _cx)?;
            if !clip_matches_origin {
                dest.write_char(' ')?;
                self.clip.to_css(dest, _cx)?;
            }
        }

        if self.composite != MaskComposite::Add {
            dest.write_char(' ')?;
            self.composite.to_css(dest, _cx)?;
        }
        if self.mode != MaskMode::MatchSource {
            dest.write_char(' ')?;
            self.mode.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

fn write_mask_position<'ghost, PrinterT: PrinterTrait>(
    value: &Position<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast_context();
    write_mask_horizontal_position(ast.resolve_node(value.x), dest, cx)?;
    if !matches!(ast.resolve_node(value.y), PositionComponent::Center)
        || matches!(
            ast.resolve_node(value.x),
            PositionComponent::Side {
                offset: Some(_),
                ..
            }
        )
    {
        dest.write_char(' ')?;
        write_mask_vertical_position(ast.resolve_node(value.y), dest, cx)?;
    }
    Ok(())
}

fn write_mask_size<'ghost, PrinterT: PrinterTrait>(
    value: &BackgroundSize<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if let BackgroundSize::Explicit { height, width } = value
        && matches!(
            cx.ast_context().resolve_node(*height),
            LengthPercentageOrAuto::Auto
        )
    {
        return width.to_css(dest, cx);
    }
    value.to_css(dest, cx)
}

fn write_mask_horizontal_position<'ghost, PrinterT: PrinterTrait>(
    value: &PositionComponent<'_, HorizontalPositionKeyword>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if !dest.prettify()
        && let PositionComponent::Side { offset: None, side } = value
    {
        return dest.write_str(match side {
            HorizontalPositionKeyword::Left => "0",
            HorizontalPositionKeyword::Right => "100%",
        });
    }
    value.to_css(dest, cx)
}

fn write_mask_vertical_position<'ghost, PrinterT: PrinterTrait>(
    value: &PositionComponent<'_, VerticalPositionKeyword>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if !dest.prettify()
        && let PositionComponent::Side { offset: None, side } = value
    {
        return dest.write_str(match side {
            VerticalPositionKeyword::Top => "0",
            VerticalPositionKeyword::Bottom => "100%",
        });
    }
    value.to_css(dest, cx)
}

impl<'ghost> ToCss<'ghost> for MaskBorder<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.source.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.slice.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.width.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.outset.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.repeat.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.mode.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for DropShadow<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_space_separated(&[&self.x_offset, &self.y_offset, &self.blur], dest, _cx)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Container<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.name.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.container_type.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ColorScheme {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.only {
            dest.write_str("only ")?;
        }
        match (self.light, self.dark) {
            (true, true) => dest.write_str("light dark"),
            (true, false) => dest.write_str("light"),
            (false, true) => dest.write_str("dark"),
            (false, false) => dest.write_str("normal"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for UnparsedProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if let Some(raw_value) = self.raw_value {
            dest.write_str(raw_value)
        } else {
            crate::token::write_unparsed_token_list(_cx.ast_context().vec(self.value), dest, _cx)
        }
    }
}

impl<'ghost> ToCss<'ghost> for CustomProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        crate::token::write_token_list(_cx.ast_context().vec(self.value), dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for FamilyName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        crate::values::font::write_custom_font_family(self.0, dest)
    }
}

impl<'ghost> ToCss<'ghost> for KeyframeSelector {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::From => dest.write_str("from"),
            Self::To => dest.write_str("to"),
            Self::TimelineRangePercentage(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for KeyframesName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Ident(value) => serialize_identifier(value, dest),
            Self::Custom(value) => serialize_string(value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontFaceProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Source(values) => {
                write_comma_separated(_cx.ast_context().vec(*values), dest, _cx)
            }
            Self::FontFamily(value) => value.to_css(dest, _cx),
            Self::FontStyle(value) => value.to_css(dest, _cx),
            Self::FontWeight(value) => value.to_css(dest, _cx),
            Self::FontStretch(value) => value.to_css(dest, _cx),
            Self::UnicodeRange(values) => {
                write_comma_separated(_cx.ast_context().vec(*values), dest, _cx)
            }
            Self::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

pub(crate) trait NamedProperty {
    fn css_name<'a>(&'a self, ast: &'a Compilation<'_>) -> &'a str;
}

impl NamedProperty for FontFaceProperty<'_> {
    fn css_name<'a>(&'a self, ast: &'a Compilation<'_>) -> &'a str {
        match self {
            FontFaceProperty::Source(_) => "src",
            FontFaceProperty::FontFamily(_) => "font-family",
            FontFaceProperty::FontStyle(_) => "font-style",
            FontFaceProperty::FontWeight(_) => "font-weight",
            FontFaceProperty::FontStretch(_) => "font-stretch",
            FontFaceProperty::UnicodeRange(_) => "unicode-range",
            FontFaceProperty::Custom(value) => {
                match ast.resolve_node(ast.resolve_node(*value).name) {
                    CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
                }
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Source<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Local(value) => {
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("font technologies are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for FontFaceStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Italic => dest.write_str("italic"),
            Self::Oblique(value) => {
                dest.write_str("oblique")?;
                let value = _cx.ast_context().resolve_node(*value);
                let is_default = matches!(
                    (
                        _cx.ast_context().resolve_node(value.0),
                        _cx.ast_context().resolve_node(value.1),
                    ),
                    (Angle::Deg(first), Angle::Deg(second)) if *first == 14.0 && *second == 14.0
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

impl<'ghost> ToCss<'ghost> for FontPaletteValuesProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::FontFamily(value) => value.to_css(dest, _cx),
            Self::BasePalette(value) => value.to_css(dest, _cx),
            Self::OverrideColors(values) => {
                write_comma_separated(_cx.ast_context().vec(*values), dest, _cx)
            }
            Self::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

impl NamedProperty for FontPaletteValuesProperty<'_> {
    fn css_name<'a>(&'a self, ast: &'a Compilation<'_>) -> &'a str {
        match self {
            FontPaletteValuesProperty::FontFamily(_) => "font-family",
            FontPaletteValuesProperty::BasePalette(_) => "base-palette",
            FontPaletteValuesProperty::OverrideColors(_) => "override-colors",
            FontPaletteValuesProperty::Custom(value) => {
                match ast.resolve_node(ast.resolve_node(*value).name) {
                    CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
                }
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for BasePalette {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("page pseudo classes are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for ParsedComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest, _cx),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::String(value) => serialize_string(value, dest),
            Self::Color(value) => value.to_css(dest, _cx),
            Self::Image(value) => value.to_css(dest, _cx),
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Integer(value) => serialize_int(*value, dest),
            Self::Angle(value) => value.to_css(dest, _cx),
            Self::Time(value) => value.to_css(dest, _cx),
            Self::Resolution(value) => value.to_css(dest, _cx),
            Self::TransformFunction(value) => value.to_css(dest, _cx),
            Self::TransformList(values) => {
                for (index, value) in _cx.ast_context().vec(*values).iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::CustomIdent(value) => serialize_identifier(value, dest),
            Self::Literal(value) => dest.write_str(value),
            Self::Repeated {
                components,
                multiplier,
            } => {
                let delimiter = match multiplier {
                    Multiplier::None => "",
                    Multiplier::Space => " ",
                    Multiplier::Comma => ", ",
                };
                for (index, value) in _cx.ast_context().vec(*components).iter().enumerate() {
                    if index > 0 {
                        dest.write_str(delimiter)?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::TokenList(values) => {
                crate::token::write_token_list(_cx.ast_context().vec(*values), dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Multiplier {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(match self {
            Self::None => "",
            Self::Space => "+",
            Self::Comma => "#",
        })
    }
}

impl<'ghost> ToCss<'ghost> for SyntaxString<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Universal => dest.write_char('*'),
            Self::Components(values) => {
                for (index, value) in _cx.ast_context().vec(*values).iter().enumerate() {
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

impl<'ghost> ToCss<'ghost> for SyntaxComponentKind<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Literal(value) => dest.write_str(value),
            value => {
                dest.write_char('<')?;
                dest.write_str(
                    value
                        .as_css_str()
                        .expect("literal syntax component handled separately"),
                )?;
                dest.write_char('>')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for SyntaxComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.kind.to_css(dest, _cx)?;
        self.multiplier.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ContainerSizeFeatureId {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("container size features are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for ScrollStateFeatureId {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("scroll state features are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for ContainerCondition<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Feature(value) => value.to_css(dest, _cx),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in _cx.ast_context().vec(*conditions).iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::Style(value) => {
                dest.write_str("style(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::ScrollState(value) => {
                dest.write_str("scroll-state(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::Unknown(values) => {
                crate::token::write_token_list(_cx.ast_context().vec(*values), dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for StyleQuery<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Declaration(value) => value.to_css(dest, _cx),
            Self::Property(value) => value.to_css(dest, _cx),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in _cx.ast_context().vec(*conditions).iter().enumerate() {
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

impl<'ghost> ToCss<'ghost> for ScrollStateQuery<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Feature(value) => value.to_css(dest, _cx),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in _cx.ast_context().vec(*conditions).iter().enumerate() {
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

impl<'ghost> ToCss<'ghost> for ViewTransitionProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Navigation(value) => value.to_css(dest, _cx),
            Self::Types(value) => value.to_css(dest, _cx),
            Self::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

impl NamedProperty for ViewTransitionProperty<'_> {
    fn css_name<'a>(&'a self, ast: &'a Compilation<'_>) -> &'a str {
        match self {
            ViewTransitionProperty::Navigation(_) => "navigation",
            ViewTransitionProperty::Types(_) => "types",
            ViewTransitionProperty::Custom(value) => {
                match ast.resolve_node(ast.resolve_node(*value).name) {
                    CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
                }
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Navigation {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("navigation values are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for ImportRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@import ")?;
        serialize_string(self.url, dest)?;
        if let Some(layer) = &self.layer {
            dest.write_str(" layer")?;
            if !layer.is_empty() {
                dest.write_char('(')?;
                write_layer_name(_cx.ast_context().vec(*layer), dest)?;
                dest.write_char(')')?;
            }
        }
        if let Some(supports) = &self.supports {
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
        if let Some(media) = &self.media {
            dest.write_char(' ')?;
            media.to_css(dest, _cx)?;
        }
        dest.write_char(';')
    }
}

impl<'ghost> ToCss<'ghost> for TimelineRangePercentage {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.name.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        serialize_number(self.percentage * 100.0, dest)?;
        dest.write_char('%')
    }
}

impl<'ghost> ToCss<'ghost> for UrlSource<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.url.to_css(dest, _cx)?;
        if let Some(format) = &self.format {
            dest.write_str(" format(")?;
            format.to_css(dest, _cx)?;
            dest.write_char(')')?;
        }
        if !self.tech.is_empty() {
            dest.write_str(" tech(")?;
            write_comma_separated(_cx.ast_context().vec(self.tech), dest, _cx)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for UnicodeRange {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        for wildcard_digits in 1..=6 {
            let bits = wildcard_digits * 4;
            let mask = (1_u32 << bits) - 1;
            if self.start & mask == 0 && self.end == self.start | mask {
                dest.write_str("U+")?;
                serialize_hex(self.start >> bits, 1, true, dest)?;
                for _ in 0..wildcard_digits {
                    dest.write_char('?')?;
                }
                return Ok(());
            }
        }
        dest.write_str("U+")?;
        serialize_hex(self.start, 1, true, dest)?;
        if self.start != self.end {
            dest.write_char('-')?;
            serialize_hex(self.end, 1, true, dest)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for OverrideColors<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        serialize_int(self.index, dest)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for FontFeatureDeclaration<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        serialize_identifier(self.name, dest)?;
        dest.delim(Delimiter::Colon)?;
        for (index, value) in _cx.ast_context().vec(self.values).iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            serialize_int(*value, dest)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for PageSelector<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if let Some(name) = self.name {
            serialize_identifier(name, dest)?;
        }
        for pseudo_class in _cx.ast_context().vec(self.pseudo_classes) {
            dest.write_char(':')?;
            pseudo_class.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for CharsetRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@charset ")?;
        serialize_string(self.encoding, dest)?;
        dest.write_char(';')
    }
}

impl<'ghost> ToCss<'ghost> for NamespaceRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@namespace ")?;
        if let Some(prefix) = self.prefix {
            serialize_identifier(prefix, dest)?;
            dest.write_char(' ')?;
        }
        serialize_string(self.url, dest)?;
        dest.write_char(';')
    }
}

impl<'ghost> ToCss<'ghost> for CustomMediaRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@custom-media ")?;
        dest.write_str("--")?;
        serialize_name(self.name.strip_prefix("--").unwrap_or(self.name), dest)?;
        dest.write_char(' ')?;
        self.query.to_css(dest, _cx)?;
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
