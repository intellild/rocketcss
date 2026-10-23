//! Streaming serialization for the flattened AST.

use crate::{prelude::*, rules::NamedProperty};

#[derive(Clone, Copy)]
enum LastSemicolon {
    Optional,
    Required,
}

impl<'ast, 'ghost> ToCss<'ghost> for rocketcss_ast_next::Compilation<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = self.ast();
        let sheet = self.style_sheet();
        let comments = sheet.license_comments(ast);
        let rules = sheet.rules(ast);
        for (index, comment) in comments.iter().enumerate() {
            dest.write_str("/*")?;
            dest.write_str(comment)?;
            dest.write_str("*/")?;
            if index + 1 < comments.len() || !rules.is_empty() {
                dest.new_line()?;
            }
        }
        write_rule_list(ast, rules, dest, cx)?;
        if !rules.is_empty() {
            dest.new_line()?;
        }
        Ok(())
    }
}

fn visible_rule<'ast>(ast: &Ast<'ast>, rule: CssRule<'ast>) -> bool {
    match rule.payload(ast).data(ast) {
        CssRulePayloadData::Style(style) => style
            .selectors(ast)
            .iter()
            .any(|selector| !matches!(selector.data(ast), SelectorData::Tombstone)),
        _ => true,
    }
}

fn write_rule_list<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rules: &[CssRule<'ast>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let mut first = true;
    let mut last_without_block = false;
    for (index, rule) in rules.iter().enumerate() {
        if !visible_rule(ast, *rule) {
            continue;
        }
        let has_next = rules[index + 1..]
            .iter()
            .any(|rule| visible_rule(ast, *rule));
        if !first {
            if !last_without_block || !rule_without_block(ast, rule.payload(ast)) {
                dest.blank_line()?;
            } else {
                dest.new_line()?;
            }
        }
        first = false;
        let last_semicolon = if has_next {
            LastSemicolon::Required
        } else {
            LastSemicolon::Optional
        };
        write_rule(ast, *rule, dest, last_semicolon, cx)?;
        last_without_block = rule_without_block(ast, rule.payload(ast));
    }
    Ok(())
}

