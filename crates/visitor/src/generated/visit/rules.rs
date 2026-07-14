#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for Transition<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_transition(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Transition);
        let node = self;
        Visit::visit((&node.delay).as_ref(), visitor);
        Visit::visit((&node.duration).as_ref(), visitor);
        Visit::visit((&node.property).as_ref(), visitor);
        Visit::visit((&node.timing_function).as_ref(), visitor);
        visitor.leave_node(AstType::Transition);
    }
}
impl<'a> Visit<'a> for ScrollTimeline {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_timeline(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollTimeline);
        let node = self;
        Visit::visit(&node.axis, visitor);
        Visit::visit(&node.scroller, visitor);
        visitor.leave_node(AstType::ScrollTimeline);
    }
}
impl<'a> Visit<'a> for ViewTimeline<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_view_timeline(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTimeline);
        let node = self;
        Visit::visit(&node.axis, visitor);
        Visit::visit((&node.inset).as_ref(), visitor);
        visitor.leave_node(AstType::ViewTimeline);
    }
}
impl<'a> Visit<'a> for AnimationRange<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_range(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationRange);
        let node = self;
        visitor.visit_animation_range_end((&node.end).as_ref());
        visitor.visit_animation_range_start((&node.start).as_ref());
        visitor.leave_node(AstType::AnimationRange);
    }
}
impl<'a> Visit<'a> for Animation<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Animation);
        let node = self;
        Visit::visit((&node.delay).as_ref(), visitor);
        Visit::visit(&node.direction, visitor);
        Visit::visit((&node.duration).as_ref(), visitor);
        Visit::visit(&node.fill_mode, visitor);
        Visit::visit((&node.iteration_count).as_ref(), visitor);
        Visit::visit((&node.name).as_ref(), visitor);
        Visit::visit(&node.play_state, visitor);
        Visit::visit((&node.timeline).as_ref(), visitor);
        Visit::visit((&node.timing_function).as_ref(), visitor);
        visitor.leave_node(AstType::Animation);
    }
}
impl<'a> Visit<'a> for SupportsRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_supports_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SupportsRule);
        let node = self;
        Visit::visit((&node.condition).as_ref(), visitor);
        Visit::visit(&node.span, visitor);
        for value_1 in (&node.rules).iter() {
            Visit::visit(value_1, visitor);
        }
        visitor.leave_node(AstType::SupportsRule);
    }
}
impl<'a> Visit<'a> for CounterStyleRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_counter_style_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CounterStyleRule);
        let node = self;
        Visit::visit((&node.declarations).as_ref().get_ref(), visitor);
        Visit::visit(&node.span, visitor);
        visitor.visit_str(&node.name);
        visitor.leave_node(AstType::CounterStyleRule);
    }
}
impl<'a> Visit<'a> for NamespaceRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_namespace_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NamespaceRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        if let Some(value_0) = (&node.prefix).as_ref() {
            visitor.visit_str(value_0);
        }
        visitor.visit_str(&node.url);
        visitor.leave_node(AstType::NamespaceRule);
    }
}
impl<'a> Visit<'a> for MozDocumentRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_moz_document_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MozDocumentRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.rules).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::MozDocumentRule);
    }
}
impl<'a> Visit<'a> for NestingRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_nesting_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NestingRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        Visit::visit((&node.style).as_ref(), visitor);
        visitor.leave_node(AstType::NestingRule);
    }
}
impl<'a> Visit<'a> for NestedDeclarationsRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_nested_declarations_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NestedDeclarationsRule);
        let node = self;
        Visit::visit((&node.declarations).as_ref().get_ref(), visitor);
        Visit::visit(&node.span, visitor);
        visitor.leave_node(AstType::NestedDeclarationsRule);
    }
}
impl<'a> Visit<'a> for ViewportRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_viewport_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewportRule);
        let node = self;
        Visit::visit((&node.declarations).as_ref().get_ref(), visitor);
        Visit::visit(&node.span, visitor);
        Visit::visit(&node.vendor_prefix, visitor);
        visitor.leave_node(AstType::ViewportRule);
    }
}
impl<'a> Visit<'a> for CustomMediaRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_custom_media_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CustomMediaRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        visitor.visit_str(&node.name);
        Visit::visit(&node.query, visitor);
        visitor.leave_node(AstType::CustomMediaRule);
    }
}
impl<'a> Visit<'a> for LayerStatementRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_layer_statement_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LayerStatementRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.names).iter() {
            for value_1 in (value_0).iter() {
                visitor.visit_str(value_1);
            }
        }
        visitor.leave_node(AstType::LayerStatementRule);
    }
}
impl<'a> Visit<'a> for LayerBlockRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_layer_block_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LayerBlockRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        if let Some(value_0) = (&node.name).as_ref() {
            for value_1 in (value_0).iter() {
                visitor.visit_str(value_1);
            }
        }
        for value_2 in (&node.rules).iter() {
            Visit::visit(value_2, visitor);
        }
        visitor.leave_node(AstType::LayerBlockRule);
    }
}
impl<'a> Visit<'a> for ScopeRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scope_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScopeRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.rules).iter() {
            Visit::visit(value_0, visitor);
        }
        if let Some(value_1) = (&node.scope_end).as_ref() {
            visitor.visit_selector_list((value_1).as_ref());
        }
        if let Some(value_3) = (&node.scope_start).as_ref() {
            visitor.visit_selector_list((value_3).as_ref());
        }
        visitor.leave_node(AstType::ScopeRule);
    }
}
impl<'a> Visit<'a> for StartingStyleRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_starting_style_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StartingStyleRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.rules).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::StartingStyleRule);
    }
}
impl<'a> Visit<'a> for PositionTryRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_position_try_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PositionTryRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        visitor.visit_str(&node.name);
        Visit::visit((&node.declarations).as_ref().get_ref(), visitor);
        visitor.leave_node(AstType::PositionTryRule);
    }
}
impl<'a> Visit<'a> for UnknownAtRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_unknown_at_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UnknownAtRule);
        let node = self;
        if let Some(value_0) = (&node.block).as_ref() {
            for value_1 in (value_0).iter() {
                Visit::visit(value_1, visitor);
            }
        }
        Visit::visit(&node.span, visitor);
        visitor.visit_str(&node.name);
        for value_2 in (&node.prelude).iter() {
            Visit::visit(value_2, visitor);
        }
        visitor.leave_node(AstType::UnknownAtRule);
    }
}
impl<'a> Visit<'a> for Position<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Position);
        let node = self;
        Visit::visit((&node.x).as_ref(), visitor);
        Visit::visit((&node.y).as_ref(), visitor);
        visitor.leave_node(AstType::Position);
    }
}
impl<'a> Visit<'a> for WebKitGradientPoint<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient_point(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitGradientPoint);
        let node = self;
        Visit::visit((&node.x).as_ref(), visitor);
        Visit::visit((&node.y).as_ref(), visitor);
        visitor.leave_node(AstType::WebKitGradientPoint);
    }
}
impl<'a> Visit<'a> for WebKitColorStop<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_color_stop(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitColorStop);
        let node = self;
        Visit::visit((&node.color).as_ref(), visitor);
        visitor.leave_node(AstType::WebKitColorStop);
    }
}
impl<'a> Visit<'a> for ImageSet<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_image_set(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ImageSet);
        let node = self;
        for value_0 in (&node.options).iter() {
            Visit::visit(value_0, visitor);
        }
        Visit::visit(&node.vendor_prefix, visitor);
        visitor.leave_node(AstType::ImageSet);
    }
}
impl<'a> Visit<'a> for ImageSetOption<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_image_set_option(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ImageSetOption);
        let node = self;
        if let Some(value_0) = (&node.file_type).as_ref() {
            visitor.visit_str(value_0);
        }
        Visit::visit((&node.image).as_ref(), visitor);
        Visit::visit((&node.resolution).as_ref(), visitor);
        visitor.leave_node(AstType::ImageSetOption);
    }
}
impl<'a> Visit<'a> for BackgroundPosition<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_background_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundPosition);
        let node = self;
        Visit::visit((&node.x).as_ref(), visitor);
        Visit::visit((&node.y).as_ref(), visitor);
        visitor.leave_node(AstType::BackgroundPosition);
    }
}
impl<'a> Visit<'a> for BackgroundRepeat {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_background_repeat(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundRepeat);
        let node = self;
        Visit::visit(&node.x, visitor);
        Visit::visit(&node.y, visitor);
        visitor.leave_node(AstType::BackgroundRepeat);
    }
}
impl<'a> Visit<'a> for Background<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_background(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Background);
        let node = self;
        Visit::visit(&node.attachment, visitor);
        Visit::visit(&node.clip, visitor);
        Visit::visit((&node.color).as_ref(), visitor);
        Visit::visit((&node.image).as_ref(), visitor);
        Visit::visit(&node.origin, visitor);
        Visit::visit((&node.position).as_ref(), visitor);
        Visit::visit((&node.repeat).as_ref(), visitor);
        Visit::visit((&node.size).as_ref(), visitor);
        visitor.leave_node(AstType::Background);
    }
}
impl<'a> Visit<'a> for BoxShadow<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_box_shadow(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxShadow);
        let node = self;
        Visit::visit((&node.blur).as_ref(), visitor);
        Visit::visit((&node.color).as_ref(), visitor);
        Visit::visit((&node.spread).as_ref(), visitor);
        Visit::visit((&node.x_offset).as_ref(), visitor);
        Visit::visit((&node.y_offset).as_ref(), visitor);
        visitor.leave_node(AstType::BoxShadow);
    }
}
impl<'a> Visit<'a> for BorderRadius<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_radius(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderRadius);
        let node = self;
        Visit::visit((&node.bottom_left).as_ref(), visitor);
        Visit::visit((&node.bottom_right).as_ref(), visitor);
        Visit::visit((&node.top_left).as_ref(), visitor);
        Visit::visit((&node.top_right).as_ref(), visitor);
        visitor.leave_node(AstType::BorderRadius);
    }
}
impl<'a> Visit<'a> for BorderImageRepeat {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image_repeat(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderImageRepeat);
        let node = self;
        Visit::visit(&node.horizontal, visitor);
        Visit::visit(&node.vertical, visitor);
        visitor.leave_node(AstType::BorderImageRepeat);
    }
}
impl<'a> Visit<'a> for BorderImageSlice<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image_slice(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderImageSlice);
        let node = self;
        Visit::visit((&node.offsets).as_ref(), visitor);
        visitor.leave_node(AstType::BorderImageSlice);
    }
}
impl<'a> Visit<'a> for BorderImage<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderImage);
        let node = self;
        Visit::visit((&node.outset).as_ref(), visitor);
        Visit::visit((&node.repeat).as_ref(), visitor);
        Visit::visit((&node.slice).as_ref(), visitor);
        Visit::visit((&node.source).as_ref(), visitor);
        Visit::visit((&node.width).as_ref(), visitor);
        visitor.leave_node(AstType::BorderImage);
    }
}
impl<'a> Visit<'a> for BorderColor<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderColor);
        let node = self;
        Visit::visit((&node.bottom).as_ref(), visitor);
        Visit::visit((&node.left).as_ref(), visitor);
        Visit::visit((&node.right).as_ref(), visitor);
        Visit::visit((&node.top).as_ref(), visitor);
        visitor.leave_node(AstType::BorderColor);
    }
}
impl<'a> Visit<'a> for BorderStyle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderStyle);
        let node = self;
        Visit::visit(&node.bottom, visitor);
        Visit::visit(&node.left, visitor);
        Visit::visit(&node.right, visitor);
        Visit::visit(&node.top, visitor);
        visitor.leave_node(AstType::BorderStyle);
    }
}
impl<'a> Visit<'a> for BorderWidth<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_width(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderWidth);
        let node = self;
        Visit::visit((&node.bottom).as_ref(), visitor);
        Visit::visit((&node.left).as_ref(), visitor);
        Visit::visit((&node.right).as_ref(), visitor);
        Visit::visit((&node.top).as_ref(), visitor);
        visitor.leave_node(AstType::BorderWidth);
    }
}
impl<'a> Visit<'a> for BorderBlockColor<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_block_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderBlockColor);
        let node = self;
        Visit::visit((&node.end).as_ref(), visitor);
        Visit::visit((&node.start).as_ref(), visitor);
        visitor.leave_node(AstType::BorderBlockColor);
    }
}
impl<'a> Visit<'a> for BorderBlockStyle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_block_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderBlockStyle);
        let node = self;
        Visit::visit(&node.end, visitor);
        Visit::visit(&node.start, visitor);
        visitor.leave_node(AstType::BorderBlockStyle);
    }
}
impl<'a> Visit<'a> for BorderBlockWidth<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_block_width(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderBlockWidth);
        let node = self;
        Visit::visit((&node.end).as_ref(), visitor);
        Visit::visit((&node.start).as_ref(), visitor);
        visitor.leave_node(AstType::BorderBlockWidth);
    }
}
impl<'a> Visit<'a> for BorderInlineColor<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderInlineColor);
        let node = self;
        Visit::visit((&node.end).as_ref(), visitor);
        Visit::visit((&node.start).as_ref(), visitor);
        visitor.leave_node(AstType::BorderInlineColor);
    }
}
impl<'a> Visit<'a> for BorderInlineStyle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderInlineStyle);
        let node = self;
        Visit::visit(&node.end, visitor);
        Visit::visit(&node.start, visitor);
        visitor.leave_node(AstType::BorderInlineStyle);
    }
}
impl<'a> Visit<'a> for BorderInlineWidth<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_width(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderInlineWidth);
        let node = self;
        Visit::visit((&node.end).as_ref(), visitor);
        Visit::visit((&node.start).as_ref(), visitor);
        visitor.leave_node(AstType::BorderInlineWidth);
    }
}
impl<'a, S> Visit<'a> for GenericBorder<'a, S>
where
    S: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_generic_border(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GenericBorder);
        let node = self;
        Visit::visit((&node.color).as_ref(), visitor);
        Visit::visit(&node.style, visitor);
        Visit::visit((&node.width).as_ref(), visitor);
        visitor.leave_node(AstType::GenericBorder);
    }
}
impl<'a> Visit<'a> for ContainerCondition<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_container_condition(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContainerCondition);
        let node = self;
        match node {
            ContainerCondition::Feature(field_0) => {
                visitor.visit_container_size_feature((field_0).as_ref());
            }
            ContainerCondition::Not(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ContainerCondition::Operation {
                conditions,
                operator,
            } => {
                for value_2 in (conditions).iter() {
                    Visit::visit(value_2, visitor);
                }
                Visit::visit(operator, visitor);
            }
            ContainerCondition::Style(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ContainerCondition::ScrollState(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ContainerCondition::Unknown(field_0) => {
                for value_5 in (field_0).iter() {
                    Visit::visit(value_5, visitor);
                }
            }
        }
        visitor.leave_node(AstType::ContainerCondition);
    }
}
impl<'a> Visit<'a> for ContainerSizeFeatureId {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_container_size_feature_id(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContainerSizeFeatureId);
        let node = self;
        match node {
            ContainerSizeFeatureId::Width => {}
            ContainerSizeFeatureId::Height => {}
            ContainerSizeFeatureId::InlineSize => {}
            ContainerSizeFeatureId::BlockSize => {}
            ContainerSizeFeatureId::AspectRatio => {}
            ContainerSizeFeatureId::Orientation => {}
        }
        visitor.leave_node(AstType::ContainerSizeFeatureId);
    }
}
impl<'a> Visit<'a> for StyleQuery<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_style_query(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StyleQuery);
        let node = self;
        match node {
            StyleQuery::Declaration(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            StyleQuery::Property(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            StyleQuery::Not(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            StyleQuery::Operation {
                conditions,
                operator,
            } => {
                for value_3 in (conditions).iter() {
                    Visit::visit(value_3, visitor);
                }
                Visit::visit(operator, visitor);
            }
        }
        visitor.leave_node(AstType::StyleQuery);
    }
}
impl<'a> Visit<'a> for ScrollStateQuery<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_state_query(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollStateQuery);
        let node = self;
        match node {
            ScrollStateQuery::Feature(field_0) => {
                visitor.visit_scroll_state_feature((field_0).as_ref());
            }
            ScrollStateQuery::Not(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ScrollStateQuery::Operation {
                conditions,
                operator,
            } => {
                for value_2 in (conditions).iter() {
                    Visit::visit(value_2, visitor);
                }
                Visit::visit(operator, visitor);
            }
        }
        visitor.leave_node(AstType::ScrollStateQuery);
    }
}
impl<'a> Visit<'a> for ScrollStateFeatureId {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_state_feature_id(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollStateFeatureId);
        let node = self;
        match node {
            ScrollStateFeatureId::Stuck => {}
            ScrollStateFeatureId::Snapped => {}
            ScrollStateFeatureId::Scrollable => {}
            ScrollStateFeatureId::Scrolled => {}
        }
        visitor.leave_node(AstType::ScrollStateFeatureId);
    }
}
impl<'a> Visit<'a> for Container<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_container(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Container);
        let node = self;
        Visit::visit(&node.container_type, visitor);
        Visit::visit((&node.name).as_ref(), visitor);
        visitor.leave_node(AstType::Container);
    }
}
impl<'a> Visit<'a> for ContainerRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_container_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContainerRule);
        let node = self;
        if let Some(value_0) = (&node.condition).as_ref() {
            Visit::visit((value_0).as_ref(), visitor);
        }
        Visit::visit(&node.span, visitor);
        if let Some(value_2) = (&node.name).as_ref() {
            visitor.visit_str(value_2);
        }
        for value_3 in (&node.rules).iter() {
            Visit::visit(value_3, visitor);
        }
        visitor.leave_node(AstType::ContainerRule);
    }
}
impl<'a> Visit<'a> for FontFaceProperty<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_face_property(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFaceProperty);
        let node = self;
        match node {
            FontFaceProperty::Source(field_0) => {
                for value_0 in (field_0).iter() {
                    Visit::visit(value_0, visitor);
                }
            }
            FontFaceProperty::FontFamily(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            FontFaceProperty::FontStyle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            FontFaceProperty::FontWeight(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            FontFaceProperty::FontStretch(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            FontFaceProperty::UnicodeRange(field_0) => {
                for value_5 in (field_0).iter() {
                    Visit::visit(value_5, visitor);
                }
            }
            FontFaceProperty::Custom(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::FontFaceProperty);
    }
}
impl<'a> Visit<'a> for Source<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_source(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Source);
        let node = self;
        match node {
            Source::Url(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Source::Local(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Source);
    }
}
impl<'a> Visit<'a> for FontFormat<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_format(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFormat);
        let node = self;
        match node {
            FontFormat::Woff => {}
            FontFormat::Woff2 => {}
            FontFormat::Truetype => {}
            FontFormat::Opentype => {}
            FontFormat::EmbeddedOpentype => {}
            FontFormat::Collection => {}
            FontFormat::Svg => {}
            FontFormat::String(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::FontFormat);
    }
}
impl<'a> Visit<'a> for FontTechnology {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_technology(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontTechnology);
        let node = self;
        match node {
            FontTechnology::FeaturesOpentype => {}
            FontTechnology::FeaturesAat => {}
            FontTechnology::FeaturesGraphite => {}
            FontTechnology::ColorColrv0 => {}
            FontTechnology::ColorColrv1 => {}
            FontTechnology::ColorSvg => {}
            FontTechnology::ColorSbix => {}
            FontTechnology::ColorCbdt => {}
            FontTechnology::Variations => {}
            FontTechnology::Palettes => {}
            FontTechnology::Incremental => {}
        }
        visitor.leave_node(AstType::FontTechnology);
    }
}
impl<'a> Visit<'a> for FontFaceStyle<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_face_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFaceStyle);
        let node = self;
        match node {
            FontFaceStyle::Normal => {}
            FontFaceStyle::Italic => {}
            FontFaceStyle::Oblique(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::FontFaceStyle);
    }
}
impl<'a> Visit<'a> for FontPaletteValuesProperty<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_palette_values_property(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontPaletteValuesProperty);
        let node = self;
        match node {
            FontPaletteValuesProperty::FontFamily(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            FontPaletteValuesProperty::BasePalette(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            FontPaletteValuesProperty::OverrideColors(field_0) => {
                for value_2 in (field_0).iter() {
                    Visit::visit(value_2, visitor);
                }
            }
            FontPaletteValuesProperty::Custom(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::FontPaletteValuesProperty);
    }
}
impl<'a> Visit<'a> for BasePalette {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_base_palette(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BasePalette);
        let node = self;
        match node {
            BasePalette::Light => {}
            BasePalette::Dark => {}
            BasePalette::Integer(field_0) => {}
        }
        visitor.leave_node(AstType::BasePalette);
    }
}
impl<'a> Visit<'a> for FontFeatureSubruleType {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_subrule_type(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFeatureSubruleType);
        let node = self;
        match node {
            FontFeatureSubruleType::Stylistic => {}
            FontFeatureSubruleType::HistoricalForms => {}
            FontFeatureSubruleType::Styleset => {}
            FontFeatureSubruleType::CharacterVariant => {}
            FontFeatureSubruleType::Swash => {}
            FontFeatureSubruleType::Ornaments => {}
            FontFeatureSubruleType::Annotation => {}
        }
        visitor.leave_node(AstType::FontFeatureSubruleType);
    }
}
impl<'a> Visit<'a> for Font<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Font);
        let node = self;
        for value_0 in (&node.family).iter() {
            Visit::visit(value_0, visitor);
        }
        Visit::visit((&node.line_height).as_ref(), visitor);
        Visit::visit((&node.size).as_ref(), visitor);
        Visit::visit((&node.stretch).as_ref(), visitor);
        Visit::visit((&node.style).as_ref(), visitor);
        Visit::visit(&node.variant_caps, visitor);
        Visit::visit((&node.weight).as_ref(), visitor);
        visitor.leave_node(AstType::Font);
    }
}
impl<'a> Visit<'a> for FontFaceRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_face_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFaceRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.properties).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::FontFaceRule);
    }
}
impl<'a> Visit<'a> for UrlSource<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_url_source(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UrlSource);
        let node = self;
        if let Some(value_0) = (&node.format).as_ref() {
            Visit::visit((value_0).as_ref(), visitor);
        }
        for value_2 in (&node.tech).iter() {
            Visit::visit(value_2, visitor);
        }
        Visit::visit((&node.url).as_ref(), visitor);
        visitor.leave_node(AstType::UrlSource);
    }
}
impl<'a> Visit<'a> for UnicodeRange {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_unicode_range(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UnicodeRange);
        let node = self;
        visitor.leave_node(AstType::UnicodeRange);
    }
}
impl<'a> Visit<'a> for FontPaletteValuesRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_palette_values_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontPaletteValuesRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        visitor.visit_str(&node.name);
        for value_0 in (&node.properties).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::FontPaletteValuesRule);
    }
}
impl<'a> Visit<'a> for OverrideColors<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_override_colors(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::OverrideColors);
        let node = self;
        Visit::visit((&node.color).as_ref(), visitor);
        visitor.leave_node(AstType::OverrideColors);
    }
}
impl<'a> Visit<'a> for FontFeatureValuesRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_values_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFeatureValuesRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.name).iter() {
            Visit::visit(value_0, visitor);
        }
        for value_1 in (&node.rules).iter() {
            Visit::visit(value_1, visitor);
        }
        visitor.leave_node(AstType::FontFeatureValuesRule);
    }
}
impl<'a> Visit<'a> for FontFeatureSubrule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_subrule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFeatureSubrule);
        let node = self;
        for value_0 in (&node.declarations).iter() {
            Visit::visit(value_0, visitor);
        }
        Visit::visit(&node.span, visitor);
        Visit::visit(&node.name, visitor);
        visitor.leave_node(AstType::FontFeatureSubrule);
    }
}
impl<'a> Visit<'a> for FontFeatureDeclaration<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_declaration(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFeatureDeclaration);
        let node = self;
        visitor.visit_str(&node.name);
        for value_0 in (&node.values).iter() {}
        visitor.leave_node(AstType::FontFeatureDeclaration);
    }
}
impl<'a> Visit<'a> for FamilyName<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_family_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FamilyName);
        let node = self;
        visitor.visit_str(&node.0);
        visitor.leave_node(AstType::FamilyName);
    }
}
impl<'a> Visit<'a> for KeyframeSelector<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_keyframe_selector(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::KeyframeSelector);
        let node = self;
        match node {
            KeyframeSelector::Percentage(field_0) => {}
            KeyframeSelector::From => {}
            KeyframeSelector::To => {}
            KeyframeSelector::TimelineRangePercentage(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::KeyframeSelector);
    }
}
impl<'a> Visit<'a> for KeyframesName<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_keyframes_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::KeyframesName);
        let node = self;
        match node {
            KeyframesName::Ident(field_0) => {
                visitor.visit_str(field_0);
            }
            KeyframesName::Custom(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::KeyframesName);
    }
}
impl<'a> Visit<'a> for KeyframesRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_keyframes_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::KeyframesRule);
        let node = self;
        for value_0 in (&node.keyframes).iter() {
            Visit::visit(value_0, visitor);
        }
        Visit::visit(&node.span, visitor);
        Visit::visit((&node.name).as_ref(), visitor);
        Visit::visit(&node.vendor_prefix, visitor);
        visitor.leave_node(AstType::KeyframesRule);
    }
}
impl<'a> Visit<'a> for Keyframe<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_keyframe(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Keyframe);
        let node = self;
        Visit::visit((&node.declarations).as_ref().get_ref(), visitor);
        for value_0 in (&node.selectors).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::Keyframe);
    }
}
impl<'a> Visit<'a> for TimelineRangePercentage {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_timeline_range_percentage(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TimelineRangePercentage);
        let node = self;
        Visit::visit(&node.name, visitor);
        visitor.leave_node(AstType::TimelineRangePercentage);
    }
}
impl<'a> Visit<'a> for AspectRatio<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_aspect_ratio(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AspectRatio);
        let node = self;
        if let Some(value_0) = (&node.ratio).as_ref() {
            Visit::visit((value_0).as_ref(), visitor);
        }
        visitor.leave_node(AstType::AspectRatio);
    }
}
impl<'a> Visit<'a> for Overflow {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_overflow(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Overflow);
        let node = self;
        Visit::visit(&node.x, visitor);
        Visit::visit(&node.y, visitor);
        visitor.leave_node(AstType::Overflow);
    }
}
impl<'a> Visit<'a> for InsetBlock<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_inset_block(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::InsetBlock);
        let node = self;
        Visit::visit((&node.block_end).as_ref(), visitor);
        Visit::visit((&node.block_start).as_ref(), visitor);
        visitor.leave_node(AstType::InsetBlock);
    }
}
impl<'a> Visit<'a> for InsetInline<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_inset_inline(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::InsetInline);
        let node = self;
        Visit::visit((&node.inline_end).as_ref(), visitor);
        Visit::visit((&node.inline_start).as_ref(), visitor);
        visitor.leave_node(AstType::InsetInline);
    }
}
impl<'a> Visit<'a> for Inset<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_inset(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Inset);
        let node = self;
        Visit::visit((&node.bottom).as_ref(), visitor);
        Visit::visit((&node.left).as_ref(), visitor);
        Visit::visit((&node.right).as_ref(), visitor);
        Visit::visit((&node.top).as_ref(), visitor);
        visitor.leave_node(AstType::Inset);
    }
}
impl<'a> Visit<'a> for FlexFlow {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_flow(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FlexFlow);
        let node = self;
        Visit::visit(&node.direction, visitor);
        Visit::visit(&node.wrap, visitor);
        visitor.leave_node(AstType::FlexFlow);
    }
}
impl<'a> Visit<'a> for Flex<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_flex(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Flex);
        let node = self;
        Visit::visit((&node.basis).as_ref(), visitor);
        visitor.leave_node(AstType::Flex);
    }
}
impl<'a> Visit<'a> for PlaceContent<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_place_content(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PlaceContent);
        let node = self;
        Visit::visit((&node.align).as_ref(), visitor);
        Visit::visit((&node.justify).as_ref(), visitor);
        visitor.leave_node(AstType::PlaceContent);
    }
}
impl<'a> Visit<'a> for PlaceSelf<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_place_self(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PlaceSelf);
        let node = self;
        Visit::visit((&node.align).as_ref(), visitor);
        Visit::visit((&node.justify).as_ref(), visitor);
        visitor.leave_node(AstType::PlaceSelf);
    }
}
impl<'a> Visit<'a> for PlaceItems<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_place_items(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PlaceItems);
        let node = self;
        Visit::visit((&node.align).as_ref(), visitor);
        Visit::visit((&node.justify).as_ref(), visitor);
        visitor.leave_node(AstType::PlaceItems);
    }
}
impl<'a> Visit<'a> for Gap<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_gap(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Gap);
        let node = self;
        Visit::visit((&node.column).as_ref(), visitor);
        Visit::visit((&node.row).as_ref(), visitor);
        visitor.leave_node(AstType::Gap);
    }
}
impl<'a> Visit<'a> for TrackRepeat<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_track_repeat(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TrackRepeat);
        let node = self;
        Visit::visit((&node.count).as_ref(), visitor);
        for value_1 in (&node.line_names).iter() {
            for value_2 in (value_1).iter() {
                visitor.visit_str(value_2);
            }
        }
        for value_3 in (&node.track_sizes).iter() {
            Visit::visit(value_3, visitor);
        }
        visitor.leave_node(AstType::TrackRepeat);
    }
}
impl<'a> Visit<'a> for GridAutoFlow {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_auto_flow(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridAutoFlow);
        let node = self;
        Visit::visit(&node.direction, visitor);
        visitor.leave_node(AstType::GridAutoFlow);
    }
}
impl<'a> Visit<'a> for GridTemplate<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_template(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridTemplate);
        let node = self;
        Visit::visit((&node.areas).as_ref(), visitor);
        Visit::visit((&node.columns).as_ref(), visitor);
        Visit::visit((&node.rows).as_ref(), visitor);
        visitor.leave_node(AstType::GridTemplate);
    }
}
impl<'a> Visit<'a> for Grid<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_grid(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Grid);
        let node = self;
        Visit::visit((&node.areas).as_ref(), visitor);
        for value_1 in (&node.auto_columns).iter() {
            Visit::visit(value_1, visitor);
        }
        Visit::visit((&node.auto_flow).as_ref(), visitor);
        for value_3 in (&node.auto_rows).iter() {
            Visit::visit(value_3, visitor);
        }
        Visit::visit((&node.columns).as_ref(), visitor);
        Visit::visit((&node.rows).as_ref(), visitor);
        visitor.leave_node(AstType::Grid);
    }
}
impl<'a> Visit<'a> for GridRow<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_row(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridRow);
        let node = self;
        Visit::visit((&node.end).as_ref(), visitor);
        Visit::visit((&node.start).as_ref(), visitor);
        visitor.leave_node(AstType::GridRow);
    }
}
impl<'a> Visit<'a> for GridColumn<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_column(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridColumn);
        let node = self;
        Visit::visit((&node.end).as_ref(), visitor);
        Visit::visit((&node.start).as_ref(), visitor);
        visitor.leave_node(AstType::GridColumn);
    }
}
impl<'a> Visit<'a> for GridArea<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_area(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridArea);
        let node = self;
        Visit::visit((&node.column_end).as_ref(), visitor);
        Visit::visit((&node.column_start).as_ref(), visitor);
        Visit::visit((&node.row_end).as_ref(), visitor);
        Visit::visit((&node.row_start).as_ref(), visitor);
        visitor.leave_node(AstType::GridArea);
    }
}
impl<'a> Visit<'a> for MarginBlock<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_margin_block(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MarginBlock);
        let node = self;
        Visit::visit((&node.block_end).as_ref(), visitor);
        Visit::visit((&node.block_start).as_ref(), visitor);
        visitor.leave_node(AstType::MarginBlock);
    }
}
impl<'a> Visit<'a> for MarginInline<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_margin_inline(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MarginInline);
        let node = self;
        Visit::visit((&node.inline_end).as_ref(), visitor);
        Visit::visit((&node.inline_start).as_ref(), visitor);
        visitor.leave_node(AstType::MarginInline);
    }
}
impl<'a> Visit<'a> for Margin<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_margin(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Margin);
        let node = self;
        Visit::visit((&node.bottom).as_ref(), visitor);
        Visit::visit((&node.left).as_ref(), visitor);
        Visit::visit((&node.right).as_ref(), visitor);
        Visit::visit((&node.top).as_ref(), visitor);
        visitor.leave_node(AstType::Margin);
    }
}
impl<'a> Visit<'a> for PaddingBlock<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_padding_block(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PaddingBlock);
        let node = self;
        Visit::visit((&node.block_end).as_ref(), visitor);
        Visit::visit((&node.block_start).as_ref(), visitor);
        visitor.leave_node(AstType::PaddingBlock);
    }
}
impl<'a> Visit<'a> for PaddingInline<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_padding_inline(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PaddingInline);
        let node = self;
        Visit::visit((&node.inline_end).as_ref(), visitor);
        Visit::visit((&node.inline_start).as_ref(), visitor);
        visitor.leave_node(AstType::PaddingInline);
    }
}
impl<'a> Visit<'a> for Padding<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_padding(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Padding);
        let node = self;
        Visit::visit((&node.bottom).as_ref(), visitor);
        Visit::visit((&node.left).as_ref(), visitor);
        Visit::visit((&node.right).as_ref(), visitor);
        Visit::visit((&node.top).as_ref(), visitor);
        visitor.leave_node(AstType::Padding);
    }
}
impl<'a> Visit<'a> for ScrollMarginBlock<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin_block(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollMarginBlock);
        let node = self;
        Visit::visit((&node.block_end).as_ref(), visitor);
        Visit::visit((&node.block_start).as_ref(), visitor);
        visitor.leave_node(AstType::ScrollMarginBlock);
    }
}
impl<'a> Visit<'a> for ScrollMarginInline<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin_inline(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollMarginInline);
        let node = self;
        Visit::visit((&node.inline_end).as_ref(), visitor);
        Visit::visit((&node.inline_start).as_ref(), visitor);
        visitor.leave_node(AstType::ScrollMarginInline);
    }
}
impl<'a> Visit<'a> for ScrollMargin<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollMargin);
        let node = self;
        Visit::visit((&node.bottom).as_ref(), visitor);
        Visit::visit((&node.left).as_ref(), visitor);
        Visit::visit((&node.right).as_ref(), visitor);
        Visit::visit((&node.top).as_ref(), visitor);
        visitor.leave_node(AstType::ScrollMargin);
    }
}
impl<'a> Visit<'a> for ScrollPaddingBlock<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding_block(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollPaddingBlock);
        let node = self;
        Visit::visit((&node.block_end).as_ref(), visitor);
        Visit::visit((&node.block_start).as_ref(), visitor);
        visitor.leave_node(AstType::ScrollPaddingBlock);
    }
}
impl<'a> Visit<'a> for ScrollPaddingInline<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding_inline(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollPaddingInline);
        let node = self;
        Visit::visit((&node.inline_end).as_ref(), visitor);
        Visit::visit((&node.inline_start).as_ref(), visitor);
        visitor.leave_node(AstType::ScrollPaddingInline);
    }
}
impl<'a> Visit<'a> for ScrollPadding<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollPadding);
        let node = self;
        Visit::visit((&node.bottom).as_ref(), visitor);
        Visit::visit((&node.left).as_ref(), visitor);
        Visit::visit((&node.right).as_ref(), visitor);
        Visit::visit((&node.top).as_ref(), visitor);
        visitor.leave_node(AstType::ScrollPadding);
    }
}
impl<'a> Visit<'a> for PageMarginBox {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_page_margin_box(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PageMarginBox);
        let node = self;
        match node {
            PageMarginBox::TopLeftCorner => {}
            PageMarginBox::TopLeft => {}
            PageMarginBox::TopCenter => {}
            PageMarginBox::TopRight => {}
            PageMarginBox::TopRightCorner => {}
            PageMarginBox::LeftTop => {}
            PageMarginBox::LeftMiddle => {}
            PageMarginBox::LeftBottom => {}
            PageMarginBox::RightTop => {}
            PageMarginBox::RightMiddle => {}
            PageMarginBox::RightBottom => {}
            PageMarginBox::BottomLeftCorner => {}
            PageMarginBox::BottomLeft => {}
            PageMarginBox::BottomCenter => {}
            PageMarginBox::BottomRight => {}
            PageMarginBox::BottomRightCorner => {}
        }
        visitor.leave_node(AstType::PageMarginBox);
    }
}
impl<'a> Visit<'a> for PagePseudoClass {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_page_pseudo_class(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PagePseudoClass);
        let node = self;
        match node {
            PagePseudoClass::Left => {}
            PagePseudoClass::Right => {}
            PagePseudoClass::First => {}
            PagePseudoClass::Last => {}
            PagePseudoClass::Blank => {}
        }
        visitor.leave_node(AstType::PagePseudoClass);
    }
}
impl<'a> Visit<'a> for PageRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_page_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PageRule);
        let node = self;
        Visit::visit((&node.declarations).as_ref().get_ref(), visitor);
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.rules).iter() {
            Visit::visit(value_0, visitor);
        }
        for value_1 in (&node.selectors).iter() {
            Visit::visit(value_1, visitor);
        }
        visitor.leave_node(AstType::PageRule);
    }
}
impl<'a> Visit<'a> for PageMarginRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_page_margin_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PageMarginRule);
        let node = self;
        Visit::visit((&node.declarations).as_ref().get_ref(), visitor);
        Visit::visit(&node.span, visitor);
        Visit::visit(&node.margin_box, visitor);
        visitor.leave_node(AstType::PageMarginRule);
    }
}
impl<'a> Visit<'a> for PageSelector<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_page_selector(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PageSelector);
        let node = self;
        if let Some(value_0) = (&node.name).as_ref() {
            visitor.visit_str(value_0);
        }
        for value_1 in (&node.pseudo_classes).iter() {
            Visit::visit(value_1, visitor);
        }
        visitor.leave_node(AstType::PageSelector);
    }
}
impl<'a> Visit<'a> for ParsedComponent<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_parsed_component(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ParsedComponent);
        let node = self;
        match node {
            ParsedComponent::Length(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ParsedComponent::Number(field_0) => {}
            ParsedComponent::Percentage(field_0) => {}
            ParsedComponent::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            ParsedComponent::String(field_0) => {
                visitor.visit_str(field_0);
            }
            ParsedComponent::Color(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ParsedComponent::Image(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ParsedComponent::Url(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ParsedComponent::Integer(field_0) => {}
            ParsedComponent::Angle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ParsedComponent::Time(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ParsedComponent::Resolution(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ParsedComponent::TransformFunction(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ParsedComponent::TransformList(field_0) => {
                for value_9 in (field_0).iter() {
                    Visit::visit(value_9, visitor);
                }
            }
            ParsedComponent::CustomIdent(field_0) => {
                visitor.visit_str(field_0);
            }
            ParsedComponent::Literal(field_0) => {
                visitor.visit_str(field_0);
            }
            ParsedComponent::Repeated {
                components,
                multiplier,
            } => {
                for value_10 in (components).iter() {
                    Visit::visit(value_10, visitor);
                }
                Visit::visit(multiplier, visitor);
            }
            ParsedComponent::TokenList(field_0) => {
                for value_11 in (field_0).iter() {
                    Visit::visit(value_11, visitor);
                }
            }
        }
        visitor.leave_node(AstType::ParsedComponent);
    }
}
impl<'a> Visit<'a> for Multiplier {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_multiplier(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Multiplier);
        let node = self;
        match node {
            Multiplier::None => {}
            Multiplier::Space => {}
            Multiplier::Comma => {}
        }
        visitor.leave_node(AstType::Multiplier);
    }
}
impl<'a> Visit<'a> for SyntaxString<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_syntax_string(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SyntaxString);
        let node = self;
        match node {
            SyntaxString::Components(field_0) => {
                for value_0 in (field_0).iter() {
                    Visit::visit(value_0, visitor);
                }
            }
            SyntaxString::Universal => {}
        }
        visitor.leave_node(AstType::SyntaxString);
    }
}
impl<'a> Visit<'a> for SyntaxComponentKind<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_syntax_component_kind(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SyntaxComponentKind);
        let node = self;
        match node {
            SyntaxComponentKind::Length => {}
            SyntaxComponentKind::Number => {}
            SyntaxComponentKind::Percentage => {}
            SyntaxComponentKind::LengthPercentage => {}
            SyntaxComponentKind::String => {}
            SyntaxComponentKind::Color => {}
            SyntaxComponentKind::Image => {}
            SyntaxComponentKind::Url => {}
            SyntaxComponentKind::Integer => {}
            SyntaxComponentKind::Angle => {}
            SyntaxComponentKind::Time => {}
            SyntaxComponentKind::Resolution => {}
            SyntaxComponentKind::TransformFunction => {}
            SyntaxComponentKind::TransformList => {}
            SyntaxComponentKind::CustomIdent => {}
            SyntaxComponentKind::Literal(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::SyntaxComponentKind);
    }
}
impl<'a> Visit<'a> for UnparsedProperty<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_unparsed_property(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UnparsedProperty);
        let node = self;
        Visit::visit((&node.property_id).as_ref(), visitor);
        for value_1 in (&node.value).iter() {
            Visit::visit(value_1, visitor);
        }
        visitor.leave_node(AstType::UnparsedProperty);
    }
}
impl<'a> Visit<'a> for CustomProperty<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_custom_property(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CustomProperty);
        let node = self;
        Visit::visit((&node.name).as_ref(), visitor);
        for value_1 in (&node.value).iter() {
            Visit::visit(value_1, visitor);
        }
        visitor.leave_node(AstType::CustomProperty);
    }
}
impl<'a> Visit<'a> for PropertyRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_property_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PropertyRule);
        let node = self;
        if let Some(value_0) = (&node.initial_value).as_ref() {
            Visit::visit((value_0).as_ref(), visitor);
        }
        Visit::visit(&node.span, visitor);
        visitor.visit_str(&node.name);
        Visit::visit((&node.syntax).as_ref(), visitor);
        visitor.leave_node(AstType::PropertyRule);
    }
}
impl<'a> Visit<'a> for SyntaxComponent<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_syntax_component(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SyntaxComponent);
        let node = self;
        Visit::visit((&node.kind).as_ref(), visitor);
        Visit::visit(&node.multiplier, visitor);
        visitor.leave_node(AstType::SyntaxComponent);
    }
}
impl<'a> Visit<'a> for InsetRect<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_inset_rect(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::InsetRect);
        let node = self;
        Visit::visit((&node.radius).as_ref(), visitor);
        Visit::visit((&node.rect).as_ref(), visitor);
        visitor.leave_node(AstType::InsetRect);
    }
}
impl<'a> Visit<'a> for CircleShape<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_circle_shape(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CircleShape);
        let node = self;
        Visit::visit((&node.position).as_ref(), visitor);
        Visit::visit((&node.radius).as_ref(), visitor);
        visitor.leave_node(AstType::CircleShape);
    }
}
impl<'a> Visit<'a> for EllipseShape<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_ellipse_shape(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::EllipseShape);
        let node = self;
        Visit::visit((&node.position).as_ref(), visitor);
        Visit::visit((&node.radius_x).as_ref(), visitor);
        Visit::visit((&node.radius_y).as_ref(), visitor);
        visitor.leave_node(AstType::EllipseShape);
    }
}
impl<'a> Visit<'a> for Polygon<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_polygon(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Polygon);
        let node = self;
        Visit::visit(&node.fill_rule, visitor);
        for value_0 in (&node.points).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::Polygon);
    }
}
impl<'a> Visit<'a> for Point<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_point(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Point);
        let node = self;
        visitor.visit_length_percentage((&node.x).as_ref());
        visitor.visit_length_percentage((&node.y).as_ref());
        visitor.leave_node(AstType::Point);
    }
}
impl<'a> Visit<'a> for Mask<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_mask(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Mask);
        let node = self;
        Visit::visit((&node.clip).as_ref(), visitor);
        Visit::visit(&node.composite, visitor);
        Visit::visit((&node.image).as_ref(), visitor);
        Visit::visit(&node.mode, visitor);
        Visit::visit(&node.origin, visitor);
        Visit::visit((&node.position).as_ref(), visitor);
        Visit::visit((&node.repeat).as_ref(), visitor);
        Visit::visit((&node.size).as_ref(), visitor);
        visitor.leave_node(AstType::Mask);
    }
}
impl<'a> Visit<'a> for MaskBorder<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_border(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MaskBorder);
        let node = self;
        Visit::visit(&node.mode, visitor);
        Visit::visit((&node.outset).as_ref(), visitor);
        Visit::visit((&node.repeat).as_ref(), visitor);
        Visit::visit((&node.slice).as_ref(), visitor);
        Visit::visit((&node.source).as_ref(), visitor);
        Visit::visit((&node.width).as_ref(), visitor);
        visitor.leave_node(AstType::MaskBorder);
    }
}
impl<'a> Visit<'a> for DropShadow<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_drop_shadow(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DropShadow);
        let node = self;
        Visit::visit((&node.blur).as_ref(), visitor);
        Visit::visit((&node.color).as_ref(), visitor);
        Visit::visit((&node.x_offset).as_ref(), visitor);
        Visit::visit((&node.y_offset).as_ref(), visitor);
        visitor.leave_node(AstType::DropShadow);
    }
}
impl<'a> Visit<'a> for DefaultAtRule {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_default_at_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DefaultAtRule);
        let node = self;
        visitor.leave_node(AstType::DefaultAtRule);
    }
}
impl<'a> Visit<'a> for StyleSheet<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_style_sheet(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StyleSheet);
        let node = self;
        for value_0 in (&node.license_comments).iter() {
            visitor.visit_str(value_0);
        }
        for value_1 in (&node.rules).iter() {
            Visit::visit(value_1, visitor);
        }
        for value_2 in (&node.source_map_urls).iter() {
            if let Some(value_3) = (value_2).as_ref() {
                visitor.visit_str(value_3);
            }
        }
        for value_4 in (&node.sources).iter() {
            visitor.visit_str(value_4);
        }
        visitor.leave_node(AstType::StyleSheet);
    }
}
impl<'a> Visit<'a> for MediaRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        Visit::visit(&node.query, visitor);
        for value_0 in (&node.rules).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::MediaRule);
    }
}
impl<'a> Visit<'a> for MediaList<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_list(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaList);
        let node = self;
        for value_0 in (&node.media_queries).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::MediaList);
    }
}
impl<'a> Visit<'a> for MediaQuery<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_query(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaQuery);
        let node = self;
        if let Some(value_0) = (&node.condition).as_ref() {
            Visit::visit((value_0).as_ref(), visitor);
        }
        Visit::visit(&node.media_type, visitor);
        if let Some(value_2) = (&node.qualifier).as_ref() {
            Visit::visit(value_2, visitor);
        }
        visitor.leave_node(AstType::MediaQuery);
    }
}
impl<'a> Visit<'a> for LengthValue {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_length_value(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LengthValue);
        let node = self;
        Visit::visit(&node.unit, visitor);
        visitor.leave_node(AstType::LengthValue);
    }
}
impl<'a> Visit<'a> for EnvironmentVariable<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_environment_variable(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::EnvironmentVariable);
        let node = self;
        if let Some(value_0) = (&node.fallback).as_ref() {
            for value_1 in (value_0).iter() {
                Visit::visit(value_1, visitor);
            }
        }
        for value_2 in (&node.indices).iter() {}
        Visit::visit((&node.name).as_ref(), visitor);
        visitor.leave_node(AstType::EnvironmentVariable);
    }
}
impl<'a> Visit<'a> for Url<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_url(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Url);
        let node = self;
        Visit::visit(&node.span, visitor);
        visitor.visit_str(&node.url);
        visitor.leave_node(AstType::Url);
    }
}
impl<'a> Visit<'a> for Variable<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_variable(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Variable);
        let node = self;
        if let Some(value_0) = (&node.fallback).as_ref() {
            for value_1 in (value_0).iter() {
                Visit::visit(value_1, visitor);
            }
        }
        Visit::visit((&node.name).as_ref(), visitor);
        visitor.leave_node(AstType::Variable);
    }
}
impl<'a> Visit<'a> for DashedIdentReference<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_dashed_ident_reference(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DashedIdentReference);
        let node = self;
        if let Some(value_0) = (&node.from).as_ref() {
            Visit::visit((value_0).as_ref(), visitor);
        }
        visitor.visit_str(&node.ident);
        visitor.leave_node(AstType::DashedIdentReference);
    }
}
impl<'a> Visit<'a> for Function<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_function(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Function);
        let node = self;
        for value_0 in (&node.arguments).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.visit_str(&node.name);
        if let Some(value_1) = (&node.replacement).as_ref() {
            Visit::visit(value_1, visitor);
        }
        visitor.leave_node(AstType::Function);
    }
}
impl<'a> Visit<'a> for FunctionReplacement {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_function_replacement(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FunctionReplacement);
        let node = self;
        match node {
            FunctionReplacement::GrayAlpha { alpha, lightness } => {}
            FunctionReplacement::Number(field_0) => {}
            FunctionReplacement::Dimension { unit, value } => {
                Visit::visit(unit, visitor);
            }
            FunctionReplacement::Percentage(field_0) => {}
            FunctionReplacement::Rgb { blue, green, red } => {}
            FunctionReplacement::Rgba {
                alpha,
                blue,
                green,
                red,
                use_hex,
            } => {}
        }
        visitor.leave_node(AstType::FunctionReplacement);
    }
}
impl<'a> Visit<'a> for ImportRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_import_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ImportRule);
        let node = self;
        if let Some(value_0) = (&node.layer).as_ref() {
            for value_1 in (value_0).iter() {
                visitor.visit_str(value_1);
            }
        }
        Visit::visit(&node.span, visitor);
        if let Some(value_2) = (&node.media).as_ref() {
            Visit::visit((value_2).as_ref(), visitor);
        }
        if let Some(value_4) = (&node.supports).as_ref() {
            Visit::visit((value_4).as_ref(), visitor);
        }
        visitor.visit_str(&node.url);
        visitor.leave_node(AstType::ImportRule);
    }
}
impl<'a> Visit<'a> for StyleRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_style_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StyleRule);
        let node = self;
        Visit::visit((&node.declarations).as_ref().get_ref(), visitor);
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.rules).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.visit_selector_list((&node.selectors).as_ref());
        Visit::visit(&node.vendor_prefix, visitor);
        visitor.leave_node(AstType::StyleRule);
    }
}
impl<'a> Visit<'a> for DeclarationBlock<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_declaration_block(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DeclarationBlock);
        let node = self;
        for value_0 in (&node.declarations).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::DeclarationBlock);
    }
}
impl<'a> Visit<'a> for TextTransform {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_transform(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextTransform);
        let node = self;
        Visit::visit(&node.case, visitor);
        visitor.leave_node(AstType::TextTransform);
    }
}
impl<'a> Visit<'a> for TextIndent<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_indent(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextIndent);
        let node = self;
        visitor.visit_length_percentage((&node.value).as_ref());
        visitor.leave_node(AstType::TextIndent);
    }
}
impl<'a> Visit<'a> for TextDecoration<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextDecoration);
        let node = self;
        Visit::visit((&node.color).as_ref(), visitor);
        Visit::visit((&node.line).as_ref(), visitor);
        Visit::visit(&node.style, visitor);
        Visit::visit((&node.thickness).as_ref(), visitor);
        visitor.leave_node(AstType::TextDecoration);
    }
}
impl<'a> Visit<'a> for TextEmphasis<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasis);
        let node = self;
        Visit::visit((&node.color).as_ref(), visitor);
        Visit::visit((&node.style).as_ref(), visitor);
        visitor.leave_node(AstType::TextEmphasis);
    }
}
impl<'a> Visit<'a> for TextEmphasisPosition {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasisPosition);
        let node = self;
        Visit::visit(&node.horizontal, visitor);
        Visit::visit(&node.vertical, visitor);
        visitor.leave_node(AstType::TextEmphasisPosition);
    }
}
impl<'a> Visit<'a> for TextShadow<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_shadow(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextShadow);
        let node = self;
        Visit::visit((&node.blur).as_ref(), visitor);
        Visit::visit((&node.color).as_ref(), visitor);
        Visit::visit((&node.spread).as_ref(), visitor);
        Visit::visit((&node.x_offset).as_ref(), visitor);
        Visit::visit((&node.y_offset).as_ref(), visitor);
        visitor.leave_node(AstType::TextShadow);
    }
}
impl<'a> Visit<'a> for MatrixForFloat {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_matrix_for_float(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MatrixForFloat);
        let node = self;
        visitor.leave_node(AstType::MatrixForFloat);
    }
}
impl<'a> Visit<'a> for Matrix3DForFloat {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_matrix_3_d_for_float(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Matrix3DForFloat);
        let node = self;
        visitor.leave_node(AstType::Matrix3DForFloat);
    }
}
impl<'a> Visit<'a> for Rotate<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_rotate(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Rotate);
        let node = self;
        Visit::visit((&node.angle).as_ref(), visitor);
        visitor.leave_node(AstType::Rotate);
    }
}
impl<'a> Visit<'a> for Cursor<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_cursor(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Cursor);
        let node = self;
        for value_0 in (&node.images).iter() {
            Visit::visit(value_0, visitor);
        }
        Visit::visit(&node.keyword, visitor);
        visitor.leave_node(AstType::Cursor);
    }
}
impl<'a> Visit<'a> for CursorImage<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_cursor_image(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CursorImage);
        let node = self;
        if let Some(value_0) = (&node.hotspot).as_ref() {}
        Visit::visit((&node.url).as_ref(), visitor);
        visitor.leave_node(AstType::CursorImage);
    }
}
impl<'a> Visit<'a> for Caret<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_caret(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Caret);
        let node = self;
        Visit::visit((&node.color).as_ref(), visitor);
        Visit::visit(&node.shape, visitor);
        visitor.leave_node(AstType::Caret);
    }
}
impl<'a> Visit<'a> for ListStyle<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_list_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ListStyle);
        let node = self;
        Visit::visit((&node.image).as_ref(), visitor);
        Visit::visit((&node.list_style_type).as_ref(), visitor);
        Visit::visit(&node.position, visitor);
        visitor.leave_node(AstType::ListStyle);
    }
}
impl<'a> Visit<'a> for Composes<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_composes(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Composes);
        let node = self;
        if let Some(value_0) = (&node.from).as_ref() {
            Visit::visit((value_0).as_ref(), visitor);
        }
        Visit::visit(&node.span, visitor);
        for value_2 in (&node.names).iter() {
            visitor.visit_str(value_2);
        }
        visitor.leave_node(AstType::Composes);
    }
}
impl<'a> Visit<'a> for ColorScheme {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_color_scheme(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ColorScheme);
        let node = self;
        visitor.leave_node(AstType::ColorScheme);
    }
}
impl<'a> Visit<'a> for ViewTransitionProperty<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_property(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionProperty);
        let node = self;
        match node {
            ViewTransitionProperty::Navigation(field_0) => {
                Visit::visit(field_0, visitor);
            }
            ViewTransitionProperty::Types(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ViewTransitionProperty::Custom(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::ViewTransitionProperty);
    }
}
impl<'a> Visit<'a> for Navigation {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_navigation(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Navigation);
        let node = self;
        match node {
            Navigation::None => {}
            Navigation::Auto => {}
        }
        visitor.leave_node(AstType::Navigation);
    }
}
impl<'a> Visit<'a> for ViewTransitionPartSelector<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_part_selector(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionPartSelector);
        let node = self;
        for value_0 in (&node.classes).iter() {
            visitor.visit_str(value_0);
        }
        if let Some(value_1) = (&node.name).as_ref() {
            Visit::visit((value_1).as_ref(), visitor);
        }
        visitor.leave_node(AstType::ViewTransitionPartSelector);
    }
}
impl<'a> Visit<'a> for ViewTransitionRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionRule);
        let node = self;
        Visit::visit(&node.span, visitor);
        for value_0 in (&node.properties).iter() {
            Visit::visit(value_0, visitor);
        }
        visitor.leave_node(AstType::ViewTransitionRule);
    }
}
