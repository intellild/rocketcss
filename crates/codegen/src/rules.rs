use crate::prelude::*;

fn write_space_separated<PrinterT: PrinterTrait, T: ToCss>(
    values: &[&T],
    dest: &mut PrinterT,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        value.to_css(dest)?;
    }
    Ok(())
}

fn write_comma_separated<PrinterT: PrinterTrait, T: ToCss>(
    values: &[T],
    dest: &mut PrinterT,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest)?;
    }
    Ok(())
}

fn write_pair<PrinterT: PrinterTrait, T: ToCss + PartialEq>(
    first: &T,
    second: &T,
    dest: &mut PrinterT,
) -> fmt::Result {
    first.to_css(dest)?;
    if first != second {
        dest.write_char(' ')?;
        second.to_css(dest)?;
    }
    Ok(())
}

fn write_four<PrinterT: PrinterTrait, T: ToCss + PartialEq>(
    top: &T,
    right: &T,
    bottom: &T,
    left: &T,
    dest: &mut PrinterT,
) -> fmt::Result {
    top.to_css(dest)?;
    if top == right && top == bottom && top == left {
        return Ok(());
    }
    dest.write_char(' ')?;
    right.to_css(dest)?;
    if top == bottom && right == left {
        return Ok(());
    }
    dest.write_char(' ')?;
    bottom.to_css(dest)?;
    if right != left {
        dest.write_char(' ')?;
        left.to_css(dest)?;
    }
    Ok(())
}

impl ToCss for Position<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.x.to_css(dest)?;
        dest.write_char(' ')?;
        self.y.to_css(dest)
    }
}

impl ToCss for WebKitGradientPoint {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.x.to_css(dest)?;
        dest.write_char(' ')?;
        self.y.to_css(dest)
    }
}

impl ToCss for WebKitColorStop<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if self.position == 0.0 {
            dest.write_str("from(")?;
            self.color.to_css(dest)?;
        } else if self.position == 1.0 {
            dest.write_str("to(")?;
            self.color.to_css(dest)?;
        } else {
            dest.write_str("color-stop(")?;
            serialize_number(self.position, dest)?;
            dest.delim(Delimiter::Comma)?;
            self.color.to_css(dest)?;
        }
        dest.write_char(')')
    }
}

impl ToCss for ImageSet<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.vendor_prefix.to_css(dest)?;
        dest.write_str("image-set(")?;
        write_comma_separated(&self.options, dest)?;
        dest.write_char(')')
    }
}

impl ToCss for ImageSetOption<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.image.to_css(dest)?;
        dest.write_char(' ')?;
        self.resolution.to_css(dest)?;
        if let Some(file_type) = self.file_type {
            dest.write_str(" type(")?;
            serialize_string(file_type, dest)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl ToCss for BackgroundPosition<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.x.to_css(dest)?;
        dest.write_char(' ')?;
        self.y.to_css(dest)
    }
}

impl ToCss for BackgroundRepeat {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&self.x, &self.y, dest)
    }
}

impl ToCss for Background<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if matches!(&*self.image, Image::None)
            && is_zero_background_position(&self.position)
            && matches!(
                &*self.size,
                BackgroundSize::Explicit { height, width }
                    if matches!(&**height, LengthPercentageOrAuto::Auto)
                        && matches!(&**width, LengthPercentageOrAuto::Auto)
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
            return self.color.to_css(dest);
        }

        self.image.to_css(dest)?;
        dest.write_char(' ')?;
        self.position.to_css(dest)?;
        dest.write_str(" / ")?;
        self.size.to_css(dest)?;
        dest.write_char(' ')?;
        self.repeat.to_css(dest)?;
        dest.write_char(' ')?;
        self.attachment.to_css(dest)?;
        dest.write_char(' ')?;
        self.origin.to_css(dest)?;
        if self.clip
            != match &self.origin {
                BackgroundOrigin::BorderBox => BackgroundClip::BorderBox,
                BackgroundOrigin::PaddingBox => BackgroundClip::PaddingBox,
                BackgroundOrigin::ContentBox => BackgroundClip::ContentBox,
            }
        {
            dest.write_char(' ')?;
            self.clip.to_css(dest)?;
        }
        dest.write_char(' ')?;
        self.color.to_css(dest)
    }
}

fn is_zero_background_position(position: &BackgroundPosition<'_>) -> bool {
    fn is_zero(component: &PositionComponent<'_, impl Sized>) -> bool {
        matches!(
            component,
            PositionComponent::Length(value)
                if matches!(&**value, DimensionPercentage::Percentage(0.0) | DimensionPercentage::Zero)
        )
    }

    is_zero(&position.x) && is_zero(&position.y)
}

impl ToCss for BoxShadow<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if self.inset {
            dest.write_str("inset ")?;
        }
        write_space_separated(
            &[&*self.x_offset, &*self.y_offset, &*self.blur, &*self.spread],
            dest,
        )?;
        dest.write_char(' ')?;
        self.color.to_css(dest)
    }
}

impl ToCss for AspectRatio {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if self.auto {
            dest.write_str("auto")?;
            if self.ratio.is_some() {
                dest.write_char(' ')?;
            }
        }
        self.ratio.to_css(dest)
    }
}

impl ToCss for Overflow {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&self.x, &self.y, dest)
    }
}

