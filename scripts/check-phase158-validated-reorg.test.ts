import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { historicalFixtureView } from "./check-phase164-canonical-identity/fixture-source.ts";
import { PHASE162, PHASE162_DOC, PHASE162_IMPLEMENTATION, PHASE162_COMPLETE, PHASE162_EVIDENCE, checkPhase162ClaimState, loadPhase162OptionalSources } from "./check-phase158-validated-reorg.ts";
import { CHECK_FILES, CHECKPOINT_CONTRACTS, CONTRACTS, DOC, PHASE, REGISTRATIONS, SURFACE, TESTS, checkPhase158ValidatedReorg, maybeBoundary } from "./check-phase158-validated-reorg.ts";
import { PHASE163, PHASE163_DOC, V25_REQUIREMENTS, V25_ROADMAPS, PROVENANCE_FILES, PROVENANCE_CLAIMS, ORIGINAL_AUDITS, CLOSURE_CLAIM, READINESS_CLAIM, ARCHIVED_CLAIM, GAP_CLAIM } from "./check-phase163-provenance.ts";
import { v25FixtureSources } from "./check-phase163-provenance/fixture-controls.ts";

const REPO = resolve(import.meta.dir, "..");
const roots: string[] = [];

function transitionSources(maybeBase?: ReadonlyMap<string, string>): Map<string, string> {
  const historicalFiles = new Set([...CHECK_FILES, ...PHASE162_EVIDENCE, ".planning/REQUIREMENTS.md", ".planning/ROADMAP.md"]);
  const selected = v25FixtureSources(maybeBase ?? currentSources());
  const sources = new Map([...selected].filter(([file]) => historicalFiles.has(file)));
  for (const [file, source] of sources) if (file.endsWith(".md") || file.endsWith(".json")) sources.set(file, PROVENANCE_CLAIMS.reduce((view, claim) => view.replaceAll(claim, ""), source.replaceAll(PHASE162_COMPLETE, PHASE162_IMPLEMENTATION)));
  sources.set(".planning/ROADMAP.md", sources.get(".planning/ROADMAP.md")!.replace("**Requirements**: CFPR-02, CFGR-02", "**Requirements**: CFPR-02, CFGR-01, CFGR-02").replace("**Requirements**: CFGR-01", "**Requirements**: none"));
  let requirements = sources.get(".planning/REQUIREMENTS.md")!.replace(/(\|\s*CFGR-01\s*\|)\s*Phase 163\s*\|\s*(?:Pending|Complete)\s*\|/, "$1 Phase 162 | Pending |");
  for (const id of ["CFPR-02", "CFGR-01", "CFGR-02"]) requirements = requirements.replace(`[x] **${id}**`, `[ ] **${id}**`).replace(new RegExp(`(\\|\\s*${id}\\s*\\|\\s*Phase 162\\s*\\|)\\s*Complete\\s*\\|`), "$1 Pending |");
  sources.set(".planning/REQUIREMENTS.md", requirements);
  const index = JSON.parse(sources.get("docs/parity/index.json")!);
  const owner = index.checklist.surfaces.find((row: { id: string }) => row.id === "v2-5-integrated-retained-filter-proof");
  owner.status = "in_progress";
  if (!owner.evidence.includes(PHASE162_DOC)) owner.evidence.push(PHASE162_DOC);
  index.checklist.surfaces.find((row: { id: string }) => row.id === SURFACE).known_gaps = [PHASE162_IMPLEMENTATION];
  sources.set("docs/parity/index.json", JSON.stringify(index));
  sources.set(PHASE162_DOC, `---\nphase_lifecycle_id: 162-2026-10-11T16-25-37\nlifecycle_mode: yolo\nimplementation_status: proven\ngenerated_at: "2026-10-11T23:00:00Z"\n---\n${PHASE162_IMPLEMENTATION}\n` + PHASE162_EVIDENCE.map(file => `| \`${file}\` | \`${createHash("sha256").update(historicalFixtureView(sources).get(file)!).digest("hex")}\` |`).join("\n"));
  sources.set("README.md", sources.get("README.md")! + "\n" + PHASE162_IMPLEMENTATION);
  return sources;
}

test("current implementation transition requires linked source evidence without future formal output", () => {
  // Arrange
  const sources = transitionSources();
  // Act / Assert
  expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
  expect(checkPhase162ClaimState(file => sources.get(file) ?? "", [])).toBe("implementation");
  expect(sources.has(PHASE162 + "162-VERIFICATION.md")).toBe(false);
});

test("source binding accepts GFM table alignment without weakening hash identity", () => {
  // Arrange
  const sources = transitionSources();
  sources.set(PHASE162_DOC, sources.get(PHASE162_DOC)!.replaceAll("` | `", "`      |     `"));
  // Act / Assert
  expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
});

