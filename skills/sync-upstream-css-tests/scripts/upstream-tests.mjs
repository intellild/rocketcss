#!/usr/bin/env zx

import { createHash } from "node:crypto";
import { existsSync } from "node:fs";
import {
  copyFile,
  cp,
  mkdir,
  mkdtemp,
  readFile,
  readdir,
  rename,
  rm,
  stat,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

import { $, chalk } from "zx";

$.verbose = false;

const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));
const REPOSITORY_ROOT = path.resolve(SCRIPT_DIR, "../../..");
const DEFAULT_SNAPSHOT_ROOT = path.join(
  REPOSITORY_ROOT,
  "tests/upstream-sources",
);
const PROJECTS = ["lightningcss", "cssnano"];
const EXCLUDED_DIRECTORIES = new Set([
  ".git",
  ".idea",
  "coverage",
  "node_modules",
  "target",
]);

const usage = `
Compare RocketCSS tests with local Lightning CSS and CSSNano checkouts.

Usage:
  pnpm upstream-tests [status|diff|sync|check] [options]

Commands:
  status  Show local fixture counts and upstream snapshot drift (default).
  diff    Print a git-style diff between the snapshot and live upstream tests.
  sync    Replace the snapshot with exact copies of the live upstream tests.
  check   Exit with status 1 when the committed snapshot differs from upstream.

Options:
  --project <all|lightningcss|cssnano>  Limit the operation (default: all).
  --lightningcss <path>                 Lightning CSS checkout.
  --cssnano <path>                      CSSNano checkout.
  --snapshot <path>                     Snapshot directory.
  --stat                                With diff, print only diffstat.
  --name-only                           With diff, print only changed paths.
  -h, --help                            Show this help.

Defaults can also be overridden with LIGHTNINGCSS_DIR and CSSNANO_DIR.
The snapshot is a source audit corpus. Copying a source into it does not mean
that the case has been ported to a runnable RocketCSS fixture.
`;

function parseArguments(rawArguments) {
  const options = {
    command: "status",
    project: "all",
    lightningcss:
      process.env.LIGHTNINGCSS_DIR ??
      path.resolve(REPOSITORY_ROOT, "../lightningcss"),
    cssnano:
      process.env.CSSNANO_DIR ?? path.resolve(REPOSITORY_ROOT, "../cssnano"),
    snapshot: DEFAULT_SNAPSHOT_ROOT,
    stat: false,
    nameOnly: false,
  };
  const argumentsToParse = [...rawArguments];

  if (argumentsToParse[0] && !argumentsToParse[0].startsWith("-")) {
    options.command = argumentsToParse.shift();
  }

  while (argumentsToParse.length > 0) {
    const argument = argumentsToParse.shift();
    if (argument === "-h" || argument === "--help") {
      options.help = true;
    } else if (argument === "--stat") {
      options.stat = true;
    } else if (argument === "--name-only") {
      options.nameOnly = true;
    } else if (
      argument === "--project" ||
      argument === "--lightningcss" ||
      argument === "--cssnano" ||
      argument === "--snapshot"
    ) {
      const value = argumentsToParse.shift();
      if (!value) {
        throw new Error(`${argument} requires a value`);
      }
      options[argument.slice(2)] = value;
    } else {
      throw new Error(`unknown option: ${argument}`);
    }
  }

  if (!["status", "diff", "sync", "check"].includes(options.command)) {
    throw new Error(`unknown command: ${options.command}`);
  }
  if (!["all", ...PROJECTS].includes(options.project)) {
    throw new Error(`unknown project: ${options.project}`);
  }
  if (options.stat && options.nameOnly) {
    throw new Error("--stat and --name-only cannot be used together");
  }

  options.lightningcss = path.resolve(options.lightningcss);
  options.cssnano = path.resolve(options.cssnano);
  options.snapshot = path.resolve(options.snapshot);
  return options;
}

