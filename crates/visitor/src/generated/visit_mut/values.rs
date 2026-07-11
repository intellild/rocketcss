#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitMutNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_image<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Image<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Image);
    match node {
        Image::None => {}
        Image::Url(field_0) => {
            visitor.visit_url((field_0).as_mut());
        }
        Image::Gradient(field_0) => {
            visitor.visit_gradient((field_0).as_mut());
        }
        Image::ImageSet(field_0) => {
            visitor.visit_image_set((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Image);
}
pub fn walk_gradient<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Gradient<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Gradient);
    match node {
        Gradient::Linear {
            direction,
            items,
            vendor_prefix,
        } => {
            visitor.visit_line_direction((direction).as_mut());
            for value_1 in (items).iter_mut() {
                visitor.visit_gradient_item(value_1);
            }
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        Gradient::RepeatingLinear {
            direction,
            items,
            vendor_prefix,
        } => {
            visitor.visit_line_direction((direction).as_mut());
            for value_3 in (items).iter_mut() {
                visitor.visit_gradient_item(value_3);
            }
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        Gradient::Radial {
            items,
            position,
            shape,
            vendor_prefix,
        } => {
            for value_4 in (items).iter_mut() {
                visitor.visit_gradient_item(value_4);
            }
            visitor.visit_position((position).as_mut());
            visitor.visit_ending_shape((shape).as_mut());
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        Gradient::RepeatingRadial {
            items,
            position,
            shape,
            vendor_prefix,
        } => {
            for value_7 in (items).iter_mut() {
                visitor.visit_gradient_item(value_7);
            }
            visitor.visit_position((position).as_mut());
            visitor.visit_ending_shape((shape).as_mut());
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        Gradient::Conic {
            angle,
            items,
            position,
        } => {
            visitor.visit_angle((angle).as_mut());
            for value_11 in (items).iter_mut() {
                visitor.visit_gradient_item(value_11);
            }
            visitor.visit_position((position).as_mut());
        }
        Gradient::RepeatingConic {
            angle,
            items,
            position,
        } => {
            visitor.visit_angle((angle).as_mut());
            for value_14 in (items).iter_mut() {
                visitor.visit_gradient_item(value_14);
            }
            visitor.visit_position((position).as_mut());
        }
        Gradient::WebKitGradient(field_0) => {
            visitor.visit_web_kit_gradient((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Gradient);
}
pub fn walk_web_kit_gradient<'a, VisitorT>(visitor: &mut VisitorT, node: &mut WebKitGradient<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::WebKitGradient);
    match node {
        WebKitGradient::Linear { from, to, stops } => {
            visitor.visit_web_kit_gradient_point((from).as_mut());
            visitor.visit_web_kit_gradient_point((to).as_mut());
            for value_2 in (stops).iter_mut() {
                visitor.visit_web_kit_color_stop(value_2);
            }
        }
        WebKitGradient::Radial {
            from,
            start_radius,
            to,
            end_radius,
            stops,
        } => {
            visitor.visit_web_kit_gradient_point((from).as_mut());
            visitor.visit_web_kit_gradient_point((to).as_mut());
            for value_5 in (stops).iter_mut() {
                visitor.visit_web_kit_color_stop(value_5);
            }
        }
    }
    visitor.leave_node(AstType::WebKitGradient);
}
pub fn walk_line_direction<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LineDirection<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LineDirection);
    match node {
        LineDirection::Angle(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        LineDirection::Horizontal(field_0) => {
            visitor.visit_horizontal_position_keyword(field_0);
        }
        LineDirection::Vertical(field_0) => {
            visitor.visit_vertical_position_keyword(field_0);
        }
        LineDirection::Corner {
            horizontal,
            vertical,
        } => {
            visitor.visit_horizontal_position_keyword(horizontal);
            visitor.visit_vertical_position_keyword(vertical);
        }
    }
    visitor.leave_node(AstType::LineDirection);
}
pub fn walk_horizontal_position_keyword<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut HorizontalPositionKeyword,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::HorizontalPositionKeyword);
    match node {
        HorizontalPositionKeyword::Left => {}
        HorizontalPositionKeyword::Right => {}
    }
    visitor.leave_node(AstType::HorizontalPositionKeyword);
}
pub fn walk_vertical_position_keyword<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut VerticalPositionKeyword,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::VerticalPositionKeyword);
    match node {
        VerticalPositionKeyword::Top => {}
        VerticalPositionKeyword::Bottom => {}
    }
    visitor.leave_node(AstType::VerticalPositionKeyword);
}
pub fn walk_gradient_item<'a, D, VisitorT>(visitor: &mut VisitorT, node: &mut GradientItem<'a, D>)
where
    VisitorT: ?Sized + VisitMut<'a>,
    D: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::GradientItem);
    match node {
        GradientItem::ColorStop { color, position } => {
            visitor.visit_css_color((color).as_mut());
            if let Some(value_1) = (position).as_mut() {
                visitor.visit_dimension_percentage((value_1).as_mut());
            }
        }
        GradientItem::Hint(field_0) => {
            visitor.visit_dimension_percentage((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::GradientItem);
}
pub fn walk_dimension_percentage<'a, D, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut DimensionPercentage<'a, D>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
    D: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::DimensionPercentage);
    match node {
        DimensionPercentage::Dimension(field_0) => {
            VisitMutNode::visit_node((field_0).as_mut(), visitor);
        }
        DimensionPercentage::Percentage(field_0) => {}
        DimensionPercentage::Calc(field_0) => {
            visitor.visit_calc((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::DimensionPercentage);
}
pub fn walk_position_component<'a, S, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut PositionComponent<'a, S>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
    S: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::PositionComponent);
    match node {
        PositionComponent::Center => {}
        PositionComponent::Length(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        PositionComponent::Side { offset, side } => {
            if let Some(value_1) = (offset).as_mut() {
                visitor.visit_length_percentage((value_1).as_mut());
            }
            VisitMutNode::visit_node(side, visitor);
        }
    }
    visitor.leave_node(AstType::PositionComponent);
}
pub fn walk_ending_shape<'a, VisitorT>(visitor: &mut VisitorT, node: &mut EndingShape<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::EndingShape);
    match node {
        EndingShape::Ellipse(field_0) => {
            visitor.visit_ellipse((field_0).as_mut());
        }
        EndingShape::Circle(field_0) => {
            visitor.visit_circle((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::EndingShape);
}
pub fn walk_ellipse<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Ellipse<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Ellipse);
    match node {
        Ellipse::Size { x, y } => {
            visitor.visit_length_percentage((x).as_mut());
            visitor.visit_length_percentage((y).as_mut());
        }
        Ellipse::Extent(field_0) => {
            visitor.visit_shape_extent(field_0);
        }
    }
    visitor.leave_node(AstType::Ellipse);
}
pub fn walk_shape_extent<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ShapeExtent)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ShapeExtent);
    match node {
        ShapeExtent::ClosestSide => {}
        ShapeExtent::FarthestSide => {}
        ShapeExtent::ClosestCorner => {}
        ShapeExtent::FarthestCorner => {}
    }
    visitor.leave_node(AstType::ShapeExtent);
}
pub fn walk_circle<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Circle<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Circle);
    match node {
        Circle::Radius(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
        Circle::Extent(field_0) => {
            visitor.visit_shape_extent(field_0);
        }
    }
    visitor.leave_node(AstType::Circle);
}
pub fn walk_web_kit_gradient_point_component<'a, S, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut WebKitGradientPointComponent<'a, S>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
    S: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::WebKitGradientPointComponent);
    match node {
        WebKitGradientPointComponent::Center => {}
        WebKitGradientPointComponent::Number(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        WebKitGradientPointComponent::Side(field_0) => {
            VisitMutNode::visit_node(field_0, visitor);
        }
    }
    visitor.leave_node(AstType::WebKitGradientPointComponent);
}
pub fn walk_number_or_percentage<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut NumberOrPercentage,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::NumberOrPercentage);
    match node {
        NumberOrPercentage::Number(field_0) => {}
        NumberOrPercentage::Percentage(field_0) => {}
    }
    visitor.leave_node(AstType::NumberOrPercentage);
}
pub fn walk_background_size<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BackgroundSize<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BackgroundSize);
    match node {
        BackgroundSize::Explicit { height, width } => {
            visitor.visit_length_percentage_or_auto((height).as_mut());
            visitor.visit_length_percentage_or_auto((width).as_mut());
        }
        BackgroundSize::Cover => {}
        BackgroundSize::Contain => {}
    }
    visitor.leave_node(AstType::BackgroundSize);
}
pub fn walk_length_percentage_or_auto<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut LengthPercentageOrAuto<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LengthPercentageOrAuto);
    match node {
        LengthPercentageOrAuto::Auto => {}
        LengthPercentageOrAuto::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::LengthPercentageOrAuto);
}
pub fn walk_background_repeat_keyword<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BackgroundRepeatKeyword,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BackgroundRepeatKeyword);
    match node {
        BackgroundRepeatKeyword::Repeat => {}
        BackgroundRepeatKeyword::Space => {}
        BackgroundRepeatKeyword::Round => {}
        BackgroundRepeatKeyword::NoRepeat => {}
    }
    visitor.leave_node(AstType::BackgroundRepeatKeyword);
}
pub fn walk_background_attachment<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BackgroundAttachment,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BackgroundAttachment);
    match node {
        BackgroundAttachment::Scroll => {}
        BackgroundAttachment::Fixed => {}
        BackgroundAttachment::Local => {}
    }
    visitor.leave_node(AstType::BackgroundAttachment);
}
pub fn walk_background_clip<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BackgroundClip)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BackgroundClip);
    match node {
        BackgroundClip::BorderBox => {}
        BackgroundClip::PaddingBox => {}
        BackgroundClip::ContentBox => {}
        BackgroundClip::Border => {}
        BackgroundClip::Text => {}
    }
    visitor.leave_node(AstType::BackgroundClip);
}
pub fn walk_background_origin<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BackgroundOrigin)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BackgroundOrigin);
    match node {
        BackgroundOrigin::BorderBox => {}
        BackgroundOrigin::PaddingBox => {}
        BackgroundOrigin::ContentBox => {}
    }
    visitor.leave_node(AstType::BackgroundOrigin);
}
pub fn walk_display<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Display<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Display);
    match node {
        Display::Keyword(field_0) => {
            visitor.visit_display_keyword(field_0);
        }
        Display::Pair {
            inside,
            is_list_item,
            outside,
        } => {
            visitor.visit_display_inside((inside).as_mut());
            visitor.visit_display_outside(outside);
        }
    }
    visitor.leave_node(AstType::Display);
}
pub fn walk_display_keyword<'a, VisitorT>(visitor: &mut VisitorT, node: &mut DisplayKeyword)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::DisplayKeyword);
    match node {
        DisplayKeyword::None => {}
        DisplayKeyword::Contents => {}
        DisplayKeyword::TableRowGroup => {}
        DisplayKeyword::TableHeaderGroup => {}
        DisplayKeyword::TableFooterGroup => {}
        DisplayKeyword::TableRow => {}
        DisplayKeyword::TableCell => {}
        DisplayKeyword::TableColumnGroup => {}
        DisplayKeyword::TableColumn => {}
        DisplayKeyword::TableCaption => {}
        DisplayKeyword::RubyBase => {}
        DisplayKeyword::RubyText => {}
        DisplayKeyword::RubyBaseContainer => {}
        DisplayKeyword::RubyTextContainer => {}
    }
    visitor.leave_node(AstType::DisplayKeyword);
}
pub fn walk_display_inside<'a, VisitorT>(visitor: &mut VisitorT, node: &mut DisplayInside)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::DisplayInside);
    match node {
        DisplayInside::Flow => {}
        DisplayInside::FlowRoot => {}
        DisplayInside::Table => {}
        DisplayInside::Flex { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        DisplayInside::Box { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        DisplayInside::Grid => {}
        DisplayInside::Ruby => {}
    }
    visitor.leave_node(AstType::DisplayInside);
}
pub fn walk_display_outside<'a, VisitorT>(visitor: &mut VisitorT, node: &mut DisplayOutside)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::DisplayOutside);
    match node {
        DisplayOutside::Block => {}
        DisplayOutside::Inline => {}
        DisplayOutside::RunIn => {}
    }
    visitor.leave_node(AstType::DisplayOutside);
}
pub fn walk_visibility<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Visibility)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Visibility);
    match node {
        Visibility::Visible => {}
        Visibility::Hidden => {}
        Visibility::Collapse => {}
    }
    visitor.leave_node(AstType::Visibility);
}
pub fn walk_size<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Size<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Size);
    match node {
        Size::Auto => {}
        Size::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        Size::MinContent { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        Size::MaxContent { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        Size::FitContent { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        Size::FitContentFunction(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        Size::Stretch { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        Size::Contain => {}
    }
    visitor.leave_node(AstType::Size);
}
pub fn walk_max_size<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MaxSize<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MaxSize);
    match node {
        MaxSize::None => {}
        MaxSize::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        MaxSize::MinContent { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        MaxSize::MaxContent { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        MaxSize::FitContent { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        MaxSize::FitContentFunction(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        MaxSize::Stretch { vendor_prefix } => {
            visitor.visit_vendor_prefix(vendor_prefix);
        }
        MaxSize::Contain => {}
    }
    visitor.leave_node(AstType::MaxSize);
}
pub fn walk_box_sizing<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BoxSizing)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BoxSizing);
    match node {
        BoxSizing::ContentBox => {}
        BoxSizing::BorderBox => {}
    }
    visitor.leave_node(AstType::BoxSizing);
}
pub fn walk_overflow_keyword<'a, VisitorT>(visitor: &mut VisitorT, node: &mut OverflowKeyword)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::OverflowKeyword);
    match node {
        OverflowKeyword::Visible => {}
        OverflowKeyword::Hidden => {}
        OverflowKeyword::Clip => {}
        OverflowKeyword::Scroll => {}
        OverflowKeyword::Auto => {}
    }
    visitor.leave_node(AstType::OverflowKeyword);
}
pub fn walk_text_overflow<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextOverflow)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextOverflow);
    match node {
        TextOverflow::Clip => {}
        TextOverflow::Ellipsis => {}
    }
    visitor.leave_node(AstType::TextOverflow);
}
pub fn walk_position_property<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PositionProperty)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PositionProperty);
    match node {
        PositionProperty::Static => {}
        PositionProperty::Relative => {}
        PositionProperty::Absolute => {}
        PositionProperty::Sticky(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PositionProperty::Fixed => {}
    }
    visitor.leave_node(AstType::PositionProperty);
}
pub fn walk_size_2_d<'a, T, VisitorT>(visitor: &mut VisitorT, node: &mut Size2D<'a, T>)
where
    VisitorT: ?Sized + VisitMut<'a>,
    T: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::Size2D);
    VisitMutNode::visit_node((&mut node.0).as_mut(), visitor);
    VisitMutNode::visit_node((&mut node.1).as_mut(), visitor);
    visitor.leave_node(AstType::Size2D);
}
pub fn walk_rect<'a, T, VisitorT>(visitor: &mut VisitorT, node: &mut Rect<'a, T>)
where
    VisitorT: ?Sized + VisitMut<'a>,
    T: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::Rect);
    VisitMutNode::visit_node((&mut node.0).as_mut(), visitor);
    VisitMutNode::visit_node((&mut node.1).as_mut(), visitor);
    VisitMutNode::visit_node((&mut node.2).as_mut(), visitor);
    VisitMutNode::visit_node((&mut node.3).as_mut(), visitor);
    visitor.leave_node(AstType::Rect);
}
pub fn walk_line_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LineStyle)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LineStyle);
    match node {
        LineStyle::None => {}
        LineStyle::Hidden => {}
        LineStyle::Inset => {}
        LineStyle::Groove => {}
        LineStyle::Outset => {}
        LineStyle::Ridge => {}
        LineStyle::Dotted => {}
        LineStyle::Dashed => {}
        LineStyle::Solid => {}
        LineStyle::Double => {}
    }
    visitor.leave_node(AstType::LineStyle);
}
pub fn walk_border_side_width<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderSideWidth<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderSideWidth);
    match node {
        BorderSideWidth::Thin => {}
        BorderSideWidth::Medium => {}
        BorderSideWidth::Thick => {}
        BorderSideWidth::Length(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::BorderSideWidth);
}
pub fn walk_length_or_number<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LengthOrNumber<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LengthOrNumber);
    match node {
        LengthOrNumber::Number(field_0) => {}
        LengthOrNumber::Length(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::LengthOrNumber);
}
pub fn walk_border_image_repeat_keyword<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BorderImageRepeatKeyword,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderImageRepeatKeyword);
    match node {
        BorderImageRepeatKeyword::Stretch => {}
        BorderImageRepeatKeyword::Repeat => {}
        BorderImageRepeatKeyword::Round => {}
        BorderImageRepeatKeyword::Space => {}
    }
    visitor.leave_node(AstType::BorderImageRepeatKeyword);
}
pub fn walk_border_image_side_width<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BorderImageSideWidth<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderImageSideWidth);
    match node {
        BorderImageSideWidth::Number(field_0) => {}
        BorderImageSideWidth::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        BorderImageSideWidth::Auto => {}
    }
    visitor.leave_node(AstType::BorderImageSideWidth);
}
pub fn walk_outline_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut OutlineStyle)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::OutlineStyle);
    match node {
        OutlineStyle::Auto => {}
        OutlineStyle::LineStyle(field_0) => {
            visitor.visit_line_style(field_0);
        }
    }
    visitor.leave_node(AstType::OutlineStyle);
}
pub fn walk_flex_direction<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FlexDirection)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FlexDirection);
    match node {
        FlexDirection::Row => {}
        FlexDirection::RowReverse => {}
        FlexDirection::Column => {}
        FlexDirection::ColumnReverse => {}
    }
    visitor.leave_node(AstType::FlexDirection);
}
pub fn walk_flex_wrap<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FlexWrap)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FlexWrap);
    match node {
        FlexWrap::Nowrap => {}
        FlexWrap::Wrap => {}
        FlexWrap::WrapReverse => {}
    }
    visitor.leave_node(AstType::FlexWrap);
}
pub fn walk_align_content<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AlignContent)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AlignContent);
    match node {
        AlignContent::Normal => {}
        AlignContent::BaselinePosition(field_0) => {
            visitor.visit_baseline_position(field_0);
        }
        AlignContent::ContentDistribution(field_0) => {
            visitor.visit_content_distribution(field_0);
        }
        AlignContent::ContentPosition { overflow, value } => {
            if let Some(value_0) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_0);
            }
            visitor.visit_content_position(value);
        }
    }
    visitor.leave_node(AstType::AlignContent);
}
pub fn walk_baseline_position<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BaselinePosition)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BaselinePosition);
    match node {
        BaselinePosition::First => {}
        BaselinePosition::Last => {}
    }
    visitor.leave_node(AstType::BaselinePosition);
}
pub fn walk_content_distribution<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ContentDistribution,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ContentDistribution);
    match node {
        ContentDistribution::SpaceBetween => {}
        ContentDistribution::SpaceAround => {}
        ContentDistribution::SpaceEvenly => {}
        ContentDistribution::Stretch => {}
    }
    visitor.leave_node(AstType::ContentDistribution);
}
pub fn walk_overflow_position<'a, VisitorT>(visitor: &mut VisitorT, node: &mut OverflowPosition)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::OverflowPosition);
    match node {
        OverflowPosition::Safe => {}
        OverflowPosition::Unsafe => {}
    }
    visitor.leave_node(AstType::OverflowPosition);
}
pub fn walk_content_position<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ContentPosition)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ContentPosition);
    match node {
        ContentPosition::Center => {}
        ContentPosition::Start => {}
        ContentPosition::End => {}
        ContentPosition::FlexStart => {}
        ContentPosition::FlexEnd => {}
    }
    visitor.leave_node(AstType::ContentPosition);
}
pub fn walk_justify_content<'a, VisitorT>(visitor: &mut VisitorT, node: &mut JustifyContent)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::JustifyContent);
    match node {
        JustifyContent::Normal => {}
        JustifyContent::ContentDistribution(field_0) => {
            visitor.visit_content_distribution(field_0);
        }
        JustifyContent::ContentPosition { overflow, value } => {
            if let Some(value_0) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_0);
            }
            visitor.visit_content_position(value);
        }
        JustifyContent::Left { overflow } => {
            if let Some(value_1) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_1);
            }
        }
        JustifyContent::Right { overflow } => {
            if let Some(value_2) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_2);
            }
        }
    }
    visitor.leave_node(AstType::JustifyContent);
}
pub fn walk_align_self<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AlignSelf)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AlignSelf);
    match node {
        AlignSelf::Auto => {}
        AlignSelf::Normal => {}
        AlignSelf::Stretch => {}
        AlignSelf::BaselinePosition(field_0) => {
            visitor.visit_baseline_position(field_0);
        }
        AlignSelf::SelfPosition { overflow, value } => {
            if let Some(value_0) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_0);
            }
            visitor.visit_self_position(value);
        }
    }
    visitor.leave_node(AstType::AlignSelf);
}
pub fn walk_self_position<'a, VisitorT>(visitor: &mut VisitorT, node: &mut SelfPosition)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SelfPosition);
    match node {
        SelfPosition::Center => {}
        SelfPosition::Start => {}
        SelfPosition::End => {}
        SelfPosition::SelfStart => {}
        SelfPosition::SelfEnd => {}
        SelfPosition::FlexStart => {}
        SelfPosition::FlexEnd => {}
    }
    visitor.leave_node(AstType::SelfPosition);
}
pub fn walk_justify_self<'a, VisitorT>(visitor: &mut VisitorT, node: &mut JustifySelf)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::JustifySelf);
    match node {
        JustifySelf::Auto => {}
        JustifySelf::Normal => {}
        JustifySelf::Stretch => {}
        JustifySelf::BaselinePosition(field_0) => {
            visitor.visit_baseline_position(field_0);
        }
        JustifySelf::SelfPosition { overflow, value } => {
            if let Some(value_0) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_0);
            }
            visitor.visit_self_position(value);
        }
        JustifySelf::Left { overflow } => {
            if let Some(value_1) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_1);
            }
        }
        JustifySelf::Right { overflow } => {
            if let Some(value_2) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_2);
            }
        }
    }
    visitor.leave_node(AstType::JustifySelf);
}
pub fn walk_align_items<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AlignItems)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AlignItems);
    match node {
        AlignItems::Normal => {}
        AlignItems::Stretch => {}
        AlignItems::BaselinePosition(field_0) => {
            visitor.visit_baseline_position(field_0);
        }
        AlignItems::SelfPosition { overflow, value } => {
            if let Some(value_0) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_0);
            }
            visitor.visit_self_position(value);
        }
    }
    visitor.leave_node(AstType::AlignItems);
}
pub fn walk_justify_items<'a, VisitorT>(visitor: &mut VisitorT, node: &mut JustifyItems)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::JustifyItems);
    match node {
        JustifyItems::Normal => {}
        JustifyItems::Stretch => {}
        JustifyItems::BaselinePosition(field_0) => {
            visitor.visit_baseline_position(field_0);
        }
        JustifyItems::SelfPosition { overflow, value } => {
            if let Some(value_0) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_0);
            }
            visitor.visit_self_position(value);
        }
        JustifyItems::Left { overflow } => {
            if let Some(value_1) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_1);
            }
        }
        JustifyItems::Right { overflow } => {
            if let Some(value_2) = (overflow).as_mut() {
                visitor.visit_overflow_position(value_2);
            }
        }
        JustifyItems::Legacy(field_0) => {
            visitor.visit_legacy_justify(field_0);
        }
    }
    visitor.leave_node(AstType::JustifyItems);
}
pub fn walk_legacy_justify<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LegacyJustify)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LegacyJustify);
    match node {
        LegacyJustify::Left => {}
        LegacyJustify::Right => {}
        LegacyJustify::Center => {}
    }
    visitor.leave_node(AstType::LegacyJustify);
}
pub fn walk_gap_value<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GapValue<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GapValue);
    match node {
        GapValue::Normal => {}
        GapValue::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::GapValue);
}
pub fn walk_box_orient<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BoxOrient)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BoxOrient);
    match node {
        BoxOrient::Horizontal => {}
        BoxOrient::Vertical => {}
        BoxOrient::InlineAxis => {}
        BoxOrient::BlockAxis => {}
    }
    visitor.leave_node(AstType::BoxOrient);
}
pub fn walk_box_direction<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BoxDirection)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BoxDirection);
    match node {
        BoxDirection::Normal => {}
        BoxDirection::Reverse => {}
    }
    visitor.leave_node(AstType::BoxDirection);
}
pub fn walk_box_align<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BoxAlign)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BoxAlign);
    match node {
        BoxAlign::Start => {}
        BoxAlign::End => {}
        BoxAlign::Center => {}
        BoxAlign::Baseline => {}
        BoxAlign::Stretch => {}
    }
    visitor.leave_node(AstType::BoxAlign);
}
pub fn walk_box_pack<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BoxPack)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BoxPack);
    match node {
        BoxPack::Start => {}
        BoxPack::End => {}
        BoxPack::Center => {}
        BoxPack::Justify => {}
    }
    visitor.leave_node(AstType::BoxPack);
}
pub fn walk_box_lines<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BoxLines)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BoxLines);
    match node {
        BoxLines::Single => {}
        BoxLines::Multiple => {}
    }
    visitor.leave_node(AstType::BoxLines);
}
pub fn walk_flex_pack<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FlexPack)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FlexPack);
    match node {
        FlexPack::Start => {}
        FlexPack::End => {}
        FlexPack::Center => {}
        FlexPack::Justify => {}
        FlexPack::Distribute => {}
    }
    visitor.leave_node(AstType::FlexPack);
}
pub fn walk_flex_item_align<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FlexItemAlign)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FlexItemAlign);
    match node {
        FlexItemAlign::Auto => {}
        FlexItemAlign::Start => {}
        FlexItemAlign::End => {}
        FlexItemAlign::Center => {}
        FlexItemAlign::Baseline => {}
        FlexItemAlign::Stretch => {}
    }
    visitor.leave_node(AstType::FlexItemAlign);
}
pub fn walk_flex_line_pack<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FlexLinePack)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FlexLinePack);
    match node {
        FlexLinePack::Start => {}
        FlexLinePack::End => {}
        FlexLinePack::Center => {}
        FlexLinePack::Justify => {}
        FlexLinePack::Distribute => {}
        FlexLinePack::Stretch => {}
    }
    visitor.leave_node(AstType::FlexLinePack);
}
pub fn walk_track_sizing<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TrackSizing<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TrackSizing);
    match node {
        TrackSizing::None => {}
        TrackSizing::TrackList { items, line_names } => {
            for value_0 in (items).iter_mut() {
                visitor.visit_track_list_item(value_0);
            }
            for value_1 in (line_names).iter_mut() {
                for value_2 in (value_1).iter_mut() {
                    visitor.visit_str(value_2);
                }
            }
        }
    }
    visitor.leave_node(AstType::TrackSizing);
}
pub fn walk_track_list_item<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TrackListItem<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TrackListItem);
    match node {
        TrackListItem::TrackSize(field_0) => {
            visitor.visit_track_size((field_0).as_mut());
        }
        TrackListItem::TrackRepeat(field_0) => {
            visitor.visit_track_repeat((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::TrackListItem);
}
pub fn walk_track_size<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TrackSize<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TrackSize);
    match node {
        TrackSize::TrackBreadth(field_0) => {
            visitor.visit_track_breadth((field_0).as_mut());
        }
        TrackSize::MinMax { max, min } => {
            visitor.visit_track_breadth((max).as_mut());
            visitor.visit_track_breadth((min).as_mut());
        }
        TrackSize::FitContent(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::TrackSize);
}
pub fn walk_track_breadth<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TrackBreadth<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TrackBreadth);
    match node {
        TrackBreadth::Length(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        TrackBreadth::Flex(field_0) => {}
        TrackBreadth::MinContent => {}
        TrackBreadth::MaxContent => {}
        TrackBreadth::Auto => {}
    }
    visitor.leave_node(AstType::TrackBreadth);
}
pub fn walk_repeat_count<'a, VisitorT>(visitor: &mut VisitorT, node: &mut RepeatCount)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::RepeatCount);
    match node {
        RepeatCount::Number(field_0) => {}
        RepeatCount::AutoFill => {}
        RepeatCount::AutoFit => {}
    }
    visitor.leave_node(AstType::RepeatCount);
}
pub fn walk_auto_flow_direction<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AutoFlowDirection)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AutoFlowDirection);
    match node {
        AutoFlowDirection::Row => {}
        AutoFlowDirection::Column => {}
    }
    visitor.leave_node(AstType::AutoFlowDirection);
}
pub fn walk_grid_template_areas<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut GridTemplateAreas<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GridTemplateAreas);
    match node {
        GridTemplateAreas::None => {}
        GridTemplateAreas::Areas { areas, columns } => {
            for value_0 in (areas).iter_mut() {
                if let Some(value_1) = (value_0).as_mut() {
                    visitor.visit_str(value_1);
                }
            }
        }
    }
    visitor.leave_node(AstType::GridTemplateAreas);
}
pub fn walk_grid_line<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GridLine<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GridLine);
    match node {
        GridLine::Auto => {}
        GridLine::Area { name } => {
            visitor.visit_str(name);
        }
        GridLine::Line { index, name } => {
            if let Some(value_0) = (name).as_mut() {
                visitor.visit_str(value_0);
            }
        }
        GridLine::Span { index, name } => {
            if let Some(value_1) = (name).as_mut() {
                visitor.visit_str(value_1);
            }
        }
    }
    visitor.leave_node(AstType::GridLine);
}
pub fn walk_font_weight<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontWeight<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontWeight);
    match node {
        FontWeight::Absolute(field_0) => {
            visitor.visit_absolute_font_weight((field_0).as_mut());
        }
        FontWeight::Bolder => {}
        FontWeight::Lighter => {}
    }
    visitor.leave_node(AstType::FontWeight);
}
pub fn walk_absolute_font_weight<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut AbsoluteFontWeight,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AbsoluteFontWeight);
    match node {
        AbsoluteFontWeight::Weight(field_0) => {}
        AbsoluteFontWeight::Normal => {}
        AbsoluteFontWeight::Bold => {}
    }
    visitor.leave_node(AstType::AbsoluteFontWeight);
}
pub fn walk_font_size<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontSize<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontSize);
    match node {
        FontSize::Length(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        FontSize::Absolute(field_0) => {
            visitor.visit_absolute_font_size(field_0);
        }
        FontSize::Relative(field_0) => {
            visitor.visit_relative_font_size(field_0);
        }
    }
    visitor.leave_node(AstType::FontSize);
}
pub fn walk_absolute_font_size<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AbsoluteFontSize)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AbsoluteFontSize);
    match node {
        AbsoluteFontSize::XxSmall => {}
        AbsoluteFontSize::XSmall => {}
        AbsoluteFontSize::Small => {}
        AbsoluteFontSize::Medium => {}
        AbsoluteFontSize::Large => {}
        AbsoluteFontSize::XLarge => {}
        AbsoluteFontSize::XxLarge => {}
        AbsoluteFontSize::XxxLarge => {}
    }
    visitor.leave_node(AstType::AbsoluteFontSize);
}
pub fn walk_relative_font_size<'a, VisitorT>(visitor: &mut VisitorT, node: &mut RelativeFontSize)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::RelativeFontSize);
    match node {
        RelativeFontSize::Smaller => {}
        RelativeFontSize::Larger => {}
    }
    visitor.leave_node(AstType::RelativeFontSize);
}
pub fn walk_font_stretch<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontStretch)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontStretch);
    match node {
        FontStretch::Keyword(field_0) => {
            visitor.visit_font_stretch_keyword(field_0);
        }
        FontStretch::Percentage(field_0) => {}
    }
    visitor.leave_node(AstType::FontStretch);
}
pub fn walk_font_stretch_keyword<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut FontStretchKeyword,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontStretchKeyword);
    match node {
        FontStretchKeyword::Normal => {}
        FontStretchKeyword::UltraCondensed => {}
        FontStretchKeyword::ExtraCondensed => {}
        FontStretchKeyword::Condensed => {}
        FontStretchKeyword::SemiCondensed => {}
        FontStretchKeyword::SemiExpanded => {}
        FontStretchKeyword::Expanded => {}
        FontStretchKeyword::ExtraExpanded => {}
        FontStretchKeyword::UltraExpanded => {}
    }
    visitor.leave_node(AstType::FontStretchKeyword);
}
pub fn walk_font_family<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontFamily<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFamily);
    match node {
        FontFamily::Generic(field_0) => {
            visitor.visit_generic_font_family(field_0);
        }
        FontFamily::FamilyName(field_0) => {
            visitor.visit_family_name(field_0);
        }
    }
    visitor.leave_node(AstType::FontFamily);
}
pub fn walk_generic_font_family<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GenericFontFamily)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GenericFontFamily);
    match node {
        GenericFontFamily::Serif => {}
        GenericFontFamily::SansSerif => {}
        GenericFontFamily::Cursive => {}
        GenericFontFamily::Fantasy => {}
        GenericFontFamily::Monospace => {}
        GenericFontFamily::SystemUi => {}
        GenericFontFamily::Emoji => {}
        GenericFontFamily::Math => {}
        GenericFontFamily::Fangsong => {}
        GenericFontFamily::UiSerif => {}
        GenericFontFamily::UiSansSerif => {}
        GenericFontFamily::UiMonospace => {}
        GenericFontFamily::UiRounded => {}
        GenericFontFamily::Initial => {}
        GenericFontFamily::Inherit => {}
        GenericFontFamily::Unset => {}
        GenericFontFamily::Default => {}
        GenericFontFamily::Revert => {}
        GenericFontFamily::RevertLayer => {}
    }
    visitor.leave_node(AstType::GenericFontFamily);
}
pub fn walk_font_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontStyle<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontStyle);
    match node {
        FontStyle::Normal => {}
        FontStyle::Italic => {}
        FontStyle::Oblique(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::FontStyle);
}
pub fn walk_font_variant_caps<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontVariantCaps)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontVariantCaps);
    match node {
        FontVariantCaps::Normal => {}
        FontVariantCaps::SmallCaps => {}
        FontVariantCaps::AllSmallCaps => {}
        FontVariantCaps::PetiteCaps => {}
        FontVariantCaps::AllPetiteCaps => {}
        FontVariantCaps::Unicase => {}
        FontVariantCaps::TitlingCaps => {}
    }
    visitor.leave_node(AstType::FontVariantCaps);
}
pub fn walk_line_height<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LineHeight<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LineHeight);
    match node {
        LineHeight::Normal => {}
        LineHeight::Number(field_0) => {}
        LineHeight::Length(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::LineHeight);
}
pub fn walk_vertical_align<'a, VisitorT>(visitor: &mut VisitorT, node: &mut VerticalAlign<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::VerticalAlign);
    match node {
        VerticalAlign::Keyword(field_0) => {
            visitor.visit_vertical_align_keyword(field_0);
        }
        VerticalAlign::Length(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::VerticalAlign);
}
pub fn walk_vertical_align_keyword<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut VerticalAlignKeyword,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::VerticalAlignKeyword);
    match node {
        VerticalAlignKeyword::Baseline => {}
        VerticalAlignKeyword::Sub => {}
        VerticalAlignKeyword::Super => {}
        VerticalAlignKeyword::Top => {}
        VerticalAlignKeyword::TextTop => {}
        VerticalAlignKeyword::Middle => {}
        VerticalAlignKeyword::Bottom => {}
        VerticalAlignKeyword::TextBottom => {}
    }
    visitor.leave_node(AstType::VerticalAlignKeyword);
}
pub fn walk_easing_function<'a, VisitorT>(visitor: &mut VisitorT, node: &mut EasingFunction)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::EasingFunction);
    match node {
        EasingFunction::Linear => {}
        EasingFunction::Ease => {}
        EasingFunction::EaseIn => {}
        EasingFunction::EaseOut => {}
        EasingFunction::EaseInOut => {}
        EasingFunction::CubicBezier { x1, x2, y1, y2 } => {}
        EasingFunction::Steps { count, position } => {
            visitor.visit_step_position(position);
        }
    }
    visitor.leave_node(AstType::EasingFunction);
}
pub fn walk_step_position<'a, VisitorT>(visitor: &mut VisitorT, node: &mut StepPosition)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::StepPosition);
    match node {
        StepPosition::Start => {}
        StepPosition::End => {}
        StepPosition::JumpNone => {}
        StepPosition::JumpBoth => {}
    }
    visitor.leave_node(AstType::StepPosition);
}
pub fn walk_animation_iteration_count<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut AnimationIterationCount,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationIterationCount);
    match node {
        AnimationIterationCount::Number(field_0) => {}
        AnimationIterationCount::Infinite => {}
    }
    visitor.leave_node(AstType::AnimationIterationCount);
}
pub fn walk_animation_direction<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AnimationDirection)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationDirection);
    match node {
        AnimationDirection::Normal => {}
        AnimationDirection::Reverse => {}
        AnimationDirection::Alternate => {}
        AnimationDirection::AlternateReverse => {}
    }
    visitor.leave_node(AstType::AnimationDirection);
}
pub fn walk_animation_play_state<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut AnimationPlayState,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationPlayState);
    match node {
        AnimationPlayState::Running => {}
        AnimationPlayState::Paused => {}
    }
    visitor.leave_node(AstType::AnimationPlayState);
}
pub fn walk_animation_fill_mode<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AnimationFillMode)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationFillMode);
    match node {
        AnimationFillMode::None => {}
        AnimationFillMode::Forwards => {}
        AnimationFillMode::Backwards => {}
        AnimationFillMode::Both => {}
    }
    visitor.leave_node(AstType::AnimationFillMode);
}
pub fn walk_animation_composition<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut AnimationComposition,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationComposition);
    match node {
        AnimationComposition::Replace => {}
        AnimationComposition::Add => {}
        AnimationComposition::Accumulate => {}
    }
    visitor.leave_node(AstType::AnimationComposition);
}
pub fn walk_animation_timeline<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut AnimationTimeline<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationTimeline);
    match node {
        AnimationTimeline::Auto => {}
        AnimationTimeline::None => {}
        AnimationTimeline::DashedIdent(field_0) => {
            visitor.visit_str(field_0);
        }
        AnimationTimeline::Scroll(field_0) => {
            visitor.visit_scroll_timeline((field_0).as_mut());
        }
        AnimationTimeline::View(field_0) => {
            visitor.visit_view_timeline((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::AnimationTimeline);
}
pub fn walk_scroll_axis<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ScrollAxis)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollAxis);
    match node {
        ScrollAxis::Block => {}
        ScrollAxis::Inline => {}
        ScrollAxis::X => {}
        ScrollAxis::Y => {}
    }
    visitor.leave_node(AstType::ScrollAxis);
}
pub fn walk_scroller<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Scroller)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Scroller);
    match node {
        Scroller::Root => {}
        Scroller::Nearest => {}
        Scroller::Self_ => {}
    }
    visitor.leave_node(AstType::Scroller);
}
pub fn walk_animation_attachment_range<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut AnimationAttachmentRange<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationAttachmentRange);
    match node {
        AnimationAttachmentRange::Normal => {}
        AnimationAttachmentRange::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        AnimationAttachmentRange::TimelineRange { name, offset } => {
            visitor.visit_timeline_range_name(name);
            visitor.visit_length_percentage((offset).as_mut());
        }
    }
    visitor.leave_node(AstType::AnimationAttachmentRange);
}
pub fn walk_timeline_range_name<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TimelineRangeName)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TimelineRangeName);
    match node {
        TimelineRangeName::Cover => {}
        TimelineRangeName::Contain => {}
        TimelineRangeName::Entry => {}
        TimelineRangeName::Exit => {}
        TimelineRangeName::EntryCrossing => {}
        TimelineRangeName::ExitCrossing => {}
    }
    visitor.leave_node(AstType::TimelineRangeName);
}
pub fn walk_transform<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Transform<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Transform);
    match node {
        Transform::Translate(field_0) => {
            visitor.visit_length_percentage((&mut (field_0).0).as_mut());
            visitor.visit_length_percentage((&mut (field_0).1).as_mut());
        }
        Transform::TranslateX(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        Transform::TranslateY(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        Transform::TranslateZ(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
        Transform::Translate3d(field_0) => {
            visitor.visit_length_percentage((&mut (field_0).0).as_mut());
            visitor.visit_length_percentage((&mut (field_0).1).as_mut());
            visitor.visit_length((&mut (field_0).2).as_mut());
        }
        Transform::Scale(field_0) => {
            visitor.visit_number_or_percentage((&mut (field_0).0).as_mut());
            visitor.visit_number_or_percentage((&mut (field_0).1).as_mut());
        }
        Transform::ScaleX(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Transform::ScaleY(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Transform::ScaleZ(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Transform::Scale3d(field_0) => {
            visitor.visit_number_or_percentage((&mut (field_0).0).as_mut());
            visitor.visit_number_or_percentage((&mut (field_0).1).as_mut());
            visitor.visit_number_or_percentage((&mut (field_0).2).as_mut());
        }
        Transform::Rotate(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        Transform::RotateX(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        Transform::RotateY(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        Transform::RotateZ(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        Transform::Rotate3d(field_0) => {
            visitor.visit_angle((&mut (field_0).3).as_mut());
        }
        Transform::Skew(field_0) => {
            visitor.visit_angle((&mut (field_0).0).as_mut());
            visitor.visit_angle((&mut (field_0).1).as_mut());
        }
        Transform::SkewX(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        Transform::SkewY(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        Transform::Perspective(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
        Transform::Matrix(field_0) => {
            visitor.visit_matrix_for_float((field_0).as_mut());
        }
        Transform::Matrix3d(field_0) => {
            visitor.visit_matrix_3_d_for_float((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Transform);
}
pub fn walk_transform_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TransformStyle)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TransformStyle);
    match node {
        TransformStyle::Flat => {}
        TransformStyle::Preserve3d => {}
    }
    visitor.leave_node(AstType::TransformStyle);
}
pub fn walk_transform_box<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TransformBox)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TransformBox);
    match node {
        TransformBox::ContentBox => {}
        TransformBox::BorderBox => {}
        TransformBox::FillBox => {}
        TransformBox::StrokeBox => {}
        TransformBox::ViewBox => {}
    }
    visitor.leave_node(AstType::TransformBox);
}
pub fn walk_backface_visibility<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BackfaceVisibility)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BackfaceVisibility);
    match node {
        BackfaceVisibility::Visible => {}
        BackfaceVisibility::Hidden => {}
    }
    visitor.leave_node(AstType::BackfaceVisibility);
}
pub fn walk_perspective<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Perspective<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Perspective);
    match node {
        Perspective::None => {}
        Perspective::Length(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Perspective);
}
pub fn walk_translate<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Translate<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Translate);
    match node {
        Translate::None => {}
        Translate::Xyz { x, y, z } => {
            visitor.visit_length_percentage((x).as_mut());
            visitor.visit_length_percentage((y).as_mut());
            visitor.visit_length((z).as_mut());
        }
    }
    visitor.leave_node(AstType::Translate);
}
pub fn walk_scale<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Scale<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Scale);
    match node {
        Scale::None => {}
        Scale::Xyz { x, y, z } => {
            visitor.visit_number_or_percentage((x).as_mut());
            visitor.visit_number_or_percentage((y).as_mut());
            visitor.visit_number_or_percentage((z).as_mut());
        }
    }
    visitor.leave_node(AstType::Scale);
}
pub fn walk_text_transform_case<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextTransformCase)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextTransformCase);
    match node {
        TextTransformCase::None => {}
        TextTransformCase::Uppercase => {}
        TextTransformCase::Lowercase => {}
        TextTransformCase::Capitalize => {}
    }
    visitor.leave_node(AstType::TextTransformCase);
}
pub fn walk_white_space<'a, VisitorT>(visitor: &mut VisitorT, node: &mut WhiteSpace)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::WhiteSpace);
    match node {
        WhiteSpace::Normal => {}
        WhiteSpace::Pre => {}
        WhiteSpace::Nowrap => {}
        WhiteSpace::PreWrap => {}
        WhiteSpace::BreakSpaces => {}
        WhiteSpace::PreLine => {}
    }
    visitor.leave_node(AstType::WhiteSpace);
}
pub fn walk_word_break<'a, VisitorT>(visitor: &mut VisitorT, node: &mut WordBreak)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::WordBreak);
    match node {
        WordBreak::Normal => {}
        WordBreak::KeepAll => {}
        WordBreak::BreakAll => {}
        WordBreak::BreakWord => {}
    }
    visitor.leave_node(AstType::WordBreak);
}
pub fn walk_line_break<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LineBreak)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LineBreak);
    match node {
        LineBreak::Auto => {}
        LineBreak::Loose => {}
        LineBreak::Normal => {}
        LineBreak::Strict => {}
        LineBreak::Anywhere => {}
    }
    visitor.leave_node(AstType::LineBreak);
}
pub fn walk_hyphens<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Hyphens)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Hyphens);
    match node {
        Hyphens::None => {}
        Hyphens::Manual => {}
        Hyphens::Auto => {}
    }
    visitor.leave_node(AstType::Hyphens);
}
pub fn walk_overflow_wrap<'a, VisitorT>(visitor: &mut VisitorT, node: &mut OverflowWrap)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::OverflowWrap);
    match node {
        OverflowWrap::Normal => {}
        OverflowWrap::Anywhere => {}
        OverflowWrap::BreakWord => {}
    }
    visitor.leave_node(AstType::OverflowWrap);
}
pub fn walk_text_align<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextAlign)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextAlign);
    match node {
        TextAlign::Start => {}
        TextAlign::End => {}
        TextAlign::Left => {}
        TextAlign::Right => {}
        TextAlign::Center => {}
        TextAlign::Justify => {}
        TextAlign::MatchParent => {}
        TextAlign::JustifyAll => {}
    }
    visitor.leave_node(AstType::TextAlign);
}
pub fn walk_text_align_last<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextAlignLast)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextAlignLast);
    match node {
        TextAlignLast::Auto => {}
        TextAlignLast::Start => {}
        TextAlignLast::End => {}
        TextAlignLast::Left => {}
        TextAlignLast::Right => {}
        TextAlignLast::Center => {}
        TextAlignLast::Justify => {}
        TextAlignLast::MatchParent => {}
    }
    visitor.leave_node(AstType::TextAlignLast);
}
pub fn walk_text_justify<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextJustify)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextJustify);
    match node {
        TextJustify::Auto => {}
        TextJustify::None => {}
        TextJustify::InterWord => {}
        TextJustify::InterCharacter => {}
    }
    visitor.leave_node(AstType::TextJustify);
}
pub fn walk_spacing<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Spacing<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Spacing);
    match node {
        Spacing::Normal => {}
        Spacing::Length(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Spacing);
}
pub fn walk_text_decoration_line<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextDecorationLine<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextDecorationLine);
    match node {
        TextDecorationLine::ExclusiveTextDecorationLine(field_0) => {
            visitor.visit_exclusive_text_decoration_line(field_0);
        }
        TextDecorationLine::Value(field_0) => {
            for value_0 in (field_0).iter_mut() {
                visitor.visit_other_text_decoration_line(value_0);
            }
        }
    }
    visitor.leave_node(AstType::TextDecorationLine);
}
pub fn walk_exclusive_text_decoration_line<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ExclusiveTextDecorationLine,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ExclusiveTextDecorationLine);
    match node {
        ExclusiveTextDecorationLine::None => {}
        ExclusiveTextDecorationLine::SpellingError => {}
        ExclusiveTextDecorationLine::GrammarError => {}
    }
    visitor.leave_node(AstType::ExclusiveTextDecorationLine);
}
pub fn walk_other_text_decoration_line<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut OtherTextDecorationLine,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::OtherTextDecorationLine);
    match node {
        OtherTextDecorationLine::Underline => {}
        OtherTextDecorationLine::Overline => {}
        OtherTextDecorationLine::LineThrough => {}
        OtherTextDecorationLine::Blink => {}
    }
    visitor.leave_node(AstType::OtherTextDecorationLine);
}
pub fn walk_text_decoration_style<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextDecorationStyle,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextDecorationStyle);
    match node {
        TextDecorationStyle::Solid => {}
        TextDecorationStyle::Double => {}
        TextDecorationStyle::Dotted => {}
        TextDecorationStyle::Dashed => {}
        TextDecorationStyle::Wavy => {}
    }
    visitor.leave_node(AstType::TextDecorationStyle);
}
pub fn walk_text_decoration_thickness<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextDecorationThickness<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextDecorationThickness);
    match node {
        TextDecorationThickness::Auto => {}
        TextDecorationThickness::FromFont => {}
        TextDecorationThickness::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::TextDecorationThickness);
}
pub fn walk_text_decoration_skip_ink<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextDecorationSkipInk,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextDecorationSkipInk);
    match node {
        TextDecorationSkipInk::Auto => {}
        TextDecorationSkipInk::None => {}
        TextDecorationSkipInk::All => {}
    }
    visitor.leave_node(AstType::TextDecorationSkipInk);
}
pub fn walk_text_emphasis_style<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextEmphasisStyle<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextEmphasisStyle);
    match node {
        TextEmphasisStyle::None => {}
        TextEmphasisStyle::Keyword { fill, shape } => {
            visitor.visit_text_emphasis_fill_mode(fill);
            if let Some(value_0) = (shape).as_mut() {
                visitor.visit_text_emphasis_shape(value_0);
            }
        }
        TextEmphasisStyle::String(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::TextEmphasisStyle);
}
pub fn walk_text_emphasis_fill_mode<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextEmphasisFillMode,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextEmphasisFillMode);
    match node {
        TextEmphasisFillMode::Filled => {}
        TextEmphasisFillMode::Open => {}
    }
    visitor.leave_node(AstType::TextEmphasisFillMode);
}
pub fn walk_text_emphasis_shape<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextEmphasisShape)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextEmphasisShape);
    match node {
        TextEmphasisShape::Dot => {}
        TextEmphasisShape::Circle => {}
        TextEmphasisShape::DoubleCircle => {}
        TextEmphasisShape::Triangle => {}
        TextEmphasisShape::Sesame => {}
    }
    visitor.leave_node(AstType::TextEmphasisShape);
}
pub fn walk_text_emphasis_position_horizontal<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextEmphasisPositionHorizontal,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextEmphasisPositionHorizontal);
    match node {
        TextEmphasisPositionHorizontal::Left => {}
        TextEmphasisPositionHorizontal::Right => {}
    }
    visitor.leave_node(AstType::TextEmphasisPositionHorizontal);
}
pub fn walk_text_emphasis_position_vertical<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextEmphasisPositionVertical,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextEmphasisPositionVertical);
    match node {
        TextEmphasisPositionVertical::Over => {}
        TextEmphasisPositionVertical::Under => {}
    }
    visitor.leave_node(AstType::TextEmphasisPositionVertical);
}
pub fn walk_text_size_adjust<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextSizeAdjust)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextSizeAdjust);
    match node {
        TextSizeAdjust::Auto => {}
        TextSizeAdjust::None => {}
        TextSizeAdjust::Percentage(field_0) => {}
    }
    visitor.leave_node(AstType::TextSizeAdjust);
}
pub fn walk_text_direction<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextDirection)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextDirection);
    match node {
        TextDirection::Ltr => {}
        TextDirection::Rtl => {}
    }
    visitor.leave_node(AstType::TextDirection);
}
pub fn walk_unicode_bidi<'a, VisitorT>(visitor: &mut VisitorT, node: &mut UnicodeBidi)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::UnicodeBidi);
    match node {
        UnicodeBidi::Normal => {}
        UnicodeBidi::Embed => {}
        UnicodeBidi::Isolate => {}
        UnicodeBidi::BidiOverride => {}
        UnicodeBidi::IsolateOverride => {}
        UnicodeBidi::Plaintext => {}
    }
    visitor.leave_node(AstType::UnicodeBidi);
}
pub fn walk_box_decoration_break<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BoxDecorationBreak,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BoxDecorationBreak);
    match node {
        BoxDecorationBreak::Slice => {}
        BoxDecorationBreak::Clone => {}
    }
    visitor.leave_node(AstType::BoxDecorationBreak);
}
pub fn walk_resize<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Resize)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Resize);
    match node {
        Resize::None => {}
        Resize::Both => {}
        Resize::Horizontal => {}
        Resize::Vertical => {}
        Resize::Block => {}
        Resize::Inline => {}
    }
    visitor.leave_node(AstType::Resize);
}
pub fn walk_cursor_keyword<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CursorKeyword)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CursorKeyword);
    match node {
        CursorKeyword::Auto => {}
        CursorKeyword::Default => {}
        CursorKeyword::None => {}
        CursorKeyword::ContextMenu => {}
        CursorKeyword::Help => {}
        CursorKeyword::Pointer => {}
        CursorKeyword::Progress => {}
        CursorKeyword::Wait => {}
        CursorKeyword::Cell => {}
        CursorKeyword::Crosshair => {}
        CursorKeyword::Text => {}
        CursorKeyword::VerticalText => {}
        CursorKeyword::Alias => {}
        CursorKeyword::Copy => {}
        CursorKeyword::Move => {}
        CursorKeyword::NoDrop => {}
        CursorKeyword::NotAllowed => {}
        CursorKeyword::Grab => {}
        CursorKeyword::Grabbing => {}
        CursorKeyword::EResize => {}
        CursorKeyword::NResize => {}
        CursorKeyword::NeResize => {}
        CursorKeyword::NwResize => {}
        CursorKeyword::SResize => {}
        CursorKeyword::SeResize => {}
        CursorKeyword::SwResize => {}
        CursorKeyword::WResize => {}
        CursorKeyword::EwResize => {}
        CursorKeyword::NsResize => {}
        CursorKeyword::NeswResize => {}
        CursorKeyword::NwseResize => {}
        CursorKeyword::ColResize => {}
        CursorKeyword::RowResize => {}
        CursorKeyword::AllScroll => {}
        CursorKeyword::ZoomIn => {}
        CursorKeyword::ZoomOut => {}
    }
    visitor.leave_node(AstType::CursorKeyword);
}
pub fn walk_color_or_auto<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ColorOrAuto<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ColorOrAuto);
    match node {
        ColorOrAuto::Auto => {}
        ColorOrAuto::Color(field_0) => {
            visitor.visit_css_color((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::ColorOrAuto);
}
pub fn walk_caret_shape<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CaretShape)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CaretShape);
    match node {
        CaretShape::Auto => {}
        CaretShape::Bar => {}
        CaretShape::Block => {}
        CaretShape::Underscore => {}
    }
    visitor.leave_node(AstType::CaretShape);
}
pub fn walk_user_select<'a, VisitorT>(visitor: &mut VisitorT, node: &mut UserSelect)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::UserSelect);
    match node {
        UserSelect::Auto => {}
        UserSelect::Text => {}
        UserSelect::None => {}
        UserSelect::Contain => {}
        UserSelect::All => {}
    }
    visitor.leave_node(AstType::UserSelect);
}
pub fn walk_appearance<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Appearance<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Appearance);
    match node {
        Appearance::None => {}
        Appearance::Auto => {}
        Appearance::Textfield => {}
        Appearance::MenulistButton => {}
        Appearance::Button => {}
        Appearance::Checkbox => {}
        Appearance::Listbox => {}
        Appearance::Menulist => {}
        Appearance::Meter => {}
        Appearance::ProgressBar => {}
        Appearance::PushButton => {}
        Appearance::Radio => {}
        Appearance::Searchfield => {}
        Appearance::SliderHorizontal => {}
        Appearance::SquareButton => {}
        Appearance::Textarea => {}
        Appearance::NonStandard(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::Appearance);
}
pub fn walk_list_style_type<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ListStyleType<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ListStyleType);
    match node {
        ListStyleType::None => {}
        ListStyleType::String(field_0) => {
            visitor.visit_str(field_0);
        }
        ListStyleType::CounterStyle(field_0) => {
            visitor.visit_counter_style((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::ListStyleType);
}
pub fn walk_counter_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CounterStyle<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CounterStyle);
    match node {
        CounterStyle::Predefined(field_0) => {
            visitor.visit_predefined_counter_style(field_0);
        }
        CounterStyle::Name(field_0) => {
            visitor.visit_str(field_0);
        }
        CounterStyle::Symbols { symbols, system } => {
            for value_0 in (symbols).iter_mut() {
                visitor.visit_symbol(value_0);
            }
            visitor.visit_symbols_type(system);
        }
    }
    visitor.leave_node(AstType::CounterStyle);
}
pub fn walk_symbols_type<'a, VisitorT>(visitor: &mut VisitorT, node: &mut SymbolsType)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SymbolsType);
    match node {
        SymbolsType::Cyclic => {}
        SymbolsType::Numeric => {}
        SymbolsType::Alphabetic => {}
        SymbolsType::Symbolic => {}
        SymbolsType::Fixed => {}
    }
    visitor.leave_node(AstType::SymbolsType);
}
pub fn walk_predefined_counter_style<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut PredefinedCounterStyle,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PredefinedCounterStyle);
    match node {
        PredefinedCounterStyle::Decimal => {}
        PredefinedCounterStyle::DecimalLeadingZero => {}
        PredefinedCounterStyle::ArabicIndic => {}
        PredefinedCounterStyle::Armenian => {}
        PredefinedCounterStyle::UpperArmenian => {}
        PredefinedCounterStyle::LowerArmenian => {}
        PredefinedCounterStyle::Bengali => {}
        PredefinedCounterStyle::Cambodian => {}
        PredefinedCounterStyle::Khmer => {}
        PredefinedCounterStyle::CjkDecimal => {}
        PredefinedCounterStyle::Devanagari => {}
        PredefinedCounterStyle::Georgian => {}
        PredefinedCounterStyle::Gujarati => {}
        PredefinedCounterStyle::Gurmukhi => {}
        PredefinedCounterStyle::Hebrew => {}
        PredefinedCounterStyle::Kannada => {}
        PredefinedCounterStyle::Lao => {}
        PredefinedCounterStyle::Malayalam => {}
        PredefinedCounterStyle::Mongolian => {}
        PredefinedCounterStyle::Myanmar => {}
        PredefinedCounterStyle::Oriya => {}
        PredefinedCounterStyle::Persian => {}
        PredefinedCounterStyle::LowerRoman => {}
        PredefinedCounterStyle::UpperRoman => {}
        PredefinedCounterStyle::Tamil => {}
        PredefinedCounterStyle::Telugu => {}
        PredefinedCounterStyle::Thai => {}
        PredefinedCounterStyle::Tibetan => {}
        PredefinedCounterStyle::LowerAlpha => {}
        PredefinedCounterStyle::LowerLatin => {}
        PredefinedCounterStyle::UpperAlpha => {}
        PredefinedCounterStyle::UpperLatin => {}
        PredefinedCounterStyle::LowerGreek => {}
        PredefinedCounterStyle::Hiragana => {}
        PredefinedCounterStyle::HiraganaIroha => {}
        PredefinedCounterStyle::Katakana => {}
        PredefinedCounterStyle::KatakanaIroha => {}
        PredefinedCounterStyle::Disc => {}
        PredefinedCounterStyle::Circle => {}
        PredefinedCounterStyle::Square => {}
        PredefinedCounterStyle::DisclosureOpen => {}
        PredefinedCounterStyle::DisclosureClosed => {}
        PredefinedCounterStyle::CjkEarthlyBranch => {}
        PredefinedCounterStyle::CjkHeavenlyStem => {}
        PredefinedCounterStyle::JapaneseInformal => {}
        PredefinedCounterStyle::JapaneseFormal => {}
        PredefinedCounterStyle::KoreanHangulFormal => {}
        PredefinedCounterStyle::KoreanHanjaInformal => {}
        PredefinedCounterStyle::KoreanHanjaFormal => {}
        PredefinedCounterStyle::SimpChineseInformal => {}
        PredefinedCounterStyle::SimpChineseFormal => {}
        PredefinedCounterStyle::TradChineseInformal => {}
        PredefinedCounterStyle::TradChineseFormal => {}
        PredefinedCounterStyle::EthiopicNumeric => {}
    }
    visitor.leave_node(AstType::PredefinedCounterStyle);
}
pub fn walk_symbol<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Symbol<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Symbol);
    match node {
        Symbol::String(field_0) => {
            visitor.visit_str(field_0);
        }
        Symbol::Image(field_0) => {
            visitor.visit_image((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Symbol);
}
pub fn walk_list_style_position<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ListStylePosition)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ListStylePosition);
    match node {
        ListStylePosition::Inside => {}
        ListStylePosition::Outside => {}
    }
    visitor.leave_node(AstType::ListStylePosition);
}
pub fn walk_marker_side<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MarkerSide)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MarkerSide);
    match node {
        MarkerSide::MatchSelf => {}
        MarkerSide::MatchParent => {}
    }
    visitor.leave_node(AstType::MarkerSide);
}
pub fn walk_svg_paint<'a, VisitorT>(visitor: &mut VisitorT, node: &mut SVGPaint<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SVGPaint);
    match node {
        SVGPaint::Url { fallback, url } => {
            if let Some(value_0) = (fallback).as_mut() {
                visitor.visit_svg_paint_fallback((value_0).as_mut());
            }
            visitor.visit_url((url).as_mut());
        }
        SVGPaint::Color(field_0) => {
            visitor.visit_css_color((field_0).as_mut());
        }
        SVGPaint::ContextFill => {}
        SVGPaint::ContextStroke => {}
        SVGPaint::None => {}
    }
    visitor.leave_node(AstType::SVGPaint);
}
pub fn walk_svg_paint_fallback<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut SVGPaintFallback<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SVGPaintFallback);
    match node {
        SVGPaintFallback::None => {}
        SVGPaintFallback::Color(field_0) => {
            visitor.visit_css_color((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::SVGPaintFallback);
}
pub fn walk_fill_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FillRule)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FillRule);
    match node {
        FillRule::Nonzero => {}
        FillRule::Evenodd => {}
    }
    visitor.leave_node(AstType::FillRule);
}
pub fn walk_stroke_linecap<'a, VisitorT>(visitor: &mut VisitorT, node: &mut StrokeLinecap)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::StrokeLinecap);
    match node {
        StrokeLinecap::Butt => {}
        StrokeLinecap::Round => {}
        StrokeLinecap::Square => {}
    }
    visitor.leave_node(AstType::StrokeLinecap);
}
pub fn walk_stroke_linejoin<'a, VisitorT>(visitor: &mut VisitorT, node: &mut StrokeLinejoin)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::StrokeLinejoin);
    match node {
        StrokeLinejoin::Miter => {}
        StrokeLinejoin::MiterClip => {}
        StrokeLinejoin::Round => {}
        StrokeLinejoin::Bevel => {}
        StrokeLinejoin::Arcs => {}
    }
    visitor.leave_node(AstType::StrokeLinejoin);
}
pub fn walk_stroke_dasharray<'a, VisitorT>(visitor: &mut VisitorT, node: &mut StrokeDasharray<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::StrokeDasharray);
    match node {
        StrokeDasharray::None => {}
        StrokeDasharray::Values(field_0) => {
            for value_0 in (field_0).iter_mut() {
                visitor.visit_length_percentage(value_0);
            }
        }
    }
    visitor.leave_node(AstType::StrokeDasharray);
}
pub fn walk_marker<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Marker<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Marker);
    match node {
        Marker::None => {}
        Marker::Url(field_0) => {
            visitor.visit_url((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Marker);
}
pub fn walk_color_interpolation<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ColorInterpolation)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ColorInterpolation);
    match node {
        ColorInterpolation::Auto => {}
        ColorInterpolation::Srgb => {}
        ColorInterpolation::Linearrgb => {}
    }
    visitor.leave_node(AstType::ColorInterpolation);
}
pub fn walk_color_rendering<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ColorRendering)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ColorRendering);
    match node {
        ColorRendering::Auto => {}
        ColorRendering::Optimizespeed => {}
        ColorRendering::Optimizequality => {}
    }
    visitor.leave_node(AstType::ColorRendering);
}
pub fn walk_shape_rendering<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ShapeRendering)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ShapeRendering);
    match node {
        ShapeRendering::Auto => {}
        ShapeRendering::Optimizespeed => {}
        ShapeRendering::Crispedges => {}
        ShapeRendering::Geometricprecision => {}
    }
    visitor.leave_node(AstType::ShapeRendering);
}
pub fn walk_text_rendering<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextRendering)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextRendering);
    match node {
        TextRendering::Auto => {}
        TextRendering::Optimizespeed => {}
        TextRendering::Optimizelegibility => {}
        TextRendering::Geometricprecision => {}
    }
    visitor.leave_node(AstType::TextRendering);
}
pub fn walk_image_rendering<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ImageRendering)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ImageRendering);
    match node {
        ImageRendering::Auto => {}
        ImageRendering::Optimizespeed => {}
        ImageRendering::Optimizequality => {}
    }
    visitor.leave_node(AstType::ImageRendering);
}
pub fn walk_clip_path<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ClipPath<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ClipPath);
    match node {
        ClipPath::None => {}
        ClipPath::Url(field_0) => {
            visitor.visit_url((field_0).as_mut());
        }
        ClipPath::Shape {
            reference_box,
            shape,
        } => {
            visitor.visit_geometry_box(reference_box);
            visitor.visit_basic_shape((shape).as_mut());
        }
        ClipPath::Box(field_0) => {
            visitor.visit_geometry_box(field_0);
        }
    }
    visitor.leave_node(AstType::ClipPath);
}
pub fn walk_geometry_box<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GeometryBox)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GeometryBox);
    match node {
        GeometryBox::BorderBox => {}
        GeometryBox::PaddingBox => {}
        GeometryBox::ContentBox => {}
        GeometryBox::MarginBox => {}
        GeometryBox::FillBox => {}
        GeometryBox::StrokeBox => {}
        GeometryBox::ViewBox => {}
    }
    visitor.leave_node(AstType::GeometryBox);
}
pub fn walk_basic_shape<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BasicShape<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BasicShape);
    match node {
        BasicShape::Inset(field_0) => {
            visitor.visit_inset_rect((field_0).as_mut());
        }
        BasicShape::Circle(field_0) => {
            visitor.visit_circle_shape((field_0).as_mut());
        }
        BasicShape::Ellipse(field_0) => {
            visitor.visit_ellipse_shape((field_0).as_mut());
        }
        BasicShape::Polygon(field_0) => {
            visitor.visit_polygon((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::BasicShape);
}
pub fn walk_shape_radius<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ShapeRadius<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ShapeRadius);
    match node {
        ShapeRadius::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        ShapeRadius::ClosestSide => {}
        ShapeRadius::FarthestSide => {}
    }
    visitor.leave_node(AstType::ShapeRadius);
}
pub fn walk_mask_mode<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MaskMode)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MaskMode);
    match node {
        MaskMode::Luminance => {}
        MaskMode::Alpha => {}
        MaskMode::MatchSource => {}
    }
    visitor.leave_node(AstType::MaskMode);
}
pub fn walk_mask_clip<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MaskClip)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MaskClip);
    match node {
        MaskClip::GeometryBox(field_0) => {
            visitor.visit_geometry_box(field_0);
        }
        MaskClip::NoClip => {}
    }
    visitor.leave_node(AstType::MaskClip);
}
pub fn walk_mask_composite<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MaskComposite)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MaskComposite);
    match node {
        MaskComposite::Add => {}
        MaskComposite::Subtract => {}
        MaskComposite::Intersect => {}
        MaskComposite::Exclude => {}
    }
    visitor.leave_node(AstType::MaskComposite);
}
pub fn walk_mask_type<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MaskType)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MaskType);
    match node {
        MaskType::Luminance => {}
        MaskType::Alpha => {}
    }
    visitor.leave_node(AstType::MaskType);
}
pub fn walk_mask_border_mode<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MaskBorderMode)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MaskBorderMode);
    match node {
        MaskBorderMode::Luminance => {}
        MaskBorderMode::Alpha => {}
    }
    visitor.leave_node(AstType::MaskBorderMode);
}
pub fn walk_web_kit_mask_composite<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut WebKitMaskComposite,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::WebKitMaskComposite);
    match node {
        WebKitMaskComposite::Clear => {}
        WebKitMaskComposite::Copy => {}
        WebKitMaskComposite::SourceOver => {}
        WebKitMaskComposite::SourceIn => {}
        WebKitMaskComposite::SourceOut => {}
        WebKitMaskComposite::SourceAtop => {}
        WebKitMaskComposite::DestinationOver => {}
        WebKitMaskComposite::DestinationIn => {}
        WebKitMaskComposite::DestinationOut => {}
        WebKitMaskComposite::DestinationAtop => {}
        WebKitMaskComposite::Xor => {}
    }
    visitor.leave_node(AstType::WebKitMaskComposite);
}
pub fn walk_web_kit_mask_source_type<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut WebKitMaskSourceType,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::WebKitMaskSourceType);
    match node {
        WebKitMaskSourceType::Auto => {}
        WebKitMaskSourceType::Luminance => {}
        WebKitMaskSourceType::Alpha => {}
    }
    visitor.leave_node(AstType::WebKitMaskSourceType);
}
pub fn walk_filter_list<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FilterList<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FilterList);
    match node {
        FilterList::None => {}
        FilterList::Filters(field_0) => {
            for value_0 in (field_0).iter_mut() {
                visitor.visit_filter(value_0);
            }
        }
    }
    visitor.leave_node(AstType::FilterList);
}
pub fn walk_filter<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Filter<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Filter);
    match node {
        Filter::Blur(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
        Filter::Brightness(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Filter::Contrast(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Filter::Grayscale(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Filter::HueRotate(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        Filter::Invert(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Filter::Opacity(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Filter::Saturate(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Filter::Sepia(field_0) => {
            visitor.visit_number_or_percentage((field_0).as_mut());
        }
        Filter::DropShadow(field_0) => {
            visitor.visit_drop_shadow((field_0).as_mut());
        }
        Filter::Url(field_0) => {
            visitor.visit_url((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Filter);
}
pub fn walk_z_index<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ZIndex)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ZIndex);
    match node {
        ZIndex::Auto => {}
        ZIndex::Integer(field_0) => {}
    }
    visitor.leave_node(AstType::ZIndex);
}
pub fn walk_container_type<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ContainerType)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ContainerType);
    match node {
        ContainerType::Normal => {}
        ContainerType::InlineSize => {}
        ContainerType::Size => {}
        ContainerType::ScrollState => {}
    }
    visitor.leave_node(AstType::ContainerType);
}
pub fn walk_container_name_list<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ContainerNameList<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ContainerNameList);
    match node {
        ContainerNameList::None => {}
        ContainerNameList::Names(field_0) => {
            for value_0 in (field_0).iter_mut() {
                visitor.visit_str(value_0);
            }
        }
    }
    visitor.leave_node(AstType::ContainerNameList);
}
pub fn walk_view_transition_name<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ViewTransitionName<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ViewTransitionName);
    match node {
        ViewTransitionName::None => {}
        ViewTransitionName::Auto => {}
        ViewTransitionName::Custom(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::ViewTransitionName);
}
pub fn walk_none_or_custom_ident_list<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut NoneOrCustomIdentList<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::NoneOrCustomIdentList);
    match node {
        NoneOrCustomIdentList::None => {}
        NoneOrCustomIdentList::Idents(field_0) => {
            for value_0 in (field_0).iter_mut() {
                visitor.visit_str(value_0);
            }
        }
    }
    visitor.leave_node(AstType::NoneOrCustomIdentList);
}
pub fn walk_view_transition_group<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ViewTransitionGroup<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ViewTransitionGroup);
    match node {
        ViewTransitionGroup::Normal => {}
        ViewTransitionGroup::Contain => {}
        ViewTransitionGroup::Nearest => {}
        ViewTransitionGroup::Custom(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::ViewTransitionGroup);
}
pub fn walk_print_color_adjust<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PrintColorAdjust)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PrintColorAdjust);
    match node {
        PrintColorAdjust::Economy => {}
        PrintColorAdjust::Exact => {}
    }
    visitor.leave_node(AstType::PrintColorAdjust);
}
pub fn walk_css_wide_keyword<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CSSWideKeyword)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CSSWideKeyword);
    match node {
        CSSWideKeyword::Initial => {}
        CSSWideKeyword::Inherit => {}
        CSSWideKeyword::Unset => {}
        CSSWideKeyword::Revert => {}
        CSSWideKeyword::RevertLayer => {}
    }
    visitor.leave_node(AstType::CSSWideKeyword);
}
pub fn walk_custom_property_name<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut CustomPropertyName<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CustomPropertyName);
    match node {
        CustomPropertyName::Custom(field_0) => {
            visitor.visit_str(field_0);
        }
        CustomPropertyName::Unknown(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::CustomPropertyName);
}
pub fn walk_length_percentage<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LengthPercentage<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LengthPercentage);
    visitor.visit_dimension_percentage(node);
    visitor.leave_node(AstType::LengthPercentage);
}
pub fn walk_angle_percentage<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AnglePercentage<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnglePercentage);
    visitor.visit_dimension_percentage(node);
    visitor.leave_node(AstType::AnglePercentage);
}
pub fn walk_animation_range_start<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut AnimationRangeStart<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationRangeStart);
    visitor.visit_animation_attachment_range(node);
    visitor.leave_node(AstType::AnimationRangeStart);
}
pub fn walk_animation_range_end<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut AnimationRangeEnd<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationRangeEnd);
    visitor.visit_animation_attachment_range(node);
    visitor.leave_node(AstType::AnimationRangeEnd);
}
