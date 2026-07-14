import assert from "node:assert/strict";
import test from "node:test";

import { formatResults } from "./format-minify-results.mjs";

test("formats Divan minifier results as a Markdown table", () => {
  const output = `Timer precision: 41 ns
minify           fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ cssnano       89.67 ms      │ 137 ms        │ 97.94 ms      │ 102.8 ms      │ 5       │ 5
│                3.134 MB/s    │ 2.05 MB/s     │ 2.869 MB/s    │ 2.732 MB/s    │         │
├─ lightningcss  3.565 ms      │ 5.464 ms      │ 3.687 ms      │ 4.167 ms      │ 5       │ 5
│                78.82 MB/s    │ 51.43 MB/s    │ 76.21 MB/s    │ 67.43 MB/s    │         │
╰─ rocketcss     2.237 ms      │ 2.734 ms      │ 2.247 ms      │ 2.358 ms      │ 5       │ 5
                 125.6 MB/s    │ 102.7 MB/s    │ 125 MB/s      │ 119.1 MB/s    │         │
`;

  assert.equal(
    formatResults(output),
    `| Minifier | Fastest | Median | Mean | Slowest | Mean throughput | Samples | Iterations |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| RocketCSS | 2.237 ms | 2.247 ms | 2.358 ms | 2.734 ms | 119.1 MB/s | 5 | 5 |
| Lightning CSS | 3.565 ms | 3.687 ms | 4.167 ms | 5.464 ms | 67.43 MB/s | 5 | 5 |
| cssnano | 89.67 ms | 97.94 ms | 102.8 ms | 137 ms | 2.732 MB/s | 5 | 5 |`,
  );
});

test("returns Markdown when no benchmark rows are present", () => {
  assert.equal(
    formatResults("benchmark failed"),
    "_No benchmark results could be parsed._",
  );
});