async function walkFiles(root, current = root, files = []) {
  if (!existsSync(current)) {
    return files;
  }

  const entries = await readdir(current, { withFileTypes: true });
  entries.sort((left, right) => left.name.localeCompare(right.name));
  for (const entry of entries) {
    if (entry.isDirectory() && EXCLUDED_DIRECTORIES.has(entry.name)) {
      continue;
    }
    const absolutePath = path.join(current, entry.name);
    if (entry.isDirectory()) {
      await walkFiles(root, absolutePath, files);
    } else if (entry.isFile()) {
      files.push({
        absolutePath,
        relativePath: path
          .relative(root, absolutePath)
          .split(path.sep)
          .join("/"),
      });
    }
  }
  return files;
}

function isTestDirectory(relativePath) {
  return relativePath
    .split("/")
    .some(
      (part) => part === "test" || part === "tests" || part === "__tests__",
    );
}

async function selectUpstreamFiles(project, root) {
  const candidates = await walkFiles(root);
  const selected = [];

  for (const candidate of candidates) {
    const { absolutePath, relativePath } = candidate;
    if (isTestDirectory(relativePath)) {
      selected.push(candidate);
      continue;
    }

    if (project === "cssnano") {
      if (
        /(?:^|\/)util\/testHelpers\.js$/.test(relativePath) ||
        /(?:^|\/)[^/]+\.(?:test|spec)\.[cm]?[jt]sx?$/.test(relativePath)
      ) {
        selected.push(candidate);
      }
      continue;
    }

    if (relativePath.endsWith(".rs")) {
      const source = await readFile(absolutePath, "utf8");
      if (
        /#\s*\[\s*test\s*\]/.test(source) ||
        /#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]/.test(source)
      ) {
        selected.push(candidate);
      }
    }
  }

  return selected.sort((left, right) =>
    left.relativePath.localeCompare(right.relativePath),
  );
}