macro_rules! logical_pair {
    ($($ty:ty, $first:ident, $second:ident);+ $(;)?) => {
        $(
            impl ToCss for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
                    write_pair(&*self.$first, &*self.$second, dest)
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
            impl ToCss for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
                    write_four(&*self.top, &*self.right, &*self.bottom, &*self.left, dest)
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
    BorderColor<'_>;
    BorderWidth<'_>;
}

impl ToCss for BorderStyle {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_four(&self.top, &self.right, &self.bottom, &self.left, dest)
    }
}

impl ToCss for BorderRadius<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_four(
            &*self.top_left.0,
            &*self.top_right.0,
            &*self.bottom_right.0,
            &*self.bottom_left.0,
            dest,
        )?;
        let horizontal = [
            &*self.top_left.0,
            &*self.top_right.0,
            &*self.bottom_right.0,
            &*self.bottom_left.0,
        ];
        let vertical = [
            &*self.top_left.1,
            &*self.top_right.1,
            &*self.bottom_right.1,
            &*self.bottom_left.1,
        ];
        if horizontal != vertical {
            dest.write_str(" / ")?;
            write_four(vertical[0], vertical[1], vertical[2], vertical[3], dest)?;
        }
        Ok(())
    }
}

impl ToCss for BorderImageRepeat {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&self.horizontal, &self.vertical, dest)
    }
}

impl ToCss for BorderImageSlice<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.offsets.to_css(dest)?;
        if self.fill {
            dest.write_str(" fill")?;
        }
        Ok(())
    }
}

impl ToCss for BorderImage<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.source.to_css(dest)?;
        dest.write_char(' ')?;
        self.slice.to_css(dest)?;
        dest.write_str(" / ")?;
        self.width.to_css(dest)?;
        dest.write_str(" / ")?;
        self.outset.to_css(dest)?;
        dest.write_char(' ')?;
        self.repeat.to_css(dest)
    }
}

impl ToCss for BorderBlockColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&*self.start, &*self.end, dest)
    }
}

impl ToCss for BorderBlockStyle {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&self.start, &self.end, dest)
    }
}

impl ToCss for BorderBlockWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&*self.start, &*self.end, dest)
    }
}

impl ToCss for BorderInlineColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&*self.start, &*self.end, dest)
    }
}

impl ToCss for BorderInlineStyle {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&self.start, &self.end, dest)
    }
}

impl ToCss for BorderInlineWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&*self.start, &*self.end, dest)
    }
}

impl<S: ToCss> ToCss for GenericBorder<'_, S> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.width.to_css(dest)?;
        dest.write_char(' ')?;
        self.style.to_css(dest)?;
        dest.write_char(' ')?;
        self.color.to_css(dest)
    }
}

impl ToCss for FlexFlow {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.direction.to_css(dest)?;
        dest.write_char(' ')?;
        self.wrap.to_css(dest)
    }
}

impl ToCss for Flex<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_number(self.grow, dest)?;
        dest.write_char(' ')?;
        serialize_number(self.shrink, dest)?;
        dest.write_char(' ')?;
        self.basis.to_css(dest)
    }
}

macro_rules! place_pair {
    ($($ty:ty);+ $(;)?) => {
        $(
            impl ToCss for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
                    self.align.to_css(dest)?;
                    dest.write_char(' ')?;
                    self.justify.to_css(dest)
                }
            }
        )+
    };
}

place_pair! { PlaceContent; PlaceSelf; PlaceItems }

impl ToCss for Gap<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_pair(&*self.row, &*self.column, dest)
    }
}

impl ToCss for ColumnRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        debug_assert!(self.width.is_some() || self.style.is_some() || self.color.is_some());
        let mut wrote_value = false;
        if let Some(width) = &self.width {
            width.to_css(dest)?;
            wrote_value = true;
        }
        if let Some(style) = &self.style {
            if wrote_value {
                dest.write_char(' ')?;
            }
            style.to_css(dest)?;
            wrote_value = true;
        }
        if let Some(color) = &self.color {
            if wrote_value {
                dest.write_char(' ')?;
            }
            color.to_css(dest)?;
        }
        Ok(())
    }
}

impl ToCss for ColumnWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Length(value) => value.to_css(dest),
        }
    }
}

impl ToCss for ColumnCount {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

impl ToCss for Columns<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let width_is_auto = matches!(&self.width, ColumnWidth::Auto);
        let count_is_auto = matches!(self.count, ColumnCount::Auto);
        if width_is_auto && count_is_auto {
            return dest.write_str("auto");
        }
        if !width_is_auto {
            self.width.to_css(dest)?;
        }
        if !count_is_auto {
            if !width_is_auto {
                dest.write_char(' ')?;
            }
            self.count.to_css(dest)?;
        }
        Ok(())
    }
}

impl ToCss for TrackRepeat<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("repeat(")?;
        self.count.to_css(dest)?;
        dest.delim(Delimiter::Comma)?;
        for (index, track_size) in self.track_sizes.iter().enumerate() {
            if let Some(names) = self.line_names.get(index)
                && !names.is_empty()
            {
                crate::values::write_line_names(names, dest)?;
                dest.write_char(' ')?;
            }
            track_size.to_css(dest)?;
            if index + 1 < self.track_sizes.len() {
                dest.write_char(' ')?;
            }
        }
        if let Some(names) = self.line_names.get(self.track_sizes.len())
            && !names.is_empty()
        {
            dest.write_char(' ')?;
            crate::values::write_line_names(names, dest)?;
        }
        dest.write_char(')')
    }
}

impl ToCss for GridAutoFlow {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.direction.to_css(dest)?;
        if self.dense {
            dest.write_str(" dense")?;
        }
        Ok(())
    }
}

