use rs_css_allocator::{Allocator, vec::Vec};
use rs_css_ast::{
    AttrSelectorOperator, CSSWideKeyword, Combinator, CssColor, CssRule, CustomProperty,
    CustomPropertyName, Declaration, DeclarationBlock, DimensionPercentage, Display, DisplayInside,
    DisplayKeyword, DisplayOutside, FamilyName, FontFeatureDeclaration, FontFeatureSubrule,
    FontFeatureSubruleType, FontFeatureValuesRule, FontPaletteValuesProperty,
    FontPaletteValuesRule, Function, ImportRule, Keyframe, KeyframeSelector, KeyframesName,
    KeyframesRule, LayerBlockRule, LayerStatementRule, LengthUnit, LengthValue, MediaCondition,
    MediaList, MediaQuery, MediaRule, MediaType, Multiplier, NamespaceRule, Navigation,
    NestingRule, NoneOrCustomIdentList, PageMarginBox, PageMarginRule, PagePseudoClass, PageRule,
    PageSelector, ParsedCaseSensitivity, ParsedComponent, PropertyId, PropertyRule, PseudoClass,
    PseudoElement, Qualifier, RGBA, ScopeRule, Selector, SelectorComponent, SelectorList, Size,
    StartingStyleRule, StyleRule, StyleSheet, SupportsCondition, SupportsRule, SyntaxComponent,
    SyntaxComponentKind, SyntaxString, Token as ValueToken, TokenOrValue, UnknownAtRule,
    UnparsedProperty, Url, VendorPrefix, ViewTransitionProperty, ViewTransitionRule, Visibility,
};

use crate::{
    BasicParseErrorKind, Delimiter, Error, Parse, ParseError, ParseErrorKind, Parser, ParserError,
    ParserInput, ParserOptions, ParserState, SourcePosition, Span,
};

const MAX_NESTING_DEPTH: usize = 500;

impl<'i> Parse<'i> for SelectorList<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        parse_selector_list(input, allocator, 0)
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

/// Parses a stylesheet using the span-only tokenizer and arena-backed AST.
pub fn parse<'i>(
    source: &'i str,
    allocator: &'i Allocator,
    options: ParserOptions<'i>,
) -> Result<StyleSheet<'i>, Error<'i>> {
    let mut input = ParserInput::new(source, allocator);
    let mut parser = Parser::new(&mut input);
    let mut license_comments = allocator.vec();

    let mut state = parser.state();
    while let Ok(token) = parser.next_including_whitespace_and_comments() {
        match token {
            ValueToken::WhiteSpace(_) => {}
            ValueToken::Comment(comment) if comment.starts_with('!') => {
                license_comments.push(*comment);
            }
            _ => break,
        }
        state = parser.state();
    }
    parser.reset(&state);

    let rules = parse_rule_list(&mut parser, allocator, &options, 0)
        .map_err(|error| into_error(error, options.filename))?;
    let source_map_url = parser.current_source_map_url();
    let mut sources = allocator.vec();
    sources.push(options.filename);
    let mut source_map_urls = allocator.vec();
    source_map_urls.push(source_map_url);

    Ok(StyleSheet {
        license_comments,
        rules,
        source_map_urls,
        sources,
    })
}

fn parse_rule_list<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, CssRule<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut rules = allocator.vec();
    let mut top_level_state = TopLevelState::Start;

    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        let result = match token {
            ValueToken::AtKeyword(name) => {
                if depth > 0
                    && matches_ignore_case(
                        name,
                        &["import", "namespace", "charset", "custom-media"],
                    )
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
                    parse_at_rule(input, allocator, options, depth, &start, name, false)
                }
            }
            ValueToken::Cdo | ValueToken::Cdc | ValueToken::Semicolon => continue,
            _ => {
                input.reset(&start);
                parse_qualified_rule(input, allocator, options, depth, &start)
            }
        };

        match result {
            Ok(rule) => {
                if depth == 0 {
                    top_level_state = match &rule {
                        CssRule::Ignored => top_level_state,
                        CssRule::Import(_) => TopLevelState::Imports,
                        CssRule::Namespace(_) => TopLevelState::Namespaces,
                        CssRule::LayerStatement(_) if top_level_state <= TopLevelState::Layers => {
                            TopLevelState::Layers
                        }
                        _ => TopLevelState::Body,
                    };
                }
                rules.push(rule)
            }
            Err(_) if options.error_recovery => recover_rule(input),
            Err(error) => return Err(error),
        }
    }

    Ok(rules)
}

fn parse_group_rule_body<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    in_style_rule: bool,
) -> Result<Vec<'i, CssRule<'i>>, ParseError<'i, ParserError<'i>>> {
    if !in_style_rule {
        return parse_rule_list(input, allocator, options, depth);
    }

    let start = input.state();
    let (declarations, mut rules) = parse_style_contents(input, allocator, options, depth)?;
    if !declarations.is_empty() {
        rules.insert(
            0,
            CssRule::NestedDeclarations(allocator.boxed(rs_css_ast::NestedDeclarationsRule {
                declarations: allocator.boxed(declarations),
                span: span_from(&start, input.position()),
            })),
        );
    }
    Ok(rules)
}

