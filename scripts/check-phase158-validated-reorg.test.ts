import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { CHECK_FILES, CONTRACTS, DOC, PHASE, REGISTRATIONS, SURFACE, TESTS, checkPhase158ValidatedReorg, maybeBoundary } from "./check-phase158-validated-reorg.ts";

const REPO = resolve(import.meta.dir, "..");
const roots: string[] = [];
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "phase158-evidence-"));
  roots.push(root);
  for (const file of CHECK_FILES) {
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
function mutateFunction(root: string, file: string, symbol: string, transform: (body: string) => string, maybeImpl?: string): void {
  edit(root, file, source => {
    const maybeBody = maybeBoundary(source, { file, symbol, anchors: [], maybeImpl });
    if (!maybeBody) throw new Error(`missing actual fixture function ${symbol}`);
    return source.slice(0, maybeBody.start) + transform(source.slice(maybeBody.start, maybeBody.end)) + source.slice(maybeBody.end);
  });
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

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