impl ToCss for GridTemplate<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.rows.to_css(dest)?;
        dest.write_str(" / ")?;
        self.columns.to_css(dest)?;
        if !matches!(*self.areas, GridTemplateAreas::None) {
            dest.write_char(' ')?;
            self.areas.to_css(dest)?;
        }
        Ok(())
    }
}

fn write_track_sizes<PrinterT: PrinterTrait>(
    values: &[TrackSize<'_>],
    dest: &mut PrinterT,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        value.to_css(dest)?;
    }
    Ok(())
}

impl ToCss for Grid<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.rows.to_css(dest)?;
        dest.write_str(" / ")?;
        self.columns.to_css(dest)?;
        dest.write_str(" auto-flow ")?;
        self.auto_flow.to_css(dest)?;
        if !self.auto_rows.is_empty() {
            dest.write_char(' ')?;
            write_track_sizes(&self.auto_rows, dest)?;
        }
        if !self.auto_columns.is_empty() {
            dest.write_str(" / ")?;
            write_track_sizes(&self.auto_columns, dest)?;
        }
        if !matches!(*self.areas, GridTemplateAreas::None) {
            dest.write_char(' ')?;
            self.areas.to_css(dest)?;
        }
        Ok(())
    }
}

macro_rules! grid_pair {
    ($($ty:ty);+ $(;)?) => {
        $(
            impl ToCss for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
                    self.start.to_css(dest)?;
                    dest.write_str(" / ")?;
                    self.end.to_css(dest)
                }
            }
        )+
    };
}

grid_pair! { GridRow<'_>; GridColumn<'_> }

impl ToCss for GridArea<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.row_start.to_css(dest)?;
        dest.write_str(" / ")?;
        self.column_start.to_css(dest)?;
        dest.write_str(" / ")?;
        self.row_end.to_css(dest)?;
        dest.write_str(" / ")?;
        self.column_end.to_css(dest)
    }
}

impl ToCss for Font<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.style.to_css(dest)?;
        dest.write_char(' ')?;
        self.variant_caps.to_css(dest)?;
        dest.write_char(' ')?;
        self.weight.to_css(dest)?;
        dest.write_char(' ')?;
        self.stretch.to_css(dest)?;
        dest.write_char(' ')?;
        self.size.to_css(dest)?;
        dest.write_str(" / ")?;
        self.line_height.to_css(dest)?;
        dest.write_char(' ')?;
        write_comma_separated(&self.family, dest)
    }
}

impl ToCss for Transition<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.property.to_css(dest)?;
        dest.write_char(' ')?;
        self.duration.to_css(dest)?;
        dest.write_char(' ')?;
        self.timing_function.to_css(dest)?;
        dest.write_char(' ')?;
        self.delay.to_css(dest)
    }
}

impl ToCss for ScrollTimeline {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.scroller.to_css(dest)?;
        dest.write_char(' ')?;
        self.axis.to_css(dest)
    }
}

impl ToCss for ViewTimeline<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.axis.to_css(dest)?;
        dest.write_char(' ')?;
        self.inset.to_css(dest)
    }
}

impl ToCss for AnimationRange<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.start.to_css(dest)?;
        dest.write_char(' ')?;
        self.end.to_css(dest)
    }
}

impl ToCss for Animation<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        // Components print in their stored order: authored order after
        // parsing, canonical order after the ORDER_VALUES minify pass, which
        // also moves a name colliding with a keyword class behind that class.
        for (index, component) in self.components.iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            // A quoted name colliding with a keyword class must stay quoted
            // unless the class appears before it; unquoted it would reparse
            // into the class slot.
            if let AnimationComponent::Name(name) = component
                && let AnimationName::String(value) = &**name
                && name.keyword_class().is_some_and(|class| {
                    !self.components[..index]
                        .iter()
                        .any(|component| component.keyword_class() == Some(class))
                })
            {
                serialize_string(value, dest)?;
                continue;
            }
            component.to_css(dest)?;
        }
        Ok(())
    }
}

impl ToCss for AnimationComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Name(value) => value.to_css(dest),
            Self::Duration(value) | Self::Delay(value) => value.to_css(dest),
            Self::TimingFunction(value) => value.to_css(dest),
            Self::IterationCount(value) => value.to_css(dest),
            Self::Direction(value) => value.to_css(dest),
            Self::FillMode(value) => value.to_css(dest),
            Self::PlayState(value) => value.to_css(dest),
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

impl ToCss for MatrixForFloat {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("matrix(")?;
        write_numbers(&[self.a, self.b, self.c, self.d, self.e, self.f], dest)?;
        dest.write_char(')')
    }
}

impl ToCss for Matrix3DForFloat {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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

impl ToCss for Rotate {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_numbers(&[self.x, self.y, self.z], dest)?;
        dest.write_char(' ')?;
        self.angle.to_css(dest)
    }
}

impl ToCss for TextTransform {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.case.to_css(dest)?;
        if self.full_width {
            dest.write_str(" full-width")?;
        }
        if self.full_size_kana {
            dest.write_str(" full-size-kana")?;
        }
        Ok(())
    }
}

impl ToCss for TextIndent<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.value.to_css(dest)?;
        if self.hanging {
            dest.write_str(" hanging")?;
        }
        if self.each_line {
            dest.write_str(" each-line")?;
        }
        Ok(())
    }
}

impl ToCss for TextDecoration<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.line.to_css(dest)?;
        dest.write_char(' ')?;
        self.thickness.to_css(dest)?;
        dest.write_char(' ')?;
        self.style.to_css(dest)?;
        dest.write_char(' ')?;
        self.color.to_css(dest)
    }
}

