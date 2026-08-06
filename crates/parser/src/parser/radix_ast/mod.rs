//! Parser for the compiler-owned Radix AST.

#[cfg(test)]
use rocketcss_ast::{
    CascadeOrigin, CascadePhase, CssEffectiveKey, CssHistorySegment, SelectorPathId,
    SelectorValueId,
};
use rocketcss_ast::{
    ContainerRule, CounterStyleRule, CssDeclaration, CssDeclarationBlockId, CssEffectiveContext,
    CssRule, CssRuleId, DeclarationBlockOwner, EffectiveKeyId, FontFaceRule, FontFeatureSubrule,
    FontFeatureValuesRule, FontPaletteValuesRule, Keyframe, KeyframesRule, LayerBlockRule,
    LayerStatementRule, MediaRule, MozDocumentRule, NestedDeclarationsRule, NestingRule,
    PageDeclarationsRule, PageMarginRule, PageRule, PositionTryRule, PropertyRule,
    PropertyRuleDescriptor, RuleListId, ScopeRule, SelectorFrameKind, StartingStyleRule, StyleRule,
    StyleSheet, StyleSheetCapacity, SupportsRule, UnknownAtRule, ViewTransitionRule, ViewportRule,
};

use super::{
    css_rule::{RuleBodyDelimiter, scan_rule_body},
    media::{parse_import_rule, parse_media_list, parse_supports_condition},
    properties::parse_declaration_with_css_wide_hint,
    rules::{
        at_rule_vendor_prefix, font_feature_subrule_type, page_margin_box, parse_charset,
        parse_container_prelude, parse_custom_media, parse_family_names,
        parse_font_face_contents_into, parse_font_feature_declarations_into,
        parse_font_palette_contents_into, parse_keyframe_selector, parse_keyframes_name,
        parse_layer_names, parse_namespace, parse_page_selectors,
        parse_property_rule_descriptors_into, parse_scope_prelude, parse_single_ident,
        parse_view_transition_contents_into, validate_moz_document_prelude,
    },
    selector::{parse_selector_list, parse_selector_list_with_recovery, parse_selector_string},
    stylesheet::{check_depth, into_error, recover_declaration, recover_rule, span_from},
    values::collect_tokens,
};
use crate::prelude::*;

mod at_rule;
mod style;

use at_rule::parse_group_at_rule;
use style::parse_style_rule;

impl<'ast> Compiler<'ast> {
    /// Parses a stylesheet directly into the authoritative Radix stores.
    pub(crate) fn parse_stylesheet(
        &mut self,
        source: &'ast str,
        options: ParserOptions<'ast>,
    ) -> Result<StyleSheet<'ast>, Error<'ast>> {
        self.cursor = super::ParserCursor::new(source);
        self.replay.reset_for_new_source();
        let mut stylesheet =
            StyleSheet::with_capacity_in(self.allocator(), stylesheet_capacity(source.len()));

        let mut state = self.state();
        while let Ok(token) = self.next_including_whitespace_and_comments() {
            match token {
                ValueToken::WhiteSpace(_) => {}
                ValueToken::Comment(comment) if comment.starts_with('!') => {
                    stylesheet.push_license_comment(comment);
                }
                _ => break,
            }
            state = self.state();
        }
        self.reset(&state);

        let root = stylesheet.stylesheet_root().root_rules();
        parse_rule_list(
            self,
            &mut stylesheet,
            root,
            CssEffectiveContext::<'ast>::default(),
            &options,
            0,
        )
        .map_err(|error| into_error(error, options.filename))?;
        Ok(stylesheet)
    }
}

