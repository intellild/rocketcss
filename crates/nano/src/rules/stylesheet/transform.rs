use super::*;

pub(super) fn minify_transform_function<'ast>(
    function: &mut Function<'ast>,
    arguments: &mut Vec<'ast, TokenOrValue<'ast>>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) -> bool {
    if function.kind() == KnownFunction::RotateZ && arguments.len() == 1 {
        function.set_name("rotate", ast.ast_context_mut());
        return true;
    }
    if function.kind() == KnownFunction::Matrix3d {
        let values = arguments.as_slice();
        if values.len() == 31
            && number_at(values, 4, ast) == Some(0.0)
            && number_at(values, 6, ast) == Some(0.0)
            && number_at(values, 12, ast) == Some(0.0)
            && number_at(values, 14, ast) == Some(0.0)
            && number_at(values, 16, ast) == Some(0.0)
            && number_at(values, 18, ast) == Some(0.0)
            && number_at(values, 20, ast) == Some(1.0)
            && number_at(values, 22, ast) == Some(0.0)
            && number_at(values, 28, ast) == Some(0.0)
            && number_at(values, 30, ast) == Some(1.0)
        {
            function.set_name("matrix", ast.ast_context_mut());
            compact_arguments(arguments, &[0, 1, 2, 3, 8, 9, 10, 11, 24, 25, 26]);
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Rotate3d && arguments.len() == 7 {
        let name = match (
            number_at(arguments, 0, ast),
            number_at(arguments, 2, ast),
            number_at(arguments, 4, ast),
        ) {
            (Some(1.0), Some(0.0), Some(0.0)) => "rotateX",
            (Some(0.0), Some(1.0), Some(0.0)) => "rotateY",
            (Some(0.0), Some(0.0), Some(1.0)) => "rotate",
            _ => return false,
        };
        function.set_name(name, ast.ast_context_mut());
        compact_arguments(arguments, &[6]);
        return true;
    }
    if function.kind() == KnownFunction::Scale && arguments.len() == 3 {
        if crate::token::token_or_value_eq(&arguments[0], &arguments[2], ast)
            && !is_empty_variable_function(&arguments[0], ast)
        {
            arguments.truncate(1);
            return true;
        }
        let first = number_at(arguments, 0, ast);
        let second = number_at(arguments, 2, ast);
        if first == second && first.is_some() {
            arguments.truncate(1);
            return true;
        }
        if second == Some(1.0) {
            function.set_name("scaleX", ast.ast_context_mut());
            arguments.truncate(1);
            return true;
        }
        if first == Some(1.0) {
            function.set_name("scaleY", ast.ast_context_mut());
            compact_arguments(arguments, &[2]);
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Scale3d && arguments.len() == 5 {
        let values = [
            number_at(arguments, 0, ast),
            number_at(arguments, 2, ast),
            number_at(arguments, 4, ast),
        ];
        let (name, index) = if values[1] == Some(1.0) && values[2] == Some(1.0) {
            ("scaleX", 0)
        } else if values[0] == Some(1.0) && values[2] == Some(1.0) {
            ("scaleY", 2)
        } else if values[0] == Some(1.0) && values[1] == Some(1.0) {
            ("scaleZ", 4)
        } else {
            return false;
        };
        function.set_name(name, ast.ast_context_mut());
        compact_arguments(arguments, &[index]);
        return true;
    }
    if function.kind() == KnownFunction::Translate && arguments.len() == 3 {
        if number_at(arguments, 2, ast) == Some(0.0) {
            arguments.truncate(1);
            return true;
        }
        if number_at(arguments, 0, ast) == Some(0.0) {
            function.set_name("translateY", ast.ast_context_mut());
            compact_arguments(arguments, &[2]);
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Translate3d
        && arguments.len() == 5
        && number_at(arguments, 0, ast) == Some(0.0)
        && number_at(arguments, 2, ast) == Some(0.0)
    {
        function.set_name("translateZ", ast.ast_context_mut());
        compact_arguments(arguments, &[4]);
        return true;
    }
    false
}

fn is_empty_variable_function(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Function(function)
    if {
        let function = ast.ast_context().resolve_node(*function);
        ast.ast_context().vec_len(function.arguments) == 0 && function.kind().is_variable()
    })
}

fn compact_arguments(
    arguments: &mut rocketcss_common::vec::Vec<'_, TokenOrValue<'_>>,
    indices: &[usize],
) {
    for (target, &source) in indices.iter().enumerate() {
        if target != source {
            arguments.swap(target, source);
        }
    }
    arguments.truncate(indices.len());
}