fn parse_at_rule<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &ParserState,
    name: &'i str,
    in_style_rule: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    if in_style_rule
        && matches_ignore_case(
            name,
            &[
                "import",
                "namespace",
                "charset",
                "custom-media",
                "font-face",
                "font-feature-values",
                "font-palette-values",
                "counter-style",
                "keyframes",
                "-webkit-keyframes",
                "-moz-keyframes",
                "-o-keyframes",
                "-ms-keyframes",
                "page",
                "property",
                "position-try",
                "viewport",
                "-ms-viewport",
                "view-transition",
            ],
        )
    {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let prelude_start = input.position();
    let prelude = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| collect_tokens(input, allocator, depth + 1),
    )?;
    let prelude_end = input.position();
    let raw_prelude = input.slice(prelude_start..prelude_end).trim();

    enum Ending {
        None,
        Semicolon,
        Block,
    }

    let ending = match input.next() {
        Ok(ValueToken::Semicolon) => Ending::Semicolon,
        Ok(ValueToken::CurlyBracketBlock) => Ending::Block,
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => Ending::None,
        Ok(_) => return Err(input.new_custom_error(ParserError::InvalidAtRule(name))),
        Err(error) => return Err(error.into()),
    };

    let rule = if name.eq_ignore_ascii_case("import") {
        if matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        parse_import(raw_prelude, allocator, start, input.position())?
    } else if name.eq_ignore_ascii_case("media") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let query = parse_media_list(raw_prelude, allocator)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, options, depth + 1, in_style_rule)
        })?;
        CssRule::Media(allocator.boxed(MediaRule {
            span: span_from(start, input.position()),
            query: allocator.boxed(query),
            rules,
        }))
    } else if name.eq_ignore_ascii_case("supports") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, options, depth + 1, in_style_rule)
        })?;
        CssRule::Supports(allocator.boxed(SupportsRule {
            condition: allocator.boxed(parse_supports_condition(raw_prelude)),
            span: span_from(start, input.position()),
            rules,
        }))
    } else if name.eq_ignore_ascii_case("starting-style") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, options, depth + 1, in_style_rule)
        })?;
        CssRule::StartingStyle(allocator.boxed(StartingStyleRule {
            span: span_from(start, input.position()),
            rules,
        }))
    } else if name.eq_ignore_ascii_case("font-face") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let properties = input.parse_nested_block(|input| {
            parse_font_face_contents(input, allocator, options, depth + 1)
        })?;
        CssRule::FontFace(allocator.boxed(rs_css_ast::FontFaceRule {
            span: span_from(start, input.position()),
            properties,
        }))
    } else if name.eq_ignore_ascii_case("charset") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        validate_charset(raw_prelude, allocator)?;
        CssRule::Ignored
    } else if name.eq_ignore_ascii_case("namespace") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let (prefix, url) = parse_namespace(raw_prelude, allocator)?;
        CssRule::Namespace(allocator.boxed(NamespaceRule {
            span: span_from(start, input.position()),
            prefix,
            url,
        }))
    } else if name.eq_ignore_ascii_case("layer") {
        let mut names = parse_layer_names(raw_prelude, allocator)?;
        if matches!(ending, Ending::Block) {
            if names.len() > 1 {
                return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
            }
            let layer_name = names.pop();
            let rules = input.parse_nested_block(|input| {
                parse_group_rule_body(input, allocator, options, depth + 1, in_style_rule)
            })?;
            CssRule::LayerBlock(allocator.boxed(LayerBlockRule {
                span: span_from(start, input.position()),
                name: layer_name,
                rules,
            }))
        } else {
            if names.is_empty() {
                return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
            }
            CssRule::LayerStatement(allocator.boxed(LayerStatementRule {
                span: span_from(start, input.position()),
                names,
            }))
        }
    } else if name.eq_ignore_ascii_case("custom-media") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let (custom_name, query) = parse_custom_media(raw_prelude, allocator)?;
        CssRule::CustomMedia(allocator.boxed(rs_css_ast::CustomMediaRule {
            span: span_from(start, input.position()),
            name: custom_name,
            query: allocator.boxed(query),
        }))
    } else if matches_ignore_case(
        name,
        &[
            "keyframes",
            "-webkit-keyframes",
            "-moz-keyframes",
            "-o-keyframes",
            "-ms-keyframes",
        ],
    ) {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let keyframes_name = parse_keyframes_name(raw_prelude, allocator)?;
        let keyframes = input.parse_nested_block(|input| {
            parse_keyframe_list(input, allocator, options, depth + 1)
        })?;
        CssRule::Keyframes(allocator.boxed(KeyframesRule {
            keyframes,
            span: span_from(start, input.position()),
            name: allocator.boxed(keyframes_name),
            vendor_prefix: at_rule_vendor_prefix(name),
        }))
    } else if name.eq_ignore_ascii_case("counter-style") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let counter_name = parse_single_ident(raw_prelude, allocator)?;
        let declarations = input.parse_nested_block(|input| {
            parse_declaration_block(input, allocator, options, depth + 1)
        })?;
        CssRule::CounterStyle(allocator.boxed(rs_css_ast::CounterStyleRule {
            declarations: allocator.boxed(declarations),
            span: span_from(start, input.position()),
            name: counter_name,
        }))
    } else if matches_ignore_case(name, &["viewport", "-ms-viewport"]) {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let declarations = input.parse_nested_block(|input| {
            parse_declaration_block(input, allocator, options, depth + 1)
        })?;
        CssRule::Viewport(allocator.boxed(rs_css_ast::ViewportRule {
            declarations: allocator.boxed(declarations),
            span: span_from(start, input.position()),
            vendor_prefix: at_rule_vendor_prefix(name),
        }))
    } else if name.eq_ignore_ascii_case("position-try") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let position_name = parse_single_ident(raw_prelude, allocator)?;
        if !position_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let declarations = input.parse_nested_block(|input| {
            parse_declaration_block(input, allocator, options, depth + 1)
        })?;
        CssRule::PositionTry(allocator.boxed(rs_css_ast::PositionTryRule {
            span: span_from(start, input.position()),
            name: position_name,
            declarations: allocator.boxed(declarations),
        }))
    } else if name.eq_ignore_ascii_case("-moz-document") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        validate_moz_document_prelude(raw_prelude, allocator)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, options, depth + 1, in_style_rule)
        })?;
        CssRule::MozDocument(allocator.boxed(rs_css_ast::MozDocumentRule {
            span: span_from(start, input.position()),
            rules,
        }))
    } else if name.eq_ignore_ascii_case("container") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let (container_name, condition) = parse_container_prelude(raw_prelude, allocator)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, options, depth + 1, in_style_rule)
        })?;
        CssRule::Container(allocator.boxed(rs_css_ast::ContainerRule {
            condition,
            span: span_from(start, input.position()),
            name: container_name,
            rules,
        }))
    } else if name.eq_ignore_ascii_case("scope") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let (scope_start, scope_end) = parse_scope_prelude(raw_prelude, allocator, depth + 1)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, options, depth + 1, in_style_rule)
        })?;
        CssRule::Scope(allocator.boxed(ScopeRule {
            span: span_from(start, input.position()),
            rules,
            scope_end,
            scope_start,
        }))
    } else if name.eq_ignore_ascii_case("page") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let selectors = parse_page_selectors(raw_prelude, allocator)?;
        let (declarations, rules) = input
            .parse_nested_block(|input| parse_page_body(input, allocator, options, depth + 1))?;
        CssRule::Page(allocator.boxed(PageRule {
            declarations: allocator.boxed(declarations),
            span: span_from(start, input.position()),
            rules,
            selectors,
        }))
    } else if name.eq_ignore_ascii_case("font-palette-values") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let palette_name = parse_single_ident(raw_prelude, allocator)?;
        if !palette_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let properties = input.parse_nested_block(|input| {
            parse_font_palette_contents(input, allocator, options, depth + 1)
        })?;
        CssRule::FontPaletteValues(allocator.boxed(FontPaletteValuesRule {
            span: span_from(start, input.position()),
            name: palette_name,
            properties,
        }))
    } else if name.eq_ignore_ascii_case("font-feature-values") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let family_names = parse_family_names(raw_prelude, allocator)?;
        let rules = input.parse_nested_block(|input| {
            parse_font_feature_subrules(input, allocator, options, depth + 1)
        })?;
        CssRule::FontFeatureValues(allocator.boxed(FontFeatureValuesRule {
            span: span_from(start, input.position()),
            name: family_names,
            rules,
        }))
    } else if name.eq_ignore_ascii_case("property") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let property_name = parse_single_ident(raw_prelude, allocator)?;
        if !property_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let mut property = input.parse_nested_block(|input| {
            parse_property_rule(input, allocator, options, depth + 1, property_name)
        })?;
        property.span = span_from(start, input.position());
        CssRule::Property(allocator.boxed(property))
    } else if name.eq_ignore_ascii_case("view-transition") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let properties = input.parse_nested_block(|input| {
            parse_view_transition_contents(input, allocator, options, depth + 1)
        })?;
        CssRule::ViewTransition(allocator.boxed(ViewTransitionRule {
            span: span_from(start, input.position()),
            properties,
        }))
    } else if name.eq_ignore_ascii_case("nest") && in_style_rule {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let selectors = parse_selector_string(raw_prelude, allocator, depth + 1)?;
        let (declarations, rules) = input.parse_nested_block(|input| {
            parse_style_contents(input, allocator, options, depth + 1)
        })?;
        let span = span_from(start, input.position());
        CssRule::Nesting(allocator.boxed(NestingRule {
            span,
            style: allocator.boxed(StyleRule {
                declarations: allocator.boxed(declarations),
                span,
                rules,
                selectors: allocator.boxed(selectors),
                vendor_prefix: VendorPrefix::NONE,
            }),
        }))
    } else {
        let block = if matches!(ending, Ending::Block) {
            Some(input.parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?)
        } else {
            None
        };
        CssRule::Unknown(allocator.boxed(UnknownAtRule {
            block,
            span: span_from(start, input.position()),
            name,
            prelude,
        }))
    };

    Ok(rule)
}

fn parse_qualified_rule<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &ParserState,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let selectors = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
        parse_selector_list(input, allocator, depth + 1)
    })?;
    input.expect_curly_bracket_block()?;
    let (declarations, rules) = input
        .parse_nested_block(|input| parse_style_contents(input, allocator, options, depth + 1))?;

    Ok(CssRule::Style(allocator.boxed(StyleRule {
        declarations: allocator.boxed(declarations),
        span: span_from(start, input.position()),
        rules,
        selectors: allocator.boxed(selectors),
        vendor_prefix: VendorPrefix::NONE,
    })))
}

type StyleContents<'i> = (DeclarationBlock<'i>, Vec<'i, CssRule<'i>>);

fn parse_style_contents<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<StyleContents<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut declarations = DeclarationBlock::new(allocator);
    let mut rules = allocator.vec();

    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        let result = match token {
            ValueToken::Semicolon => continue,
            ValueToken::AtKeyword(name) => {
                parse_at_rule(input, allocator, options, depth, &start, name, true)
                    .map(|rule| Some((false, rule)))
            }
            ValueToken::Ident(name) => {
                let has_colon = input.try_parse(Parser::expect_colon).is_ok();
                let next_delimiter = next_rule_body_delimiter(input);
                if has_colon && next_delimiter != Some(RuleBodyDelimiter::CurlyBracket) {
                    parse_declaration(input, allocator, name, depth).map(
                        |(declaration, important)| {
                            declarations.push(declaration, important);
                            None
                        },
                    )
                } else if !has_colon && next_delimiter != Some(RuleBodyDelimiter::CurlyBracket) {
                    Err(input.new_custom_error(ParserError::InvalidDeclaration))
                } else {
                    input.reset(&start);
                    parse_qualified_rule(input, allocator, options, depth, &start)
                        .map(|rule| Some((true, rule)))
                }
            }
            _ => {
                input.reset(&start);
                parse_qualified_rule(input, allocator, options, depth, &start)
                    .map(|rule| Some((true, rule)))
            }
        };

        match result {
            Ok(Some((_, rule))) => rules.push(rule),
            Ok(None) => {}
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }

    Ok((declarations, rules))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RuleBodyDelimiter {
    Semicolon,
    CurlyBracket,
}

