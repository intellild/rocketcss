use super::*;

pub(in crate::parser) fn parse_page_selectors<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<Vec<'i, PageSelector<'i>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    if prelude.is_empty() {
        return Ok(allocator.vec());
    }
    input.with_source(prelude, |input| {
        let parsed = input.parse_comma_separated(|input| {
            let name = input.try_parse(Compiler::expect_ident).ok();
            let mut pseudo_classes = allocator.vec();
            while input.try_parse(Compiler::expect_colon).is_ok() {
                let pseudo = input.expect_ident()?;
                pseudo_classes.push(match_ignore_ascii_case!(
                    pseudo,
                    "left" => PagePseudoClass::Left,
                    "right" => PagePseudoClass::Right,
                    "first" => PagePseudoClass::First,
                    "last" => PagePseudoClass::Last,
                    "blank" => PagePseudoClass::Blank,
                    _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
                ));
            }
            if name.is_none() && pseudo_classes.is_empty() {
                return Err(input.new_custom_error(ParserError::InvalidSelector));
            }
            input.expect_exhausted()?;
            Ok(PageSelector {
                name: name.map(|name| input.add_str(name)),
                pseudo_classes: store_vec(pseudo_classes, input),
            })
        })?;
        let mut selectors = allocator.vec();
        selectors.extend(parsed);
        Ok(selectors)
    })
}

pub(in crate::parser) fn page_margin_box(name: &str) -> Option<PageMarginBox> {
    match_ignore_ascii_case!(
        name,
        "top-left-corner" => Some(PageMarginBox::TopLeftCorner),
        "top-left" => Some(PageMarginBox::TopLeft),
        "top-center" => Some(PageMarginBox::TopCenter),
        "top-right" => Some(PageMarginBox::TopRight),
        "top-right-corner" => Some(PageMarginBox::TopRightCorner),
        "left-top" => Some(PageMarginBox::LeftTop),
        "left-middle" => Some(PageMarginBox::LeftMiddle),
        "left-bottom" => Some(PageMarginBox::LeftBottom),
        "right-top" => Some(PageMarginBox::RightTop),
        "right-middle" => Some(PageMarginBox::RightMiddle),
        "right-bottom" => Some(PageMarginBox::RightBottom),
        "bottom-left-corner" => Some(PageMarginBox::BottomLeftCorner),
        "bottom-left" => Some(PageMarginBox::BottomLeft),
        "bottom-center" => Some(PageMarginBox::BottomCenter),
        "bottom-right" => Some(PageMarginBox::BottomRight),
        "bottom-right-corner" => Some(PageMarginBox::BottomRightCorner),
        _ => None,
    )
}
