import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const ANSI_ESCAPE_PATTERN = /\u001b\[[0-?]*[ -/]*[@-~]/g;
const DISPLAY_NAMES = new Map([
  ["rocketcss", "RocketCSS"],
  ["lightningcss", "Lightning CSS"],
  ["cssnano", "cssnano"],
]);
const DISPLAY_ORDER = new Map(
  [...DISPLAY_NAMES.keys()].map((name, index) => [name, index]),
);

export function parseResults(output) {
  const lines = output.replace(ANSI_ESCAPE_PATTERN, "").split(/\r?\n/);
  const results = [];
  let currentMinifier = null;

  for (let index = 0; index < lines.length; index++) {
    const match = lines[index].match(/^([│ ]*)[├╰]─\s+(\S+)(?:\s+(.+))?$/);
    if (!match) {
      continue;
    }

    const [, treePrefix, label, timingColumns = ""] = match;
    if (treePrefix.length === 0 && DISPLAY_NAMES.has(label)) {
      currentMinifier = label;
    }

    const timings = timingColumns.split("│").map((value) => value.trim());
    if (timings.length < 6 || timings[0] === "") {
      continue;
    }

    const name = treePrefix.length === 0 ? label : currentMinifier;
    if (name === null) {
      continue;
    }

    const throughputLine = lines[index + 1] ?? "";
    const throughputs = throughputLine
      .replace(/^[│ ]+/, "")
      .split("│")
      .map((value) => value.trim());

    results.push({
      name,
      caseName: treePrefix.length === 0 ? null : label,
      fastest: timings[0],
      slowest: timings[1],
      median: timings[2],
      mean: timings[3],
      samples: timings[4],
      iterations: timings[5],
      meanThroughput: throughputs[3] || "—",
    });
  }

  return results.sort((left, right) => {
    const caseOrder = (left.caseName ?? "").localeCompare(right.caseName ?? "");
    return caseOrder !== 0
      ? caseOrder
      : (DISPLAY_ORDER.get(left.name) ?? Number.MAX_SAFE_INTEGER) -
          (DISPLAY_ORDER.get(right.name) ?? Number.MAX_SAFE_INTEGER);
  });
}

function escapeCell(value) {
  return String(value).replaceAll("|", "\\|");
}

function formatTable(results) {
  const rows = [
    "| Minifier | Fastest | Median | Mean | Slowest | Mean throughput | Samples | Iterations |",
    "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
  ];

  for (const result of results) {
    const name = DISPLAY_NAMES.get(result.name) ?? result.name;
    rows.push(
      `| ${escapeCell(name)} | ${escapeCell(result.fastest)} | ${escapeCell(result.median)} | ${escapeCell(result.mean)} | ${escapeCell(result.slowest)} | ${escapeCell(result.meanThroughput)} | ${escapeCell(result.samples)} | ${escapeCell(result.iterations)} |`,
    );
  }

  return rows.join("\n");
}

export function formatResults(output) {
  const results = parseResults(output);
  if (results.length === 0) {
    return "_No benchmark results could be parsed._";
  }

  if (results.every((result) => result.caseName === null)) {
    return formatTable(results);
  }

  const cases = new Map();
  for (const result of results) {
    const caseName = result.caseName ?? "other";
    const caseResults = cases.get(caseName) ?? [];
    caseResults.push(result);
    cases.set(caseName, caseResults);
  }

  return [...cases]
    .map(
      ([caseName, caseResults]) =>
        `### \`${caseName}\`\n\n${formatTable(caseResults)}`,
    )
    .join("\n\n");
}

const invokedPath = process.argv[1];
const isMain =
  invokedPath &&
  import.meta.url === pathToFileURL(path.resolve(invokedPath)).href;

if (isMain) {
  const resultPath = process.argv[2];
  if (!resultPath) {
    throw new Error("usage: node format-minify-results.mjs <result-file>");
  }

  process.stdout.write(
    `${formatResults(fs.readFileSync(resultPath, "utf8"))}\n`,
  );
}
