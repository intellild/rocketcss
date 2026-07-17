#!/usr/bin/env zx
// Record runnable test cases from upstream CSSNano test files.
//
// CSSNano builds many cases dynamically: helpers such as testTimingFunction
// wrap a value in several declaration templates, and suites loop over keyword
// lists. Static source snapshots cannot capture those cases, so this script
// executes each upstream test file inside a node:vm context with `node:test`
// and `util/testHelpers.js` stubbed out, and records every
// processCSS/passthroughCSS call as a concrete (input, expected) pair.
//
// The recorded JSON specs land in tests/fixtures/minify-dynamic/cssnano and
// are expanded and executed by the Rust harness in tests/src/minify_dynamic.rs.
//
// By default the test files are read from the byte-for-byte snapshot in
// tests/upstream-sources/cssnano (maintained by `upstream-tests.mjs sync`), so
// recording is reproducible without a live upstream checkout. The snapshot
// contains only test sources, so any require that cannot be resolved — the
// plugin implementation, postcss, and other runtime dependencies — is
// replaced with an inert stub; the recorded cases never execute plugin code.

import { createRequire } from "node:module";
import { existsSync } from "node:fs";
import { mkdir, readdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import vm from "node:vm";

import { $, chalk } from "zx";

$.verbose = false;

const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));
const REPOSITORY_ROOT = path.resolve(SCRIPT_DIR, "../../../..");
const DEFAULT_CSSNANO = path.join(
  REPOSITORY_ROOT,
  "tests/upstream-sources/cssnano",
);
const DEFAULT_OUTPUT = path.join(
  REPOSITORY_ROOT,
  "tests/fixtures/minify-dynamic/cssnano",
);

const usage = `
Record runnable CSSNano test cases into dynamic fixture specs.

Usage:
  record-dynamic-cases.mjs [options]

Options:
  --cssnano <path>   CSSNano test sources (default: the
                     tests/upstream-sources/cssnano snapshot; pass a live
                     checkout to record unreleased upstream changes).
  --output <path>    Spec output directory
                     (default: tests/fixtures/minify-dynamic/cssnano).
  -h, --help         Show this help.
`;

function parseArguments(rawArguments) {
  const options = {
    cssnano: process.env.CSSNANO_DIR ?? DEFAULT_CSSNANO,
    output: DEFAULT_OUTPUT,
  };
  const argumentsToParse = [...rawArguments];
  while (argumentsToParse.length > 0) {
    const argument = argumentsToParse.shift();
    if (argument === "-h" || argument === "--help") {
      options.help = true;
    } else if (argument === "--cssnano" || argument === "--output") {
      const value = argumentsToParse.shift();
      if (!value) {
        throw new Error(`${argument} requires a value`);
      }
      options[argument.slice(2)] = value;
    } else {
      throw new Error(`unknown option: ${argument}`);
    }
  }
  options.cssnano = path.resolve(options.cssnano);
  options.output = path.resolve(options.output);
  return options;
}

async function walkTestFiles(root, current = root, files = []) {
  if (!existsSync(current)) {
    return files;
  }
  const entries = await readdir(current, { withFileTypes: true });
  entries.sort((left, right) => left.name.localeCompare(right.name));
  for (const entry of entries) {
    const absolutePath = path.join(current, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === "node_modules") {
        continue;
      }
      await walkTestFiles(root, absolutePath, files);
    } else if (entry.isFile() && /\.[cm]?js$/.test(entry.name)) {
      files.push(absolutePath);
    }
  }
  return files;
}

async function gitRevision(root) {
  const result = await $({
    cwd: root,
    quiet: true,
    nothrow: true,
  })`git rev-parse HEAD`;
  if (result.exitCode === 0) {
    return result.stdout.trim();
  }
  // The snapshot is not a git checkout; fall back to the revision recorded in
  // the upstream-sources manifest by `upstream-tests.mjs sync`.
  const manifestPath = path.join(root, "../manifest.json");
  if (existsSync(manifestPath)) {
    const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
    return manifest.projects?.cssnano?.revision ?? null;
  }
  return null;
}

async function main() {
  const rawArguments = process.argv.slice(2);
  if (
    rawArguments[0] &&
    path.resolve(rawArguments[0]) === fileURLToPath(import.meta.url)
  ) {
    rawArguments.shift();
  }
  const options = parseArguments(rawArguments);
  if (options.help) {
    console.log(usage.trim());
    return;
  }

  const packagesRoot = path.join(options.cssnano, "packages");
  if (!existsSync(packagesRoot)) {
    throw new Error(
      `cssnano packages directory does not exist: ${packagesRoot}`,
    );
  }
  const revision = await gitRevision(options.cssnano);

  const packages = (await readdir(packagesRoot, { withFileTypes: true }))
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort();

  await rm(options.output, { recursive: true, force: true });
  await mkdir(options.output, { recursive: true });

  let totalCases = 0;
  let totalFiles = 0;
  for (const packageName of packages) {
    for (const testDirName of ["test", "tests", "__tests__"]) {
      const testRoot = path.join(packagesRoot, packageName, testDirName);
      const testFiles = await walkTestFiles(testRoot);
      for (const testFile of testFiles) {
        const source = await readFile(testFile, "utf8");
        const relativeTestFile = path.relative(packagesRoot, testFile);
        const recorder = recordCases(source, testFile);
        if (recorder.cases.length === 0) {
          if (recorder.errors.length > 0) {
            console.log(
              chalk.yellow(
                `  no cases: ${relativeTestFile} (${recorder.errors[0]})`,
              ),
            );
          }
          continue;
        }

        totalFiles += 1;
        totalCases += recorder.cases.length;
        const spec = {
          upstream: `packages/${relativeTestFile.split(path.sep).join("/")}`,
          revision,
          cases: recorder.cases,
        };
        const outputName = relativeTestFile
          .split(path.sep)
          .join("__")
          .replace(/\.[cm]?js$/, ".json");
        await writeFile(
          path.join(options.output, outputName),
          `${JSON.stringify(spec, null, 2)}\n`,
        );
        console.log(
          `  ${outputName}: ${recorder.cases.length} cases` +
            (recorder.stubbedRequires
              ? chalk.yellow(` (${recorder.stubbedRequires} stubbed requires)`)
              : "") +
            (recorder.errors.length
              ? chalk.yellow(` (${recorder.errors.length} test body errors)`)
              : ""),
        );
      }
    }
  }

  console.log(
    chalk.green(
      `\nrecorded ${totalCases} cases from ${totalFiles} test files -> ${path.relative(REPOSITORY_ROOT, options.output)}`,
    ),
  );
}

