# Parser corpus generator

`generate.py` records parser inputs from the exact dependency versions used by
the benchmark:

- `lightningcss 1.0.0-alpha.71`
- `stylo 0.19.0` / `selectors 0.39.0`

Lightning CSS builds many inputs dynamically. To avoid losing those cases, the
generator requires a log from an instrumented run of its 117 library tests.
The instrumentation wraps each `StyleSheet::parse` call without changing its
result, and writes one line in this form:

```text
RS_CSS_CORPUS|<source-line>|<ok>|<error-recovery>|<css-modules>|<flags>|<debug-source>|RS_CSS_CORPUS_END
```

The wrapper is `#[track_caller]`; the three parser-error/recovery helper
functions must also be `#[track_caller]` so the recorded line remains the
original test call site. Run the tests with `--test-threads=1 --nocapture` and
capture combined stdout/stderr.

Then regenerate the checked-in corpus with:

```sh
python3 tasks/parser-test-corpus/generate.py \
  --lightningcss /path/to/lightningcss-1.0.0-alpha.71/src/lib.rs \
  --lightningcss-log /path/to/lightningcss-test.log \
  --stylo-selectors /path/to/selectors-0.39.0/parser.rs \
  --output crates/parser/tests/upstream/corpus.json
```

The generator also reads Lightning CSS's custom-parser and serde integration
tests directly. Parser-error cases are counted for the audit but omitted from
the output case arrays because the suite's boundary requires an AST result.
