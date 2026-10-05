import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { CHECK_FILES, CONTRACTS, DOC, SURFACE, checkPhase156PruneCoordination } from "./check-phase156-prune-coordination.ts";

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
