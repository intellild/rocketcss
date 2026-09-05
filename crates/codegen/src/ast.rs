//! Streaming serialization for the persistent AST.

use crate::{prelude::*, rules::NamedProperty};
use rocketcss_ast::{
    AstContext, ConcreteDeclarationBlockId as DeclarationBlockId, ConcreteRuleId as RuleId,
    CssRulePayload, DeclarationPayload, FontFeatureSubrulePayload, PageRulePayload,
    PropertyRuleDescriptor, PropertyRulePayload, RuleListId, RuleListIter, RuleRecord,
};

#[derive(Clone, Copy)]
enum LastSemicolon {
    Optional,
    Required,
}

impl<'ghost> ToCss<'ghost> for AstContext<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let cx = ToCssContext::with_ast(cx.token(), self);
        let writer = AstWriter(self);
        for (index, comment) in self.license_comments().iter().enumerate() {
            dest.write_str("/*")?;
            dest.write_str(comment)?;
            dest.write_str("*/")?;
            if index + 1 < self.license_comments().len() || !writer.root_is_empty() {
                dest.new_line()?;
            }
        }
        writer.write_rule_list(self.stylesheet().root_rules(), dest, &cx)?;
        if !writer.root_is_empty() {
            dest.new_line()?;
        }
        Ok(())
    }
}

struct AstWriter<'comp, 'ast>(&'comp AstContext<'ast>);

