import { dirname, join, posix } from "node:path";
import { compact, maybeFunction, rustCode } from "../check-phase157-index-catch-up/rust-evidence.ts";
import { CLI, NODE, RPC, SOURCE_FILES } from "./contracts.ts";

type ModuleEdge = { parent: string; child: string; symbol: string };
type Texts = (file: string) => string;
const overrides = new Map<string, [string, string]>([
  [NODE + "network/runtime_authority/filter_index/query/tests.rs", [NODE + "sync/tests/filter_index/catch_up.rs", "phase159_query_tests"]],
  [NODE + "network/runtime_authority/filter_index/readiness/tests.rs", [NODE + "sync/tests/filter_index/catch_up.rs", "phase159_readiness_tests"]],
  [NODE + "storage/fjall_store/filters/query/tests.rs", [NODE + "storage/fjall_store/filters/tests.rs", "query"]],
  [NODE + "storage/fjall_store/validation_history/tests.rs", [NODE + "storage/validation_history.rs", "store_tests"]],
  [RPC + "bin/open_bitcoind/coins_flush.rs", [RPC + "bin/open-bitcoind.rs", "coins_flush"]],
  [RPC + "bin/open_bitcoind/tests.rs", [RPC + "bin/open-bitcoind.rs", "tests"]],
  [CLI + "client.rs", [CLI + "main.rs", "client"]],
]);

function ownerOf(child: string): ModuleEdge | undefined {
  if ([NODE + "lib.rs", RPC + "lib.rs", RPC + "bin/open-bitcoind.rs", CLI + "lib.rs", CLI + "main.rs"].includes(child)) return undefined;
  const maybeOverride = overrides.get(child);
  if (maybeOverride) return { parent: maybeOverride[0], symbol: maybeOverride[1], child };
  const directory = dirname(child);
  return { parent: directory.endsWith("/src") ? join(directory, "lib.rs") : directory + ".rs", child, symbol: posix.basename(child, ".rs") };
}

/** Fixed current owners include the physical-path exceptions used by real Rust modules. */
function moduleEdges(): ModuleEdge[] {
  const edges = new Map<string, ModuleEdge>();
  for (const file of SOURCE_FILES) {
    let maybeEdge = ownerOf(file);
    while (maybeEdge && !edges.has(maybeEdge.child)) {
      edges.set(maybeEdge.child, maybeEdge);
      maybeEdge = ownerOf(maybeEdge.parent);
    }
  }
  return [...edges.values()];
}
export const MODULE_EDGES = moduleEdges();
export const OWNER_FILES = [...new Set(MODULE_EDGES.flatMap(edge => [edge.parent, edge.child]))];