fn next_rule_body_delimiter(input: &mut Parser<'_, '_>) -> Option<RuleBodyDelimiter> {
    let state = input.state();
    let _: Result<(), ParseError<'_, ()>> = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| {
            while input.next().is_ok() {}
            Ok(())
        },
    );
    let delimiter = match input.next() {
        Ok(ValueToken::Semicolon) => Some(RuleBodyDelimiter::Semicolon),
        Ok(ValueToken::CurlyBracketBlock) => Some(RuleBodyDelimiter::CurlyBracket),
        _ => None,
    };
    input.reset(&state);
    delimiter
}

fn parse_font_face_contents<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, rs_css_ast::FontFaceProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = allocator.vec();
    loop {
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, ValueToken::Semicolon) {
            continue;
        }

        let result = match token {
            ValueToken::Ident(name) => {
                input.expect_colon()?;
                let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                    collect_tokens(input, allocator, depth + 1)
                })?;
                let _ = input.try_parse(Parser::expect_semicolon);
                if remove_important(&mut value) {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                Ok(rs_css_ast::FontFaceProperty::Custom(allocator.boxed(
                    CustomProperty {
                        name: allocator.boxed(CustomPropertyName::Unknown(name)),
                        value,
                    },
                )))
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };

        match result {
            Ok(property) => properties.push(property),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(properties)
}

fn parse_declaration<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    name: &'i str,
    depth: usize,
) -> Result<(Declaration<'i>, bool), ParseError<'i, ParserError<'i>>> {
    let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
        collect_tokens(input, allocator, depth + 1)
    })?;
    let _ = input.try_parse(Parser::expect_semicolon);
    let important = remove_important(&mut value);
    trim_leading_whitespace(&mut value);

    let declaration = if name.starts_with("--") {
        Declaration::Custom(allocator.boxed(CustomProperty {
            name: allocator.boxed(CustomPropertyName::Custom(name)),
            value,
        }))
    } else if let Some(declaration) = parse_typed_declaration(name, &value, allocator) {
        declaration
    } else if name.eq_ignore_ascii_case("all") && value.len() == 1 {
        match token_ident(&value[0]).and_then(css_wide_keyword) {
            Some(keyword) => Declaration::All(keyword),
            None => unparsed_declaration(name, value, allocator),
        }
    } else {
        unparsed_declaration(name, value, allocator)
    };

    Ok((declaration, important))
}

fn unparsed_declaration<'i>(
    name: &'i str,
    value: Vec<'i, TokenOrValue<'i>>,
    allocator: &'i Allocator,
) -> Declaration<'i> {
    Declaration::Unparsed(allocator.boxed(UnparsedProperty {
        property_id: allocator.boxed(PropertyId::from_name(name)),
        value,
    }))
}

fn parse_typed_declaration<'i>(
    name: &str,
    value: &[TokenOrValue<'i>],
    allocator: &'i Allocator,
) -> Option<Declaration<'i>> {
    if name.eq_ignore_ascii_case("color") {
        return parse_color(value, allocator)
            .map(|color| Declaration::Color(allocator.boxed(color)));
    }
    if name.eq_ignore_ascii_case("background-color") {
        return parse_color(value, allocator)
            .map(|color| Declaration::BackgroundColor(allocator.boxed(color)));
    }
    if name.eq_ignore_ascii_case("opacity") {
        return match single_token(value)? {
            ValueToken::Number(value) => Some(Declaration::Opacity(*value)),
            ValueToken::Percentage(value) => Some(Declaration::Opacity(*value)),
            _ => None,
        };
    }
    if name.eq_ignore_ascii_case("visibility") {
        let value = token_ident(value.first()?)?;
        let visibility = if value.eq_ignore_ascii_case("visible") {
            Visibility::Visible
        } else if value.eq_ignore_ascii_case("hidden") {
            Visibility::Hidden
        } else if value.eq_ignore_ascii_case("collapse") {
            Visibility::Collapse
        } else {
            return None;
        };
        return Some(Declaration::Visibility(visibility));
    }
    if name.eq_ignore_ascii_case("display") {
        return parse_display(value, allocator)
            .map(|display| Declaration::Display(allocator.boxed(display)));
    }

    let size = parse_size(value, allocator)?;
    let size = allocator.boxed(size);
    if name.eq_ignore_ascii_case("width") {
        Some(Declaration::Width(size))
    } else if name.eq_ignore_ascii_case("height") {
        Some(Declaration::Height(size))
    } else if name.eq_ignore_ascii_case("min-width") {
        Some(Declaration::MinWidth(size))
    } else if name.eq_ignore_ascii_case("min-height") {
        Some(Declaration::MinHeight(size))
    } else if name.eq_ignore_ascii_case("block-size") {
        Some(Declaration::BlockSize(size))
    } else if name.eq_ignore_ascii_case("inline-size") {
        Some(Declaration::InlineSize(size))
    } else if name.eq_ignore_ascii_case("min-block-size") {
        Some(Declaration::MinBlockSize(size))
    } else if name.eq_ignore_ascii_case("min-inline-size") {
        Some(Declaration::MinInlineSize(size))
    } else {
        None
    }
}

fn parse_color<'i>(value: &[TokenOrValue<'i>], _allocator: &'i Allocator) -> Option<CssColor<'i>> {
    let token = single_token(value)?;
    match token {
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("currentcolor") => {
            Some(CssColor::CurrentColor)
        }
        ValueToken::Ident(name) => named_color(name).map(CssColor::Rgba),
        ValueToken::Hash(value) | ValueToken::IdHash(value) => {
            parse_hex_color(value).map(CssColor::Rgba)
        }
        _ => None,
    }
}

fn named_color(name: &str) -> Option<RGBA> {
    Some(if name.eq_ignore_ascii_case("transparent") {
        RGBA {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    } else if name.eq_ignore_ascii_case("black") {
        rgba(0, 0, 0)
    } else if name.eq_ignore_ascii_case("silver") {
        rgba(192, 192, 192)
    } else if name.eq_ignore_ascii_case("gray") {
        rgba(128, 128, 128)
    } else if name.eq_ignore_ascii_case("white") {
        rgba(255, 255, 255)
    } else if name.eq_ignore_ascii_case("maroon") {
        rgba(128, 0, 0)
    } else if name.eq_ignore_ascii_case("red") {
        rgba(255, 0, 0)
    } else if name.eq_ignore_ascii_case("purple") {
        rgba(128, 0, 128)
    } else if name.eq_ignore_ascii_case("fuchsia") {
        rgba(255, 0, 255)
    } else if name.eq_ignore_ascii_case("green") {
        rgba(0, 128, 0)
    } else if name.eq_ignore_ascii_case("lime") {
        rgba(0, 255, 0)
    } else if name.eq_ignore_ascii_case("olive") {
        rgba(128, 128, 0)
    } else if name.eq_ignore_ascii_case("yellow") {
        rgba(255, 255, 0)
    } else if name.eq_ignore_ascii_case("navy") {
        rgba(0, 0, 128)
    } else if name.eq_ignore_ascii_case("blue") {
        rgba(0, 0, 255)
    } else if name.eq_ignore_ascii_case("teal") {
        rgba(0, 128, 128)
    } else if name.eq_ignore_ascii_case("aqua") {
        rgba(0, 255, 255)
    } else {
        return None;
    })
}

const fn rgba(red: u8, green: u8, blue: u8) -> RGBA {
    RGBA {
        red,
        green,
        blue,
        alpha: 255,
    }
}

fn parse_hex_color(value: &str) -> Option<RGBA> {
    fn pair(value: &str) -> Option<u8> {
        u8::from_str_radix(value, 16).ok()
    }
    Some(match value.len() {
        3 | 4 => {
            let mut bytes = value.bytes().map(|byte| {
                let digit = (byte as char).to_digit(16)? as u8;
                Some(digit * 17)
            });
            RGBA {
                red: bytes.next()??,
                green: bytes.next()??,
                blue: bytes.next()??,
                alpha: match bytes.next() {
                    Some(value) => value?,
                    None => 255,
                },
            }
        }
        6 | 8 => RGBA {
            red: pair(&value[0..2])?,
            green: pair(&value[2..4])?,
            blue: pair(&value[4..6])?,
            alpha: if value.len() == 8 {
                pair(&value[6..8])?
            } else {
                255
            },
        },
        _ => return None,
    })
}

fn parse_display<'i>(value: &[TokenOrValue<'i>], allocator: &'i Allocator) -> Option<Display<'i>> {
    let ident = token_ident(value.first()?)?;
    if value.len() != 1 {
        return None;
    }
    if ident.eq_ignore_ascii_case("none") {
        return Some(Display::Keyword(DisplayKeyword::None));
    }
    if ident.eq_ignore_ascii_case("contents") {
        return Some(Display::Keyword(DisplayKeyword::Contents));
    }

    let (outside, inside, is_list_item) = if ident.eq_ignore_ascii_case("block") {
        (DisplayOutside::Block, DisplayInside::Flow, false)
    } else if ident.eq_ignore_ascii_case("inline") {
        (DisplayOutside::Inline, DisplayInside::Flow, false)
    } else if ident.eq_ignore_ascii_case("flow-root") {
        (DisplayOutside::Block, DisplayInside::FlowRoot, false)
    } else if ident.eq_ignore_ascii_case("flex") {
        (
            DisplayOutside::Block,
            DisplayInside::Flex {
                vendor_prefix: VendorPrefix::NONE,
            },
            false,
        )
    } else if ident.eq_ignore_ascii_case("inline-flex") {
        (
            DisplayOutside::Inline,
            DisplayInside::Flex {
                vendor_prefix: VendorPrefix::NONE,
            },
            false,
        )
    } else if ident.eq_ignore_ascii_case("grid") {
        (DisplayOutside::Block, DisplayInside::Grid, false)
    } else if ident.eq_ignore_ascii_case("inline-grid") {
        (DisplayOutside::Inline, DisplayInside::Grid, false)
    } else if ident.eq_ignore_ascii_case("list-item") {
        (DisplayOutside::Block, DisplayInside::Flow, true)
    } else {
        return None;
    };
    Some(Display::Pair {
        inside: allocator.boxed(inside),
        is_list_item,
        outside,
    })
}