impl<'ast> std::ops::Deref for AstWriter<'_, 'ast> {
    type Target = AstContext<'ast>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'ast> AstWriter<'_, 'ast> {
    fn root_is_empty(&self) -> bool {
        self.rule_list(self.stylesheet().root_rules())
            .is_none_or(|list| list.live_len() == 0)
    }

    fn write_rule_list<'ghost, PrinterT: PrinterTrait>(
        &self,
        list: RuleListId<'ast>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let mut first = true;
        let mut last_without_block = false;
        let mut rules = self
            .rules_in_list(list)
            .expect("codegen only traverses a validated rule list");
        let mut current = next_visible_rule(self.0, &mut rules);
        while let Some((id, rule)) = current {
            current = next_visible_rule(self.0, &mut rules);
            if !first {
                if !last_without_block || !rule_without_block(rule.payload()) {
                    dest.blank_line()?;
                } else {
                    dest.new_line()?;
                }
            }
            first = false;
            let last_semicolon = if current.is_some() {
                LastSemicolon::Required
            } else {
                LastSemicolon::Optional
            };
            self.write_rule(id, rule, dest, last_semicolon, cx)?;
            last_without_block = rule_without_block(rule.payload());
        }
        Ok(())
    }

    fn write_rule<'ghost, PrinterT: PrinterTrait>(
        &self,
        id: RuleId<'ast>,
        rule: &RuleRecord<'ast, CssRulePayload<'ast>>,
        dest: &mut PrinterT,
        last_semicolon: LastSemicolon,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match rule.payload() {
            CssRulePayload::Style(payload) => {
                let selector = self
                    .selector_value(payload.selector_value)
                    .expect("a style selector value remains resolvable");
                selector.selectors().to_css(dest, cx)?;
                self.write_style_body(rule, dest, cx)
            }
            CssRulePayload::Media(payload) => {
                dest.write_str("@media ")?;
                payload.query.to_css(dest, cx)?;
                self.write_child_rule_block(rule, dest, cx)
            }
            CssRulePayload::Supports(payload) => {
                dest.write_str("@supports ")?;
                payload.condition.to_css(dest, cx)?;
                self.write_child_rule_block(rule, dest, cx)
            }
            CssRulePayload::StartingStyle(_) => {
                dest.write_str("@starting-style")?;
                self.write_child_rule_block(rule, dest, cx)
            }
            CssRulePayload::LayerStatement(payload) => {
                dest.write_str("@layer ")?;
                for (index, name) in self.vec(payload.names).iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    write_layer_name(self.vec(*name), dest)?;
                }
                dest.write_char(';')
            }
            CssRulePayload::LayerBlock(payload) => {
                dest.write_str("@layer")?;
                if let Some(name) = &payload.name {
                    dest.write_char(' ')?;
                    write_layer_name(self.vec(*name), dest)?;
                }
                self.write_child_rule_block(rule, dest, cx)
            }
            CssRulePayload::Container(payload) => {
                dest.write_str("@container")?;
                if let Some(name) = payload.name {
                    dest.write_char(' ')?;
                    serialize_identifier(name, dest)?;
                }
                if let Some(condition) = &payload.condition {
                    dest.write_char(' ')?;
                    condition.to_css(dest, cx)?;
                }
                self.write_child_rule_block(rule, dest, cx)
            }
            CssRulePayload::Scope(payload) => {
                dest.write_str("@scope")?;
                if let Some(start) = &payload.scope_start {
                    dest.write_str(" (")?;
                    start.to_css(dest, cx)?;
                    dest.write_char(')')?;
                }
                if let Some(end) = &payload.scope_end {
                    dest.write_str(" to (")?;
                    end.to_css(dest, cx)?;
                    dest.write_char(')')?;
                }
                self.write_child_rule_block(rule, dest, cx)
            }
            CssRulePayload::MozDocument(_) => {
                dest.write_str("@-moz-document url-prefix()")?;
                self.write_child_rule_block(rule, dest, cx)
            }
            CssRulePayload::Unknown(payload) => {
                dest.write_char('@')?;
                serialize_identifier(payload.name, dest)?;
                if !payload.prelude.is_empty() {
                    dest.write_char(' ')?;
                    crate::token::write_token_list_without_outer_whitespace(
                        self.vec(payload.prelude),
                        dest,
                        cx,
                    )?;
                }
                if let Some(block) = &payload.block {
                    write_block(dest, |dest| {
                        crate::token::write_token_list_without_outer_whitespace(
                            self.vec(*block),
                            dest,
                            cx,
                        )
                    })
                } else {
                    dest.write_char(';')
                }
            }
            CssRulePayload::CounterStyle(payload) => {
                dest.write_str("@counter-style ")?;
                serialize_identifier(payload.name, dest)?;
                self.write_property_block(rule, dest, cx)
            }
            CssRulePayload::Viewport(payload) => {
                dest.write_char('@')?;
                payload.vendor_prefix.to_css(dest, cx)?;
                dest.write_str("viewport")?;
                self.write_property_block(rule, dest, cx)
            }
            CssRulePayload::PositionTry(payload) => {
                dest.write_str("@position-try ")?;
                dest.write_str("--")?;
                serialize_name(
                    payload.name.strip_prefix("--").unwrap_or(payload.name),
                    dest,
                )?;
                self.write_property_block(rule, dest, cx)
            }
            CssRulePayload::FontFace(_) => {
                dest.write_str("@font-face")?;
                self.write_named_property_block(id, dest, cx, NamedKind::FontFace)
            }
            CssRulePayload::FontPaletteValues(payload) => {
                dest.write_str("@font-palette-values ")?;
                serialize_identifier(payload.name, dest)?;
                self.write_named_property_block(id, dest, cx, NamedKind::FontPalette)
            }
            CssRulePayload::ViewTransition(_) => {
                dest.write_str("@view-transition")?;
                self.write_named_property_block(id, dest, cx, NamedKind::ViewTransition)
            }
            CssRulePayload::Import(payload) => payload.to_css(dest, cx),
            CssRulePayload::Charset(payload) => payload.to_css(dest, cx),
            CssRulePayload::Namespace(payload) => payload.to_css(dest, cx),
            CssRulePayload::CustomMedia(payload) => payload.to_css(dest, cx),
            CssRulePayload::Keyframes(payload) => {
                dest.write_char('@')?;
                payload.vendor_prefix.to_css(dest, cx)?;
                dest.write_str("keyframes ")?;
                payload.name.to_css(dest, cx)?;
                let child_list = rule.child_list();
                if child_list.is_none_or(|list| {
                    self.rule_list(list)
                        .is_none_or(|record| record.live_len() == 0)
                }) {
                    dest.whitespace()?;
                    dest.write_char('{')?;
                    dest.new_line()?;
                    dest.write_char('}')
                } else {
                    self.write_child_rule_block(rule, dest, cx)
                }
            }
            CssRulePayload::Keyframe(payload) => {
                write_comma_separated(self.vec(payload.selectors), dest, cx)?;
                self.write_property_block(rule, dest, cx)
            }
            CssRulePayload::Page(payload) => self.write_page_rule(rule, payload, dest, cx),
            CssRulePayload::PageMargin(payload) => {
                dest.write_char('@')?;
                payload.margin_box.to_css(dest, cx)?;
                self.write_property_block(rule, dest, cx)
            }
            CssRulePayload::PageDeclarations(_) => self
                .write_property_declarations(
                    rule.declaration_block()
                        .expect("a page declaration segment owns a block"),
                    dest,
                    last_semicolon,
                    cx,
                )
                .map(|_| ()),
            CssRulePayload::Nesting(payload) => {
                dest.write_str("@nest ")?;
                self.selector_value(payload.selector_value)
                    .expect("a nesting selector value remains resolvable")
                    .selectors()
                    .to_css(dest, cx)?;
                self.write_style_body(rule, dest, cx)
            }
            CssRulePayload::FontFeatureValues(payload) => {
                dest.write_str("@font-feature-values ")?;
                write_comma_separated(self.vec(payload.name), dest, cx)?;
                self.write_child_rule_block(rule, dest, cx)
            }
            CssRulePayload::FontFeatureSubrule(payload) => {
                self.write_font_feature_subrule(id, payload, dest, cx)
            }
            CssRulePayload::Property(payload) => self.write_property_rule(id, payload, dest, cx),
            CssRulePayload::NestedDeclarations(_) => self
                .write_property_declarations(
                    rule.declaration_block()
                        .expect("a nested declaration segment owns a block"),
                    dest,
                    last_semicolon,
                    cx,
                )
                .map(|_| ()),
        }
    }

    fn write_style_body<'ghost, PrinterT: PrinterTrait>(
        &self,
        rule: &RuleRecord<'ast, CssRulePayload<'ast>>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let children = rule.child_list();
        let has_children = children.is_some_and(|list| {
            self.rule_list(list)
                .is_some_and(|record| record.live_len() > 0)
        });
        let block = rule
            .declaration_block()
            .expect("a style syntax position owns a declaration block");
        write_block(dest, |dest| {
            let has_declarations = self.write_property_declarations(
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
            if let Some(children) = children {
                self.write_rule_list(children, dest, cx)?;
            }
            Ok(())
        })
    }

    fn write_child_rule_block<'ghost, PrinterT: PrinterTrait>(
        &self,
        rule: &RuleRecord<'ast, CssRulePayload<'ast>>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let child_list = rule.child_list();
        write_block(dest, |dest| {
            if let Some(child_list) = child_list {
                self.write_rule_list(child_list, dest, cx)?;
            }
            Ok(())
        })
    }

    fn write_property_block<'ghost, PrinterT: PrinterTrait>(
        &self,
        rule: &RuleRecord<'ast, CssRulePayload<'ast>>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let block = rule
            .declaration_block()
            .expect("a declaration owner remains bound to its block");
        write_block(dest, |dest| {
            self.write_property_declarations(block, dest, LastSemicolon::Optional, cx)
                .map(|_| ())
        })
    }

    fn write_property_declarations<'ghost, PrinterT: PrinterTrait>(
        &self,
        block: DeclarationBlockId<'ast>,
        dest: &mut PrinterT,
        last_semicolon: LastSemicolon,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> Result<bool, fmt::Error> {
        let mut declarations = self
            .declarations_in_block(block)
            .expect("codegen only reads a validated declaration block")
            .filter(|record| {
                matches!(record.payload(), DeclarationPayload::Property(value) if !value.is_tombstone())
            })
            .peekable();
        let mut wrote_declaration = false;
        while let Some(record) = declarations.next() {
            wrote_declaration = true;
            let DeclarationPayload::Property(declaration) = record.payload() else {
                panic!("a property block contains a descriptor payload")
            };
            declaration.to_css(dest, cx)?;
            if record.is_important() {
                dest.write_str(" !important")?;
            }
            let has_next = declarations.peek().is_some();
            if has_next {
                dest.write_char(';')?;
                dest.new_line()?;
            } else {
                write_last_semicolon(last_semicolon, dest)?;
            }
        }
        Ok(wrote_declaration)
    }

    fn write_named_property_block<'ghost, PrinterT: PrinterTrait>(
        &self,
        id: RuleId<'ast>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
        kind: NamedKind,
    ) -> fmt::Result {
        let block = self
            .rule(id)
            .and_then(|rule| rule.declaration_block())
            .expect("a named descriptor owner remains bound to its block");
        write_block(dest, |dest| {
            let mut values = self
                .declarations_in_block(block)
                .expect("codegen only reads a validated descriptor block")
                .peekable();
            while let Some(record) = values.next() {
                match (kind, record.payload()) {
                    (NamedKind::FontFace, DeclarationPayload::FontFace(value)) => {
                        write_named_property(value, dest, cx)?;
                    }
                    (NamedKind::FontPalette, DeclarationPayload::FontPaletteValues(value)) => {
                        write_named_property(value, dest, cx)?;
                    }
                    (NamedKind::ViewTransition, DeclarationPayload::ViewTransition(value)) => {
                        write_named_property(value, dest, cx)?;
                    }
                    _ => panic!("a named descriptor block contains another payload family"),
                }
                dest.semicolon(values.peek().is_some())?;
                if values.peek().is_some() {
                    dest.new_line()?;
                }
            }
            Ok(())
        })
    }

    fn write_font_feature_subrule<'ghost, PrinterT: PrinterTrait>(
        &self,
        id: RuleId<'ast>,
        payload: &FontFeatureSubrulePayload,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_char('@')?;
        payload.name.to_css(dest, cx)?;
        let block = self
            .rule(id)
            .and_then(|rule| rule.declaration_block())
            .expect("a font feature subrule owns a descriptor block");
        write_block(dest, |dest| {
            let mut values = self
                .declarations_in_block(block)
                .expect("codegen only reads a validated descriptor block")
                .peekable();
            while let Some(record) = values.next() {
                let DeclarationPayload::FontFeature(value) = record.payload() else {
                    panic!("a font feature block contains another payload family")
                };
                value.to_css(dest, cx)?;
                if values.peek().is_some() {
                    dest.write_char(';')?;
                    dest.new_line()?;
                }
            }
            Ok(())
        })
    }

    fn write_page_rule<'ghost, PrinterT: PrinterTrait>(
        &self,
        rule: &RuleRecord<'ast, CssRulePayload<'ast>>,
        payload: &PageRulePayload<'_>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@page")?;
        if !payload.selectors.is_empty() {
            dest.write_char(' ')?;
            write_comma_separated(self.vec(payload.selectors), dest, cx)?;
        }
        let parent_block = rule
            .declaration_block()
            .expect("a page rule owns its initial declaration block");
        write_block(dest, |dest| {
            let child_list = rule.child_list();
            let mut declaration_segment_count = usize::from(self.block_is_non_empty(parent_block));
            let mut margin_count = 0_usize;
            if let Some(children) = child_list {
                for (_, child) in self
                    .rules_in_list(children)
                    .expect("the page child list remains valid")
                {
                    match child.payload() {
                        CssRulePayload::PageDeclarations(_) => {
                            let block = child
                                .declaration_block()
                                .expect("a page declaration segment owns a block");
                            declaration_segment_count +=
                                usize::from(self.block_is_non_empty(block));
                        }
                        CssRulePayload::PageMargin(_) => margin_count += 1,
                        _ => panic!("a page child list contains another rule family"),
                    }
                }
            }

            let mut written_segments = 0_usize;
            if self.block_is_non_empty(parent_block) {
                written_segments += 1;
                self.write_property_declarations(
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
            if let Some(children) = child_list {
                for (_, child) in self
                    .rules_in_list(children)
                    .expect("the page child list remains valid")
                {
                    if matches!(child.payload(), CssRulePayload::PageDeclarations(_)) {
                        let block = child
                            .declaration_block()
                            .expect("a page declaration segment owns a block");
                        if self.block_is_non_empty(block) {
                            if written_segments > 0 {
                                dest.new_line()?;
                            }
                            written_segments += 1;
                            self.write_property_declarations(
                                block,
                                dest,
                                if written_segments < declaration_segment_count || margin_count > 0
                                {
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
                for (child_id, child) in self
                    .rules_in_list(children)
                    .expect("the page child list remains valid")
                {
                    if matches!(child.payload(), CssRulePayload::PageMargin(_)) {
                        if written_segments > 0 || written_margins > 0 {
                            dest.blank_line()?;
                        }
                        written_margins += 1;
                        self.write_rule(child_id, child, dest, LastSemicolon::Optional, cx)?;
                    }
                }
            }
            Ok(())
        })
    }

    fn write_property_rule<'ghost, PrinterT: PrinterTrait>(
        &self,
        _id: RuleId<'ast>,
        payload: &PropertyRulePayload<'ast>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@property ")?;
        dest.write_str("--")?;
        serialize_name(
            payload.name.strip_prefix("--").unwrap_or(payload.name),
            dest,
        )?;
        write_block(dest, |dest| {
            if let Some(syntax) = payload.syntax {
                dest.write_str("syntax")?;
                dest.delim(Delimiter::Colon)?;
                let PropertyRuleDescriptor::Syntax(value) = self.property_descriptor(syntax) else {
                    panic!("the resolved syntax id references another descriptor")
                };
                let syntax = value.to_css_string(dest.options(), cx)?;
                serialize_string(&syntax, dest)?;
            }
            if let Some(inherits) = payload.inherits {
                if payload.syntax.is_some() {
                    dest.write_char(';')?;
                    dest.new_line()?;
                }
                dest.write_str("inherits")?;
                dest.delim(Delimiter::Colon)?;
                let PropertyRuleDescriptor::Inherits(value) = self.property_descriptor(inherits)
                else {
                    panic!("the resolved inherits id references another descriptor")
                };
                dest.write_str(if *value { "true" } else { "false" })?;
            }
            if let Some(initial_value) = payload.initial_value {
                if payload.syntax.is_some() || payload.inherits.is_some() {
                    dest.write_char(';')?;
                    dest.new_line()?;
                }
                dest.write_str("initial-value")?;
                dest.delim(Delimiter::Colon)?;
                let PropertyRuleDescriptor::InitialValue(value) =
                    self.property_descriptor(initial_value)
                else {
                    panic!("the resolved initial-value id references another descriptor")
                };
                value.to_css(dest, cx)?;
            }
            dest.semicolon(false)
        })
    }

    fn property_descriptor(
        &self,
        id: rocketcss_ast::DeclarationId<'ast>,
    ) -> &PropertyRuleDescriptor<'_> {
        let record = self
            .declaration(id)
            .expect("a resolved property descriptor remains in the declaration tape");
        let DeclarationPayload::PropertyRule(descriptor) = record.payload() else {
            panic!("a property descriptor id references another declaration family")
        };
        descriptor
    }

    fn block_is_non_empty(&self, block: DeclarationBlockId<'ast>) -> bool {
        self.declarations_in_block(block)
            .expect("codegen only reads a validated declaration block")
            .any(|record| {
                matches!(record.payload(), DeclarationPayload::Property(value) if !value.is_tombstone())
            })
    }
}

type VisibleRule<'comp, 'ast> = (RuleId<'ast>, &'comp RuleRecord<'ast, CssRulePayload<'ast>>);

fn next_visible_rule<'comp, 'ast>(
    compilation: &'comp AstContext<'ast>,
    rules: &mut RuleListIter<'ast, 'comp, CssRulePayload<'ast>>,
) -> Option<VisibleRule<'comp, 'ast>> {
    for (id, rule) in rules {
        if let CssRulePayload::Style(style) = rule.payload() {
            let selector = compilation
                .selector_value(style.selector_value)
                .expect("a style selector value remains resolvable");
            if compilation
                .vec(*selector.selectors())
                .iter()
                .all(Selector::is_tombstone)
            {
                continue;
            }
        }
        return Some((id, rule));
    }
    None
}

#[derive(Clone, Copy)]
enum NamedKind {
    FontFace,
    FontPalette,
    ViewTransition,
}

fn rule_without_block(payload: &CssRulePayload<'_>) -> bool {
    matches!(
        payload,
        CssRulePayload::Charset(_)
            | CssRulePayload::Import(_)
            | CssRulePayload::Namespace(_)
            | CssRulePayload::LayerStatement(_)
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

fn write_named_property<'ghost, PrinterT: PrinterTrait, T>(
    value: &T,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    T: NamedProperty + ToCss<'ghost>,
{
    serialize_name(value.css_name(cx.ast_context()), dest)?;
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

fn write_comma_separated<'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost>>(
    values: &[T],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}
