use crate::*;

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum ContainerType {
    Normal,
    InlineSize,
    Size,
    ScrollState,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ContainerNameList<'a> {
    None,
    Names(Vec<'a, AstStr<'a>>),
}

impl_inline_node!(ContainerNameList<'ast>, 0x001e_0001);

impl<'ast> AstNodeClone<'ast> for ContainerNameList<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Names(values) => Self::Names(context.clone_encoded_vec(values)),
        }
    }
}
