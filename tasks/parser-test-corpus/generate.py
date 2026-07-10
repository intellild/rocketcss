#!/usr/bin/env python3
"""Extract text-to-AST parser cases from the pinned upstream Rust sources.

This is intentionally a small Rust lexer rather than a collection of regular
expressions. It keeps the copied corpus traceable to the exact upstream call
site while discarding assertions about printing, minification, and transforms.
"""

from __future__ import annotations

import argparse
import ast
import bisect
import dataclasses
import json
import re
import textwrap
from pathlib import Path


@dataclasses.dataclass(frozen=True)
class Token:
    kind: str
    text: str
    value: str | None
    start: int
    end: int
    line: int


@dataclasses.dataclass(frozen=True)
class Function:
    name: str
    start_token: int
    body_start_token: int
    end_token: int
    is_test: bool


def decode_string(body: str) -> str:
    result: list[str] = []
    index = 0
    while index < len(body):
        char = body[index]
        if char != "\\":
            result.append(char)
            index += 1
            continue

        index += 1
        if index >= len(body):
            raise ValueError("unterminated escape")
        escaped = body[index]
        index += 1
        simple = {
            "n": "\n",
            "r": "\r",
            "t": "\t",
            "0": "\0",
            "\\": "\\",
            '"': '"',
            "'": "'",
        }
        if escaped in simple:
            result.append(simple[escaped])
        elif escaped == "x":
            result.append(chr(int(body[index : index + 2], 16)))
            index += 2
        elif escaped == "u" and index < len(body) and body[index] == "{":
            end = body.index("}", index)
            result.append(chr(int(body[index + 1 : end].replace("_", ""), 16)))
            index = end + 1
        elif escaped == "\n":
            while index < len(body) and body[index] in " \t\n\r":
                index += 1
        elif escaped == "\r":
            if index < len(body) and body[index] == "\n":
                index += 1
            while index < len(body) and body[index] in " \t\n\r":
                index += 1
        else:
            raise ValueError(f"unsupported escape: \\{escaped}")
    return "".join(result)


def lex(source: str) -> list[Token]:
    tokens: list[Token] = []
    line_starts = [0]
    for match in re.finditer("\n", source):
        line_starts.append(match.end())

    def add(kind: str, start: int, end: int, value: str | None = None) -> None:
        tokens.append(
            Token(
                kind,
                source[start:end],
                value,
                start,
                end,
                bisect.bisect_right(line_starts, start),
            )
        )

    index = 0
    while index < len(source):
        char = source[index]
        if char.isspace():
            index += 1
            continue
        if source.startswith("//", index):
            newline = source.find("\n", index + 2)
            index = len(source) if newline < 0 else newline + 1
            continue
        if source.startswith("/*", index):
            depth = 1
            cursor = index + 2
            while depth:
                opening = source.find("/*", cursor)
                closing = source.find("*/", cursor)
                if closing < 0:
                    raise ValueError(f"unterminated block comment at {index}")
                if 0 <= opening < closing:
                    depth += 1
                    cursor = opening + 2
                else:
                    depth -= 1
                    cursor = closing + 2
            index = cursor
            continue

        raw = re.match(r"(?:br|r)(?P<hashes>#{0,255})\"", source[index:])
        if raw:
            hashes = raw.group("hashes")
            content_start = index + raw.end()
            terminator = '"' + hashes
            content_end = source.find(terminator, content_start)
            if content_end < 0:
                raise ValueError(f"unterminated raw string at {index}")
            end = content_end + len(terminator)
            add("string", index, end, source[content_start:content_end])
            index = end
            continue

        prefix_length = 1 if source.startswith('b"', index) else 0
        if char == '"' or prefix_length:
            start = index
            cursor = index + prefix_length + 1
            escaped = False
            while cursor < len(source):
                current = source[cursor]
                if current == '"' and not escaped:
                    break
                if current == "\\" and not escaped:
                    escaped = True
                else:
                    escaped = False
                cursor += 1
            if cursor >= len(source):
                raise ValueError(f"unterminated string at {start}")
            body_start = index + prefix_length + 1
            add("string", start, cursor + 1, decode_string(source[body_start:cursor]))
            index = cursor + 1
            continue

        if char.isalpha() or char == "_":
            match = re.match(r"[A-Za-z_][A-Za-z0-9_]*", source[index:])
            assert match
            end = index + match.end()
            add("ident", index, end, source[index:end])
            index = end
            continue

        add("punct", index, index + 1, char)
        index += 1
    return tokens


