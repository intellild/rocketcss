use std::{fs, path::Path};

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