impl ToCss for TextEmphasis<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.style.to_css(dest)?;
        dest.write_char(' ')?;
        self.color.to_css(dest)
    }
}

impl ToCss for TextEmphasisPosition {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.vertical.to_css(dest)?;
        dest.write_char(' ')?;
        self.horizontal.to_css(dest)
    }
}

impl ToCss for TextShadow<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_space_separated(
            &[&*self.x_offset, &*self.y_offset, &*self.blur, &*self.spread],
            dest,
        )?;
        dest.write_char(' ')?;
        self.color.to_css(dest)
    }
}

impl ToCss for Cursor<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        for image in &self.images {
            image.to_css(dest)?;
            dest.delim(Delimiter::Comma)?;
        }
        self.keyword.to_css(dest)
    }
}

impl ToCss for CursorImage<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.url.to_css(dest)?;
        if let Some((x, y)) = self.hotspot {
            dest.write_char(' ')?;
            serialize_number(x, dest)?;
            dest.write_char(' ')?;
            serialize_number(y, dest)?;
        }
        Ok(())
    }
}

impl ToCss for Caret<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.color.to_css(dest)?;
        dest.write_char(' ')?;
        self.shape.to_css(dest)
    }
}

impl ToCss for ListStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.position.to_css(dest)?;
        dest.write_char(' ')?;
        self.image.to_css(dest)?;
        dest.write_char(' ')?;
        self.list_style_type.to_css(dest)
    }
}

impl ToCss for Composes<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        for (index, name) in self.names.iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            serialize_identifier(name, dest)?;
        }
        if let Some(from) = &self.from {
            dest.write_str(" from ")?;
            from.to_css(dest)?;
        }
        Ok(())
    }
}

impl ToCss for InsetRect<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("inset(")?;
        self.rect.to_css(dest)?;
        dest.write_str(" round ")?;
        self.radius.to_css(dest)?;
        dest.write_char(')')
    }
}

impl ToCss for CircleShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("circle(")?;
        self.radius.to_css(dest)?;
        dest.write_str(" at ")?;
        self.position.to_css(dest)?;
        dest.write_char(')')
    }
}

impl ToCss for EllipseShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("ellipse(")?;
        self.radius_x.to_css(dest)?;
        dest.write_char(' ')?;
        self.radius_y.to_css(dest)?;
        dest.write_str(" at ")?;
        self.position.to_css(dest)?;
        dest.write_char(')')
    }
}

impl ToCss for Polygon<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("polygon(")?;
        self.fill_rule.to_css(dest)?;
        dest.delim(Delimiter::Comma)?;
        write_comma_separated(&self.points, dest)?;
        dest.write_char(')')
    }
}

impl ToCss for Point<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.x.to_css(dest)?;
        dest.write_char(' ')?;
        self.y.to_css(dest)
    }
}

impl ToCss for Mask<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.image.to_css(dest)?;
        dest.write_char(' ')?;
        self.position.to_css(dest)?;
        dest.write_str(" / ")?;
        self.size.to_css(dest)?;
        dest.write_char(' ')?;
        self.repeat.to_css(dest)?;
        dest.write_char(' ')?;
        self.origin.to_css(dest)?;
        dest.write_char(' ')?;
        self.clip.to_css(dest)?;
        dest.write_char(' ')?;
        self.composite.to_css(dest)?;
        dest.write_char(' ')?;
        self.mode.to_css(dest)
    }
}

impl ToCss for MaskBorder<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.source.to_css(dest)?;
        dest.write_char(' ')?;
        self.slice.to_css(dest)?;
        dest.write_str(" / ")?;
        self.width.to_css(dest)?;
        dest.write_str(" / ")?;
        self.outset.to_css(dest)?;
        dest.write_char(' ')?;
        self.repeat.to_css(dest)?;
        dest.write_char(' ')?;
        self.mode.to_css(dest)
    }
}

impl ToCss for DropShadow<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_space_separated(&[&*self.x_offset, &*self.y_offset, &*self.blur], dest)?;
        dest.write_char(' ')?;
        self.color.to_css(dest)
    }
}

impl ToCss for Container<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.name.to_css(dest)?;
        dest.write_str(" / ")?;
        self.container_type.to_css(dest)
    }
}

impl ToCss for ColorScheme {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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

impl ToCss for UnparsedProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        crate::token::write_token_list(&self.value, dest)
    }
}

impl ToCss for CustomProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        crate::token::write_token_list(&self.value, dest)
    }
}

impl ToCss for FamilyName<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        crate::values::font::write_custom_font_family(self.0, dest)
    }
}

impl ToCss for KeyframeSelector {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::From => dest.write_str("from"),
            Self::To => dest.write_str("to"),
            Self::TimelineRangePercentage(value) => value.to_css(dest),
        }
    }
}

impl ToCss for KeyframesName<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Ident(value) => serialize_identifier(value, dest),
            Self::Custom(value) => serialize_string(value, dest),
        }
    }
}

impl ToCss for FontFaceProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Source(values) => write_comma_separated(values, dest),
            Self::FontFamily(value) => value.to_css(dest),
            Self::FontStyle(value) => value.to_css(dest),
            Self::FontWeight(value) => value.to_css(dest),
            Self::FontStretch(value) => value.to_css(dest),
            Self::UnicodeRange(values) => write_comma_separated(values, dest),
            Self::Custom(value) => value.to_css(dest),
        }
    }
}

