#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for AlignContent {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_align_content(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AlignContent);
        let node = self;
        match node {
            AlignContent::Normal => {}
            AlignContent::BaselinePosition(field_0) => {
                Visit::visit(field_0, visitor);
            }
            AlignContent::ContentDistribution(field_0) => {
                Visit::visit(field_0, visitor);
            }
            AlignContent::ContentPosition { overflow, value } => {
                if let Some(value_0) = (overflow).as_ref() {
                    Visit::visit(value_0, visitor);
                }
                Visit::visit(value, visitor);
            }
        }
        visitor.leave_node(AstType::AlignContent);
    }
}
impl<'a> Visit<'a> for BaselinePosition {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_baseline_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BaselinePosition);
        let node = self;
        match node {
            BaselinePosition::First => {}
            BaselinePosition::Last => {}
        }
        visitor.leave_node(AstType::BaselinePosition);
    }
}
impl<'a> Visit<'a> for ContentDistribution {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_content_distribution(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContentDistribution);
        let node = self;
        match node {
            ContentDistribution::SpaceBetween => {}
            ContentDistribution::SpaceAround => {}
            ContentDistribution::SpaceEvenly => {}
            ContentDistribution::Stretch => {}
        }
        visitor.leave_node(AstType::ContentDistribution);
    }
}
impl<'a> Visit<'a> for OverflowPosition {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_overflow_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::OverflowPosition);
        let node = self;
        match node {
            OverflowPosition::Safe => {}
            OverflowPosition::Unsafe => {}
        }
        visitor.leave_node(AstType::OverflowPosition);
    }
}
impl<'a> Visit<'a> for ContentPosition {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_content_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContentPosition);
        let node = self;
        match node {
            ContentPosition::Center => {}
            ContentPosition::Start => {}
            ContentPosition::End => {}
            ContentPosition::FlexStart => {}
            ContentPosition::FlexEnd => {}
        }
        visitor.leave_node(AstType::ContentPosition);
    }
}
impl<'a> Visit<'a> for JustifyContent {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_justify_content(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::JustifyContent);
        let node = self;
        match node {
            JustifyContent::Normal => {}
            JustifyContent::ContentDistribution(field_0) => {
                Visit::visit(field_0, visitor);
            }
            JustifyContent::ContentPosition { overflow, value } => {
                if let Some(value_0) = (overflow).as_ref() {
                    Visit::visit(value_0, visitor);
                }
                Visit::visit(value, visitor);
            }
            JustifyContent::Left { overflow } => {
                if let Some(value_1) = (overflow).as_ref() {
                    Visit::visit(value_1, visitor);
                }
            }
            JustifyContent::Right { overflow } => {
                if let Some(value_2) = (overflow).as_ref() {
                    Visit::visit(value_2, visitor);
                }
            }
        }
        visitor.leave_node(AstType::JustifyContent);
    }
}
impl<'a> Visit<'a> for AlignSelf {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_align_self(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AlignSelf);
        let node = self;
        match node {
            AlignSelf::Auto => {}
            AlignSelf::Normal => {}
            AlignSelf::Stretch => {}
            AlignSelf::BaselinePosition(field_0) => {
                Visit::visit(field_0, visitor);
            }
            AlignSelf::SelfPosition { overflow, value } => {
                if let Some(value_0) = (overflow).as_ref() {
                    Visit::visit(value_0, visitor);
                }
                Visit::visit(value, visitor);
            }
        }
        visitor.leave_node(AstType::AlignSelf);
    }
}
impl<'a> Visit<'a> for SelfPosition {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_self_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SelfPosition);
        let node = self;
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
}
impl<'a> Visit<'a> for JustifySelf {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_justify_self(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::JustifySelf);
        let node = self;
        match node {
            JustifySelf::Auto => {}
            JustifySelf::Normal => {}
            JustifySelf::Stretch => {}
            JustifySelf::BaselinePosition(field_0) => {
                Visit::visit(field_0, visitor);
            }
            JustifySelf::SelfPosition { overflow, value } => {
                if let Some(value_0) = (overflow).as_ref() {
                    Visit::visit(value_0, visitor);
                }
                Visit::visit(value, visitor);
            }
            JustifySelf::Left { overflow } => {
                if let Some(value_1) = (overflow).as_ref() {
                    Visit::visit(value_1, visitor);
                }
            }
            JustifySelf::Right { overflow } => {
                if let Some(value_2) = (overflow).as_ref() {
                    Visit::visit(value_2, visitor);
                }
            }
        }
        visitor.leave_node(AstType::JustifySelf);
    }
}
impl<'a> Visit<'a> for AlignItems {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_align_items(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AlignItems);
        let node = self;
        match node {
            AlignItems::Normal => {}
            AlignItems::Stretch => {}
            AlignItems::BaselinePosition(field_0) => {
                Visit::visit(field_0, visitor);
            }
            AlignItems::SelfPosition { overflow, value } => {
                if let Some(value_0) = (overflow).as_ref() {
                    Visit::visit(value_0, visitor);
                }
                Visit::visit(value, visitor);
            }
        }
        visitor.leave_node(AstType::AlignItems);
    }
}
impl<'a> Visit<'a> for JustifyItems {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_justify_items(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::JustifyItems);
        let node = self;
        match node {
            JustifyItems::Normal => {}
            JustifyItems::Stretch => {}
            JustifyItems::BaselinePosition(field_0) => {
                Visit::visit(field_0, visitor);
            }
            JustifyItems::SelfPosition { overflow, value } => {
                if let Some(value_0) = (overflow).as_ref() {
                    Visit::visit(value_0, visitor);
                }
                Visit::visit(value, visitor);
            }
            JustifyItems::Left { overflow } => {
                if let Some(value_1) = (overflow).as_ref() {
                    Visit::visit(value_1, visitor);
                }
            }
            JustifyItems::Right { overflow } => {
                if let Some(value_2) = (overflow).as_ref() {
                    Visit::visit(value_2, visitor);
                }
            }
            JustifyItems::Legacy(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::JustifyItems);
    }
}
impl<'a> Visit<'a> for LegacyJustify {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_legacy_justify(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LegacyJustify);
        let node = self;
        match node {
            LegacyJustify::Left => {}
            LegacyJustify::Right => {}
            LegacyJustify::Center => {}
        }
        visitor.leave_node(AstType::LegacyJustify);
    }
}
impl<'a> Visit<'a> for GapValue<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_gap_value(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GapValue);
        let node = self;
        match node {
            GapValue::Normal => {}
            GapValue::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
        }
        visitor.leave_node(AstType::GapValue);
    }
}
impl<'a> Visit<'a> for EasingFunction {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_easing_function(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::EasingFunction);
        let node = self;
        match node {
            EasingFunction::Linear => {}
            EasingFunction::Ease => {}
            EasingFunction::EaseIn => {}
            EasingFunction::EaseOut => {}
            EasingFunction::EaseInOut => {}
            EasingFunction::CubicBezier { x1, x2, y1, y2 } => {}
            EasingFunction::Steps { count, position } => {
                Visit::visit(position, visitor);
            }
        }
        visitor.leave_node(AstType::EasingFunction);
    }
}
impl<'a> Visit<'a> for StepPosition {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_step_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StepPosition);
        let node = self;
        match node {
            StepPosition::Start => {}
            StepPosition::End => {}
            StepPosition::JumpNone => {}
            StepPosition::JumpBoth => {}
        }
        visitor.leave_node(AstType::StepPosition);
    }
}
impl<'a> Visit<'a> for AnimationIterationCount {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_iteration_count(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationIterationCount);
        let node = self;
        match node {
            AnimationIterationCount::Number(field_0) => {}
            AnimationIterationCount::Infinite => {}
        }
        visitor.leave_node(AstType::AnimationIterationCount);
    }
}
impl<'a> Visit<'a> for AnimationDirection {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_direction(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationDirection);
        let node = self;
        match node {
            AnimationDirection::Normal => {}
            AnimationDirection::Reverse => {}
            AnimationDirection::Alternate => {}
            AnimationDirection::AlternateReverse => {}
        }
        visitor.leave_node(AstType::AnimationDirection);
    }
}
impl<'a> Visit<'a> for AnimationPlayState {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_play_state(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationPlayState);
        let node = self;
        match node {
            AnimationPlayState::Running => {}
            AnimationPlayState::Paused => {}
        }
        visitor.leave_node(AstType::AnimationPlayState);
    }
}
impl<'a> Visit<'a> for AnimationFillMode {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_fill_mode(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationFillMode);
        let node = self;
        match node {
            AnimationFillMode::None => {}
            AnimationFillMode::Forwards => {}
            AnimationFillMode::Backwards => {}
            AnimationFillMode::Both => {}
        }
        visitor.leave_node(AstType::AnimationFillMode);
    }
}
impl<'a> Visit<'a> for AnimationComposition {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_composition(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationComposition);
        let node = self;
        match node {
            AnimationComposition::Replace => {}
            AnimationComposition::Add => {}
            AnimationComposition::Accumulate => {}
        }
        visitor.leave_node(AstType::AnimationComposition);
    }
}
impl<'a> Visit<'a> for AnimationTimeline<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_timeline(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationTimeline);
        let node = self;
        match node {
            AnimationTimeline::Auto => {}
            AnimationTimeline::None => {}
            AnimationTimeline::DashedIdent(field_0) => {
                visitor.visit_str(field_0);
            }
            AnimationTimeline::Scroll(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            AnimationTimeline::View(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::AnimationTimeline);
    }
}
impl<'a> Visit<'a> for ScrollAxis {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_axis(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ScrollAxis);
        let node = self;
        match node {
            ScrollAxis::Block => {}
            ScrollAxis::Inline => {}
            ScrollAxis::X => {}
            ScrollAxis::Y => {}
        }
        visitor.leave_node(AstType::ScrollAxis);
    }
}
impl<'a> Visit<'a> for Scroller {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scroller(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Scroller);
        let node = self;
        match node {
            Scroller::Root => {}
            Scroller::Nearest => {}
            Scroller::Self_ => {}
        }
        visitor.leave_node(AstType::Scroller);
    }
}
impl<'a> Visit<'a> for AnimationAttachmentRange<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_attachment_range(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AnimationAttachmentRange);
        let node = self;
        match node {
            AnimationAttachmentRange::Normal => {}
            AnimationAttachmentRange::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            AnimationAttachmentRange::TimelineRange { name, offset } => {
                Visit::visit(name, visitor);
                visitor.visit_length_percentage((offset).as_ref());
            }
        }
        visitor.leave_node(AstType::AnimationAttachmentRange);
    }
}
impl<'a> Visit<'a> for TimelineRangeName {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_timeline_range_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TimelineRangeName);
        let node = self;
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
}
impl<'a> Visit<'a> for LineStyle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_line_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LineStyle);
        let node = self;
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
}
impl<'a> Visit<'a> for BorderSideWidth<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_side_width(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderSideWidth);
        let node = self;
        match node {
            BorderSideWidth::Thin => {}
            BorderSideWidth::Medium => {}
            BorderSideWidth::Thick => {}
            BorderSideWidth::Length(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::BorderSideWidth);
    }
}
impl<'a> Visit<'a> for LengthOrNumber<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_length_or_number(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LengthOrNumber);
        let node = self;
        match node {
            LengthOrNumber::Number(field_0) => {}
            LengthOrNumber::Length(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::LengthOrNumber);
    }
}
impl<'a> Visit<'a> for BorderImageRepeatKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image_repeat_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderImageRepeatKeyword);
        let node = self;
        match node {
            BorderImageRepeatKeyword::Stretch => {}
            BorderImageRepeatKeyword::Repeat => {}
            BorderImageRepeatKeyword::Round => {}
            BorderImageRepeatKeyword::Space => {}
        }
        visitor.leave_node(AstType::BorderImageRepeatKeyword);
    }
}
impl<'a> Visit<'a> for BorderImageSideWidth<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image_side_width(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BorderImageSideWidth);
        let node = self;
        match node {
            BorderImageSideWidth::Number(field_0) => {}
            BorderImageSideWidth::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            BorderImageSideWidth::Auto => {}
        }
        visitor.leave_node(AstType::BorderImageSideWidth);
    }
}
impl<'a> Visit<'a> for OutlineStyle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_outline_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::OutlineStyle);
        let node = self;
        match node {
            OutlineStyle::Auto => {}
            OutlineStyle::LineStyle(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::OutlineStyle);
    }
}
impl<'a> Visit<'a> for Display<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_display(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Display);
        let node = self;
        match node {
            Display::Keyword(field_0) => {
                Visit::visit(field_0, visitor);
            }
            Display::Pair {
                inside,
                is_list_item,
                outside,
            } => {
                Visit::visit((inside).as_ref(), visitor);
                Visit::visit(outside, visitor);
            }
        }
        visitor.leave_node(AstType::Display);
    }
}
impl<'a> Visit<'a> for DisplayKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_display_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DisplayKeyword);
        let node = self;
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
}
impl<'a> Visit<'a> for DisplayInside {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_display_inside(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DisplayInside);
        let node = self;
        match node {
            DisplayInside::Flow => {}
            DisplayInside::FlowRoot => {}
            DisplayInside::Table => {}
            DisplayInside::Flex { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            DisplayInside::Box { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            DisplayInside::Grid => {}
            DisplayInside::Ruby => {}
        }
        visitor.leave_node(AstType::DisplayInside);
    }
}
impl<'a> Visit<'a> for DisplayOutside {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_display_outside(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DisplayOutside);
        let node = self;
        match node {
            DisplayOutside::Block => {}
            DisplayOutside::Inline => {}
            DisplayOutside::RunIn => {}
        }
        visitor.leave_node(AstType::DisplayOutside);
    }
}
impl<'a> Visit<'a> for Visibility {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_visibility(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Visibility);
        let node = self;
        match node {
            Visibility::Visible => {}
            Visibility::Hidden => {}
            Visibility::Collapse => {}
        }
        visitor.leave_node(AstType::Visibility);
    }
}
impl<'a> Visit<'a> for Size<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_size(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Size);
        let node = self;
        match node {
            Size::Auto => {}
            Size::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            Size::MinContent { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            Size::MaxContent { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            Size::FitContent { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            Size::FitContentFunction(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            Size::Stretch { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            Size::Contain => {}
        }
        visitor.leave_node(AstType::Size);
    }
}
impl<'a> Visit<'a> for MaxSize<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_max_size(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MaxSize);
        let node = self;
        match node {
            MaxSize::None => {}
            MaxSize::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            MaxSize::MinContent { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            MaxSize::MaxContent { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            MaxSize::FitContent { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            MaxSize::FitContentFunction(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            MaxSize::Stretch { vendor_prefix } => {
                Visit::visit(vendor_prefix, visitor);
            }
            MaxSize::Contain => {}
        }
        visitor.leave_node(AstType::MaxSize);
    }
}
impl<'a> Visit<'a> for BoxSizing {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_box_sizing(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxSizing);
        let node = self;
        match node {
            BoxSizing::ContentBox => {}
            BoxSizing::BorderBox => {}
        }
        visitor.leave_node(AstType::BoxSizing);
    }
}
impl<'a> Visit<'a> for OverflowKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_overflow_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::OverflowKeyword);
        let node = self;
        match node {
            OverflowKeyword::Visible => {}
            OverflowKeyword::Hidden => {}
            OverflowKeyword::Clip => {}
            OverflowKeyword::Scroll => {}
            OverflowKeyword::Auto => {}
        }
        visitor.leave_node(AstType::OverflowKeyword);
    }
}
impl<'a> Visit<'a> for TextOverflow {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_overflow(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextOverflow);
        let node = self;
        match node {
            TextOverflow::Clip => {}
            TextOverflow::Ellipsis => {}
        }
        visitor.leave_node(AstType::TextOverflow);
    }
}
impl<'a> Visit<'a> for PositionProperty {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_position_property(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PositionProperty);
        let node = self;
        match node {
            PositionProperty::Static => {}
            PositionProperty::Relative => {}
            PositionProperty::Absolute => {}
            PositionProperty::Sticky(field_0) => {
                Visit::visit(field_0, visitor);
            }
            PositionProperty::Fixed => {}
        }
        visitor.leave_node(AstType::PositionProperty);
    }
}
impl<'a, T> Visit<'a> for Size2D<'a, T>
where
    T: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_size_2_d(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Size2D);
        let node = self;
        Visit::visit((&node.0).as_ref(), visitor);
        Visit::visit((&node.1).as_ref(), visitor);
        visitor.leave_node(AstType::Size2D);
    }
}
impl<'a, T> Visit<'a> for Rect<'a, T>
where
    T: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_rect(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Rect);
        let node = self;
        Visit::visit((&node.0).as_ref(), visitor);
        Visit::visit((&node.1).as_ref(), visitor);
        Visit::visit((&node.2).as_ref(), visitor);
        Visit::visit((&node.3).as_ref(), visitor);
        visitor.leave_node(AstType::Rect);
    }
}
impl<'a> Visit<'a> for BoxDecorationBreak {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_box_decoration_break(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxDecorationBreak);
        let node = self;
        match node {
            BoxDecorationBreak::Slice => {}
            BoxDecorationBreak::Clone => {}
        }
        visitor.leave_node(AstType::BoxDecorationBreak);
    }
}
impl<'a> Visit<'a> for ZIndex {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_z_index(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ZIndex);
        let node = self;
        match node {
            ZIndex::Auto => {}
            ZIndex::Integer(field_0) => {}
        }
        visitor.leave_node(AstType::ZIndex);
    }
}
impl<'a> Visit<'a> for ContainerType {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_container_type(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContainerType);
        let node = self;
        match node {
            ContainerType::Normal => {}
            ContainerType::InlineSize => {}
            ContainerType::Size => {}
            ContainerType::ScrollState => {}
        }
        visitor.leave_node(AstType::ContainerType);
    }
}
impl<'a> Visit<'a> for ContainerNameList<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_container_name_list(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ContainerNameList);
        let node = self;
        match node {
            ContainerNameList::None => {}
            ContainerNameList::Names(field_0) => {
                for value_0 in (field_0).iter() {
                    visitor.visit_str(value_0);
                }
            }
        }
        visitor.leave_node(AstType::ContainerNameList);
    }
}
impl<'a> Visit<'a> for FilterList<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_filter_list(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FilterList);
        let node = self;
        match node {
            FilterList::None => {}
            FilterList::Filters(field_0) => {
                for value_0 in (field_0).iter() {
                    Visit::visit(value_0, visitor);
                }
            }
        }
        visitor.leave_node(AstType::FilterList);
    }
}
impl<'a> Visit<'a> for Filter<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_filter(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Filter);
        let node = self;
        match node {
            Filter::Blur(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::Brightness(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::Contrast(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::Grayscale(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::HueRotate(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::Invert(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::Opacity(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::Saturate(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::Sepia(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::DropShadow(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Filter::Url(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Filter);
    }
}
impl<'a> Visit<'a> for FlexDirection {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_direction(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FlexDirection);
        let node = self;
        match node {
            FlexDirection::Row => {}
            FlexDirection::RowReverse => {}
            FlexDirection::Column => {}
            FlexDirection::ColumnReverse => {}
        }
        visitor.leave_node(AstType::FlexDirection);
    }
}
impl<'a> Visit<'a> for FlexWrap {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_wrap(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FlexWrap);
        let node = self;
        match node {
            FlexWrap::Nowrap => {}
            FlexWrap::Wrap => {}
            FlexWrap::WrapReverse => {}
        }
        visitor.leave_node(AstType::FlexWrap);
    }
}
impl<'a> Visit<'a> for BoxOrient {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_box_orient(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxOrient);
        let node = self;
        match node {
            BoxOrient::Horizontal => {}
            BoxOrient::Vertical => {}
            BoxOrient::InlineAxis => {}
            BoxOrient::BlockAxis => {}
        }
        visitor.leave_node(AstType::BoxOrient);
    }
}
impl<'a> Visit<'a> for BoxDirection {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_box_direction(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxDirection);
        let node = self;
        match node {
            BoxDirection::Normal => {}
            BoxDirection::Reverse => {}
        }
        visitor.leave_node(AstType::BoxDirection);
    }
}
impl<'a> Visit<'a> for BoxAlign {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_box_align(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxAlign);
        let node = self;
        match node {
            BoxAlign::Start => {}
            BoxAlign::End => {}
            BoxAlign::Center => {}
            BoxAlign::Baseline => {}
            BoxAlign::Stretch => {}
        }
        visitor.leave_node(AstType::BoxAlign);
    }
}
impl<'a> Visit<'a> for BoxPack {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_box_pack(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxPack);
        let node = self;
        match node {
            BoxPack::Start => {}
            BoxPack::End => {}
            BoxPack::Center => {}
            BoxPack::Justify => {}
        }
        visitor.leave_node(AstType::BoxPack);
    }
}
impl<'a> Visit<'a> for BoxLines {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_box_lines(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BoxLines);
        let node = self;
        match node {
            BoxLines::Single => {}
            BoxLines::Multiple => {}
        }
        visitor.leave_node(AstType::BoxLines);
    }
}
impl<'a> Visit<'a> for FlexPack {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_pack(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FlexPack);
        let node = self;
        match node {
            FlexPack::Start => {}
            FlexPack::End => {}
            FlexPack::Center => {}
            FlexPack::Justify => {}
            FlexPack::Distribute => {}
        }
        visitor.leave_node(AstType::FlexPack);
    }
}
impl<'a> Visit<'a> for FlexItemAlign {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_item_align(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FlexItemAlign);
        let node = self;
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
}
impl<'a> Visit<'a> for FlexLinePack {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_line_pack(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FlexLinePack);
        let node = self;
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
}
impl<'a> Visit<'a> for FontWeight<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_weight(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontWeight);
        let node = self;
        match node {
            FontWeight::Absolute(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            FontWeight::Bolder => {}
            FontWeight::Lighter => {}
        }
        visitor.leave_node(AstType::FontWeight);
    }
}
impl<'a> Visit<'a> for AbsoluteFontWeight {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_absolute_font_weight(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AbsoluteFontWeight);
        let node = self;
        match node {
            AbsoluteFontWeight::Weight(field_0) => {}
            AbsoluteFontWeight::Normal => {}
            AbsoluteFontWeight::Bold => {}
        }
        visitor.leave_node(AstType::AbsoluteFontWeight);
    }
}
impl<'a> Visit<'a> for FontSize<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_size(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontSize);
        let node = self;
        match node {
            FontSize::Length(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            FontSize::Absolute(field_0) => {
                Visit::visit(field_0, visitor);
            }
            FontSize::Relative(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::FontSize);
    }
}
impl<'a> Visit<'a> for AbsoluteFontSize {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_absolute_font_size(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AbsoluteFontSize);
        let node = self;
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
}
impl<'a> Visit<'a> for RelativeFontSize {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_relative_font_size(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::RelativeFontSize);
        let node = self;
        match node {
            RelativeFontSize::Smaller => {}
            RelativeFontSize::Larger => {}
        }
        visitor.leave_node(AstType::RelativeFontSize);
    }
}
impl<'a> Visit<'a> for FontStretch {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_stretch(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontStretch);
        let node = self;
        match node {
            FontStretch::Keyword(field_0) => {
                Visit::visit(field_0, visitor);
            }
            FontStretch::Percentage(field_0) => {}
        }
        visitor.leave_node(AstType::FontStretch);
    }
}
impl<'a> Visit<'a> for FontStretchKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_stretch_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontStretchKeyword);
        let node = self;
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
}
impl<'a> Visit<'a> for FontFamily<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_family(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontFamily);
        let node = self;
        match node {
            FontFamily::Generic(field_0) => {
                Visit::visit(field_0, visitor);
            }
            FontFamily::FamilyName(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::FontFamily);
    }
}
impl<'a> Visit<'a> for GenericFontFamily {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_generic_font_family(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GenericFontFamily);
        let node = self;
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
}
impl<'a> Visit<'a> for FontStyle<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontStyle);
        let node = self;
        match node {
            FontStyle::Normal => {}
            FontStyle::Italic => {}
            FontStyle::Oblique(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::FontStyle);
    }
}
impl<'a> Visit<'a> for FontVariantCaps {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_font_variant_caps(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FontVariantCaps);
        let node = self;
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
}
impl<'a> Visit<'a> for LineHeight<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_line_height(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LineHeight);
        let node = self;
        match node {
            LineHeight::Normal => {}
            LineHeight::Number(field_0) => {}
            LineHeight::Length(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
        }
        visitor.leave_node(AstType::LineHeight);
    }
}
impl<'a> Visit<'a> for VerticalAlign<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_vertical_align(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::VerticalAlign);
        let node = self;
        match node {
            VerticalAlign::Keyword(field_0) => {
                Visit::visit(field_0, visitor);
            }
            VerticalAlign::Length(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
        }
        visitor.leave_node(AstType::VerticalAlign);
    }
}
impl<'a> Visit<'a> for VerticalAlignKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_vertical_align_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::VerticalAlignKeyword);
        let node = self;
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
}
impl<'a> Visit<'a> for TrackSizing<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_track_sizing(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TrackSizing);
        let node = self;
        match node {
            TrackSizing::None => {}
            TrackSizing::TrackList { items, line_names } => {
                for value_0 in (items).iter() {
                    Visit::visit(value_0, visitor);
                }
                for value_1 in (line_names).iter() {
                    for value_2 in (value_1).iter() {
                        visitor.visit_str(value_2);
                    }
                }
            }
        }
        visitor.leave_node(AstType::TrackSizing);
    }
}
impl<'a> Visit<'a> for TrackListItem<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_track_list_item(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TrackListItem);
        let node = self;
        match node {
            TrackListItem::TrackSize(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TrackListItem::TrackRepeat(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::TrackListItem);
    }
}
impl<'a> Visit<'a> for TrackSize<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_track_size(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TrackSize);
        let node = self;
        match node {
            TrackSize::TrackBreadth(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            TrackSize::MinMax { max, min } => {
                Visit::visit((max).as_ref(), visitor);
                Visit::visit((min).as_ref(), visitor);
            }
            TrackSize::FitContent(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
        }
        visitor.leave_node(AstType::TrackSize);
    }
}
impl<'a> Visit<'a> for TrackBreadth<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_track_breadth(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TrackBreadth);
        let node = self;
        match node {
            TrackBreadth::Length(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            TrackBreadth::Flex(field_0) => {}
            TrackBreadth::MinContent => {}
            TrackBreadth::MaxContent => {}
            TrackBreadth::Auto => {}
        }
        visitor.leave_node(AstType::TrackBreadth);
    }
}
impl<'a> Visit<'a> for RepeatCount {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_repeat_count(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::RepeatCount);
        let node = self;
        match node {
            RepeatCount::Number(field_0) => {}
            RepeatCount::AutoFill => {}
            RepeatCount::AutoFit => {}
        }
        visitor.leave_node(AstType::RepeatCount);
    }
}
impl<'a> Visit<'a> for AutoFlowDirection {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_auto_flow_direction(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AutoFlowDirection);
        let node = self;
        match node {
            AutoFlowDirection::Row => {}
            AutoFlowDirection::Column => {}
        }
        visitor.leave_node(AstType::AutoFlowDirection);
    }
}
impl<'a> Visit<'a> for GridTemplateAreas<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_template_areas(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridTemplateAreas);
        let node = self;
        match node {
            GridTemplateAreas::None => {}
            GridTemplateAreas::Areas { areas, columns } => {
                for value_0 in (areas).iter() {
                    if let Some(value_1) = (value_0).as_ref() {
                        visitor.visit_str(value_1);
                    }
                }
            }
        }
        visitor.leave_node(AstType::GridTemplateAreas);
    }
}
impl<'a> Visit<'a> for GridLine<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_line(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GridLine);
        let node = self;
        match node {
            GridLine::Auto => {}
            GridLine::Area { name } => {
                visitor.visit_str(name);
            }
            GridLine::Line { index, name } => {
                if let Some(value_0) = (name).as_ref() {
                    visitor.visit_str(value_0);
                }
            }
            GridLine::Span { index, name } => {
                if let Some(value_1) = (name).as_ref() {
                    visitor.visit_str(value_1);
                }
            }
        }
        visitor.leave_node(AstType::GridLine);
    }
}
impl<'a> Visit<'a> for Image<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_image(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Image);
        let node = self;
        match node {
            Image::None => {}
            Image::Url(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Image::Gradient(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Image::ImageSet(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Image);
    }
}
impl<'a> Visit<'a> for Gradient<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_gradient(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Gradient);
        let node = self;
        match node {
            Gradient::Linear {
                direction,
                items,
                vendor_prefix,
            } => {
                Visit::visit((direction).as_ref(), visitor);
                for value_1 in (items).iter() {
                    Visit::visit(value_1, visitor);
                }
                Visit::visit(vendor_prefix, visitor);
            }
            Gradient::RepeatingLinear {
                direction,
                items,
                vendor_prefix,
            } => {
                Visit::visit((direction).as_ref(), visitor);
                for value_3 in (items).iter() {
                    Visit::visit(value_3, visitor);
                }
                Visit::visit(vendor_prefix, visitor);
            }
            Gradient::Radial {
                items,
                position,
                shape,
                vendor_prefix,
            } => {
                for value_4 in (items).iter() {
                    Visit::visit(value_4, visitor);
                }
                Visit::visit((position).as_ref(), visitor);
                Visit::visit((shape).as_ref(), visitor);
                Visit::visit(vendor_prefix, visitor);
            }
            Gradient::RepeatingRadial {
                items,
                position,
                shape,
                vendor_prefix,
            } => {
                for value_7 in (items).iter() {
                    Visit::visit(value_7, visitor);
                }
                Visit::visit((position).as_ref(), visitor);
                Visit::visit((shape).as_ref(), visitor);
                Visit::visit(vendor_prefix, visitor);
            }
            Gradient::Conic {
                angle,
                items,
                position,
            } => {
                Visit::visit((angle).as_ref(), visitor);
                for value_11 in (items).iter() {
                    Visit::visit(value_11, visitor);
                }
                Visit::visit((position).as_ref(), visitor);
            }
            Gradient::RepeatingConic {
                angle,
                items,
                position,
            } => {
                Visit::visit((angle).as_ref(), visitor);
                for value_14 in (items).iter() {
                    Visit::visit(value_14, visitor);
                }
                Visit::visit((position).as_ref(), visitor);
            }
            Gradient::WebKitGradient(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Gradient);
    }
}
impl<'a> Visit<'a> for WebKitGradient<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitGradient);
        let node = self;
        match node {
            WebKitGradient::Linear { from, to, stops } => {
                Visit::visit((from).as_ref(), visitor);
                Visit::visit((to).as_ref(), visitor);
                for value_2 in (stops).iter() {
                    Visit::visit(value_2, visitor);
                }
            }
            WebKitGradient::Radial {
                from,
                start_radius,
                to,
                end_radius,
                stops,
            } => {
                Visit::visit((from).as_ref(), visitor);
                Visit::visit((to).as_ref(), visitor);
                for value_5 in (stops).iter() {
                    Visit::visit(value_5, visitor);
                }
            }
        }
        visitor.leave_node(AstType::WebKitGradient);
    }
}
impl<'a> Visit<'a> for LineDirection<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_line_direction(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LineDirection);
        let node = self;
        match node {
            LineDirection::Angle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            LineDirection::Horizontal(field_0) => {
                Visit::visit(field_0, visitor);
            }
            LineDirection::Vertical(field_0) => {
                Visit::visit(field_0, visitor);
            }
            LineDirection::Corner {
                horizontal,
                vertical,
            } => {
                Visit::visit(horizontal, visitor);
                Visit::visit(vertical, visitor);
            }
        }
        visitor.leave_node(AstType::LineDirection);
    }
}
impl<'a> Visit<'a> for HorizontalPositionKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_horizontal_position_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::HorizontalPositionKeyword);
        let node = self;
        match node {
            HorizontalPositionKeyword::Left => {}
            HorizontalPositionKeyword::Right => {}
        }
        visitor.leave_node(AstType::HorizontalPositionKeyword);
    }
}
impl<'a> Visit<'a> for VerticalPositionKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_vertical_position_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::VerticalPositionKeyword);
        let node = self;
        match node {
            VerticalPositionKeyword::Top => {}
            VerticalPositionKeyword::Bottom => {}
        }
        visitor.leave_node(AstType::VerticalPositionKeyword);
    }
}
impl<'a, D> Visit<'a> for GradientItem<'a, D>
where
    D: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_gradient_item(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GradientItem);
        let node = self;
        match node {
            GradientItem::ColorStop { color, position } => {
                Visit::visit((color).as_ref(), visitor);
                if let Some(value_1) = (position).as_ref() {
                    Visit::visit((value_1).as_ref(), visitor);
                }
            }
            GradientItem::Hint(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::GradientItem);
    }
}
impl<'a, D> Visit<'a> for DimensionPercentage<'a, D>
where
    D: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_dimension_percentage(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::DimensionPercentage);
        let node = self;
        match node {
            DimensionPercentage::Dimension(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            DimensionPercentage::Percentage(field_0) => {}
            DimensionPercentage::Zero => {}
            DimensionPercentage::Calc(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::DimensionPercentage);
    }
}
impl<'a, S> Visit<'a> for PositionComponent<'a, S>
where
    S: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_position_component(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PositionComponent);
        let node = self;
        match node {
            PositionComponent::Center => {}
            PositionComponent::Length(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            PositionComponent::Side { offset, side } => {
                if let Some(value_1) = (offset).as_ref() {
                    visitor.visit_length_percentage((value_1).as_ref());
                }
                Visit::visit(side, visitor);
            }
        }
        visitor.leave_node(AstType::PositionComponent);
    }
}
impl<'a> Visit<'a> for EndingShape<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_ending_shape(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::EndingShape);
        let node = self;
        match node {
            EndingShape::Ellipse(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            EndingShape::Circle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::EndingShape);
    }
}
impl<'a> Visit<'a> for Ellipse<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_ellipse(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Ellipse);
        let node = self;
        match node {
            Ellipse::Size { x, y } => {
                visitor.visit_length_percentage((x).as_ref());
                visitor.visit_length_percentage((y).as_ref());
            }
            Ellipse::Extent(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::Ellipse);
    }
}
impl<'a> Visit<'a> for ShapeExtent {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_shape_extent(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ShapeExtent);
        let node = self;
        match node {
            ShapeExtent::ClosestSide => {}
            ShapeExtent::FarthestSide => {}
            ShapeExtent::ClosestCorner => {}
            ShapeExtent::FarthestCorner => {}
        }
        visitor.leave_node(AstType::ShapeExtent);
    }
}
impl<'a> Visit<'a> for Circle<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_circle(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Circle);
        let node = self;
        match node {
            Circle::Radius(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Circle::Extent(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::Circle);
    }
}
impl<'a, S> Visit<'a> for WebKitGradientPointComponent<'a, S>
where
    S: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient_point_component(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitGradientPointComponent);
        let node = self;
        match node {
            WebKitGradientPointComponent::Center => {}
            WebKitGradientPointComponent::Number(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            WebKitGradientPointComponent::Side(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::WebKitGradientPointComponent);
    }
}
impl<'a> Visit<'a> for NumberOrPercentage {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_number_or_percentage(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NumberOrPercentage);
        let node = self;
        match node {
            NumberOrPercentage::Number(field_0) => {}
            NumberOrPercentage::Percentage(field_0) => {}
        }
        visitor.leave_node(AstType::NumberOrPercentage);
    }
}
impl<'a> Visit<'a> for BackgroundSize<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_background_size(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundSize);
        let node = self;
        match node {
            BackgroundSize::Explicit { height, width } => {
                Visit::visit((height).as_ref(), visitor);
                Visit::visit((width).as_ref(), visitor);
            }
            BackgroundSize::Cover => {}
            BackgroundSize::Contain => {}
        }
        visitor.leave_node(AstType::BackgroundSize);
    }
}
impl<'a> Visit<'a> for LengthPercentageOrAuto<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_length_percentage_or_auto(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LengthPercentageOrAuto);
        let node = self;
        match node {
            LengthPercentageOrAuto::Auto => {}
            LengthPercentageOrAuto::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
        }
        visitor.leave_node(AstType::LengthPercentageOrAuto);
    }
}
impl<'a> Visit<'a> for BackgroundRepeatKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_background_repeat_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundRepeatKeyword);
        let node = self;
        match node {
            BackgroundRepeatKeyword::Repeat => {}
            BackgroundRepeatKeyword::Space => {}
            BackgroundRepeatKeyword::Round => {}
            BackgroundRepeatKeyword::NoRepeat => {}
        }
        visitor.leave_node(AstType::BackgroundRepeatKeyword);
    }
}
impl<'a> Visit<'a> for BackgroundAttachment {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_background_attachment(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundAttachment);
        let node = self;
        match node {
            BackgroundAttachment::Scroll => {}
            BackgroundAttachment::Fixed => {}
            BackgroundAttachment::Local => {}
        }
        visitor.leave_node(AstType::BackgroundAttachment);
    }
}
impl<'a> Visit<'a> for BackgroundClip {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_background_clip(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundClip);
        let node = self;
        match node {
            BackgroundClip::BorderBox => {}
            BackgroundClip::PaddingBox => {}
            BackgroundClip::ContentBox => {}
            BackgroundClip::Border => {}
            BackgroundClip::Text => {}
        }
        visitor.leave_node(AstType::BackgroundClip);
    }
}
impl<'a> Visit<'a> for BackgroundOrigin {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_background_origin(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackgroundOrigin);
        let node = self;
        match node {
            BackgroundOrigin::BorderBox => {}
            BackgroundOrigin::PaddingBox => {}
            BackgroundOrigin::ContentBox => {}
        }
        visitor.leave_node(AstType::BackgroundOrigin);
    }
}
impl<'a> Visit<'a> for ListStyleType<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_list_style_type(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ListStyleType);
        let node = self;
        match node {
            ListStyleType::None => {}
            ListStyleType::String(field_0) => {
                visitor.visit_str(field_0);
            }
            ListStyleType::CounterStyle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::ListStyleType);
    }
}
impl<'a> Visit<'a> for CounterStyle<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_counter_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CounterStyle);
        let node = self;
        match node {
            CounterStyle::Predefined(field_0) => {
                Visit::visit(field_0, visitor);
            }
            CounterStyle::Name(field_0) => {
                visitor.visit_str(field_0);
            }
            CounterStyle::Symbols { symbols, system } => {
                for value_0 in (symbols).iter() {
                    Visit::visit(value_0, visitor);
                }
                Visit::visit(system, visitor);
            }
        }
        visitor.leave_node(AstType::CounterStyle);
    }
}
impl<'a> Visit<'a> for SymbolsType {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_symbols_type(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SymbolsType);
        let node = self;
        match node {
            SymbolsType::Cyclic => {}
            SymbolsType::Numeric => {}
            SymbolsType::Alphabetic => {}
            SymbolsType::Symbolic => {}
            SymbolsType::Fixed => {}
        }
        visitor.leave_node(AstType::SymbolsType);
    }
}
impl<'a> Visit<'a> for PredefinedCounterStyle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_predefined_counter_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PredefinedCounterStyle);
        let node = self;
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
}
impl<'a> Visit<'a> for Symbol<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_symbol(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Symbol);
        let node = self;
        match node {
            Symbol::String(field_0) => {
                visitor.visit_str(field_0);
            }
            Symbol::Image(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Symbol);
    }
}
impl<'a> Visit<'a> for ListStylePosition {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_list_style_position(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ListStylePosition);
        let node = self;
        match node {
            ListStylePosition::Inside => {}
            ListStylePosition::Outside => {}
        }
        visitor.leave_node(AstType::ListStylePosition);
    }
}
impl<'a> Visit<'a> for MarkerSide {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_marker_side(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MarkerSide);
        let node = self;
        match node {
            MarkerSide::MatchSelf => {}
            MarkerSide::MatchParent => {}
        }
        visitor.leave_node(AstType::MarkerSide);
    }
}
impl<'a> Visit<'a> for MaskMode {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_mode(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MaskMode);
        let node = self;
        match node {
            MaskMode::Luminance => {}
            MaskMode::Alpha => {}
            MaskMode::MatchSource => {}
        }
        visitor.leave_node(AstType::MaskMode);
    }
}
impl<'a> Visit<'a> for MaskClip {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_clip(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MaskClip);
        let node = self;
        match node {
            MaskClip::GeometryBox(field_0) => {
                Visit::visit(field_0, visitor);
            }
            MaskClip::NoClip => {}
        }
        visitor.leave_node(AstType::MaskClip);
    }
}
impl<'a> Visit<'a> for MaskComposite {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_composite(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MaskComposite);
        let node = self;
        match node {
            MaskComposite::Add => {}
            MaskComposite::Subtract => {}
            MaskComposite::Intersect => {}
            MaskComposite::Exclude => {}
        }
        visitor.leave_node(AstType::MaskComposite);
    }
}
impl<'a> Visit<'a> for MaskType {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_type(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MaskType);
        let node = self;
        match node {
            MaskType::Luminance => {}
            MaskType::Alpha => {}
        }
        visitor.leave_node(AstType::MaskType);
    }
}
impl<'a> Visit<'a> for MaskBorderMode {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_border_mode(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MaskBorderMode);
        let node = self;
        match node {
            MaskBorderMode::Luminance => {}
            MaskBorderMode::Alpha => {}
        }
        visitor.leave_node(AstType::MaskBorderMode);
    }
}
impl<'a> Visit<'a> for WebKitMaskComposite {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_mask_composite(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitMaskComposite);
        let node = self;
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
}
impl<'a> Visit<'a> for WebKitMaskSourceType {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_mask_source_type(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitMaskSourceType);
        let node = self;
        match node {
            WebKitMaskSourceType::Auto => {}
            WebKitMaskSourceType::Luminance => {}
            WebKitMaskSourceType::Alpha => {}
        }
        visitor.leave_node(AstType::WebKitMaskSourceType);
    }
}
impl<'a> Visit<'a> for CSSWideKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_css_wide_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CSSWideKeyword);
        let node = self;
        match node {
            CSSWideKeyword::Initial => {}
            CSSWideKeyword::Inherit => {}
            CSSWideKeyword::Unset => {}
            CSSWideKeyword::Revert => {}
            CSSWideKeyword::RevertLayer => {}
        }
        visitor.leave_node(AstType::CSSWideKeyword);
    }
}
impl<'a> Visit<'a> for CustomPropertyName<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_custom_property_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CustomPropertyName);
        let node = self;
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
}
impl<'a> Visit<'a> for ClipPath<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_clip_path(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ClipPath);
        let node = self;
        match node {
            ClipPath::None => {}
            ClipPath::Url(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            ClipPath::Shape {
                reference_box,
                shape,
            } => {
                Visit::visit(reference_box, visitor);
                Visit::visit((shape).as_ref(), visitor);
            }
            ClipPath::Box(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::ClipPath);
    }
}
impl<'a> Visit<'a> for GeometryBox {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_geometry_box(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::GeometryBox);
        let node = self;
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
}
impl<'a> Visit<'a> for BasicShape<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_basic_shape(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BasicShape);
        let node = self;
        match node {
            BasicShape::Inset(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            BasicShape::Circle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            BasicShape::Ellipse(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            BasicShape::Polygon(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::BasicShape);
    }
}
impl<'a> Visit<'a> for ShapeRadius<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_shape_radius(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ShapeRadius);
        let node = self;
        match node {
            ShapeRadius::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            ShapeRadius::ClosestSide => {}
            ShapeRadius::FarthestSide => {}
        }
        visitor.leave_node(AstType::ShapeRadius);
    }
}
impl<'a> Visit<'a> for SVGPaint<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_svg_paint(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SVGPaint);
        let node = self;
        match node {
            SVGPaint::Url { fallback, url } => {
                if let Some(value_0) = (fallback).as_ref() {
                    Visit::visit((value_0).as_ref(), visitor);
                }
                Visit::visit((url).as_ref(), visitor);
            }
            SVGPaint::Color(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            SVGPaint::ContextFill => {}
            SVGPaint::ContextStroke => {}
            SVGPaint::None => {}
        }
        visitor.leave_node(AstType::SVGPaint);
    }
}
impl<'a> Visit<'a> for SVGPaintFallback<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_svg_paint_fallback(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SVGPaintFallback);
        let node = self;
        match node {
            SVGPaintFallback::None => {}
            SVGPaintFallback::Color(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::SVGPaintFallback);
    }
}
impl<'a> Visit<'a> for FillRule {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_fill_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FillRule);
        let node = self;
        match node {
            FillRule::Nonzero => {}
            FillRule::Evenodd => {}
        }
        visitor.leave_node(AstType::FillRule);
    }
}
impl<'a> Visit<'a> for StrokeLinecap {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_stroke_linecap(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StrokeLinecap);
        let node = self;
        match node {
            StrokeLinecap::Butt => {}
            StrokeLinecap::Round => {}
            StrokeLinecap::Square => {}
        }
        visitor.leave_node(AstType::StrokeLinecap);
    }
}
impl<'a> Visit<'a> for StrokeLinejoin {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_stroke_linejoin(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StrokeLinejoin);
        let node = self;
        match node {
            StrokeLinejoin::Miter => {}
            StrokeLinejoin::MiterClip => {}
            StrokeLinejoin::Round => {}
            StrokeLinejoin::Bevel => {}
            StrokeLinejoin::Arcs => {}
        }
        visitor.leave_node(AstType::StrokeLinejoin);
    }
}
impl<'a> Visit<'a> for StrokeDasharray<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_stroke_dasharray(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::StrokeDasharray);
        let node = self;
        match node {
            StrokeDasharray::None => {}
            StrokeDasharray::Values(field_0) => {
                for value_0 in (field_0).iter() {
                    visitor.visit_length_percentage(value_0);
                }
            }
        }
        visitor.leave_node(AstType::StrokeDasharray);
    }
}
impl<'a> Visit<'a> for Marker<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_marker(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Marker);
        let node = self;
        match node {
            Marker::None => {}
            Marker::Url(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Marker);
    }
}
impl<'a> Visit<'a> for ColorInterpolation {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_color_interpolation(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ColorInterpolation);
        let node = self;
        match node {
            ColorInterpolation::Auto => {}
            ColorInterpolation::Srgb => {}
            ColorInterpolation::Linearrgb => {}
        }
        visitor.leave_node(AstType::ColorInterpolation);
    }
}
impl<'a> Visit<'a> for ColorRendering {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_color_rendering(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ColorRendering);
        let node = self;
        match node {
            ColorRendering::Auto => {}
            ColorRendering::Optimizespeed => {}
            ColorRendering::Optimizequality => {}
        }
        visitor.leave_node(AstType::ColorRendering);
    }
}
impl<'a> Visit<'a> for ShapeRendering {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_shape_rendering(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ShapeRendering);
        let node = self;
        match node {
            ShapeRendering::Auto => {}
            ShapeRendering::Optimizespeed => {}
            ShapeRendering::Crispedges => {}
            ShapeRendering::Geometricprecision => {}
        }
        visitor.leave_node(AstType::ShapeRendering);
    }
}
impl<'a> Visit<'a> for TextRendering {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_rendering(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextRendering);
        let node = self;
        match node {
            TextRendering::Auto => {}
            TextRendering::Optimizespeed => {}
            TextRendering::Optimizelegibility => {}
            TextRendering::Geometricprecision => {}
        }
        visitor.leave_node(AstType::TextRendering);
    }
}
impl<'a> Visit<'a> for ImageRendering {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_image_rendering(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ImageRendering);
        let node = self;
        match node {
            ImageRendering::Auto => {}
            ImageRendering::Optimizespeed => {}
            ImageRendering::Optimizequality => {}
        }
        visitor.leave_node(AstType::ImageRendering);
    }
}
impl<'a> Visit<'a> for TextTransformCase {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_transform_case(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextTransformCase);
        let node = self;
        match node {
            TextTransformCase::None => {}
            TextTransformCase::Uppercase => {}
            TextTransformCase::Lowercase => {}
            TextTransformCase::Capitalize => {}
        }
        visitor.leave_node(AstType::TextTransformCase);
    }
}
impl<'a> Visit<'a> for WhiteSpace {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_white_space(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WhiteSpace);
        let node = self;
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
}
impl<'a> Visit<'a> for WordBreak {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_word_break(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WordBreak);
        let node = self;
        match node {
            WordBreak::Normal => {}
            WordBreak::KeepAll => {}
            WordBreak::BreakAll => {}
            WordBreak::BreakWord => {}
        }
        visitor.leave_node(AstType::WordBreak);
    }
}
impl<'a> Visit<'a> for LineBreak {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_line_break(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LineBreak);
        let node = self;
        match node {
            LineBreak::Auto => {}
            LineBreak::Loose => {}
            LineBreak::Normal => {}
            LineBreak::Strict => {}
            LineBreak::Anywhere => {}
        }
        visitor.leave_node(AstType::LineBreak);
    }
}
impl<'a> Visit<'a> for Hyphens {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_hyphens(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Hyphens);
        let node = self;
        match node {
            Hyphens::None => {}
            Hyphens::Manual => {}
            Hyphens::Auto => {}
        }
        visitor.leave_node(AstType::Hyphens);
    }
}
impl<'a> Visit<'a> for OverflowWrap {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_overflow_wrap(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::OverflowWrap);
        let node = self;
        match node {
            OverflowWrap::Normal => {}
            OverflowWrap::Anywhere => {}
            OverflowWrap::BreakWord => {}
        }
        visitor.leave_node(AstType::OverflowWrap);
    }
}
impl<'a> Visit<'a> for TextAlign {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_align(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextAlign);
        let node = self;
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
}
impl<'a> Visit<'a> for TextAlignLast {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_align_last(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextAlignLast);
        let node = self;
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
}
impl<'a> Visit<'a> for TextJustify {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_justify(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextJustify);
        let node = self;
        match node {
            TextJustify::Auto => {}
            TextJustify::None => {}
            TextJustify::InterWord => {}
            TextJustify::InterCharacter => {}
        }
        visitor.leave_node(AstType::TextJustify);
    }
}
impl<'a> Visit<'a> for Spacing<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_spacing(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Spacing);
        let node = self;
        match node {
            Spacing::Normal => {}
            Spacing::Length(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Spacing);
    }
}
impl<'a> Visit<'a> for TextDecorationLine<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_line(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextDecorationLine);
        let node = self;
        match node {
            TextDecorationLine::ExclusiveTextDecorationLine(field_0) => {
                Visit::visit(field_0, visitor);
            }
            TextDecorationLine::Value(field_0) => {
                for value_0 in (field_0).iter() {
                    Visit::visit(value_0, visitor);
                }
            }
        }
        visitor.leave_node(AstType::TextDecorationLine);
    }
}
impl<'a> Visit<'a> for ExclusiveTextDecorationLine {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_exclusive_text_decoration_line(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ExclusiveTextDecorationLine);
        let node = self;
        match node {
            ExclusiveTextDecorationLine::None => {}
            ExclusiveTextDecorationLine::SpellingError => {}
            ExclusiveTextDecorationLine::GrammarError => {}
        }
        visitor.leave_node(AstType::ExclusiveTextDecorationLine);
    }
}
impl<'a> Visit<'a> for OtherTextDecorationLine {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_other_text_decoration_line(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::OtherTextDecorationLine);
        let node = self;
        match node {
            OtherTextDecorationLine::Underline => {}
            OtherTextDecorationLine::Overline => {}
            OtherTextDecorationLine::LineThrough => {}
            OtherTextDecorationLine::Blink => {}
        }
        visitor.leave_node(AstType::OtherTextDecorationLine);
    }
}
impl<'a> Visit<'a> for TextDecorationStyle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextDecorationStyle);
        let node = self;
        match node {
            TextDecorationStyle::Solid => {}
            TextDecorationStyle::Double => {}
            TextDecorationStyle::Dotted => {}
            TextDecorationStyle::Dashed => {}
            TextDecorationStyle::Wavy => {}
        }
        visitor.leave_node(AstType::TextDecorationStyle);
    }
}
impl<'a> Visit<'a> for TextDecorationThickness<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_thickness(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextDecorationThickness);
        let node = self;
        match node {
            TextDecorationThickness::Auto => {}
            TextDecorationThickness::FromFont => {}
            TextDecorationThickness::LengthPercentage(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
        }
        visitor.leave_node(AstType::TextDecorationThickness);
    }
}
impl<'a> Visit<'a> for TextDecorationSkipInk {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_skip_ink(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextDecorationSkipInk);
        let node = self;
        match node {
            TextDecorationSkipInk::Auto => {}
            TextDecorationSkipInk::None => {}
            TextDecorationSkipInk::All => {}
        }
        visitor.leave_node(AstType::TextDecorationSkipInk);
    }
}
impl<'a> Visit<'a> for TextEmphasisStyle<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasisStyle);
        let node = self;
        match node {
            TextEmphasisStyle::None => {}
            TextEmphasisStyle::Keyword { fill, shape } => {
                Visit::visit(fill, visitor);
                if let Some(value_0) = (shape).as_ref() {
                    Visit::visit(value_0, visitor);
                }
            }
            TextEmphasisStyle::String(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::TextEmphasisStyle);
    }
}
impl<'a> Visit<'a> for TextEmphasisFillMode {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_fill_mode(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasisFillMode);
        let node = self;
        match node {
            TextEmphasisFillMode::Filled => {}
            TextEmphasisFillMode::Open => {}
        }
        visitor.leave_node(AstType::TextEmphasisFillMode);
    }
}
impl<'a> Visit<'a> for TextEmphasisShape {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_shape(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasisShape);
        let node = self;
        match node {
            TextEmphasisShape::Dot => {}
            TextEmphasisShape::Circle => {}
            TextEmphasisShape::DoubleCircle => {}
            TextEmphasisShape::Triangle => {}
            TextEmphasisShape::Sesame => {}
        }
        visitor.leave_node(AstType::TextEmphasisShape);
    }
}
impl<'a> Visit<'a> for TextEmphasisPositionHorizontal {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position_horizontal(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasisPositionHorizontal);
        let node = self;
        match node {
            TextEmphasisPositionHorizontal::Left => {}
            TextEmphasisPositionHorizontal::Right => {}
        }
        visitor.leave_node(AstType::TextEmphasisPositionHorizontal);
    }
}
impl<'a> Visit<'a> for TextEmphasisPositionVertical {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position_vertical(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextEmphasisPositionVertical);
        let node = self;
        match node {
            TextEmphasisPositionVertical::Over => {}
            TextEmphasisPositionVertical::Under => {}
        }
        visitor.leave_node(AstType::TextEmphasisPositionVertical);
    }
}
impl<'a> Visit<'a> for TextSizeAdjust {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_size_adjust(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextSizeAdjust);
        let node = self;
        match node {
            TextSizeAdjust::Auto => {}
            TextSizeAdjust::None => {}
            TextSizeAdjust::Percentage(field_0) => {}
        }
        visitor.leave_node(AstType::TextSizeAdjust);
    }
}
impl<'a> Visit<'a> for TextDirection {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_text_direction(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TextDirection);
        let node = self;
        match node {
            TextDirection::Ltr => {}
            TextDirection::Rtl => {}
        }
        visitor.leave_node(AstType::TextDirection);
    }
}
impl<'a> Visit<'a> for UnicodeBidi {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_unicode_bidi(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UnicodeBidi);
        let node = self;
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
}
impl<'a> Visit<'a> for Transform<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_transform(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Transform);
        let node = self;
        match node {
            Transform::Translate(field_0) => {
                visitor.visit_length_percentage((&(field_0).0).as_ref());
                visitor.visit_length_percentage((&(field_0).1).as_ref());
            }
            Transform::TranslateX(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            Transform::TranslateY(field_0) => {
                visitor.visit_length_percentage((field_0).as_ref());
            }
            Transform::TranslateZ(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::Translate3d(field_0) => {
                visitor.visit_length_percentage((&(field_0).0).as_ref());
                visitor.visit_length_percentage((&(field_0).1).as_ref());
                Visit::visit((&(field_0).2).as_ref(), visitor);
            }
            Transform::Scale(field_0) => {
                Visit::visit((&(field_0).0).as_ref(), visitor);
                Visit::visit((&(field_0).1).as_ref(), visitor);
            }
            Transform::ScaleX(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::ScaleY(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::ScaleZ(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::Scale3d(field_0) => {
                Visit::visit((&(field_0).0).as_ref(), visitor);
                Visit::visit((&(field_0).1).as_ref(), visitor);
                Visit::visit((&(field_0).2).as_ref(), visitor);
            }
            Transform::Rotate(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::RotateX(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::RotateY(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::RotateZ(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::Rotate3d(field_0) => {
                Visit::visit((&(field_0).3).as_ref(), visitor);
            }
            Transform::Skew(field_0) => {
                Visit::visit((&(field_0).0).as_ref(), visitor);
                Visit::visit((&(field_0).1).as_ref(), visitor);
            }
            Transform::SkewX(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::SkewY(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::Perspective(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::Matrix(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Transform::Matrix3d(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Transform);
    }
}
impl<'a> Visit<'a> for TransformStyle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_transform_style(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TransformStyle);
        let node = self;
        match node {
            TransformStyle::Flat => {}
            TransformStyle::Preserve3d => {}
        }
        visitor.leave_node(AstType::TransformStyle);
    }
}
impl<'a> Visit<'a> for TransformBox {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_transform_box(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::TransformBox);
        let node = self;
        match node {
            TransformBox::ContentBox => {}
            TransformBox::BorderBox => {}
            TransformBox::FillBox => {}
            TransformBox::StrokeBox => {}
            TransformBox::ViewBox => {}
        }
        visitor.leave_node(AstType::TransformBox);
    }
}
impl<'a> Visit<'a> for BackfaceVisibility {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_backface_visibility(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BackfaceVisibility);
        let node = self;
        match node {
            BackfaceVisibility::Visible => {}
            BackfaceVisibility::Hidden => {}
        }
        visitor.leave_node(AstType::BackfaceVisibility);
    }
}
impl<'a> Visit<'a> for Perspective<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_perspective(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Perspective);
        let node = self;
        match node {
            Perspective::None => {}
            Perspective::Length(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Perspective);
    }
}
impl<'a> Visit<'a> for Translate<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_translate(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Translate);
        let node = self;
        match node {
            Translate::None => {}
            Translate::Xyz { x, y, z } => {
                visitor.visit_length_percentage((x).as_ref());
                visitor.visit_length_percentage((y).as_ref());
                Visit::visit((z).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Translate);
    }
}
impl<'a> Visit<'a> for Scale<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_scale(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Scale);
        let node = self;
        match node {
            Scale::None => {}
            Scale::Xyz { x, y, z } => {
                Visit::visit((x).as_ref(), visitor);
                Visit::visit((y).as_ref(), visitor);
                Visit::visit((z).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Scale);
    }
}
impl<'a> Visit<'a> for Resize {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_resize(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Resize);
        let node = self;
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
}
impl<'a> Visit<'a> for CursorKeyword {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_cursor_keyword(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CursorKeyword);
        let node = self;
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
}
impl<'a> Visit<'a> for ColorOrAuto<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_color_or_auto(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ColorOrAuto);
        let node = self;
        match node {
            ColorOrAuto::Auto => {}
            ColorOrAuto::Color(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::ColorOrAuto);
    }
}
impl<'a> Visit<'a> for CaretShape {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_caret_shape(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CaretShape);
        let node = self;
        match node {
            CaretShape::Auto => {}
            CaretShape::Bar => {}
            CaretShape::Block => {}
            CaretShape::Underscore => {}
        }
        visitor.leave_node(AstType::CaretShape);
    }
}
impl<'a> Visit<'a> for UserSelect {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_user_select(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UserSelect);
        let node = self;
        match node {
            UserSelect::Auto => {}
            UserSelect::Text => {}
            UserSelect::None => {}
            UserSelect::Contain => {}
            UserSelect::All => {}
        }
        visitor.leave_node(AstType::UserSelect);
    }
}
impl<'a> Visit<'a> for Appearance<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_appearance(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Appearance);
        let node = self;
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
}
impl<'a> Visit<'a> for PrintColorAdjust {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_print_color_adjust(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PrintColorAdjust);
        let node = self;
        match node {
            PrintColorAdjust::Economy => {}
            PrintColorAdjust::Exact => {}
        }
        visitor.leave_node(AstType::PrintColorAdjust);
    }
}
impl<'a> Visit<'a> for ViewTransitionName<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionName);
        let node = self;
        match node {
            ViewTransitionName::None => {}
            ViewTransitionName::Auto => {}
            ViewTransitionName::Custom(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::ViewTransitionName);
    }
}
impl<'a> Visit<'a> for NoneOrCustomIdentList<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_none_or_custom_ident_list(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NoneOrCustomIdentList);
        let node = self;
        match node {
            NoneOrCustomIdentList::None => {}
            NoneOrCustomIdentList::Idents(field_0) => {
                for value_0 in (field_0).iter() {
                    visitor.visit_str(value_0);
                }
            }
        }
        visitor.leave_node(AstType::NoneOrCustomIdentList);
    }
}
impl<'a> Visit<'a> for ViewTransitionGroup<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_group(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionGroup);
        let node = self;
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
}
