#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { closeSync, constants, fstatSync, lstatSync, openSync, readSync } from "node:fs";
import { resolve } from "node:path";
import { EDGES, SOURCE_FILES, TEST_MODULES, PHASE162_SOURCE_INVERSE, FIXTURE_ADAPTER_INVERSE, HISTORICAL_TOOLING_INVERSE, FILTER_GUARD_INVERSES, COPIED_CLI_INVERSES, maybeRestoreReviewedSource } from "./check-phase164-canonical-identity/contracts.ts";
import { regions, SENSITIVE_CALLS } from "./check-phase164-canonical-identity/regions.ts";
import { compact, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { supportedOwnership } from "./check-phase159-filter-rpcs/evidence.ts";
import { CLAIM_FILES, checkCurrentClaims } from "./check-phase164-canonical-identity/claims.ts";
import { CURRENT_TOOLING_FILES, HISTORICAL_TOOLING_FILES, MANIFEST_FILE, historicalToolingView, CURRENT_VERIFY_ADDITIONS, verificationHistoricalBase } from "./check-phase164-canonical-identity/historical-tooling.ts";
import { fixtureSourceAttribution, MIGRATED_FIXTURE, LEGACY_CONTRACT_BUNDLE } from "./check-phase164-canonical-identity/fixture-source.ts";

export type Finding = { file: string; rule: string; detail: string };
export const NATIVE_COMMAND = "bun run scripts/check-phase164-canonical-identity.ts";
const GENESIS = "packages/open-bitcoin-consensus/src/chain_spec/genesis.rs";
const SPEC = "packages/open-bitcoin-consensus/src/chain_spec.rs";
export const INPUT_FILES = [...new Set([...SOURCE_FILES, ...CLAIM_FILES, "scripts/verify.sh", ...CURRENT_TOOLING_FILES, ...HISTORICAL_TOOLING_FILES, LEGACY_CONTRACT_BUNDLE])];
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
  const descriptor = openSync(file, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    if (!fstatSync(descriptor).isFile()) throw new Error(`${file}: regular input required`);
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
  let maybeHistoricalAdapter: string | undefined;
  try { maybeHistoricalAdapter = historicalFixtureAdapter(inputs); }
  catch (error) { fail("scripts/check-phase164-canonical-identity/fixture-source.ts", "history", error instanceof Error ? error.message : String(error)); }
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
  checkWorkPreservationRegistration(text, fail);
  checkAdapterWorkRegistration(text, fail);
  if (maybeHistoricalAdapter !== undefined) checkCurrentClaims(text, fail);
  if (maybeHistoricalAdapter === undefined || fixtureSourceAttribution(text(MIGRATED_FIXTURE)).evidenceClass !== "current-explicit-fixture") fail(MIGRATED_FIXTURE, "history", "reviewed current fixture wiring required; no ordinary qualification");
  const sha = (source: string) => createHash("sha256").update(source).digest("hex");
  if (sha(text(LEGACY_CONTRACT_BUNDLE)) !== "1330515f81cb39108b7605814f50ed4c1f1572a0280b8415902085791f9eab5c") fail(LEGACY_CONTRACT_BUNDLE, "bridge-data", "reviewed current legacy-contract bridge data changed; original captures are separate");
  if (sha(text(MANIFEST_FILE)) !== "9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0") fail(MANIFEST_FILE, "history", "reviewed historical manifest required");
  for (const [file, hash] of [["scripts/check-phase164-canonical-identity/historical-tooling.ts","47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65"],["scripts/check-phase164-canonical-identity/fixture-source.ts","b92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3"]]) {
    const historical = file === "scripts/check-phase164-canonical-identity/fixture-source.ts" ? maybeHistoricalAdapter
      : maybeRestoreReviewedSource(text(file), HISTORICAL_TOOLING_INVERSE);
    if (sha(historical ?? "") !== hash) fail(file, "history", "reviewed historical adapter required");
  }
  if (maybeHistoricalAdapter !== undefined) {
    try { historicalToolingView(inputs); }
    catch (error) { fail(MANIFEST_FILE, "history", error instanceof Error ? error.message : String(error)); }
  }
  checkRegistration(text("scripts/verify.sh"), fail);
  return findings;
}

/** Bind the PASSED raw generations before any imported fixture classifier counts. */
export function historicalFixtureAdapter(inputs: ReadonlyMap<string, string>): string {
  const sha = (value: string) => createHash("sha256").update(value).digest("hex");
  const tooling = inputs.get("scripts/check-phase164-canonical-identity/historical-tooling.ts") ?? "";
  const maybeHistoricalTooling = maybeRestoreReviewedSource(tooling, HISTORICAL_TOOLING_INVERSE);
  if (maybeHistoricalTooling === undefined || sha(maybeHistoricalTooling) !== "47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65") {
    throw new Error("paired current raw historical-tooling generation and complete exact inverse required");
  }
  for (const [file, inverse] of FILTER_GUARD_INVERSES) {
    const maybeGuard = inputs.get(file);
    if (maybeGuard === undefined && file !== "scripts/check-phase159-filter-rpcs.test.ts") continue;
    if (maybeGuard === undefined || sha(maybeGuard) !== inverse.currentRawSha
      || maybeRestoreReviewedSource(maybeGuard, inverse) === undefined) {
      throw new Error(`${file}: reviewed current filter guard generation and complete exact inverse required`);
    }
  }
  for (const [file, inverse] of COPIED_CLI_INVERSES) {
    const maybeCopier = inputs.get(file);
    if (maybeCopier === undefined || sha(maybeCopier) !== inverse.currentRawSha
      || maybeRestoreReviewedSource(maybeCopier, inverse) === undefined) {
      throw new Error(`${file}: reviewed current copied CLI generation and complete exact inverse required`);
    }
  }
  const source = inputs.get(MIGRATED_FIXTURE) ?? "";
  const adapter = inputs.get("scripts/check-phase164-canonical-identity/fixture-source.ts") ?? "";
  const maybeSource = maybeRestoreReviewedSource(source, PHASE162_SOURCE_INVERSE);
  const maybeAdapter = maybeRestoreReviewedSource(adapter, FIXTURE_ADAPTER_INVERSE);
  if (maybeSource === undefined || sha(maybeSource) !== "2d0a7e7d93034333e00f5c90977693bcfb7a03ff8ab704bf834d91fec680d307"
    || maybeAdapter === undefined || sha(maybeAdapter) !== "b92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3") {
    throw new Error("paired current raw fixture/adapter generation and complete exact inverses required");
  }
  return maybeAdapter;
}

export { maybeRestoreReviewedSource, PHASE162_SOURCE_INVERSE, FIXTURE_ADAPTER_INVERSE, HISTORICAL_TOOLING_INVERSE };

/** Exact literal-bearing bytes; even formatting changes require reviewed contract renewal. */
export function sourceDigest(source: string): string {
  return createHash("sha256").update(source).digest("hex");
}

export function reviewedOwnership(file: string, source: string, start: number, symbol: string): boolean {
  const lease = "packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs";
  const prefix = rustCode(source).slice(0, start);
  if (file === lease && symbol === "open_without_ensure_schema_for_test") {
    // regions() retains its mechanical non-test bucket for cfg(test) impls.
    // This one relocated helper must still compile only in the exact test impl.
    const maybeOwner = prefix.match(/((?:#\[[^\]]*\]\s*)+)impl\s+super::FjallNodeStore\s*\{\s*pub\(crate\)\s*$/);
    if (!maybeOwner || compact(maybeOwner[1]) !== "#[cfg(test)]") return false;
    return supportedOwnership(source, start, true);
  }
  if (file === lease && symbol === "acquire") {
    const candidates = regions(source).filter(row => row.symbol === "acquire");
    const maybeAttributes = prefix.match(/((?:#\[[^\]]*\]\s*)+)pub\(super\)\s*$/);
    if (candidates.length !== 2 || candidates[1].start !== start
      || !maybeAttributes || compact(maybeAttributes[1]) !== "#[cfg(unix)]") return false;
    const index = prefix.lastIndexOf("#[cfg(unix)]");
    const view = source.slice(0, index) + " ".repeat("#[cfg(unix)]".length) + source.slice(index + "#[cfg(unix)]".length);
    return supportedOwnership(view, start, true);
  }
  if (supportedOwnership(source, start, true)) return true;
  // Two existing Unix-only physical symlink tests, under separately checked cfg(test) roots.
  const unixTests = [
    ["packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs", "chain_compatibility_preserves_unsafe_paths_and_their_closed_store_targets"],
    ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs", "canonical_chain_daemon_unsafe_alias_and_non_directory_preserve_physical_sources"],
  ];
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

function checkWorkPreservationRegistration(text: (file: string) => string, fail: (file: string, rule: string, detail: string) => void): void {
  const opener = "packages/open-bitcoin-node/src/storage/chain_compatibility.rs";
  const fixture = "packages/open-bitcoin-node/src/storage/chain_compatibility/tests/phase165_work.rs";
  const code = rustCode(text(opener));
  const declarations = [...code.matchAll(/\bmod\s+phase165_work\s*;/g)];
  const maybeDeclaration = declarations.length === 1 ? declarations[0] : undefined;
  const prefix = maybeDeclaration ? code.slice(0, maybeDeclaration.index) : "";
  const maybeAttributes = prefix.match(/(?:#\s*\[[^\]]*\]\s*)+$/);
  const depth = [...prefix].reduce((value, char) => value + Number(char === "{") - Number(char === "}"), 0);
  if (!maybeDeclaration || !maybeAttributes || depth !== 0
    || !/^#\[cfg\(test\)\]\s*#\[path\s*=\s*"chain_compatibility\/tests\/phase165_work.rs"\]\s*$/.test(text(opener).slice(maybeAttributes.index, maybeDeclaration.index))) {
    fail(opener, "ownership", "exact executable cfg-test work preservation path required");
  }
  const fixtureCode = rustCode(text(fixture));
  const includes = [...fixtureCode.matchAll(/\binclude\s*!/g)];
  const maybeInclude = includes.length === 1 ? includes[0] : undefined;
  if (!maybeInclude || fixtureCode.slice(0, maybeInclude.index).trim() !== ""
    || !/^include!\(\s*"preservation.rs"\s*\);/.test(text(fixture).slice(maybeInclude.index))) {
    fail(fixture, "ownership", "exact executable original preservation include required");
  }
}

function checkAdapterWorkRegistration(text: (file: string) => string, fail: (file: string, rule: string, detail: string) => void): void {
  for (const [file, expected] of [
    ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs", '#[path = "tests/phase165_work.rs"]\nmod phase165_work;'],
    ["packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs", 'pub(crate) mod phase165_work;'],
  ]) {
    const source = text(file);
    const code = rustCode(source);
    const declarations = [...code.matchAll(/(?:pub\(crate\)\s+)?\bmod\s+phase165_work\s*;/g)];
    const maybeDeclaration = declarations.length === 1 ? declarations[0] : undefined;
    const prefix = maybeDeclaration ? code.slice(0, maybeDeclaration.index) : "";
    const maybeAttributes = prefix.match(/(?:#\s*\[[^\]]*\]\s*)+$/);
    const start = maybeAttributes?.index ?? maybeDeclaration?.index ?? 0;
    const end = maybeDeclaration ? maybeDeclaration.index + maybeDeclaration[0].length : 0;
    const depth = [...prefix].reduce((value, char) => value + Number(char === "{") - Number(char === "}"), 0);
    if (!maybeDeclaration || depth !== 0 || source.slice(start, end) !== expected) {
      fail(file, "ownership", "exact executable adapter work-test registration required");
    }
  }
  for (const file of [
    "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/phase165_work.rs",
    "packages/open-bitcoin-rpc/tests/phase165_work_startup.rs",
    "packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests/phase165_work.rs",
    "packages/open-bitcoin-cli/src/operator/wallet/tests.rs",
    "packages/open-bitcoin-cli/src/operator/soak/tests/recovery_resume_bounds.rs",
  ]) {
    const source = text(file);
    const code = rustCode(source);
    const functions = regions(source);
    if (/#\s*!\s*\[/.test(code)) {
      fail(file, "ownership", "unreviewed inner attributes cannot disable adapter evidence files");
    }
    for (const [, symbol, ordinal] of EDGES.filter(row => row[0] === file && row[1].startsWith("phase165_"))) {
      const maybeFunction = functions.filter(row => row.symbol === symbol)[ordinal];
      const prefix = code.slice(0, maybeFunction?.start ?? 0);
      const maybeAttributes = prefix.match(/(?:#\s*\[[^\]]*\]\s*)+$/);
      if (!maybeFunction || !maybeAttributes || compact(source.slice(maybeAttributes.index, maybeFunction.start)) !== "#[test]") {
        fail(file, "ownership", `${symbol}: exact executable test attribute required`);
      }
    }
  }
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
  const lines = source.split("\n");
  try {
    if (CURRENT_VERIFY_ADDITIONS.some(line => lines.filter(actual => actual === line).length !== 1)) {
      fail("scripts/verify.sh", "registration", "mandatory current Phase68/165 executable and declared registrations required");
    }
    verificationHistoricalBase(source);
  } catch (error) {
    fail("scripts/verify.sh", "registration", error instanceof Error ? error.message : String(error));
  }
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
