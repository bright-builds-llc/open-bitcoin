import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { CHECK_FILES, checkPhase155FilterIndex } from "./check-phase155-filter-index.ts";
import { maybeRustFunction } from "./check-phase156-prune-coordination/rust-evidence.ts";

const REPO = resolve(import.meta.dir, "..");
const roots: string[] = [];
const NODE = "packages/open-bitcoin-node/src/";
const CORE = "packages/open-bitcoin-chainstate/src/";
const DOC = "docs/parity/catalog/basic-compact-filters.md";
const SURFACE = "v2-5-recoverable-basic-index-and-startup-protection";
const DISPATCH = `match basic_filter_mode {
        BasicFilterStartupMode::PreserveSaved => {
            store.recover_basic_filter_index_before_prune(maybe_best_block)?
        }
        mode => store.configure_basic_filter_index_before_prune(maybe_best_block, mode)?,
    }`;
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "filter-index-evidence-"));
  roots.push(root);
  for (const file of CHECK_FILES) {
    mkdirSync(dirname(join(root, file)), { recursive: true });
    writeFileSync(join(root, file), readFileSync(join(REPO, file)));
  }
  return root;
}
function mutate(root: string, file: string, before: string, after = "removed_evidence"): void {
  const target = join(root, file);
  const source = readFileSync(target, "utf8");
  expect(source.includes(before)).toBe(true);
  writeFileSync(target, source.replaceAll(before, after));
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

test("fixed evidence passes and check mode preserves all bytes", () => {
  // Arrange
  const root = fixture();
  const before = CHECK_FILES.map(file => readFileSync(join(root, file), "utf8"));
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures).toEqual([]);
  expect(CHECK_FILES.map(file => readFileSync(join(root, file), "utf8"))).toEqual(before);
});
for (const file of [CORE + "filter_index/recovery.rs", NODE + "storage/fjall_store/filters/publication.rs",
  NODE + "sync/tests/filter_index/recovery.rs", NODE + "sync/tests/filter_index/faults.rs",
  "scripts/check-phase155-filter-index.test.ts"]) {
  test(`missing evidence file refuses: ${file}`, () => {
    // Arrange
    const root = fixture();
    unlinkSync(join(root, file));
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes(`${file}: cannot read`))).toBe(true);
  });
}
for (const [label, file, before, category] of [
  ["coins recovery", NODE + "chainstate/flush_lifecycle.rs", "let recovered = apply_recovery_decision", "startup ordering"],
  ["mandatory guard", NODE + "chainstate/flush_lifecycle.rs", DISPATCH, "startup ordering"],
  ["runtime constructor", NODE + "sync/open_runtime.rs", "initialize_configured(&store", "runtime constructor"],
  ["managed constructor", NODE + "sync/open_runtime.rs", "ManagedChainstate::from_recovered_chainstate", "runtime constructor"],
  ["direct intent protection", NODE + "storage/fjall_store/filters/startup.rs", ".check_prune_intent(intent.height)", "guard mechanism"],
  ["full integrity scan", NODE + "storage/fjall_store/filters/startup.rs", "self.validate_basic_filter_records()?;", "guard mechanism"],
  ["pre-mutation protection", NODE + "storage/fjall_store/filters/startup.rs", "self.maybe_prune_intent()?", "guard ordering"],
  ["real reopen", NODE + "sync/tests/filter_index/recovery.rs", "DurableSyncRuntime::open(", "validated history"],
  ["actual validation", NODE + "sync/tests/filter_index/recovery.rs", ".stage_connect_block_with_current_time(", "validated history"],
  ["historical undo", NODE + "sync/tests/filter_index/recovery.rs", "BasicFilterInputs::from_historical", "validated history"],
  ["actual coins fault", NODE + "sync/tests/filter_index/faults.rs", ".batch_write_with_limit(", "fault evidence"],
  ["actual metadata fault", NODE + "sync/tests/filter_index/faults.rs", "lifecycle.execute_flush(", "fault evidence"],
  ["before record fault", NODE + "sync/tests/filter_index/faults.rs", "FilterPublicationFault::BeforeRecords", "fault evidence"],
  ["before checkpoint fault", NODE + "sync/tests/filter_index/faults.rs", "FilterPublicationFault::BeforeCheckpoint", "fault evidence"],
  ["before protection fault", NODE + "sync/tests/filter_index/faults.rs", "FilterPublicationFault::BeforeProtection", "fault evidence"],
  ["after commit fault", NODE + "sync/tests/filter_index/faults.rs", "FilterPublicationFault::AfterCommit", "fault evidence"],
  ["bounded scan corpus", NODE + "storage/fjall_store/filters/tests/faults.rs", "4 * records.len() - 1", "linear scan evidence"],
  ["schema 2", NODE + "storage.rs", "pub const CURRENT: Self = Self(2);", "schema 2 compatibility"],
  ["same-height conflict", NODE + "sync/tests/filter_index/startup/recovery.rs", "filter_index_production_reopen_refuses_same_height_saved_fence_conflict", "runtime matrix"],
  ["ahead proof", NODE + "sync/tests/filter_index/recovery.rs", "filter_index_production_validated_ahead_records_do_not_mint_cursor", "validated history"],
  ["fork proof", NODE + "sync/tests/filter_index/recovery.rs", "filter_index_production_validated_equal_height_fork_retains_common_prefix", "validated history"],
  ["missing prefix proof", NODE + "sync/tests/filter_index/recovery.rs", "filter_index_production_validated_missing_prefix_preserves_history_and_intent", "validated history"],
  ["post-record commit proof", NODE + "sync/tests/filter_index/faults.rs", "filter_index_production_after_record_commit_error_retains_ahead_rows_without_cursor", "fault evidence"],
  ["unique parity owner", "docs/parity/index.json", SURFACE, "surface owner"],
  ["scope caveat", DOC, "159–162 own RPC/peer/operator serving", "scoped documentation"],
  ["bounded memory caveat", DOC, "not a total runtime/memory cap", "scoped documentation"],
  ["hardware caveat", DOC, "hardware power-loss simulation", "scoped documentation"],
  ["executable checker", "scripts/verify.sh", 'run_step "check Phase 155 recoverable filter index evidence"', "default verifier"],
  ["executable tests", "scripts/verify.sh", 'run_step "test Phase 155 recoverable filter index checker"', "default verifier"],
] as const) {
  test(`mutation refuses missing ${label}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, file, before);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes(category))).toBe(true);
  });
}
for (const after of ["resume_prune_intent(store, &locks)?;", "lifecycle.readiness = ManagerReadiness::ReadyToFlush;",
  "let cache = CoinsCache::from_parent(recovered);"]) {
  test(`guard moved after ${after} refuses`, () => {
    // Arrange
    const root = fixture();
    const file = NODE + "chainstate/flush_lifecycle.rs";
    const call = DISPATCH;
    mutate(root, file, call, "");
    mutate(root, file, after, `${after}\n    ${call}`);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("startup ordering"))).toBe(true);
  });
}
test("conditional initialize guard refuses", () => {
  // Arrange
  const root = fixture();
  const call = DISPATCH;
  mutate(root, NODE + "chainstate/flush_lifecycle.rs", call, `if enabled { ${call} }`);
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("mandatory guard"))).toBe(true);
});

const STARTUP = NODE + "storage/fjall_store/filters/startup.rs";
const LIFECYCLE = NODE + "storage/fjall_store/filters/lifecycle.rs";
const FRESH_PREFLIGHT = "self.preflight_basic_filter_history(IndexInputProtection::FromHeight(0), fence)?;";
const SAVED_PREFLIGHT = "self.preflight_basic_filter_history(checkpoint.input_protection(), fence)?;";
for (const [label, file, anchor, category] of [
  ["compatibility initialize policy", NODE + "chainstate/flush_lifecycle.rs", "BasicFilterStartupMode::PreserveSaved,", "compatibility route"],
  ["compatibility runtime policy", NODE + "sync/open_runtime.rs", "BasicFilterStartupMode::PreserveSaved,", "compatibility route"],
  ["configured selected policy", NODE + "sync/open_runtime.rs", "u64::MAX, basic_filter_mode", "runtime constructor"],
  ["PreserveSaved dispatch", STARTUP, "self.recover_basic_filter_index(maybe_recovered_best_block)", "independent startup routes"],
  ["Enabled dispatch", STARTUP, "self.enable_basic_filter_index(&fence)", "independent startup routes"],
  ["Enabled recovered fence", STARTUP, "VerifiedChainstateFence::new(maybe_recovered_best_block, Some(&positions))", "independent startup routes"],
  ["PreserveSaved inspection", STARTUP, "self.inspect_basic_filter_recovery(owner, &fence)?;", "PreserveSaved protection"],
  ["PreserveSaved recovered prefix", STARTUP, "self.scan_basic_filter_checkpoint(saved, owner.saved_protection(), fence)?;", "PreserveSaved retained protection"],
  ["PreserveSaved stronger protection", STARTUP, "retained.covers(protection)", "PreserveSaved retained protection"],
  ["fresh full suffix", LIFECYCLE, FRESH_PREFLIGHT, "Enabled suffix preflight"],
  ["saved required suffix", LIFECYCLE, SAVED_PREFLIGHT, "Enabled suffix preflight"],
  ["Enabled inspection", LIFECYCLE, "self.inspect_basic_filter_recovery(owner, fence)?;", "Enabled suffix preflight"],
  ["suffix body", LIFECYCLE, "self.load_block(position.block_hash)?", "required suffix validation"],
  ["suffix undo", LIFECYCLE, "self.load_undo(position.block_hash)?", "required suffix validation"],
  ["suffix bound authority", LIFECYCLE, "BasicFilterInputs::from_historical(&block, position, maybe_history)", "required suffix validation"],
  ["publication block ceiling", NODE + "storage/fjall_store/filters/publication.rs", "BASIC_INDEX_MAX_CANDIDATES", "atomic publication"],
  ["publication total ceiling", NODE + "storage/fjall_store/filters/publication.rs", "BASIC_INDEX_MAX_ENCODED_BYTES", "atomic publication"],
  ["publication singleton ceiling", NODE + "storage/fjall_store/filters/publication.rs", "BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES", "atomic publication"],
] as const) {
  test(`independent route refuses missing ${label}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, file, anchor);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes(category))).toBe(true);
  });
}
for (const preflight of [FRESH_PREFLIGHT, SAVED_PREFLIGHT]) {
  test(`conditional suffix preflight refuses: ${preflight}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, LIFECYCLE, preflight, `if false { ${preflight} }`);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("mandatory validation"))).toBe(true);
  });
  test(`suffix preflight after Active return refuses: ${preflight}`, () => {
    // Arrange
    const root = fixture();
    const returnCall = "return self.install_recovered_basic_filter_append_guarded(fence, &mut control);";
    mutate(root, LIFECYCLE, preflight, "");
    mutate(root, LIFECYCLE, returnCall, `${returnCall}\n${preflight}`);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("Enabled suffix preflight"))).toBe(true);
  });
}
for (const [file, statement, category] of [
  [STARTUP, "self.inspect_basic_filter_recovery(owner, &fence)?;", "PreserveSaved protection"],
  [LIFECYCLE, "self.load_block(position.block_hash)?", "required suffix validation"],
  [LIFECYCLE, "self.load_undo(position.block_hash)?", "required suffix validation"],
  [LIFECYCLE, "BasicFilterInputs::from_historical(&block, position, maybe_history)", "required suffix validation"],
] as const) {
  test(`conditional recovery or suffix input bypass refuses: ${statement}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, file, statement, `if false { ${statement} }`);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes(category) && item.includes("cannot be conditional"))).toBe(true);
  });
}
for (const substitute of [
  `// ${DISPATCH}`, `/* ${DISPATCH} */`, `let decoy = "${DISPATCH}";`,
  `let decoy = r##"${DISPATCH}"##;`,
]) {
  test(`comment or literal cannot substitute configured dispatch: ${substitute.slice(0, 20)}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, NODE + "chainstate/flush_lifecycle.rs", DISPATCH, substitute);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("startup ordering"))).toBe(true);
  });
}
for (const declaration of ["mod", "pub mod", "pub(crate) mod"]) {
  for (const [file, symbol, category] of [
    [NODE + "chainstate/flush_lifecycle.rs", "initialize_configured", "startup ordering"],
    [STARTUP, "recover_basic_filter_index", "PreserveSaved protection"],
    [LIFECYCLE, "enable_basic_filter_index", "Enabled suffix preflight"],
  ] as const) {
    test(`test-only ${declaration} cannot substitute ${symbol}`, () => {
      // Arrange
      const root = fixture();
      const target = join(root, file);
      const source = readFileSync(target, "utf8");
      const maybeFunction = maybeRustFunction(source, symbol, true);
      expect(maybeFunction).toBeDefined();
      mutate(root, file, `fn ${symbol}(`, `fn removed_${symbol}(`);
      writeFileSync(target, readFileSync(target, "utf8")
        + `\n#[cfg(test)] ${declaration} decoy { fn ${symbol}() { ${maybeFunction?.body} } }\n`);
      // Act
      const failures = checkPhase155FilterIndex(root);
      // Assert
      expect(failures.some(item => item.includes(category))).toBe(true);
    });
  }
}
test("ordinary source ignores comment, raw literal and qualified test-only decoys", () => {
  // Arrange
  const root = fixture();
  const target = join(root, NODE + "sync/open_runtime.rs");
  const decoy = "fn open_with_configured_runtime_activation() { obsolete_constructor(); }";
  writeFileSync(target, readFileSync(target, "utf8")
    + `\n// ${decoy}\nconst DECOY: &str = r##"${decoy}"##;\n#[cfg(test)] pub(crate) mod decoy { ${decoy} }\n`);
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures).toEqual([]);
});
test("breadcrumb mapping removal refuses", () => {
  // Arrange
  const root = fixture();
  mutate(root, "docs/parity/source-breadcrumbs.json", NODE + "sync/tests/filter_index/faults.rs");
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("breadcrumb manifest"))).toBe(true);
});
test("duplicate requirement owner refuses", () => {
  // Arrange
  const root = fixture();
  const file = join(root, "docs/parity/index.json");
  const index = JSON.parse(readFileSync(file, "utf8"));
  index.checklist.surfaces.push({ id: "duplicate", requirements: ["CFIX-02"] });
  writeFileSync(file, JSON.stringify(index));
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("CFIX-02: wrong surface owner"))).toBe(true);
});
test("parity source anchor removal refuses", () => {
  // Arrange
  const root = fixture();
  const file = join(root, "docs/parity/index.json");
  const index = JSON.parse(readFileSync(file, "utf8"));
  index.checklist.surfaces.find((row: { id: string }) => row.id === SURFACE).upstream.sources = [];
  writeFileSync(file, JSON.stringify(index));
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("source anchor"))).toBe(true);
});
test("production dependency expansion refuses", () => {
  // Arrange
  const root = fixture();
  const file = join(root, "packages/open-bitcoin-chainstate/Cargo.toml");
  writeFileSync(file, readFileSync(file, "utf8") + '\nfjall = "3"\n');
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("dependency expansion"))).toBe(true);
});
test("I/O in pure recovery refuses", () => {
  // Arrange
  const root = fixture();
  const file = join(root, CORE + "filter_index/recovery.rs");
  writeFileSync(file, readFileSync(file, "utf8") + "\nuse std::fs;\n");
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("pure policy"))).toBe(true);
});
test("CLI refuses incomplete evidence with exit one and actionable diagnostics", () => {
  // Arrange
  const root = fixture();
  const file = "scripts/check-phase155-filter-index.ts";
  writeFileSync(join(root, file), readFileSync(join(REPO, file)));
  unlinkSync(join(root, CORE + "filter_index/recovery.rs"));
  // Act
  const result = spawnSync(process.execPath, ["run", join(root, file)], { encoding: "utf8", timeout: 10_000 });
  // Assert
  expect(result.error).toBeUndefined();
  expect(result.status).toBe(1);
  expect(result.stderr).toContain("cannot read required evidence");
});
for (const claim of ["Filter index activation is enabled.", "Scheduled catch-up is supported.",
  "RPC serving is available.", "Peer serving is enabled.", "Runtime reorg is shipped.",
  "Prune ownership is active.", "Production-funds use is safe.", "Hardware power-loss proof is available."]) {
  test(`unsupported scope claim refuses: ${claim}`, () => {
    // Arrange
    const root = fixture();
    const file = join(root, DOC);
    writeFileSync(file, readFileSync(file, "utf8") + `\n${claim}\n`);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("unsupported claim"))).toBe(true);
  });
}