trait NamedProperty {
    fn css_name(&self) -> &str;
}

impl NamedProperty for FontFaceProperty<'_> {
    fn css_name(&self) -> &str {
        match self {
            FontFaceProperty::Source(_) => "src",
            FontFaceProperty::FontFamily(_) => "font-family",
            FontFaceProperty::FontStyle(_) => "font-style",
            FontFaceProperty::FontWeight(_) => "font-weight",
            FontFaceProperty::FontStretch(_) => "font-stretch",
            FontFaceProperty::UnicodeRange(_) => "unicode-range",
            FontFaceProperty::Custom(value) => match &*value.name {
                CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
            },
        }
    }
}

impl ToCss for Source<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Url(value) => value.to_css(dest),
            Self::Local(value) => {
                dest.write_str("local(")?;
                value.to_css(dest)?;
                dest.write_char(')')
            }
        }
    }
}

impl ToCss for FontFormat<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let value = match self {
            Self::String(value) => value,
            value => value
                .as_css_str()
                .expect("custom font format handled separately"),
        };
        serialize_string(value, dest)
    }
}

impl ToCss for FontTechnology {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("font technologies are static keywords"),
        )
    }
}

impl ToCss for FontFaceStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Italic => dest.write_str("italic"),
            Self::Oblique(value) => {
                dest.write_str("oblique")?;
                let is_default = matches!(
                    (&*value.0, &*value.1),
                    (Angle::Deg(first), Angle::Deg(second)) if *first == 14.0 && *second == 14.0
                );
                if !is_default {
                    dest.write_char(' ')?;
                    value.to_css(dest)?;
                }
                Ok(())
            }
        }
    }
}

impl ToCss for FontPaletteValuesProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::FontFamily(value) => value.to_css(dest),
            Self::BasePalette(value) => value.to_css(dest),
            Self::OverrideColors(values) => write_comma_separated(values, dest),
            Self::Custom(value) => value.to_css(dest),
        }
    }
}

impl NamedProperty for FontPaletteValuesProperty<'_> {
    fn css_name(&self) -> &str {
        match self {
            FontPaletteValuesProperty::FontFamily(_) => "font-family",
            FontPaletteValuesProperty::BasePalette(_) => "base-palette",
            FontPaletteValuesProperty::OverrideColors(_) => "override-colors",
            FontPaletteValuesProperty::Custom(value) => match &*value.name {
                CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
            },
        }
    }
}

impl ToCss for BasePalette {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Light => dest.write_str("light"),
            Self::Dark => dest.write_str("dark"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

impl ToCss for FontFeatureSubruleType {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("font feature subrule types are static keywords"),
        )
    }
}

impl ToCss for PageMarginBox {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("page margin boxes are static keywords"),
        )
    }
}

impl ToCss for PagePseudoClass {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("page pseudo classes are static keywords"),
        )
    }
}

impl ToCss for ParsedComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::LengthPercentage(value) => value.to_css(dest),
            Self::String(value) => serialize_string(value, dest),
            Self::Color(value) => value.to_css(dest),
            Self::Image(value) => value.to_css(dest),
            Self::Url(value) => value.to_css(dest),
            Self::Integer(value) => serialize_int(*value, dest),
            Self::Angle(value) => value.to_css(dest),
            Self::Time(value) => value.to_css(dest),
            Self::Resolution(value) => value.to_css(dest),
            Self::TransformFunction(value) => value.to_css(dest),
            Self::TransformList(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest)?;
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
                for (index, value) in components.iter().enumerate() {
                    if index > 0 {
                        dest.write_str(delimiter)?;
                    }
                    value.to_css(dest)?;
                }
                Ok(())
            }
            Self::TokenList(values) => crate::token::write_token_list(values, dest),
        }
    }
}

impl ToCss for Multiplier {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(match self {
            Self::None => "",
            Self::Space => "+",
            Self::Comma => "#",
        })
    }
}

impl ToCss for SyntaxString<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Universal => dest.write_char('*'),
            Self::Components(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_str(" | ")?;
                    }
                    value.to_css(dest)?;
                }
                Ok(())
            }
        }
    }
}

impl ToCss for SyntaxComponentKind<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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

impl ToCss for SyntaxComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.kind.to_css(dest)?;
        self.multiplier.to_css(dest)
    }
}

impl ToCss for ContainerSizeFeatureId {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("container size features are static keywords"),
        )
    }
}

impl ToCss for ScrollStateFeatureId {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("scroll state features are static keywords"),
        )
    }
}

impl ToCss for ContainerCondition<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Feature(value) => value.to_css(dest),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in conditions.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest)?;
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest)?;
                }
                Ok(())
            }
            Self::Style(value) => {
                dest.write_str("style(")?;
                value.to_css(dest)?;
                dest.write_char(')')
            }
            Self::ScrollState(value) => {
                dest.write_str("scroll-state(")?;
                value.to_css(dest)?;
                dest.write_char(')')
            }
            Self::Unknown(values) => crate::token::write_token_list(values, dest),
        }
    }
}

impl ToCss for StyleQuery<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Declaration(value) => value.to_css(dest),
            Self::Property(value) => value.to_css(dest),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in conditions.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest)?;
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest)?;
                }
                Ok(())
            }
        }
    }
}

impl ToCss for ScrollStateQuery<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Feature(value) => value.to_css(dest),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in conditions.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest)?;
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest)?;
                }
                Ok(())
            }
        }
    }
}