fn parse_size<'i>(value: &[TokenOrValue<'i>], allocator: &'i Allocator) -> Option<Size<'i>> {
    let token = single_token(value)?;
    match token {
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("auto") => Some(Size::Auto),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("min-content") => {
            Some(Size::MinContent {
                vendor_prefix: VendorPrefix::NONE,
            })
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("max-content") => {
            Some(Size::MaxContent {
                vendor_prefix: VendorPrefix::NONE,
            })
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("fit-content") => {
            Some(Size::FitContent {
                vendor_prefix: VendorPrefix::NONE,
            })
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("stretch") => Some(Size::Stretch {
            vendor_prefix: VendorPrefix::NONE,
        }),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("contain") => Some(Size::Contain),
        ValueToken::Percentage(value) => Some(Size::LengthPercentage(
            allocator.boxed(DimensionPercentage::Percentage(*value)),
        )),
        ValueToken::Dimension { unit, value } => {
            let unit = parse_length_unit(unit)?;
            Some(Size::LengthPercentage(allocator.boxed(
                DimensionPercentage::Dimension(allocator.boxed(LengthValue {
                    unit,
                    value: *value,
                })),
            )))
        }
        ValueToken::Number(value) if *value == 0.0 => {
            Some(Size::LengthPercentage(allocator.boxed(
                DimensionPercentage::Dimension(allocator.boxed(LengthValue {
                    unit: LengthUnit::Px,
                    value: 0.0,
                })),
            )))
        }
        _ => None,
    }
}

fn parse_length_unit(unit: &str) -> Option<LengthUnit> {
    Some(if unit.eq_ignore_ascii_case("px") {
        LengthUnit::Px
    } else if unit.eq_ignore_ascii_case("in") {
        LengthUnit::In
    } else if unit.eq_ignore_ascii_case("cm") {
        LengthUnit::Cm
    } else if unit.eq_ignore_ascii_case("mm") {
        LengthUnit::Mm
    } else if unit.eq_ignore_ascii_case("q") {
        LengthUnit::Q
    } else if unit.eq_ignore_ascii_case("pt") {
        LengthUnit::Pt
    } else if unit.eq_ignore_ascii_case("pc") {
        LengthUnit::Pc
    } else if unit.eq_ignore_ascii_case("em") {
        LengthUnit::Em
    } else if unit.eq_ignore_ascii_case("rem") {
        LengthUnit::Rem
    } else if unit.eq_ignore_ascii_case("ex") {
        LengthUnit::Ex
    } else if unit.eq_ignore_ascii_case("ch") {
        LengthUnit::Ch
    } else if unit.eq_ignore_ascii_case("lh") {
        LengthUnit::Lh
    } else if unit.eq_ignore_ascii_case("rlh") {
        LengthUnit::Rlh
    } else if unit.eq_ignore_ascii_case("vw") {
        LengthUnit::Vw
    } else if unit.eq_ignore_ascii_case("vh") {
        LengthUnit::Vh
    } else if unit.eq_ignore_ascii_case("vmin") {
        LengthUnit::Vmin
    } else if unit.eq_ignore_ascii_case("vmax") {
        LengthUnit::Vmax
    } else {
        return None;
    })
}

fn single_token<'a, 'i>(value: &'a [TokenOrValue<'i>]) -> Option<&'a ValueToken<'i>> {
    if let [TokenOrValue::Token(token)] = value {
        Some(token)
    } else {
        None
    }
}

fn collect_tokens<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<Vec<'i, TokenOrValue<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut tokens = allocator.vec();

    loop {
        let state = input.state();
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        match token {
            ValueToken::Function(name) => {
                let arguments = input
                    .parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?;
                tokens.push(TokenOrValue::Function(
                    allocator.boxed(Function { arguments, name }),
                ));
            }
            ValueToken::UnquotedUrl(url) => {
                tokens.push(TokenOrValue::Url(allocator.boxed(Url {
                    span: input.current_token_span().unwrap_or_default(),
                    url,
                })));
            }
            ValueToken::Ident(name) if name.starts_with("--") => {
                tokens.push(TokenOrValue::DashedIdent(name));
            }
            opening @ (ValueToken::ParenthesisBlock
            | ValueToken::SquareBracketBlock
            | ValueToken::CurlyBracketBlock) => {
                let closing = match opening {
                    ValueToken::ParenthesisBlock => ValueToken::CloseParenthesis,
                    ValueToken::SquareBracketBlock => ValueToken::CloseSquareBracket,
                    ValueToken::CurlyBracketBlock => ValueToken::CloseCurlyBracket,
                    _ => unreachable!(),
                };
                tokens.push(TokenOrValue::Token(allocator.boxed(opening)));
                let nested = input
                    .parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?;
                tokens.extend(nested);
                tokens.push(TokenOrValue::Token(allocator.boxed(closing)));
            }
            ValueToken::BadUrl(_)
            | ValueToken::BadString(_)
            | ValueToken::CloseParenthesis
            | ValueToken::CloseSquareBracket
            | ValueToken::CloseCurlyBracket => {
                let token = input.current_token().unwrap_or_else(|| {
                    crate::TokenAndSpan::new(crate::Token::BadString, Span::default())
                });
                input.reset(&state);
                return Err(input.new_custom_error(ParserError::UnexpectedToken(token)));
            }
            token => tokens.push(TokenOrValue::Token(allocator.boxed(token))),
        }
    }

    Ok(tokens)
}

fn parse_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<SelectorList<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let parsed =
        input.parse_comma_separated(|input| parse_selector(input, allocator, depth + 1))?;
    let mut selectors = allocator.vec();
    selectors.extend(parsed);
    Ok(selectors)
}

fn parse_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<Selector<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut selector = allocator.vec();
    let mut pending_descendant = false;
    let mut can_have_descendant = false;

    loop {
        let token = match input.next_including_whitespace() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        if matches!(token, ValueToken::WhiteSpace(_)) {
            pending_descendant |= can_have_descendant;
            continue;
        }

        let explicit_combinator = match &token {
            ValueToken::Delim(">") => Some(Combinator::Child),
            ValueToken::Delim("+") => Some(Combinator::NextSibling),
            ValueToken::Delim("~") => Some(Combinator::LaterSibling),
            _ => None,
        };
        if let Some(combinator) = explicit_combinator {
            selector.push(SelectorComponent::Combinator(combinator));
            pending_descendant = false;
            can_have_descendant = false;
            continue;
        }

        if pending_descendant && can_have_descendant {
            selector.push(SelectorComponent::Combinator(Combinator::Descendant));
        }
        pending_descendant = false;

        let component = match token {
            ValueToken::Ident(name) => SelectorComponent::LocalName {
                name,
                lower_name: ascii_lowercase(name, allocator),
            },
            ValueToken::IdHash(id) => SelectorComponent::Id(id),
            ValueToken::Delim("*") => SelectorComponent::ExplicitUniversalType,
            ValueToken::Delim("&") => SelectorComponent::Nesting,
            ValueToken::Delim(".") => SelectorComponent::Class(input.expect_ident()?),
            ValueToken::Colon => parse_pseudo(input, allocator, depth + 1)?,
            ValueToken::SquareBracketBlock => {
                input.parse_nested_block(|input| parse_attribute(input, allocator))?
            }
            _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
        };
        selector.push(component);
        can_have_descendant = true;
    }

    if selector.is_empty() || matches!(selector.last(), Some(SelectorComponent::Combinator(_))) {
        return Err(input.new_custom_error(ParserError::InvalidSelector));
    }
    Ok(selector)
}

