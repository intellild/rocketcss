#!/usr/bin/env zx

import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

import { chalk } from "zx";

function upperBound(values, target) {
  let low = 0;
  let high = values.length;
  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    if (values[middle] <= target) {
      low = middle + 1;
    } else {
      high = middle;
    }
  }
  return low;
}

function decodeString(body) {
  const result = [];
  let index = 0;
  while (index < body.length) {
    const character = body[index];
    if (character !== "\\") {
      result.push(character);
      index += 1;
      continue;
    }

    index += 1;
    if (index >= body.length) {
      throw new Error("unterminated escape");
    }
    const escaped = body[index];
    index += 1;
    const simple = {
      n: "\n",
      r: "\r",
      t: "\t",
      0: "\0",
      "\\": "\\",
      '"': '"',
      "'": "'",
    };
    if (Object.hasOwn(simple, escaped)) {
      result.push(simple[escaped]);
    } else if (escaped === "x") {
      result.push(
        String.fromCodePoint(Number.parseInt(body.slice(index, index + 2), 16)),
      );
      index += 2;
    } else if (escaped === "u" && body[index] === "{") {
      const end = body.indexOf("}", index);
      if (end < 0) {
        throw new Error("unterminated unicode escape");
      }
      const codePoint = Number.parseInt(
        body.slice(index + 1, end).replaceAll("_", ""),
        16,
      );
      result.push(String.fromCodePoint(codePoint));
      index = end + 1;
    } else if (escaped === "\n") {
      while (index < body.length && /[ \t\n\r]/.test(body[index])) {
        index += 1;
      }
    } else if (escaped === "\r") {
      if (body[index] === "\n") {
        index += 1;
      }
      while (index < body.length && /[ \t\n\r]/.test(body[index])) {
        index += 1;
      }
    } else {
      throw new Error(`unsupported escape: \\${escaped}`);
    }
  }
  return result.join("");
}

