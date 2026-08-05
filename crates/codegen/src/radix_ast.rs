//! Streaming serialization for the flat Radix AST.

use crate::{prelude::*, rules::NamedProperty};
use rocketcss_ast::{
    CssDeclaration, CssDeclarationBlockId as DeclarationBlockId, CssRule, CssRuleId as RuleId,
    FontFeatureSubrule, PageRule, PropertyRule, PropertyRuleDescriptor, RuleRecord, RuleTreeEvent,
    RuleTreeEventIter, StyleSheet,
};

#[derive(Clone, Copy)]
enum LastSemicolon {
    Optional,
    Required,
}

impl<'ghost> ToCss<'ghost> for StyleSheet<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let writer = RadixWriter(self);
        let mut cursor = RuleTreeCursor::new(self.rule_tree_events());
        let first_root = writer.next_visible_rule(None, &mut cursor);
        let has_root_rules = first_root.is_some();
        for (index, comment) in self.license_comments().iter().enumerate() {
            dest.write_str("/*")?;
            dest.write_str(comment)?;
            dest.write_str("*/")?;
            if index + 1 < self.license_comments().len() || has_root_rules {
                dest.new_line()?;
            }
        }
        writer.write_rule_list(None, first_root, &mut cursor, dest, cx)?;
        debug_assert!(cursor.events.peek().is_none());
        if has_root_rules {
            dest.new_line()?;
        }
        Ok(())
    }
}

struct RadixWriter<'comp, 'ast>(&'comp StyleSheet<'ast>);

#[derive(Clone, Copy)]
struct VisibleRule<'comp, 'ast> {
    event: RuleTreeEvent<CssRule<'ast>>,
    rule: &'comp RuleRecord<CssRule<'ast>>,
}

struct RuleTreeCursor<'comp, 'ast> {
    events: std::iter::Peekable<RuleTreeEventIter<'comp, 'ast, CssRule<'ast>>>,
}

impl<'comp, 'ast> RuleTreeCursor<'comp, 'ast> {
    fn new(events: RuleTreeEventIter<'comp, 'ast, CssRule<'ast>>) -> Self {
        Self {
            events: events.peekable(),
        }
    }