impl ToCss for ViewTransitionProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Navigation(value) => value.to_css(dest),
            Self::Types(value) => value.to_css(dest),
            Self::Custom(value) => value.to_css(dest),
        }
    }
}

impl NamedProperty for ViewTransitionProperty<'_> {
    fn css_name(&self) -> &str {
        match self {
            ViewTransitionProperty::Navigation(_) => "navigation",
            ViewTransitionProperty::Types(_) => "types",
            ViewTransitionProperty::Custom(value) => match &*value.name {
                CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
            },
        }
    }
}

impl ToCss for Navigation {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("navigation values are static keywords"),
        )
    }
}

impl ToCss for DefaultAtRule {
    fn to_css<PrinterT: PrinterTrait>(&self, _dest: &mut PrinterT) -> fmt::Result {
        Ok(())
    }
}

pub(crate) fn write_rule_list<'ghost, PrinterT: PrinterTrait>(
    rules: &[CssRule<'_, 'ghost>],
    token: &GhostToken<'ghost>,
    dest: &mut PrinterT,
) -> fmt::Result {
    let mut first = true;
    let mut last_without_block = false;
    for rule in rules {
        if matches!(rule, CssRule::Style(style)
            if style
                .get(token)
                .selectors
                .iter()
                .all(Selector::is_tombstone))
        {
            continue;
        }
        if !first {
            if !last_without_block
                || !matches!(
                    rule,
                    CssRule::Charset(_)
                        | CssRule::Import(_)
                        | CssRule::Namespace(_)
                        | CssRule::LayerStatement(_)
                )
            {
                dest.blank_line()?;
            } else {
                dest.new_line()?;
            }
        }
        first = false;
        rule.to_css_with_ghost(token, dest)?;
        last_without_block = matches!(
            rule,
            CssRule::Charset(_)
                | CssRule::Import(_)
                | CssRule::Namespace(_)
                | CssRule::LayerStatement(_)
        );
    }
    Ok(())
}

fn write_block<PrinterT: PrinterTrait, F>(dest: &mut PrinterT, callback: F) -> fmt::Result
where
    F: FnOnce(&mut PrinterT) -> fmt::Result,
{
    dest.whitespace()?;
    dest.write_char('{')?;
    dest.indent();
    dest.new_line()?;
    callback(dest)?;
    dest.dedent();
    dest.new_line()?;
    dest.write_char('}')
}

#[derive(Clone, Copy)]
enum LastSemicolon {
    Omit,
    Optional,
    Required,
}

fn write_declarations<PrinterT: PrinterTrait>(
    declarations: &DeclarationBlock<'_>,
    dest: &mut PrinterT,
    last_semicolon: LastSemicolon,
) -> fmt::Result {
    let mut declarations = declarations.iter_live().peekable();
    while let Some((declaration, important)) = declarations.next() {
        declaration.to_css(dest)?;
        if important {
            dest.write_str(" !important")?;
        }
        let has_next = declarations.peek().is_some();
        if has_next {
            dest.write_char(';')?;
        } else {
            match last_semicolon {
                LastSemicolon::Omit => {}
                LastSemicolon::Optional => dest.semicolon(false)?,
                LastSemicolon::Required => dest.write_char(';')?,
            }
        }
        if has_next {
            dest.new_line()?;
        }
    }
    Ok(())
}

fn style_rule_chain_is_output_empty<'ghost>(
    tail: &StyleRule<'_, 'ghost>,
    token: &GhostToken<'ghost>,
) -> bool {
    let mut current = tail;
    loop {
        if !current.declarations.borrow(token).is_output_empty() {
            return false;
        }
        let Some(previous) = current.previous_merged() else {
            return true;
        };
        current = previous.get(token).get_ref();
    }
}

fn write_style_rule_declaration_chain<'ghost, PrinterT: PrinterTrait>(
    tail: &StyleRule<'_, 'ghost>,
    token: &GhostToken<'ghost>,
    dest: &mut PrinterT,
    last_semicolon: LastSemicolon,
) -> fmt::Result {
    write_style_rule_declaration_chain_recursive(tail, token, dest, last_semicolon).map(|_| ())
}

fn write_style_rule_declaration_chain_recursive<'ghost, PrinterT: PrinterTrait>(
    current: &StyleRule<'_, 'ghost>,
    token: &GhostToken<'ghost>,
    dest: &mut PrinterT,
    last_semicolon: LastSemicolon,
) -> Result<bool, fmt::Error> {
    let current_is_empty = current.declarations.borrow(token).is_output_empty();
    let wrote_previous = if let Some(previous) = current.previous_merged() {
        write_style_rule_declaration_chain_recursive(
            previous.get(token).get_ref(),
            token,
            dest,
            if current_is_empty {
                last_semicolon
            } else {
                LastSemicolon::Required
            },
        )?
    } else {
        false
    };

    if current_is_empty {
        return Ok(wrote_previous);
    }
    if wrote_previous {
        dest.new_line()?;
    }
    write_declarations(current.declarations.borrow(token), dest, last_semicolon)?;
    Ok(true)
}

impl ToCss for DeclarationBlock<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_declarations(self, dest, LastSemicolon::Omit)
    }
}

fn write_declaration_block<PrinterT: PrinterTrait>(
    declarations: &DeclarationBlock<'_>,
    dest: &mut PrinterT,
) -> fmt::Result {
    write_block(dest, |dest| {
        write_declarations(declarations, dest, LastSemicolon::Optional)
    })
}

