use super::*;

pub(super) fn can_unquote_url(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(|character| {
            character.is_whitespace()
                || character.is_control()
                || matches!(character, '(' | ')' | '\\')
        })
}

pub(crate) fn normalize_url_text(value: &str) -> Option<std::string::String> {
    let trimmed = value.trim();
    if trimmed
        .get(..5)
        .is_some_and(|prefix| match_ignore_ascii_case!(prefix, "data:" => true, _ => false))
    {
        return (trimmed != value).then(|| trimmed.to_owned());
    }

    let suffix_start = trimmed.find(['?', '#']).unwrap_or(trimmed.len());
    let (base, suffix) = trimmed.split_at(suffix_start);
    let (authority, path) = split_url_authority(base);
    let authority = normalize_url_authority(authority);
    let path = normalize_url_path(path);
    let mut normalized = std::string::String::with_capacity(trimmed.len());
    normalized.push_str(&authority);
    normalized.push_str(&path);
    normalized.push_str(suffix);
    (normalized != value).then_some(normalized)
}

fn split_url_authority(value: &str) -> (&str, &str) {
    let authority_start = if let Some(scheme) = value.find("://") {
        scheme + 3
    } else if value.starts_with("//") {
        2
    } else {
        return ("", value);
    };
    let Some(path_start) = value[authority_start..].find('/') else {
        return (value, "");
    };
    let path_start = authority_start + path_start + 1;
    (&value[..path_start], &value[path_start..])
}

fn normalize_url_authority(value: &str) -> std::string::String {
    if value.is_empty() {
        return std::string::String::new();
    }
    let without_slash = value.strip_suffix('/').unwrap_or(value);
    let default_port = if without_slash
        .get(..7)
        .is_some_and(|prefix| match_ignore_ascii_case!(prefix, "http://" => true, _ => false))
        || without_slash.starts_with("//")
    {
        Some(":80")
    } else if without_slash
        .get(..8)
        .is_some_and(|prefix| match_ignore_ascii_case!(prefix, "https://" => true, _ => false))
    {
        Some(":443")
    } else {
        None
    };
    let Some(port) = default_port.filter(|port| without_slash.ends_with(port)) else {
        return value.to_owned();
    };
    let mut normalized = std::string::String::with_capacity(value.len() - port.len());
    normalized.push_str(&without_slash[..without_slash.len() - port.len()]);
    if value.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

fn normalize_url_path(value: &str) -> std::string::String {
    if value.is_empty() {
        return std::string::String::new();
    }
    let leading_slash = value.starts_with('/');
    let trailing_slash = value.ends_with('/');
    let mut segments = std::vec::Vec::new();
    for segment in value.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if segments.last().is_some_and(|segment| *segment != "..") {
                    segments.pop();
                } else if !leading_slash {
                    segments.push(segment);
                }
            }
            _ => segments.push(segment),
        }
    }
    let mut normalized = std::string::String::with_capacity(value.len());
    if leading_slash {
        normalized.push('/');
    }
    for (index, segment) in segments.iter().enumerate() {
        if index != 0 {
            normalized.push('/');
        }
        normalized.push_str(segment);
    }
    if trailing_slash && !normalized.ends_with('/') {
        normalized.push('/');
    }
    normalized
}
