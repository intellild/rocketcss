use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

mod codegen;
mod parser;
mod visitor;

fn read_fixture(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read fixture {}: {error}", path.display()))
        .replace("\r\n", "\n")
}

fn expected_path(input: &Path) -> std::path::PathBuf {
    input.with_file_name("output.css")
}

fn fixture_paths(relative_dir: &str) -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(relative_dir);
    let mut paths = Vec::new();
    collect_fixture_paths(&root, &mut paths);
    paths.sort();
    assert!(!paths.is_empty(), "no fixtures found in {}", root.display());
    paths
}

fn collect_fixture_paths(dir: &Path, paths: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|error| {
        panic!(
            "failed to read fixture directory {}: {error}",
            dir.display()
        )
    });

    for entry in entries {
        let entry = entry
            .unwrap_or_else(|error| panic!("failed to read entry in {}: {error}", dir.display()));
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_paths(&path, paths);
        } else if path.file_name() == Some(OsStr::new("input.css")) {
            paths.push(path);
        }
    }
}
