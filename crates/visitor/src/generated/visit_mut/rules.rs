#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitorMut};
use crate::AstType;
use rocketcss_ast::*;
use std::pin::Pin;
impl<'a> VisitMut<'a> for Transition<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_transition(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Transition);
        let node = self;
        VisitMut::visit_mut((&mut node.delay).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.duration).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.property).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.timing_function).as_mut(), visitor);
        visitor.leave_node(AstType::Transition);
    }
}
impl<'a> VisitMut<'a> for ScrollTimeline {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_timeline(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollTimeline);
        let node = self;
        VisitMut::visit_mut(&mut node.axis, visitor);
        VisitMut::visit_mut(&mut node.scroller, visitor);
        visitor.leave_node(AstType::ScrollTimeline);
    }
}
impl<'a> VisitMut<'a> for ViewTimeline<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_timeline(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTimeline);
        let node = self;
        VisitMut::visit_mut(&mut node.axis, visitor);
        VisitMut::visit_mut((&mut node.inset).as_mut(), visitor);
        visitor.leave_node(AstType::ViewTimeline);
    }
}
impl<'a> VisitMut<'a> for AnimationRange<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_range(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationRange);
        let node = self;
        visitor.visit_animation_range_end((&mut node.end).as_mut());
        visitor.visit_animation_range_start((&mut node.start).as_mut());
        visitor.leave_node(AstType::AnimationRange);
    }
}
impl<'a> VisitMut<'a> for Animation<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Animation);
        let node = self;
        VisitMut::visit_mut((&mut node.delay).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.direction, visitor);
        VisitMut::visit_mut((&mut node.duration).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.fill_mode, visitor);
        VisitMut::visit_mut((&mut node.iteration_count).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.name).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.play_state, visitor);
        VisitMut::visit_mut((&mut node.timeline).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.timing_function).as_mut(), visitor);
        visitor.leave_node(AstType::Animation);
    }
}
impl<'a> VisitMut<'a> for SupportsRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_supports_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SupportsRule);
        let node = self;
        VisitMut::visit_mut((&mut node.condition).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_1 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        visitor.leave_node(AstType::SupportsRule);
    }
}
impl<'a> VisitMut<'a> for CounterStyleRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_counter_style_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CounterStyleRule);
        let node = self;
        let mut value_0 = (&mut node.declarations).as_mut();
        VisitMut::visit_mut(&mut value_0, visitor);
        VisitMut::visit_mut(&mut node.span, visitor);
        visitor.visit_str(&mut node.name);
        visitor.leave_node(AstType::CounterStyleRule);
    }
}
impl<'a> VisitMut<'a> for NamespaceRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_namespace_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NamespaceRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        if let Some(value_0) = (&mut node.prefix).as_mut() {
            visitor.visit_str(value_0);
        }
        visitor.visit_str(&mut node.url);
        visitor.leave_node(AstType::NamespaceRule);
    }
}
impl<'a> VisitMut<'a> for MozDocumentRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_moz_document_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MozDocumentRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_0 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::MozDocumentRule);
    }
}
impl<'a> VisitMut<'a> for NestingRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_nesting_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NestingRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        VisitMut::visit_mut((&mut node.style).as_mut(), visitor);
        visitor.leave_node(AstType::NestingRule);
    }
}
impl<'a> VisitMut<'a> for NestedDeclarationsRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_nested_declarations_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NestedDeclarationsRule);
        let node = self;
        let mut value_0 = (&mut node.declarations).as_mut();
        VisitMut::visit_mut(&mut value_0, visitor);
        VisitMut::visit_mut(&mut node.span, visitor);
        visitor.leave_node(AstType::NestedDeclarationsRule);
    }
}
impl<'a> VisitMut<'a> for ViewportRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_viewport_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewportRule);
        let node = self;
        let mut value_0 = (&mut node.declarations).as_mut();
        VisitMut::visit_mut(&mut value_0, visitor);
        VisitMut::visit_mut(&mut node.span, visitor);
        VisitMut::visit_mut(&mut node.vendor_prefix, visitor);
        visitor.leave_node(AstType::ViewportRule);
    }
}
impl<'a> VisitMut<'a> for CustomMediaRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_custom_media_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CustomMediaRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        visitor.visit_str(&mut node.name);
        VisitMut::visit_mut(&mut node.query, visitor);
        visitor.leave_node(AstType::CustomMediaRule);
    }
}
impl<'a> VisitMut<'a> for LayerStatementRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_layer_statement_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LayerStatementRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_0 in (&mut node.names).iter_mut() {
            for value_1 in (value_0).iter_mut() {
                visitor.visit_str(value_1);
            }
        }
        visitor.leave_node(AstType::LayerStatementRule);
    }
}
impl<'a> VisitMut<'a> for LayerBlockRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_layer_block_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LayerBlockRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        if let Some(value_0) = (&mut node.name).as_mut() {
            for value_1 in (value_0).iter_mut() {
                visitor.visit_str(value_1);
            }
        }
        for value_2 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_2, visitor);
        }
        visitor.leave_node(AstType::LayerBlockRule);
    }
}
impl<'a> VisitMut<'a> for ScopeRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scope_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScopeRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_0 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        if let Some(value_1) = (&mut node.scope_end).as_mut() {
            visitor.visit_selector_list((value_1).as_mut());
        }
        if let Some(value_3) = (&mut node.scope_start).as_mut() {
            visitor.visit_selector_list((value_3).as_mut());
        }
        visitor.leave_node(AstType::ScopeRule);
    }
}
impl<'a> VisitMut<'a> for StartingStyleRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_starting_style_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StartingStyleRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_0 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::StartingStyleRule);
    }
}
impl<'a> VisitMut<'a> for PositionTryRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_position_try_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PositionTryRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        visitor.visit_str(&mut node.name);
        let mut value_0 = (&mut node.declarations).as_mut();
        VisitMut::visit_mut(&mut value_0, visitor);
        visitor.leave_node(AstType::PositionTryRule);
    }
}
impl<'a> VisitMut<'a> for UnknownAtRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unknown_at_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UnknownAtRule);
        let node = self;
        if let Some(value_0) = (&mut node.block).as_mut() {
            for value_1 in (value_0).iter_mut() {
                VisitMut::visit_mut(value_1, visitor);
            }
        }
        VisitMut::visit_mut(&mut node.span, visitor);
        visitor.visit_str(&mut node.name);
        for value_2 in (&mut node.prelude).iter_mut() {
            VisitMut::visit_mut(value_2, visitor);
        }
        visitor.leave_node(AstType::UnknownAtRule);
    }
}
impl<'a> VisitMut<'a> for Position<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_position(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Position);
        let node = self;
        VisitMut::visit_mut((&mut node.x).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.y).as_mut(), visitor);
        visitor.leave_node(AstType::Position);
    }
}
impl<'a> VisitMut<'a> for WebKitGradientPoint<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient_point(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitGradientPoint);
        let node = self;
        VisitMut::visit_mut((&mut node.x).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.y).as_mut(), visitor);
        visitor.leave_node(AstType::WebKitGradientPoint);
    }
}
impl<'a> VisitMut<'a> for WebKitColorStop<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_color_stop(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitColorStop);
        let node = self;
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        visitor.leave_node(AstType::WebKitColorStop);
    }
}
impl<'a> VisitMut<'a> for ImageSet<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_image_set(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ImageSet);
        let node = self;
        for value_0 in (&mut node.options).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        VisitMut::visit_mut(&mut node.vendor_prefix, visitor);
        visitor.leave_node(AstType::ImageSet);
    }
}
impl<'a> VisitMut<'a> for ImageSetOption<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_image_set_option(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ImageSetOption);
        let node = self;
        if let Some(value_0) = (&mut node.file_type).as_mut() {
            visitor.visit_str(value_0);
        }
        VisitMut::visit_mut((&mut node.image).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.resolution).as_mut(), visitor);
        visitor.leave_node(AstType::ImageSetOption);
    }
}
impl<'a> VisitMut<'a> for BackgroundPosition<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_position(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundPosition);
        let node = self;
        VisitMut::visit_mut((&mut node.x).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.y).as_mut(), visitor);
        visitor.leave_node(AstType::BackgroundPosition);
    }
}
impl<'a> VisitMut<'a> for BackgroundRepeat {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_repeat(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundRepeat);
        let node = self;
        VisitMut::visit_mut(&mut node.x, visitor);
        VisitMut::visit_mut(&mut node.y, visitor);
        visitor.leave_node(AstType::BackgroundRepeat);
    }
}
impl<'a> VisitMut<'a> for Background<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Background);
        let node = self;
        VisitMut::visit_mut(&mut node.attachment, visitor);
        VisitMut::visit_mut(&mut node.clip, visitor);
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.image).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.origin, visitor);
        VisitMut::visit_mut((&mut node.position).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.repeat).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.size).as_mut(), visitor);
        visitor.leave_node(AstType::Background);
    }
}
impl<'a> VisitMut<'a> for BoxShadow<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_shadow(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxShadow);
        let node = self;
        VisitMut::visit_mut((&mut node.blur).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.spread).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.x_offset).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.y_offset).as_mut(), visitor);
        visitor.leave_node(AstType::BoxShadow);
    }
}
impl<'a> VisitMut<'a> for BorderRadius<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_radius(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderRadius);
        let node = self;
        VisitMut::visit_mut((&mut node.bottom_left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.bottom_right).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top_left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top_right).as_mut(), visitor);
        visitor.leave_node(AstType::BorderRadius);
    }
}
impl<'a> VisitMut<'a> for BorderImageRepeat {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_image_repeat(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderImageRepeat);
        let node = self;
        VisitMut::visit_mut(&mut node.horizontal, visitor);
        VisitMut::visit_mut(&mut node.vertical, visitor);
        visitor.leave_node(AstType::BorderImageRepeat);
    }
}
impl<'a> VisitMut<'a> for BorderImageSlice<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_image_slice(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderImageSlice);
        let node = self;
        VisitMut::visit_mut((&mut node.offsets).as_mut(), visitor);
        visitor.leave_node(AstType::BorderImageSlice);
    }
}
impl<'a> VisitMut<'a> for BorderImage<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_image(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderImage);
        let node = self;
        VisitMut::visit_mut((&mut node.outset).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.repeat).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.slice).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.source).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.width).as_mut(), visitor);
        visitor.leave_node(AstType::BorderImage);
    }
}
impl<'a> VisitMut<'a> for BorderColor<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_color(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderColor);
        let node = self;
        VisitMut::visit_mut((&mut node.bottom).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.right).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top).as_mut(), visitor);
        visitor.leave_node(AstType::BorderColor);
    }
}
impl<'a> VisitMut<'a> for BorderStyle {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_style(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderStyle);
        let node = self;
        VisitMut::visit_mut(&mut node.bottom, visitor);
        VisitMut::visit_mut(&mut node.left, visitor);
        VisitMut::visit_mut(&mut node.right, visitor);
        VisitMut::visit_mut(&mut node.top, visitor);
        visitor.leave_node(AstType::BorderStyle);
    }
}
impl<'a> VisitMut<'a> for BorderWidth<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_width(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderWidth);
        let node = self;
        VisitMut::visit_mut((&mut node.bottom).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.right).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top).as_mut(), visitor);
        visitor.leave_node(AstType::BorderWidth);
    }
}
impl<'a> VisitMut<'a> for BorderBlockColor<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_block_color(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderBlockColor);
        let node = self;
        VisitMut::visit_mut((&mut node.end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.start).as_mut(), visitor);
        visitor.leave_node(AstType::BorderBlockColor);
    }
}
impl<'a> VisitMut<'a> for BorderBlockStyle {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_block_style(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderBlockStyle);
        let node = self;
        VisitMut::visit_mut(&mut node.end, visitor);
        VisitMut::visit_mut(&mut node.start, visitor);
        visitor.leave_node(AstType::BorderBlockStyle);
    }
}
impl<'a> VisitMut<'a> for BorderBlockWidth<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_block_width(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderBlockWidth);
        let node = self;
        VisitMut::visit_mut((&mut node.end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.start).as_mut(), visitor);
        visitor.leave_node(AstType::BorderBlockWidth);
    }
}
impl<'a> VisitMut<'a> for BorderInlineColor<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_color(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderInlineColor);
        let node = self;
        VisitMut::visit_mut((&mut node.end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.start).as_mut(), visitor);
        visitor.leave_node(AstType::BorderInlineColor);
    }
}
impl<'a> VisitMut<'a> for BorderInlineStyle {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_style(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderInlineStyle);
        let node = self;
        VisitMut::visit_mut(&mut node.end, visitor);
        VisitMut::visit_mut(&mut node.start, visitor);
        visitor.leave_node(AstType::BorderInlineStyle);
    }
}
impl<'a> VisitMut<'a> for BorderInlineWidth<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_width(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderInlineWidth);
        let node = self;
        VisitMut::visit_mut((&mut node.end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.start).as_mut(), visitor);
        visitor.leave_node(AstType::BorderInlineWidth);
    }
}
impl<'a, S> VisitMut<'a> for GenericBorder<'a, S>
where
    S: VisitMut<'a>,
{
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_generic_border(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GenericBorder);
        let node = self;
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.style, visitor);
        VisitMut::visit_mut((&mut node.width).as_mut(), visitor);
        visitor.leave_node(AstType::GenericBorder);
    }
}
impl<'a> VisitMut<'a> for ContainerCondition<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container_condition(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContainerCondition);
        let node = self;
        match node {
            ContainerCondition::Feature(field_0) => {
                visitor.visit_container_size_feature((field_0).as_mut());
            }
            ContainerCondition::Not(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ContainerCondition::Operation {
                conditions,
                operator,
            } => {
                for value_2 in (conditions).iter_mut() {
                    VisitMut::visit_mut(value_2, visitor);
                }
                VisitMut::visit_mut(operator, visitor);
            }
            ContainerCondition::Style(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ContainerCondition::ScrollState(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ContainerCondition::Unknown(field_0) => {
                for value_5 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_5, visitor);
                }
            }
        }
        visitor.leave_node(AstType::ContainerCondition);
    }
}
impl<'a> VisitMut<'a> for ContainerSizeFeatureId {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container_size_feature_id(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for StyleQuery<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_style_query(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StyleQuery);
        let node = self;
        match node {
            StyleQuery::Declaration(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            StyleQuery::Property(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            StyleQuery::Not(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            StyleQuery::Operation {
                conditions,
                operator,
            } => {
                for value_3 in (conditions).iter_mut() {
                    VisitMut::visit_mut(value_3, visitor);
                }
                VisitMut::visit_mut(operator, visitor);
            }
        }
        visitor.leave_node(AstType::StyleQuery);
    }
}
impl<'a> VisitMut<'a> for ScrollStateQuery<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_state_query(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollStateQuery);
        let node = self;
        match node {
            ScrollStateQuery::Feature(field_0) => {
                visitor.visit_scroll_state_feature((field_0).as_mut());
            }
            ScrollStateQuery::Not(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ScrollStateQuery::Operation {
                conditions,
                operator,
            } => {
                for value_2 in (conditions).iter_mut() {
                    VisitMut::visit_mut(value_2, visitor);
                }
                VisitMut::visit_mut(operator, visitor);
            }
        }
        visitor.leave_node(AstType::ScrollStateQuery);
    }
}
impl<'a> VisitMut<'a> for ScrollStateFeatureId {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_state_feature_id(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for Container<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Container);
        let node = self;
        VisitMut::visit_mut(&mut node.container_type, visitor);
        VisitMut::visit_mut((&mut node.name).as_mut(), visitor);
        visitor.leave_node(AstType::Container);
    }
}
impl<'a> VisitMut<'a> for ContainerRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContainerRule);
        let node = self;
        if let Some(value_0) = (&mut node.condition).as_mut() {
            VisitMut::visit_mut((value_0).as_mut(), visitor);
        }
        VisitMut::visit_mut(&mut node.span, visitor);
        if let Some(value_2) = (&mut node.name).as_mut() {
            visitor.visit_str(value_2);
        }
        for value_3 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_3, visitor);
        }
        visitor.leave_node(AstType::ContainerRule);
    }
}
impl<'a> VisitMut<'a> for FontFaceProperty<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_face_property(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFaceProperty);
        let node = self;
        match node {
            FontFaceProperty::Source(field_0) => {
                for value_0 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_0, visitor);
                }
            }
            FontFaceProperty::FontFamily(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            FontFaceProperty::FontStyle(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            FontFaceProperty::FontWeight(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            FontFaceProperty::FontStretch(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            FontFaceProperty::UnicodeRange(field_0) => {
                for value_5 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_5, visitor);
                }
            }
            FontFaceProperty::Custom(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
        }
        visitor.leave_node(AstType::FontFaceProperty);
    }
}
impl<'a> VisitMut<'a> for Source<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_source(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Source);
        let node = self;
        match node {
            Source::Url(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            Source::Local(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
        }
        visitor.leave_node(AstType::Source);
    }
}
impl<'a> VisitMut<'a> for FontFormat<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_format(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for FontTechnology {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_technology(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for FontFaceStyle<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_face_style(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFaceStyle);
        let node = self;
        match node {
            FontFaceStyle::Normal => {}
            FontFaceStyle::Italic => {}
            FontFaceStyle::Oblique(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
        }
        visitor.leave_node(AstType::FontFaceStyle);
    }
}
impl<'a> VisitMut<'a> for FontPaletteValuesProperty<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_palette_values_property(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontPaletteValuesProperty);
        let node = self;
        match node {
            FontPaletteValuesProperty::FontFamily(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            FontPaletteValuesProperty::BasePalette(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            FontPaletteValuesProperty::OverrideColors(field_0) => {
                for value_2 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_2, visitor);
                }
            }
            FontPaletteValuesProperty::Custom(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
        }
        visitor.leave_node(AstType::FontPaletteValuesProperty);
    }
}
impl<'a> VisitMut<'a> for BasePalette {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_base_palette(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for FontFeatureSubruleType {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_subrule_type(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for Font<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Font);
        let node = self;
        for value_0 in (&mut node.family).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        VisitMut::visit_mut((&mut node.line_height).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.size).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.stretch).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.style).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.variant_caps, visitor);
        VisitMut::visit_mut((&mut node.weight).as_mut(), visitor);
        visitor.leave_node(AstType::Font);
    }
}
impl<'a> VisitMut<'a> for FontFaceRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_face_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFaceRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_0 in (&mut node.properties).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::FontFaceRule);
    }
}
impl<'a> VisitMut<'a> for UrlSource<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_url_source(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UrlSource);
        let node = self;
        if let Some(value_0) = (&mut node.format).as_mut() {
            VisitMut::visit_mut((value_0).as_mut(), visitor);
        }
        for value_2 in (&mut node.tech).iter_mut() {
            VisitMut::visit_mut(value_2, visitor);
        }
        VisitMut::visit_mut((&mut node.url).as_mut(), visitor);
        visitor.leave_node(AstType::UrlSource);
    }
}
impl<'a> VisitMut<'a> for UnicodeRange {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unicode_range(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UnicodeRange);
        let node = self;
        visitor.leave_node(AstType::UnicodeRange);
    }
}
impl<'a> VisitMut<'a> for FontPaletteValuesRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_palette_values_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontPaletteValuesRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        visitor.visit_str(&mut node.name);
        for value_0 in (&mut node.properties).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::FontPaletteValuesRule);
    }
}
impl<'a> VisitMut<'a> for OverrideColors<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_override_colors(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::OverrideColors);
        let node = self;
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        visitor.leave_node(AstType::OverrideColors);
    }
}
impl<'a> VisitMut<'a> for FontFeatureValuesRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_values_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFeatureValuesRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_0 in (&mut node.name).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        for value_1 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        visitor.leave_node(AstType::FontFeatureValuesRule);
    }
}
impl<'a> VisitMut<'a> for FontFeatureSubrule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_subrule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFeatureSubrule);
        let node = self;
        for value_0 in (&mut node.declarations).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        VisitMut::visit_mut(&mut node.span, visitor);
        VisitMut::visit_mut(&mut node.name, visitor);
        visitor.leave_node(AstType::FontFeatureSubrule);
    }
}
impl<'a> VisitMut<'a> for FontFeatureDeclaration<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_declaration(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFeatureDeclaration);
        let node = self;
        visitor.visit_str(&mut node.name);
        for value_0 in (&mut node.values).iter_mut() {}
        visitor.leave_node(AstType::FontFeatureDeclaration);
    }
}
impl<'a> VisitMut<'a> for FamilyName<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_family_name(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FamilyName);
        let node = self;
        visitor.visit_str(&mut node.0);
        visitor.leave_node(AstType::FamilyName);
    }
}
impl<'a> VisitMut<'a> for KeyframeSelector<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_keyframe_selector(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::KeyframeSelector);
        let node = self;
        match node {
            KeyframeSelector::Percentage(field_0) => {}
            KeyframeSelector::From => {}
            KeyframeSelector::To => {}
            KeyframeSelector::TimelineRangePercentage(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
        }
        visitor.leave_node(AstType::KeyframeSelector);
    }
}
impl<'a> VisitMut<'a> for KeyframesName<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_keyframes_name(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for KeyframesRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_keyframes_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::KeyframesRule);
        let node = self;
        for value_0 in (&mut node.keyframes).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        VisitMut::visit_mut(&mut node.span, visitor);
        VisitMut::visit_mut((&mut node.name).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.vendor_prefix, visitor);
        visitor.leave_node(AstType::KeyframesRule);
    }
}
impl<'a> VisitMut<'a> for Keyframe<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_keyframe(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Keyframe);
        let node = self;
        let mut value_0 = (&mut node.declarations).as_mut();
        VisitMut::visit_mut(&mut value_0, visitor);
        for value_1 in (&mut node.selectors).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        visitor.leave_node(AstType::Keyframe);
    }
}
impl<'a> VisitMut<'a> for TimelineRangePercentage {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_timeline_range_percentage(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TimelineRangePercentage);
        let node = self;
        VisitMut::visit_mut(&mut node.name, visitor);
        visitor.leave_node(AstType::TimelineRangePercentage);
    }
}
impl<'a> VisitMut<'a> for AspectRatio<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_aspect_ratio(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AspectRatio);
        let node = self;
        if let Some(value_0) = (&mut node.ratio).as_mut() {
            VisitMut::visit_mut((value_0).as_mut(), visitor);
        }
        visitor.leave_node(AstType::AspectRatio);
    }
}
impl<'a> VisitMut<'a> for Overflow {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_overflow(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Overflow);
        let node = self;
        VisitMut::visit_mut(&mut node.x, visitor);
        VisitMut::visit_mut(&mut node.y, visitor);
        visitor.leave_node(AstType::Overflow);
    }
}
impl<'a> VisitMut<'a> for InsetBlock<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_inset_block(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::InsetBlock);
        let node = self;
        VisitMut::visit_mut((&mut node.block_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.block_start).as_mut(), visitor);
        visitor.leave_node(AstType::InsetBlock);
    }
}
impl<'a> VisitMut<'a> for InsetInline<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_inset_inline(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::InsetInline);
        let node = self;
        VisitMut::visit_mut((&mut node.inline_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.inline_start).as_mut(), visitor);
        visitor.leave_node(AstType::InsetInline);
    }
}
impl<'a> VisitMut<'a> for Inset<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_inset(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Inset);
        let node = self;
        VisitMut::visit_mut((&mut node.bottom).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.right).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top).as_mut(), visitor);
        visitor.leave_node(AstType::Inset);
    }
}
impl<'a> VisitMut<'a> for FlexFlow {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex_flow(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FlexFlow);
        let node = self;
        VisitMut::visit_mut(&mut node.direction, visitor);
        VisitMut::visit_mut(&mut node.wrap, visitor);
        visitor.leave_node(AstType::FlexFlow);
    }
}
impl<'a> VisitMut<'a> for Flex<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Flex);
        let node = self;
        VisitMut::visit_mut((&mut node.basis).as_mut(), visitor);
        visitor.leave_node(AstType::Flex);
    }
}
impl<'a> VisitMut<'a> for PlaceContent<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_place_content(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PlaceContent);
        let node = self;
        VisitMut::visit_mut((&mut node.align).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.justify).as_mut(), visitor);
        visitor.leave_node(AstType::PlaceContent);
    }
}
impl<'a> VisitMut<'a> for PlaceSelf<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_place_self(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PlaceSelf);
        let node = self;
        VisitMut::visit_mut((&mut node.align).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.justify).as_mut(), visitor);
        visitor.leave_node(AstType::PlaceSelf);
    }
}
impl<'a> VisitMut<'a> for PlaceItems<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_place_items(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PlaceItems);
        let node = self;
        VisitMut::visit_mut((&mut node.align).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.justify).as_mut(), visitor);
        visitor.leave_node(AstType::PlaceItems);
    }
}
impl<'a> VisitMut<'a> for Gap<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_gap(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Gap);
        let node = self;
        VisitMut::visit_mut((&mut node.column).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.row).as_mut(), visitor);
        visitor.leave_node(AstType::Gap);
    }
}
impl<'a> VisitMut<'a> for TrackRepeat<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_track_repeat(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TrackRepeat);
        let node = self;
        VisitMut::visit_mut((&mut node.count).as_mut(), visitor);
        for value_1 in (&mut node.line_names).iter_mut() {
            for value_2 in (value_1).iter_mut() {
                visitor.visit_str(value_2);
            }
        }
        for value_3 in (&mut node.track_sizes).iter_mut() {
            VisitMut::visit_mut(value_3, visitor);
        }
        visitor.leave_node(AstType::TrackRepeat);
    }
}
impl<'a> VisitMut<'a> for GridAutoFlow {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_auto_flow(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridAutoFlow);
        let node = self;
        VisitMut::visit_mut(&mut node.direction, visitor);
        visitor.leave_node(AstType::GridAutoFlow);
    }
}
impl<'a> VisitMut<'a> for GridTemplate<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_template(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridTemplate);
        let node = self;
        VisitMut::visit_mut((&mut node.areas).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.columns).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.rows).as_mut(), visitor);
        visitor.leave_node(AstType::GridTemplate);
    }
}
impl<'a> VisitMut<'a> for Grid<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Grid);
        let node = self;
        VisitMut::visit_mut((&mut node.areas).as_mut(), visitor);
        for value_1 in (&mut node.auto_columns).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        VisitMut::visit_mut((&mut node.auto_flow).as_mut(), visitor);
        for value_3 in (&mut node.auto_rows).iter_mut() {
            VisitMut::visit_mut(value_3, visitor);
        }
        VisitMut::visit_mut((&mut node.columns).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.rows).as_mut(), visitor);
        visitor.leave_node(AstType::Grid);
    }
}
impl<'a> VisitMut<'a> for GridRow<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_row(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridRow);
        let node = self;
        VisitMut::visit_mut((&mut node.end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.start).as_mut(), visitor);
        visitor.leave_node(AstType::GridRow);
    }
}
impl<'a> VisitMut<'a> for GridColumn<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_column(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridColumn);
        let node = self;
        VisitMut::visit_mut((&mut node.end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.start).as_mut(), visitor);
        visitor.leave_node(AstType::GridColumn);
    }
}
impl<'a> VisitMut<'a> for GridArea<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_area(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridArea);
        let node = self;
        VisitMut::visit_mut((&mut node.column_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.column_start).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.row_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.row_start).as_mut(), visitor);
        visitor.leave_node(AstType::GridArea);
    }
}
impl<'a> VisitMut<'a> for MarginBlock<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_margin_block(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MarginBlock);
        let node = self;
        VisitMut::visit_mut((&mut node.block_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.block_start).as_mut(), visitor);
        visitor.leave_node(AstType::MarginBlock);
    }
}
impl<'a> VisitMut<'a> for MarginInline<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_margin_inline(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MarginInline);
        let node = self;
        VisitMut::visit_mut((&mut node.inline_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.inline_start).as_mut(), visitor);
        visitor.leave_node(AstType::MarginInline);
    }
}
impl<'a> VisitMut<'a> for Margin<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_margin(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Margin);
        let node = self;
        VisitMut::visit_mut((&mut node.bottom).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.right).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top).as_mut(), visitor);
        visitor.leave_node(AstType::Margin);
    }
}
impl<'a> VisitMut<'a> for PaddingBlock<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_padding_block(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PaddingBlock);
        let node = self;
        VisitMut::visit_mut((&mut node.block_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.block_start).as_mut(), visitor);
        visitor.leave_node(AstType::PaddingBlock);
    }
}
impl<'a> VisitMut<'a> for PaddingInline<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_padding_inline(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PaddingInline);
        let node = self;
        VisitMut::visit_mut((&mut node.inline_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.inline_start).as_mut(), visitor);
        visitor.leave_node(AstType::PaddingInline);
    }
}
impl<'a> VisitMut<'a> for Padding<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_padding(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Padding);
        let node = self;
        VisitMut::visit_mut((&mut node.bottom).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.right).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top).as_mut(), visitor);
        visitor.leave_node(AstType::Padding);
    }
}
impl<'a> VisitMut<'a> for ScrollMarginBlock<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin_block(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollMarginBlock);
        let node = self;
        VisitMut::visit_mut((&mut node.block_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.block_start).as_mut(), visitor);
        visitor.leave_node(AstType::ScrollMarginBlock);
    }
}
impl<'a> VisitMut<'a> for ScrollMarginInline<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin_inline(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollMarginInline);
        let node = self;
        VisitMut::visit_mut((&mut node.inline_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.inline_start).as_mut(), visitor);
        visitor.leave_node(AstType::ScrollMarginInline);
    }
}
impl<'a> VisitMut<'a> for ScrollMargin<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollMargin);
        let node = self;
        VisitMut::visit_mut((&mut node.bottom).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.right).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top).as_mut(), visitor);
        visitor.leave_node(AstType::ScrollMargin);
    }
}
impl<'a> VisitMut<'a> for ScrollPaddingBlock<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding_block(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollPaddingBlock);
        let node = self;
        VisitMut::visit_mut((&mut node.block_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.block_start).as_mut(), visitor);
        visitor.leave_node(AstType::ScrollPaddingBlock);
    }
}
impl<'a> VisitMut<'a> for ScrollPaddingInline<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding_inline(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollPaddingInline);
        let node = self;
        VisitMut::visit_mut((&mut node.inline_end).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.inline_start).as_mut(), visitor);
        visitor.leave_node(AstType::ScrollPaddingInline);
    }
}
impl<'a> VisitMut<'a> for ScrollPadding<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollPadding);
        let node = self;
        VisitMut::visit_mut((&mut node.bottom).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.left).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.right).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.top).as_mut(), visitor);
        visitor.leave_node(AstType::ScrollPadding);
    }
}
impl<'a> VisitMut<'a> for PageMarginBox {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_margin_box(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for PagePseudoClass {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_pseudo_class(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for PageRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PageRule);
        let node = self;
        let mut value_0 = (&mut node.declarations).as_mut();
        VisitMut::visit_mut(&mut value_0, visitor);
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_1 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        for value_2 in (&mut node.selectors).iter_mut() {
            VisitMut::visit_mut(value_2, visitor);
        }
        visitor.leave_node(AstType::PageRule);
    }
}
impl<'a> VisitMut<'a> for PageMarginRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_margin_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PageMarginRule);
        let node = self;
        let mut value_0 = (&mut node.declarations).as_mut();
        VisitMut::visit_mut(&mut value_0, visitor);
        VisitMut::visit_mut(&mut node.span, visitor);
        VisitMut::visit_mut(&mut node.margin_box, visitor);
        visitor.leave_node(AstType::PageMarginRule);
    }
}
impl<'a> VisitMut<'a> for PageSelector<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_selector(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PageSelector);
        let node = self;
        if let Some(value_0) = (&mut node.name).as_mut() {
            visitor.visit_str(value_0);
        }
        for value_1 in (&mut node.pseudo_classes).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        visitor.leave_node(AstType::PageSelector);
    }
}
impl<'a> VisitMut<'a> for ParsedComponent<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_parsed_component(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ParsedComponent);
        let node = self;
        match node {
            ParsedComponent::Length(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ParsedComponent::Number(field_0) => {}
            ParsedComponent::Percentage(field_0) => {}
            ParsedComponent::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_mut());
            }
            ParsedComponent::String(field_0) => {
                visitor.visit_str(field_0);
            }
            ParsedComponent::Color(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ParsedComponent::Image(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ParsedComponent::Url(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ParsedComponent::Integer(field_0) => {}
            ParsedComponent::Angle(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ParsedComponent::Time(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ParsedComponent::Resolution(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ParsedComponent::TransformFunction(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ParsedComponent::TransformList(field_0) => {
                for value_9 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_9, visitor);
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
                for value_10 in (components).iter_mut() {
                    VisitMut::visit_mut(value_10, visitor);
                }
                VisitMut::visit_mut(multiplier, visitor);
            }
            ParsedComponent::TokenList(field_0) => {
                for value_11 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_11, visitor);
                }
            }
        }
        visitor.leave_node(AstType::ParsedComponent);
    }
}
impl<'a> VisitMut<'a> for Multiplier {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_multiplier(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for SyntaxString<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_syntax_string(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SyntaxString);
        let node = self;
        match node {
            SyntaxString::Components(field_0) => {
                for value_0 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_0, visitor);
                }
            }
            SyntaxString::Universal => {}
        }
        visitor.leave_node(AstType::SyntaxString);
    }
}
impl<'a> VisitMut<'a> for SyntaxComponentKind<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_syntax_component_kind(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
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
impl<'a> VisitMut<'a> for UnparsedProperty<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unparsed_property(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UnparsedProperty);
        let node = self;
        VisitMut::visit_mut((&mut node.property_id).as_mut(), visitor);
        for value_1 in (&mut node.value).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        visitor.leave_node(AstType::UnparsedProperty);
    }
}
impl<'a> VisitMut<'a> for CustomProperty<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_custom_property(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CustomProperty);
        let node = self;
        VisitMut::visit_mut((&mut node.name).as_mut(), visitor);
        for value_1 in (&mut node.value).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        visitor.leave_node(AstType::CustomProperty);
    }
}
impl<'a> VisitMut<'a> for PropertyRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_property_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PropertyRule);
        let node = self;
        if let Some(value_0) = (&mut node.initial_value).as_mut() {
            VisitMut::visit_mut((value_0).as_mut(), visitor);
        }
        VisitMut::visit_mut(&mut node.span, visitor);
        visitor.visit_str(&mut node.name);
        VisitMut::visit_mut((&mut node.syntax).as_mut(), visitor);
        visitor.leave_node(AstType::PropertyRule);
    }
}
impl<'a> VisitMut<'a> for SyntaxComponent<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_syntax_component(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SyntaxComponent);
        let node = self;
        VisitMut::visit_mut((&mut node.kind).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.multiplier, visitor);
        visitor.leave_node(AstType::SyntaxComponent);
    }
}
impl<'a> VisitMut<'a> for InsetRect<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_inset_rect(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::InsetRect);
        let node = self;
        VisitMut::visit_mut((&mut node.radius).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.rect).as_mut(), visitor);
        visitor.leave_node(AstType::InsetRect);
    }
}
impl<'a> VisitMut<'a> for CircleShape<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_circle_shape(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CircleShape);
        let node = self;
        VisitMut::visit_mut((&mut node.position).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.radius).as_mut(), visitor);
        visitor.leave_node(AstType::CircleShape);
    }
}
impl<'a> VisitMut<'a> for EllipseShape<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_ellipse_shape(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::EllipseShape);
        let node = self;
        VisitMut::visit_mut((&mut node.position).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.radius_x).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.radius_y).as_mut(), visitor);
        visitor.leave_node(AstType::EllipseShape);
    }
}
impl<'a> VisitMut<'a> for Polygon<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_polygon(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Polygon);
        let node = self;
        VisitMut::visit_mut(&mut node.fill_rule, visitor);
        for value_0 in (&mut node.points).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::Polygon);
    }
}
impl<'a> VisitMut<'a> for Point<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_point(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Point);
        let node = self;
        visitor.visit_length_percentage((&mut node.x).as_mut());
        visitor.visit_length_percentage((&mut node.y).as_mut());
        visitor.leave_node(AstType::Point);
    }
}
impl<'a> VisitMut<'a> for Mask<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Mask);
        let node = self;
        VisitMut::visit_mut((&mut node.clip).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.composite, visitor);
        VisitMut::visit_mut((&mut node.image).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.mode, visitor);
        VisitMut::visit_mut(&mut node.origin, visitor);
        VisitMut::visit_mut((&mut node.position).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.repeat).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.size).as_mut(), visitor);
        visitor.leave_node(AstType::Mask);
    }
}
impl<'a> VisitMut<'a> for MaskBorder<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask_border(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MaskBorder);
        let node = self;
        VisitMut::visit_mut(&mut node.mode, visitor);
        VisitMut::visit_mut((&mut node.outset).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.repeat).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.slice).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.source).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.width).as_mut(), visitor);
        visitor.leave_node(AstType::MaskBorder);
    }
}
impl<'a> VisitMut<'a> for DropShadow<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_drop_shadow(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DropShadow);
        let node = self;
        VisitMut::visit_mut((&mut node.blur).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.x_offset).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.y_offset).as_mut(), visitor);
        visitor.leave_node(AstType::DropShadow);
    }
}
impl<'a> VisitMut<'a> for DefaultAtRule {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_default_at_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DefaultAtRule);
        let node = self;
        visitor.leave_node(AstType::DefaultAtRule);
    }
}
impl<'a> VisitMut<'a> for StyleSheet<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_style_sheet(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StyleSheet);
        let node = self;
        for value_0 in (&mut node.license_comments).iter_mut() {
            visitor.visit_str(value_0);
        }
        for value_1 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        for value_2 in (&mut node.source_map_urls).iter_mut() {
            if let Some(value_3) = (value_2).as_mut() {
                visitor.visit_str(value_3);
            }
        }
        for value_4 in (&mut node.sources).iter_mut() {
            visitor.visit_str(value_4);
        }
        visitor.leave_node(AstType::StyleSheet);
    }
}
impl<'a> VisitMut<'a> for MediaRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        VisitMut::visit_mut(&mut node.query, visitor);
        for value_0 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::MediaRule);
    }
}
impl<'a> VisitMut<'a> for MediaList<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_list(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaList);
        let node = self;
        for value_0 in (&mut node.media_queries).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::MediaList);
    }
}
impl<'a> VisitMut<'a> for MediaQuery<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_query(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaQuery);
        let node = self;
        if let Some(value_0) = (&mut node.condition).as_mut() {
            VisitMut::visit_mut((value_0).as_mut(), visitor);
        }
        VisitMut::visit_mut(&mut node.media_type, visitor);
        if let Some(value_2) = (&mut node.qualifier).as_mut() {
            VisitMut::visit_mut(value_2, visitor);
        }
        visitor.leave_node(AstType::MediaQuery);
    }
}
impl<'a> VisitMut<'a> for LengthValue {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_length_value(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LengthValue);
        let node = self;
        VisitMut::visit_mut(&mut node.unit, visitor);
        visitor.leave_node(AstType::LengthValue);
    }
}
impl<'a> VisitMut<'a> for EnvironmentVariable<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_environment_variable(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::EnvironmentVariable);
        let node = self;
        if let Some(value_0) = (&mut node.fallback).as_mut() {
            for value_1 in (value_0).iter_mut() {
                VisitMut::visit_mut(value_1, visitor);
            }
        }
        for value_2 in (&mut node.indices).iter_mut() {}
        VisitMut::visit_mut((&mut node.name).as_mut(), visitor);
        visitor.leave_node(AstType::EnvironmentVariable);
    }
}
impl<'a> VisitMut<'a> for Url<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_url(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Url);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        visitor.visit_str(&mut node.url);
        visitor.leave_node(AstType::Url);
    }
}
impl<'a> VisitMut<'a> for Variable<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_variable(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Variable);
        let node = self;
        if let Some(value_0) = (&mut node.fallback).as_mut() {
            for value_1 in (value_0).iter_mut() {
                VisitMut::visit_mut(value_1, visitor);
            }
        }
        VisitMut::visit_mut((&mut node.name).as_mut(), visitor);
        visitor.leave_node(AstType::Variable);
    }
}
impl<'a> VisitMut<'a> for DashedIdentReference<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_dashed_ident_reference(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DashedIdentReference);
        let node = self;
        if let Some(value_0) = (&mut node.from).as_mut() {
            VisitMut::visit_mut((value_0).as_mut(), visitor);
        }
        visitor.visit_str(&mut node.ident);
        visitor.leave_node(AstType::DashedIdentReference);
    }
}
impl<'a> VisitMut<'a> for Function<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_function(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Function);
        let node = self;
        for value_0 in (&mut node.arguments).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.visit_str(&mut node.name);
        if let Some(value_1) = (&mut node.replacement).as_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        visitor.leave_node(AstType::Function);
    }
}
impl<'a> VisitMut<'a> for FunctionReplacement {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_function_replacement(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FunctionReplacement);
        let node = self;
        match node {
            FunctionReplacement::GrayAlpha { alpha, lightness } => {}
            FunctionReplacement::Number(field_0) => {}
            FunctionReplacement::Dimension { unit, value } => {
                VisitMut::visit_mut(unit, visitor);
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
impl<'a> VisitMut<'a> for ImportRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_import_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ImportRule);
        let node = self;
        if let Some(value_0) = (&mut node.layer).as_mut() {
            for value_1 in (value_0).iter_mut() {
                visitor.visit_str(value_1);
            }
        }
        VisitMut::visit_mut(&mut node.span, visitor);
        if let Some(value_2) = (&mut node.media).as_mut() {
            VisitMut::visit_mut((value_2).as_mut(), visitor);
        }
        if let Some(value_4) = (&mut node.supports).as_mut() {
            VisitMut::visit_mut((value_4).as_mut(), visitor);
        }
        visitor.visit_str(&mut node.url);
        visitor.leave_node(AstType::ImportRule);
    }
}
impl<'a> VisitMut<'a> for StyleRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_style_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StyleRule);
        let node = self;
        let mut value_0 = (&mut node.declarations).as_mut();
        VisitMut::visit_mut(&mut value_0, visitor);
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_1 in (&mut node.rules).iter_mut() {
            VisitMut::visit_mut(value_1, visitor);
        }
        visitor.visit_selector_list((&mut node.selectors).as_mut());
        VisitMut::visit_mut(&mut node.vendor_prefix, visitor);
        visitor.leave_node(AstType::StyleRule);
    }
}
impl<'a> VisitMut<'a> for Pin<&mut DeclarationBlock<'a>> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_declaration_block(self.as_mut());
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DeclarationBlock);
        let node = unsafe { self.as_mut().get_unchecked_mut() };
        for value_0 in (&mut node.declarations).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::DeclarationBlock);
    }
}
impl<'a> VisitMut<'a> for TextTransform {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_transform(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextTransform);
        let node = self;
        VisitMut::visit_mut(&mut node.case, visitor);
        visitor.leave_node(AstType::TextTransform);
    }
}
impl<'a> VisitMut<'a> for TextIndent<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_indent(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextIndent);
        let node = self;
        visitor.visit_length_percentage((&mut node.value).as_mut());
        visitor.leave_node(AstType::TextIndent);
    }
}
impl<'a> VisitMut<'a> for TextDecoration<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextDecoration);
        let node = self;
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.line).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.style, visitor);
        VisitMut::visit_mut((&mut node.thickness).as_mut(), visitor);
        visitor.leave_node(AstType::TextDecoration);
    }
}
impl<'a> VisitMut<'a> for TextEmphasis<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasis);
        let node = self;
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.style).as_mut(), visitor);
        visitor.leave_node(AstType::TextEmphasis);
    }
}
impl<'a> VisitMut<'a> for TextEmphasisPosition {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasisPosition);
        let node = self;
        VisitMut::visit_mut(&mut node.horizontal, visitor);
        VisitMut::visit_mut(&mut node.vertical, visitor);
        visitor.leave_node(AstType::TextEmphasisPosition);
    }
}
impl<'a> VisitMut<'a> for TextShadow<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_shadow(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextShadow);
        let node = self;
        VisitMut::visit_mut((&mut node.blur).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.spread).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.x_offset).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.y_offset).as_mut(), visitor);
        visitor.leave_node(AstType::TextShadow);
    }
}
impl<'a> VisitMut<'a> for MatrixForFloat {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_matrix_for_float(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MatrixForFloat);
        let node = self;
        visitor.leave_node(AstType::MatrixForFloat);
    }
}
impl<'a> VisitMut<'a> for Matrix3DForFloat {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_matrix_3_d_for_float(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Matrix3DForFloat);
        let node = self;
        visitor.leave_node(AstType::Matrix3DForFloat);
    }
}
impl<'a> VisitMut<'a> for Rotate<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_rotate(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Rotate);
        let node = self;
        VisitMut::visit_mut((&mut node.angle).as_mut(), visitor);
        visitor.leave_node(AstType::Rotate);
    }
}
impl<'a> VisitMut<'a> for Cursor<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_cursor(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Cursor);
        let node = self;
        for value_0 in (&mut node.images).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        VisitMut::visit_mut(&mut node.keyword, visitor);
        visitor.leave_node(AstType::Cursor);
    }
}
impl<'a> VisitMut<'a> for CursorImage<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_cursor_image(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CursorImage);
        let node = self;
        if let Some(value_0) = (&mut node.hotspot).as_mut() {}
        VisitMut::visit_mut((&mut node.url).as_mut(), visitor);
        visitor.leave_node(AstType::CursorImage);
    }
}
impl<'a> VisitMut<'a> for Caret<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_caret(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Caret);
        let node = self;
        VisitMut::visit_mut((&mut node.color).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.shape, visitor);
        visitor.leave_node(AstType::Caret);
    }
}
impl<'a> VisitMut<'a> for ListStyle<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_list_style(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ListStyle);
        let node = self;
        VisitMut::visit_mut((&mut node.image).as_mut(), visitor);
        VisitMut::visit_mut((&mut node.list_style_type).as_mut(), visitor);
        VisitMut::visit_mut(&mut node.position, visitor);
        visitor.leave_node(AstType::ListStyle);
    }
}
impl<'a> VisitMut<'a> for Composes<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_composes(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Composes);
        let node = self;
        if let Some(value_0) = (&mut node.from).as_mut() {
            VisitMut::visit_mut((value_0).as_mut(), visitor);
        }
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_2 in (&mut node.names).iter_mut() {
            visitor.visit_str(value_2);
        }
        visitor.leave_node(AstType::Composes);
    }
}
impl<'a> VisitMut<'a> for ColorScheme {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_color_scheme(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ColorScheme);
        let node = self;
        visitor.leave_node(AstType::ColorScheme);
    }
}
impl<'a> VisitMut<'a> for ViewTransitionProperty<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_property(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionProperty);
        let node = self;
        match node {
            ViewTransitionProperty::Navigation(field_0) => {
                VisitMut::visit_mut(field_0, visitor);
            }
            ViewTransitionProperty::Types(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            ViewTransitionProperty::Custom(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
        }
        visitor.leave_node(AstType::ViewTransitionProperty);
    }
}
impl<'a> VisitMut<'a> for Navigation {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_navigation(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Navigation);
        let node = self;
        match node {
            Navigation::None => {}
            Navigation::Auto => {}
        }
        visitor.leave_node(AstType::Navigation);
    }
}
impl<'a> VisitMut<'a> for ViewTransitionPartSelector<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_part_selector(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionPartSelector);
        let node = self;
        for value_0 in (&mut node.classes).iter_mut() {
            visitor.visit_str(value_0);
        }
        if let Some(value_1) = (&mut node.name).as_mut() {
            VisitMut::visit_mut((value_1).as_mut(), visitor);
        }
        visitor.leave_node(AstType::ViewTransitionPartSelector);
    }
}
impl<'a> VisitMut<'a> for ViewTransitionRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionRule);
        let node = self;
        VisitMut::visit_mut(&mut node.span, visitor);
        for value_0 in (&mut node.properties).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        visitor.leave_node(AstType::ViewTransitionRule);
    }
}
