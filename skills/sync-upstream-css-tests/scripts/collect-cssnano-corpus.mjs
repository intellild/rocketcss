#!/usr/bin/env zx

import { readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

import { $, chalk } from "zx";

$.verbose = false;

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(scriptDirectory, "../../..");

function parseArguments(rawArguments) {
  const argumentsToParse = [...rawArguments];
  if (
    argumentsToParse[0] &&
    path.resolve(argumentsToParse[0]) === fileURLToPath(import.meta.url)
  ) {
    argumentsToParse.shift();
  }
  const options = {
    cssnano: path.resolve(repositoryRoot, "../cssnano"),
    output: path.resolve(
      repositoryRoot,
      "tests/fixtures/minify/cssnano/corpus.json",
    ),
  };
  while (argumentsToParse.length > 0) {
    const argument = argumentsToParse.shift();
    if (argument === "-h" || argument === "--help") {
      options.help = true;
    } else if (argument === "--cssnano" || argument === "--output") {
      const value = argumentsToParse.shift();
      if (!value) throw new Error(`${argument} requires a value`);
      options[argument.slice(2)] = path.resolve(value);
    } else {
      throw new Error(`unknown option: ${argument}`);
    }
  }
  return options;
}

const usage = `
Collect CSS input/output pairs by running the complete local CSSNano test suite.

Usage:
  pnpm cssnano-test-corpus [--cssnano /path/to/cssnano] [--output path]
`;

async function gitRevision(root) {
  const result = await $({ cwd: root, quiet: true })`git rev-parse HEAD`;
  return result.stdout.trim();
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  if (options.help) {
    console.log(usage.trim());
    return;
  }

  const rawOutput = `${options.output}.ndjson`;
  await writeFile(rawOutput, "");
  const preload = path.join(scriptDirectory, "cssnano-corpus-preload.cjs");
  await $({
    cwd: options.cssnano,
    env: {
      ...process.env,
      CSSNANO_CORPUS_OUTPUT: rawOutput,
      CSSNANO_CORPUS_ROOT: options.cssnano,
      NODE_OPTIONS: [process.env.NODE_OPTIONS, `--require=${preload}`]
        .filter(Boolean)
        .join(" "),
    },
    stdio: "inherit",
  })`pnpm exec node --test --test-concurrency=1`;

  const cases = (await readFile(rawOutput, "utf8"))
    .split("\n")
    .filter(Boolean)
    .map((line) => JSON.parse(line));
  const seen = new Set();
  const uniqueCases = cases.filter((item) => {
    const key = JSON.stringify([
      item.plugin,
      item.source,
      item.expected,
      item.options,
    ]);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
  uniqueCases.forEach((item, index) => {
    item.name = `${item.upstream_file}:${item.line}/${index + 1}`;
  });
  const corpus = {
    format: 1,
    upstream: "cssnano",
    revision: await gitRevision(options.cssnano),
    extraction:
      "runtime calls to CSSNano processCSSFactory and cssnano integration helpers",
    cases: uniqueCases,
  };
  await writeFile(options.output, `${JSON.stringify(corpus, null, 2)}\n`);
  await rm(rawOutput);
  console.log(
    chalk.green(
      `collected ${uniqueCases.length} unique CSS transform cases (${cases.length} runtime calls)`,
    ),
  );
}

main().catch((error) => {
  console.error(chalk.red(error.stack ?? error.message ?? String(error)));
  process.exitCode = 2;
});
