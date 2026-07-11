//! Shared inputs for the CSS tokenizer, parser, and code generator benchmarks.

use std::fmt;

#[derive(Clone, Copy)]
pub struct BenchCase {
    pub name: &'static str,
    pub source: &'static str,
}

impl fmt::Display for BenchCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name)
    }
}

pub const BENCH_CASES: &[BenchCase] = &[
    BenchCase {
        name: "bootstrap",
        source: include_str!("../files/bootstrap.css"),
    },
    BenchCase {
        name: "bootstrap.min",
        source: include_str!("../files/bootstrap.min.css"),
    },
];