fn parse_pseudo<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<SelectorComponent<'i>, ParseError<'i, ParserError<'i>>> {
    let is_element = input.try_parse(Parser::expect_colon).is_ok();
    let token = input.next()?.clone();

    match token {
        ValueToken::Ident(name) if is_element => Ok(SelectorComponent::PseudoElement(
            allocator.boxed(pseudo_element(name)),
        )),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("root") => Ok(SelectorComponent::Root),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("empty") => {
            Ok(SelectorComponent::Empty)
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("scope") => {
            Ok(SelectorComponent::Scope)
        }
        ValueToken::Ident(name) => Ok(SelectorComponent::PseudoClass(
            allocator.boxed(pseudo_class(name)),
        )),
        ValueToken::Function(name) if is_element => {
            let arguments =
                input.parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?;
            Ok(SelectorComponent::PseudoElement(
                allocator.boxed(PseudoElement::CustomFunction { name, arguments }),
            ))
        }
        ValueToken::Function(name) => {
            let component =
                if name.eq_ignore_ascii_case("not") {
                    SelectorComponent::Negation(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else if name.eq_ignore_ascii_case("is") {
                    SelectorComponent::Is(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else if name.eq_ignore_ascii_case("where") {
                    SelectorComponent::Where(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else if name.eq_ignore_ascii_case("has") {
                    SelectorComponent::Has(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else {
                    let arguments = input
                        .parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?;
                    SelectorComponent::PseudoClass(
                        allocator.boxed(PseudoClass::CustomFunction { name, arguments }),
                    )
                };
            Ok(component)
        }
        _ => Err(input.new_custom_error(ParserError::InvalidSelector)),
    }
}

fn parse_attribute<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
) -> Result<SelectorComponent<'i>, ParseError<'i, ParserError<'i>>> {
    let name = input.expect_ident()?;
    let lower_name = ascii_lowercase(name, allocator);
    if input.is_exhausted() {
        return Ok(SelectorComponent::AttributeInNoNamespaceExists {
            local_name: name,
            local_name_lower: lower_name,
        });
    }

    let operator = match input.next()? {
        ValueToken::Delim("=") => AttrSelectorOperator::Equal,
        ValueToken::IncludeMatch => AttrSelectorOperator::Includes,
        ValueToken::DashMatch => AttrSelectorOperator::DashMatch,
        ValueToken::PrefixMatch => AttrSelectorOperator::Prefix,
        ValueToken::SubstringMatch => AttrSelectorOperator::Substring,
        ValueToken::SuffixMatch => AttrSelectorOperator::Suffix,
        _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
    };
    let value = input.expect_ident_or_string()?;
    let case_sensitivity = if let Ok(flag) = input.try_parse(Parser::expect_ident) {
        if flag.eq_ignore_ascii_case("i") {
            ParsedCaseSensitivity::AsciiCaseInsensitive
        } else if flag.eq_ignore_ascii_case("s") {
            ParsedCaseSensitivity::ExplicitCaseSensitive
        } else {
            return Err(input.new_custom_error(ParserError::InvalidSelector));
        }
    } else {
        ParsedCaseSensitivity::CaseSensitive
    };
    input.expect_exhausted()?;

    Ok(SelectorComponent::AttributeInNoNamespace {
        local_name: name,
        operator,
        value,
        case_sensitivity,
        never_matches: false,
    })
}

fn pseudo_class(name: &str) -> PseudoClass<'_> {
    if name.eq_ignore_ascii_case("hover") {
        PseudoClass::Hover
    } else if name.eq_ignore_ascii_case("active") {
        PseudoClass::Active
    } else if name.eq_ignore_ascii_case("focus") {
        PseudoClass::Focus
    } else if name.eq_ignore_ascii_case("focus-visible") {
        PseudoClass::FocusVisible
    } else if name.eq_ignore_ascii_case("focus-within") {
        PseudoClass::FocusWithin
    } else if name.eq_ignore_ascii_case("visited") {
        PseudoClass::Visited
    } else if name.eq_ignore_ascii_case("link") {
        PseudoClass::Link
    } else if name.eq_ignore_ascii_case("checked") {
        PseudoClass::Checked
    } else if name.eq_ignore_ascii_case("disabled") {
        PseudoClass::Disabled
    } else if name.eq_ignore_ascii_case("enabled") {
        PseudoClass::Enabled
    } else {
        PseudoClass::Custom { name }
    }
}

fn pseudo_element(name: &str) -> PseudoElement<'_> {
    if name.eq_ignore_ascii_case("before") {
        PseudoElement::Before
    } else if name.eq_ignore_ascii_case("after") {
        PseudoElement::After
    } else if name.eq_ignore_ascii_case("first-line") {
        PseudoElement::FirstLine
    } else if name.eq_ignore_ascii_case("first-letter") {
        PseudoElement::FirstLetter
    } else if name.eq_ignore_ascii_case("marker") {
        PseudoElement::Marker
    } else if name.eq_ignore_ascii_case("placeholder") {
        PseudoElement::Placeholder(VendorPrefix::NONE)
    } else {
        PseudoElement::Custom { name }
    }
}

fn parse_namespace<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(Option<&'i str>, &'i str), ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let state = parser.state();
    if let Ok(prefix) = parser.try_parse(Parser::expect_ident)
        && let Ok(url) = parser.expect_url_or_string()
    {
        parser.expect_exhausted()?;
        return Ok((Some(prefix), url));
    }
    parser.reset(&state);
    let url = parser.expect_url_or_string()?;
    parser.expect_exhausted()?;
    Ok((None, url))
}

fn validate_charset<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    parser.expect_string()?;
    parser.expect_exhausted()?;
    Ok(())
}

fn parse_layer_names<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<Vec<'i, Vec<'i, &'i str>>, ParseError<'i, ParserError<'i>>> {
    if prelude.is_empty() {
        return Ok(allocator.vec());
    }
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let parsed = parser.parse_comma_separated(|input| {
        let mut name = allocator.vec();
        name.push(input.expect_ident()?);
        while input.try_parse(|input| input.expect_delim('.')).is_ok() {
            name.push(input.expect_ident()?);
        }
        input.expect_exhausted()?;
        Ok(name)
    })?;
    let mut names = allocator.vec();
    names.extend(parsed);
    Ok(names)
}

fn parse_custom_media<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(&'i str, MediaList<'i>), ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let name = parser.expect_ident()?;
    if !name.starts_with("--") {
        return Err(parser.new_custom_error(ParserError::InvalidValue));
    }
    let query = parser
        .slice(parser.position()..SourcePosition(prelude.len()))
        .trim();
    if query.is_empty() {
        return Err(parser.new_custom_error(ParserError::InvalidValue));
    }
    Ok((name, parse_media_list(query, allocator)?))
}

fn parse_single_ident<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<&'i str, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let name = parser.expect_ident()?;
    parser.expect_exhausted()?;
    Ok(name)
}

fn parse_keyframes_name<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<KeyframesName<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let name = match parser.next()? {
        ValueToken::Ident(name)
            if !matches_ignore_case(
                name,
                &[
                    "none",
                    "initial",
                    "inherit",
                    "unset",
                    "default",
                    "revert",
                    "revert-layer",
                ],
            ) =>
        {
            KeyframesName::Ident(name)
        }
        ValueToken::String(name) => KeyframesName::Custom(name),
        _ => return Err(parser.new_custom_error(ParserError::InvalidValue)),
    };
    parser.expect_exhausted()?;
    Ok(name)
}

