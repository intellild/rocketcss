use rs_css_allocator::{boxed::Box as ArenaBox, vec::Vec as ArenaVec};
use rs_css_ast::{
    Declaration, DeclarationBlock, PropertyId, Token, TokenOrValue, UnparsedProperty,
};

use crate::{
    MinifyContext,
    context::{PropertyContext, ValueContext},
};

pub(crate) fn minify_declaration_block<'a>(
    declarations: &mut DeclarationBlock<'a>,
    context: &mut MinifyContext<'a>,
) {
    if declarations.len() < 2 {
        return;
    }

    let old = std::mem::replace(declarations, DeclarationBlock::new(context.allocator()));
    let mut kept = std::vec::Vec::with_capacity(old.declarations.len());
    if context.options().discard_duplicates {
        for (declaration, important) in old
            .declarations
            .into_iter()
            .zip(old.declarations_importance.iter())
            .rev()
        {
            if kept.iter().any(|(existing, existing_important)| {
                *existing_important == important && existing == &declaration
            }) {
                context.record_declarations_removed(1);
            } else {
                kept.push((declaration, important));
            }
        }
        kept.reverse();
    } else {
        kept.extend(
            old.declarations
                .into_iter()
                .zip(old.declarations_importance.iter()),
        );
    }

    if context.options().merge_rules {
        let merged = merge_box_longhands(&mut kept, context);
        context.record_declarations_removed(merged * 3);
    }
    for (declaration, important) in kept {
        declarations.push(declaration, important);
    }
}

fn merge_box_longhands<'a>(
    declarations: &mut std::vec::Vec<(Declaration<'a>, bool)>,
    context: &MinifyContext<'a>,
) -> usize {
    let mut merged = 0;
    let mut index = 0;
    while index + 4 <= declarations.len() {
        let important = declarations[index].1;
        if declarations[index..index + 4]
            .iter()
            .any(|(_, candidate_important)| *candidate_important != important)
        {
            index += 1;
            continue;
        }
        let Some(kind) = box_longhand(&declarations[index].0).map(|(kind, _)| kind) else {
            index += 1;
            continue;
        };
        let mut seen = [false; 4];
        let matches = declarations[index..index + 4]
            .iter()
            .all(|(declaration, _)| {
                box_longhand(declaration).is_some_and(|(candidate_kind, side)| {
                    if candidate_kind == kind && !seen[side] {
                        seen[side] = true;
                        true
                    } else {
                        false
                    }
                })
            });
        if !matches {
            index += 1;
            continue;
        }

        let drained: std::vec::Vec<_> = declarations.drain(index..index + 4).collect();
        let mut sides: [Option<ArenaVec<'a, TokenOrValue<'a>>>; 4] = [None, None, None, None];
        for (declaration, _) in drained {
            let (_, side) = box_longhand(&declaration).expect("longhand was classified above");
            let Declaration::Unparsed(value) = declaration else {
                unreachable!("only unparsed box longhands are merged")
            };
            sides[side] = Some(ArenaBox::into_inner(value).value);
        }
        let selected: &[usize] =
            if sides[0] == sides[1] && sides[1] == sides[2] && sides[2] == sides[3] {
                &[0]
            } else if sides[0] == sides[2] && sides[1] == sides[3] {
                &[0, 1]
            } else if sides[1] == sides[3] {
                &[0, 1, 2]
            } else {
                &[0, 1, 2, 3]
            };
        let mut value = context.allocator().vec();
        for (output_index, side) in selected.iter().copied().enumerate() {
            if output_index > 0 {
                value.push(TokenOrValue::Token(
                    context.allocator().boxed(Token::WhiteSpace(" ")),
                ));
            }
            value.extend(sides[side].take().expect("each side appears exactly once"));
        }
        let property_id = match kind {
            BoxKind::Margin => PropertyId::Margin,
            BoxKind::Padding => PropertyId::Padding,
        };
        declarations.insert(
            index,
            (
                Declaration::Unparsed(context.allocator().boxed(UnparsedProperty {
                    property_id: context.allocator().boxed(property_id),
                    value,
                })),
                important,
            ),
        );
        merged += 1;
        index += 1;
    }
    merged
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BoxKind {
    Margin,
    Padding,
}

fn box_longhand(declaration: &Declaration<'_>) -> Option<(BoxKind, usize)> {
    let Declaration::Unparsed(value) = declaration else {
        return None;
    };
    Some(match &*value.property_id {
        PropertyId::MarginTop => (BoxKind::Margin, 0),
        PropertyId::MarginRight => (BoxKind::Margin, 1),
        PropertyId::MarginBottom => (BoxKind::Margin, 2),
        PropertyId::MarginLeft => (BoxKind::Margin, 3),
        PropertyId::PaddingTop => (BoxKind::Padding, 0),
        PropertyId::PaddingRight => (BoxKind::Padding, 1),
        PropertyId::PaddingBottom => (BoxKind::Padding, 2),
        PropertyId::PaddingLeft => (BoxKind::Padding, 3),
        _ => return None,
    })
}

pub(crate) fn value_context(property_id: &PropertyId<'_>) -> ValueContext {
    let name = property_id.name();
    let allow_color = name.contains("color")
        || name.contains("background")
        || name.contains("border")
        || name.contains("shadow")
        || name.contains("outline")
        || name.contains("decoration")
        || matches!(name, "fill" | "stroke" | "caret" | "column-rule");
    ValueContext {
        allow_color,
        allow_unitless_zero: true,
        skip_value_transforms: false,
        property: if name.eq_ignore_ascii_case("display") {
            PropertyContext::Display
        } else if name.eq_ignore_ascii_case("font-family") {
            PropertyContext::FontFamily
        } else if name.eq_ignore_ascii_case("font-weight") {
            PropertyContext::FontWeight
        } else if name.eq_ignore_ascii_case("transform") || name.ends_with("-transform") {
            PropertyContext::Transform
        } else if name.contains("timing-function")
            || name.eq_ignore_ascii_case("animation")
            || name.ends_with("-animation")
            || name.eq_ignore_ascii_case("transition")
            || name.ends_with("-transition")
        {
            PropertyContext::Timing
        } else if name.eq_ignore_ascii_case("background-repeat")
            || name.eq_ignore_ascii_case("mask-repeat")
            || name.eq_ignore_ascii_case("background")
            || name.eq_ignore_ascii_case("mask")
        {
            PropertyContext::Repeat
        } else if name.eq_ignore_ascii_case("background-position")
            || name.eq_ignore_ascii_case("perspective-origin")
            || name.ends_with("-perspective-origin")
        {
            PropertyContext::Position
        } else if is_box_value(name) {
            PropertyContext::Box
        } else {
            PropertyContext::Generic
        },
    }
}

pub(crate) fn custom_property_context(context: &MinifyContext<'_>) -> ValueContext {
    ValueContext {
        allow_color: context.options().transform_custom_properties,
        allow_unitless_zero: false,
        property: PropertyContext::Generic,
        skip_value_transforms: !context.options().transform_custom_properties,
    }
}

fn is_box_value(name: &str) -> bool {
    matches!(
        name,
        "margin"
            | "padding"
            | "border-color"
            | "border-style"
            | "border-width"
            | "inset"
            | "scroll-margin"
            | "scroll-padding"
    )
}