def matching(tokens: list[Token]) -> dict[int, int]:
    pairs = {"(": ")", "[": "]", "{": "}"}
    stack: list[tuple[str, int]] = []
    result: dict[int, int] = {}
    for index, token in enumerate(tokens):
        if token.text in pairs:
            stack.append((token.text, index))
        elif token.text in pairs.values():
            if not stack or pairs[stack[-1][0]] != token.text:
                continue
            _, opening = stack.pop()
            result[opening] = index
            result[index] = opening
    return result


def functions(tokens: list[Token], matches: dict[int, int]) -> list[Function]:
    result: list[Function] = []
    for index, token in enumerate(tokens):
        if token.text != "fn" or index + 1 >= len(tokens):
            continue
        name = tokens[index + 1].text
        cursor = index + 2
        while cursor < len(tokens) and tokens[cursor].text != "{":
            if tokens[cursor].text == ";":
                break
            cursor += 1
        if cursor >= len(tokens) or tokens[cursor].text != "{" or cursor not in matches:
            continue
        prefix_start = index - 1
        while prefix_start >= 0 and tokens[prefix_start].text not in {"{", "}", ";"}:
            prefix_start -= 1
        prefix = [item.text for item in tokens[prefix_start + 1 : index]]
        is_test = "#" in prefix and "test" in prefix
        result.append(Function(name, index, cursor, matches[cursor], is_test))
    return result


def strip_reference(expression: list[Token]) -> list[Token]:
    while expression and expression[0].text in {"&", "*"}:
        expression = expression[1:]
    return expression


def eval_expression(expression: list[Token], variables: dict[str, str]) -> str | None:
    expression = strip_reference(expression)
    if len(expression) == 1:
        token = expression[0]
        if token.kind == "string":
            return token.value
        if token.kind == "ident":
            return variables.get(token.text)

    if len(expression) >= 4 and expression[0].kind == "ident" and expression[1].text == "!":
        macro = expression[0].text
        inner = expression[3:-1] if expression[2].text in {"(", "[", "{"} else []
        strings = [token.value for token in inner if token.kind == "string"]
        if macro == "indoc" and len(strings) == 1:
            return textwrap.dedent(strings[0]).lstrip("\n").rstrip()
        if macro == "concat" and strings:
            return "".join(value for value in strings if value is not None)
        if macro == "format" and len(strings) == 1 and "{" not in strings[0]:
            return strings[0]
    return None


def first_argument(tokens: list[Token], opening: int, closing: int) -> list[Token]:
    depth = 0
    for index in range(opening + 1, closing):
        text = tokens[index].text
        if text in {"(", "[", "{"}:
            depth += 1
        elif text in {")", "]", "}"}:
            depth -= 1
        elif text == "," and depth == 0:
            return tokens[opening + 1 : index]
    return tokens[opening + 1 : closing]


LIGHTNING_SUCCESS = {
    "test",
    "test_with_options",
    "test_with_printer_options",
    "minify_test",
    "minify_test_with_options",
    "minify_error_test_with_options",
    "prefix_test",
    "nesting_test",
    "nesting_test_with_targets",
    "nesting_test_no_targets",
    "css_modules_test",
    "custom_media_test",
}
LIGHTNING_FAILURE = {"error_test", "error_test_with_options", "css_modules_error_test"}
LIGHTNING_RECOVERY = {"error_recovery_test"}


