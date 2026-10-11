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

test("disable epoch advance alone cannot replace read-integrity revocation", () => {
  // Arrange
  const root = fixture();
  const file = NODE + "storage/fjall_store/filters/lifecycle.rs";
  const target = join(root, file);
  const source = readFileSync(target, "utf8");
  const changed = source.replace("control.invalidate_read_integrity()?;", "control.advance_integrity_epoch()?;");
  const maybeDisable = maybeRustFunction(changed, "disable_basic_filter_index_guarded", true);
  expect(maybeDisable?.body.includes("control.advance_integrity_epoch()?;")).toBe(true);
  expect(maybeDisable?.body.includes("control.invalidate_read_integrity()?;")).toBe(false);
  writeFileSync(target, changed);
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("Disabled route"))).toBe(true);
});

const recoveryOwnerProof = "let Some(owner) = self.maybe_basic_filter_owner_guarded(control)? else {";
for (const [mode, before, after] of [
  ["false state", "if self.maybe_basic_filter_state()?.is_some()", "if false"],
  ["inverted state", "if self.maybe_basic_filter_state()?.is_some()", "if self.maybe_basic_filter_state()?.is_none()"],
  ["missing proof", recoveryOwnerProof, "let Some(owner) = None else {"],
  ["premature total", recoveryOwnerProof, "control.install_retention(retained);\n" + recoveryOwnerProof],
  ["early return", recoveryOwnerProof, "return Ok(());\n" + recoveryOwnerProof],
] as const) {
  test(`state-gated recovery rejects ${mode}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, "packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs", before, after);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("PreserveSaved protection"))).toBe(true);
  });
}

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
  ["full integrity scan", NODE + "storage/fjall_store/filters/startup.rs", "self.measure_basic_filter_records(limits)?", "guard mechanism"],
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
const RETENTION = NODE + "storage/fjall_store/filters/retention.rs";
const PUBLICATION = NODE + "storage/fjall_store/filters/publication.rs";
const APPEND = NODE + "storage/fjall_store/filters/append.rs";
const FRESH_PREFLIGHT = "self.preflight_basic_filter_history(IndexInputProtection::FromHeight(0), fence)?;";
const SAVED_PREFLIGHT = "self.preflight_basic_filter_history(checkpoint.input_protection(), fence)?;";
for (const replacement of ["", "if false { control.reset_verified_forest(retained); }",
  "return Ok(()); control.reset_verified_forest(retained);", "// control.reset_verified_forest(retained);",
  'let decoy = r##"control.reset_verified_forest(retained);"##;']) {
  test(`successful disable closing reset cannot be skipped or substituted: ${replacement}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, LIFECYCLE, "control.reset_verified_forest(retained);", replacement);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("Disabled route"))).toBe(true);
  });
}
for (const mode of ["reset before validation", "guarded failure ignored"]) {
  test(`disable verified reset refuses ${mode}`, () => {
    // Arrange
    const root = fixture();
    const target = join(root, LIFECYCLE);
    const source = readFileSync(target, "utf8");
    const changed = source.replace(/pub\(crate\) fn disable_basic_filter_index\([\s\S]*?^    }/m, body => mode === "reset before validation"
      ? body.replace("self.disable_basic_filter_index_guarded(", "let retained = control.retention;\ncontrol.reset_verified_forest(retained);\nself.disable_basic_filter_index_guarded(")
      : body.replace(/(RetentionObservationLimits::default\(\),\s*\))\?;/, "$1;"));
    expect(changed).not.toBe(source);
    writeFileSync(target, changed);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("Disabled route"))).toBe(true);
  });
}
for (const [label, file, anchor, category] of [
  ["compatibility initialize policy", NODE + "chainstate/flush_lifecycle.rs", "BasicFilterStartupMode::PreserveSaved,", "compatibility route"],
  ["compatibility runtime policy", NODE + "sync/open_runtime.rs", "BasicFilterStartupMode::PreserveSaved,", "compatibility route"],
  ["configured selected policy", NODE + "sync/open_runtime.rs", "u64::MAX, basic_filter_mode", "runtime constructor"],
  ["PreserveSaved dispatch", STARTUP, "self.recover_basic_filter_index_guarded(maybe_recovered_best_block, limits, control)", "independent startup routes"],
  ["Enabled dispatch", STARTUP, "self.enable_basic_filter_index_guarded(&fence, control, limits)", "independent startup routes"],
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
  ["publication total ceiling", NODE + "storage/fjall_store/filters/retention.rs", "BASIC_INDEX_MAX_ENCODED_BYTES", "atomic publication"],
  ["publication singleton ceiling", NODE + "storage/fjall_store/filters/retention.rs", "BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES", "atomic publication"],
  ["configured default limits route", STARTUP, "self.configure_basic_filter_index_before_prune_with_retention_limits(", "configured route"],
  ["guarded dispatcher", STARTUP, ".apply_basic_filter_startup_guarded(", "configured route"],
  ["successful startup capture revocation", STARTUP, ".and_then(|()| control.advance_integrity_epoch())", "configured route"],
  ["Disabled dispatch", STARTUP, "self.disable_basic_filter_index_guarded(control, limits)", "independent startup routes"],
  ["Disabled durable invalidation", LIFECYCLE, "control.invalidate_read_integrity()?;", "Disabled route"],
  ["Disabled checkpoint validation", LIFECYCLE, "self.maybe_basic_filter_checkpoint()?;", "Disabled route"],
  ["Disabled release ordering", LIFECYCLE, "LifecyclePublicationPoint::AfterDisable", "Disabled route"],
  ["immutable scan", NODE + "storage/fjall_store/filters.rs", "for guard in self.block_index.prefix(codec::RECORD_PREFIX)", "guard mechanism"],
  ["projection scan", NODE + "storage/fjall_store/filters.rs", "for guard in self.block_index.prefix(codec::ACTIVE_PREFIX)", "guard mechanism"],
  ["shared candidate ceiling", RETENTION, "records.len() as u64 > BASIC_INDEX_MAX_CANDIDATES", "atomic publication"],
  ["shared singleton comparison", RETENTION, "envelope > BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES", "atomic publication"],
  ["shared total comparison", RETENTION, "total > BASIC_INDEX_MAX_ENCODED_BYTES", "atomic publication"],
  ["shared envelope overflow", RETENTION, ".checked_add(codec::RECORD_OVERHEAD as u64)", "atomic publication"],
  ["shared cumulative overflow", RETENTION, ".checked_add(envelope)", "atomic publication"],
  ["atomic SyncAll durability", PUBLICATION, ".durability(Some(FjallPersistMode::SyncAll))", "atomic publication"],
  ["atomic commit poison", PUBLICATION, "control.poisoned = true;", "atomic publication"],
  ["atomic fence verification", PUBLICATION, "self.verify_basic_filter_fence(fence)?;", "atomic publication"],
  ["shared poison refusal", PUBLICATION, "if guard.poisoned", "atomic publication"],
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
for (const refusal of ["BASIC append record count bound", "BASIC append singleton byte bound", "BASIC append aggregate byte bound"]) {
  test(`shared immutable admission refuses removed ${refusal} return`, () => {
    // Arrange
    const root = fixture();
    mutate(root, RETENTION, `return Err(index_corruption("${refusal}"));`, `record_bound_removed();`);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("immutable admission ceilings/refusal"))).toBe(true);
  });
}
test("shared immutable admission after early success refuses", () => {
  // Arrange
  const root = fixture();
  const statement = "if records.len() as u64 > BASIC_INDEX_MAX_CANDIDATES";
  mutate(root, RETENTION, statement, `return Ok(());\n${statement}`);
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("immutable admission ceilings/refusal"))).toBe(true);
});
for (const [file, statement, category] of [
  [LIFECYCLE, "self.measure_basic_filter_records(limits)?;", "Disabled route"],
  [LIFECYCLE, "self.maybe_basic_filter_checkpoint()?;", "Disabled route"],
] as const) {
  test(`conditional extracted startup validation refuses ${statement}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, file, statement, `if false { ${statement} }`);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes(category))).toBe(true);
  });
}
test("conditional extracted guarded dispatcher refuses", () => {
  // Arrange
  const root = fixture();
  const source = readFileSync(join(root, STARTUP), "utf8");
  const maybeCall = source.match(/let result = self\s*\.apply_basic_filter_startup_guarded\([\s\S]*?;\n/);
  if (!maybeCall) throw new Error("missing ordinary guarded startup dispatch anchor");
  mutate(root, STARTUP, maybeCall[0], `if false { ${maybeCall[0]} }`);
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("configured route") && item.includes("cannot be conditional"))).toBe(true);
});
test("configured guarded dispatcher after early success refuses", () => {
  // Arrange
  const root = fixture();
  const statement = "let result = self";
  mutate(root, STARTUP, statement, `return Ok(());\n${statement}`);
  // Act
  const failures = checkPhase155FilterIndex(root);
  // Assert
  expect(failures.some(item => item.includes("configured route") && item.includes("bypassed"))).toBe(true);
});
for (const [file, call, allocation] of [
  [PUBLICATION, "super::retention::check_envelope_bounds(records)?;", "let mut encoded = Vec::with_capacity(records.len());"],
  [APPEND, "check_envelope_bounds(records)?;", "let mut prepared = PreparedBasicFilterAppend {"],
] as const) {
  for (const substitute of ["", `if false { ${call} }`, `return Ok(Default::default()); ${call}`,
    `// ${call}`, `let decoy = r##"${call}"##;`]) {
    test(`immutable admission bypass refuses ${file}: ${substitute.slice(0, 24)}`, () => {
      // Arrange
      const root = fixture();
      mutate(root, file, call, substitute);
      // Act
      const failures = checkPhase155FilterIndex(root);
      // Assert
      expect(failures.some(item => item.includes("mandatory immutable admission"))).toBe(true);
    });
  }
  test(`immutable admission after allocation refuses ${file}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, file, call, "");
    mutate(root, file, allocation, `${allocation}\n${call}`);
    // Act
    const failures = checkPhase155FilterIndex(root);
    // Assert
    expect(failures.some(item => item.includes("mandatory immutable admission"))).toBe(true);
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
    const source = readFileSync(join(root, LIFECYCLE), "utf8");
    const maybeReturn = [...source.matchAll(/return self\.install_recovered_basic_filter_append_guarded\([\s\S]*?\);/g)]
      .find(match => preflight === FRESH_PREFLIGHT || match[0].includes(", retained)"));
    if (!maybeReturn) throw new Error("missing ordinary recovered append return anchor");
    const returnCall = maybeReturn[0];
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
    [STARTUP, "recover_basic_filter_index_guarded", "PreserveSaved protection"],
    [LIFECYCLE, "enable_basic_filter_index_guarded", "Enabled suffix preflight"],
    [STARTUP, "configure_basic_filter_index_before_prune_with_retention_limits", "configured route"],
    [LIFECYCLE, "disable_basic_filter_index_guarded", "Disabled route"],
    [RETENTION, "check_envelope_bounds", "atomic publication"],
    [PUBLICATION, "finish_basic_filter_batch", "atomic publication"],
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
