use super::*;

pub(super) fn minify_transform_function(function: &mut Function<'_>) -> bool {
    if function.kind() == KnownFunction::RotateZ && function.arguments.len() == 1 {
        function.set_name("rotate");
        return true;
    }
    if function.kind() == KnownFunction::Matrix3d {
        let values = &function.arguments;
        if values.len() == 31
            && number_at(values, 4) == Some(0.0)
            && number_at(values, 6) == Some(0.0)
            && number_at(values, 12) == Some(0.0)
            && number_at(values, 14) == Some(0.0)
            && number_at(values, 16) == Some(0.0)
            && number_at(values, 18) == Some(0.0)
            && number_at(values, 20) == Some(1.0)
            && number_at(values, 22) == Some(0.0)
            && number_at(values, 28) == Some(0.0)
            && number_at(values, 30) == Some(1.0)
        {
            function.set_name("matrix");
            compact_arguments(
                &mut function.arguments,
                &[0, 1, 2, 3, 8, 9, 10, 11, 24, 25, 26],
            );
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Rotate3d && function.arguments.len() == 7 {
        let name = match (
            number_at(&function.arguments, 0),
            number_at(&function.arguments, 2),
            number_at(&function.arguments, 4),
        ) {
            (Some(1.0), Some(0.0), Some(0.0)) => "rotateX",
            (Some(0.0), Some(1.0), Some(0.0)) => "rotateY",
            (Some(0.0), Some(0.0), Some(1.0)) => "rotate",
            _ => return false,
        };
        function.set_name(name);
        compact_arguments(&mut function.arguments, &[6]);
        return true;
    }
    if function.kind() == KnownFunction::Scale && function.arguments.len() == 3 {
        if function.arguments[0] == function.arguments[2]
            && !is_empty_variable_function(&function.arguments[0])
        {
            function.arguments.truncate(1);
            return true;
        }
        let first = number_at(&function.arguments, 0);
        let second = number_at(&function.arguments, 2);
        if first == second && first.is_some() {
            function.arguments.truncate(1);
            return true;
        }
        if second == Some(1.0) {
            function.set_name("scaleX");
            function.arguments.truncate(1);
            return true;
        }
        if first == Some(1.0) {
            function.set_name("scaleY");
            compact_arguments(&mut function.arguments, &[2]);
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Scale3d && function.arguments.len() == 5 {
        let values = [
            number_at(&function.arguments, 0),
            number_at(&function.arguments, 2),
            number_at(&function.arguments, 4),
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
        function.set_name(name);
        compact_arguments(&mut function.arguments, &[index]);
        return true;
    }
    if function.kind() == KnownFunction::Translate && function.arguments.len() == 3 {
        if number_at(&function.arguments, 2) == Some(0.0) {
            function.arguments.truncate(1);
            return true;
        }
        if number_at(&function.arguments, 0) == Some(0.0) {
            function.set_name("translateY");
            compact_arguments(&mut function.arguments, &[2]);
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Translate3d
        && function.arguments.len() == 5
        && number_at(&function.arguments, 0) == Some(0.0)
        && number_at(&function.arguments, 2) == Some(0.0)
    {
        function.set_name("translateZ");
        compact_arguments(&mut function.arguments, &[4]);
        return true;
    }
    false
}

fn is_empty_variable_function(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Function(function)
        if function.arguments.is_empty() && function.kind().is_variable())
}

fn compact_arguments(
    arguments: &mut rocketcss_allocator::vec::Vec<'_, TokenOrValue<'_>>,
    indices: &[usize],
) {
    for (target, &source) in indices.iter().enumerate() {
        if target != source {
            arguments.swap(target, source);
        }
    }
    arguments.truncate(indices.len());
}