for (const mutation of ["proof prose alone", "removed raw input", "removed paired undo", "removed reopen", "premature activation", "duplicate owner", "missing evidence root", "stale source hash", "stale lifecycle"]) {
  test(`current transition refuses ${mutation}`, () => {
    // Arrange
    const sources = transitionSources();
    if (mutation === "proof prose alone") sources.delete(PHASE162_DOC);
    if (mutation === "removed raw input") sources.set(PHASE162_EVIDENCE[0], sources.get(PHASE162_EVIDENCE[0])!.replace("encode_block(block)", "unrelated_filter_result(block)"));
    if (mutation === "removed paired undo") sources.set(PHASE162_EVIDENCE[1], sources.get(PHASE162_EVIDENCE[1])!.replace(".load_undo(hash)", ".load_block(hash)"));
    if (mutation === "removed reopen") sources.set(PHASE162_EVIDENCE[2], sources.get(PHASE162_EVIDENCE[2])!.replace("drop(opened);", "retain(opened);"));
    if (mutation.startsWith("removed")) sources.set(PHASE162_DOC, sources.get(PHASE162_DOC)!.replace(/\| `([^`]+)` \| `[a-f0-9]{64}` \|/g, (_row, file) => `| \`${file}\` | \`${createHash("sha256").update(historicalFixtureView(sources).get(file)!).digest("hex")}\` |`));
    if (mutation === "premature activation") sources.set(".planning/REQUIREMENTS.md", sources.get(".planning/REQUIREMENTS.md")!.replace("[ ] **CFPR-02**", "[x] **CFPR-02**"));
    if (mutation === "duplicate owner" || mutation === "missing evidence root") {
      const index = JSON.parse(sources.get("docs/parity/index.json")!);
      const owner = index.checklist.surfaces.find((row: { id: string }) => row.id === "v2-5-integrated-retained-filter-proof");
      if (mutation === "duplicate owner") index.checklist.surfaces.push({ ...owner, id: "duplicate" });
      else owner.evidence = owner.evidence.filter((file: string) => file !== PHASE162_EVIDENCE[1]);
      sources.set("docs/parity/index.json", JSON.stringify(index));
    }
    if (mutation === "stale source hash") sources.set(PHASE162_EVIDENCE[0], sources.get(PHASE162_EVIDENCE[0])! + "\n// changed after evidence\n");
    if (mutation === "stale lifecycle") sources.set(PHASE162_DOC, sources.get(PHASE162_DOC)!.replaceAll("162-2026-10-11T16-25-37", "162-stale"));
    // Act
    const failures = checkPhase158ValidatedReorg(undefined, sources);
    // Assert
    expect(failures.some(failure => failure.includes("T-162-10"))).toBe(true);
  });
}

test("formal scoped completion requires all consistent fresh formal gates", () => {
  // Arrange
  const sources = transitionSources();
  sources.set("README.md", sources.get("README.md")!.replace(PHASE162_IMPLEMENTATION, PHASE162_COMPLETE));
  sources.set(PHASE162_DOC, sources.get(PHASE162_DOC)!.replace(PHASE162_IMPLEMENTATION, PHASE162_COMPLETE));
  // Act / Assert
  expect(checkPhase158ValidatedReorg(undefined, sources).some(failure => failure.includes("T-162-10"))).toBe(true);
});

for (const claim of ["BIP37 serving is supported.", "V0 is enabled.", "Assumeutxo is supported.", "Repair is enabled.", "Archive-scale is proven.", "Public defaults are enabled.", "Public-network CI is enabled.", "Production service is ready.", "Production-funds safety is proven.", "v2.5 is shipped."]) {
  for (const file of [PHASE162_DOC, "docs/parity/index.json", PHASE + "158-REORG-MEASUREMENTS.md", PHASE + "158-06-SUMMARY.md"]) {
    test(`scoped current evidence cannot authorize historical/excluded ${claim} in ${file}`, () => {
      // Arrange
      const sources = transitionSources();
      if (file.endsWith(".json")) {
        const index = JSON.parse(sources.get(file)!);
        index.checklist.surfaces.find((row: { id: string }) => row.id === "v2-5-integrated-retained-filter-proof").known_gaps.push(claim);
        sources.set(file, JSON.stringify(index));
      } else sources.set(file, sources.get(file)! + "\n" + claim);
      // Act / Assert
      expect(checkPhase158ValidatedReorg(undefined, sources).some(failure => failure.includes("unsupported"))).toBe(true);
    });
  }
}

