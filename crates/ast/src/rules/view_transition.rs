use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionProperty<'a> {
    Navigation(Navigation),
    Types(NodeId<'a, NoneOrCustomIdentList<'a>>),
    Custom(NodeId<'a, CustomProperty<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Navigation {
    None,
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
pub struct ViewTransitionPartSelector<'a> {
    pub classes: Vec<'a, AstStr<'a>>,
    pub name: Option<NodeId<'a, ViewTransitionPartName<'a>>>,
}

impl_inline_node!(ViewTransitionPartSelector<'ast>, 0x001b_0005);

impl<'ast> AstNodeClone<'ast> for ViewTransitionPartSelector<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            classes: context.clone_encoded_vec(self.classes),
            name: self.name.map(|name| context.clone_encoded_node(name)),
        }
    }
}