fn parse_keyframe_list<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, Keyframe<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut keyframes = allocator.vec();
    loop {
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }
        let parsed = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
            input.parse_comma_separated(parse_keyframe_selector)
        })?;
        let mut selectors = allocator.vec();
        selectors.extend(parsed);
        input.expect_curly_bracket_block()?;
        let declarations = input.parse_nested_block(|input| {
            parse_declaration_block(input, allocator, options, depth + 1)
        })?;
        keyframes.push(Keyframe {
            declarations: allocator.boxed(declarations),
            selectors,
        });
    }
    Ok(keyframes)
}

fn parse_keyframe_selector<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<KeyframeSelector<'i>, ParseError<'i, ParserError<'i>>> {
    match input.next()? {
        ValueToken::Percentage(value) if (0.0..=1.0).contains(value) => {
            Ok(KeyframeSelector::Percentage(*value))
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("from") => Ok(KeyframeSelector::From),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("to") => Ok(KeyframeSelector::To),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    }
}

fn parse_declaration_block<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<DeclarationBlock<'i>, ParseError<'i, ParserError<'i>>> {
    let (declarations, rules) = parse_style_contents(input, allocator, options, depth)?;
    if !rules.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidDeclaration));
    }
    Ok(declarations)
}

fn at_rule_vendor_prefix(name: &str) -> VendorPrefix {
    if name
        .get(..8)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-webkit-"))
    {
        VendorPrefix::WEBKIT
    } else if name
        .get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-moz-"))
    {
        VendorPrefix::MOZ
    } else if name
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-ms-"))
    {
        VendorPrefix::MS
    } else if name
        .get(..3)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-o-"))
    {
        VendorPrefix::O
    } else {
        VendorPrefix::NONE
    }
}

fn validate_moz_document_prelude<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    parser.expect_function_matching("url-prefix")?;
    parser.parse_nested_block(|input| {
        if !input.is_exhausted() && !input.expect_string()?.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        input.expect_exhausted()?;
        Ok(())
    })?;
    parser.expect_exhausted()?;
    Ok(())
}

type ContainerPrelude<'i> = (
    Option<&'i str>,
    Option<rs_css_allocator::boxed::Box<'i, rs_css_ast::ContainerCondition<'i>>>,
);

fn parse_container_prelude<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<ContainerPrelude<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let name = parser.try_parse(Parser::expect_ident).ok();
    let condition = if parser.is_exhausted() {
        None
    } else {
        Some(
            allocator.boxed(rs_css_ast::ContainerCondition::Unknown(collect_tokens(
                &mut parser,
                allocator,
                0,
            )?)),
        )
    };
    if name.is_none() && condition.is_none() {
        return Err(parser.new_custom_error(ParserError::InvalidValue));
    }
    Ok((name, condition))
}

type ScopePrelude<'i> = (
    Option<rs_css_allocator::boxed::Box<'i, SelectorList<'i>>>,
    Option<rs_css_allocator::boxed::Box<'i, SelectorList<'i>>>,
);

fn parse_scope_prelude<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<ScopePrelude<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let scope_start = if parser.try_parse(Parser::expect_parenthesis_block).is_ok() {
        Some(allocator.boxed(
            parser.parse_nested_block(|input| parse_selector_list(input, allocator, depth + 1))?,
        ))
    } else {
        None
    };

    let scope_end = if parser
        .try_parse(|input| input.expect_ident_matching("to"))
        .is_ok()
    {
        parser.expect_parenthesis_block()?;
        Some(allocator.boxed(
            parser.parse_nested_block(|input| parse_selector_list(input, allocator, depth + 1))?,
        ))
    } else {
        None
    };
    parser.expect_exhausted()?;
    Ok((scope_start, scope_end))
}

fn parse_page_selectors<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<Vec<'i, PageSelector<'i>>, ParseError<'i, ParserError<'i>>> {
    if prelude.is_empty() {
        return Ok(allocator.vec());
    }
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let parsed = parser.parse_comma_separated(|input| {
        let name = input.try_parse(Parser::expect_ident).ok();
        let mut pseudo_classes = allocator.vec();
        while input.try_parse(Parser::expect_colon).is_ok() {
            let pseudo = input.expect_ident()?;
            pseudo_classes.push(if pseudo.eq_ignore_ascii_case("left") {
                PagePseudoClass::Left
            } else if pseudo.eq_ignore_ascii_case("right") {
                PagePseudoClass::Right
            } else if pseudo.eq_ignore_ascii_case("first") {
                PagePseudoClass::First
            } else if pseudo.eq_ignore_ascii_case("last") {
                PagePseudoClass::Last
            } else if pseudo.eq_ignore_ascii_case("blank") {
                PagePseudoClass::Blank
            } else {
                return Err(input.new_custom_error(ParserError::InvalidSelector));
            });
        }
        if name.is_none() && pseudo_classes.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidSelector));
        }
        input.expect_exhausted()?;
        Ok(PageSelector {
            name,
            pseudo_classes,
        })
    })?;
    let mut selectors = allocator.vec();
    selectors.extend(parsed);
    Ok(selectors)
}

fn parse_page_body<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<(DeclarationBlock<'i>, Vec<'i, PageMarginRule<'i>>), ParseError<'i, ParserError<'i>>> {
    let mut declarations = DeclarationBlock::new(allocator);
    let mut rules = allocator.vec();

    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, ValueToken::Semicolon) {
            continue;
        }

        let result = match token {
            ValueToken::Ident(name) => {
                input.expect_colon()?;
                let (declaration, important) =
                    parse_declaration(input, allocator, name, depth + 1)?;
                declarations.push(declaration, important);
                Ok(None)
            }
            ValueToken::AtKeyword(name) => {
                let margin_box = page_margin_box(name)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidAtRule(name)))?;
                input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
                    input.expect_exhausted()?;
                    Ok::<_, ParseError<'i, ParserError<'i>>>(())
                })?;
                input.expect_curly_bracket_block()?;
                let declarations = input.parse_nested_block(|input| {
                    parse_declaration_block(input, allocator, options, depth + 1)
                })?;
                Ok(Some(PageMarginRule {
                    declarations: allocator.boxed(declarations),
                    span: span_from(&start, input.position()),
                    margin_box,
                }))
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };

        match result {
            Ok(Some(rule)) => rules.push(rule),
            Ok(None) => {}
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }

    Ok((declarations, rules))
}

fn parse_selector_string<'i>(
    source: &'i str,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<SelectorList<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(source, allocator);
    let mut parser = Parser::new(&mut input);
    parser.parse_entirely(|input| parse_selector_list(input, allocator, depth))
}

fn parse_family_names<'i>(
    source: &'i str,
    allocator: &'i Allocator,
) -> Result<Vec<'i, FamilyName<'i>>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(source, allocator);
    let mut parser = Parser::new(&mut input);
    let parsed = parser.parse_comma_separated(|input| {
        if let Ok(name) = input.try_parse(Parser::expect_string) {
            input.expect_exhausted()?;
            return Ok(FamilyName(name));
        }
        let mut name = std::string::String::new();
        while !input.is_exhausted() {
            if !name.is_empty() {
                name.push(' ');
            }
            name.push_str(input.expect_ident()?);
        }
        if name.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(FamilyName(allocator.alloc_str(&name)))
    })?;
    let mut names = allocator.vec();
    names.extend(parsed);
    Ok(names)
}

fn parse_font_feature_subrules<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, FontFeatureSubrule<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut rules = allocator.vec();
    loop {
        let start = input.state();
        let name = match input.next() {
            Ok(ValueToken::AtKeyword(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidRule)),
            Err(error) => return Err(error.into()),
        };
        let kind = font_feature_subrule_type(name)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidAtRule(name)))?;
        input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
            input.expect_exhausted()?;
            Ok::<_, ParseError<'i, ParserError<'i>>>(())
        })?;
        input.expect_curly_bracket_block()?;
        let declarations = input.parse_nested_block(|input| {
            parse_font_feature_declarations(input, allocator, options, depth + 1)
        })?;
        rules.push(FontFeatureSubrule {
            declarations,
            span: span_from(&start, input.position()),
            name: kind,
        });
    }
    Ok(rules)
}

fn parse_font_feature_declarations<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, FontFeatureDeclaration<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut declarations = allocator.vec();
    loop {
        let name = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let values = input.parse_until_before(Delimiter::Semicolon, |input| {
                let mut values = allocator.vec();
                while !input.is_exhausted() {
                    values.push(input.expect_integer()?);
                }
                if values.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                Ok(values)
            })?;
            let _ = input.try_parse(Parser::expect_semicolon);
            Ok::<_, ParseError<'i, ParserError<'i>>>(FontFeatureDeclaration { name, values })
        })();
        match result {
            Ok(declaration) => declarations.push(declaration),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(declarations)
}