impl<'ghost> ToCssWithGhost<'ghost> for StyleSheet<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        for (index, comment) in self.license_comments.iter().enumerate() {
            dest.write_str("/*")?;
            dest.write_str(comment)?;
            dest.write_str("*/")?;
            if index + 1 < self.license_comments.len() || !self.rules.is_empty() {
                dest.new_line()?;
            }
        }
        write_rule_list(&self.rules, token, dest)?;
        if !self.rules.is_empty() {
            dest.new_line()?;
        }
        Ok(())
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for MediaRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@media ")?;
        self.query.to_css(dest)?;
        write_block(dest, |dest| write_rule_list(&self.rules, token, dest))
    }
}

impl ToCss for ImportRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@import ")?;
        serialize_string(self.url, dest)?;
        if let Some(layer) = &self.layer {
            dest.write_str(" layer")?;
            if !layer.is_empty() {
                dest.write_char('(')?;
                write_layer_name(layer, dest)?;
                dest.write_char(')')?;
            }
        }
        if let Some(supports) = &self.supports {
            dest.write_str(" supports(")?;
            let serialized = supports.to_css_string(dest.options())?;
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
            media.to_css(dest)?;
        }
        dest.write_char(';')
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for StyleRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        if self.selectors.iter().all(Selector::is_tombstone) {
            return Ok(());
        }
        self.selectors.to_css(dest)?;
        write_block(dest, |dest| {
            write_style_rule_declaration_chain(
                self,
                token,
                dest,
                if self.rules.is_empty() {
                    LastSemicolon::Optional
                } else {
                    LastSemicolon::Required
                },
            )?;
            if !style_rule_chain_is_output_empty(self, token) && !self.rules.is_empty() {
                dest.blank_line()?;
            }
            write_rule_list(&self.rules, token, dest)
        })
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for KeyframesRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_char('@')?;
        self.vendor_prefix.to_css(dest)?;
        dest.write_str("keyframes ")?;
        self.name.to_css(dest)?;
        if self.keyframes.is_empty() {
            dest.whitespace()?;
            dest.write_char('{')?;
            dest.new_line()?;
            return dest.write_char('}');
        }
        write_block(dest, |dest| {
            for (index, keyframe) in self.keyframes.iter().enumerate() {
                if index > 0 {
                    dest.blank_line()?;
                }
                keyframe.to_css_with_ghost(token, dest)?;
            }
            Ok(())
        })
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for Keyframe<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        write_comma_separated(&self.selectors, dest)?;
        write_declaration_block(self.declarations.borrow(token), dest)
    }
}

impl ToCss for TimelineRangePercentage {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.name.to_css(dest)?;
        dest.write_char(' ')?;
        serialize_number(self.percentage * 100.0, dest)?;
        dest.write_char('%')
    }
}

fn write_named_property_block<PrinterT: PrinterTrait, T>(
    values: &[T],
    dest: &mut PrinterT,
) -> fmt::Result
where
    T: ToCss + NamedProperty,
{
    write_block(dest, |dest| {
        for (index, value) in values.iter().enumerate() {
            serialize_name(value.css_name(), dest)?;
            dest.write_char(':')?;
            dest.whitespace()?;
            value.to_css(dest)?;
            dest.semicolon(index + 1 < values.len())?;
            if index + 1 < values.len() {
                dest.new_line()?;
            }
        }
        Ok(())
    })
}

impl ToCss for FontFaceRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@font-face")?;
        write_named_property_block(&self.properties, dest)
    }
}

impl ToCss for UrlSource<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.url.to_css(dest)?;
        if let Some(format) = &self.format {
            dest.write_str(" format(")?;
            format.to_css(dest)?;
            dest.write_char(')')?;
        }
        if !self.tech.is_empty() {
            dest.write_str(" tech(")?;
            write_comma_separated(&self.tech, dest)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl ToCss for UnicodeRange {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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

impl ToCss for FontPaletteValuesRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@font-palette-values ")?;
        serialize_identifier(self.name, dest)?;
        write_named_property_block(&self.properties, dest)
    }
}

impl ToCss for OverrideColors<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_int(self.index, dest)?;
        dest.write_char(' ')?;
        self.color.to_css(dest)
    }
}

impl ToCss for FontFeatureValuesRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@font-feature-values ")?;
        write_comma_separated(&self.name, dest)?;
        write_block(dest, |dest| {
            for (index, rule) in self.rules.iter().enumerate() {
                if index > 0 {
                    dest.blank_line()?;
                }
                rule.to_css(dest)?;
            }
            Ok(())
        })
    }
}

impl ToCss for FontFeatureSubrule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_char('@')?;
        self.name.to_css(dest)?;
        write_block(dest, |dest| {
            for (index, value) in self.declarations.iter().enumerate() {
                value.to_css(dest)?;
                if index + 1 < self.declarations.len() {
                    dest.write_char(';')?;
                    dest.new_line()?;
                }
            }
            Ok(())
        })
    }
}

impl ToCss for FontFeatureDeclaration<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_identifier(self.name, dest)?;
        dest.delim(Delimiter::Colon)?;
        for (index, value) in self.values.iter().enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            serialize_int(*value, dest)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for PageRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@page")?;
        if !self.selectors.is_empty() {
            dest.write_char(' ')?;
            write_comma_separated(&self.selectors, dest)?;
        }
        write_block(dest, |dest| {
            write_declarations(
                self.declarations.borrow(token),
                dest,
                if self.rules.is_empty() {
                    LastSemicolon::Optional
                } else {
                    LastSemicolon::Required
                },
            )?;
            if !self.declarations.borrow(token).is_output_empty() && !self.rules.is_empty() {
                dest.blank_line()?;
            }
            for (index, rule) in self.rules.iter().enumerate() {
                if index > 0 {
                    dest.blank_line()?;
                }
                rule.to_css_with_ghost(token, dest)?;
            }
            Ok(())
        })
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for PageMarginRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_char('@')?;
        self.margin_box.to_css(dest)?;
        write_declaration_block(self.declarations.borrow(token), dest)
    }
}