def extract_lightning(path: Path) -> dict[str, object]:
    source = path.read_text()
    tokens = lex(source)
    matches = matching(tokens)
    all_functions = functions(tokens, matches)
    test_functions = [function for function in all_functions if function.is_test]
    cases: list[dict[str, object]] = []
    unresolved: list[dict[str, object]] = []

    for test in test_functions:
        local_functions = {
            function.name
            for function in all_functions
            if test.body_start_token < function.start_token < test.end_token
        }
        variables: dict[str, str] = {}
        ordinal = 0
        cursor = test.body_start_token + 1
        while cursor < test.end_token:
            token = tokens[cursor]
            if token.text == "let" and cursor + 3 < test.end_token:
                name_token = tokens[cursor + 1]
                equals = cursor + 2
                while equals < test.end_token and tokens[equals].text not in {"=", ";"}:
                    equals += 1
                if equals < test.end_token and tokens[equals].text == "=":
                    end = equals + 1
                    depth = 0
                    while end < test.end_token:
                        if tokens[end].text in {"(", "[", "{"}:
                            depth += 1
                        elif tokens[end].text in {")", "]", "}"}:
                            depth -= 1
                        elif tokens[end].text == ";" and depth == 0:
                            break
                        end += 1
                    value = eval_expression(tokens[equals + 1 : end], variables)
                    if name_token.kind == "ident" and value is not None:
                        variables[name_token.text] = value

            helper = token.text
            expected = None
            if helper in LIGHTNING_SUCCESS:
                expected = "ok"
            elif helper in LIGHTNING_FAILURE:
                expected = "err"
            elif helper in LIGHTNING_RECOVERY:
                expected = "recovery"
            if (
                expected
                and helper not in local_functions
                and cursor + 1 < test.end_token
                and tokens[cursor + 1].text == "("
                and tokens[cursor + 1].start in range(token.end, tokens[cursor + 1].end + 1)
            ):
                opening = cursor + 1
                closing = matches.get(opening)
                if closing is not None and closing <= test.end_token:
                    expression = first_argument(tokens, opening, closing)
                    value = eval_expression(expression, variables)
                    ordinal += 1
                    item = {
                        "name": f"{test.name}/{ordinal}",
                        "test": test.name,
                        "helper": helper,
                        "line": token.line,
                        "expected": expected,
                    }
                    if value is None:
                        item["expression"] = source[
                            expression[0].start : expression[-1].end
                        ] if expression else ""
                        unresolved.append(item)
                    else:
                        item["source"] = value
                        cases.append(item)
                    cursor = closing
            cursor += 1

    parser_errors = sum(case["expected"] == "err" for case in cases)
    cases = [case for case in cases if case["expected"] != "err"]
    return {
        "upstream": "lightningcss",
        "version": "1.0.0-alpha.71",
        "source_file": "src/lib.rs",
        "cases": cases,
        "excluded_parser_errors": parser_errors,
        "unresolved": unresolved,
    }


def extract_lightning_runtime(path: Path, log_path: Path) -> dict[str, object]:
    """Read calls recorded by the instrumented upstream StyleSheet::parse.

    Running the original tests is required because some inputs are constructed
    by loops and format! expressions. The log also records the parse result and
    option mode used by the upstream parser, so no downstream minify/print
    assertion is copied into this corpus.
    """
    source = path.read_text()
    tokens = lex(source)
    matches = matching(tokens)
    tests = [function for function in functions(tokens, matches) if function.is_test]

    def test_for_line(line: int) -> str:
        for function in tests:
            start = tokens[function.start_token].line
            end = tokens[function.end_token].line
            if start <= line <= end:
                return function.name
        return "unknown"

    cases: list[dict[str, object]] = []
    pattern = re.compile(
        r"RS_CSS_CORPUS\|(\d+)\|(true|false)\|(true|false)\|(true|false)\|(\d+)\|(.*?)\|RS_CSS_CORPUS_END"
    )
    for match in pattern.finditer(log_path.read_text()):
        line, ok, recovery, css_modules, flags, debug_source = match.groups()
        source_value = ast.literal_eval(debug_source)
        cases.append(
            {
                "test": test_for_line(int(line)),
                "line": int(line),
                "expected": "ok" if ok == "true" else "err",
                "error_recovery": recovery == "true",
                "css_modules": css_modules == "true",
                "flags": int(flags),
                "source": source_value,
            }
        )

    cases.sort(
        key=lambda case: (
            case["test"],
            case["line"],
            case["source"],
            case["expected"],
            case["error_recovery"],
            case["css_modules"],
            case["flags"],
        )
    )
    ordinals: dict[tuple[str, int], int] = {}
    for case in cases:
        key = (str(case["test"]), int(case["line"]))
        ordinals[key] = ordinals.get(key, 0) + 1
        case["name"] = f"{key[0]}:{key[1]}/{ordinals[key]}"

    crate_root = path.parent.parent
    companion_specs = [
        (crate_root / "tests/test_custom_parser.rs", "minify_test", False),
        (crate_root / "tests/test_serde.rs", "parse", True),
    ]
    for companion_path, helper, qualified in companion_specs:
        companion_tokens = lex(companion_path.read_text())
        companion_matches = matching(companion_tokens)
        companion_functions = functions(companion_tokens, companion_matches)
        for test in (function for function in companion_functions if function.is_test):
            variables: dict[str, str] = {}
            cursor = test.body_start_token + 1
            while cursor < test.end_token:
                token = companion_tokens[cursor]
                if token.text == "let" and cursor + 3 < test.end_token:
                    name = companion_tokens[cursor + 1]
                    equals = cursor + 2
                    while equals < test.end_token and companion_tokens[equals].text not in {"=", ";"}:
                        equals += 1
                    if equals < test.end_token and companion_tokens[equals].text == "=":
                        end = equals + 1
                        while end < test.end_token and companion_tokens[end].text != ";":
                            end += 1
                        value = eval_expression(companion_tokens[equals + 1 : end], variables)
                        if name.kind == "ident" and value is not None:
                            variables[name.text] = value

                is_qualified = (
                    cursor >= 3
                    and companion_tokens[cursor - 1].text == ":"
                    and companion_tokens[cursor - 2].text == ":"
                    and companion_tokens[cursor - 3].text == "StyleSheet"
                )
                if (
                    token.text == helper
                    and cursor + 1 < test.end_token
                    and companion_tokens[cursor + 1].text == "("
                    and is_qualified == qualified
                ):
                    opening = cursor + 1
                    closing = companion_matches[opening]
                    expression = first_argument(companion_tokens, opening, closing)
                    value = eval_expression(expression, variables)
                    if value is None:
                        raise ValueError(
                            f"unresolved companion parser input in {companion_path}:{token.line}"
                        )
                    cases.append(
                        {
                            "test": test.name,
                            "line": token.line,
                            "expected": "ok",
                            "error_recovery": False,
                            "css_modules": False,
                            "flags": 0,
                            "source": value,
                            "name": f"{companion_path.name}/{test.name}:{token.line}",
                        }
                    )
                    cursor = closing
                cursor += 1

    parser_errors = sum(case["expected"] == "err" for case in cases)
    cases = [case for case in cases if case["expected"] == "ok"]
    return {
        "upstream": "lightningcss",
        "version": "1.0.0-alpha.71",
        "source_file": "src/lib.rs",
        "extraction": "instrumented StyleSheet::parse calls from an upstream test run",
        "cases": cases,
        "excluded_parser_errors": parser_errors,
        "unresolved": [],
    }