function attributesBefore(code: string, offset: number): { source: string; start: number } {
  let prefix = code.slice(0, offset);
  while (true) {
    const previous = prefix;
    prefix = prefix.replace(/\b(?:pub(?:\s*\([^)]*\))?|async|const|unsafe|extern)\s*$/, "");
    if (prefix === previous) break;
  }
  const source = prefix.match(/(?:#\s*\[[^\]]*\]\s*)+$/)?.[0] ?? "";
  return { source, start: prefix.length - source.length };
}

function supportedAttributes(attributes: string, testConfiguration: boolean): boolean {
  for (const attribute of attributes.matchAll(/#\s*!?\s*\[([^\]]*)\]/g)) {
    const item = compact(attribute[1]);
    if (item === "ignore" || item.startsWith("ignore(")) return false;
    if (item.startsWith("cfg_attr(")) {
      // Preserve the existing crate lint policy; it cannot exclude tests or add ignore.
      const lintPolicy = /^cfg_attr\(not\(test\),deny\((?:clippy::[a-z_]+,?)+\)\)$/;
      if (!lintPolicy.test(item)) return false;
    }
    if (item.startsWith("cfg(") && !(testConfiguration && item === "cfg(test)")) return false;
  }
  return true;
}

function closingBrace(code: string, open: number): number {
  let depth = 1;
  let end = open + 1;
  while (end < code.length && depth) {
    if (code[end] === "{") depth++;
    if (code[end] === "}") depth--;
    end++;
  }
  return depth ? -1 : end;
}

/** Check declaration/file/inline module ownership only, never arbitrary body attributes. */
export function supportedOwnership(source: string, offset: number, testConfiguration: boolean): boolean {
  const code = rustCode(source);
  if (!supportedAttributes(attributesBefore(code, offset).source, testConfiguration)) return false;
  const inner = code.match(/^\s*(?:#!\s*\[[^\]]*\]\s*)*/)?.[0] ?? "";
  if (!supportedAttributes(inner, testConfiguration)) return false;
  for (const module of code.matchAll(/\bmod\s+\w+\s*\{/g)) {
    const open = module.index + module[0].lastIndexOf("{");
    const end = closingBrace(code, open);
    if (open < offset && end > offset && !supportedAttributes(attributesBefore(code, module.index).source, testConfiguration)) return false;
  }
  return true;
}

/** Every fixed source/test owner must remain registered at its actual module path. */
export function checkModuleOwnership(text: Texts, failures: string[]): void {
  for (const edge of MODULE_EDGES) {
    const source = text(edge.parent);
    const code = rustCode(source);
    const declarations = [...code.matchAll(new RegExp(`\\bmod\\s+${edge.symbol}\\s*;`, "g"))];
    if (declarations.length !== 1) { failures.push(`${edge.parent}: owning module registration ${edge.symbol}`); continue; }
    const declaration = declarations[0];
    // Canonical cfg(test) is valid when compiling tests, but cannot own production code.
    const isTest = /\/tests(?:\/|\.rs)|_tests\.rs/.test(edge.child);
    if (!supportedOwnership(source, declaration.index, true)
      || (!isTest && !supportedOwnership(source, declaration.index, false))) failures.push(`${edge.parent}: unsupported owning attributes ${edge.symbol}`);
    const attributes = attributesBefore(code, declaration.index);
    const original = source.slice(attributes.start, declaration.index);
    const maybePath = original.match(/#\s*\[\s*path\s*=\s*"([^\"]+)"\s*\]/)?.[1];
    const actual = maybePath ? posix.normalize(join(dirname(edge.parent), maybePath))
      : join(["lib.rs", "main.rs"].includes(posix.basename(edge.parent)) ? dirname(edge.parent) : edge.parent.slice(0, -3), edge.symbol + ".rs");
    if (actual !== edge.child) failures.push(`${edge.parent}: owning module path ${edge.symbol}`);
    const childCode = rustCode(text(edge.child));
    const inner = childCode.match(/^\s*(?:#!\s*\[[^\]]*\]\s*)*/)?.[0] ?? "";
    if (!supportedAttributes(inner, true) || (!isTest && !supportedAttributes(inner, false))) failures.push(`${edge.child}: unsupported owning file attributes`);
  }
}

function argumentsOf(expression: string): string[] {
  const arguments_: string[] = [];
  let depth = 0;
  let start = 0;
  for (let index = 0; index < expression.length; index++) {
    if ("([{<".includes(expression[index])) depth++;
    if (")]}>".includes(expression[index])) depth--;
    if (expression[index] === "," && depth === 0) { arguments_.push(expression.slice(start, index)); start = index + 1; }
  }
  arguments_.push(expression.slice(start));
  return arguments_;
}

function literalExpression(expression: string): boolean {
  // Deliberately bounded lexical classification, not Rust constant evaluation.
  const identifiers = expression
    .replace(/\b(?:0[xX][\da-fA-F_]+|0[bB][01_]+|\d[\d_]*(?:\.\d[\d_]*)?)(?:[ui](?:8|16|32|64|128|size)|f(?:32|64))?\b/g, "")
    .replace(/\b(?:true|false)\b/g, "")
    .replace(/\b(?:[ui](?:8|16|32|64|128|size)|f(?:32|64))::(?:MAX|MIN)\b/g, "");
  return !/[A-Za-z_]/.test(identifiers);
}

function selfComparison(expression: string): boolean {
  const maybeComparison = compact(expression).match(/^\(*(.*?)\s*(==|!=|<=|>=|<|>)\s*(.*?)\)*$/);
  if (!maybeComparison) return false;
  const left = maybeComparison[1].replace(/^\(+|\)+$/g, "");
  const right = maybeComparison[3].replace(/^\(+|\)+$/g, "");
  return left === right && /^[&*]?[A-Za-z_]\w*(?:(?:::|\.)\w+|\[[\w\d]+\])*$/.test(left);
}

/** Reject literal-only and simple self-comparison assertions, including message arguments. */
export function meaningfulAssertions(source: string, body: string, depth = 0): boolean {
  for (const match of body.matchAll(/\b(assert(?:_eq|_ne)?)\s*!\s*\(/g)) {
    let nesting = 1;
    let end = match.index + match[0].length;
    const start = end;
    while (end < body.length && nesting) {
      if (body[end] === "(") nesting++;
      if (body[end] === ")") nesting--;
      end++;
    }
    const arguments_ = argumentsOf(body.slice(start, end - 1));
    if (match[1] === "assert") {
      const condition = arguments_[0];
      if (!literalExpression(condition) && !selfComparison(condition)) return true;
    } else {
      const operands = arguments_.slice(0, 2).map(compact);
      const selfEqual = operands[0] === operands[1] && /^[&*]?[A-Za-z_]\w*(?:(?:::|\.)\w+|\[[\w\d]+\])*$/.test(operands[0]);
      if (operands.length === 2 && !operands.every(literalExpression) && !selfEqual) return true;
    }
  }
  if (depth >= 3) return false;
  return [...body.matchAll(/\b(\w+)\s*\(/g)].some(match => {
    const maybeHelper = maybeFunction(source, match[1]);
    return maybeHelper !== undefined && supportedOwnership(source, maybeHelper.start, true)
      && meaningfulAssertions(source, maybeHelper.body, depth + 1);
  });
}
