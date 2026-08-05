Source: Lightning CSS commit `5929e7346abf2a1b6f4e09f74cd11b53528a1f9`,
`src/lib.rs`, `test_mask` multi-layer case at lines 27701–27704.

This case is retained as an explicit skip because RocketCSS currently applies
its own absolute-length normalization (`16px` becomes `1pc`) in this typed
path; the upstream expected output is intentionally not rewritten.