fn font_feature_subrule_type(name: &str) -> Option<FontFeatureSubruleType> {
    Some(if name.eq_ignore_ascii_case("stylistic") {
        FontFeatureSubruleType::Stylistic
    } else if name.eq_ignore_ascii_case("historical-forms") {
        FontFeatureSubruleType::HistoricalForms
    } else if name.eq_ignore_ascii_case("styleset") {
        FontFeatureSubruleType::Styleset
    } else if name.eq_ignore_ascii_case("character-variant") {
        FontFeatureSubruleType::CharacterVariant
    } else if name.eq_ignore_ascii_case("swash") {
        FontFeatureSubruleType::Swash
    } else if name.eq_ignore_ascii_case("ornaments") {
        FontFeatureSubruleType::Ornaments
    } else if name.eq_ignore_ascii_case("annotation") {
        FontFeatureSubruleType::Annotation
    } else {
        return None;
    })
}

fn parse_font_palette_contents<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, FontPaletteValuesProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = allocator.vec();
    loop {
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, ValueToken::Semicolon) {
            continue;
        }
        let result = match token {
            ValueToken::Ident(name) => {
                input.expect_colon()?;
                let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                    collect_tokens(input, allocator, depth + 1)
                })?;
                let _ = input.try_parse(Parser::expect_semicolon);
                if remove_important(&mut value) {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                trim_leading_whitespace(&mut value);
                Ok(FontPaletteValuesProperty::Custom(allocator.boxed(
                    CustomProperty {
                        name: allocator.boxed(CustomPropertyName::Unknown(name)),
                        value,
                    },
                )))
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };
        match result {
            Ok(property) => properties.push(property),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(properties)
}

fn parse_property_rule<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    name: &'i str,
) -> Result<PropertyRule<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut syntax = None;
    let mut inherits = None;
    let mut initial_value = None;

    loop {
        let descriptor = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, allocator, depth + 1)
            })?;
            let _ = input.try_parse(Parser::expect_semicolon);
            if remove_important(&mut value) {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            trim_leading_whitespace(&mut value);

            if descriptor.eq_ignore_ascii_case("syntax") {
                let Some(ValueToken::String(value)) = single_token(&value) else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                syntax = Some(parse_syntax_string(value, allocator)?);
            } else if descriptor.eq_ignore_ascii_case("inherits") {
                let Some(value) = value.first().and_then(token_ident) else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                if value.eq_ignore_ascii_case("true") {
                    inherits = Some(true);
                } else if value.eq_ignore_ascii_case("false") {
                    inherits = Some(false);
                } else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
            } else if descriptor.eq_ignore_ascii_case("initial-value") {
                initial_value = Some(allocator.boxed(ParsedComponent::TokenList(value)));
            }
            Ok::<_, ParseError<'i, ParserError<'i>>>(())
        })();
        if let Err(error) = result {
            if options.error_recovery {
                recover_declaration(input);
            } else {
                return Err(error);
            }
        }
    }

    let syntax = syntax.ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let is_universal = matches!(syntax, SyntaxString::Universal);
    let inherits = inherits.ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    if !is_universal && initial_value.is_none() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok(PropertyRule {
        inherits,
        initial_value,
        span: Span::default(),
        name,
        syntax: allocator.boxed(syntax),
    })
}

fn parse_syntax_string<'i>(
    value: &'i str,
    allocator: &'i Allocator,
) -> Result<SyntaxString<'i>, ParseError<'i, ParserError<'i>>> {
    if value == "*" {
        return Ok(SyntaxString::Universal);
    }
    let mut components = allocator.vec();
    for raw_component in value.split('|') {
        let raw_component = raw_component.trim();
        let (component, multiplier) = if let Some(component) = raw_component.strip_suffix('+') {
            (component.trim_end(), Multiplier::Space)
        } else if let Some(component) = raw_component.strip_suffix('#') {
            (component.trim_end(), Multiplier::Comma)
        } else {
            (raw_component, Multiplier::None)
        };
        let kind = if component.eq_ignore_ascii_case("<length>") {
            SyntaxComponentKind::Length
        } else if component.eq_ignore_ascii_case("<number>") {
            SyntaxComponentKind::Number
        } else if component.eq_ignore_ascii_case("<percentage>") {
            SyntaxComponentKind::Percentage
        } else if component.eq_ignore_ascii_case("<length-percentage>") {
            SyntaxComponentKind::LengthPercentage
        } else if component.eq_ignore_ascii_case("<string>") {
            SyntaxComponentKind::String
        } else if component.eq_ignore_ascii_case("<color>") {
            SyntaxComponentKind::Color
        } else if component.eq_ignore_ascii_case("<image>") {
            SyntaxComponentKind::Image
        } else if component.eq_ignore_ascii_case("<url>") {
            SyntaxComponentKind::Url
        } else if component.eq_ignore_ascii_case("<integer>") {
            SyntaxComponentKind::Integer
        } else if component.eq_ignore_ascii_case("<angle>") {
            SyntaxComponentKind::Angle
        } else if component.eq_ignore_ascii_case("<time>") {
            SyntaxComponentKind::Time
        } else if component.eq_ignore_ascii_case("<resolution>") {
            SyntaxComponentKind::Resolution
        } else if component.eq_ignore_ascii_case("<transform-function>") {
            SyntaxComponentKind::TransformFunction
        } else if component.eq_ignore_ascii_case("<transform-list>") {
            SyntaxComponentKind::TransformList
        } else if component.eq_ignore_ascii_case("<custom-ident>") {
            SyntaxComponentKind::CustomIdent
        } else if !component.is_empty()
            && component
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
        {
            SyntaxComponentKind::Literal(component)
        } else {
            return Err(
                crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue)
            );
        };
        components.push(SyntaxComponent {
            kind: allocator.boxed(kind),
            multiplier,
        });
    }
    if components.is_empty() {
        return Err(crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue));
    }
    Ok(SyntaxString::Components(components))
}

fn parse_view_transition_contents<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, ViewTransitionProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = allocator.vec();
    loop {
        let descriptor = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, allocator, depth + 1)
            })?;
            let _ = input.try_parse(Parser::expect_semicolon);
            if remove_important(&mut value) {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            trim_leading_whitespace(&mut value);

            let property = if descriptor.eq_ignore_ascii_case("navigation") {
                let value = value
                    .first()
                    .and_then(token_ident)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                ViewTransitionProperty::Navigation(if value.eq_ignore_ascii_case("auto") {
                    Navigation::Auto
                } else if value.eq_ignore_ascii_case("none") {
                    Navigation::None
                } else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                })
            } else if descriptor.eq_ignore_ascii_case("types") {
                let mut idents = allocator.vec();
                for token in &value {
                    if let Some(ident) = token_ident(token) {
                        idents.push(ident);
                    } else if !matches!(token, TokenOrValue::Token(token) if matches!(**token, ValueToken::WhiteSpace(_)))
                    {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                }
                let types = if idents.len() == 1 && idents[0].eq_ignore_ascii_case("none") {
                    NoneOrCustomIdentList::None
                } else if idents.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                } else {
                    NoneOrCustomIdentList::Idents(idents)
                };
                ViewTransitionProperty::Types(allocator.boxed(types))
            } else {
                ViewTransitionProperty::Custom(allocator.boxed(CustomProperty {
                    name: allocator.boxed(CustomPropertyName::Unknown(descriptor)),
                    value,
                }))
            };
            Ok::<_, ParseError<'i, ParserError<'i>>>(property)
        })();

        match result {
            Ok(property) => properties.push(property),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(properties)
}

fn page_margin_box(name: &str) -> Option<PageMarginBox> {
    Some(if name.eq_ignore_ascii_case("top-left-corner") {
        PageMarginBox::TopLeftCorner
    } else if name.eq_ignore_ascii_case("top-left") {
        PageMarginBox::TopLeft
    } else if name.eq_ignore_ascii_case("top-center") {
        PageMarginBox::TopCenter
    } else if name.eq_ignore_ascii_case("top-right") {
        PageMarginBox::TopRight
    } else if name.eq_ignore_ascii_case("top-right-corner") {
        PageMarginBox::TopRightCorner
    } else if name.eq_ignore_ascii_case("left-top") {
        PageMarginBox::LeftTop
    } else if name.eq_ignore_ascii_case("left-middle") {
        PageMarginBox::LeftMiddle
    } else if name.eq_ignore_ascii_case("left-bottom") {
        PageMarginBox::LeftBottom
    } else if name.eq_ignore_ascii_case("right-top") {
        PageMarginBox::RightTop
    } else if name.eq_ignore_ascii_case("right-middle") {
        PageMarginBox::RightMiddle
    } else if name.eq_ignore_ascii_case("right-bottom") {
        PageMarginBox::RightBottom
    } else if name.eq_ignore_ascii_case("bottom-left-corner") {
        PageMarginBox::BottomLeftCorner
    } else if name.eq_ignore_ascii_case("bottom-left") {
        PageMarginBox::BottomLeft
    } else if name.eq_ignore_ascii_case("bottom-center") {
        PageMarginBox::BottomCenter
    } else if name.eq_ignore_ascii_case("bottom-right") {
        PageMarginBox::BottomRight
    } else if name.eq_ignore_ascii_case("bottom-right-corner") {
        PageMarginBox::BottomRightCorner
    } else {
        return None;
    })
}