function formalSources(): Map<string, string> {
  const sources = transitionSources();
  for (const [file, source] of sources) sources.set(file, source.replaceAll(PHASE162_IMPLEMENTATION, PHASE162_COMPLETE));
  let requirements = sources.get(".planning/REQUIREMENTS.md")!;
  for (const id of ["CFPR-02", "CFGR-01", "CFGR-02"]) requirements = requirements.replace(`[ ] **${id}**`, `[x] **${id}**`).replace(new RegExp(`(\\|\\s*${id}\\s*\\|\\s*Phase 162\\s*\\|)\\s*Pending\\s*\\|`), "$1 Complete |");
  sources.set(".planning/REQUIREMENTS.md", requirements);
  const index = JSON.parse(sources.get("docs/parity/index.json")!.replaceAll(PHASE162_IMPLEMENTATION, PHASE162_COMPLETE));
  const owner = index.checklist.surfaces.find((row: { id: string }) => row.id === "v2-5-integrated-retained-filter-proof");
  owner.status = "done";
  const digest = createHash("sha256").update(PHASE162_EVIDENCE.map(file => `${file}:${createHash("sha256").update(historicalFixtureView(sources).get(file)!).digest("hex")}`).join("\n")).digest("hex");
  for (const [name, status, generator] of [["VERIFICATION", "passed", "gsd-verifier"], ["NATIVE-EVIDENCE", "passed", "gsd-execute-phase"], ["REVIEW", "clean", "gsd-code-reviewer"], ["SECURITY", "verified", "gsd-security-auditor"]]) {
    const file = PHASE162 + `162-${name}.md`;
    owner.evidence.push(file);
    sources.set(file, `---\nstatus: ${status}\ngenerated_by: ${generator}\nphase_lifecycle_id: 162-2026-10-11T16-25-37\nlifecycle_mode: yolo\ngenerated_at: "2026-10-11T23:01:00Z"\nsource_digest: ${digest}\nlifecycle_validated: true\nthreats_open: 0\n---\nbash scripts/verify.sh\n`);
  }
  sources.set("docs/parity/index.json", JSON.stringify(index));
  return sources;
}

test("current formal fixture requires scoped completion consistency while milestone stays unaudited", () => {
  // Arrange: in-memory gate contracts, never persisted as executed reports.
  const sources = formalSources();
  // Act / Assert
  expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
});

test("native evidence cannot collide with GSD formal verification suffix selection", () => {
  // Arrange
  const sources = formalSources();
  // Act
  const formalFiles = [...sources.keys()].filter(file => file.startsWith(PHASE162) && file.endsWith("-VERIFICATION.md")).sort();
  // Assert
  expect(formalFiles).toEqual([PHASE162 + "162-VERIFICATION.md"]);
  expect(sources.has(PHASE162 + "162-NATIVE-EVIDENCE.md")).toBe(true);
});
for (const name of ["VERIFICATION", "NATIVE-EVIDENCE", "REVIEW", "SECURITY"]) {
  for (const mutation of ["missing", "wrong lifecycle", "stale hash", "old report", "failing status"]) {
    test(`formal transition refuses ${mutation} ${name}`, () => {
      // Arrange
      const sources = formalSources();
      const file = PHASE162 + `162-${name}.md`;
      if (mutation === "missing") sources.delete(file);
      else sources.set(file, sources.get(file)!.replace(
        mutation === "wrong lifecycle" ? "162-2026-10-11T16-25-37" : mutation === "stale hash" ? /source_digest: [a-f0-9]+/ : mutation === "old report" ? "23:01:00" : /status: \w+/,
        mutation === "wrong lifecycle" ? "162-stale" : mutation === "stale hash" ? "source_digest: stale" : mutation === "old report" ? "22:00:00" : "status: failed"));
      // Act / Assert
      expect(checkPhase158ValidatedReorg(undefined, sources).some(failure => failure.includes("T-162-10"))).toBe(true);
    });
  }
}

for (const [name, file, before, after] of [
  ["fast-only native command", "NATIVE-EVIDENCE", "bash scripts/verify.sh", "bash scripts/verify.sh --fast"],
  ["invalid lifecycle result", "VERIFICATION", "lifecycle_validated: true", "lifecycle_validated: false"],
  ["open security threat", "SECURITY", "threats_open: 0", "threats_open: 1"],
]) {
  test(`formal transition refuses ${name}`, () => {
    // Arrange
    const sources = formalSources();
    const target = PHASE162 + `162-${file}.md`;
    sources.set(target, sources.get(target)!.replace(before, after));
    // Act / Assert
    expect(checkPhase158ValidatedReorg(undefined, sources).some(failure => failure.includes("full native/security/lifecycle"))).toBe(true);
  });
}
function fixture(maybeSources?: ReadonlyMap<string, string>): string {
  const root = mkdtempSync(join(tmpdir(), "phase158-evidence-"));
  roots.push(root);
  for (const [file, source] of maybeSources ?? currentSources()) {
    mkdirSync(dirname(join(root, file)), { recursive: true });
    writeFileSync(join(root, file), source);
  }
  return root;
}