/// Estimate the dense authored shape without scanning the source a second
/// time. The divisors intentionally undershoot punctuation-heavy stylesheets:
/// one ordinary growth remains cheap, while avoiding the repeated geometric
/// reallocations of starting every global store at zero.
///
/// Calibrated against the benchmark corpora (bootstrap.css, tailwind.css) so
/// each dense store is preallocated past its final authored length without a
/// geometric reallocation. The parser capacity guard tests any corpus whose
/// actual count exceeds its estimate; tighten the divisor before adding more
/// dense nodes per byte.
#[doc(hidden)]
pub fn stylesheet_capacity(source_len: usize) -> StyleSheetCapacity {
    let rules = source_len / 96;
    StyleSheetCapacity {
        rules,
        rule_lists: source_len / 512,
        declaration_blocks: rules,
        declarations: source_len / 44,
        selectors: source_len / 100,
        contexts: source_len / 512,
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum TopLevelState {
    Start,
    Layers,
    Imports,
    Namespaces,
    Body,
}

fn parse_rule_list<'ast>(
    input: &mut Compiler<'ast>,
    stylesheet: &mut StyleSheet<'ast>,
    list: RuleListId,
    context: CssEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    check_depth(input, depth)?;
    let mut top_level_state = TopLevelState::Start;
    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        let result = match token {
            ValueToken::Cdo | ValueToken::Cdc | ValueToken::Semicolon => continue,
            ValueToken::AtKeyword(name) => {
                if depth > 0
                    && (name.eq_ignore_ascii_case("import")
                        || name.eq_ignore_ascii_case("namespace")
                        || name.eq_ignore_ascii_case("charset")
                        || name.eq_ignore_ascii_case("custom-media"))
                {
                    Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
                } else if depth == 0
                    && name.eq_ignore_ascii_case("import")
                    && top_level_state > TopLevelState::Imports
                {
                    Err(input.new_custom_error(ParserError::UnexpectedImportRule))
                } else if depth == 0
                    && name.eq_ignore_ascii_case("namespace")
                    && top_level_state > TopLevelState::Namespaces
                {
                    Err(input.new_custom_error(ParserError::UnexpectedNamespaceRule))
                } else {
                    parse_group_at_rule(
                        input, stylesheet, list, context, options, depth, &start, name,
                    )
                }
            }
            _ => {
                input.reset(&start);
                parse_style_rule(input, stylesheet, list, context, options, depth, &start)
            }
        };
        match result {
            Ok(rule) => {
                if depth == 0 {
                    top_level_state = match stylesheet
                        .rule(rule)
                        .expect("the parsed top-level rule remains resolvable")
                        .payload()
                    {
                        CssRule::Charset(_) => top_level_state,
                        CssRule::Import(_) => TopLevelState::Imports,
                        CssRule::Namespace(_) => TopLevelState::Namespaces,
                        CssRule::LayerStatement(_) if top_level_state <= TopLevelState::Layers => {
                            TopLevelState::Layers
                        }
                        _ => TopLevelState::Body,
                    };
                }
            }
            Err(_) if options.error_recovery => recover_rule(input),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn mutation_error<'ast>(
    input: &Compiler<'ast>,
    error: rocketcss_ast::StyleSheetMutationError<'ast>,
) -> ParseError<'ast, ParserError<'ast>> {
    use rocketcss_ast::StyleSheetMutationError;

    let error = match error {
        StyleSheetMutationError::<'ast>::PrimaryRuleCapacityExhausted
        | StyleSheetMutationError::<'ast>::PrimaryDeclarationBlockCapacityExhausted
        | StyleSheetMutationError::<'ast>::RuleListCapacityExhausted
        | StyleSheetMutationError::<'ast>::EffectiveKeyCapacityExhausted
        | StyleSheetMutationError::<'ast>::SelectorContextCapacityExhausted
        | StyleSheetMutationError::<'ast>::DeclarationCapacityExhausted
        | StyleSheetMutationError::<'ast>::DeclarationOverflowCapacityExhausted
        | StyleSheetMutationError::<'ast>::LocalRuleCapacityExhausted(_)
        | StyleSheetMutationError::<'ast>::LocalDeclarationBlockCapacityExhausted(_) => {
            ParserError::AstCapacityExceeded
        }
        StyleSheetMutationError::<'ast>::UnknownRule(_)
        | StyleSheetMutationError::<'ast>::UnknownRuleList(_)
        | StyleSheetMutationError::<'ast>::UnknownEffectiveKey(_)
        | StyleSheetMutationError::<'ast>::RetiredRule(_)
        | StyleSheetMutationError::<'ast>::ChildListAlreadyExists(_)
        | StyleSheetMutationError::<'ast>::DeclarationBlockAlreadyExists(_)
        | StyleSheetMutationError::<'ast>::UnknownDeclarationBlock(_)
        | StyleSheetMutationError::<'ast>::UnknownDeclarationOverflow(_)
        | StyleSheetMutationError::<'ast>::UnknownDeclaration(_)
        | StyleSheetMutationError::<'ast>::NonContiguousDeclarationRange(_)
        | StyleSheetMutationError::<'ast>::InvalidRuleTopology(_)
        | StyleSheetMutationError::<'ast>::RuleHasChildren(_) => ParserError::InvalidRule,
    };
    input.new_custom_error(error)
}

#[cfg(test)]
mod tests;
