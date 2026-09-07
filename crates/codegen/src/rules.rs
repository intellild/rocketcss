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

fn write_shadow<'id, 'ghost, PrinterT: PrinterTrait>(
    offsets: [NodeId<'id, Length<'id>>; 4],
    color: NodeId<'id, CssColor<'id>>,
    inset: bool,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if inset {
        dest.write_str("inset ")?;
    }
    for offset in offsets {
        offset.to_css(dest, cx)?;
        dest.write_char(' ')?;
    }
    color.to_css(dest, cx)
}

fn write_comma_separated<'ghost, PrinterT, I>(
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
    T: ToCss<'ghost> + PartialEq + AstNodeStorage<'ast>,
{
    let first = cx.ast_context().resolve_node(*first);
    first.to_css(dest, cx)?;
    let second = cx.ast_context().resolve_node(*second);
    if !css_values_are_equal(&first, &second, cx) {
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
    T: ToCss<'ghost> + PartialEq + AstNodeStorage<'ast>,
{
    let ast = cx.ast_context();
    let top_value = ast.resolve_node(*top);
    let right_value = ast.resolve_node(*right);
    let bottom_value = ast.resolve_node(*bottom);
    let left_value = ast.resolve_node(*left);
    top_value.to_css(dest, cx)?;
    if css_values_are_equal(&top_value, &right_value, cx)
        && css_values_are_equal(&top_value, &bottom_value, cx)
        && css_values_are_equal(&top_value, &left_value, cx)
    {
        return Ok(());
    }
    dest.write_char(' ')?;
    right_value.to_css(dest, cx)?;
    if css_values_are_equal(&top_value, &bottom_value, cx)
        && css_values_are_equal(&right_value, &left_value, cx)
    {
        return Ok(());
    }
    dest.write_char(' ')?;
    bottom_value.to_css(dest, cx)?;
    if !css_values_are_equal(&right_value, &left_value, cx) {
        dest.write_char(' ')?;
        left_value.to_css(dest, cx)?;
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

pub(crate) trait NamedProperty {
    fn css_name<'a>(&'a self, ast: &'a AstContext<'_>) -> &'a str;
}

pub(crate) mod animation;
pub(crate) mod at_rule;
pub(crate) mod background;
pub(crate) mod border;
pub(crate) mod container;
pub(crate) mod font;
pub(crate) mod keyframes;
pub(crate) mod layout;
pub(crate) mod page;
pub(crate) mod property;
pub(crate) mod shape;
pub(crate) mod stylesheet;
pub(crate) mod text;
pub(crate) mod transform;
pub(crate) mod ui;
pub(crate) mod view_transition;