for (const state of ["implementation", "formal"]) {
  test(`filesystem loader permits current ${state} evidence without losing optional roots`, () => {
    // Arrange
    const sources = state === "implementation" ? transitionSources() : formalSources();
    const root = fixture(sources);
    // Act / Assert
    expect(checkPhase158ValidatedReorg(root)).toEqual([]);
    expect(loadPhase162OptionalSources(root).has(PHASE162_DOC)).toBe(true);
    expect(loadPhase162OptionalSources(root).has(PHASE162 + "162-VERIFICATION.md")).toBe(state === "formal");
    expect([...loadPhase162OptionalSources(root).keys()].sort()).toEqual([...new Set([...PROVENANCE_FILES, ...PHASE162_EVIDENCE])].filter(file => sources.has(file)).sort());
  });
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
function mutateFunction(root: string, file: string, symbol: string, transform: (body: string) => string, maybeImpl?: string): void {
  edit(root, file, source => {
    const maybeBody = maybeBoundary(source, { file, symbol, anchors: [], maybeImpl });
    if (!maybeBody) throw new Error(`missing actual fixture function ${symbol}`);
    return source.slice(0, maybeBody.start) + transform(source.slice(maybeBody.start, maybeBody.end)) + source.slice(maybeBody.end);
  });
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

const CHAINSTATE = "packages/open-bitcoin-node/src/chainstate.rs";
const LIFECYCLE = "packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs";
const PRUNE_FLUSH = "packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs";
test("checkpoint guard accepts rustfmt-style delegate whitespace", () => {
  // Arrange
  const root = fixture();
  mutateFunction(root, CHAINSTATE, "flush_applying_plan_with_header_checkpoint", body =>
    body.replace(pattern("self.flush_lifecycle.execute_flush_applying_plan_with_header_checkpoint("),
      "self\n  .flush_lifecycle\n  .execute_flush_applying_plan_with_header_checkpoint\n  ("));
  // Act
  const failures = checkPhase158ValidatedReorg(root);
  // Assert
  expect(failures).toEqual([]);
});
const CHECKPOINT_MUTATIONS: [string, string, string, (body: string) => string][] = [
  ["removed delegate", CHAINSTATE, "flush_applying_plan_with_header_checkpoint", body => body.replace(pattern("self.flush_lifecycle.execute_flush_applying_plan_with_header_checkpoint("), "removed_delegate(")],
  ["changed IfNeeded", CHAINSTATE, "persist", body => body.replace("FlushMode::IfNeeded", "FlushMode::Always")],
  ["removed IfNeeded", CHAINSTATE, "persist", body => body.replace("FlushMode::IfNeeded", "removed_mode")],
  ["actual leftover snapshot", CHAINSTATE, "persist", body => body.replace("self.flush_with_mode(", "self.save_snapshot(self.chainstate.snapshot())?; self.flush_with_mode(")],
  ["comment-only delegate", CHAINSTATE, "flush_applying_plan_with_header_checkpoint", body => body.replace(pattern("self.flush_lifecycle.execute_flush_applying_plan_with_header_checkpoint("), "/* self.flush_lifecycle.execute_flush_applying_plan_with_header_checkpoint( */ removed_delegate(")],
  ["literal-only delegate", CHAINSTATE, "flush_applying_plan_with_header_checkpoint", body => body.replace(pattern("self.flush_lifecycle.execute_flush_applying_plan_with_header_checkpoint("), '"self.flush_lifecycle.execute_flush_applying_plan_with_header_checkpoint("; removed_delegate(')],
  ["removed checkpoint", LIFECYCLE, "execute_flush_applying_plan_with_header_checkpoint", body => body.replace("checkpoint(sink)?;", "")],
  ["checkpoint before prefix", LIFECYCLE, "execute_flush_applying_plan_with_header_checkpoint", body => body.replace("checkpoint(sink)?;", "").replace("persist_ordered_prefix(sink,", "checkpoint(sink)?; persist_ordered_prefix(sink,")],
  ["checkpoint after unlink", LIFECYCLE, "execute_flush_applying_plan_with_header_checkpoint", body => body.replace("checkpoint(sink)?;", "").replace("complete_coins_write(sink,", "checkpoint(sink)?; complete_coins_write(sink,")],
  ["swallowed checkpoint failure", LIFECYCLE, "execute_flush_applying_plan_with_header_checkpoint", body => body.replace("checkpoint(sink)?;", "let _ = checkpoint(sink);")],
  ["untyped checkpoint result", LIFECYCLE, "execute_flush_applying_plan_with_header_checkpoint", body => body.replace("FnMut(&S) -> Result<(), StorageError>", "FnMut(&S) -> Result<(), OtherError>")],
  ["eager checkpoint before readiness", LIFECYCLE, "execute_flush_applying_plan_with_header_checkpoint", body => body.replace("if self.readiness", "checkpoint(sink)?; if self.readiness")],
  ["eager producer outside callback", PRUNE_FLUSH, "flush_and_evict_pruned_blocks", body => body.replace("chainstate.flush_applying_plan_with_header_checkpoint(", "let _ = crate::network::validation_history::TrustedHeaderSnapshot::maybe_capture(store, peer_manager.header_store()); chainstate.flush_applying_plan_with_header_checkpoint(")],
  ["swallowed snapshot failure", PRUNE_FLUSH, "flush_and_evict_pruned_blocks", body => body.replace("snapshot.persist()", "let _ = snapshot.persist(); Ok(())")],
];
for (const [name, file, symbol, transform] of CHECKPOINT_MUTATIONS) {
  test(`checkpoint guard refuses ${name}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, file, symbol, transform);
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes("T-162-15") && item.includes(`${file}:${symbol}`))).toBe(true);
  });
}
for (const row of CHECKPOINT_CONTRACTS) for (const anchor of row.anchors) {
  test(`checkpoint guard actual ${row.symbol} requires ${anchor}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, row.file, row.symbol, body => body.replace(pattern(anchor), "removed_checkpoint_boundary"));
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes("T-162-15") && item.includes(`${row.file}:${row.symbol}`))).toBe(true);
  });
}

