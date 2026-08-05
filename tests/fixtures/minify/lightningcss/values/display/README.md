Source: Lightning CSS commit `5929e7346abf2a1b6f4e09f74cd11b53528a1f9`,
`src/lib.rs`, `test_display` cases at lines 15615–15643.

The invalid `table-cell flow` case remains an explicit skip because RocketCSS
preserves the unparsed value rather than applying Lightning's repair pass.
