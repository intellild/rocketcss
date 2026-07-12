# rocketcss

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://app.codspeed.io/intellild/rocketcss?utm_source=badge)

A fast, arena-backed CSS toolchain written in Rust. The workspace is split into
focused crates for allocation, the AST, parsing, visiting, code generation, and
minification.

## Workspace crates

- `rocketcss_allocator` — arena allocator backing the AST
- `rocketcss_ast` — CSS syntax tree definitions
- `rocketcss_parser` — CSS tokenizer and parser
- `rocketcss_visitor` — AST visitor and plugin pipeline
- `rocketcss_codegen` — CSS serialization
- `rocketcss_minify` — node-local CSS normalization

## Benchmarks

Performance is tracked continuously with [CodSpeed](https://codspeed.io). The
benchmark targets live in `tasks/benchmark` and use the Divan compatibility
layer. See [`tasks/benchmark/README.md`](tasks/benchmark/README.md) for details
on running them locally.

## License

Licensed under the Apache License, Version 2.0.