test("current bounded roots pass without changing evidence", () => {
  // Arrange
  const root = fixture();
  const before = CHECK_FILES.map(file => readFileSync(join(root, file), "utf8"));
  // Act
  const failures = checkPhase158ValidatedReorg(root);
  // Assert
  expect(failures).toEqual([]);
  expect(CHECK_FILES.map(file => readFileSync(join(root, file), "utf8"))).toEqual(before);
});
for (const row of CONTRACTS) for (const anchor of row.anchors) {
  test(`actual route ${row.symbol} requires ${anchor}`, () => {
    // Arrange
    const root = fixture();
    mutateFunction(root, row.file, row.symbol, body => body.replace(pattern(anchor), "removed_boundary"), row.maybeImpl);
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes(`${row.file}:${row.symbol}`))).toBe(true);
  });
}
for (const declaration of ["fn", "async fn", "pub(crate) async fn", "pub(in crate::network) unsafe fn"]) {
  test(`test-only ${declaration} cannot replace actual acceptance`, () => {
    // Arrange: a small lexical control, never simulated runtime evidence.
    const root = fixture();
    const row = CONTRACTS.find(row => row.symbol === "commit_prepared_reorg")!;
    edit(root, row.file, source => source.replace(/pub\(crate\) fn commit_prepared_reorg/, `#[cfg(test)]\n#[allow(dead_code)]\n${declaration} commit_prepared_reorg`));
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes(`${row.file}:${row.symbol}`))).toBe(true);
  });
}
for (const disguise of ["comment", "literal", "raw literal"]) {
  test(`receipt call cannot be supplied by ${disguise}`, () => {
    // Arrange
    const root = fixture();
    const row = CONTRACTS.find(row => row.symbol === "commit_prepared_reorg")!;
    const call = "absorb_staged_reorg_with_receipt(prepared.staged)";
    const replacements = { comment: `// ${call}\n`, literal: `"${call}"`, "raw literal": `r###"${call}"###` };
    mutateFunction(root, row.file, row.symbol, body => body.replace(call, replacements[disguise as keyof typeof replacements]));
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes(row.symbol))).toBe(true);
  });
}
for (const [file, name] of TESTS) {
  test(`required executable behavior remains ${name}`, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => source.replace(name, "removed_test"));
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes(name))).toBe(true);
  });
}
test("ignored required behavior refuses", () => {
  // Arrange
  const root = fixture();
  const [file, name] = TESTS[0];
  edit(root, file, source => source.replace(`fn ${name}`, `#[ignore]\nfn ${name}`));
  // Act
  const failures = checkPhase158ValidatedReorg(root);
  // Assert
  expect(failures.some(item => item.includes(name))).toBe(true);
});
for (const symbol of ["StagedChainstateReorg", "AcceptedChainstateReorg", "ValidatedBasicFilterReorg", "ValidatedBasicFilterAppendPositions", "BasicFilterAppendProof"]) {
  test(`sealed ${symbol} fields cannot become public`, () => {
    // Arrange
    const root = fixture();
    const file = CHECK_FILES.find(file => file.endsWith(symbol === "BasicFilterAppendProof" ? "/ownership.rs" : symbol.startsWith("Validated") ? "/fjall_store/reorg.rs" : "/engine.rs"))!;
    edit(root, file, source => source.replace(new RegExp(`(struct ${symbol}(?:<'a>)?\\s*\\{\\s*)(\\w+)`), "$1pub $2"));
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes(`private ${symbol}`))).toBe(true);
  });
}
const SEALED_FIELDS = [
  ["StagedChainstateReorg", "overlay"], ["AcceptedChainstateReorg", "maybe_old_endpoint"],
  ["ValidatedBasicFilterReorg", "facts"], ["ValidatedBasicFilterAppendPositions", "positions"],
  ...["publication", "preparation_work", "maybe_maximum_work"].map(field => ["BasicFilterAppendProof", field]),
];
function mutateSealedField(root: string, symbol: string, field: string, visibility: string): void {
  const file = CHECK_FILES.find(file => file.endsWith(symbol === "BasicFilterAppendProof" ? "/ownership.rs" : symbol.startsWith("Validated") ? "/fjall_store/reorg.rs" : "/engine.rs"))!;
  edit(root, file, source => source.replace(new RegExp(`struct ${symbol}(?:<'a>)?\\s*\\{[^}]+\\}`), block =>
    block.replace(new RegExp(`\\n(\\s*)(?:pub\\s*\\([^)]*\\)\\s+)?${field}\\s*:`), `\n$1${visibility} ${field}:`)));
}
for (const [symbol, field] of SEALED_FIELDS) {
  for (const visibility of ["pub", "pub(super)", "pub(crate)", "pub(self)", "pub(in crate)"]) {
    test(`restricted visibility refuses ${symbol}.${field}: ${visibility}`, () => {
      // Arrange
      const root = fixture();
      mutateSealedField(root, symbol, field, visibility);
      // Act
      const failures = checkPhase158ValidatedReorg(root);
      // Assert
      expect(failures.some(item => item.includes(`private ${symbol}`))).toBe(true);
    });
  }
}
for (const visibility of ["pub", "pub(crate)", "pub(self)", "pub(in crate)", "pub(in super)"]) {
  test(`restricted visibility refuses proof identity outside exact pub(super): ${visibility}`, () => {
    // Arrange
    const root = fixture();
    mutateSealedField(root, "BasicFilterAppendProof", "identity", visibility);
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes("private BasicFilterAppendProof"))).toBe(true);
  });
}
for (const claim of ["Peer filter serving is supported.", "Filter/index RPC is available.", "v2.5 is shipped.", "Production readiness is established.",
  "Production-funds use is safe.", "Hardware power-loss proof passed.", "Arbitrary retained forks are supported.", "Whole-runtime constant memory is guaranteed.",
  "Isolated publication latency is guaranteed.", "Deep reorg support is complete.", "Public serving defaults are enabled.",
  "Peer filter serving is now fully supported.", "Public serving defaults are already enabled.", "v2.5 has shipped."]) {
  for (const file of [DOC, "README.md", "packages/README.md", PHASE + "158-UAT.md"]) {
    test(`independent unsupported claim refuses in ${file}: ${claim}`, () => {
      // Arrange
      const root = fixture();
      edit(root, file, source => source + `\n${claim}\nOther products remain deferred.\n`);
      // Act
      const failures = checkPhase158ValidatedReorg(root);
      // Assert
      expect(failures.some(item => item.includes("unsupported claim") && item.includes(file))).toBe(true);
    });
  }
}
test("another subject's negation cannot authorize production readiness", () => {
  // Arrange
  const root = fixture();
  edit(root, DOC, source => source + "\nNo claim of peer filter serving, production readiness is established.\n");
  // Act
  const failures = checkPhase158ValidatedReorg(root);
  // Assert
  expect(failures.some(item => item.includes("unsupported claim"))).toBe(true);
});
for (const command of ["bun test ./scripts/check-phase158-validated-reorg.test.ts", "bun run scripts/check-phase158-validated-reorg.ts"]) {
  for (const executed of [false, true]) {
    test(`ordered ${executed ? "executed" : "documented"} verifier requires ${command}`, () => {
      // Arrange
      const root = fixture();
      edit(root, "scripts/verify.sh", source => source.replace(new RegExp(`^${executed ? "run_step [^\\n]+" : ""}${command.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`, "m"), "# removed stage"));
      // Act
      const failures = checkPhase158ValidatedReorg(root);
      // Assert
      expect(failures.some(item => item.includes("verifier"))).toBe(true);
    });
  }
}
test("breadcrumb mapping cannot be removed", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-node/src/chainstate/filter_reorg.rs";
  edit(root, "docs/parity/source-breadcrumbs.json", source => source.replace(file, "removed/path.rs"));
  // Act
  const failures = checkPhase158ValidatedReorg(root);
  // Assert
  expect(failures.some(item => item.includes("breadcrumb") && item.includes(file))).toBe(true);
});
for (const file of CHECK_FILES.filter(file => !file.startsWith("scripts/check-phase158"))) {
  test(`fixed current evidence root must exist: ${file}`, () => {
    // Arrange
    const root = fixture();
    unlinkSync(join(root, file));
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes(file) && item.includes("cannot read"))).toBe(true);
  });
}
function requirementsFixture(): string {
  const sources = transitionSources();
  sources.set(V25_REQUIREMENTS[1], sources.get(V25_REQUIREMENTS[0])!);
  return fixture(sources);
}
test("logical requirements accept valid archive when active is absent", () => {
  // Arrange
  const root = requirementsFixture();
  unlinkSync(join(root, V25_REQUIREMENTS[0]));
  // Act
  const failures = checkPhase158ValidatedReorg(root);
  // Assert
  expect(failures).toEqual([]);
});
test("logical requirements refuse all fixed candidates missing", () => {
  // Arrange
  const root = requirementsFixture();
  for (const file of V25_REQUIREMENTS) unlinkSync(join(root, file));
  // Act
  const failures = checkPhase158ValidatedReorg(root);
  // Assert
  expect(failures.some(item => item.includes("T-162-10") && item.includes("cannot read required evidence") && V25_REQUIREMENTS.every(file => item.includes(file)))).toBe(true);
});
for (const active of ["", "**Milestone:** v2.x\n"]) {
  test(`logical requirements refuse invalid-present active despite valid archive: ${JSON.stringify(active)}`, () => {
    // Arrange
    const root = requirementsFixture();
    writeFileSync(join(root, V25_REQUIREMENTS[0]), active);
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes(`${V25_REQUIREMENTS[0]}: invalid active milestone identity/baseline`))).toBe(true);
  });
}
for (const [file, module] of REGISTRATIONS) {
  test(`executable registration must remain: ${file}:${module}`, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => source.replace(module, `// ${module}`));
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes("registration") && item.includes(file))).toBe(true);
  });
}
for (const field of ["title", "rationale", "evidence", "known_gaps", "intentional_differences"]) {
  test(`positive parity ${field} claim cannot borrow another deferred clause`, () => {
    // Arrange
    const root = fixture();
    edit(root, "docs/parity/index.json", source => {
      const index = JSON.parse(source);
      const surface = index.checklist.surfaces.find((row: { id: string }) => row.id === SURFACE);
      const claim = "Peer filter serving is now fully supported.";
      if (typeof surface[field] === "string") surface[field] += " " + claim;
      else surface[field].push(claim);
      surface.known_gaps.push("Other products remain deferred.");
      return JSON.stringify(index);
    });
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes("unsupported claim"))).toBe(true);
  });
}
for (const field of ["requirements", "evidence", "intentional_differences", "known_gaps"]) {
  test(`parity ${field} cannot be emptied`, () => {
    // Arrange
    const root = fixture();
    edit(root, "docs/parity/index.json", source => {
      const index = JSON.parse(source);
      index.checklist.surfaces.find((row: { id: string }) => row.id === SURFACE)[field] = [];
      return JSON.stringify(index);
    });
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes("parity") || item.includes("resource-policy") || item.includes("serving boundary"))).toBe(true);
  });
}
for (const anchor of ["54 concrete Fjall configurations and 149 ordinary bounded turns", "not isolated new-index timings", "401-block history", "98 record operations and 56 point reads"]) {
  test(`current measured limit remains ${anchor}`, () => {
    // Arrange
    const root = fixture();
    edit(root, PHASE + "158-REORG-MEASUREMENTS.md", source => source.replace(anchor, "removed_measurement"));
    // Act
    const failures = checkPhase158ValidatedReorg(root);
    // Assert
    expect(failures.some(item => item.includes("measurement"))).toBe(true);
  });
}
test("missing current root exits with bounded category diagnostics", () => {
  // Arrange
  const root = fixture();
  unlinkSync(join(root, DOC));
  // Act
  const result = spawnSync(process.execPath, ["run", join(root, "scripts/check-phase158-validated-reorg.ts")], { encoding: "utf8", timeout: 10_000 });
  // Assert
  expect(result.error).toBeUndefined();
  expect(result.status).toBe(1);
  expect(result.stderr).toContain("cannot read required evidence");
  expect(result.stderr.length).toBeLessThan(10_000);
});

