import { expect, test } from "bun:test";
import { CLI_MODULES, copiedInputs, DOWNSTREAM_SOURCE_MUTATIONS, PEER_SOURCE_MUTATIONS, PUBLIC_REQUALIFICATION_MUTATIONS } from "./check-phase165-shared-exact-work/test-support.ts";
import { mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { assertInputInventory, checkPhase165, INPUT_FILES, loadInputs, MAX_FILE_BYTES, maybeTestDeclaration } from "./check-phase165-shared-exact-work.ts";
import { CORPUS_FILE, HISTORICAL_IDENTITIES, SOURCE_CONTRACTS, TEST_OWNERS, WORK_CLAIM_FILE, FIXTURE_SOURCE_MUTATIONS, LIVE_SOURCE_MUTATIONS, OWNER_SOURCE_MUTATIONS, CONSUMER_SOURCE_MUTATIONS, DAEMON_SOURCE_MUTATIONS, PRUNE_SOURCE_MUTATIONS, PREPARATION_SOURCE_MUTATIONS, LEGACY_BRIDGE_ROWS, LEGACY_BUNDLE_MUTATIONS, TEST_REGISTRATIONS, CLAIM_SCOPE_REFUSALS, CLAIM_SCOPE_DENIALS, BRIDGE_MANDATORY_COMMANDS, TOOLING_INVERSE_CASES, WORK_DOC_OVERCLAIMS } from "./check-phase165-shared-exact-work/contracts.ts";
import { regions } from "./check-phase164-canonical-identity/regions.ts";
import { CURRENT_VERIFY_ADDITIONS, historicalToolingView, legacyToolingBase, verificationHistoricalBase } from "./check-phase164-canonical-identity/historical-tooling.ts";
import { checkPhase164 } from "./check-phase164-canonical-identity.ts";
import { rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { createHash } from "node:crypto";
import { LEGACY_CONTRACT_BUNDLE, legacyFunctionContractView, legacyFunctionSources, historicalFixtureView, MIGRATED_FIXTURE } from "./check-phase164-canonical-identity/fixture-source.ts";

import { loadSources as loadLegacyBridgeInputs, checkPhase162IntegratedFilterProof, checkPhase162LegacyFunctionControls, assertionSpans, factSpans, NATIVE_COMMANDS, SHARED_FORMAT } from "./check-phase162-integrated-filter-proof.ts";
const real = loadInputs();
const bridgeReal = loadLegacyBridgeInputs();
const bridgeSha = (source: string) => createHash("sha256").update(source).digest("hex");
for (const [file, digest] of LEGACY_BRIDGE_ROWS) {
  test(`phase165 C legacy bridge exact authenticated source ${file}`, () => {
    // Arrange / Act: only the explicit legacy structural view sees captured bytes.
    const before = [...bridgeReal];
    const legacy = legacyFunctionContractView(bridgeReal);
    // Assert
    expect(bridgeSha(legacy.get(file)!)).toBe(digest);
    expect([...bridgeReal]).toEqual(before);
    expect([...legacyFunctionSources(bridgeReal)]).toEqual([...legacy]);
    expect([...legacyFunctionContractView(legacy)]).toEqual([...legacy]);
  });
  for (const change of ["missing", "comment", "assertion", "order"]) test(`phase165 C legacy bridge refuses ${file} ${change}`, () => {
    // Arrange
    const changed = new Map(bridgeReal);
    if (change === "missing") changed.delete(file);
    else changed.set(file, change === "comment" ? bridgeReal.get(file)! + "\n// unknown version\n"
      : change === "assertion" ? bridgeReal.get(file)!.replace("assert", "assert_changed") : bridgeReal.get(file)!.split("\n").reverse().join("\n"));
    // Act / Assert: the same owner must refuse, without a raw-source fallback.
    expect(() => legacyFunctionContractView(changed)).toThrow(file + ": legacy function source version");
    expect(checkPhase162IntegratedFilterProof(changed).some(failure => failure.includes(file + ": legacy function source version") || change === "missing" && failure.includes(file + ": required current source missing"))).toBe(true);
  });
}
const BRIDGE_CHILD = "packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs";
for (const [before, after] of [["recipe::Recipe::new(params, ScriptVerifyFlags::P2SH, count)", "recipe::Recipe::new(params, ScriptVerifyFlags::NONE, count)"], ["for block in &recipe.blocks {", "for block in recipe.blocks.iter().skip(1) {"]]) test(`phase165 C legacy bridge refuses changed recipe/loop ${before}`, () => {
  const changed = bridgeReal.get(MIGRATED_FIXTURE)!.replace(before, after);
  expect(changed).not.toBe(bridgeReal.get(MIGRATED_FIXTURE));
  expect(() => legacyFunctionContractView(new Map(bridgeReal).set(MIGRATED_FIXTURE, changed))).toThrow(MIGRATED_FIXTURE + ": legacy function source version");
});
for (const change of ["missing", "alias", "registration", "count"]) test(`phase165 C legacy bridge child binding ${change}`, () => {
  const changed = new Map(bridgeReal);
  if (change === "missing") changed.delete(BRIDGE_CHILD);
  else changed.set(BRIDGE_CHILD, change === "alias" ? bridgeReal.get(BRIDGE_CHILD)!.replace("stale_runtime_configured(false)", "redirected_helper(false)") : change === "count" ? bridgeReal.get(BRIDGE_CHILD)!.replace("#[test]", "") : "#![cfg(any())]\n" + bridgeReal.get(BRIDGE_CHILD)!);
  expect(changed.get(BRIDGE_CHILD)).not.toBe(bridgeReal.get(BRIDGE_CHILD));
  expect(() => legacyFunctionContractView(changed)).toThrow(BRIDGE_CHILD + ": legacy function source version");
});
for (const change of LEGACY_BUNDLE_MUTATIONS) test(`phase165 C legacy bridge bundle ${change}`, () => {
  // Arrange
  const changed = new Map(bridgeReal);
  const bundle = JSON.parse(bridgeReal.get(LEGACY_CONTRACT_BUNDLE)!);
  if (change === "missing") changed.delete(LEGACY_CONTRACT_BUNDLE);
  else if (change === "oversized") changed.set(LEGACY_CONTRACT_BUNDLE, "x".repeat(MAX_FILE_BYTES + 1));
  else if (change === "truncated") changed.set(LEGACY_CONTRACT_BUNDLE, bridgeReal.get(LEGACY_CONTRACT_BUNDLE)!.slice(0, -1));
  else if (change === "malformed") changed.set(LEGACY_CONTRACT_BUNDLE, "{invalid");
  else {
    if (change === "capture") bundle.pre_c_capture.scope += " changed";
    if (change === "base") bundle.base_git_capture.git_blob = "0".repeat(40);
    if (change === "mislabeled") bundle.base_git_capture.scope = bundle.pre_c_capture.scope;
    if (change === "path") bundle.files[7].path = "../../redirect";
    if (change === "extra") bundle.files.push(bundle.files[0]);
    if (change === "duplicate") bundle.files[1] = bundle.files[0];
    if (change === "row-missing") bundle.files.pop();
    if (change === "reordered") bundle.files.reverse();
    if (change === "source") bundle.files[0].source += "\n";
    if (change === "hash") bundle.files[0].sha256 = "0".repeat(64);
    changed.set(LEGACY_CONTRACT_BUNDLE, JSON.stringify(bundle));
  }
  // Act / Assert: full-byte authentication precedes any JSON interpretation.
  expect(() => legacyFunctionContractView(changed)).toThrow(LEGACY_CONTRACT_BUNDLE);
});
test("phase165 C legacy bridge separates base provenance and daemon claim source", () => {
  const bundle = JSON.parse(bridgeReal.get(LEGACY_CONTRACT_BUNDLE)!);
  const legacy = legacyFunctionContractView(bridgeReal);
  expect(bundle.files).toHaveLength(8);
  expect(bundle.pre_c_capture.artifact_sha256).toBe("4f557ede3d82567c18c44ba7fc86c88afb9f83c00bfdb110ad26a15040bee981");
  expect(bundle.base_git_capture.artifact_sha256).toBe("2f15d035bfc1e57bc9ef2a9e93f69607d8c46b24ba441938ae5236b24a9bf783");
  expect(bundle.base_git_capture.scope).not.toBe(bundle.pre_c_capture.scope);
  expect(bridgeSha(legacy.get(MIGRATED_FIXTURE)!)).toBe("933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18");
  expect(bridgeSha(historicalFixtureView(bridgeReal).get(MIGRATED_FIXTURE)!)).toBe("faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615");
  expect(legacy.get(BRIDGE_CHILD)).toBe(bridgeReal.get(BRIDGE_CHILD));
});
for (const [index, kind] of [...Array.from({ length: 5 }, (_, index) => [index, "assertion"] as const), [0, "fact"] as const]) test(`phase165 C legacy bridge original helper ${kind} ${index}`, () => {
  // Arrange: authentic Git-base helper, not the relocated current function.
  const legacy = legacyFunctionSources(bridgeReal), file = LEGACY_BRIDGE_ROWS[7][0];
  expect(checkPhase162LegacyFunctionControls(legacy)).toEqual([]);
  const span = (kind === "assertion" ? assertionSpans : factSpans)(legacy.get(file)!, "stale_runtime_configured")[index];
  // Act / Assert: require the original selected structural predicate.
  legacy.set(file, legacy.get(file)!.slice(0, span.start) + "assert!(true)" + legacy.get(file)!.slice(span.end));
  expect(checkPhase162LegacyFunctionControls(legacy).some(failure => failure.includes(file + ":stale_runtime_configured: exact executable " + (kind === "assertion" ? "assertion" : "protocol/matrix fact")))).toBe(true);
});
for (const [file, digest, anchor] of TOOLING_INVERSE_CASES) {
  const prefix = file.includes("phase158") ? "phase165 current158 tooling" : "phase165 C legacy bridge";
  test(`${prefix} exact tooling inverse ${file}`, () => {
    expect(bridgeSha(legacyToolingBase(file, bridgeReal.get(file)!))).toBe(digest);
    expect(() => historicalToolingView(bridgeReal)).not.toThrow();
  });
  for (const change of ["unrelated", "partial", "duplicate", "reordered", "context"]) test(`${prefix} tooling refuses ${file} ${change}`, () => {
    const source = bridgeReal.get(file)!, lines = source.split("\n"), index = lines.findIndex(line => line.includes(anchor)), line = lines[index];
    expect(index).toBeGreaterThanOrEqual(0);
    const changed = change === "unrelated" ? source + "\n" : change === "reordered" ? source.replace(line + "\n" + lines[index + 1], lines[index + 1] + "\n" + line)
      : source.replace(line, change === "partial" ? "" : change === "context" ? "/* changed context */" + line : line + "\n" + line);
    expect(changed).not.toBe(source);
    expect(legacyToolingBase(file, changed)).toBe(changed);
    expect(() => historicalToolingView(new Map(bridgeReal).set(file, changed))).toThrow("unreviewed current tooling fingerprint");
  });
}
test("phase165 C legacy bridge current owner guard remains independent", () => {
  // Arrange / Act / Assert: a valid historical view cannot bless changed current source.
  expect(() => legacyFunctionContractView(bridgeReal)).not.toThrow();
  const file = LEGACY_BRIDGE_ROWS[0][0], changed = real.get(file)!.replace("params.coinbase_maturity, 100", "params.coinbase_maturity, 1");
  expect(changed).not.toBe(real.get(file));
  expect(checkPhase165(real).filter(row => row.rule === "work-owner")).toEqual([]);
  rejects(new Map(real).set(file, changed), file, "work-owner");
});
for (const command of [...NATIVE_COMMANDS, SHARED_FORMAT]) for (const change of ["remove", "duplicate", "replace"]) test(`phase165 C legacy bridge raw native ${change} ${command}`, () => {
  // Arrange / Act: mutate an original documented command in the passing current script.
  expect(checkPhase162IntegratedFilterProof(bridgeReal)).toEqual([]);
  const file = "scripts/verify.sh", changed = bridgeReal.get(file)!.replace(command + "\n", (change === "remove" ? "" : change === "duplicate" ? command + "\n" + command : "removed-command") + "\n");
  // Assert: projection errors cannot replace the raw original diagnostic.
  expect(checkPhase162IntegratedFilterProof(new Map(bridgeReal).set(file, changed)).some(failure => failure.includes(command === SHARED_FORMAT ? "pinned shared fixture format required" : "exact once ordered Phase161/162 native commands"))).toBe(true);
});
for (const line of CURRENT_VERIFY_ADDITIONS) for (const change of ["remove", "duplicate", "comment", "partial", "reorder"]) test(`phase165 C legacy bridge current registration ${change} ${line}`, () => {
  const file = "scripts/verify.sh", source = bridgeReal.get(file)!;
  const changed = source.replace(line + "\n", change === "remove" ? "" : change === "duplicate" ? line + "\n" + line + "\n" : change === "comment" ? "# " + line + "\n" : change === "partial" ? line + " --unknown\n" : "# displaced registration\n") + (change === "reorder" ? line + "\n" : "");
  expect(checkPhase162IntegratedFilterProof(new Map(bridgeReal).set(file, changed)).some(failure => failure.includes("scripts/verify.sh: current registration/projection"))).toBe(true);
});
test("phase165 C legacy bridge original native script and unknown command", () => {
  const file = "scripts/verify.sh", original = verificationHistoricalBase(bridgeReal.get(file)!);
  expect(bridgeSha(original)).toBe("2925344e06e3604fbba1821e49a00b0f2f292448eae205107882d0ed65aacc43");
  expect(checkPhase162IntegratedFilterProof(new Map(bridgeReal).set(file, original))).toEqual([]);
  const changed = original.replace("VERIFY_COMMAND_ORDER'\n", "VERIFY_COMMAND_ORDER'\nunknown-extra-command\n");
  expect(changed).not.toBe(original);
  expect(checkPhase162IntegratedFilterProof(new Map(bridgeReal).set(file, changed)).some(failure => failure.includes("complete historical/native command order required"))).toBe(true);
});
test("phase165 C legacy bridge projection refusal retains raw diagnostics", () => {
  const file = "scripts/verify.sh", changed = bridgeReal.get(file)!.replace(CURRENT_VERIFY_ADDITIONS[0], "# removed addition").replace(SHARED_FORMAT + "\n", "# removed raw shared format\n");
  const failures = checkPhase162IntegratedFilterProof(new Map(bridgeReal).set(file, changed));
  expect(failures.some(failure => failure.includes("current registration/projection"))).toBe(true);
  expect(failures.some(failure => failure.includes("pinned shared fixture format required"))).toBe(true);
});
for (const command of BRIDGE_MANDATORY_COMMANDS) for (const change of ["remove", "duplicate", "replace"]) test(`phase165 C legacy bridge original mandatory ${change} ${command}`, () => {
  // Arrange: explicitly project the original script before historical-only mutations.
  const file = "scripts/verify.sh", original = verificationHistoricalBase(bridgeReal.get(file)!);
  expect(checkPhase162IntegratedFilterProof(new Map(bridgeReal).set(file, original))).toEqual([]);
  const lines = original.split("\n"), index = lines.findIndex(line => line.trimStart().startsWith("run_step ") && line.endsWith(command));
  expect(index).toBeGreaterThanOrEqual(0);
  // Act / Assert: mandatory removal/replacement and complete-order duplication have distinct predicates.
  if (change === "duplicate") lines.splice(index, 0, lines[index]);
  else lines[index] = change === "remove" ? "# removed mandatory gate" : lines[index].replace(command, "removed-command");
  expect(checkPhase162IntegratedFilterProof(new Map(bridgeReal).set(file, lines.join("\n"))).some(failure => failure.includes(change === "duplicate" ? "complete historical/native command order required" : "original mandatory gate " + command))).toBe(true);
});
const ASYNC_RPC_OWNER = "packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs";
test("phase165 C async owner recognizes actual registered RPC tests", () => {
  // Arrange / Act / Assert: real eleven-test registration, unrelated findings cannot satisfy it.
  const findings = checkPhase165(real);
  expect(findings.filter(row => row.file === ASYNC_RPC_OWNER && row.rule === "ownership")).toEqual([]);
});
test("phase165 C async second named offset preserves earlier declarations and modifier", () => {
  // Arrange
  const source = real.get(ASYNC_RPC_OWNER)!;
  const fn = regions(source).filter(fn => maybeTestDeclaration(source, fn.start))[1];
  const declaration = maybeTestDeclaration(source, fn.start)!;
  // Act
  const changed = replaceTestAttributes(source, fn.start, '#[ignore = "reason"]\n' + declaration.registration + "\n");
  const changedFn = regions(changed).find(row => row.symbol === fn.symbol)!;
  // Assert
  expect(changed.slice(0, declaration.start)).toBe(source.slice(0, declaration.start));
  expect(changed.slice(changedFn.start - 6, changedFn.start)).toBe("async ");
  rejects(new Map(real).set(ASYNC_RPC_OWNER, changed), ASYNC_RPC_OWNER, "ownership");
});
for (const options of ['(flavor = "current_thread")', '(flavor="multi_thread", worker_threads=2)', '(start_paused = true)', '(flavor="bad")', '(worker_threads=2)', '(unknown=true)']) {
  test(`phase165 C async macro options ${options}`, () => {
    // Arrange / Act: change only the actual named test's immediate registration.
    const source = real.get(ASYNC_RPC_OWNER)!;
    const fn = regions(source).find(fn => maybeTestDeclaration(source, fn.start))!;
    const changed = replaceTestAttributes(source, fn.start, `#[tokio::test${options}]\n`);
    const selected = regions(changed).find(row => row.symbol === fn.symbol)!;
    // Assert: bounded supported options retain ownership; malformed options refuse.
    const accepted = !options.includes("bad") && !options.includes("unknown") && options !== "(worker_threads=2)";
    expect(maybeTestDeclaration(changed, selected.start) !== undefined).toBe(accepted);
    expect(changed.slice(selected.start - 6, selected.start)).toBe("async ");
    expect(checkPhase165(new Map(real).set(ASYNC_RPC_OWNER, changed)).some(row => row.file === ASYNC_RPC_OWNER && row.rule === "ownership")).toBe(!accepted);
  });
}
test("phase165 downstream fixture guard unchanged actual source", () => {
  expect(checkPhase165(real)).toEqual([]);
});
for (const [owner, symbol, before, after] of FIXTURE_SOURCE_MUTATIONS) {
  test(`phase165 current fixture source guard ${owner}:${symbol}:${before}`, () => {
    // Arrange: mutate one executable anchor in the real reviewed owner.
    expect(checkPhase165(real)).toEqual([]);
    const file = owner.startsWith("packages/") ? owner : "packages/open-bitcoin-node/src/" + owner;
    expect(real.has(file)).toBe(true);
    const fn = regions(real.get(file)!).find(region => region.symbol === symbol)!;
    expect(fn.raw.split(before).length).toBe(2);
    // Act / Assert: source-policy evidence complements actual Rust behavior.
    rejects(mutation(file, fn.raw, fn.raw.replace(before, after)), file, "work-owner");
  });
}
for (const [file, symbol, before, after] of DOWNSTREAM_SOURCE_MUTATIONS) {
  test(`phase165 downstream fixture guard ${file}:${symbol}:${before}`, () => {
    // Arrange: select the actual owner before replacing its confirmed executable anchor.
    expect(checkPhase165(real)).toEqual([]);
    const source = real.get(file)!;
    const owner = regions(source).find(fn => fn.symbol === symbol)!;
    expect(owner).toBeDefined();
    expect(owner.raw.includes(before)).toBe(true);
    const changed = owner.raw.replace(before, after);
    expect(changed).not.toBe(owner.raw);
    const inputs = new Map(real).set(file, source.slice(0, owner.start) + changed + source.slice(owner.end));
    // Act / Assert: source controls complement the executed Rust behavior.
    expect(checkPhase165(inputs).some(row => row.file === file && row.rule === "work-owner")).toBe(true);
  });
}

const N = "packages/open-bitcoin-node/src/";
const H = "packages/open-bitcoin-network/src/header_store";
const E = "packages/open-bitcoin-chainstate/src/engine";
const P = "packages/open-bitcoin-consensus/src/";

const LIVE = N + "sync/live_fixture.rs";
const LIVE_UNIT = N + "sync/tests/phase165_live_fixture.rs";
const LIVE_EXTERNAL = "packages/open-bitcoin-node/tests/phase165_live_fixture.rs";

for (const [before, after] of LIVE_SOURCE_MUTATIONS) {
  test(`phase165 live fixture guard: ${before} -> ${after}`, () => {
    // Arrange: every mutation starts from the actual unchanged loaded baseline.
    expect(checkPhase165(real)).toEqual([]);
    const inputs = mutation(LIVE, before, after);
    // Act / Assert: a source identity finding is not executed semantic mutation evidence.
    rejects(inputs, LIVE, "work-owner");
  });
}

for (const file of [LIVE_UNIT, LIVE_EXTERNAL]) {
  for (const change of ["ignore", "disable", "remove"]) {
    test(`phase165 live fixture guard: ${file} enabled tests ${change}`, () => {
      expect(checkPhase165(real)).toEqual([]);
      const source = real.get(file)!;
      expect(source.includes("#[test]")).toBe(true);
      const changed = change === "ignore" ? source.replace("#[test]", "#[test]\n#[ignore]")
        : change === "disable" ? "#![cfg(any())]\n" + source : source.replace("#[test]", "");
      rejects(new Map(real).set(file, changed), file, "ownership");
    });
  }
  for (const chain of ["Mainnet", "Testnet3", "Signet", "Regtest"]) {
    test(`phase165 live fixture guard: ${file} actual canonical ${chain} route required`, () => {
      expect(checkPhase165(real)).toEqual([]);
      const call = file === LIVE_UNIT ? `canonical_case(CanonicalChain::${chain})`
        : `canonical_refuses(CanonicalChain::${chain})`;
      rejects(mutation(file, call, "assert!(true)"), file, "ownership");
    });
  }
  for (const [before, after] of [
    ["pow_limit_bits: genesis.header.bits", "pow_limit_bits: 0x207f_ffff"],
    ["enforce_bip34_height_in_coinbase: false", "enforce_bip34_height_in_coinbase: true"],
    [".validate_live_origin()", ".has_earned_work()"],
    [".bind_live_work_source(selected)", ".bind_live_work_source(ConsensusParams::default())"],
  ]) {
    const owner = file === LIVE_UNIT ? N + "sync/tests.rs" : file;
    test(`phase165 live fixture guard: ${owner} genuine canonical setup ${before}`, () => {
      expect(checkPhase165(real)).toEqual([]);
      const source = real.get(owner)!;
      const symbol = file === LIVE_UNIT ? "canonical_case" : "canonical_refuses";
      const maybeRegion = regions(source).find(fn => fn.symbol === symbol);
      expect(maybeRegion).toBeDefined();
      const region = maybeRegion!;
      expect(region.raw.includes(before)).toBe(true);
      expect(before).not.toBe(after);
      const changed = region.raw.replace(before, after);
      rejects(new Map(real).set(owner, source.slice(0, region.start) + changed
        + source.slice(region.end)), owner, "ownership");
    });
  }
}

for (const [file, before, after, rule] of [
  [N + "sync.rs", "mod live_fixture;", "#[cfg(test)] mod live_fixture;", "work-owner"],
  [N + "sync/tests.rs", "mod phase165_live_fixture;", "#[cfg(any())] mod phase165_live_fixture;", "ownership"],
  [N + "sync/open_runtime.rs", "let network = ManagedNetworkHandle::new(network);", "let _guard = store.filter_publication_guard()?; let network = ManagedNetworkHandle::new(network);", "work-owner"],
  [N + "sync/open_runtime.rs", "network.initialize_basic_filter_index_owner()?;", "// normal BASIC owner omitted", "work-owner"],
  [N + "chainstate.rs", "#[cfg(test)]\nmod live_fixture;", "mod live_fixture;", "work-owner"],
  [N + "network/runtime_authority.rs", "#[cfg(test)]\nmod live_fixture;", "mod live_fixture;", "work-owner"],
] as const) {
  test(`phase165 live fixture guard: preserved source boundary ${file} ${before}`, () => {
    expect(checkPhase165(real)).toEqual([]);
    rejects(mutation(file, before, after), file, rule);
  });
}

test("current ordered fixed inventory passes before reads", () => {
  // Arrange / Act / Assert
  expect(() => assertInputInventory(INPUT_FILES)).not.toThrow();
});

for (const files of [
  ["../../outside-input", ...INPUT_FILES.slice(1)],
  [...INPUT_FILES, "README-extra.md"],
  INPUT_FILES.slice(1),
  [...INPUT_FILES].reverse(),
]) {
  test(`manifest cannot redirect, extend, remove or reorder fixed input paths: ${files[0]}`, () => {
    // Arrange / Act / Assert
    expect(() => assertInputInventory(files)).toThrow("before any source read");
  });
}

function mutation(file: string, before: string, after: string): Map<string, string> {
  const source = real.get(file)!;
  expect(source).toBeDefined();
  expect(source.includes(before)).toBe(true);
  expect(before).not.toBe(after);
  return new Map(real).set(file, source.replace(before, after));
}

function rejects(inputs: ReadonlyMap<string, string>, file: string, rule: string): void {
  const findings = checkPhase165(inputs);
  expect(findings.some(row => row.file === file && row.rule === rule)).toBe(true);
}

test("actual current source satisfies the exact-work contracts", () => {
  // Arrange
  const inputs = loadInputs();
  // Act
  const findings = checkPhase165(inputs);
  // Assert
  expect(findings).toEqual([]);
});
for (const [file] of TOOLING_INVERSE_CASES) test(`phase165 reviewed current158 history ${file}`, () => {
  expect(checkPhase165(real)).toEqual([]);
  expect(checkPhase165(new Map(real).set(file, real.get(file)! + "\n"))).toContainEqual(expect.objectContaining({ file: "packages/testdata/phase164/historical-tooling/manifest.json", rule: "history", detail: expect.stringContaining(file) }));
});

for (const change of [...OWNER_SOURCE_MUTATIONS, ...PEER_SOURCE_MUTATIONS, ...PUBLIC_REQUALIFICATION_MUTATIONS, ...CONSUMER_SOURCE_MUTATIONS, ...DAEMON_SOURCE_MUTATIONS, ...PRUNE_SOURCE_MUTATIONS, ...PREPARATION_SOURCE_MUTATIONS]) {
  const [file, before, after] = change;
  test(`actual owner mutation refuses: ${file}: ${after}`, () => {
    // Arrange
    if (PEER_SOURCE_MUTATIONS.includes(change) || PUBLIC_REQUALIFICATION_MUTATIONS.includes(change) || CONSUMER_SOURCE_MUTATIONS.includes(change) || DAEMON_SOURCE_MUTATIONS.includes(change) || PRUNE_SOURCE_MUTATIONS.includes(change) || PREPARATION_SOURCE_MUTATIONS.includes(change)) { expect(real.get(file)!.split(before)).toHaveLength(2); expect(checkPhase165(real)).toEqual([]); }
    const inputs = mutation(file, before, after);
    // Act / Assert
    rejects(inputs, file, SOURCE_CONTRACTS.find(row => row[0] === file)![1]);
  });
}

function replaceTestAttributes(source: string, functionStart: number, attribute: string): string {
  const declaration = maybeTestDeclaration(source, functionStart)!;
  expect(declaration).toBeDefined();
  return source.slice(0, declaration.start) + attribute + source.slice(declaration.end);
}

test("phase165 named attribute offset preserves earlier enabled tests", () => {
  // Arrange: the second named test shares the first test's attribute text.
  const source = real.get(LIVE_UNIT)!;
  const tests = regions(source).filter(fn => fn.symbol.startsWith("phase165_"));
  const fn = tests[1];
  const attributeStart = maybeTestDeclaration(source, fn.start)!.start;
  // Act
  const changed = replaceTestAttributes(source, fn.start, "#[ignore]\n#[test]\n");
  // Assert
  expect(changed.slice(0, attributeStart)).toBe(source.slice(0, attributeStart));
  expect(changed.slice(attributeStart, attributeStart + 24)).toStartWith("#[ignore]\n#[test]\n");
});

for (const [file] of TEST_OWNERS) {
  const source = real.get(file)!;
  const code = rustCode(source);
  const actualTests = regions(source).filter(fn => maybeTestDeclaration(source, fn.start));
  expect(actualTests.length).toBeGreaterThan(0);
  // Exercise each current owner and every new phase-named behavioral case.
  const controls = actualTests.filter((fn, index) => index === 0 || fn.symbol.startsWith("phase165_"));
  for (const fn of controls) {
    const registration = maybeTestDeclaration(source, fn.start)!.registration;
    const attributes = ["", `#[ignore]\n${registration}\n`, `#[cfg(any())]\n${registration}\n`];
    if (registration.includes("tokio")) attributes.push(`#[ignore = "reason"]\n${registration}\n`);
    for (const attribute of attributes) {
      test(`enabled test owner refuses ${file}:${fn.symbol}:${attribute}`, () => {
        // Arrange
        const attributeStart = maybeTestDeclaration(source, fn.start)!.start;
        const changed = replaceTestAttributes(source, fn.start, attribute);
        expect(changed.slice(0, attributeStart)).toBe(source.slice(0, attributeStart));
        expect(changed.slice(attributeStart, attributeStart + attribute.length)).toBe(attribute);
        const inputs = new Map(real).set(file, changed);
        // Act / Assert
        rejects(inputs, file, "ownership");
      });
    }
  }
  for (const attribute of ["#![cfg(any())]", "#![cfg_attr(test, ignore)]"]) {
    test(`file-level disabling refuses ${file}:${attribute}`, () => {
      // Arrange
      const inputs = new Map(real).set(file, attribute + "\n" + source);
      // Act / Assert
      rejects(inputs, file, "ownership");
    });
  }
  test(`commented inner cfg does not disable executable tests: ${file}`, () => {
    // Arrange
    const inputs = new Map(real).set(file, "// #![cfg(any())]\n" + source);
    // Act
    const findings = checkPhase165(inputs);
    // Assert
    expect(findings).toEqual([]);
  });
}

for (const [file, declaration] of TEST_REGISTRATIONS) {
  for (const replacement of ["", `/* ${declaration} */`, `#[cfg(any())]\n${declaration}`, '#[path = "spoofed.rs"]\n' + declaration]) {
    test(`real test registration refuses: ${file}:${replacement}`, () => {
      // Arrange
      const inputs = mutation(file, declaration, replacement);
      const rule = SOURCE_CONTRACTS.find(row => row[0] === file)![1];
      // Act / Assert
      rejects(inputs, file, rule);
    });
  }
}

for (const line of CURRENT_VERIFY_ADDITIONS) {
  for (const replacement of ["", line + "\n" + line, "# " + line, "false && " + line]) {
    test(`both current guards refuse missing/duplicate/commented/disabled registration: ${line}:${replacement}`, () => {
      // Arrange
      const inputs = mutation("scripts/verify.sh", line + "\n", replacement + "\n");
      // Act
      const findings = checkPhase164(inputs);
      // Assert
      rejects(inputs, "scripts/verify.sh", "registration");
      expect(findings.some(row => row.file === "scripts/verify.sh" && row.rule === "registration")).toBe(true);
    });
  }
}

test("every old verification byte is still pinned", () => {
  // Arrange
  const inputs = mutation("scripts/verify.sh", "set -euo pipefail", "set -eu");
  // Act / Assert
  rejects(inputs, "scripts/verify.sh", "registration");
  expect(() => historicalToolingView(inputs)).toThrow();
});

test("older guard independently prevents removal of all six new mandatory lines", () => {
  // Arrange
  expect(checkPhase164(real)).toEqual([]);
  const source = real.get("scripts/verify.sh")!;
  const removed = source.split("\n").filter(line => !CURRENT_VERIFY_ADDITIONS.includes(line)).join("\n");
  const inputs = new Map(real).set("scripts/verify.sh", removed);
  // Act: this invokes only the already mandatory older guard.
  const findings = checkPhase164(inputs);
  // Assert: raw historical attribution still works, but current execution refuses.
  expect(() => historicalToolingView(inputs)).not.toThrow();
  expect(findings.some(row => row.rule === "input")).toBe(false);
  expect(findings.some(row => row.file === "scripts/verify.sh" && row.rule === "registration")).toBe(true);
});

test("preference cannot move behind staged BASIC preparation", () => {
  // Arrange
  const file = N + "chainstate/work_authority.rs";
  const check = "authorization.check_stage(&self.chainstate, &staged)?;";
  const later = "let maybe_index = self.prepare_basic_index_reorg(&staged, replacement_branch)?;";
  expect(real.get(file)!.includes(later)).toBe(true);
  const inputs = mutation(file, check, "");
  inputs.set(file, inputs.get(file)!.replace(later, later + "\n" + check));
  // Act / Assert
  rejects(inputs, file, "work-owner");
});

for (const [file] of HISTORICAL_IDENTITIES) {
  test(`historical raw receipt/tooling remains immutable: ${file}`, () => {
    // Arrange
    const inputs = new Map(real).set(file, real.get(file)! + "\n");
    // Act / Assert
    rejects(inputs, file, "history");
  });
}

test("historical source digest cannot be renewed to claim current acceptance", () => {
  // Arrange
  const file = HISTORICAL_IDENTITIES.find(row => row[0].endsWith("163-VERIFICATION.md"))![0];
  const before = "source_digest: 14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977";
  const inputs = mutation(file, before, "source_digest: " + "0".repeat(64));
  // Act / Assert
  rejects(inputs, file, "history");
});

for (const [key, value] of [["evidence_class", "actual-daemon-differential"], ["proof_hex", "0".repeat(64)], ["commit", "0".repeat(40)]] as const) {
  test(`independent corpus refuses changed ${key}`, () => {
    // Arrange
    const corpus = JSON.parse(real.get(CORPUS_FILE)!);
    const previous = key === "proof_hex" ? corpus.proof_vectors[0].proof_hex
      : key === "commit" ? corpus.source_pin.commit : corpus.evidence_class;
    const inputs = mutation(CORPUS_FILE, `"${key}": "${previous}"`, `"${key}": "${value}"`);
    // Act / Assert
    rejects(inputs, CORPUS_FILE, "corpus");
  });
}

for (const phase of [166, 167, 168]) {
  test(`current claims cannot deliver deferred Phase${phase}`, () => {
    // Arrange
    const file = "docs/parity/v2-6-canonical-chain-identity.md";
    const inputs = new Map(real).set(file, real.get(file)! + `\n\nPhase ${phase} is complete and qualified.\n`);
    // Act / Assert
    rejects(inputs, file, "claim");
  });
}

for (const [before, after] of WORK_DOC_OVERCLAIMS) {
  test(`actual work document overclaim refuses: ${after}`, () => {
    // Arrange
    const inputs = mutation(WORK_CLAIM_FILE, before, after);
    // Act / Assert
    rejects(inputs, WORK_CLAIM_FILE, "claim");
  });
  test(`appended work document overclaim refuses: ${after}`, () => {
    // Arrange: preserve every required fact to exercise the positive claim rule.
    const inputs = new Map(real).set(WORK_CLAIM_FILE, real.get(WORK_CLAIM_FILE)! + "\n\n" + after);
    // Act / Assert
    rejects(inputs, WORK_CLAIM_FILE, "claim");
  });
}

for (const claim of CLAIM_SCOPE_REFUSALS) {
  test(`phase165 claim negation scope rejects appended ${claim}`, () => {
    // Arrange: preserve every required denial and append a positive claim.
    const inputs = new Map(real).set(WORK_CLAIM_FILE, real.get(WORK_CLAIM_FILE)! + "\n\n" + claim);
    // Act / Assert
    rejects(inputs, WORK_CLAIM_FILE, "claim");
  });
}
for (const claim of ["Phase 168 pinned-Knots daemon qualification is complete.", "Phase 166 canonical bootstrap is complete."]) {
  test(`phase165 claim negation scope preserves compound phase subject ${claim}`, () => {
    // Arrange: outside the inherited README current-section scope and work-only rules.
    const file = "README.md";
    const inputs = new Map(real).set(file, real.get(file)! + "\n\n" + claim);
    // Act / Assert
    rejects(inputs, file, "claim");
  });
}
for (const claim of [null, "Phase 166 canonical bootstrap is complete.", "Phase 168 pinned-Knots daemon qualification is complete.", "{invalid"]) {
  test(`phase165 claim JSON string ownership ${claim}`, () => {
    const file = "docs/parity/index.json";
    const index = JSON.parse(real.get(file)!);
    if (claim !== null) index.phase165_claim_control = claim;
    const inputs = new Map(real).set(file, claim === "{invalid" ? claim : JSON.stringify(index));
    const findings = checkPhase165(inputs).filter(row => row.file === file);
    if (claim === null) expect(findings).toEqual([]);
    else expect(findings.some(row => row.rule === (claim === "{invalid" ? "input" : "claim"))).toBe(true);
  });
}
for (const claim of CLAIM_SCOPE_DENIALS) {
  test(`phase165 claim negation scope preserves attached denial ${claim}`, () => {
    // Arrange
    const inputs = new Map(real).set(WORK_CLAIM_FILE, real.get(WORK_CLAIM_FILE)! + "\n\n" + claim);
    // Act / Assert
    expect(checkPhase165(inputs).filter(row => row.file === WORK_CLAIM_FILE && row.rule === "claim")).toEqual([]);
  });
}

test("pure input guard refuses missing and oversized actual entries", () => {
  // Arrange
  const file = INPUT_FILES[0];
  const missing = new Map(real);
  missing.delete(file);
  const oversized = new Map(real).set(file, "x".repeat(MAX_FILE_BYTES + 1));
  // Act / Assert
  rejects(missing, file, "input");
  rejects(oversized, file, "input");
});

for (const input of ["packages/testdata/phase164/historical-tooling/manifest.json", LEGACY_CONTRACT_BUNDLE]) for (const shape of ["missing", "malformed", "symlink", "oversized", "redirect", "extra", "reordered"] as const) {
  test(`phase165 bounded manifest fresh import ${input} ${shape}`, () => {
    // Arrange: real fourteen-module CLI tree, isolated data and a fresh process.
    const root = mkdtempSync(join(tmpdir(), "open-bitcoin-phase165-import-"));
    const manifest = input;
    try {
      copiedInputs(root, new Map([...bridgeReal, ...real]));
      if (shape === "missing") rmSync(join(root, manifest));
      else if (shape === "malformed") writeFileSync(join(root, manifest), "{invalid JSON");
      else if (shape === "oversized") writeFileSync(join(root, manifest), Buffer.alloc(MAX_FILE_BYTES + 1, 0x7b));
      else if (shape === "symlink") {
        const target = join(root, "invalid-target.json");
        writeFileSync(target, "{invalid JSON");
        rmSync(join(root, manifest));
        symlinkSync(target, join(root, manifest));
      } else {
        const changed = JSON.parse(real.get(manifest)!);
        const rows = manifest === LEGACY_CONTRACT_BUNDLE ? changed.files : changed.bindings;
        if (shape === "redirect") rows[0][manifest === LEGACY_CONTRACT_BUNDLE ? "path" : "file"] = "../../unreviewed-source";
        if (shape === "extra") rows.push({ ...rows[0], file: "unreviewed-extra" });
        if (shape === "reordered") rows.reverse();
        writeFileSync(join(root, manifest), JSON.stringify(changed));
      }
      // Act: importing modules alone cannot open/parse data or change input paths.
      const moduleUrl = new URL("file://" + join(root, CLI_MODULES[0])).href;
      const imported = Bun.spawnSync([process.execPath, "-e", `const module = await import(${JSON.stringify(moduleUrl)}); console.log(JSON.stringify(module.INPUT_FILES));`]);
      // Assert: bounded loading owns unsafe data; policy authenticates inert JSON bytes.
      expect(imported.stderr.toString()).toBe("");
      expect(imported.exitCode).toBe(0);
      expect(JSON.parse(imported.stdout.toString())).toEqual(INPUT_FILES);
      if (["missing", "symlink", "oversized"].includes(shape)) expect(() => loadInputs(root)).toThrow(manifest);
      else rejects(loadInputs(root), manifest, manifest === LEGACY_CONTRACT_BUNDLE ? "bridge-data" : "history");
      if (manifest === LEGACY_CONTRACT_BUNDLE && ["missing", "symlink", "oversized"].includes(shape)) expect(() => loadLegacyBridgeInputs(root)).toThrow(manifest);
    } finally { rmSync(root, { recursive: true }); }
  });
}

for (const shape of ["missing", "directory", "symlink", "oversized"] as const) {
  test(`read-only fixed loader refuses ${shape} input without checkout fallback`, () => {
    // Arrange
    const root = mkdtempSync(join(tmpdir(), "open-bitcoin-phase165-"));
    const file = join(root, INPUT_FILES[0]);
    try {
      mkdirSync(dirname(file), { recursive: true });
      if (shape === "directory") mkdirSync(file);
      if (shape === "symlink") symlinkSync(join(import.meta.dir, "check-phase165-shared-exact-work.ts"), file);
      if (shape === "oversized") writeFileSync(file, Buffer.alloc(MAX_FILE_BYTES + 1));
      // Act / Assert
      expect(() => loadInputs(root)).toThrow(shape === "missing" ? "ENOENT" : shape === "oversized" ? "byte limit" : "regular input");
    } finally { rmSync(root, { recursive: true }); }
  });
}
