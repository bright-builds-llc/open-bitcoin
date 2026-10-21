import { afterEach, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { CHECK_FILES, checkPhase157IndexCatchUp } from "./check-phase157-index-catch-up.ts";
import { CONTRACTS, CORPUS, DOC, DOCUMENTED_LIMITS, EVIDENCE, NEW_RUST, NODE, PHASE, PRODUCTION_BUDGET, REGISTRATIONS, SURFACE, TESTS } from "./check-phase157-index-catch-up/contracts.ts";
import { maybeFunction, ordinaryRust } from "./check-phase157-index-catch-up/rust-evidence.ts";

test("commented cfg-test owner cannot qualify explicit fixture startup", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs";
  edit(root, file, source => source.replace('#[cfg(test)]\n#[path = "open_bitcoind/tests.rs"]\nmod tests;', '#[cfg(any())]\n#[path = "open_bitcoind/tests.rs"]\nmod tests;\n/* #[cfg(test)]\n#[path = "open_bitcoind/tests.rs"]\nmod tests; */'));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures).toContain("T-157-30/D-03: exact cfg-test fixture owner required");
});

test("commented fixture module cannot replace executable registration", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs";
  edit(root, file, source => source.replace('pub(crate) mod fixture_runtime;', '/* pub(crate) mod fixture_runtime; */'));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures).toContain("T-157-30/D-03: exact cfg-test fixture owner required");
});

test("attribute-only comment cannot authorize disabled ordinary owner", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs";
  edit(root, file, source => source.replace('#[cfg(test)]\n#[path = "open_bitcoind/tests.rs"]\nmod tests;', '#[cfg(any())]\n#[path = "open_bitcoind/tests.rs"]\n// #[cfg(test)] #[path = "open_bitcoind/tests.rs"]\nmod tests;'));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures).toContain("T-157-30/D-03: exact cfg-test fixture owner required");
});

test("attribute-only comment cannot authorize a wrong fixture module path", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs";
  edit(root, file, source => source.replace('#[path = "tests/fixture_runtime.rs"]\npub(crate) mod fixture_runtime;', '#[path = "wrong.rs"]\n// #[path = "tests/fixture_runtime.rs"]\npub(crate) mod fixture_runtime;'));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures).toContain("T-157-30/D-03: exact cfg-test fixture owner required");
});