function currentSources(): Map<string, string> {
  const sources = loadPhase162OptionalSources(REPO);
  for (const file of CHECK_FILES) sources.set(file, readFileSync(join(REPO, file), "utf8"));
  return sources;
}

test("actual live provenance preserves historical Phase162 proof in every valid closure state", () => {
  const sources = currentSources();
  expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
  expect(checkPhase162ClaimState(file => sources.get(file) ?? "", [])).toBe("complete");
});

function gapPendingSources(): Map<string, string> {
  const sources = v25FixtureSources(currentSources());
  sources.set(".planning/REQUIREMENTS.md", sources.get(".planning/REQUIREMENTS.md")!.replace("[x] **CFGR-01**", "[ ] **CFGR-01**").replace(/(\|\s*CFGR-01\s*\|\s*Phase 163\s*\|)\s*Complete\s*\|/, "$1 Pending |"));
  sources.set(".planning/v2.5-MILESTONE-AUDIT.md", sources.get(ORIGINAL_AUDITS[0])!);
  return sources;
}

test("explicit audited Phase163 Pending preserves formal proof without future reports", () => {
  const sources = gapPendingSources();
  expect(sources.has(PHASE163 + "163-VERIFICATION.md")).toBe(false);
  sources.set("README.md", sources.get("README.md")! + "\n" + GAP_CLAIM);
  expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
});