function countStaticTestDeclarations(project, relativePath, source) {
  if (!/\.(?:[cm]?[jt]sx?|rs)$/.test(relativePath)) {
    return 0;
  }
  if (project === "lightningcss") {
    return source.match(/#\s*\[\s*test\s*\]/g)?.length ?? 0;
  }

  // This is intentionally labelled "static": CSSNano helpers and loops can
  // turn one call site into many runtime cases.
  return (
    source.match(/\b(?:it|test)(?:\.(?:only|skip|todo))?\s*\(/g)?.length ?? 0
  );
}

async function describeFile(project, file) {
  const contents = await readFile(file.absolutePath);
  const source = contents.toString("utf8");
  return {
    path: file.relativePath,
    sha256: createHash("sha256").update(contents).digest("hex"),
    bytes: contents.byteLength,
    static_test_declarations: countStaticTestDeclarations(
      project,
      file.relativePath,
      source,
    ),
  };
}

async function gitRevision(root) {
  const result = await $({
    cwd: root,
    quiet: true,
    nothrow: true,
  })`git rev-parse HEAD`;
  return result.exitCode === 0 ? result.stdout.trim() : null;
}

async function inventoryProject(project, root) {
  if (!existsSync(root) || !(await stat(root)).isDirectory()) {
    throw new Error(`${project} checkout does not exist: ${root}`);
  }
  const selected = await selectUpstreamFiles(project, root);
  const files = [];
  for (const file of selected) {
    files.push(await describeFile(project, file));
  }
  return {
    project,
    root,
    revision: await gitRevision(root),
    files,
  };
}

async function inventorySnapshot(project, snapshotRoot) {
  const root = path.join(snapshotRoot, project);
  const files = await walkFiles(root);
  const descriptions = [];
  for (const file of files) {
    descriptions.push(await describeFile(project, file));
  }
  return { project, root, files: descriptions };
}

function compareInventories(snapshot, upstream) {
  const oldFiles = new Map(snapshot.files.map((file) => [file.path, file]));
  const newFiles = new Map(upstream.files.map((file) => [file.path, file]));
  const added = [];
  const removed = [];
  const changed = [];

  for (const [filePath, file] of newFiles) {
    const oldFile = oldFiles.get(filePath);
    if (!oldFile) {
      added.push(filePath);
    } else if (oldFile.sha256 !== file.sha256) {
      changed.push(filePath);
    }
  }
  for (const filePath of oldFiles.keys()) {
    if (!newFiles.has(filePath)) {
      removed.push(filePath);
    }
  }
  return { added, removed, changed };
}

async function localFixtureSummary() {
  const fixtureRoot = path.join(REPOSITORY_ROOT, "tests/fixtures/minify");
  const harnessSource = await readFile(
    path.join(REPOSITORY_ROOT, "tests/src/minify.rs"),
    "utf8",
  );
  const skipFunction = harnessSource.slice(
    harnessSource.indexOf("fn requires_nonlocal_or_rebuilding_transform"),
  );
  const skippedPatterns = [
    ...skipFunction.matchAll(/"(\/(?:cssnano|lightningcss)\/[^"\n]+\/)"/g),
  ].map((match) => match[1]);
  const summary = {};
  for (const project of PROJECTS) {
    const projectRoot = path.join(fixtureRoot, project);
    const files = await walkFiles(projectRoot);
    const inputs = files.filter(
      (file) => path.basename(file.relativePath) === "input.css",
    );
    let completePairs = 0;
    for (const input of inputs) {
      if (
        existsSync(path.join(path.dirname(input.absolutePath), "output.css"))
      ) {
        completePairs += 1;
      }
    }
    const skipped = inputs.filter((input) => {
      const fixturePath = `/${project}/${path.dirname(input.relativePath)}/`;
      return skippedPatterns.some((pattern) => fixturePath.includes(pattern));
    }).length;
    summary[project] = {
      inputs: inputs.length,
      completePairs,
      skipped,
      executed: inputs.length - skipped,
    };
  }
  return summary;
}

function selectedProjects(options) {
  return options.project === "all" ? PROJECTS : [options.project];
}

function projectRoot(options, project) {
  return options[project];
}

function totalStaticTests(inventory) {
  return inventory.files.reduce(
    (total, file) => total + file.static_test_declarations,
    0,
  );
}

function printDrift(project, upstream, snapshot, drift) {
  const revision = upstream.revision
    ? upstream.revision.slice(0, 12)
    : "unknown";
  console.log(chalk.bold(`\n${project}`));
  console.log(`  upstream: ${upstream.files.length} files @ ${revision}`);
  console.log(
    `  static test declarations: ${totalStaticTests(upstream)} (generated cases are not included)`,
  );
  console.log(`  snapshot: ${snapshot.files.length} files`);
  console.log(
    `  drift: +${drift.added.length} -${drift.removed.length} ~${drift.changed.length}`,
  );
}

async function loadInventories(options) {
  const results = [];
  for (const project of selectedProjects(options)) {
    const upstream = await inventoryProject(
      project,
      projectRoot(options, project),
    );
    const snapshot = await inventorySnapshot(project, options.snapshot);
    results.push({
      project,
      upstream,
      snapshot,
      drift: compareInventories(snapshot, upstream),
    });
  }
  return results;
}

function hasDrift(result) {
  return (
    result.drift.added.length > 0 ||
    result.drift.removed.length > 0 ||
    result.drift.changed.length > 0
  );
}

async function statusCommand(options, checkOnly = false) {
  const fixtureSummary = await localFixtureSummary();
  const inventories = await loadInventories(options);

  if (!checkOnly) {
    console.log(chalk.bold("Local runnable minify fixtures"));
    for (const project of selectedProjects(options)) {
      const summary = fixtureSummary[project];
      console.log(
        `  ${project}: ${summary.completePairs} pairs, ${summary.executed} executed, ${summary.skipped} skipped`,
      );
    }
    console.log(
      chalk.yellow(
        "  These counts include skipped fixtures and do not imply full upstream coverage.",
      ),
    );
  }

  for (const result of inventories) {
    if (!checkOnly) {
      printDrift(
        result.project,
        result.upstream,
        result.snapshot,
        result.drift,
      );
    }
  }

  const drifted = inventories.filter(hasDrift);
  if (checkOnly && drifted.length > 0) {
    for (const result of drifted) {
      printDrift(
        result.project,
        result.upstream,
        result.snapshot,
        result.drift,
      );
    }
    console.error(chalk.red("\nUpstream test snapshots are out of date."));
    process.exitCode = 1;
  } else if (!checkOnly) {
    console.log(
      drifted.length === 0
        ? chalk.green("\nSnapshots match the selected upstream test sources.")
        : chalk.yellow(
            `\n${drifted.length} snapshot(s) differ; run "pnpm upstream-tests sync" to update them.`,
          ),
    );
  }
}

async function materializeInventory(inventory, destination) {
  await mkdir(destination, { recursive: true });
  for (const file of inventory.files) {
    const destinationPath = path.join(destination, file.path);
    await mkdir(path.dirname(destinationPath), { recursive: true });
    await copyFile(path.join(inventory.root, file.path), destinationPath);
  }
}

async function diffCommand(options) {
  const inventories = await loadInventories(options);
  let differs = false;

  for (const result of inventories) {
    const temporaryRoot = await mkdtemp(
      path.join(tmpdir(), `rocketcss-${result.project}-tests-`),
    );
    const oldRoot = path.join(temporaryRoot, "snapshot");
    const newRoot = path.join(temporaryRoot, "upstream");
    await mkdir(oldRoot, { recursive: true });
    if (existsSync(result.snapshot.root)) {
      await cp(result.snapshot.root, oldRoot, { recursive: true });
    }
    await materializeInventory(result.upstream, newRoot);

    console.log(chalk.bold(`\n${result.project}`));
    const diffArguments = ["diff", "--no-index", "--no-ext-diff"];
    if (options.stat) {
      diffArguments.push("--stat");
    } else if (options.nameOnly) {
      diffArguments.push("--name-only");
    }
    diffArguments.push(
      `--src-prefix=a/${result.project}/`,
      `--dst-prefix=b/${result.project}/`,
      "--",
      "snapshot",
      "upstream",
    );
    const diff = await $({
      cwd: temporaryRoot,
      nothrow: true,
      stdio: "inherit",
    })`git ${diffArguments}`;
    await rm(temporaryRoot, { recursive: true, force: true });

    if (diff.exitCode === 1) {
      differs = true;
    } else if (diff.exitCode !== 0) {
      throw new Error(`git diff failed for ${result.project}`);
    }
  }

  if (differs) {
    console.log(
      chalk.yellow(
        "\nDifferences found; use the check command for a failing exit code.",
      ),
    );
  }
}

async function syncCommand(options) {
  const inventories = [];
  for (const project of selectedProjects(options)) {
    inventories.push(
      await inventoryProject(project, projectRoot(options, project)),
    );
  }

  await mkdir(options.snapshot, { recursive: true });
  for (const inventory of inventories) {
    const destination = path.join(options.snapshot, inventory.project);
    await rm(destination, { recursive: true, force: true });
    await materializeInventory(inventory, destination);
    console.log(
      chalk.green(
        `synced ${inventory.project}: ${inventory.files.length} files (${totalStaticTests(inventory)} static test declarations)`,
      ),
    );
  }

  const manifestPath = path.join(options.snapshot, "manifest.json");
  let manifest = { format: 1, projects: {} };
  if (existsSync(manifestPath)) {
    manifest = JSON.parse(await readFile(manifestPath, "utf8"));
  }
  manifest.format = 1;
  manifest.description =
    "Exact upstream test-source snapshot; this is not a RocketCSS coverage claim.";
  manifest.projects ??= {};
  for (const inventory of inventories) {
    manifest.projects[inventory.project] = {
      revision: inventory.revision,
      files: inventory.files,
    };
  }
  const temporaryManifest = `${manifestPath}.tmp`;
  await writeFile(temporaryManifest, `${JSON.stringify(manifest, null, 2)}\n`);
  await rename(temporaryManifest, manifestPath);
  console.log(`manifest: ${path.relative(REPOSITORY_ROOT, manifestPath)}`);
}

async function main() {
  const rawArguments = process.argv.slice(2);
  // zx keeps the script path in argv when invoked as `zx script.mjs` while a
  // direct shebang invocation does not.
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

  if (options.command === "status") {
    await statusCommand(options);
  } else if (options.command === "check") {
    await statusCommand(options, true);
  } else if (options.command === "diff") {
    await diffCommand(options);
  } else {
    await syncCommand(options);
  }
}

main().catch((error) => {
  console.error(chalk.red(error.stack ?? error.message ?? String(error)));
  process.exitCode = 2;
});
