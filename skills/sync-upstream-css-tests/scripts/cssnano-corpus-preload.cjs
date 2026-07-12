"use strict";

const { appendFileSync } = require("node:fs");
const Module = require("node:module");
const path = require("node:path");

const output = process.env.CSSNANO_CORPUS_OUTPUT;
const root = process.env.CSSNANO_CORPUS_ROOT;
const originalLoad = Module._load;
const wrappedModules = new Map();
let ordinal = 0;

function caller() {
  const stack = new Error().stack?.split("\n").slice(2) ?? [];
  for (const frame of stack) {
    const match =
      frame.match(/\((.*?):(\d+):(\d+)\)$/) ??
      frame.match(/at (.*?):(\d+):(\d+)$/);
    if (!match) continue;
    const filename = match[1];
    if (
      filename.startsWith(root) &&
      filename.includes(`${path.sep}test${path.sep}`) &&
      !filename.endsWith(`${path.sep}testHelpers.js`) &&
      !filename.endsWith(`${path.sep}_processCss.js`) &&
      !filename.endsWith(`${path.sep}_processCSS.js`)
    ) {
      return {
        file: path.relative(root, filename).split(path.sep).join("/"),
        line: Number(match[2]),
      };
    }
  }
  return { file: "unknown", line: 0 };
}

function record(plugin, source, expected, options) {
  if (!output || typeof source !== "string" || typeof expected !== "string") {
    return;
  }
  // A few upstream passthrough helpers accidentally interpolate an omitted
  // second argument as the literal word `undefined`. The Node tests never
  // execute those returned assertions. They are not runtime behavior and
  // cannot provide a trustworthy expected result, so exclude them.
  if (expected.includes("undefined") && !source.includes("undefined")) {
    return;
  }
  const location = caller();
  const packageName = location.file.match(/^packages\/([^/]+)\/test\//)?.[1];
  const resolvedPlugin =
    packageName && ["anonymous", "pluginCreator"].includes(plugin)
      ? packageName
      : plugin;
  ordinal += 1;
  appendFileSync(
    output,
    `${JSON.stringify({
      name: `${location.file}:${location.line}/${ordinal}`,
      upstream_file: location.file,
      line: location.line,
      plugin: resolvedPlugin,
      source,
      expected,
      options: options ?? null,
    })}\n`,
  );
}

function pluginName(plugin) {
  if (Array.isArray(plugin)) {
    return plugin.map(pluginName).join(",");
  }
  return plugin?.postcssPlugin ?? plugin?.name ?? "anonymous";
}

function wrapTestHelpers(exports) {
  if (exports.__rocketcssCorpusWrapped) return exports;
  const originalFactory = exports.processCSSFactory;
  const wrapped = { ...exports };
  wrapped.processCSSFactory = (plugin) => {
    const helpers = originalFactory(plugin);
    const name = pluginName(plugin);
    return {
      ...helpers,
      processCSS(source, expected, options) {
        record(name, source, expected, options);
        return helpers.processCSS(source, expected, options);
      },
      passthroughCSS(source, options) {
        record(name, source, source, options);
        return helpers.passthroughCSS(source, options);
      },
    };
  };
  Object.defineProperty(wrapped, "__rocketcssCorpusWrapped", { value: true });
  return wrapped;
}

function wrapCssnanoProcess(exports) {
  if (exports.__rocketcssCorpusWrapped) return exports;
  function wrapped(source, expected, options) {
    record("cssnano", source, expected, options);
    return exports(source, expected, options);
  }
  wrapped.passthrough = (source, options) => {
    record("cssnano", source, source, options);
    return exports.passthrough(source, options);
  };
  Object.defineProperty(wrapped, "__rocketcssCorpusWrapped", { value: true });
  return wrapped;
}

Module._load = function load(request, parent, isMain) {
  const resolved = Module._resolveFilename(request, parent, isMain);
  if (wrappedModules.has(resolved)) {
    return wrappedModules.get(resolved);
  }
  const exports = originalLoad.call(this, request, parent, isMain);
  let wrapped = exports;
  if (resolved === path.join(root, "util/testHelpers.js")) {
    wrapped = wrapTestHelpers(exports);
  } else if (
    resolved === path.join(root, "packages/cssnano/test/_processCss.js")
  ) {
    wrapped = wrapCssnanoProcess(exports);
  }
  if (wrapped !== exports) {
    wrappedModules.set(resolved, wrapped);
  }
  return wrapped;
};
