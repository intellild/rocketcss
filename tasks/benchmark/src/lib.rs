//! Shared inputs for the CSS tokenizer, parser, and code generator benchmarks.

use std::fmt;

#[derive(Clone, Copy)]
pub struct BenchCase {
    pub name: &'static str,
    pub file_name: &'static str,
    pub source: &'static str,
    pub pipeline_iterations: usize,
}

pub const WRITER_CAPACITY_PADDING: usize = 1024;

impl fmt::Display for BenchCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name)
    }
}

pub const BENCH_CASES: &[BenchCase] = &[
    BenchCase {
        name: "bootstrap",
        file_name: "bootstrap.css",
        source: include_str!("../files/bootstrap.css"),
        pipeline_iterations: 10,
    },
    BenchCase {
        name: "tailwind",
        file_name: "tailwind.css",
        source: include_str!("../files/tailwind.css"),
        pipeline_iterations: 1,
    },
];