test("original implementation fixture restores Phase162 ownership after later closure completion", () => {
  const advanced = v25FixtureSources(currentSources());
  advanced.set(".planning/REQUIREMENTS.md", advanced.get(".planning/REQUIREMENTS.md")!.replace("[ ] **CFGR-01**", "[x] **CFGR-01**").replace(/(\|\s*CFGR-01\s*\|\s*Phase 163\s*\|)\s*Pending\s*\|/, "$1 Complete |"));
  const sources = transitionSources(advanced);
  expect([...sources.keys()].sort()).toEqual([...transitionSources().keys()].sort());
  expect(checkPhase162ClaimState(file => sources.get(file) ?? "", [])).toBe("implementation");
  expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
});

for (const file of [PHASE162 + "162-VERIFICATION.md", ".planning/v2.5-MILESTONE-AUDIT.md", PHASE163 + "163-GAP-SCOPE.md", ".planning/REQUIREMENTS.md"]) {
  test(`supplied-map Pending cannot rescue absent ${file} from valid disk`, () => {
    const sources = gapPendingSources();
    sources.delete(file);
    expect(checkPhase158ValidatedReorg(undefined, sources).some(failure => failure.includes("T-162-10"))).toBe(true);
  });
}

for (const claim of [CLOSURE_CLAIM, READINESS_CLAIM, ARCHIVED_CLAIM]) {
  test(`Pending cannot publish exact unauthorized sentence ${claim}`, () => {
    const sources = gapPendingSources();
    sources.set("README.md", sources.get("README.md")! + "\n" + claim);
    expect(checkPhase158ValidatedReorg(undefined, sources).some(failure => failure.includes("unsupported claim"))).toBe(true);
  });
}

