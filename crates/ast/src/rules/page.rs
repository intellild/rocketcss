use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum PageMarginBox {
    TopLeftCorner,
    TopLeft,
    TopCenter,
    TopRight,
    TopRightCorner,
    LeftTop,
    LeftMiddle,
    LeftBottom,
    RightTop,
    RightMiddle,
    RightBottom,
    BottomLeftCorner,
    BottomLeft,
    BottomCenter,
    BottomRight,
    BottomRightCorner,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum PagePseudoClass {
    Left,
    Right,
    First,
    Last,
    Blank,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PageRule<'a> {
    #[visit(with = visit_declaration_block_id, with_mut = visit_declaration_block_id_mut)]
    pub declarations: DeclarationBlockId,
    pub span: Span,
    pub rules: Vec<'a, PageMarginRule>,
    pub selectors: Vec<'a, PageSelector<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PageMarginRule {
    #[visit(with = visit_declaration_block_id, with_mut = visit_declaration_block_id_mut)]
    pub declarations: DeclarationBlockId,
    pub span: Span,
    pub margin_box: PageMarginBox,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PageSelector<'a> {
    pub name: Option<&'a str>,
    pub pseudo_classes: Vec<'a, PagePseudoClass>,
}