// Stub `node:test` and `util/testHelpers.js`, execute the file in a vm
// context, and collect every processCSS/passthroughCSS call. Requires that
// cannot be resolved from the test sources (plugin implementations, runtime
// data files) fall back to an inert stub; `stubbedRequires` counts them so
// data-driven files whose cases silently shrink stay visible.
function recordCases(source, absolutePath) {
  const cases = [];
  const errors = [];
  let currentTestName = null;
  let currentRegistrationSkip = false;
  let stubbedRequires = 0;

  const record = (input, expected, passthrough) => {
    cases.push({
      test: currentTestName,
      input: String(input),
      expected: String(expected),
      ...(passthrough ? { passthrough: true } : {}),
      ...(currentRegistrationSkip ? { upstreamSkip: true } : {}),
    });
    const closure = () => {};
    closure.__recordedCaseIndex = cases.length - 1;
    return closure;
  };

  const testStub = (name, body, upstreamSkip = false) => {
    // Suites such as normalize-positions' `suite(...)` run during argument
    // evaluation — before `test(name, ...)` itself is invoked — so the cases
    // they generate are recorded without a name. Attribute every case
    // recorded since the previous registration to this test; this keeps each
    // recorded case linked 1:1 to the upstream test registration that
    // produced it.
    for (const recorded of cases) {
      if (recorded.test === null) {
        recorded.test = String(name);
        if (upstreamSkip) {
          recorded.upstreamSkip = true;
        }
      }
    }
    if (body && typeof body.__recordedCaseIndex === "number") {
      cases[body.__recordedCaseIndex].test = String(name);
      if (upstreamSkip) {
        cases[body.__recordedCaseIndex].upstreamSkip = true;
      }
      return;
    }
    if (typeof body !== "function") {
      return;
    }
    const previous = currentTestName;
    const previousSkip = currentRegistrationSkip;
    currentTestName = String(name);
    currentRegistrationSkip = upstreamSkip;
    try {
      const returned = body({});
      if (returned && typeof returned.catch === "function") {
        returned.catch(() => {});
      }
    } catch (error) {
      errors.push(`${name}: ${error.message}`);
    } finally {
      currentTestName = previous;
      currentRegistrationSkip = previousSkip;
    }
  };
  // node:test modifiers. `skip`/`todo` mark cases so the Rust harness can
  // skip them exactly as upstream does; `only` records like a normal test.
  testStub.skip = (name, body) => testStub(name, body, true);
  testStub.todo = (name, body) => testStub(name, body, true);
  testStub.only = (name, body) => testStub(name, body, false);

  const testHelpersStub = {
    usePostCSSPlugin: () => () => {},
    processCSSFactory: () => ({
      processor: () => () => Promise.resolve({ css: "" }),
      processCSS: (fixture, expected) => record(fixture, expected, false),
      passthroughCSS: (fixture) => record(fixture, fixture, true),
    }),
  };

  const realRequire = createRequire(absolutePath);
  const customRequire = (id) => {
    if (id === "node:test" || id === "test") {
      return { test: testStub, default: testStub };
    }
    if (/(?:^|\/)util\/testHelpers\.js$/.test(id)) {
      return testHelpersStub;
    }
    try {
      return realRequire(id);
    } catch {
      // The snapshot ships only test sources: the plugin implementation,
      // postcss, and other runtime dependencies are absent. Recorded cases
      // never execute plugin code, so an inert stand-in is enough.
      stubbedRequires += 1;
      return () => ({ postcssPlugin: "rocketcss-recorder-stub" });
    }
  };

  const module = { exports: {} };
  const context = vm.createContext({
    require: customRequire,
    module,
    exports: module.exports,
    __dirname: path.dirname(absolutePath),
    __filename: absolutePath,
    console,
    process,
    Buffer,
    URL,
    setTimeout,
    clearTimeout,
    setInterval,
    clearInterval,
    queueMicrotask,
  });

  try {
    vm.runInContext(source, context, { filename: absolutePath });
  } catch (error) {
    errors.push(`load: ${error.message}`);
  }
  return { cases, errors, stubbedRequires };
}

main().catch((error) => {
  console.error(chalk.red(error.stack ?? error.message ?? String(error)));
  process.exitCode = 2;
});
