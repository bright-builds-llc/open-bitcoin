import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { CHECK_FILES, CONTRACTS, DOC, SURFACE, checkPhase156PruneCoordination } from "./check-phase156-prune-coordination.ts";
import { maybeRustFunction } from "./check-phase156-prune-coordination/rust-evidence.ts";

const REPO = resolve(import.meta.dir, "..");
const roots: string[] = [];
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "prune-coordination-evidence-"));
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
function append(root: string, file: string, added: string): void {
  const target = join(root, file);
  writeFileSync(target, readFileSync(target, "utf8") + added);
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

const STARTUP = "packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs";
const LIFECYCLE = "packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs";
const FRESH_PREFLIGHT = "self.preflight_basic_filter_history(IndexInputProtection::FromHeight(0), fence)?;";
const SAVED_PREFLIGHT = "self.preflight_basic_filter_history(checkpoint.input_protection(), fence)?;";

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
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("T-156-06"))).toBe(true);
  });
}

test("disable epoch advance alone cannot replace read-integrity revocation", () => {
  // Arrange
  const root = fixture();
  const target = join(root, LIFECYCLE);
  const source = readFileSync(target, "utf8");
  const changed = source.replace("control.invalidate_read_integrity()?;", "control.advance_integrity_epoch()?;");
  const maybeDisable = maybeRustFunction(changed, "disable_basic_filter_index_guarded", true);
  expect(maybeDisable?.body.includes("control.advance_integrity_epoch()?;")).toBe(true);
  expect(maybeDisable?.body.includes("control.invalidate_read_integrity()?;")).toBe(false);
  writeFileSync(target, changed);
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("T-156-08"))).toBe(true);
});
for (const mode of ["removed", "conditional", "after return", "before validation", "ignored failure", "comment", "raw literal"]) {
  test(`public disable verified reset refuses ${mode}`, () => {
    // Arrange
    const root = fixture();
    const target = join(root, LIFECYCLE);
    const source = readFileSync(target, "utf8");
    const reset = "control.reset_verified_forest(retained);";
    const changed = source.replace(/pub\(crate\) fn disable_basic_filter_index\([\s\S]*?^    }/m, body => {
      if (mode === "before validation") return body.replace("self.disable_basic_filter_index_guarded(", `let retained = control.retention;\n${reset}\nself.disable_basic_filter_index_guarded(`);
      if (mode === "ignored failure") return body.replace(/(RetentionObservationLimits::default\(\),\s*\))\?;/, "$1;");
      const replacement = mode === "removed" ? "" : mode === "conditional" ? `if false { ${reset} }`
        : mode === "after return" ? `return Ok(()); ${reset}` : mode === "comment" ? `// ${reset}` : `let decoy = r##"${reset}"##;`;
      return body.replace(reset, replacement);
    });
    expect(changed).not.toBe(source);
    writeFileSync(target, changed);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("T-156-08") || item.includes("production boundary"))).toBe(true);
  });
}
for (const [file, statement, category] of [
  [STARTUP, "self.inspect_basic_filter_recovery(owner, &fence)?;", "T-156-06"],
  [STARTUP, "self.measure_basic_filter_records(limits)?", "T-156-06"],
  [STARTUP, "self.filter_publication_guard()?;", "T-156-06"],
  [LIFECYCLE, "self.maybe_basic_filter_checkpoint()?;", "T-156-08"],
  [LIFECYCLE, FRESH_PREFLIGHT, "T-156-10"],
  [LIFECYCLE, SAVED_PREFLIGHT, "T-156-10"],
  [LIFECYCLE, "self.load_block(position.block_hash)?", "T-156-09"],
  [LIFECYCLE, "self.load_undo(position.block_hash)?", "T-156-09"],
] as const) {
  test(`extracted mandatory validation cannot be conditional: ${statement}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, file, statement, `if false { ${statement} }`);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes(category) && item.includes("cannot be conditional"))).toBe(true);
  });
}
for (const statement of ["owner.lifecycle().disable()", "locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK)",
  "self.measure_basic_filter_records(limits)?;", ".check_prune_intent(intent.height)"]) {
  test(`owned lifecycle protection/refusal remains required: ${statement}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, LIFECYCLE, statement);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("T-156-08") || item.includes("T-156-10"))).toBe(true);
  });
}
for (const preflight of [FRESH_PREFLIGHT, SAVED_PREFLIGHT]) {
  test(`extracted suffix preflight after corresponding Active return refuses: ${preflight}`, () => {
    // Arrange
    const root = fixture();
    const source = readFileSync(join(root, LIFECYCLE), "utf8");
    const maybeReturn = [...source.matchAll(/return self\.install_recovered_basic_filter_append_guarded\([\s\S]*?\);/g)]
      .find(match => preflight === FRESH_PREFLIGHT || match[0].includes(", retained)"));
    if (!maybeReturn) throw new Error("missing actual recovered append return");
    mutate(root, LIFECYCLE, preflight, "");
    mutate(root, LIFECYCLE, maybeReturn[0], `${maybeReturn[0]}\n${preflight}`);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("T-156-10"))).toBe(true);
  });
}
for (const [file, symbol, category] of [
  [STARTUP, "configure_basic_filter_index_before_prune_with_retention_limits", "production boundary"],
  [STARTUP, "recover_basic_filter_index_guarded", "T-156-06"],
  [LIFECYCLE, "enable_basic_filter_index_guarded", "T-156-10"],
  [LIFECYCLE, "disable_basic_filter_index_guarded", "T-156-08"],
] as const) {
  for (const substitute of ["comment", "raw literal", "test-only"]) {
    test(`ordinary extracted ${symbol} cannot be substituted by ${substitute}`, () => {
      // Arrange
      const root = fixture();
      const source = readFileSync(join(root, file), "utf8");
      const maybeFunction = maybeRustFunction(source, symbol, true);
      if (!maybeFunction) throw new Error("missing ordinary guarded fixture function");
      mutate(root, file, `fn ${symbol}(`, `fn removed_${symbol}(`);
      const decoy = `fn ${symbol}() { ${maybeFunction.body} }`;
      append(root, file, substitute === "comment" ? `\n/* ${decoy} */\n`
        : substitute === "raw literal" ? `\nconst DECOY: &str = r##"${decoy}"##;\n`
          : `\n#[cfg(test)] pub(crate) mod counterfeit { ${decoy} }\n`);
      // Act
      const failures = checkPhase156PruneCoordination(root);
      // Assert
      expect(failures.some(item => item.includes(category) && item.includes(symbol))).toBe(true);
    });
  }
}
for (const [symbol, statement] of [
  ["enable", "self.enable_basic_filter_index_guarded("],
  ["disable", "self.disable_basic_filter_index_guarded("],
] as const) {
  test(`public ${symbol} wrapper cannot succeed before guarded helper`, () => {
    // Arrange
    const root = fixture();
    mutate(root, LIFECYCLE, statement, `return Ok(());\n${statement}`);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("cannot be conditional or bypassed"))).toBe(true);
  });
}
for (const symbol of ["enable", "disable"]) {
  test(`public ${symbol} wrapper cannot conditionally invoke its guarded helper`, () => {
    // Arrange
    const root = fixture();
    const source = readFileSync(join(root, LIFECYCLE), "utf8");
    const expression = symbol === "enable"
      ? /self\.enable_basic_filter_index_guarded\(\s*fence,\s*&mut control,\s*RetentionObservationLimits::default\(\),\s*\)/
      : /self\.disable_basic_filter_index_guarded\(\s*&mut control,\s*RetentionObservationLimits::default\(\),\s*\)\?;/;
    const maybeCall = source.match(expression);
    if (!maybeCall) throw new Error("missing actual public wrapper call");
    mutate(root, LIFECYCLE, maybeCall[0], `if false { ${maybeCall[0]} }`);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("cannot be conditional or bypassed"))).toBe(true);
  });
}

