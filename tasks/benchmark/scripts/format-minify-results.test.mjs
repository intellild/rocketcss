import assert from "node:assert/strict";
import test from "node:test";

import { formatResults } from "./format-minify-results.mjs";

test("formats stage results as one table per input with a total column", () => {
  const output = `Timer precision: 41 ns
minify              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ codegen                        │               │               │               │         │
│  ├─ lightningcss                │               │               │               │         │
│  │  ├─ bootstrap  526.8 µs      │ 676.5 µs      │ 601.6 µs      │ 601.6 µs      │ 2       │ 2
│  │  │             533.4 MB/s    │ 415.4 MB/s    │ 467 MB/s      │ 467 MB/s      │         │
│  │  ╰─ tailwind   9.483 ms      │ 9.843 ms      │ 9.663 ms      │ 9.663 ms      │ 2       │ 2
│  │                604.3 MB/s    │ 582.2 MB/s    │ 593.1 MB/s    │ 593.1 MB/s    │         │
│  ╰─ rocketcss                   │               │               │               │         │
│     ├─ bootstrap  349.5 µs      │ 458 µs        │ 403.8 µs      │ 403.8 µs      │ 2       │ 2
│     │             804 MB/s      │ 613.5 MB/s    │ 695.9 MB/s    │ 695.9 MB/s    │         │
│     ╰─ tailwind   5.148 ms      │ 5.174 ms      │ 5.161 ms      │ 5.161 ms      │ 2       │ 2
│                   1.113 GB/s    │ 1.107 GB/s    │ 1.11 GB/s     │ 1.11 GB/s     │         │
├─ minify                         │               │               │               │         │
│  ├─ lightningcss                │               │               │               │         │
│  │  ├─ bootstrap  935.3 µs      │ 1.043 ms      │ 989.3 µs      │ 989.3 µs      │ 2       │ 2
│  │  │             300.4 MB/s    │ 269.3 MB/s    │ 284 MB/s      │ 284 MB/s      │         │
│  │  ╰─ tailwind   16.03 ms      │ 16.32 ms      │ 16.18 ms      │ 16.18 ms      │ 2       │ 2
│  │                357.3 MB/s    │ 351.1 MB/s    │ 354.2 MB/s    │ 354.2 MB/s    │         │
│  ╰─ rocketcss                   │               │               │               │         │
│     ├─ bootstrap  319 µs        │ 384.7 µs      │ 351.8 µs      │ 351.8 µs      │ 2       │ 2
│     │             880.7 MB/s    │ 730.5 MB/s    │ 798.6 MB/s    │ 798.6 MB/s    │         │
│     ╰─ tailwind   5.345 ms      │ 5.384 ms      │ 5.365 ms      │ 5.365 ms      │ 2       │ 2
│                   1.072 GB/s    │ 1.064 GB/s    │ 1.068 GB/s    │ 1.068 GB/s    │         │
├─ parse                          │               │               │               │         │
│  ├─ lightningcss                │               │               │               │         │
│  │  ├─ bootstrap  1.976 ms      │ 2.083 ms      │ 2.029 ms      │ 2.029 ms      │ 2       │ 2
│  │  │             142.2 MB/s    │ 134.9 MB/s    │ 138.4 MB/s    │ 138.4 MB/s    │         │
│  │  ╰─ tailwind   34.78 ms      │ 34.81 ms      │ 34.79 ms      │ 34.79 ms      │ 2       │ 2
│  │                164.7 MB/s    │ 164.6 MB/s    │ 164.7 MB/s    │ 164.7 MB/s    │         │
│  ╰─ rocketcss                   │               │               │               │         │
│     ├─ bootstrap  1.918 ms      │ 1.954 ms      │ 1.936 ms      │ 1.936 ms      │ 2       │ 2
│     │             146.4 MB/s    │ 143.8 MB/s    │ 145.1 MB/s    │ 145.1 MB/s    │         │
│     ╰─ tailwind   29.74 ms      │ 29.75 ms      │ 29.74 ms      │ 29.74 ms      │ 2       │ 2
│                   192.6 MB/s    │ 192.6 MB/s    │ 192.6 MB/s    │ 192.6 MB/s    │         │
╰─ total                          │               │               │               │         │
   ╰─ cssnano                     │               │               │               │         │
      ├─ bootstrap  92.04 ms      │ 135.1 ms      │ 113.5 ms      │ 113.5 ms      │ 2       │ 2
      │             3.053 MB/s    │ 2.08 MB/s     │ 2.474 MB/s    │ 2.474 MB/s    │         │
      ╰─ tailwind   940.8 ms      │ 959.3 ms      │ 950.1 ms      │ 950.1 ms      │ 2       │ 2
                    6.091 MB/s    │ 5.974 MB/s    │ 6.032 MB/s    │ 6.032 MB/s    │         │
`;

  assert.equal(
    formatResults(output),
    `### \`bootstrap\`

| Minifier | Parse | Minify | Codegen | Total |
| --- | ---: | ---: | ---: | ---: |
| RocketCSS | 1.936 ms | 351.8 µs | 403.8 µs | 2.692 ms |
| Lightning CSS | 2.029 ms | 989.3 µs | 601.6 µs | 3.62 ms |
| cssnano | — | — | — | 113.5 ms |

### \`tailwind\`

| Minifier | Parse | Minify | Codegen | Total |
| --- | ---: | ---: | ---: | ---: |
| RocketCSS | 29.74 ms | 5.365 ms | 5.161 ms | 40.27 ms |
| Lightning CSS | 34.79 ms | 16.18 ms | 9.663 ms | 60.63 ms |
| cssnano | — | — | — | 950.1 ms |`,
  );
});

test("leaves the total blank when a stage is missing", () => {
  const output = `Timer precision: 41 ns
minify              fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ parse                          │               │               │               │         │
   ╰─ rocketcss                   │               │               │               │         │
      ╰─ bootstrap  1.918 ms      │ 1.954 ms      │ 1.936 ms      │ 1.936 ms      │ 2       │ 2
                    146.4 MB/s    │ 143.8 MB/s    │ 145.1 MB/s    │ 145.1 MB/s    │         │
`;

  assert.equal(
    formatResults(output),
    `### \`bootstrap\`

| Minifier | Parse | Minify | Codegen | Total |
| --- | ---: | ---: | ---: | ---: |
| RocketCSS | 1.936 ms | — | — | — |`,
  );
});

test("returns Markdown when no benchmark rows are present", () => {
  assert.equal(
    formatResults("benchmark failed"),
    "_No benchmark results could be parsed._",
  );
});