    fn next_child(&mut self, parent: Option<RuleId<'ast>>) -> Option<RuleTreeEvent<CssRule<'ast>>> {
        (self.events.peek()?.parent() == parent).then(|| self.events.next().unwrap())
    }

    fn skip_children(&mut self, parent: RuleId<'ast>) {
        while let Some(child) = self.next_child(Some(parent)) {
            self.skip_children(child.rule());
        }
    }
}

impl<'ast> std::ops::Deref for RadixWriter<'_, 'ast> {
    type Target = StyleSheet<'ast>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'comp, 'ast> RadixWriter<'comp, 'ast> {
    fn write_rule_list<'ghost, PrinterT: PrinterTrait>(
        &self,
        parent: Option<RuleId<'ast>>,
        first_rule: Option<VisibleRule<'comp, 'ast>>,
        cursor: &mut RuleTreeCursor<'comp, 'ast>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let mut first = true;
        let mut last_without_block = false;
        let mut current = first_rule.or_else(|| self.next_visible_rule(parent, cursor));
        while let Some(visible) = current {
            if !first {
                if !last_without_block || !rule_without_block(visible.rule.payload()) {
                    dest.blank_line()?;
                } else {
                    dest.new_line()?;
                }
            }
            first = false;
            let mut wrote_unterminated_declaration = false;
            self.write_rule(
                visible,
                cursor,
                dest,
                LastSemicolon::Optional,
                &mut wrote_unterminated_declaration,
                cx,
            )?;
            last_without_block = rule_without_block(visible.rule.payload());
            current = self.next_visible_rule(parent, cursor);
            if current.is_some() && wrote_unterminated_declaration {
                dest.write_char(';')?;
            }
        }
        Ok(())
    }

    fn next_visible_rule(
        &self,
        parent: Option<RuleId<'ast>>,
        cursor: &mut RuleTreeCursor<'comp, 'ast>,
    ) -> Option<VisibleRule<'comp, 'ast>> {
        while let Some(event) = cursor.next_child(parent) {
            let rule = self
                .0
                .rule(event.rule())
                .expect("a tree event's live rule remains resolvable");
            if let CssRule::Style(style) = rule.payload() {
                let selector = self
                    .selector_value(style.selector_value)
                    .expect("a style selector value remains resolvable");
                if selector.selectors().iter().all(Selector::is_tombstone) {
                    cursor.skip_children(event.rule());
                    continue;
                }
            }
            return Some(VisibleRule { event, rule });
        }
        None
    }

    fn write_rule<'ghost, PrinterT: PrinterTrait>(
        &self,
        visible: VisibleRule<'comp, 'ast>,
        cursor: &mut RuleTreeCursor<'comp, 'ast>,
        dest: &mut PrinterT,
        last_semicolon: LastSemicolon,
        wrote_unterminated_declaration: &mut bool,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let event = visible.event;
        let id = event.rule();
        let rule = visible.rule;
        match rule.payload() {
            CssRule::Style(payload) => {
                let selector = self
                    .selector_value(payload.selector_value)
                    .expect("a style selector value remains resolvable");
                selector.selectors().to_css(dest, cx)?;
                self.write_style_body(event, rule, cursor, dest, cx)
            }
            CssRule::Media(payload) => {
                dest.write_str("@media ")?;
                payload.query.to_css(dest, cx)?;
                self.write_child_rule_block(event, cursor, dest, cx)
            }
            CssRule::Supports(payload) => {
                dest.write_str("@supports ")?;
                payload.condition.to_css(dest, cx)?;
                self.write_child_rule_block(event, cursor, dest, cx)
            }
            CssRule::StartingStyle(_) => {
                dest.write_str("@starting-style")?;
                self.write_child_rule_block(event, cursor, dest, cx)
            }
            CssRule::LayerStatement(payload) => {
                dest.write_str("@layer ")?;
                for (index, name) in payload.names.iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    write_layer_name(name, dest)?;
                }
                dest.write_char(';')
            }
            CssRule::LayerBlock(payload) => {
                dest.write_str("@layer")?;
                if let Some(name) = &payload.name {
                    dest.write_char(' ')?;
                    write_layer_name(name, dest)?;
                }
                self.write_child_rule_block(event, cursor, dest, cx)
            }
            CssRule::Container(payload) => {
                dest.write_str("@container")?;
                if let Some(name) = payload.name {
                    dest.write_char(' ')?;
                    serialize_identifier(name, dest)?;
                }
                if let Some(condition) = &payload.condition {
                    dest.write_char(' ')?;
                    condition.to_css(dest, cx)?;
                }
                self.write_child_rule_block(event, cursor, dest, cx)
            }
            CssRule::Scope(payload) => {
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
                self.write_child_rule_block(event, cursor, dest, cx)
            }
            CssRule::MozDocument(_) => {
                dest.write_str("@-moz-document url-prefix()")?;
                self.write_child_rule_block(event, cursor, dest, cx)
            }
            CssRule::Unknown(payload) => {
                dest.write_char('@')?;
                serialize_identifier(payload.name, dest)?;
                if !payload.prelude.is_empty() {
                    dest.write_char(' ')?;
                    crate::token::write_token_list_without_outer_whitespace(
                        &payload.prelude,
                        dest,
                        cx,
                    )?;
                }
                if let Some(block) = &payload.block {
                    write_block(dest, |dest| {
                        crate::token::write_token_list_without_outer_whitespace(block, dest, cx)
                    })
                } else {
                    dest.write_char(';')
                }
            }
            CssRule::CounterStyle(payload) => {
                dest.write_str("@counter-style ")?;
                serialize_identifier(payload.name, dest)?;
                self.write_property_block(rule, dest, cx)
            }
            CssRule::Viewport(payload) => {
                dest.write_char('@')?;
                payload.vendor_prefix.to_css(dest, cx)?;
                dest.write_str("viewport")?;
                self.write_property_block(rule, dest, cx)
            }
            CssRule::PositionTry(payload) => {
                dest.write_str("@position-try ")?;
                dest.write_str("--")?;
                serialize_name(
                    payload.name.strip_prefix("--").unwrap_or(payload.name),
                    dest,
                )?;
                self.write_property_block(rule, dest, cx)
            }
            CssRule::FontFace(_) => {
                dest.write_str("@font-face")?;
                self.write_named_property_block(id, dest, cx, NamedKind::FontFace)
            }
            CssRule::FontPaletteValues(payload) => {
                dest.write_str("@font-palette-values ")?;
                serialize_identifier(payload.name, dest)?;
                self.write_named_property_block(id, dest, cx, NamedKind::FontPalette)
            }
            CssRule::ViewTransition(_) => {
                dest.write_str("@view-transition")?;
                self.write_named_property_block(id, dest, cx, NamedKind::ViewTransition)
            }
            CssRule::Import(payload) => payload.to_css(dest, cx),
            CssRule::Charset(payload) => payload.to_css(dest, cx),
            CssRule::Namespace(payload) => payload.to_css(dest, cx),
            CssRule::CustomMedia(payload) => payload.to_css(dest, cx),
            CssRule::Keyframes(payload) => {
                dest.write_char('@')?;
                payload.vendor_prefix.to_css(dest, cx)?;
                dest.write_str("keyframes ")?;
                payload.name.to_css(dest, cx)?;
                if !event.has_children() {
                    dest.whitespace()?;
                    dest.write_char('{')?;
                    dest.new_line()?;
                    dest.write_char('}')
                } else {
                    self.write_child_rule_block(event, cursor, dest, cx)
                }
            }
            CssRule::Keyframe(payload) => {
                write_comma_separated(&payload.selectors, dest, cx)?;
                self.write_property_block(rule, dest, cx)
            }
            CssRule::Page(payload) => self.write_page_rule(event, rule, payload, cursor, dest, cx),
            CssRule::PageMargin(payload) => {
                dest.write_char('@')?;
                payload.margin_box.to_css(dest, cx)?;
                self.write_property_block(rule, dest, cx)
            }
            CssRule::PageDeclarations(_) => {
                let wrote_declaration = self.write_property_declarations(
                    rule.declaration_block()
                        .expect("a page declaration segment owns a block"),
                    dest,
                    last_semicolon,
                    cx,
                )?;
                *wrote_unterminated_declaration = wrote_declaration
                    && matches!(last_semicolon, LastSemicolon::Optional)
                    && !dest.prettify();
                Ok(())
            }
            CssRule::Nesting(payload) => {
                dest.write_str("@nest ")?;
                self.selector_value(payload.selector_value)
                    .expect("a nesting selector value remains resolvable")
                    .selectors()
                    .to_css(dest, cx)?;
                self.write_style_body(event, rule, cursor, dest, cx)
            }
            CssRule::FontFeatureValues(payload) => {
                dest.write_str("@font-feature-values ")?;
                write_comma_separated(&payload.name, dest, cx)?;
                self.write_child_rule_block(event, cursor, dest, cx)
            }
            CssRule::FontFeatureSubrule(payload) => {
                self.write_font_feature_subrule(id, payload, dest, cx)
            }
            CssRule::Property(payload) => self.write_property_rule(id, payload, dest, cx),
            CssRule::NestedDeclarations(_) => {
                let wrote_declaration = self.write_property_declarations(
                    rule.declaration_block()
                        .expect("a nested declaration segment owns a block"),
                    dest,
                    last_semicolon,
                    cx,
                )?;
                *wrote_unterminated_declaration = wrote_declaration
                    && matches!(last_semicolon, LastSemicolon::Optional)
                    && !dest.prettify();
                Ok(())
            }
        }
    }

    fn write_style_body<'ghost, PrinterT: PrinterTrait>(
        &self,
        event: RuleTreeEvent<CssRule<'ast>>,
        rule: &RuleRecord<CssRule<'ast>>,
        cursor: &mut RuleTreeCursor<'comp, 'ast>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let id = event.rule();
        let has_children = event.has_children();
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
            if has_children {
                self.write_rule_list(Some(id), None, cursor, dest, cx)?;
            }
            Ok(())
        })
    }

    fn write_child_rule_block<'ghost, PrinterT: PrinterTrait>(
        &self,
        event: RuleTreeEvent<CssRule<'ast>>,
        cursor: &mut RuleTreeCursor<'comp, 'ast>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let id = event.rule();
        write_block(dest, |dest| {
            if event.has_children() {
                self.write_rule_list(Some(id), None, cursor, dest, cx)?;
            }
            Ok(())
        })
    }

    fn write_property_block<'ghost, PrinterT: PrinterTrait>(
        &self,
        rule: &RuleRecord<CssRule<'ast>>,
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
                matches!(record.payload(), CssDeclaration::Property(value) if !value.is_tombstone())
            })
            .peekable();
        let mut wrote_declaration = false;
        while let Some(record) = declarations.next() {
            wrote_declaration = true;
            let CssDeclaration::Property(declaration) = record.payload() else {
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
                    (NamedKind::FontFace, CssDeclaration::FontFace(value)) => {
                        write_named_property(value, dest, cx)?;
                    }
                    (NamedKind::FontPalette, CssDeclaration::FontPaletteValues(value)) => {
                        write_named_property(value, dest, cx)?;
                    }
                    (NamedKind::ViewTransition, CssDeclaration::ViewTransition(value)) => {
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
        payload: &FontFeatureSubrule,
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
                let CssDeclaration::FontFeature(value) = record.payload() else {
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
        event: RuleTreeEvent<CssRule<'ast>>,
        rule: &RuleRecord<CssRule<'ast>>,
        payload: &PageRule<'_>,
        cursor: &mut RuleTreeCursor<'comp, 'ast>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@page")?;
        if !payload.selectors.is_empty() {
            dest.write_char(' ')?;
            write_comma_separated(&payload.selectors, dest, cx)?;
        }
        let parent_block = rule
            .declaration_block()
            .expect("a page rule owns its initial declaration block");
        write_block(dest, |dest| {
            let mut children = std::vec::Vec::with_capacity(event.child_count() as usize);
            while let Some(child) = self.next_visible_rule(Some(event.rule()), cursor) {
                assert!(
                    !child.event.has_children(),
                    "a page child cannot own nested rules"
                );
                children.push(child);
            }
            let mut declaration_segment_count = usize::from(self.block_is_non_empty(parent_block));
            let mut margin_count = 0_usize;
            for child in &children {
                match child.rule.payload() {
                    CssRule::PageDeclarations(_) => {
                        let block = child
                            .rule
                            .declaration_block()
                            .expect("a page declaration segment owns a block");
                        declaration_segment_count += usize::from(self.block_is_non_empty(block));
                    }
                    CssRule::PageMargin(_) => margin_count += 1,
                    _ => panic!("a page child list contains another rule family"),
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
            for child in &children {
                if matches!(child.rule.payload(), CssRule::PageDeclarations(_)) {
                    let block = child
                        .rule
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
            for &child in &children {
                if matches!(child.rule.payload(), CssRule::PageMargin(_)) {
                    if written_segments > 0 || written_margins > 0 {
                        dest.blank_line()?;
                    }
                    written_margins += 1;
                    let mut ignored_unterminated_declaration = false;
                    self.write_rule(
                        child,
                        cursor,
                        dest,
                        LastSemicolon::Optional,
                        &mut ignored_unterminated_declaration,
                        cx,
                    )?;
                }
            }
            Ok(())
        })
    }

    fn write_property_rule<'ghost, PrinterT: PrinterTrait>(
        &self,
        _id: RuleId<'ast>,
        payload: &PropertyRule<'_>,
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

    fn property_descriptor(&self, id: rocketcss_ast::DeclarationId) -> &PropertyRuleDescriptor<'_> {
        let record = self
            .declaration(id)
            .expect("a resolved property descriptor remains in the declaration tape");
        let CssDeclaration::PropertyRule(descriptor) = record.payload() else {
            panic!("a property descriptor id references another declaration family")
        };
        descriptor
    }

    fn block_is_non_empty(&self, block: DeclarationBlockId<'ast>) -> bool {
        self.declarations_in_block(block)
            .expect("codegen only reads a validated declaration block")
            .any(|record| {
                matches!(record.payload(), CssDeclaration::Property(value) if !value.is_tombstone())
            })
    }
}

#[derive(Clone, Copy)]
enum NamedKind {
    FontFace,
    FontPalette,
    ViewTransition,
}

fn rule_without_block(payload: &CssRule<'_>) -> bool {
    matches!(
        payload,
        CssRule::Charset(_)
            | CssRule::Import(_)
            | CssRule::Namespace(_)
            | CssRule::LayerStatement(_)
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
    serialize_name(value.css_name(), dest)?;
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