def extract_stylo(path: Path) -> dict[str, object]:
    source = path.read_text()
    tokens = lex(source)
    matches = matching(tokens)
    all_functions = functions(tokens, matches)
    tests = [function for function in all_functions if function.is_test]
    parse_helpers = {
        "parse",
        "parse_expected",
        "parse_relative",
        "parse_relative_expected",
        "parse_ns",
        "parse_ns_expected",
        "parse_ns_relative",
        "parse_ns_relative_expected",
        "ancestor_hash_count",
    }
    cases: list[dict[str, object]] = []
    unresolved: list[dict[str, object]] = []
    for test in tests:
        nested_functions = [
            function
            for function in all_functions
            if test.body_start_token < function.start_token < test.end_token
        ]
        ordinal = 0
        variables: dict[str, str] = {}
        cursor = test.body_start_token + 1
        while cursor < test.end_token:
            token = tokens[cursor]
            if (
                token.text in parse_helpers
                and cursor + 1 < test.end_token
                and tokens[cursor + 1].text == "("
                and not (cursor > 0 and tokens[cursor - 1].text == "fn")
                and not any(
                    function.body_start_token <= cursor <= function.end_token
                    for function in nested_functions
                )
                and not (
                    cursor >= 2
                    and tokens[cursor - 1].text == ":"
                    and tokens[cursor - 2].text == ":"
                )
            ):
                opening = cursor + 1
                closing = matches.get(opening)
                if closing is None:
                    cursor += 1
                    continue
                expression = first_argument(tokens, opening, closing)
                value = eval_expression(expression, variables)
                ordinal += 1
                tail = "".join(item.text for item in tokens[closing + 1 : min(closing + 8, len(tokens))])
                expected = "err" if ".is_err(" in tail else "ok"
                item = {
                    "name": f"{test.name}/{ordinal}",
                    "test": test.name,
                    "helper": token.text,
                    "line": token.line,
                    "expected": expected,
                }
                if value is None:
                    item["expression"] = source[
                        expression[0].start : expression[-1].end
                    ] if expression else ""
                    unresolved.append(item)
                else:
                    item["source"] = value
                    cases.append(item)
                cursor = closing
            cursor += 1
    parser_errors = sum(case["expected"] == "err" for case in cases)
    cases = [case for case in cases if case["expected"] == "ok"]
    return {
        "upstream": "stylo/selectors",
        "version": "stylo-0.19.0 / selectors parser mirror",
        "source_file": "selectors/parser.rs",
        "cases": cases,
        "excluded_parser_errors": parser_errors,
        "unresolved": unresolved,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--lightningcss", type=Path, required=True)
    parser.add_argument("--lightningcss-log", type=Path, required=True)
    parser.add_argument("--stylo-selectors", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    arguments = parser.parse_args()
    lightning = extract_lightning_runtime(arguments.lightningcss, arguments.lightningcss_log)
    data = {
        "format": 1,
        "lightningcss": lightning,
        "stylo": extract_stylo(arguments.stylo_selectors),
    }
    arguments.output.parent.mkdir(parents=True, exist_ok=True)
    arguments.output.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n")


if __name__ == "__main__":
    main()