fn write_rule<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rule: CssRule<'ast>,
    dest: &mut PrinterT,
    last_semicolon: LastSemicolon,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    match rule.payload(ast).data(ast) {
        CssRulePayloadData::Style(payload) => {
            payload.selectors(ast).to_css(dest, cx)?;
            write_style_body(ast, rule, dest, cx)
        }
        CssRulePayloadData::Media(payload) => {
            dest.write_str("@media ")?;
            payload.query(ast).to_css(dest, cx)?;
            write_child_rule_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::Supports(payload) => {
            dest.write_str("@supports ")?;
            payload.condition(ast).to_css(dest, cx)?;
            write_child_rule_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::StartingStyle(_) => {
            dest.write_str("@starting-style")?;
            write_child_rule_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::LayerStatement(payload) => {
            dest.write_str("@layer ")?;
            for (index, name) in payload.names(ast).iter().enumerate() {
                if index > 0 {
                    dest.delim(Delimiter::Comma)?;
                }
                write_layer_name(name, dest)?;
            }
            dest.write_char(';')
        }
        CssRulePayloadData::LayerBlock(payload) => {
            dest.write_str("@layer")?;
            if let Some(name) = payload.name(ast) {
                dest.write_char(' ')?;
                write_layer_name(name, dest)?;
            }
            write_child_rule_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::Container(payload) => {
            dest.write_str("@container")?;
            if let Some(name) = payload.name(ast) {
                dest.write_char(' ')?;
                serialize_identifier(name, dest)?;
            }
            if let Some(condition) = payload.condition(ast) {
                dest.write_char(' ')?;
                condition.to_css(dest, cx)?;
            }
            write_child_rule_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::Scope(payload) => {
            dest.write_str("@scope")?;
            if let Some(start) = payload.scope_start(ast) {
                dest.write_str(" (")?;
                start.to_css(dest, cx)?;
                dest.write_char(')')?;
            }
            if let Some(end) = payload.scope_end(ast) {
                dest.write_str(" to (")?;
                end.to_css(dest, cx)?;
                dest.write_char(')')?;
            }
            write_child_rule_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::MozDocument(_) => {
            dest.write_str("@-moz-document url-prefix()")?;
            write_child_rule_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::Unknown(payload) => {
            dest.write_char('@')?;
            serialize_identifier(payload.name(ast), dest)?;
            let prelude = payload.prelude(ast);
            if !prelude.is_empty() {
                dest.write_char(' ')?;
                crate::token::write_token_list_without_outer_whitespace(prelude, dest, cx)?;
            }
            if let Some(block) = payload.block(ast) {
                write_block(dest, |dest| {
                    crate::token::write_token_list_without_outer_whitespace(block, dest, cx)
                })
            } else {
                dest.write_char(';')
            }
        }
        CssRulePayloadData::CounterStyle(payload) => {
            dest.write_str("@counter-style ")?;
            serialize_identifier(payload.name(ast), dest)?;
            write_property_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::Viewport(payload) => {
            dest.write_char('@')?;
            payload.vendor_prefix(ast).to_css(dest, cx)?;
            dest.write_str("viewport")?;
            write_property_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::PositionTry(payload) => {
            dest.write_str("@position-try ")?;
            dest.write_str("--")?;
            serialize_name(
                payload
                    .name(ast)
                    .strip_prefix("--")
                    .unwrap_or(payload.name(ast)),
                dest,
            )?;
            write_property_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::FontFace(_) => {
            dest.write_str("@font-face")?;
            write_named_property_block(ast, rule, dest, cx, NamedKind::FontFace)
        }
        CssRulePayloadData::FontPaletteValues(payload) => {
            dest.write_str("@font-palette-values ")?;
            serialize_identifier(payload.name(ast), dest)?;
            write_named_property_block(ast, rule, dest, cx, NamedKind::FontPalette)
        }
        CssRulePayloadData::ViewTransition(_) => {
            dest.write_str("@view-transition")?;
            write_named_property_block(ast, rule, dest, cx, NamedKind::ViewTransition)
        }
        CssRulePayloadData::Import(payload) => payload.to_css(dest, cx),
        CssRulePayloadData::Charset(payload) => payload.to_css(dest, cx),
        CssRulePayloadData::Namespace(payload) => payload.to_css(dest, cx),
        CssRulePayloadData::CustomMedia(payload) => payload.to_css(dest, cx),
        CssRulePayloadData::Keyframes(payload) => {
            dest.write_char('@')?;
            payload.vendor_prefix(ast).to_css(dest, cx)?;
            dest.write_str("keyframes ")?;
            payload.name(ast).to_css(dest, cx)?;
            if rule.children(ast).is_empty() {
                dest.whitespace()?;
                dest.write_char('{')?;
                dest.new_line()?;
                dest.write_char('}')
            } else {
                write_child_rule_block(ast, rule, dest, cx)
            }
        }
        CssRulePayloadData::Keyframe(payload) => {
            write_comma_separated(payload.selectors(ast), dest, cx)?;
            write_property_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::Page(payload) => write_page_rule(ast, rule, payload, dest, cx),
        CssRulePayloadData::PageMargin(payload) => {
            dest.write_char('@')?;
            payload.margin_box(ast).to_css(dest, cx)?;
            write_property_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::PageDeclarations(_) => {
            let block = rule
                .declarations(ast)
                .expect("a page declaration segment owns a block");
            write_property_declarations(ast, block, dest, last_semicolon, cx).map(|_| ())
        }
        CssRulePayloadData::Nesting(payload) => {
            dest.write_str("@nest ")?;
            payload.selectors(ast).to_css(dest, cx)?;
            write_style_body(ast, rule, dest, cx)
        }
        CssRulePayloadData::FontFeatureValues(payload) => {
            dest.write_str("@font-feature-values ")?;
            write_comma_separated(payload.name(ast), dest, cx)?;
            write_child_rule_block(ast, rule, dest, cx)
        }
        CssRulePayloadData::FontFeatureSubrule(payload) => {
            write_font_feature_subrule(ast, rule, payload, dest, cx)
        }
        CssRulePayloadData::Property(payload) => write_property_rule(ast, rule, payload, dest, cx),
        CssRulePayloadData::NestedDeclarations(_) => {
            let block = rule
                .declarations(ast)
                .expect("a nested declaration segment owns a block");
            write_property_declarations(ast, block, dest, last_semicolon, cx).map(|_| ())
        }
    }
}

fn write_style_body<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rule: CssRule<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let children = rule.children(ast);
    let has_children = !children.is_empty();
    let block = rule
        .declarations(ast)
        .expect("a style syntax position owns a declaration block");
    write_block(dest, |dest| {
        let has_declarations = write_property_declarations(
            ast,
            block,
            dest,
            if has_children {
                LastSemicolon::Required
            } else {
                LastSemicolon::Optional
            },
            cx,
        )?;
        if has_declarations && has_children {
            dest.blank_line()?;
        }
        if has_children {
            write_rule_list(ast, children, dest, cx)?;
        }
        Ok(())
    })
}

fn write_child_rule_block<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rule: CssRule<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    write_block(dest, |dest| {
        write_rule_list(ast, rule.children(ast), dest, cx)
    })
}

fn write_property_block<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rule: CssRule<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let block = rule
        .declarations(ast)
        .expect("a declaration owner remains bound to its block");
    write_block(dest, |dest| {
        write_property_declarations(ast, block, dest, LastSemicolon::Optional, cx).map(|_| ())
    })
}

fn write_property_declarations<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    block: DeclarationBlock<'ast>,
    dest: &mut PrinterT,
    last_semicolon: LastSemicolon,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> Result<bool, fmt::Error> {
    let mut declarations = block
        .declarations(ast)
        .iter()
        .zip(block.important(ast))
        .filter_map(|(payload, important)| {
            payload
                .as_property(ast)
                .filter(|declaration| !declaration.is_tombstone(ast))
                .map(|declaration| (declaration, *important))
        });
    let mut wrote_declaration = false;
    let mut current = declarations.next();
    while let Some((declaration, important)) = current {
        wrote_declaration = true;
        declaration.to_css(dest, cx)?;
        if important {
            dest.write_str(" !important")?;
        }
        current = declarations.next();
        if current.is_some() {
            dest.write_char(';')?;
            dest.new_line()?;
        } else {
            write_last_semicolon(last_semicolon, dest)?;
        }
    }
    Ok(wrote_declaration)
}

fn write_named_property_block<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rule: CssRule<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
    kind: NamedKind,
) -> fmt::Result {
    let block = rule
        .declarations(ast)
        .expect("a named descriptor owner remains bound to its block");
    write_block(dest, |dest| {
        let values = block.declarations(ast);
        for (index, payload) in values.iter().enumerate() {
            match (kind, payload.data(ast)) {
                (NamedKind::FontFace, DeclarationPayloadData::FontFace(value)) => {
                    write_named_property(value, ast, dest, cx)?;
                }
                (NamedKind::FontPalette, DeclarationPayloadData::FontPaletteValues(value)) => {
                    write_named_property(value, ast, dest, cx)?;
                }
                (NamedKind::ViewTransition, DeclarationPayloadData::ViewTransition(value)) => {
                    write_named_property(value, ast, dest, cx)?;
                }
                _ => panic!("a named descriptor block contains another payload family"),
            }
            dest.semicolon(index + 1 < values.len())?;
            if index + 1 < values.len() {
                dest.new_line()?;
            }
        }
        Ok(())
    })
}

fn write_font_feature_subrule<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rule: CssRule<'ast>,
    payload: FontFeatureSubruleRule<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_char('@')?;
    payload.name(ast).to_css(dest, cx)?;
    let block = rule
        .declarations(ast)
        .expect("a font feature subrule owns a descriptor block");
    write_block(dest, |dest| {
        let values = block.declarations(ast);
        for (index, payload) in values.iter().enumerate() {
            let DeclarationPayloadData::FontFeature(value) = payload.data(ast) else {
                panic!("a font feature block contains another payload family")
            };
            value.to_css(dest, cx)?;
            if index + 1 < values.len() {
                dest.write_char(';')?;
                dest.new_line()?;
            }
        }
        Ok(())
    })
}

fn write_page_rule<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rule: CssRule<'ast>,
    payload: PageRule<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("@page")?;
    let selectors = payload.selectors(ast);
    if !selectors.is_empty() {
        dest.write_char(' ')?;
        write_comma_separated(selectors, dest, cx)?;
    }
    let parent_block = rule
        .declarations(ast)
        .expect("a page rule owns its initial declaration block");
    write_block(dest, |dest| {
        let children = rule.children(ast);
        let mut declaration_segment_count = usize::from(block_is_non_empty(ast, parent_block));
        let mut margin_count = 0_usize;
        for child in children {
            match child.payload(ast).data(ast) {
                CssRulePayloadData::PageDeclarations(_) => {
                    let block = child
                        .declarations(ast)
                        .expect("a page declaration segment owns a block");
                    declaration_segment_count += usize::from(block_is_non_empty(ast, block));
                }
                CssRulePayloadData::PageMargin(_) => margin_count += 1,
                _ => panic!("a page child list contains another rule family"),
            }
        }

        let mut written_segments = 0_usize;
        if block_is_non_empty(ast, parent_block) {
            written_segments += 1;
            write_property_declarations(
                ast,
                parent_block,
                dest,
                if written_segments < declaration_segment_count || margin_count > 0 {
                    LastSemicolon::Required
                } else {
                    LastSemicolon::Optional
                },
                cx,
            )?;
        }
        for child in children {
            if matches!(
                child.payload(ast).data(ast),
                CssRulePayloadData::PageDeclarations(_)
            ) {
                let block = child
                    .declarations(ast)
                    .expect("a page declaration segment owns a block");
                if block_is_non_empty(ast, block) {
                    if written_segments > 0 {
                        dest.new_line()?;
                    }
                    written_segments += 1;
                    write_property_declarations(
                        ast,
                        block,
                        dest,
                        if written_segments < declaration_segment_count || margin_count > 0 {
                            LastSemicolon::Required
                        } else {
                            LastSemicolon::Optional
                        },
                        cx,
                    )?;
                }
            }
        }
        let mut written_margins = 0_usize;
        for child in children {
            if matches!(
                child.payload(ast).data(ast),
                CssRulePayloadData::PageMargin(_)
            ) {
                if written_segments > 0 || written_margins > 0 {
                    dest.blank_line()?;
                }
                written_margins += 1;
                write_rule(ast, *child, dest, LastSemicolon::Optional, cx)?;
            }
        }
        Ok(())
    })
}

fn write_property_rule<'ast, 'ghost, PrinterT: PrinterTrait>(
    ast: &Ast<'ast>,
    rule: CssRule<'ast>,
    payload: PropertyRule<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("@property ")?;
    dest.write_str("--")?;
    serialize_name(
        payload
            .name(ast)
            .strip_prefix("--")
            .unwrap_or(payload.name(ast)),
        dest,
    )?;
    write_block(dest, |dest| {
        let block = rule
            .declarations(ast)
            .expect("a property rule owns a descriptor block");
        let values = block.declarations(ast);
        for (index, payload) in values.iter().enumerate() {
            let DeclarationPayloadData::PropertyRule(descriptor) = payload.data(ast) else {
                panic!("a property block contains another payload family")
            };
            descriptor.to_css(dest, cx)?;
            dest.semicolon(index + 1 < values.len())?;
            if index + 1 < values.len() {
                dest.new_line()?;
            }
        }
        Ok(())
    })
}

fn block_is_non_empty<'ast>(ast: &Ast<'ast>, block: DeclarationBlock<'ast>) -> bool {
    block.declarations(ast).iter().any(|payload| {
        payload
            .as_property(ast)
            .is_some_and(|declaration| !declaration.is_tombstone(ast))
    })
}

#[derive(Clone, Copy)]
enum NamedKind {
    FontFace,
    FontPalette,
    ViewTransition,
}

fn rule_without_block<'ast>(ast: &Ast<'ast>, payload: CssRulePayload<'ast>) -> bool {
    matches!(
        payload.data(ast),
        CssRulePayloadData::Charset(_)
            | CssRulePayloadData::Import(_)
            | CssRulePayloadData::Namespace(_)
            | CssRulePayloadData::LayerStatement(_)
    )
}

fn write_block<PrinterT: PrinterTrait>(
    dest: &mut PrinterT,
    callback: impl FnOnce(&mut PrinterT) -> fmt::Result,
) -> fmt::Result {
    dest.whitespace()?;
    dest.write_char('{')?;
    dest.indent();
    dest.new_line()?;
    callback(dest)?;
    dest.dedent();
    dest.new_line()?;
    dest.write_char('}')
}

fn write_last_semicolon<PrinterT: PrinterTrait>(
    last_semicolon: LastSemicolon,
    dest: &mut PrinterT,
) -> fmt::Result {
    match last_semicolon {
        LastSemicolon::Optional => dest.semicolon(false),
        LastSemicolon::Required => dest.write_char(';'),
    }
}

fn write_named_property<'ast, 'ghost, PrinterT: PrinterTrait, T>(
    value: T,
    ast: &Ast<'ast>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result
where
    T: NamedProperty<'ast> + ToCss<'ghost> + Copy,
{
    serialize_name(value.css_name(ast), dest)?;
    dest.write_char(':')?;
    dest.whitespace()?;
    value.to_css(dest, cx)
}

fn write_layer_name<PrinterT: PrinterTrait>(name: &[&str], dest: &mut PrinterT) -> fmt::Result {
    for (index, part) in name.iter().enumerate() {
        if index > 0 {
            dest.write_char('.')?;
        }
        serialize_identifier(part, dest)?;
    }
    Ok(())
}

fn write_comma_separated<'ast, 'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost>>(
    values: &[T],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}
