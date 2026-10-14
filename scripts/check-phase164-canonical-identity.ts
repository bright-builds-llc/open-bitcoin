#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { closeSync, lstatSync, openSync, readSync } from "node:fs";
import { resolve } from "node:path";
import { EDGES, SOURCE_FILES, TEST_MODULES } from "./check-phase164-canonical-identity/contracts.ts";
import { regions, SENSITIVE_CALLS } from "./check-phase164-canonical-identity/regions.ts";
import { compact, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { supportedOwnership } from "./check-phase159-filter-rpcs/evidence.ts";
import { CLAIM_FILES, checkCurrentClaims } from "./check-phase164-canonical-identity/claims.ts";
import { CURRENT_TOOLING_FILES, HISTORICAL_TOOLING_FILES, MANIFEST_FILE, historicalToolingView } from "./check-phase164-canonical-identity/historical-tooling.ts";
import { fixtureSourceAttribution, MIGRATED_FIXTURE } from "./check-phase164-canonical-identity/fixture-source.ts";

export type Finding = { file: string; rule: string; detail: string };
export const NATIVE_COMMAND = "bun run scripts/check-phase164-canonical-identity.ts";
const GENESIS = "packages/open-bitcoin-consensus/src/chain_spec/genesis.rs";
const SPEC = "packages/open-bitcoin-consensus/src/chain_spec.rs";
export const INPUT_FILES = [...new Set([...SOURCE_FILES, ...CLAIM_FILES, "scripts/verify.sh", ...CURRENT_TOOLING_FILES, ...HISTORICAL_TOOLING_FILES])];
const MAX_FILE_BYTES = 512_000;

/** Thin read-only shell: fixed local inputs, bounded bytes, no archive walk or network. */
export function loadInputs(maybeRoot?: string): Map<string, string> {
  const root = maybeRoot ?? resolve(import.meta.dir, "..");
  return new Map(INPUT_FILES.map(file => {
    return [file, readBounded(resolve(root, file))];
  }));
}

function readBounded(file: string): string {
  if (!lstatSync(file).isFile()) throw new Error(`${file}: regular input required`);
  const descriptor = openSync(file, "r");
  try {
    const bytes = Buffer.alloc(MAX_FILE_BYTES + 1);
    let length = 0;
    while (length < bytes.length) {
      const count = readSync(descriptor, bytes, length, bytes.length - length, null);
      if (count === 0) break;
      length += count;
    }
    if (length > MAX_FILE_BYTES) throw new Error(`${file}: input exceeds fixed byte limit`);
    return bytes.subarray(0, length).toString("utf8");
  } finally { closeSync(descriptor); }
}

/** Conservative current-source contract only. Executed Rust preservation evidence is independent. */
export function checkPhase164(inputs: ReadonlyMap<string, string>): readonly Finding[] {
  const findings: Finding[] = [];
  const fail = (file: string, rule: string, detail: string) => findings.push({ file, rule, detail });
  for (const file of INPUT_FILES) {
    const maybeSource = inputs.get(file);
    if (!maybeSource || Buffer.byteLength(maybeSource) > MAX_FILE_BYTES) fail(file, "input", "required bounded input missing or too large");
  }
  const text = (file: string) => inputs.get(file) ?? "";
  for (const file of SOURCE_FILES) {
    try {
      const source = text(file);
      const functions = regions(source);
      const rows = EDGES.filter(row => row[0] === file);
      for (const [, symbol, ordinal, digest, test] of rows) {
        const matches = functions.filter(fn => fn.symbol === symbol);
        const maybeRegion = matches[ordinal];
        if (!maybeRegion || maybeRegion.test !== test || !reviewedOwnership(file, source, maybeRegion.start, symbol) || sourceDigest(maybeRegion.raw) !== digest) {
          fail(file, "boundary", `${symbol}[${ordinal}]: reviewed executable identity/opener/admission/fixture edge changed`);
        }
      }
      for (const call of rustCode(source).matchAll(SENSITIVE_CALLS)) {
        const owners = functions.filter(fn => call.index >= fn.start && call.index < fn.end);
        if (owners.length !== 1 || !rows.some(row => row[1] === owners[0].symbol && row[2] === owners[0].ordinal)) {
          fail(file, "opener", `uninventoried executable opener or constructor at offset ${call.index}`);
        }
      }
    } catch (error) {
      fail(file, "syntax", error instanceof Error ? error.message : String(error));
    }
  }
  for (const [file, expected] of TEST_MODULES) {
    const actual = rustCode(text(file)).match(/#\[cfg\(test\)\]\s*(?:pub(?:\([^)]*\))?\s*)?mod\s+\w+\s*[;{]/g) ?? [];
    if (JSON.stringify(actual.map(compact)) !== JSON.stringify(expected)) fail(file, "ownership", "exact fixture test-module ownership required");
  }
  checkFacts(text, fail);
  checkPreservationRegistration(text("packages/open-bitcoin-node/src/storage/chain_compatibility.rs"), fail);
  checkCurrentClaims(text, fail);
  if (fixtureSourceAttribution(text(MIGRATED_FIXTURE)).evidenceClass !== "current-explicit-fixture") fail(MIGRATED_FIXTURE, "history", "reviewed current fixture wiring required; no ordinary qualification");
  const sha = (source: string) => createHash("sha256").update(source).digest("hex");
  if (sha(text(MANIFEST_FILE)) !== "9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0") fail(MANIFEST_FILE, "history", "reviewed historical manifest required");
  for (const [file, hash] of [["scripts/check-phase164-canonical-identity/historical-tooling.ts","8922542b5721d2cc966312bbee416cf1418ea0151b1eecec01cb34dbc06fa85f"],["scripts/check-phase164-canonical-identity/fixture-source.ts","24224f0dea4e405d3a148a9fe148255884bc7c575092ad8b0c67507d157dbff2"]]) {
    if (sha(text(file)) !== hash) fail(file, "history", "reviewed historical adapter required");
  }
  try { historicalToolingView(inputs); }
  catch (error) { fail(MANIFEST_FILE, "history", error instanceof Error ? error.message : String(error)); }
  checkRegistration(text("scripts/verify.sh"), fail);
  return findings;
}

/** Exact literal-bearing bytes; even formatting changes require reviewed contract renewal. */
export function sourceDigest(source: string): string {
  return createHash("sha256").update(source).digest("hex");
}

function reviewedOwnership(file: string, source: string, start: number, symbol: string): boolean {
  if (supportedOwnership(source, start, true)) return true;
  // Two existing Unix-only physical symlink tests, under separately checked cfg(test) roots.
  const unixTests = [
    ["packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs", "chain_compatibility_preserves_unsafe_paths_and_their_closed_store_targets"],
    ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs", "canonical_chain_daemon_unsafe_alias_and_non_directory_preserve_physical_sources"],
  ];
  const prefix = rustCode(source).slice(0, start);
  if (!unixTests.some(row => row[0] === file && row[1] === symbol) || !compact(prefix).endsWith("#[cfg(unix)]#[test]")) return false;
  const index = prefix.lastIndexOf("#[cfg(unix)]");
  const view = source.slice(0, index) + "#[cfg(test)]" + source.slice(index + "#[cfg(unix)]".length);
  return supportedOwnership(view, start, true);
}

function checkPreservationRegistration(source: string, fail: (file: string, rule: string, detail: string) => void): void {
  const code = rustCode(source);
  const declarations = [...code.matchAll(/\bmod\s+preservation\s*;/g)];
  const file = "packages/open-bitcoin-node/src/storage/chain_compatibility.rs";
  const refused = () => fail(file, "ownership", "exact executable cfg-test preservation path required");
  if (declarations.length !== 1) { refused(); return; }
  const start = declarations[0].index;
  const prefix = code.slice(0, start);
  const depth = [...prefix].reduce((value, char) => value + Number(char === "{") - Number(char === "}"), 0);
  const maybeAttributes = prefix.match(/(?:#\s*\[[^\]]*\]\s*)+$/);
  if (depth !== 0 || !maybeAttributes
    || !/^#\[cfg\(test\)\]\s*#\[path\s*=\s*"chain_compatibility\/tests\/preservation.rs"\]\s*$/.test(source.slice(maybeAttributes.index, start))) refused();
}

function checkFacts(text: (file: string) => string, fail: (file: string, rule: string, detail: string) => void): void {
  const spec = text(SPEC);
  const prerequisites = spec.slice(spec.indexOf("const MISSING_HISTORY"), spec.indexOf("/// Read-only support facts"));
  if (sourceDigest(prerequisites) !== "35e0a42f2b3139c540d733df57c2f7a891f3df4110a1b1ac88e447c20ef01989") fail(SPEC, "scope", "complete immutable prerequisite rows required");
  const constants = text(GENESIS).split("pub(super) fn header")[0];
  if (sourceDigest(constants) !== "37c23358fcba308e080816e02b978cd46219f862ae8f79fba8dccf188037011b") fail(GENESIS, "identity", "exact common merkle, output-script and Signet challenge constants required");
  const genesis = compact(rustCode(text(GENESIS)));
  for (const row of [
    "Mainnet=>(1_231_006_505,2_083_236_893,0x1d00_ffff)",
    "Testnet3=>(1_296_688_602,414_098_458,0x1d00_ffff)",
    "Signet=>(1_598_918_400,52_613_770,0x1e03_77ae)",
    "Regtest=>(1_296_688_602,2,0x207f_ffff)",
  ]) if (!genesis.includes(row)) fail(GENESIS, "identity", `exact pinned vector ${row}`);
  for (const name of ["ChainSpec", "ValidationScope"]) {
    if (!compact(rustCode(text(SPEC))).includes(`pubstruct${name}{chain:CanonicalChain,}`)) fail(SPEC, "scope", "immutable private identity-only support required");
  }
}

function checkRegistration(source: string, fail: (file: string, rule: string, detail: string) => void): void {
  const ordered = source.split(": <<'VERIFY_COMMAND_ORDER'")[1]?.split("VERIFY_COMMAND_ORDER")[0] ?? "";
  const actual = source.split("\n").filter(line => line.startsWith("run_step ")).map(line => line.replace(/^run_step "[^"]+" /, "")).join("\n");
  for (const section of [ordered, actual]) {
    if (section.split("\n").filter(line => line === NATIVE_COMMAND).length !== 1
      || section.indexOf(NATIVE_COMMAND) < section.indexOf("bun test ./scripts/check-phase163-provenance/native.test.ts")) {
      fail("scripts/verify.sh", "registration", "exact once default checker after provenance gates required");
    }
  }
}

if (import.meta.main) {
  try {
    const findings = checkPhase164(loadInputs());
    if (findings.length) {
      console.error("Phase164 current-source identity boundary failed:\n" + findings.map(row => `${row.file}: ${row.rule}: ${row.detail}`).join("\n"));
      process.exit(1);
    }
    console.log(`Phase164: ${SOURCE_FILES.length} fixed Rust inputs; ${EDGES.length} reviewed source edges; identity-only ordinary refusal. No canonical qualification receipt generated.`);
  } catch (error) {
    console.error(`Phase164 required input failure: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