function lex(source) {
  const tokens = [];
  const lineStarts = [0];
  for (const match of source.matchAll(/\n/g)) {
    lineStarts.push(match.index + 1);
  }

  const add = (kind, start, end, value = null) => {
    tokens.push({
      kind,
      text: source.slice(start, end),
      value,
      start,
      end,
      line: upperBound(lineStarts, start),
    });
  };

  let index = 0;
  while (index < source.length) {
    const character = source[index];
    if (/\s/.test(character)) {
      index += 1;
      continue;
    }
    if (source.startsWith("//", index)) {
      const newline = source.indexOf("\n", index + 2);
      index = newline < 0 ? source.length : newline + 1;
      continue;
    }
    if (source.startsWith("/*", index)) {
      let depth = 1;
      let cursor = index + 2;
      while (depth > 0) {
        const opening = source.indexOf("/*", cursor);
        const closing = source.indexOf("*/", cursor);
        if (closing < 0) {
          throw new Error(`unterminated block comment at ${index}`);
        }
        if (opening >= 0 && opening < closing) {
          depth += 1;
          cursor = opening + 2;
        } else {
          depth -= 1;
          cursor = closing + 2;
        }
      }
      index = cursor;
      continue;
    }

    const raw = source.slice(index).match(/^(?:br|r)(#{0,255})"/);
    if (raw) {
      const hashes = raw[1];
      const contentStart = index + raw[0].length;
      const terminator = `"${hashes}`;
      const contentEnd = source.indexOf(terminator, contentStart);
      if (contentEnd < 0) {
        throw new Error(`unterminated raw string at ${index}`);
      }
      const end = contentEnd + terminator.length;
      add("string", index, end, source.slice(contentStart, contentEnd));
      index = end;
      continue;
    }

    const prefixLength = source.startsWith('b"', index) ? 1 : 0;
    if (character === '"' || prefixLength > 0) {
      const start = index;
      let cursor = index + prefixLength + 1;
      let escaped = false;
      while (cursor < source.length) {
        const current = source[cursor];
        if (current === '"' && !escaped) {
          break;
        }
        if (current === "\\" && !escaped) {
          escaped = true;
        } else {
          escaped = false;
        }
        cursor += 1;
      }
      if (cursor >= source.length) {
        throw new Error(`unterminated string at ${start}`);
      }
      const bodyStart = index + prefixLength + 1;
      add(
        "string",
        start,
        cursor + 1,
        decodeString(source.slice(bodyStart, cursor)),
      );
      index = cursor + 1;
      continue;
    }

    if (/[A-Za-z_]/.test(character)) {
      const identifier = source
        .slice(index)
        .match(/^[A-Za-z_][A-Za-z0-9_]*/)[0];
      const end = index + identifier.length;
      add("ident", index, end, identifier);
      index = end;
      continue;
    }

    add("punct", index, index + 1, character);
    index += 1;
  }
  return tokens;
}

function matching(tokens) {
  const pairs = new Map([
    ["(", ")"],
    ["[", "]"],
    ["{", "}"],
  ]);
  const closingTokens = new Set(pairs.values());
  const stack = [];
  const result = new Map();
  for (const [index, token] of tokens.entries()) {
    if (pairs.has(token.text)) {
      stack.push([token.text, index]);
    } else if (closingTokens.has(token.text)) {
      if (stack.length === 0 || pairs.get(stack.at(-1)[0]) !== token.text) {
        continue;
      }
      const [, opening] = stack.pop();
      result.set(opening, index);
      result.set(index, opening);
    }
  }
  return result;
}

function findFunctions(tokens, matches) {
  const result = [];
  for (const [index, token] of tokens.entries()) {
    if (token.text !== "fn" || index + 1 >= tokens.length) {
      continue;
    }
    const name = tokens[index + 1].text;
    let cursor = index + 2;
    while (cursor < tokens.length && tokens[cursor].text !== "{") {
      if (tokens[cursor].text === ";") {
        break;
      }
      cursor += 1;
    }
    if (
      cursor >= tokens.length ||
      tokens[cursor].text !== "{" ||
      !matches.has(cursor)
    ) {
      continue;
    }
    let prefixStart = index - 1;
    while (
      prefixStart >= 0 &&
      !new Set(["{", "}", ";"]).has(tokens[prefixStart].text)
    ) {
      prefixStart -= 1;
    }
    const prefix = tokens
      .slice(prefixStart + 1, index)
      .map((item) => item.text);
    result.push({
      name,
      startToken: index,
      bodyStartToken: cursor,
      endToken: matches.get(cursor),
      isTest: prefix.includes("#") && prefix.includes("test"),
    });
  }
  return result;
}

function stripReference(expression) {
  let start = 0;
  while (
    start < expression.length &&
    ["&", "*"].includes(expression[start].text)
  ) {
    start += 1;
  }
  return expression.slice(start);
}

function dedent(value) {
  const lines = value.split("\n");
  const indents = lines
    .filter((line) => line.trim().length > 0)
    .map((line) => line.match(/^[ \t]*/)[0].length);
  const width = indents.length > 0 ? Math.min(...indents) : 0;
  return lines
    .map((line) => (line.trim().length === 0 ? "" : line.slice(width)))
    .join("\n");
}

function evaluateExpression(originalExpression, variables) {
  const expression = stripReference(originalExpression);
  if (expression.length === 1) {
    const token = expression[0];
    if (token.kind === "string") {
      return token.value;
    }
    if (token.kind === "ident") {
      return variables.get(token.text) ?? null;
    }
  }

  if (
    expression.length >= 4 &&
    expression[0].kind === "ident" &&
    expression[1].text === "!"
  ) {
    const macro = expression[0].text;
    const inner = ["(", "[", "{"].includes(expression[2].text)
      ? expression.slice(3, -1)
      : [];
    const strings = inner
      .filter((token) => token.kind === "string")
      .map((token) => token.value);
    if (macro === "indoc" && strings.length === 1) {
      return dedent(strings[0]).replace(/^\n+/, "").trimEnd();
    }
    if (macro === "concat" && strings.length > 0) {
      return strings.join("");
    }
    if (
      macro === "format" &&
      strings.length === 1 &&
      !strings[0].includes("{")
    ) {
      return strings[0];
    }
  }
  return null;
}

function firstArgument(tokens, opening, closing) {
  let depth = 0;
  for (let index = opening + 1; index < closing; index += 1) {
    const text = tokens[index].text;
    if (["(", "[", "{"].includes(text)) {
      depth += 1;
    } else if ([")", "]", "}"].includes(text)) {
      depth -= 1;
    } else if (text === "," && depth === 0) {
      return tokens.slice(opening + 1, index);
    }
  }
  return tokens.slice(opening + 1, closing);
}

function testForLine(tests, tokens, line) {
  for (const test of tests) {
    const start = tokens[test.startToken].line;
    const end = tokens[test.endToken].line;
    if (start <= line && line <= end) {
      return test.name;
    }
  }
  return "unknown";
}

function decodeDebugString(value) {
  const tokens = lex(value);
  if (tokens.length !== 1 || tokens[0].kind !== "string") {
    throw new Error(`invalid Rust debug string: ${value.slice(0, 80)}`);
  }
  return tokens[0].value;
}

function compareCases(left, right) {
  const fields = [
    "test",
    "line",
    "source",
    "expected",
    "error_recovery",
    "css_modules",
    "flags",
  ];
  for (const field of fields) {
    if (left[field] < right[field]) return -1;
    if (left[field] > right[field]) return 1;
  }
  return 0;
}

async function extractLightningRuntime(sourcePath, logPath) {
  const source = await readFile(sourcePath, "utf8");
  const tokens = lex(source);
  const matches = matching(tokens);
  const tests = findFunctions(tokens, matches).filter((item) => item.isTest);
  const log = await readFile(logPath, "utf8");
  const pattern =
    /ROCKETCSS_CORPUS\|(\d+)\|(true|false)\|(true|false)\|(true|false)\|(\d+)\|(.*?)\|ROCKETCSS_CORPUS_END/g;
  const cases = [];

  for (const match of log.matchAll(pattern)) {
    const [, lineText, ok, recovery, cssModules, flags, debugSource] = match;
    const line = Number.parseInt(lineText, 10);
    cases.push({
      test: testForLine(tests, tokens, line),
      line,
      expected: ok === "true" ? "ok" : "err",
      error_recovery: recovery === "true",
      css_modules: cssModules === "true",
      flags: Number.parseInt(flags, 10),
      source: decodeDebugString(debugSource),
    });
  }

  cases.sort(compareCases);
  const ordinals = new Map();
  for (const item of cases) {
    const key = `${item.test}\0${item.line}`;
    const ordinal = (ordinals.get(key) ?? 0) + 1;
    ordinals.set(key, ordinal);
    item.name = `${item.test}:${item.line}/${ordinal}`;
  }

  const crateRoot = path.dirname(path.dirname(sourcePath));
  const companionSpecs = [
    [path.join(crateRoot, "tests/test_custom_parser.rs"), "minify_test", false],
    [path.join(crateRoot, "tests/test_serde.rs"), "parse", true],
  ];
  for (const [companionPath, helper, qualified] of companionSpecs) {
    const companionSource = await readFile(companionPath, "utf8");
    const companionTokens = lex(companionSource);
    const companionMatches = matching(companionTokens);
    const companionFunctions = findFunctions(companionTokens, companionMatches);
    for (const test of companionFunctions.filter((item) => item.isTest)) {
      const variables = new Map();
      let cursor = test.bodyStartToken + 1;
      while (cursor < test.endToken) {
        const token = companionTokens[cursor];
        if (token.text === "let" && cursor + 3 < test.endToken) {
          const name = companionTokens[cursor + 1];
          let equals = cursor + 2;
          while (
            equals < test.endToken &&
            !["=", ";"].includes(companionTokens[equals].text)
          ) {
            equals += 1;
          }
          if (equals < test.endToken && companionTokens[equals].text === "=") {
            let end = equals + 1;
            while (end < test.endToken && companionTokens[end].text !== ";") {
              end += 1;
            }
            const value = evaluateExpression(
              companionTokens.slice(equals + 1, end),
              variables,
            );
            if (name.kind === "ident" && value !== null) {
              variables.set(name.text, value);
            }
          }
        }

        const isQualified =
          cursor >= 3 &&
          companionTokens[cursor - 1].text === ":" &&
          companionTokens[cursor - 2].text === ":" &&
          companionTokens[cursor - 3].text === "StyleSheet";
        if (
          token.text === helper &&
          cursor + 1 < test.endToken &&
          companionTokens[cursor + 1].text === "(" &&
          isQualified === qualified
        ) {
          const opening = cursor + 1;
          const closing = companionMatches.get(opening);
          const expression = firstArgument(companionTokens, opening, closing);
          const value = evaluateExpression(expression, variables);
          if (value === null) {
            throw new Error(
              `unresolved companion parser input in ${companionPath}:${token.line}`,
            );
          }
          cases.push({
            test: test.name,
            line: token.line,
            expected: "ok",
            error_recovery: false,
            css_modules: false,
            flags: 0,
            source: value,
            name: `${path.basename(companionPath)}/${test.name}:${token.line}`,
          });
          cursor = closing;
        }
        cursor += 1;
      }
    }
  }

  const excludedParserErrors = cases.filter(
    (item) => item.expected === "err",
  ).length;
  return {
    upstream: "lightningcss",
    version: "1.0.0-alpha.71",
    source_file: "src/lib.rs",
    extraction:
      "instrumented StyleSheet::parse calls from an upstream test run",
    cases: cases.filter((item) => item.expected === "ok"),
    excluded_parser_errors: excludedParserErrors,
    unresolved: [],
  };
}

async function extractStylo(sourcePath) {
  const source = await readFile(sourcePath, "utf8");
  const tokens = lex(source);
  const matches = matching(tokens);
  const allFunctions = findFunctions(tokens, matches);
  const tests = allFunctions.filter((item) => item.isTest);
  const parseHelpers = new Set([
    "parse",
    "parse_expected",
    "parse_relative",
    "parse_relative_expected",
    "parse_ns",
    "parse_ns_expected",
    "parse_ns_relative",
    "parse_ns_relative_expected",
    "ancestor_hash_count",
  ]);
  const cases = [];
  const unresolved = [];

  for (const test of tests) {
    const nestedFunctions = allFunctions.filter(
      (item) =>
        test.bodyStartToken < item.startToken &&
        item.startToken < test.endToken,
    );
    let ordinal = 0;
    const variables = new Map();
    let cursor = test.bodyStartToken + 1;
    while (cursor < test.endToken) {
      const token = tokens[cursor];
      if (
        parseHelpers.has(token.text) &&
        cursor + 1 < test.endToken &&
        tokens[cursor + 1].text === "(" &&
        !(cursor > 0 && tokens[cursor - 1].text === "fn") &&
        !nestedFunctions.some(
          (item) => item.bodyStartToken <= cursor && cursor <= item.endToken,
        ) &&
        !(
          cursor >= 2 &&
          tokens[cursor - 1].text === ":" &&
          tokens[cursor - 2].text === ":"
        )
      ) {
        const opening = cursor + 1;
        const closing = matches.get(opening);
        if (closing === undefined) {
          cursor += 1;
          continue;
        }
        const expression = firstArgument(tokens, opening, closing);
        const value = evaluateExpression(expression, variables);
        ordinal += 1;
        const tail = tokens
          .slice(closing + 1, Math.min(closing + 8, tokens.length))
          .map((item) => item.text)
          .join("");
        const item = {
          name: `${test.name}/${ordinal}`,
          test: test.name,
          helper: token.text,
          line: token.line,
          expected: tail.includes(".is_err(") ? "err" : "ok",
        };
        if (value === null) {
          item.expression =
            expression.length > 0
              ? source.slice(expression[0].start, expression.at(-1).end)
              : "";
          unresolved.push(item);
        } else {
          item.source = value;
          cases.push(item);
        }
        cursor = closing;
      }
      cursor += 1;
    }
  }

  const excludedParserErrors = cases.filter(
    (item) => item.expected === "err",
  ).length;
  return {
    upstream: "stylo/selectors",
    version: "stylo-0.19.0 / selectors parser mirror",
    source_file: "selectors/parser.rs",
    cases: cases.filter((item) => item.expected === "ok"),
    excluded_parser_errors: excludedParserErrors,
    unresolved,
  };
}

function parseArguments(rawArguments) {
  const argumentsToParse = [...rawArguments];
  if (
    argumentsToParse[0] &&
    path.resolve(argumentsToParse[0]) === fileURLToPath(import.meta.url)
  ) {
    argumentsToParse.shift();
  }
  if (argumentsToParse.includes("-h") || argumentsToParse.includes("--help")) {
    return { help: true };
  }
  const options = {};
  while (argumentsToParse.length > 0) {
    const argument = argumentsToParse.shift();
    if (
      ![
        "--lightningcss",
        "--lightningcss-log",
        "--stylo-selectors",
        "--output",
      ].includes(argument)
    ) {
      throw new Error(`unknown option: ${argument}`);
    }
    const value = argumentsToParse.shift();
    if (!value) {
      throw new Error(`${argument} requires a value`);
    }
    options[argument.slice(2).replaceAll("-", "_")] = path.resolve(value);
  }
  for (const name of [
    "lightningcss",
    "lightningcss_log",
    "stylo_selectors",
    "output",
  ]) {
    if (!options[name]) {
      throw new Error(`--${name.replaceAll("_", "-")} is required`);
    }
  }
  return options;
}

const usage = `
Generate the checked-in parser corpus from pinned upstream sources.

Usage:
  pnpm parser-test-corpus \\
    --lightningcss /path/to/lightningcss/src/lib.rs \\
    --lightningcss-log /path/to/lightningcss-test.log \\
    --stylo-selectors /path/to/selectors/parser.rs \\
    --output crates/parser/tests/upstream/corpus.json
`;

async function main() {
  const options = parseArguments(process.argv.slice(2));
  if (options.help) {
    console.log(usage.trim());
    return;
  }
  const data = {
    format: 1,
    lightningcss: await extractLightningRuntime(
      options.lightningcss,
      options.lightningcss_log,
    ),
    stylo: await extractStylo(options.stylo_selectors),
  };
  await mkdir(path.dirname(options.output), { recursive: true });
  await writeFile(options.output, `${JSON.stringify(data, null, 2)}\n`);
}

main().catch((error) => {
  console.error(chalk.red(error.stack ?? error.message ?? String(error)));
  process.exitCode = 2;
});