test("current evidence passes without changing fixture bytes", () => {
  // Arrange
  const root = fixture();
  const before = CHECK_FILES.map(file => readFileSync(join(root, file), "utf8"));
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures).toEqual([]);
  expect(CHECK_FILES.map(file => readFileSync(join(root, file), "utf8"))).toEqual(before);
});
for (const contract of CONTRACTS) {
  test(`${contract.threat}: removing concrete production contract refuses`, () => {
    // Arrange
    const root = fixture();
    mutate(root, contract.file, contract.anchors[0]);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes(contract.threat))).toBe(true);
  });
  test(`${contract.threat}: removing named behavioral test refuses`, () => {
    // Arrange
    const root = fixture();
    mutate(root, contract.testFile, `fn ${contract.testName}(`);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes(contract.threat))).toBe(true);
  });
}
for (const replacement of [
  "// self.check_basic_filter_work_guarded(work, fence, &control)?;",
  'let _diagnostic = "self.check_basic_filter_work_guarded(work, fence, &control)?;";',
  'let _diagnostic = r#"self.check_basic_filter_work_guarded(work, fence, &control)?;"#;',
]) {
  test(`comment/string cannot substitute for record-only production work gate: ${replacement}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, "packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs",
      "self.check_basic_filter_work_guarded(work, fence, &control)?;", replacement);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("T-156-04"))).toBe(true);
  });
}
test("test-only function cannot replace ordinary deletion caller", () => {
  // Arrange
  const root = fixture();
  mutate(root, "packages/open-bitcoin-node/src/storage/fjall_store/prune.rs",
    "    fn commit_paired_delete_observing(", "    #[cfg(test)]\n    fn commit_paired_delete_observing(");
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("T-156-15"))).toBe(true);
});
test("inline test-only module cannot supply a missing ordinary deletion body", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-node/src/storage/fjall_store/prune.rs";
  mutate(root, file, "fn commit_paired_delete_observing(", "fn renamed_delete_observing(");
  append(root, file, `\n#[cfg(test)] mod counterfeit {
    fn unrelated() {}
    fn commit_paired_delete_observing() {
      let control = self.filter_publication_guard()?;
      self.check_prune_candidate_guarded(&control, intent)?;
      self.with_payload_mutation(|| self.commit_paired_delete_guarded(height, hash))
    }
  }\n`);
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("T-156-15"))).toBe(true);
});
for (const visibility of ["pub(crate)", "pub(super)", "pub(in crate::storage)"]) {
  test(`qualified test-only module cannot supply ordinary deletion: ${visibility}`, () => {
    // Arrange
    const root = fixture();
    const file = "packages/open-bitcoin-node/src/storage/fjall_store/prune.rs";
    mutate(root, file, "fn commit_paired_delete_observing(", "fn renamed_delete_observing(");
    append(root, file, `\n#[cfg(test)] ${visibility} mod counterfeit {
      fn unrelated() {}
      fn commit_paired_delete_observing() {
        let control = self.filter_publication_guard()?;
        self.check_prune_candidate_guarded(&control, intent)?;
        self.with_payload_mutation(|| self.commit_paired_delete_guarded(height, hash))
      }
    }\n`);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("T-156-15"))).toBe(true);
  });
}
test("wrapping generation arithmetic cannot replace checked transition", () => {
  // Arrange
  const root = fixture();
  mutate(root, "packages/open-bitcoin-chainstate/src/filter_index/lifecycle.rs", "checked_add(1)", "wrapping_add(1)");
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("production boundary") && item.includes(":next"))).toBe(true);
});
for (const claim of [
  "Public filter index activation is enabled.", "Scheduled catch-up is supported.",
  "Filter RPC serving is available.", "Peer serving is enabled.", "Runtime reorg is shipped.",
  "Operator projections are available.", "Complete client-after-prune proof is available.",
  "Production readiness is established.", "Production-funds use is safe.",
  "The full native verifier passed.", "Hardware power-loss proof is available.",
]) {
  test(`T-156-26: unqualified current claim refuses: ${claim}`, () => {
    // Arrange
    const root = fixture();
    append(root, DOC, `\n${claim}\n`);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("T-156-26: unsupported current claim"))).toBe(true);
  });
}
test("historical evidence and explicit future/deferred exclusions remain valid", () => {
  // Arrange
  const root = fixture();
  append(root, DOC, "\nPhase 155's full native verifier passed in its historical gate.\nPublic filter activation is not yet enabled.\nScheduled catch-up and serving remain deferred to future phases.\nSoftware faults do not establish hardware power-loss proof.\n");
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures).toEqual([]);
});
test("deferred clause cannot excuse a contradictory current activation claim", () => {
  // Arrange
  const root = fixture();
  append(root, DOC, "\nScheduled catch-up remains deferred, but public filter activation is enabled.\n");
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("T-156-26: unsupported current claim"))).toBe(true);
});
test("parity rationale activation claim cannot borrow a known-gap exclusion", () => {
  // Arrange
  const root = fixture();
  const file = join(root, "docs/parity/index.json");
  const index = JSON.parse(readFileSync(file, "utf8"));
  index.checklist.surfaces.find((row: { id: string }) => row.id === SURFACE).rationale = "Public filter index activation is enabled.";
  writeFileSync(file, JSON.stringify(index));
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("T-156-26: unsupported current claim"))).toBe(true);
});
test("positive activation claim cannot borrow a different deferred subject", () => {
  // Arrange
  const root = fixture();
  append(root, DOC, "\nPublic filter index activation is enabled and scheduled catch-up remain deferred.\n");
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("T-156-26: unsupported current claim"))).toBe(true);
});
for (const command of ["bun test ./scripts/check-phase156-prune-coordination.test.ts",
  "bun run scripts/check-phase156-prune-coordination.ts"]) {
  test(`T-156-27: removing executable native step refuses: ${command}`, () => {
    // Arrange
    const root = fixture();
    mutate(root, "scripts/verify.sh", command);
    // Act
    const failures = checkPhase156PruneCoordination(root);
    // Assert
    expect(failures.some(item => item.includes("T-156-27: default verifier"))).toBe(true);
  });
}
test("unique CFPR-01 parity owner is required", () => {
  // Arrange
  const root = fixture();
  const file = join(root, "docs/parity/index.json");
  const index = JSON.parse(readFileSync(file, "utf8"));
  index.checklist.surfaces.push({ id: "duplicate", requirements: ["CFPR-01"] });
  writeFileSync(file, JSON.stringify(index));
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("CFPR-01: wrong surface owner"))).toBe(true);
});
test("pinned parity root removal refuses", () => {
  // Arrange
  const root = fixture();
  const file = join(root, "docs/parity/index.json");
  const index = JSON.parse(readFileSync(file, "utf8"));
  index.checklist.surfaces.find((row: { id: string }) => row.id === SURFACE).upstream.sources = [];
  writeFileSync(file, JSON.stringify(index));
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("parity source anchor"))).toBe(true);
});
test("pinned functional test root removal refuses", () => {
  // Arrange
  const root = fixture();
  const file = join(root, "docs/parity/index.json");
  const index = JSON.parse(readFileSync(file, "utf8"));
  index.checklist.surfaces.find((row: { id: string }) => row.id === SURFACE).upstream.tests = [];
  writeFileSync(file, JSON.stringify(index));
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("parity test anchor"))).toBe(true);
});
test("nonexistent cited baseline test file refuses", () => {
  // Arrange
  const root = fixture();
  unlinkSync(join(root, "packages/bitcoin-knots/test/functional/feature_index_prune.py"));
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("feature_index_prune.py: cannot read required evidence"))).toBe(true);
});
test("breadcrumb mapping removal refuses", () => {
  // Arrange
  const root = fixture();
  mutate(root, "docs/parity/source-breadcrumbs.json", "packages/open-bitcoin-node/src/sync/tests/filter_index/prune_faults.rs");
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("breadcrumb manifest"))).toBe(true);
});
test("unregistered test module cannot count as runtime evidence", () => {
  // Arrange
  const root = fixture();
  mutate(root, "packages/open-bitcoin-node/src/sync/tests/filter_index.rs", "mod prune_faults;");
  // Act
  const failures = checkPhase156PruneCoordination(root);
  // Assert
  expect(failures.some(item => item.includes("test registration"))).toBe(true);
});
test("CLI rejects absent evidence with bounded path/category diagnostics", () => {
  // Arrange
  const root = fixture();
  unlinkSync(join(root, "packages/open-bitcoin-node/src/storage/fjall_store/prune.rs"));
  // Act
  const result = spawnSync(process.execPath, ["run", join(root, "scripts/check-phase156-prune-coordination.ts")],
    { encoding: "utf8", timeout: 10_000 });
  // Assert
  expect(result.error).toBeUndefined();
  expect(result.status).toBe(1);
  expect(result.stderr).toContain("cannot read required evidence");
  expect(result.stderr.length).toBeLessThan(16_384);
});
