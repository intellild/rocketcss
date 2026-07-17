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
const STAGES = ["parse", "minify", "codegen"];
const STAGE_DISPLAY_NAMES = new Map([
  ["parse", "Parse"],
  ["minify", "Minify"],
  ["codegen", "Codegen"],
]);
const TOTAL_STAGE = "total";
const TRACKED_STAGES = new Set([...STAGES, TOTAL_STAGE]);

const TIME_PATTERN = /^([\d.]+)\s*(ps|ns|µs|us|ms|s)$/;
const TIME_UNIT_NANOS = new Map([
  ["ps", 1e-3],
  ["ns", 1],
  ["µs", 1e3],
  ["us", 1e3],
  ["ms", 1e6],
  ["s", 1e9],
]);
const TIME_FORMAT_SCALES = [
  ["s", 1e9],
  ["ms", 1e6],
  ["µs", 1e3],
  ["ns", 1],
];

function parseTime(value) {
  const match = value.match(TIME_PATTERN);
  if (!match) {
    return null;
  }
  return Number(match[1]) * TIME_UNIT_NANOS.get(match[2]);
}

function formatTime(nanos) {
  const [unit, scale] =
    TIME_FORMAT_SCALES.find(([, unitScale]) => nanos >= unitScale) ??
    TIME_FORMAT_SCALES.at(-1);
  return `${Number((nanos / scale).toPrecision(4))} ${unit}`;
}

export function parseResults(output) {
  const lines = output.replace(ANSI_ESCAPE_PATTERN, "").split(/\r?\n/);
  const results = [];
  const groups = [];

  for (let index = 0; index < lines.length; index++) {
    const match = lines[index].match(/^([│ ]*)[├╰]─\s+(\S+)(?:\s+(.+))?$/);
    if (!match) {
      continue;
    }

    const [, treePrefix, label, timingColumns = ""] = match;
    const depth = Math.floor(treePrefix.length / 3);
    groups.length = depth;
    groups.push(label);

    const timings = timingColumns.split("│").map((value) => value.trim());
    if (timings.length < 6 || timings[0] === "") {
      continue;
    }

    // Stage benchmarks nest their rows as stage -> minifier -> input.
    const stage = depth >= 2 ? groups[0] : null;
    const name = depth === 0 ? label : groups[depth - 1];

    results.push({
      stage,
      name,
      caseName: depth === 0 ? null : label,
      median: timings[2],
    });
  }

  return results;
}

function escapeCell(value) {
  return String(value).replaceAll("|", "\\|");
}

function totalCell(stages) {
  const measured = stages.get(TOTAL_STAGE);
  if (measured) {
    return measured.median;
  }

  // rocketcss and Lightning CSS totals are the sums of their measured stages.
  const stageTimes = STAGES.map((stage) => {
    const result = stages.get(stage);
    return result ? parseTime(result.median) : null;
  });
  if (stageTimes.some((time) => time === null)) {
    return "—";
  }
  return formatTime(stageTimes.reduce((sum, time) => sum + time, 0));
}

function formatTable(byName) {
  const rows = [
    "| Minifier | Parse | Minify | Codegen | Total |",
    "| --- | ---: | ---: | ---: | ---: |",
  ];

  const names = [...byName.keys()].sort(
    (left, right) =>
      (DISPLAY_ORDER.get(left) ?? Number.MAX_SAFE_INTEGER) -
      (DISPLAY_ORDER.get(right) ?? Number.MAX_SAFE_INTEGER),
  );

  for (const name of names) {
    const stages = byName.get(name);
    const displayName = DISPLAY_NAMES.get(name) ?? name;
    const cells = STAGES.map((stage) => stages.get(stage)?.median ?? "—");
    cells.push(totalCell(stages));
    rows.push(
      `| ${escapeCell(displayName)} | ${cells.map(escapeCell).join(" | ")} |`,
    );
  }

  return rows.join("\n");
}

export function formatResults(output) {
  const results = parseResults(output).filter(
    (result) => result.stage !== null && TRACKED_STAGES.has(result.stage),
  );
  if (results.length === 0) {
    return "_No benchmark results could be parsed._";
  }

  const cases = new Map();
  for (const result of results) {
    const caseName = result.caseName ?? "other";
    const byName = cases.get(caseName) ?? new Map();
    const stages = byName.get(result.name) ?? new Map();
    stages.set(result.stage, result);
    byName.set(result.name, stages);
    cases.set(caseName, byName);
  }

  return [...cases]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(
      ([caseName, byName]) => `### \`${caseName}\`\n\n${formatTable(byName)}`,
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