const REPO = resolve(import.meta.dir, "..");
const COMPLETION_REPORTS = [PHASE + "157-VERIFICATION.md", PHASE + "157-SECURITY.md"];
const roots: string[] = [];
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "phase157-evidence-"));
  roots.push(root);
  for (const file of [...CHECK_FILES, ...COMPLETION_REPORTS.filter(file => existsSync(join(REPO, file)))]) {
    mkdirSync(dirname(join(root, file)), { recursive: true });
    writeFileSync(join(root, file), readFileSync(join(REPO, file)));
  }
  return root;
}
function edit(root: string, file: string, transform: (source: string) => string): void {
  const target = join(root, file);
  const before = readFileSync(target, "utf8");
  const after = transform(before);
  expect(after).not.toBe(before);
  writeFileSync(target, after);
}
function pattern(anchor: string): RegExp {
  return new RegExp([...anchor.replace(/\s/g, "")].map(char => char.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("\\s*"), "g");
}
function mutateFunction(root: string, file: string, symbol: string, transform: (body: string) => string): void {
  edit(root, file, source => {
    const maybeBody = maybeFunction(source, symbol, true);
    expect(maybeBody).toBeDefined();
    if (!maybeBody) throw new Error("missing fixture function");
    return source.slice(0, maybeBody.start) + transform(source.slice(maybeBody.start, maybeBody.end)) + source.slice(maybeBody.end);
  });
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

const CURRENT_STARTUP = NODE + "sync/open_runtime.rs";
const CURRENT_HISTORY = NODE + "chainstate/validation_history.rs";
const GENUINE_HELPERS = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs";
for (const [label, assertion] of [
  ["retained BASIC error", /assert!\(\s*(matches!\(result, Err\(CoinsFlushError::BasicFilter\(_\)\)\)),\s*"[^"]*"\s*\);/g],
  ["joined shutdown error", /assert!\(\s*(matches!\(result, Err\(DaemonCheckpointError::ShutdownCheckpoint\)\)),\s*"[^"]*"\s*\);/g],
  ["outer typed error", /assert!\(\s*(matches!\(\s*result\s*\.expect_err\("failure remains visible"\)\s*\.downcast_ref::<DaemonCheckpointError>\(\),\s*Some\(DaemonCheckpointError::ShutdownCheckpoint\)\s*\))\s*\);/g],
] as const) {
  test(`current shutdown error requires assertion wrapper: ${label}`, () => {
    // Arrange
    const root = fixture();
    const file = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs";
    const symbol = "phase157_shutdown_retained_basic_error_settles_all_workers_without_clean_marker";
    expect(checkPhase157IndexCatchUp(root)).toEqual([]);
    mutateFunction(root, file, symbol, body => {
      expect([...body.matchAll(assertion)]).toHaveLength(1);
      return body.replace(assertion, "$1;");
    });
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures).toContain(`D-11: concrete evidence missing ${file}:${symbol}`);
  });
}
for (const [label, file, symbol, before, after, category] of [
  ["typed stage", CURRENT_HISTORY, "commit_prepared_connect", ".validate_staged_connect(prepared.staged)?", ".unchecked_connect(prepared.staged)", "T-157-30/D-07"],
  ["propagated stage error", CURRENT_HISTORY, "commit_prepared_connect", ".validate_staged_connect(prepared.staged)?", ".validate_staged_connect(prepared.staged)", "T-157-30/D-07"],
  ["history seal", CURRENT_HISTORY, "commit_prepared_connect", "history.seal_connect(&position)", "history.dummy(&position)", "T-157-30/D-07"],
  ["publication error", CURRENT_HISTORY, "commit_prepared_connect", "return Err(map_persist(error));", "discard(error);", "T-157-30/D-07"],
  ["prepare forwarding", CURRENT_STARTUP, "open_with_configured_runtime_activation", "Self::prepare_initialized_network(\n            &store,", "Self::prepare_initialized_network(\n            &other_store,", "T-157-30/D-08"],
  ["finish forwarding", CURRENT_STARTUP, "open_with_configured_runtime_activation", "store, network, config, consensus_params", "store, other_network, config, consensus_params", "T-157-30/D-08"],
  ["authoritative handle", CURRENT_STARTUP, "finish_initialized_runtime", "ManagedNetworkHandle::new(network)", "DummyHandle::new(network)", "T-157-30/D-08"],
  ["initialization error", CURRENT_STARTUP, "finish_initialized_runtime", "network.initialize_basic_filter_index_owner()?;", "network.initialize_basic_filter_index_owner();", "T-157-30/D-08"],
  ["late initialization", CURRENT_STARTUP, "finish_initialized_runtime", "network.initialize_basic_filter_index_owner()?;\n        network.drive_basic_filter_index_turn()?;", "network.drive_basic_filter_index_turn()?;\n        network.initialize_basic_filter_index_owner()?;", "T-157-30/D-08"],
  ["genuine open", "packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs", "phase157_shutdown_retained_basic_error_settles_all_workers_without_clean_marker", "history.open_genuine(&config)", "history.open_raw(&config)", "D-11"],
  ["genuine helper edge", GENUINE_HELPERS, "open_genuine", "genuine_runtime(store.clone(), &self.blocks, config)", "raw_runtime(store.clone(), &self.blocks, config)", "D-11"],
  ["genuine acceptance", GENUINE_HELPERS, "genuine_runtime", ".connect_block(block, ProofWork::ZERO, ScriptVerifyFlags::P2SH, params())", ".seed_block(block, ProofWork::ZERO, ScriptVerifyFlags::P2SH, params())", "D-11"],
  ["physical Always", GENUINE_HELPERS, "genuine_runtime", "FlushMode::Always", "FlushMode::Periodic", "D-11"],
  ["same accepted source", GENUINE_HELPERS, "genuine_runtime", "ManagedChainstate::from_chainstate(sink, core, lifecycle)", "ManagedChainstate::dummy()", "D-11"],
  ["consuming constructor", GENUINE_HELPERS, "configured_source", "DurableSyncRuntime::from_live_fixture_source(", "DurableSyncRuntime::open(", "D-11"],
  ["configured forwarding", GENUINE_HELPERS, "configured_source", "config.inbound.enabled,", "false,", "D-11"],
] as const) {
  test(`current typed startup and shutdown refuses missing ${label}`, () => {
    // Arrange
    const root = fixture();
    expect(checkPhase157IndexCatchUp(root)).toEqual([]);
    mutateFunction(root, file, symbol, body => {
      expect(body.split(before)).toHaveLength(label === "publication error" ? 3 : 2);
      return body.replaceAll(before, after);
    });
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes(category) && item.includes(`${file}:${symbol}`))).toBe(true);
  });
}

const LIFECYCLE = NODE + "storage/fjall_store/filters/lifecycle.rs";
const RETENTION = NODE + "storage/fjall_store/filters/retention.rs";
const FRESH_PREFLIGHT = "self.preflight_basic_filter_history(IndexInputProtection::FromHeight(0), fence)?;";
const SAVED_PREFLIGHT = "self.preflight_basic_filter_history(checkpoint.input_protection(), fence)?;";
for (const [symbol, statement] of [
  ["enable_basic_filter_index_guarded", FRESH_PREFLIGHT],
  ["enable_basic_filter_index_guarded", SAVED_PREFLIGHT],
  ["preflight_basic_filter_history", "self.load_block(position.block_hash)?"],
  ["preflight_basic_filter_history", "self.load_undo(position.block_hash)?"],
] as const) {
  test(`extracted guarded suffix/input validation cannot be conditional: ${statement}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, LIFECYCLE, symbol, body => body.replace(pattern(statement), `if false { ${statement} }`));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("mandatory guarded preflight") && item.includes(symbol))).toBe(true);
  });
}
for (const preflight of [FRESH_PREFLIGHT, SAVED_PREFLIGHT]) {
  test(`required suffix after corresponding Active return refuses: ${preflight}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, LIFECYCLE, "enable_basic_filter_index_guarded", body => {
      const maybeReturn = [...body.matchAll(/return self\.install_recovered_basic_filter_append_guarded\([\s\S]*?\);/g)]
        .find(match => preflight === FRESH_PREFLIGHT || match[0].includes(", retained)"));
      if (!maybeReturn) throw new Error("missing actual recovered append return");
      return body.replace(preflight, "").replace(maybeReturn[0], `${maybeReturn[0]}\n${preflight}`);
    });
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("enable_basic_filter_index_guarded"))).toBe(true);
  });
}
test("public enable wrapper cannot succeed before guarded helper", () => {
  // Arrange
  const root = fixture();
  mutateFunction(root, LIFECYCLE, "enable_basic_filter_index", body => body.replace("{", "{ return Ok(());"));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures.some(item => item.includes("mandatory guarded preflight") && item.includes("bypassed"))).toBe(true);
});
test("public enable wrapper cannot conditionally invoke guarded helper", () => {
  // Arrange
  const root = fixture();
  mutateFunction(root, LIFECYCLE, "enable_basic_filter_index", body => body.replace("{", "{ if false {").replace(/}\s*$/, "} }"));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures.some(item => item.includes("mandatory guarded preflight"))).toBe(true);
});
test("shared immutable admission cannot conditionally enforce ceilings", () => {
  // Arrange
  const root = fixture();
  mutateFunction(root, RETENTION, "check_envelope_bounds", body => body.replace("{", "{ if false {").replace(/}\s*$/, "} }"));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures.some(item => item.includes("shared immutable admission"))).toBe(true);
});
test("shared immutable admission moved after allocation refuses", () => {
  // Arrange
  const root = fixture();
  mutateFunction(root, NODE + "storage/fjall_store/filters/append.rs", "prepare_basic_filter_append_guarded_positions", body =>
    body.replace("check_envelope_bounds(records)?;", "")
      .replace("let mut prepared = PreparedBasicFilterAppend {", "let mut prepared = PreparedBasicFilterAppend {\ncheck_envelope_bounds(records)?;"));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures.some(item => item.includes("mandatory immutable admission"))).toBe(true);
});
for (const symbol of ["enable_basic_filter_index", "enable_basic_filter_index_guarded"]) {
  for (const substitute of ["comment", "raw literal", "test-only"]) {
    test(`ordinary ${symbol} cannot be replaced by ${substitute}`, () => {
      // Arrange
      const root = fixture();
      const source = readFileSync(join(root, LIFECYCLE), "utf8");
      const maybeBody = maybeFunction(source, symbol, true);
      if (!maybeBody) throw new Error("missing actual enable function");
      mutateFunction(root, LIFECYCLE, symbol, body => body.replace(`fn ${symbol}(`, `fn removed_${symbol}(`));
      const decoy = `fn ${symbol}() { ${maybeBody.body} }`;
      edit(root, LIFECYCLE, source => source + (substitute === "comment" ? `\n/* ${decoy} */\n`
        : substitute === "raw literal" ? `\nconst DECOY: &str = r##"${decoy}"##;\n`
          : `\n#[cfg(test)] pub(crate) mod counterfeit { ${decoy} }\n`));
      // Act
      const failures = checkPhase157IndexCatchUp(root);
      // Assert
      expect(failures.some(item => item.includes(`${LIFECYCLE}:${symbol}`))).toBe(true);
    });
  }
}
for (const refusal of ["BASIC append record count bound", "BASIC append singleton byte bound", "BASIC append aggregate byte bound"]) {
  test(`shared immutable admission refusal remains required: ${refusal}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, RETENTION, "check_envelope_bounds", body => body.replace(`return Err(index_corruption("${refusal}"));`, "removed_refusal();"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("shared immutable admission"))).toBe(true);
  });
}
for (const substitute of ["if false { check_envelope_bounds(records)?; }", "return Ok(Default::default()); check_envelope_bounds(records)?;",
  "// check_envelope_bounds(records)?;", 'let decoy = r##"check_envelope_bounds(records)?;"##;']) {
  test(`shared immutable admission bypass refuses: ${substitute}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, NODE + "storage/fjall_store/filters/append.rs", "prepare_basic_filter_append_guarded_positions",
      body => body.replace("check_envelope_bounds(records)?;", substitute));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("mandatory immutable admission"))).toBe(true);
  });
}