fn parse_import<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
    start: &ParserState,
    end: SourcePosition,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let url = parser.expect_url_or_string()?;

    let layer = if parser
        .try_parse(|input| input.expect_ident_matching("layer"))
        .is_ok()
    {
        Some(allocator.vec())
    } else if parser
        .try_parse(|input| input.expect_function_matching("layer"))
        .is_ok()
    {
        Some(parser.parse_nested_block(|input| {
            let mut name = allocator.vec();
            name.push(input.expect_ident()?);
            while input.try_parse(|input| input.expect_delim('.')).is_ok() {
                name.push(input.expect_ident()?);
            }
            input.expect_exhausted()?;
            Ok::<_, ParseError<'i, ParserError<'i>>>(name)
        })?)
    } else {
        None
    };

    let supports = if parser
        .try_parse(|input| input.expect_function_matching("supports"))
        .is_ok()
    {
        Some(allocator.boxed(parser.parse_nested_block(|input| {
            let start = input.position();
            input.expect_no_error_token()?;
            let raw = input.slice_from(start).trim();
            if raw.is_empty() {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            Ok::<_, ParseError<'i, ParserError<'i>>>(parse_supports_condition(raw))
        })?))
    } else {
        None
    };

    let media = if parser.is_exhausted() {
        None
    } else {
        let rest = parser
            .slice(parser.position()..SourcePosition(prelude.len()))
            .trim();
        if rest.is_empty() {
            None
        } else {
            Some(allocator.boxed(parse_media_list(rest, allocator)?))
        }
    };
    Ok(CssRule::Import(allocator.boxed(ImportRule {
        layer,
        span: span_from(start, end),
        media,
        supports,
        url,
    })))
}

fn parse_media_list<'i>(
    source: &'i str,
    allocator: &'i Allocator,
) -> Result<MediaList<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(source, allocator);
    let mut parser = Parser::new(&mut input);
    let parsed = parser.parse_comma_separated(|input| {
        let qualifier = input
            .try_parse(|input| {
                let name = input.expect_ident()?;
                if name.eq_ignore_ascii_case("only") {
                    Ok::<_, ParseError<'i, ParserError<'i>>>(Qualifier::Only)
                } else if name.eq_ignore_ascii_case("not") {
                    Ok(Qualifier::Not)
                } else {
                    Err(input.new_custom_error(ParserError::InvalidValue))
                }
            })
            .ok();

        let type_state = input.state();
        let media_type = match input.try_parse(Parser::expect_ident) {
            Ok(name) if name.eq_ignore_ascii_case("all") => MediaType::All,
            Ok(name) if name.eq_ignore_ascii_case("print") => MediaType::Print,
            Ok(name) if name.eq_ignore_ascii_case("screen") => MediaType::Screen,
            Ok(name) if !name.eq_ignore_ascii_case("and") && !name.eq_ignore_ascii_case("or") => {
                MediaType::Custom(name)
            }
            Ok(_) | Err(_) => {
                input.reset(&type_state);
                MediaType::All
            }
        };

        let condition = if input.is_exhausted() {
            None
        } else {
            Some(allocator.boxed(MediaCondition::Unknown(collect_tokens(
                input, allocator, 0,
            )?)))
        };
        Ok(MediaQuery {
            condition,
            media_type: allocator.boxed(media_type),
            qualifier,
        })
    })?;
    let mut media_queries = allocator.vec();
    media_queries.extend(parsed);
    Ok(MediaList { media_queries })
}

fn parse_supports_condition(source: &str) -> SupportsCondition<'_> {
    SupportsCondition::Unknown(source)
}

fn remove_important(value: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let Some(important_index) = previous_non_whitespace(value, value.len()) else {
        return false;
    };
    if !token_ident(&value[important_index])
        .is_some_and(|name| name.eq_ignore_ascii_case("important"))
    {
        trim_trailing_whitespace(value);
        return false;
    }
    let Some(bang_index) = previous_non_whitespace(value, important_index) else {
        trim_trailing_whitespace(value);
        return false;
    };
    if !matches!(&value[bang_index], TokenOrValue::Token(token) if matches!(**token, ValueToken::Delim("!")))
    {
        trim_trailing_whitespace(value);
        return false;
    }
    value.truncate(bang_index);
    trim_trailing_whitespace(value);
    true
}

fn previous_non_whitespace(value: &[TokenOrValue<'_>], before: usize) -> Option<usize> {
    (0..before).rev().find(|index| {
        !matches!(&value[*index], TokenOrValue::Token(token) if matches!(**token, ValueToken::WhiteSpace(_) | ValueToken::Comment(_)))
    })
}

fn trim_trailing_whitespace(value: &mut Vec<'_, TokenOrValue<'_>>) {
    while matches!(value.last(), Some(TokenOrValue::Token(token)) if matches!(**token, ValueToken::WhiteSpace(_) | ValueToken::Comment(_)))
    {
        value.pop();
    }
}

fn trim_leading_whitespace(value: &mut Vec<'_, TokenOrValue<'_>>) {
    while matches!(value.first(), Some(TokenOrValue::Token(token)) if matches!(**token, ValueToken::WhiteSpace(_) | ValueToken::Comment(_)))
    {
        value.remove(0);
    }
}

fn token_ident<'i>(value: &TokenOrValue<'i>) -> Option<&'i str> {
    match value {
        TokenOrValue::Token(token) => match **token {
            ValueToken::Ident(name) => Some(name),
            _ => None,
        },
        _ => None,
    }
}

fn css_wide_keyword(value: &str) -> Option<CSSWideKeyword> {
    if value.eq_ignore_ascii_case("initial") {
        Some(CSSWideKeyword::Initial)
    } else if value.eq_ignore_ascii_case("inherit") {
        Some(CSSWideKeyword::Inherit)
    } else if value.eq_ignore_ascii_case("unset") {
        Some(CSSWideKeyword::Unset)
    } else if value.eq_ignore_ascii_case("revert") {
        Some(CSSWideKeyword::Revert)
    } else if value.eq_ignore_ascii_case("revert-layer") {
        Some(CSSWideKeyword::RevertLayer)
    } else {
        None
    }
}

fn ascii_lowercase<'i>(value: &'i str, allocator: &'i Allocator) -> &'i str {
    if value.bytes().all(|byte| !byte.is_ascii_uppercase()) {
        value
    } else {
        allocator.alloc_str(&value.to_ascii_lowercase())
    }
}

fn matches_ignore_case(value: &str, expected: &[&str]) -> bool {
    expected.iter().any(|item| value.eq_ignore_ascii_case(item))
}

fn check_depth<'i>(
    input: &Parser<'i, '_>,
    depth: usize,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    if depth > MAX_NESTING_DEPTH {
        Err(input.new_custom_error(ParserError::MaximumNestingDepth))
    } else {
        Ok(())
    }
}

fn span_from(start: &ParserState, end: SourcePosition) -> Span {
    Span::new(
        start.position().byte_index() as u32,
        end.byte_index() as u32,
    )
}

fn recover_rule(input: &mut Parser<'_, '_>) {
    let _ = input.next_including_whitespace_and_comments();
}

fn recover_declaration(input: &mut Parser<'_, '_>) {
    let _: Result<(), ParseError<'_, ()>> =
        input.parse_until_after(Delimiter::Semicolon, |_| Ok(()));
}

fn into_error<'i>(error: ParseError<'i, ParserError<'i>>, filename: &'i str) -> Error<'i> {
    let kind = match error.kind {
        ParseErrorKind::Custom(error) => error,
        ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(token)) => {
            ParserError::UnexpectedToken(token)
        }
        ParseErrorKind::Basic(BasicParseErrorKind::AtRuleInvalid(name)) => {
            ParserError::InvalidAtRule(name)
        }
        ParseErrorKind::Basic(_) => ParserError::InvalidRule,
    };
    Error {
        kind,
        filename,
        location: error.location,
    }
}
