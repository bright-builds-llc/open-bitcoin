#!/usr/bin/env bun
import { closeSync, constants, fstatSync, lstatSync, openSync, readFileSync, readSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, posix, resolve } from "node:path";
import { CONTRACTS, CHECKPOINT_COUNTS, HISTORICAL_COMMANDS, BREADCRUMBS, TS_CONTRACTS, type Contract } from "./check-phase162-integrated-filter-proof/contracts.ts";
import { compact, hasOrderedCode, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { meaningfulAssertions, supportedOwnership } from "./check-phase159-filter-rpcs/evidence.ts";
import { maybeSelector } from "./check-phase160-peer-filter-serving.ts";
import { CHECKPOINT_CONTRACTS, PHASE162_FILES, PHASE162_EVIDENCE, PHASE162_DOC,
  checkPhase162ClaimState, loadPhase162OptionalSources, checkPhase162ExcludedClaims,
  phase162ClaimView, sourceText } from "./check-phase158-validated-reorg.ts";

const N = "packages/open-bitcoin-node/src/";
const R = "packages/open-bitcoin-rpc/src/";
const B = R + "bin/open_bitcoind/";
const OWNED = ["scripts/check-phase162-integrated-filter-proof.ts", "scripts/check-phase162-integrated-filter-proof/contracts.ts", "scripts/check-phase162-integrated-filter-proof.test.ts"];
const PIN = "a9aee730466ac67d35a3c03ee24676be5e045878";
const SHARED = ["packages/testdata/phase162/recipe.rs", "packages/testdata/phase162/process.rs"];
import { fixtureSourceAttribution, historicalFixtureView, MIGRATED_FIXTURE } from "./check-phase164-canonical-identity/fixture-source.ts";
export { fixtureSourceAttribution, historicalFixtureView, MIGRATED_FIXTURE } from "./check-phase164-canonical-identity/fixture-source.ts";
import { LEGACY_CONTRACT_BUNDLE, legacyFunctionContractView } from "./check-phase164-canonical-identity/fixture-source.ts";
import { verificationHistoricalBase, legacyToolingBase } from "./check-phase164-canonical-identity/historical-tooling.ts";
const PROCESS = SHARED[1];
const UNIX_TESTS = "#[cfg(all(test, unix))]";
const CHECKPOINTS: Contract[] = CHECKPOINT_CONTRACTS.map((row, index) => ({ ...row, kind: "production", counts: CHECKPOINT_COUNTS[index], assertions: [], facts: [] }));
const ROOTS = [...CONTRACTS, ...CHECKPOINTS].map(row => row.file);
type Edge = { parent: string; child: string; symbol: string };

function owningEdges(): Edge[] {
  const edges = new Map<string, Edge>();
  const roots = [N + "lib.rs", R + "lib.rs", R + "bin/open-bitcoind.rs"];
  for (const file of ROOTS.filter(file => file.endsWith(".rs") && !SHARED.includes(file))) {
    let child = file;
    while (!roots.includes(child)) {
      const symbol = posix.basename(child, ".rs");
      const parent = child === R + "inbound_listener/filter_serving.rs" ? R + "inbound_listener/connection_runtime.rs"
        : child === B + "tests.rs" || child === B + "coins_flush.rs" ? R + "bin/open-bitcoind.rs"
        : dirname(child).endsWith("/src") ? posix.join(dirname(child), "lib.rs") : dirname(child) + ".rs";
      if (edges.has(child)) break;
      edges.set(child, { parent, child, symbol });
      child = parent;
    }
  }
  return [...edges.values()];
}
export const MODULE_EDGES = owningEdges();
export const NATIVE_COMMANDS = [
  "bun run scripts/check-phase161-operator-index-evidence.ts",
  "bun test ./scripts/phase162-filter-oracle.test.ts",
  "bun test ./scripts/check-phase162-integrated-filter-proof.test.ts",
  "bun run scripts/check-phase162-integrated-filter-proof.ts",
];
export const SHARED_FORMAT = "rustfmt --edition 2024 --check packages/testdata/phase162/recipe.rs packages/testdata/phase162/process.rs";
export const CHECK_FILES = [...new Set([...ROOTS, ...MODULE_EDGES.flatMap(edge => [edge.parent, edge.child]), ...SHARED, ...PHASE162_FILES, ...PHASE162_EVIDENCE,
  ...OWNED, "scripts/phase162-filter-oracle.test.ts", "scripts/check-phase158-validated-reorg.ts", "scripts/check-phase158-validated-reorg.test.ts",
  N + "chainstate/flush_lifecycle/tests/recovery_and_persist.rs", B + "tests/filter_index/peer_serving/evidence.rs",
  "docs/parity/index.json", "docs/parity/source-breadcrumbs.json", "scripts/verify.sh", "scripts/generate-peer-filter-fixtures.ts", ".gitmodules", "rust-toolchain.toml", ".bun-version", LEGACY_CONTRACT_BUNDLE])];

/** Load only fixed current roots and the frozen claim helper's lazy optional outputs. */
export function loadSources(maybeRoot?: string): Map<string, string> {
  const root = maybeRoot ?? resolve(import.meta.dir, "..");
  const sources = loadPhase162OptionalSources(root);
  for (const file of CHECK_FILES) sources.set(file, file === LEGACY_CONTRACT_BUNDLE ? readLegacyBundle(resolve(root, file)) : readFileSync(resolve(root, file), "utf8"));
  return sources;
}

function readLegacyBundle(file: string): string {
  if (!lstatSync(file).isFile()) throw new Error(`${file}: regular legacy input required`);
  const descriptor = openSync(file, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    if (!fstatSync(descriptor).isFile()) throw new Error(`${file}: regular legacy input required`);
    const bytes = Buffer.alloc(512_001);
    let length = 0;
    while (length < bytes.length) {
      const count = readSync(descriptor, bytes, length, bytes.length - length, null);
      if (count === 0) break;
      length += count;
    }
    if (length > 512_000) throw new Error(`${file}: legacy input byte limit exceeded`);
    return bytes.subarray(0, length).toString("utf8");
  } finally { closeSync(descriptor); }
}

/** Extract only executable assertion spans; masked comments/strings cannot invent a macro. */
export function assertionSpans(source: string, symbol: string, production = false): { raw: string; start: number; end: number }[] {
  return executableSpans(source, symbol, production, /\bassert(?:_eq|_ne)?\s*!\s*\(/g);
}

/** Literal-bearing protocol/matrix facts must occur at real executable span offsets. */
export function factSpans(source: string, symbol: string, production = false): { raw: string; start: number; end: number }[] {
  return executableSpans(source, symbol, production, /\b(?:serde_json::)?json!\s*\(|\bexchanges\.push\s*\(|\bfor\b[^{};]*?\bin\s*\[/g);
}

function executableSpans(source: string, symbol: string, production: boolean, pattern: RegExp): { raw: string; start: number; end: number }[] {
  const maybeBody = maybeSelector(source, symbol, production);
  if (!maybeBody) return [];
  const open = rustCode(source).indexOf("{", maybeBody.start);
  const spans = [];
  for (const match of maybeBody.body.matchAll(pattern)) {
    let depth = 1;
    let end = match.index + match[0].length;
    const opening = match[0].endsWith("[") ? "[" : "(";
    const closing = opening === "[" ? "]" : ")";
    while (end < maybeBody.body.length && depth) {
      if (maybeBody.body[end] === opening) depth++;
      if (maybeBody.body[end] === closing) depth--;
      end++;
    }
    const start = open + 1 + match.index;
    spans.push({ raw: source.slice(start, open + 1 + end), start, end: open + 1 + end });
  }
  return spans;
}

// A bounded one-version-per-file cache avoids reparsing unrelated fixed roots in isolated mutations.
const checked = new Map<string, { source: string; failures: string[] }>();

/** One validated original Unix registration earns only an equal-offset helper parsing view. */
export function processOwnershipView(source: string): string {
  const code = rustCode(source);
  const modules = [...code.matchAll(/\bmod\s+tests\s*\{/g)];
  if (modules.length !== 1) throw new Error("unique original Unix tests registration required");
  const declaration = modules[0].index;
  const prefix = code.slice(0, declaration);
  const depth = [...prefix].reduce((value, character) => value + Number(character === "{") - Number(character === "}"), 0);
  const maybeAttributes = prefix.match(/(?:#\[[^\]]*\]\s*)+$/);
  if (depth !== 0 || !maybeAttributes || maybeAttributes[0].trim() !== UNIX_TESTS
    || source.slice(maybeAttributes.index, declaration).trim() !== UNIX_TESTS
    || [...code.matchAll(/#\[cfg\(all\(test,\s*unix\)\)\]/g)].length !== 1) throw new Error("exact top-level original Unix tests registration required");
  const start = maybeAttributes.index!;
  const replacement = "#[cfg(test)]".padEnd(UNIX_TESTS.length);
  return source.slice(0, start) + replacement + source.slice(start + UNIX_TESTS.length);
}

/** Both Rust ignore forms are excluded from default executable test evidence. */
export function defaultRustTestAttributes(attributes: string): boolean {
  return /#\[(?:tokio::)?test(?:\]|\()/.test(attributes) && !/#\[\s*ignore(?:\s*[=(]|\s*\])/.test(attributes);
}

/** Bounded TS lexer: templates (including nested interpolations) are never declaration evidence. */
export function maskTypeScript(source: string): string {
  const chars = source.split("");
  const blank = (start: number, end: number) => {
    for (let index = start; index < end; index++) if (chars[index] !== "\n") chars[index] = " ";
  };
  function quoted(start: number): number {
    const quote = source[start];
    for (let index = start + 1; index < source.length; index++) {
      if (source[index] === "\\") { index++; continue; }
      if (source[index] === quote) return index + 1;
    }
    throw new Error("unterminated TS string");
  }
  function template(start: number): number {
    for (let index = start + 1; index < source.length; index++) {
      if (source[index] === "\\") { index++; continue; }
      if (source[index] === "`") return index + 1;
      if (source.startsWith("${", index)) index = interpolation(index + 2) - 1;
    }
    throw new Error("unterminated TS template");
  }
  function regex(start: number): number {
    const before = source.slice(0, start).trimEnd();
    if (!/(?:^|[=(:,!&|?{;]|\breturn|\bthrow)$/.test(before)) throw new Error(`unsupported ambiguous TS slash at ${start}`);
    let characterClass = false;
    for (let index = start + 1; index < source.length && source[index] !== "\n"; index++) {
      if (source[index] === "\\") { index++; continue; }
      if (source[index] === "[") characterClass = true;
      if (source[index] === "]") characterClass = false;
      if (source[index] === "/" && !characterClass) {
        while (/[a-z]/i.test(source[index + 1] ?? "")) index++;
        return index + 1;
      }
    }
    throw new Error("unterminated TS regular expression");
  }
  function unit(index: number): number {
    if (source.startsWith("//", index) || index === 0 && source.startsWith("#!")) {
      const end = source.indexOf("\n", index);
      return end < 0 ? source.length : end;
    }
    if (source.startsWith("/*", index)) {
      const end = source.indexOf("*/", index + 2);
      if (end < 0) throw new Error("unterminated TS comment");
      return end + 2;
    }
    if (source[index] === '"' || source[index] === "'") return quoted(index);
    if (source[index] === "`") return template(index);
    if (source[index] === "/") return regex(index);
    return index;
  }
  function interpolation(start: number): number {
    let depth = 1;
    for (let index = start; index < source.length; index++) {
      const end = unit(index);
      if (end > index) { index = end - 1; continue; }
      if (source[index] === "{") depth++;
      if (source[index] === "}" && --depth === 0) return index + 1;
    }
    throw new Error("unterminated TS interpolation");
  }
  for (let index = 0; index < source.length;) {
    const end = unit(index);
    if (end === index) { index++; continue; }
    blank(index, end);
    index = end;
  }
  return chars.join("");
}

/** Extract one actual top-level declaration, preserving original raw offsets; no general AST claim. */
export function typeScriptFunction(source: string, symbol: string): { start: number; open: number; end: number; body: string; signature: string } {
  new Bun.Transpiler({ loader: "ts" }).transformSync(source);
  const code = maskTypeScript(source);
  const declarations = [...code.matchAll(new RegExp(`\\bfunction\\s+${symbol}\\b`, "g"))];
  if (declarations.length !== 1) throw new Error("unique TS declaration required");
  const start = declarations[0].index;
  const line = code.slice(code.lastIndexOf("\n", start) + 1, start);
  const depth = [...code.slice(0, start)].reduce((value, character) => value + Number(character === "{") - Number(character === "}"), 0);
  if (depth !== 0 || !/^\s*(?:export\s+)?$/.test(line)) throw new Error("top-level TS function owner required");
  function closing(open: number, first: string, last: string): number {
    let depth = 1;
    for (let index = open + 1; index < code.length; index++) {
      if (code[index] === first) depth++;
      if (code[index] === last && --depth === 0) return index;
    }
    throw new Error("unterminated TS declaration");
  }
  const parameters = code.indexOf("(", start);
  if (parameters < 0 || !/^function\s+\w+\s*$/.test(code.slice(start, parameters))) throw new Error("unsupported TS signature");
  const endParameters = closing(parameters, "(", ")");
  const open = code.indexOf("{", endParameters);
  if (open < 0 || !/^\s*(?::[\w\s<>\[\],.]+)?\s*$/.test(code.slice(endParameters + 1, open))) throw new Error("unsupported TS return annotation");
  const end = closing(open, "{", "}") + 1;
  return { start, open, end, body: code.slice(open + 1, end - 1), signature: source.slice(start, open) };
}

/** Raw caller arguments are recovered only from verified executable offsets. */
export function typeScriptCallSpans(source: string, symbol: string): { raw: string; start: number; end: number }[] {
  const fn = typeScriptFunction(source, symbol);
  const spans = [];
  const calls = /\b(?:keys|runner|parsed|validateNode|validateCase|validateResult|rawExpectations|wireExpectations|validateRecipe|JSON\.stringify|Buffer\.byteLength|recipe\.blocks\.slice|cases\.map|ancestry\.filter|wire\.push|rows\.push|createHash|digest)\s*\(/g;
  for (const match of fn.body.matchAll(calls)) {
    let depth = 1;
    let end = match.index + match[0].length;
    while (end < fn.body.length && depth) {
      if (fn.body[end] === "(") depth++;
      if (fn.body[end] === ")") depth--;
      end++;
    }
    if (depth) throw new Error("unterminated TS call");
    const start = fn.open + 1 + match.index;
    spans.push({ raw: source.slice(start, fn.open + 1 + end), start, end: fn.open + 1 + end });
  }
  return spans;
}

function checkFunctions(file: string, source: string): string[] {
  const maybeCached = checked.get(file);
  if (maybeCached?.source === source) return maybeCached.failures;
  const failures: string[] = [];
  let view = source;
  if (file === PROCESS) {
    try { view = processOwnershipView(source); }
    catch (error) {
      const failure = `${file}: Unix ownership/registration: ${error instanceof Error ? error.message : String(error)}`;
      checked.set(file, { source, failures: [failure] });
      return [failure];
    }
    const maybeChild = maybeSelector(source, "phase162_process_child");
    if (!maybeChild || !/#\[ignore\s*=/.test(maybeChild.attributes)
      || CONTRACTS.some(row => row.file === PROCESS && row.symbol === "phase162_process_child" && row.kind === "test")) failures.push(`${file}: ignored child ownership must remain a separate supervised control`);
  } else if (rustCode(source).includes(UNIX_TESTS)) failures.push(`${file}: Unix ownership/registration belongs only to shared process fixture`);
  const allowance = "#[cfg_attr(not(test), allow(dead_code))]";
  const ownership = view.replaceAll(allowance, " ".repeat(allowance.length));
  for (const row of [...CONTRACTS, ...CHECKPOINTS].filter(row => row.file === file)) {
    const production = row.kind === "production";
    const maybeBody = maybeSelector(source, row.symbol, production);
    if (!maybeBody || !supportedOwnership(ownership, maybeBody.start, !production)
      || !hasOrderedCode(maybeBody.body, row.anchors)
      || row.anchors.some((anchor, index) => compact(maybeBody.body).split(compact(anchor)).length - 1 !== row.counts[index])
      || /^\s*(?:return\b|if\s+(?:false|true)\b)/.test(maybeBody.body)
      || /\b(?:if|while)\s+false\b/.test(maybeBody.body)
      || [...maybeBody.body.matchAll(/#\[[^\]]+\]/g)].some(match => compact(match[0]) !== "#[cfg(test)]")) failures.push(`${file}:${row.symbol}: actual ordered function edge required`);
    if (row.kind === "test" && (!maybeBody || !defaultRustTestAttributes(maybeBody.attributes)
      || !meaningfulAssertions(view, maybeBody.body))) failures.push(`${file}:${row.symbol}: assertive registered test required`);
    if (maybeBody && row.symbol === "persist" && file === N + "chainstate.rs"
      && /\b(?:save_snapshot|persist_progress)\s*\(/.test(maybeBody.body)) failures.push(`${file}:${row.symbol}: no snapshot side effect allowed`);
    if (maybeBody && row.symbol.includes("with_header_checkpoint")) {
      const signature = rustCode(source).slice(maybeBody.start, rustCode(source).indexOf("{", maybeBody.start));
      if (!compact(signature).includes("FnMut(&S)->Result<(),StorageError>")) failures.push(`${file}:${row.symbol}: typed fallible checkpoint required`);
    }
    if (maybeBody && /coinbase_maturity\s*=(?!=)/.test(maybeBody.body)) failures.push(`${file}:${row.symbol}: normal maturity required`);
    if (maybeBody && row.symbol === "save_trusted_header_snapshot"
      && /\b(?:ScriptsValid|publish_validation_history|publish_admitted_validation_headers)\b/.test(maybeBody.body)) failures.push(`${file}:${row.symbol}: header snapshot cannot grant validation authority`);
    if (maybeBody && ["accepted", "fixture", "seed_count", "at"].includes(row.symbol)
      && /phase162/.test(file) && /\b(?:MemoryCoinsView|MemoryChainstateStore)\b/.test(maybeBody.body)) failures.push(`${file}:${row.symbol}: genuine Fjall accepted history required`);
    if (maybeBody && /(?:peer_filter_transport|peer_filters)\/phase162\.rs$/.test(file)
      && /\b(?:acknowledge_write|complete_basic_filter_frame)\s*\(/.test(maybeBody.body)) failures.push(`${file}:${row.symbol}: production executor alone earns CF write receipts`);
    const actual = assertionSpans(source, row.symbol, production).map(span => compact(span.raw));
    let cursor = 0;
    for (const assertion of row.assertions) {
      const next = actual.indexOf(compact(assertion), cursor);
      if (next < 0) { failures.push(`${file}:${row.symbol}: exact executable assertion required`); break; }
      cursor = next + 1;
    }
    const facts = factSpans(source, row.symbol, production).map(span => compact(span.raw));
    if (row.facts.some((fact, index) => facts[index] !== compact(fact))) failures.push(`${file}:${row.symbol}: exact executable protocol/matrix fact required`);
  }
  checked.set(file, { source, failures });
  return failures;
}

function checkModules(text: (file: string) => string, failures: string[]): void {
  for (const { parent, child, symbol } of MODULE_EDGES) {
    const source = text(parent);
    const matches = [...rustCode(source).matchAll(new RegExp(`\\bmod\\s+${symbol}\\s*;`, "g"))];
    if (matches.length !== 1 || !supportedOwnership(source, matches[0].index, true)) {
      failures.push(`${parent}: executable registration ${symbol}`);
      continue;
    }
    const before = source.slice(0, matches[0].index);
    const maybePath = before.match(/#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s*)?$/)?.[1];
    const actual = maybePath ? posix.normalize(posix.join(dirname(parent), maybePath))
      : posix.join(["lib.rs", "main.rs"].includes(posix.basename(parent)) ? dirname(parent) : parent.slice(0, -3), symbol + ".rs");
    if (actual !== child) failures.push(`${parent}: actual module path ${symbol}`);
  }
}

function checkNative(source: string, failures: string[]): void {
  const sectionsOf = (input: string) => [input.split(": <<'VERIFY_COMMAND_ORDER'")[1]?.split("VERIFY_COMMAND_ORDER")[0] ?? "",
    input.split("\n").map(line => line.trimStart()).filter(line => line.startsWith("run_step ")).map(line => line.replace(/^run_step "[^"]+" /, "")).join("\n")];
  const sections = sectionsOf(source);
  let historicalSections: string[] = [];
  try { historicalSections = sectionsOf(verificationHistoricalBase(source)); }
  catch (error) { failures.push(`scripts/verify.sh: current registration/projection: ${error instanceof Error ? error.message : String(error)}`); }
  for (const [sectionIndex, section] of sections.entries()) {
    const lines = section.split("\n");
    const positions = NATIVE_COMMANDS.map(command => lines.indexOf(command));
    if (NATIVE_COMMANDS.some(command => lines.filter(line => line === command).length !== 1)
      || positions.some((position, index) => index > 0 && position !== positions[index - 1] + 1)) failures.push("scripts/verify.sh: exact once ordered Phase161/162 native commands");
    if (lines.filter(line => line === SHARED_FORMAT).length !== 1) failures.push("scripts/verify.sh: pinned shared fixture format required");
    const historical = historicalSections[sectionIndex]?.split("\n").filter(line => line && !NATIVE_COMMANDS.slice(1).includes(line) && line !== SHARED_FORMAT);
    if (historical && JSON.stringify(historical) !== JSON.stringify(HISTORICAL_COMMANDS[sectionIndex])) failures.push("scripts/verify.sh: complete historical/native command order required");
  }
  for (const command of ["cargo fmt --manifest-path packages/Cargo.toml --all --check", "cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings",
    "cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features", "cargo test --manifest-path packages/Cargo.toml --workspace --all-features",
    "cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --all-features", "bazel build //:core //:node //:rpc //:cli //:test_harness //:bench",
    "run_benchmark_list", "run_coverage_report"]) if (!sections[1].split("\n").includes(command)) failures.push(`scripts/verify.sh: original mandatory gate ${command}`);
}

/** Supplied source maps are hermetic: missing keys never fall back to the filesystem. */
export function checkPhase162IntegratedFilterProof(maybeSources?: ReadonlyMap<string, string>): string[] {
  const sources = maybeSources ?? loadSources();
  const text = sourceText(sources);
  const failures: string[] = [];
  for (const file of CHECK_FILES) if (!sources.has(file)) failures.push(`${file}: required current source missing`);
  const historicalText = sourceText(historicalFixtureView(sources));
  const state = checkPhase162ClaimState(historicalText, failures);
  try { failures.push(...checkPhase162LegacyFunctionControls(legacyFunctionContractView(sources))); }
  catch (error) { failures.push(error instanceof Error ? error.message : String(error)); }
  checkModules(text, failures);
  checkNative(text("scripts/verify.sh"), failures);
  checkScope(text, failures, state, historicalText);
  checkFrozenGuards(text, failures);
  checkTypeScriptOracle(text("scripts/phase162-filter-oracle.ts"), failures);
  if (!compact(rustCode(text(N + "sync/tests/peer_filter_transport/phase162.rs"))).includes("constPARTIAL:usize=31;")) failures.push("Phase162 outbound physical prefix: exact31-byte constant required");
  return [...new Set(failures)];
}

/** Synthetic legacy mutation controls only; no claims, receipts, CLI mode or current authority. */
export function checkPhase162LegacyFunctionControls(sources: ReadonlyMap<string, string>): string[] {
  return [...new Set(ROOTS)].flatMap(file => checkFunctions(file, sources.get(file) ?? ""));
}

function checkTypeScriptOracle(source: string, failures: string[]): void {
  for (const row of TS_CONTRACTS) {
    const fail = () => failures.push(`scripts/phase162-filter-oracle.ts:${row.symbol}: actual raw-oracle caller/argument contract`);
    try {
      const fn = typeScriptFunction(source, row.symbol);
      const actual = typeScriptCallSpans(source, row.symbol).map(span => compact(span.raw));
      if (compact(fn.signature) !== row.signature || !hasOrderedCode(fn.body, row.anchors)
        || row.anchors.some((anchor, index) => compact(fn.body).split(compact(anchor)).length - 1 !== row.counts[index])
        || JSON.stringify(actual) !== JSON.stringify(row.calls)
        || /\b(?:if|while)\s*\(\s*false\s*\)/.test(fn.body)) fail();
    } catch { fail(); }
  }
}

function checkScope(text: (file: string) => string, failures: string[], state: ReturnType<typeof checkPhase162ClaimState>, historicalText: (file: string) => string): void {
  try {
    const index = JSON.parse(text("docs/parity/index.json"));
    const owner = index.checklist.surfaces.find((row: { id: string }) => row.id === "v2-5-integrated-retained-filter-proof");
    for (const file of [...OWNED, ...PHASE162_EVIDENCE]) if (!owner?.evidence.includes(file)) failures.push(`docs/parity/index.json: registered current evidence ${file}`);
    const pinSource = text("scripts/generate-peer-filter-fixtures.ts");
    const pins = [...rustCode(pinSource).matchAll(/\bexport\s+const\s+PIN\s*=/g)];
    if (pins.length !== 1 || !pinSource.slice(pins[0].index).startsWith(`export const PIN = "${PIN}"`)) failures.push("Phase162: exact independent oracle Knots pin required");
    checkPhase162ExcludedClaims(JSON.stringify(owner), "docs/parity/index.json", failures, state, historicalText);
    const view = phase162ClaimView(text(PHASE162_DOC), state, historicalText);
    checkPhase162ExcludedClaims(view, PHASE162_DOC, failures, state, historicalText);
    const breadcrumbs = JSON.parse(text("docs/parity/source-breadcrumbs.json"));
    for (const [file, anchors] of Object.entries(BREADCRUMBS)) {
      const groups = breadcrumbs.groups.filter((group: { files: string[] }) => group.files.includes(file));
      if (groups.length !== 1 || JSON.stringify(groups[0].breadcrumbs) !== JSON.stringify(anchors)
        || !anchors.every(anchor => text(file).split("\n").slice(0, 12).includes(`// - ${anchor}`))) failures.push(`${file}: exact registered source breadcrumb required`);
    }
  } catch { failures.push("Phase162: valid current parity/breadcrumb inventory required"); }
}

function checkFrozenGuards(text: (file: string) => string, failures: string[]): void {
  // Phase163's exact scoped replacement and preserved checkpoint contracts require independent review.
  for (const [file, hash] of [["scripts/check-phase158-validated-reorg.ts", "385b6c6d62067ba13e8e4d29d0ea1005252614830430f800b0824ec7ba954fdd"],
    ["scripts/check-phase158-validated-reorg.test.ts", "bebdd0c7f2cc6e787f34502bb82f6ae6fd27c7a5ce5ac9bdd14be73c054193a3"],
    ["scripts/check-phase163-provenance.ts", "e42bd6a3f1660d46ac35c2bb98fe4df44766ac84ee80d7ffae2a766c22dbaa03"]]) {
    const fingerprintSource = file === "scripts/check-phase163-provenance.ts" ? text(file) : legacyToolingBase(file, text(file));
    if (createHash("sha256").update(fingerprintSource).digest("hex") !== hash) failures.push(`${file}: frozen Plan15 checkpoint/lineage/IfNeeded/no-snapshot guards changed`);
  }
}

if (import.meta.main) {
  try {
    const failures = checkPhase162IntegratedFilterProof();
    if (failures.length) {
      console.error("Phase162 integrated structural/claim guard failed:\n" + failures.map(failure => `- ${failure}`).join("\n"));
      process.exit(1);
    }
    const attribution = fixtureSourceAttribution(loadSources().get(MIGRATED_FIXTURE)!);
    console.log(`Phase162 fixture attribution: ${attribution.evidenceClass}; raw current SHA=${attribution.currentHash}; original historical SHA=${attribution.historicalHash}; no current ordinary qualification.`);
    console.log(`Phase162: ${CONTRACTS.length} fixed function contracts; Pending/implementation/formal claim consistency; executable behavior and full native/review gates remain independent.`);
  } catch (error) {
    console.error(`Phase162 cannot read required current evidence: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