test("positive snapshot passes and preserves every evidence byte", () => {
  // Arrange
  const root = fixture();
  const files = [...CHECK_FILES, ...COMPLETION_REPORTS.filter(file => existsSync(join(root, file)))];
  const before = files.map(file => readFileSync(join(root, file), "utf8"));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures).toEqual([]);
  expect(files.map(file => readFileSync(join(root, file), "utf8"))).toEqual(before);
});
test("actual async daemon becomes unavailable as ordinary evidence when test-only", () => {
  // Arrange: mutate the real declaration independently of the production selector.
  const root = fixture();
  const file = "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs";
  expect(checkPhase157IndexCatchUp(root)).toEqual([]);
  edit(root, file, source => {
    const declaration = /^async fn serve_authoritative_runtime\b/gm;
    expect([...source.matchAll(declaration)]).toHaveLength(1);
    return source.replace(declaration, "#[cfg(test)]\nasync fn serve_authoritative_runtime");
  });
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures).toEqual([`T-157-30/D-08: ordinary boundary ${file}:serve_authoritative_runtime`]);
});
for (const declaration of ["fn", "async fn", "pub async fn", "pub(crate) async fn", "pub(in crate::worker) async fn",
  "const fn", "pub const unsafe fn", "unsafe fn", "pub(crate) async unsafe fn", 'extern "C" fn', 'pub unsafe extern "C" fn']) {
  test(`test-only ${declaration} is excluded without shifting ordinary source`, () => {
    // Arrange: lexical declaration controls do not claim Rust execution.
    const source = `#[cfg(test)]\n#[allow(dead_code)]\n${declaration} hidden() { { excluded(); } }\nasync fn retained() { retained_effect(); }\n`;
    expect(maybeFunction(source.replace("#[cfg(test)]\n", ""), "hidden", true)).toBeDefined();
    const maybeOriginal = maybeFunction(source, "retained");
    expect(maybeOriginal).toBeDefined();
    if (!maybeOriginal) throw new Error("missing ordinary control function");
    // Act
    const masked = ordinaryRust(source);
    const maybeRetained = maybeFunction(source, "retained", true);
    // Assert
    expect(maybeFunction(source, "hidden", true)).toBeUndefined();
    expect(maybeFunction(source, "hidden")).toBeDefined();
    expect(masked.length).toBe(source.length);
    expect([...masked.matchAll(/\n/g)].map(match => match.index)).toEqual([...source.matchAll(/\n/g)].map(match => match.index));
    expect(maybeRetained?.body).toBe(maybeOriginal.body);
    expect(maybeRetained?.start).toBe(maybeOriginal.start);
    expect(maybeRetained?.end).toBe(maybeOriginal.end);
  });
}
for (const row of CONTRACTS) for (const anchor of row.anchors) {
  test(`ordinary ${row.decision} boundary removal: ${row.symbol}:${anchor}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, row.file, row.symbol, body => body.replace(pattern(anchor), "removed_boundary"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes(`${row.file}:${row.symbol}`))).toBe(true);
  });
}
for (const row of CONTRACTS.filter(row => row.maybeForbidden)) for (const scan of row.maybeForbidden ?? []) {
  test(`hidden full scan refuses in ${row.symbol}: ${scan}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, row.file, row.symbol, body => body.replace("{", `{ ${scan});`));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes(scan.endsWith("?") ? "early shutdown error" : "full scan") && item.includes(row.symbol))).toBe(true);
  });
}
for (const substitute of ["comment", "literal", "raw literal", "private test", "public test", "qualified test"]) {
  test(`deleted ordinary timer cannot be replaced by ${substitute}`, () => {
    // Arrange
    const root = fixture();
    const row = CONTRACTS.find(row => row.symbol === "maybe_drive_elapsed")!;
    const call = "handle.maybe_drive_index_turn()";
    const replacements: Record<string, string> = {
      comment: `// ${call}\n`, literal: `"${call}"`, "raw literal": `r###"${call}"###`,
      "private test": `#[cfg(test)] mod fake { fn maybe_drive_elapsed() { ${call}; } }`,
      "public test": `#[cfg(test)] pub mod fake { fn maybe_drive_elapsed() { ${call}; } }`,
      "qualified test": `#[cfg(test)] pub(crate) mod fake { fn maybe_drive_elapsed() { ${call}; } }`,
    };
    mutateFunction(root, row.file, row.symbol, body => body.replace(call, "removed_call"));
    edit(root, row.file, source => source + "\n" + replacements[substitute]);
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes(`${row.file}:${row.symbol}`))).toBe(true);
  });
}
for (const [file, name] of TESTS) {
  test(`named behavioral assertion must remain: ${name}`, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => source.replace(name, "removed_test"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes(name))).toBe(true);
  });
}
for (const [file, name, anchors] of EVIDENCE) for (const anchor of anchors) {
  test(`real paired-loss/reopen helper removal: ${name}:${anchor}`, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => {
      const maybeBody = maybeFunction(source, name);
      if (!maybeBody) throw new Error("missing concrete evidence fixture");
      return source.slice(0, maybeBody.start) + source.slice(maybeBody.start, maybeBody.end).replace(pattern(anchor), "removed_effect") + source.slice(maybeBody.end);
    });
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes(`concrete evidence missing ${file}:${name}`))).toBe(true);
  });
}
for (const anchor of ["blocks: 8", "body_bytes: 1024 * 1024", "undo_bytes: 4 * 1024 * 1024", "cloned_bytes: 16 * 1024 * 1024",
  "script_items: 4 * 1024 * 1024", "script_bytes: 32 * 1024 * 1024", "encoded_bytes: 1024 * 1024", "record_operations: 512",
  "checkpoint_operations: 1_000_000", "projection_operations: 256", "blocks: 1", "body_bytes: 4_000_000 * 32", "undo_bytes: 256 * 1024 * 1024",
  "cloned_bytes: 1024 * 1024 * 1024", "script_items: 128 * 1024 * 1024", "script_bytes: 64 * 1024 * 1024 * 1024", "encoded_bytes: 0x0200_0000 + 170"]) {
  test(`consumed measured budget mutation: ${anchor}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, PRODUCTION_BUDGET, "production_budget", body => body.replace(pattern(anchor), "removed_budget: 999"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("production defaults"))).toBe(true);
  });
}
for (const field of ["title", "rationale", "evidence", "known_gaps", "intentional_differences"]) {
  test(`unrelated deferred prose cannot authorize a positive ${field} claim`, () => {
    // Arrange
    const root = fixture();
    edit(root, "docs/parity/index.json", source => {
      const index = JSON.parse(source);
      const row = index.checklist.surfaces.find((item: { id: string }) => item.id === SURFACE);
      const claim = "Peer filter serving is supported.";
      if (typeof row[field] === "string") row[field] += " " + claim;
      else row[field].push(claim);
      row.known_gaps.push("Unrelated products remain deferred.");
      return JSON.stringify(index);
    });
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("unsupported claim"))).toBe(true);
  });
}
for (const expected of DOCUMENTED_LIMITS) for (const column of [1, 2]) {
  test(`published ${expected[0]} column ${column} must match consumed defaults`, () => {
    // Arrange
    const root = fixture();
    edit(root, DOC, source => source.split("\n").map(line => {
      if (!line.startsWith("|") || line.split("|")[1]?.trim() !== expected[0]) return line;
      const cells = line.split("|");
      cells[column + 1] = " 999 invented units ";
      return cells.join("|");
    }).join("\n"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes(`documented production limit drift ${expected[0]}`))).toBe(true);
  });
}
test("done status refuses without earned formal and security reports", () => {
  // Arrange
  const root = fixture();
  for (const file of COMPLETION_REPORTS) {
    const path = join(root, file);
    if (existsSync(path)) unlinkSync(path);
  }
  edit(root, "docs/parity/index.json", source => {
    const index = JSON.parse(source);
    index.checklist.surfaces.find((item: { id: string }) => item.id === SURFACE).status = "done";
    return JSON.stringify(index);
  });
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures.filter(item => item.includes("completed scope requires earned"))).toHaveLength(2);
});
test("scoped done status supports lifecycle-matched verified reports", () => {
  // Arrange: synthetic report metadata exercises the gate, not runtime proof.
  const root = fixture();
  edit(root, "docs/parity/index.json", source => {
    const index = JSON.parse(source);
    index.checklist.surfaces.find((item: { id: string }) => item.id === SURFACE).status = "done";
    return JSON.stringify(index);
  });
  const identity = "phase_lifecycle_id: 157-2026-10-05T14-50-48\n";
  writeFileSync(join(root, PHASE + "157-VERIFICATION.md"), `---\n${identity}status: passed\ngenerated_by: gsd-verifier\n---\n`);
  writeFileSync(join(root, PHASE + "157-SECURITY.md"), `---\n${identity}threats_open: 0\ngenerated_by: gsd-secure-phase\n---\n`);
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures).toEqual([]);
});
test("one subject's negation cannot authorize another positive subject", () => {
  // Arrange
  const root = fixture();
  edit(root, "README.md", source => source + "\nNo claim of runtime reorg, production readiness is established.\n");
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures.some(item => item.includes("unsupported claim"))).toBe(true);
});
for (const anchor of ["impl BasicIndexMaintenance for ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>", "self.drive_basic_filter_index_turn()", ".map(Some)", ".map_err(CoinsFlushError::BasicFilter)"]) {
  test(`concrete durable timer adapter must remain: ${anchor}`, () => {
    // Arrange
    const root = fixture();
    edit(root, "packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs", source => source.replace(pattern(anchor), "removed_adapter"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("concrete durable timer adapter"))).toBe(true);
  });
}
for (const [file, modules] of REGISTRATIONS) for (const module of modules) {
  test(`module registration must remain: ${file}:${module}`, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => source.replace(module, `// ${module}`));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("registration") && item.includes(file))).toBe(true);
  });
}
for (const file of NEW_RUST) {
  test(`new Rust mapping removal refuses: ${file}`, () => {
    // Arrange
    const root = fixture();
    edit(root, "docs/parity/source-breadcrumbs.json", source => source.replace(file, "removed/path.rs"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("breadcrumb") && item.includes(file))).toBe(true);
  });
}
for (const file of CORPUS) {
  test(`pinned corpus must exist: ${file}`, () => {
    // Arrange
    const root = fixture();
    unlinkSync(join(root, file));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes(file))).toBe(true);
  });
}
for (const claim of ["Peer filter serving is supported.", "Runtime reorg is shipped.",
  "Filter/index RPC is available.", "Production readiness is established.", "Production-funds use is safe.",
  "Unattended full sync is complete.", "Hardware power-loss proof passed.", "v2.5 is shipped."]) {
  for (const file of [DOC, "README.md", "docs/parity/index.json"]) {
    test(`independent current claim refuses in ${file}: ${claim}`, () => {
      // Arrange
      const root = fixture();
      edit(root, file, source => {
        if (file.endsWith(".json")) {
          const index = JSON.parse(source);
          const row = index.checklist.surfaces.find((item: { id: string }) => item.id === SURFACE);
          row.rationale += " " + claim;
          row.known_gaps.push("Other serving and production products remain deferred.");
          return JSON.stringify(index);
        }
        return source + "\n" + claim + "\nOther products remain deferred.\n";
      });
      // Act
      const failures = checkPhase157IndexCatchUp(root);
      // Assert
      expect(failures.some(item => item.includes("unsupported claim"))).toBe(true);
    });
  }
}
for (const anchor of ["measurement_started_at:", "measurement_ended_at:", "1 registered measurement test passed", "5,415", "989,871", "not RSS", "10,000-byte", "1,000,000"]) {
  test(`executed measurement contract must remain: ${anchor}`, () => {
    // Arrange
    const root = fixture();
    edit(root, PHASE + "157-TURN-MEASUREMENTS.md", source => source.replaceAll(anchor, "removed_measurement"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("measurement"))).toBe(true);
  });
}
for (const command of ["bun test ./scripts/check-phase157-index-catch-up.test.ts", "bun run scripts/check-phase157-index-catch-up.ts"]) {
  test(`executable default verifier stage must remain: ${command}`, () => {
    // Arrange
    const root = fixture();
    edit(root, "scripts/verify.sh", source => source.replace(new RegExp(`^run_step [^\\n]+${command.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`, "m"), "# removed stage"));
    // Act
    const failures = checkPhase157IndexCatchUp(root);
    // Assert
    expect(failures.some(item => item.includes("verifier"))).toBe(true);
  });
}
test("CLI reports bounded categories and exit one on missing evidence", () => {
  // Arrange
  const root = fixture();
  unlinkSync(join(root, CORPUS[0]));
  // Act
  const result = spawnSync(process.execPath, ["run", join(root, "scripts/check-phase157-index-catch-up.ts")], { encoding: "utf8", timeout: 10_000 });
  // Assert
  expect(result.error).toBeUndefined();
  expect(result.status).toBe(1);
  expect(result.stderr).toContain("cannot read required evidence");
  expect(result.stderr.length).toBeLessThan(10_000);
});
test("outer shutdown rejects early coins error before other real joins", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs";
  mutateFunction(root, file, "settle_daemon_shutdown", body => body.replace("let results = [sync(), coins(), retry(), checkpoint(), serve_result];", "coins()?; let results = [sync(), Ok(()), retry(), checkpoint(), serve_result];"));
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures.some(item => item.includes(`${file}:settle_daemon_shutdown`))).toBe(true);
});
test("outer shutdown rejects missing real retry join despite unrelated settlement", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs";
  mutateFunction(root, file, "serve_authoritative_runtime", body => {
    const before = "maybe_retry_worker.map_or(Ok(()), |worker| Ok(worker.shutdown()?))";
    const after = "maybe_retry_worker.map_or(Ok(()), |_worker| Ok(()))";
    expect(body.split(before)).toHaveLength(2);
    expect(before).not.toBe(after);
    const changed = body.replace(before, after);
    expect(changed).not.toBe(body);
    return changed;
  });
  // Act
  const failures = checkPhase157IndexCatchUp(root);
  // Assert
  expect(failures.some(item => item.includes(`${file}:serve_authoritative_runtime`))).toBe(true);
});
for (const missing of ["packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs", DOC]) {
  test(`current startup successors retain original parity evidence owner: ${missing}`, () => {
    // Arrange
    const root = fixture();
    edit(root, "docs/parity/index.json", source => {
      const index = JSON.parse(source);
      const surface = index.checklist.surfaces.find((row: { id: string }) => row.id === SURFACE);
      expect(surface.evidence.filter((file: string) => file === missing)).toHaveLength(1);
      surface.evidence = surface.evidence.filter((file: string) => file !== missing);
      return JSON.stringify(index);
    });
    // Act / Assert
    expect(checkPhase157IndexCatchUp(root)).toContain(`T-157-29: parity evidence missing ${missing}`);
  });
}