impl ToCss for PageSelector<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if let Some(name) = self.name {
            serialize_identifier(name, dest)?;
        }
        for pseudo_class in &self.pseudo_classes {
            dest.write_char(':')?;
            pseudo_class.to_css(dest)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for SupportsRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@supports ")?;
        self.condition.to_css(dest)?;
        write_block(dest, |dest| write_rule_list(&self.rules, token, dest))
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for CounterStyleRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@counter-style ")?;
        serialize_identifier(self.name, dest)?;
        write_declaration_block(self.declarations.borrow(token), dest)
    }
}

impl ToCss for CharsetRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@charset ")?;
        serialize_string(self.encoding, dest)?;
        dest.write_char(';')
    }
}

impl ToCss for NamespaceRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@namespace ")?;
        if let Some(prefix) = self.prefix {
            serialize_identifier(prefix, dest)?;
            dest.write_char(' ')?;
        }
        serialize_string(self.url, dest)?;
        dest.write_char(';')
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for MozDocumentRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@-moz-document url-prefix()")?;
        write_block(dest, |dest| write_rule_list(&self.rules, token, dest))
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for NestingRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@nest ")?;
        self.style
            .get(token)
            .get_ref()
            .to_css_with_ghost(token, dest)
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for NestedDeclarationsRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        write_declarations(
            self.declarations.borrow(token),
            dest,
            LastSemicolon::Optional,
        )
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for ViewportRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_char('@')?;
        self.vendor_prefix.to_css(dest)?;
        dest.write_str("viewport")?;
        write_declaration_block(self.declarations.borrow(token), dest)
    }
}

impl ToCss for CustomMediaRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@custom-media ")?;
        dest.write_str("--")?;
        serialize_name(self.name.strip_prefix("--").unwrap_or(self.name), dest)?;
        dest.write_char(' ')?;
        self.query.to_css(dest)?;
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

impl ToCss for LayerStatementRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@layer ")?;
        for (index, name) in self.names.iter().enumerate() {
            if index > 0 {
                dest.delim(Delimiter::Comma)?;
            }
            write_layer_name(name, dest)?;
        }
        dest.write_char(';')
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for LayerBlockRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@layer")?;
        if let Some(name) = &self.name {
            dest.write_char(' ')?;
            write_layer_name(name, dest)?;
        }
        write_block(dest, |dest| write_rule_list(&self.rules, token, dest))
    }
}

impl ToCss for PropertyRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@property ")?;
        dest.write_str("--")?;
        serialize_name(self.name.strip_prefix("--").unwrap_or(self.name), dest)?;
        write_block(dest, |dest| {
            dest.write_str("syntax")?;
            dest.delim(Delimiter::Colon)?;
            let syntax = self.syntax.to_css_string(dest.options())?;
            serialize_string(&syntax, dest)?;
            dest.write_char(';')?;
            dest.new_line()?;
            dest.write_str("inherits")?;
            dest.delim(Delimiter::Colon)?;
            dest.write_str(if self.inherits { "true" } else { "false" })?;
            if let Some(initial_value) = &self.initial_value {
                dest.write_char(';')?;
                dest.new_line()?;
                dest.write_str("initial-value")?;
                dest.delim(Delimiter::Colon)?;
                initial_value.to_css(dest)?;
            }
            dest.semicolon(false)
        })
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for ContainerRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@container")?;
        if let Some(name) = self.name {
            dest.write_char(' ')?;
            serialize_identifier(name, dest)?;
        }
        if let Some(condition) = &self.condition {
            dest.write_char(' ')?;
            condition.to_css(dest)?;
        }
        write_block(dest, |dest| write_rule_list(&self.rules, token, dest))
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for ScopeRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@scope")?;
        if let Some(start) = &self.scope_start {
            dest.write_str(" (")?;
            start.to_css(dest)?;
            dest.write_char(')')?;
        }
        if let Some(end) = &self.scope_end {
            dest.write_str(" to (")?;
            end.to_css(dest)?;
            dest.write_char(')')?;
        }
        write_block(dest, |dest| write_rule_list(&self.rules, token, dest))
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for StartingStyleRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@starting-style")?;
        write_block(dest, |dest| write_rule_list(&self.rules, token, dest))
    }
}

impl ToCss for ViewTransitionRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("@view-transition")?;
        write_named_property_block(&self.properties, dest)
    }
}

impl<'ghost> ToCssWithGhost<'ghost> for PositionTryRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        dest.write_str("@position-try ")?;
        dest.write_str("--")?;
        serialize_name(self.name.strip_prefix("--").unwrap_or(self.name), dest)?;
        write_declaration_block(self.declarations.borrow(token), dest)
    }
}

impl ToCss for UnknownAtRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_char('@')?;
        serialize_identifier(self.name, dest)?;
        if !self.prelude.is_empty() {
            dest.write_char(' ')?;
            crate::token::write_token_list(&self.prelude, dest)?;
        }
        if let Some(block) = &self.block {
            write_block(dest, |dest| crate::token::write_token_list(block, dest))
        } else {
            dest.write_char(';')
        }
    }
}