for (const claim of [READINESS_CLAIM, "v2.5 is shipped.", "Production service is ready.", `\`\`\`text\n${READINESS_CLAIM}\n\`\`\``]) {
  test(`optional current provenance page refuses unearned/excluded claim: ${claim}`, () => {
    // Arrange
    const sources = gapPendingSources();
    const file = PHASE163_DOC;
    sources.set(file, claim);
    // Act
    const failures = checkPhase158ValidatedReorg(undefined, sources);
    // Assert
    expect(failures.some(failure => failure.includes(file) && failure.includes("unsupported claim"))).toBe(true);
  });
}

test("optional current provenance page accepts earned gap wording and may be absent before creation", () => {
  const sources = gapPendingSources();
  const file = PHASE163_DOC;
  sources.set(file, GAP_CLAIM);
  expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
  sources.delete(file);
  expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
});

for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS]) {
  test(`empty active control remains present in158 map: ${paths[0]}`, () => {
    // Arrange
    const sources = gapPendingSources();
    sources.set(paths[1], sources.get(paths[0])!);
    sources.set(paths[0], "");
    // Act
    const failures = checkPhase158ValidatedReorg(undefined, sources);
    // Assert
    expect(failures.some(failure => failure.includes(`${paths[0]}: invalid active milestone identity`))).toBe(true);
    sources.delete(paths[0]);
    expect(checkPhase158ValidatedReorg(undefined, sources)).toEqual([]);
  });
}
