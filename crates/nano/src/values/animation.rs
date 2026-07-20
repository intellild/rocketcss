use rocketcss_ast::{Animation, AnimationComponent};

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl Minify for Animation<'_> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if self.components.len() < 2 || !cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any) {
            return;
        }
        // A keyframes name that collides with a present keyword class is
        // deferred behind the class so a reparse claims the class first.
        // With the class absent the name keeps its authored quotes instead.
        let defer_name = self.components.iter().any(|component| {
            let AnimationComponent::Name(name) = component else {
                return false;
            };
            name.keyword_class().is_some_and(|class| {
                self.components
                    .iter()
                    .any(|component| component.keyword_class() == Some(class))
            })
        });
        let rank = |component: &AnimationComponent<'_>| match component {
            AnimationComponent::Name(_) if defer_name => 8,
            AnimationComponent::Name(_) => 0,
            AnimationComponent::Duration(_) => 1,
            AnimationComponent::TimingFunction(_) => 2,
            AnimationComponent::Delay(_) => 3,
            AnimationComponent::IterationCount(_) => 4,
            AnimationComponent::Direction(_) => 5,
            AnimationComponent::FillMode(_) => 6,
            AnimationComponent::PlayState(_) => 7,
        };
        let mut changed = false;
        for right in 1..self.components.len() {
            let mut current = right;
            while current > 0
                && rank(&self.components[current - 1]) > rank(&self.components[current])
            {
                self.components.swap(current - 1, current);
                current -= 1;
                changed = true;
            }
        }
        if changed {
            cx.record_value_normalized();
        }
    }
}
