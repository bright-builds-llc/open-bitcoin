import { afterEach, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import manifest from "../../packages/testdata/phase164/historical-tooling/manifest.json";
import { historicalToolingView, legacyToolingBase, MANIFEST_FILE, toolingAttribution } from "./historical-tooling.ts";
import { loadSources } from "../check-phase162-integrated-filter-proof.ts";
import { PROVENANCE_DIGEST_FILES, PROVENANCE_FILES, PHASE163, provenanceMetadata } from "../check-phase163-provenance.ts";
import { checkPhase164, historicalFixtureAdapter, loadInputs } from "../check-phase164-canonical-identity.ts";
import { createHash } from "node:crypto";
import { COPIED_CLI_INVERSES, FILTER_GUARD_INVERSES, FIXTURE_ADAPTER_INVERSE, HISTORICAL_TOOLING_INVERSE, PHASE162_SOURCE_INVERSE, maybeRestoreReviewedSource } from "./contracts.ts";
import { MIGRATED_FIXTURE } from "./fixture-source.ts";
import { checkPhase165, loadInputs as load165Inputs } from "../check-phase165-shared-exact-work.ts";
import { CLI_MODULES, COPIED_ADAPTER_MUTATIONS, COPIED_CLI_MUTATIONS, expectCopiedAdapterCase } from "../check-phase165-shared-exact-work/test-support.ts";
import { DOC, loadPhase162OptionalSources } from "../check-phase158-validated-reorg.ts";
import { ENTRIES, REPO, assertLiveUnchanged, cleanedRoots, cleanupFixtures, copiedCliSources, createCopiedFixture, runtimeSources } from "../check-phase163-provenance/fixtures.ts";

test("original Phase163 receipt digest qualifies only the preserved historical tooling view", () => {
  // Arrange
  const sources = loadSources();
  const originalReceipt = sources.get(PHASE163 + "163-VERIFICATION.md")!;
  // Act
  const attribution = toolingAttribution(sources, PROVENANCE_DIGEST_FILES);
  // Assert
  expect(attribution.evidence_scope).toBe("historical/source-derived-regression");
  expect(attribution.currentOrdinaryQualification).toBe(false);
  expect(attribution.rawCurrentDigest).not.toBe(attribution.historicalDigest);
  expect(attribution.historicalDigest).toBe(provenanceMetadata(originalReceipt).source_digest);
  expect(sources.get(PHASE163 + "163-VERIFICATION.md")).toBe(originalReceipt);
});

for (const row of manifest.bindings) {
  test(`missing current tooling input refuses without checkout fallback: ${row.file}`, () => {
    // Arrange
    const sources = loadSources();
    sources.delete(row.file);
    // Act / Assert
    expect(() => historicalToolingView(sources)).toThrow("unreviewed current tooling fingerprint");
  });
  for (const file of [row.file, row.snapshot]) {
    test(`unknown tooling or original snapshot bytes refuse: ${file}`, () => {
      // Arrange
      const sources = loadSources();
      sources.set(file, sources.get(file)! + "\n");
      // Act / Assert
      expect(() => historicalToolingView(sources)).toThrow();
      expect(checkPhase164(sources).some(row => row.rule === "history")).toBe(true);
    });
  }
  test(`missing original snapshot refuses: ${row.snapshot}`, () => {
    // Arrange
    const sources = loadSources();
    sources.delete(row.snapshot);
    // Act / Assert
    expect(() => historicalToolingView(sources)).toThrow("snapshot changed or missing");
  });
}

const TOOLING_SOURCE = "scripts/check-phase164-canonical-identity/historical-tooling.ts";
const ADAPTER_SOURCE = "scripts/check-phase164-canonical-identity/fixture-source.ts";
function currentBoundInputs(): Map<string, string> {
  const sources = loadInputs();
  for (const [file] of FILTER_GUARD_INVERSES) sources.set(file, readFileSync(join(import.meta.dir, "../..", file), "utf8"));
  return sources;
}

for (const change of COPIED_ADAPTER_MUTATIONS.filter(row => ["old-guard-0", "unknown-guard-0", "old-guard-2", "unknown-guard-2"].includes(row[0]))) {
  test(`phase165 filter guard optional caller scope: ${change[0]}`, () => {
    // Arrange: explicit extra fixture data does not broaden the actual164 CLI inventory.
    const sources = load165Inputs();
    sources.set(change[1], readFileSync(join(import.meta.dir, "../..", change[1]), "utf8"));
    const loadSupplied = (root: string) => new Map([...load165Inputs(root), [change[1], readFileSync(join(root, change[1]), "utf8")] as const]);
    // Act / Assert: direct supplied164 data refuses; fresh fixed-input164 succeeds without that source.
    expectCopiedAdapterCase(change, sources, loadSupplied, [[CLI_MODULES[2], checkPhase164]]);
  });
}

test("phase165 filter guard current generation qualifies complete actual PASSED inputs", () => {
  // Arrange
  const sources = currentBoundInputs();
  const original = [...sources];
  // Act
  const adapter = historicalFixtureAdapter(sources);
  const maybeHistoricalTooling = maybeRestoreReviewedSource(sources.get(TOOLING_SOURCE)!, HISTORICAL_TOOLING_INVERSE);
  // Assert
  expect(rawSha(adapter)).toBe("b92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3");
  expect(rawSha(maybeHistoricalTooling!)).toBe("47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65");
  expect([...sources]).toEqual(original);
});

const CURRENT_GENERATIONS = [
  [TOOLING_SOURCE, HISTORICAL_TOOLING_INVERSE, "paired current raw historical-tooling generation"],
  [ADAPTER_SOURCE, FIXTURE_ADAPTER_INVERSE, "paired current raw fixture/adapter generation"],
  [MIGRATED_FIXTURE, PHASE162_SOURCE_INVERSE, "paired current raw fixture/adapter generation"],
  ...FILTER_GUARD_INVERSES.map(([file, inverse]) => [file, inverse, `${file}: reviewed current filter guard generation`] as const),
  ...COPIED_CLI_INVERSES.map(([file, inverse]) => [file, inverse, `${file}: reviewed current copied CLI generation`] as const),
] as const;
for (const [file, inverse, detail] of CURRENT_GENERATIONS) for (const mode of ["historical", "unknown"] as const) test(`phase165 filter guard current generation refuses ${mode} PASSED source: ${file}`, () => {
  // Arrange: the imported adapter remains current while the explicit map generation changes.
  const sources = currentBoundInputs();
  expect(() => historicalFixtureAdapter(sources)).not.toThrow();
  const current = sources.get(file)!;
  const maybeHistorical = maybeRestoreReviewedSource(current, inverse);
  expect(maybeHistorical).toBeDefined();
  // Act
  sources.set(file, mode === "historical" ? maybeHistorical! : current + "\n");
  // Assert
  expect(() => historicalFixtureAdapter(sources)).toThrow(detail);
});

for (const file of [TOOLING_SOURCE, ADAPTER_SOURCE, MIGRATED_FIXTURE, "scripts/check-phase159-filter-rpcs.test.ts"]) test(`phase165 filter guard current generation requires PASSED source: ${file}`, () => {
  // Arrange
  const sources = currentBoundInputs();
  sources.delete(file);
  // Act / Assert
  expect(() => historicalFixtureAdapter(sources)).toThrow();
});

test("phase165 filter guard current generation does not invent absent optional guard evidence", () => {
  // Arrange
  const sources = currentBoundInputs();
  for (const file of ["scripts/check-phase159-filter-rpcs.ts", "scripts/check-phase161-operator-index-evidence.ts"]) sources.delete(file);
  // Act / Assert
  expect(() => historicalFixtureAdapter(sources)).not.toThrow();
  expect(sources.has("scripts/check-phase159-filter-rpcs.ts")).toBe(false);
  expect(sources.has("scripts/check-phase161-operator-index-evidence.ts")).toBe(false);
});

for (const [phase, load, check] of [[164, currentBoundInputs, checkPhase164], [165, load165Inputs, checkPhase165]] as const) {
  for (const [file, inverse, detail] of CURRENT_GENERATIONS) test(`phase165 filter guard full${phase} rejects stale PASSED generation: ${file}`, () => {
    // Arrange: pending ordinary SOURCE rows are separate from this history-specific gate.
    const sources = load();
    for (const [guard] of FILTER_GUARD_INVERSES) sources.set(guard, readFileSync(join(import.meta.dir, "../..", guard), "utf8"));
    const baseline = check(sources);
    expect(baseline.filter(row => row.rule === "history")).toEqual([]);
    if (phase === 164) expect(baseline).toEqual([]);
    const maybeHistorical = maybeRestoreReviewedSource(sources.get(file)!, inverse);
    expect(maybeHistorical).toBeDefined();
    // Act
    sources.set(file, maybeHistorical!);
    const findings = check(sources);
    // Assert
    expect(findings.some(row => row.rule === "history" && row.detail.includes(detail))).toBe(true);
  });
}

test("original unmodified tooling is explicitly historical and portable without git", () => {
  // Arrange
  const sources = loadSources();
  for (const row of manifest.bindings) sources.set(row.file, sources.get(row.snapshot)!);
  for (const [file, inverse] of FILTER_GUARD_INVERSES) {
    if (manifest.bindings.some(row => row.file === file)) continue;
    sources.set(file, maybeRestoreReviewedSource(sources.get(file)!, inverse)!);
  }
  // Act
  const attribution = toolingAttribution(sources, PROVENANCE_DIGEST_FILES);
  // Assert
  expect(attribution.rawCurrentDigest).toBe(attribution.historicalDigest);
  expect(attribution.currentOrdinaryQualification).toBe(false);
});

const rawSha = (source: string) => createHash("sha256").update(source).digest("hex");

test("phase165 filter guard historical data preserves the complete original receipt digest", () => {
  // Arrange
  const sources = loadSources();
  const original = [...sources];
  // Act
  const view = historicalToolingView(sources);
  const attribution = toolingAttribution(sources, PROVENANCE_DIGEST_FILES);
  // Assert
  expect(attribution.historicalDigest).toBe(provenanceMetadata(sources.get(PHASE163 + "163-VERIFICATION.md")!).source_digest);
  expect([...sources]).toEqual(original);
  for (const [file, inverse] of FILTER_GUARD_INVERSES) {
    const maybeBinding = manifest.bindings.find(row => row.file === file);
    expect(rawSha(view.get(file)!)).toBe(maybeBinding?.original_sha ?? inverse.restoredRawSha);
  }
});

test("phase165 filter guard historical data accepts partial maps without adapter or optional guard reads", () => {
  // Arrange
  const sources = loadSources();
  for (const file of ["scripts/check-phase164-canonical-identity/historical-tooling.ts", "scripts/check-phase164-canonical-identity/fixture-source.ts",
    "scripts/check-phase159-filter-rpcs.ts", "scripts/check-phase161-operator-index-evidence.ts"]) sources.delete(file);
  const original = [...sources];
  // Act
  const view = historicalToolingView(sources);
  // Assert
  expect([...sources]).toEqual(original);
  expect([...view.keys()]).toEqual([...sources.keys()]);
  expect(view.get("scripts/check-phase159-filter-rpcs.test.ts")).toBe(sources.get("packages/testdata/phase164/historical-tooling/phase159-tests.txt"));
});

for (const [file, inverse] of FILTER_GUARD_INVERSES) {
  test(`phase165 filter guard historical inverse reconstructs exact before source: ${file}`, () => {
    // Arrange
    const sources = loadSources();
    const source = sources.get(file)!;
    // Act
    const maybeBefore = maybeRestoreReviewedSource(source, inverse);
    // Assert
    expect(rawSha(source)).toBe(inverse.currentRawSha);
    expect(maybeBefore).toBeDefined();
    expect(rawSha(maybeBefore!)).toBe(inverse.restoredRawSha);
    sources.set(file, maybeBefore!);
    expect(() => historicalToolingView(sources)).not.toThrow();
  });
  test(`phase165 filter guard historical data refuses unknown present generation: ${file}`, () => {
    // Arrange: authenticate all manifest owners so a different owner cannot mask this refusal.
    const sources = loadSources();
    for (const row of manifest.bindings) sources.set(row.file, sources.get(row.snapshot)!);
    sources.set(file, sources.get(file)! + "\n");
    // Act / Assert
    expect(() => historicalToolingView(sources)).toThrow(`${file}: unreviewed current tooling fingerprint`);
  });
  const source = loadSources().get(file)!;
  for (const [name, invalid] of [
    ["wrong current SHA", { ...inverse, currentRawSha: "0".repeat(64) }],
    ["wrong restored SHA", { ...inverse, restoredRawSha: "0".repeat(64) }],
    ["missing hunks", { ...inverse, hunks: [] }],
    ["duplicate hunk", { ...inverse, hunks: [inverse.hunks[0], ...inverse.hunks] }],
    ["overlapping spans", { ...inverse, hunks: [inverse.hunks[0], [inverse.hunks[0][0].slice(1), ""], ...inverse.hunks.slice(1)] }],
    ["forged descriptor", { ...inverse, unreviewed: true }],
    ...(inverse.hunks.length > 1 ? [
      ["partial hunks", { ...inverse, hunks: inverse.hunks.slice(0, -1) }],
      ["reordered hunks", { ...inverse, hunks: inverse.hunks.toReversed() }],
    ] as const : []),
  ] as const) test(`phase165 filter guard historical inverse rejects ${name}: ${file}`, () => {
    // Arrange / Act / Assert
    expect(maybeRestoreReviewedSource(source, invalid)).toBeUndefined();
  });
}

for (const file of [MANIFEST_FILE, "scripts/check-phase164-canonical-identity/historical-tooling.ts", "scripts/check-phase164-canonical-identity/fixture-source.ts"]) {
  test(`manifest or adapter cannot silently relabel original evidence: ${file}`, () => {
    // Arrange
    const inputs = loadInputs();
    inputs.set(file, inputs.get(file)! + "\n");
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.rule === "history")).toBe(true);
  });
}

test("specified corpus root cannot borrow missing historical snapshots from the checkout", () => {
  // Arrange
  const root = mkdtempSync(join(tmpdir(), "open-bitcoin-phase164-corpus-"));
  const inputs = loadInputs();
  try {
    for (const [file, source] of inputs) {
      if (file === manifest.bindings[0].snapshot) continue;
      const destination = join(root, file);
      mkdirSync(dirname(destination), { recursive: true });
      writeFileSync(destination, source);
    }
    // Act / Assert
    expect(() => loadInputs(root)).toThrow("ENOENT");
  } finally { rmSync(root, { recursive: true }); }
});

const closureRoots: string[] = [];
const MISSING_RUNTIME_HELPERS = ["contracts.ts", "source-files.ts", "test-modules.ts"]
  .map(name => "scripts/check-phase164-canonical-identity/" + name);
afterEach(() => {
  for (const root of closureRoots.splice(0)) rmSync(root, { recursive: true, force: true });
  assertLiveUnchanged();
});
function closureRoot(): string {
  const root = mkdtempSync(join(tmpdir(), "phase165-copied-closure-"));
  closureRoots.push(root);
  return root;
}
function isolatedRuntime(): string {
  const root = closureRoot();
  for (const [file, source] of runtimeSources()) {
    const destination = join(root, file);
    mkdirSync(dirname(destination), { recursive: true });
    writeFileSync(destination, source);
  }
  return root;
}
function missingDocumentFixture(): string {
  const sources = loadSources();
  sources.delete(DOC);
  return createCopiedFixture(sources, root => closureRoots.push(root), [ENTRIES[0]]);
}
function copiedClosureExit(root: string) {
  const result = spawnSync(process.execPath, ["run", join(root, ENTRIES[0])], {
    cwd: root, encoding: "utf8", timeout: 10_000, maxBuffer: 256 * 1024,
  });
  expect(result.error).toBeUndefined();
  expect(result.signal).toBeNull();
  return result;
}

test("phase165 copied CLI closure discovers actual fixed runtime graphs and all omitted helpers", () => {
  // Arrange / Act
  const first = runtimeSources([ENTRIES[0]]), union = runtimeSources();
  // Assert
  expect(first.size).toBe(11);
  expect(union.size).toBe(14);
  expect([...first.keys()].every(file => union.has(file))).toBe(true);
  for (const file of MISSING_RUNTIME_HELPERS) expect(first.get(file)).toBe(readFileSync(join(REPO, file), "utf8"));
});

test("phase165 copied CLI closure preserves supplied Map bytes, order and runtime mutations", () => {
  // Arrange
  const original = new Map([["README.md", "caller-owned evidence\n"], [ENTRIES[0], "// supplied runtime mutation\n"]]);
  const before = [...original];
  // Act
  const copied = copiedCliSources(original, [ENTRIES[0]]);
  // Assert
  expect(copied).toBeInstanceOf(Map);
  expect([...copied].slice(0, before.length)).toEqual(before);
  expect([...original]).toEqual(before);
  expect(copied.get(ENTRIES[0])).toBe(original.get(ENTRIES[0]));
  for (const file of MISSING_RUNTIME_HELPERS) expect(copied.get(file)).toBe(readFileSync(join(REPO, file), "utf8"));
});

test("phase165 copied CLI closure does not expose its cached Map to caller mutation", () => {
  // Arrange
  const first = runtimeSources();
  // Act
  first.clear();
  // Assert
  expect(runtimeSources().size).toBe(14);
});

test("phase165 copied CLI closure registers empty caller-owned roots before writes without cleanup crossover", () => {
  // Arrange
  let registered = false;
  // Act
  const root = createCopiedFixture(new Map([["README.md", "caller bytes"]]), root => {
    closureRoots.push(root);
    expect(readdirSync(root)).toEqual([]);
    registered = true;
  }, [ENTRIES[0]]);
  cleanupFixtures();
  // Assert
  expect(registered).toBe(true);
  expect(existsSync(root)).toBe(true);
  expect(cleanedRoots).not.toContain(root);
  expect(readFileSync(join(root, "README.md"), "utf8")).toBe("caller bytes");
  rmSync(root, { recursive: true });
  closureRoots.splice(closureRoots.indexOf(root), 1);
  expect(existsSync(root)).toBe(false);
});

test("phase165 copied CLI closure leaves failed setup registered for caller cleanup", () => {
  // Arrange
  const owned: string[] = [];
  // Act / Assert
  expect(() => createCopiedFixture(new Map(), root => {
    owned.push(root);
    closureRoots.push(root);
    writeFileSync(join(root, "scripts"), "block directory creation");
  }, [ENTRIES[0]])).toThrow();
  expect(owned).toHaveLength(1);
  expect(cleanedRoots).not.toContain(owned[0]);
  rmSync(owned[0], { recursive: true });
  closureRoots.splice(closureRoots.indexOf(owned[0]), 1);
  expect(existsSync(owned[0])).toBe(false);
});

for (const mode of ["directory", "file"]) test(`phase165 copied CLI closure refuses caller-created ${mode} symlinks before writing outside`, () => {
  // Arrange
  const outside = closureRoot(), marker = join(outside, "marker.md");
  writeFileSync(marker, "unchanged outside witness");
  const file = mode === "directory" ? "scripts/marker.md" : "README.md";
  // Act / Assert
  for (const fixtureMode of ["executable", "evidence-only"] as const) expect(() => createCopiedFixture(new Map([[file, "must not escape"]]), root => {
    closureRoots.push(root);
    symlinkSync(mode === "directory" ? outside : marker, join(root, mode === "directory" ? "scripts" : "README.md"), mode === "directory" ? "dir" : "file");
  }, [ENTRIES[0]], fixtureMode)).toThrow("symlink bounded fixture path");
  expect(readFileSync(marker, "utf8")).toBe("unchanged outside witness");
  expect(readdirSync(outside)).toEqual(["marker.md"]);
});

test("phase165 copied CLI closure reaches the strict missing-document evidence category", () => {
  // Arrange
  const root = missingDocumentFixture();
  // Act
  const result = copiedClosureExit(root);
  // Assert
  expect(result.status).toBe(1);
  expect(result.stderr).toContain("cannot read required evidence");
  expect(result.stderr.length).toBeLessThan(10_000);
  expect(result.stderr).not.toContain("Cannot find module");
});

for (const file of MISSING_RUNTIME_HELPERS) test(`phase165 copied CLI closure deleted helper remains a loader failure: ${file}`, () => {
  // Arrange
  const root = missingDocumentFixture();
  expect(readFileSync(join(root, file), "utf8")).toBe(readFileSync(join(REPO, file), "utf8"));
  unlinkSync(join(root, file));
  // Act
  const result = copiedClosureExit(root);
  // Assert
  expect(result.status).toBe(1);
  expect(result.stderr).toContain("Cannot find module");
  expect(result.stderr).not.toContain("cannot read required evidence");
});

test("phase165 copied CLI closure parser errors cannot substitute for evidence refusal", () => {
  // Arrange
  const root = missingDocumentFixture(), file = join(root, MISSING_RUNTIME_HELPERS[0]);
  writeFileSync(file, readFileSync(file, "utf8") + "\nconst = ;\n");
  // Act
  const result = copiedClosureExit(root);
  // Assert
  expect(result.status).toBe(1);
  expect(result.stderr).toMatch(/SyntaxError|Expected|Unexpected/);
  expect(result.stderr).toContain("contracts.ts");
  expect(result.stderr).not.toContain("cannot read required evidence");
});

for (const entries of [[], ["README.md"], [ENTRIES[0], ENTRIES[0]]]) test(`phase165 copied CLI closure refuses invalid fixed roots: ${JSON.stringify(entries)}`, () => {
  // Arrange / Act / Assert
  expect(() => runtimeSources(entries)).toThrow("fixed runtime entry subset");
});

for (const file of ["../outside.md", "scripts/../outside.md", "/tmp/outside.md", "scripts\\outside.md"]) test(`phase165 copied CLI closure refuses unsafe evidence paths before creation: ${file}`, () => {
  // Arrange
  const registered: string[] = [];
  // Act / Assert
  for (const mode of ["executable", "evidence-only"] as const) {
    expect(() => createCopiedFixture(new Map([[file, "unsafe"]]), root => registered.push(root), ENTRIES, mode)).toThrow("bounded fixture path");
  }
  expect(registered).toEqual([]);
});

test("phase165 copied CLI closure enforces the combined evidence bound before creation", () => {
  // Arrange
  const sources = new Map(Array.from({ length: 401 }, (_, index) => [`evidence/${index}.md`, "bounded evidence"]));
  const registered: string[] = [];
  // Act / Assert
  for (const mode of ["executable", "evidence-only"] as const) {
    expect(() => createCopiedFixture(sources, root => registered.push(root), ENTRIES, mode)).toThrow("evidence bound");
  }
  expect(registered).toEqual([]);
});

for (const [label, addition, category] of [
  ["outside root", 'import "../../../outside.ts";', "bounded fixture path"],
  ["bare package", 'import "unsupported-package";', "runtime package"],
  ["nonliteral import", 'const name = "./other.ts"; void import(name);', "nonliteral runtime import"],
  ["computed template import", 'const name = "./other.ts"; void `${import(name)}`;', "nonliteral runtime import"],
  ["unsupported kind", 'import "./other.css";', "bounded runtime import"],
] as const) test(`phase165 copied CLI closure refuses ${label} runtime edges`, () => {
  // Arrange
  const root = isolatedRuntime(), entry = join(root, ENTRIES[0]);
  writeFileSync(entry, readFileSync(entry, "utf8") + "\n" + addition + "\n");
  // Act / Assert
  expect(() => runtimeSources([ENTRIES[0]], root)).toThrow(category);
});

test("phase165 copied CLI closure follows side-effect, reexport and literal dynamic edges after the shebang", () => {
  // Arrange
  const root = isolatedRuntime(), entry = join(root, ENTRIES[0]);
  writeFileSync(entry, readFileSync(entry, "utf8") + '\nimport "./check-phase164-canonical-identity/contracts.ts";\nexport * from "./check-phase164-canonical-identity/contracts.ts";\nvoid import("./check-phase164-canonical-identity/contracts.ts");\nimport type { Missing } from "./type-only-missing.ts";\n');
  // Act
  const graph = runtimeSources([ENTRIES[0]], root);
  // Assert
  expect(graph.size).toBe(11);
  expect(graph.has(MISSING_RUNTIME_HELPERS[0])).toBe(true);
  expect(graph.has("scripts/type-only-missing.ts")).toBe(false);
});

for (const mode of ["missing", "directory", "symlink", "directory symlink"]) test(`phase165 copied CLI closure refuses ${mode} required modules`, () => {
  // Arrange
  const root = isolatedRuntime(), file = join(root, MISSING_RUNTIME_HELPERS[0]);
  unlinkSync(file);
  if (mode === "directory") mkdirSync(file);
  if (mode === "symlink") symlinkSync(join(isolatedRuntime(), MISSING_RUNTIME_HELPERS[0]), file);
  if (mode === "directory symlink") {
    rmSync(dirname(file), { recursive: true });
    symlinkSync(join(isolatedRuntime(), "scripts/check-phase164-canonical-identity"), dirname(file), "dir");
  }
  // Act / Assert
  expect(() => runtimeSources([ENTRIES[0]], root)).toThrow();
});

test("phase165 copied CLI closure enforces the combined runtime module bound", () => {
  // Arrange
  const root = isolatedRuntime();
  writeFileSync(join(root, ENTRIES[0]), 'import "./bounded-0.ts";\n');
  for (let index = 0; index < 32; index++) {
    writeFileSync(join(root, `scripts/bounded-${index}.ts`), index === 31 ? "export {};\n" : `import "./bounded-${index + 1}.ts";\n`);
  }
  // Act / Assert
  expect(() => runtimeSources([ENTRIES[0]], root)).toThrow("import-tree bound");
});

test("phase165 copied CLI closure accepts only separately supplied fixed JSON runtime evidence", () => {
  // Arrange
  const root = isolatedRuntime(), entry = join(root, ENTRIES[0]);
  const manifestFile = "packages/testdata/phase164/historical-tooling/manifest.json";
  writeFileSync(entry, readFileSync(entry, "utf8") + `\nvoid import("../${manifestFile}");\n`);
  // Act / Assert
  expect(() => runtimeSources([ENTRIES[0]], root)).toThrow("required fixed runtime evidence missing");
  expect(runtimeSources([ENTRIES[0]], root, new Map([[manifestFile, "caller supplied JSON"]])).size).toBe(11);
});

test("phase165 copied CLI closure preserves silent data-independent current CLI imports", () => {
  // Arrange
  const root = closureRoot();
  const imports = ["scripts/check-phase164-canonical-identity.ts", "scripts/check-phase165-shared-exact-work.ts"]
    .map(file => `await import(${JSON.stringify(join(REPO, file))});`).join("\n");
  // Act
  const result = spawnSync(process.execPath, ["-e", imports], { cwd: root, encoding: "utf8", timeout: 10_000 });
  // Assert
  expect(result.error).toBeUndefined();
  expect(result.status).toBe(0);
  expect(result.stdout).toBe("");
  expect(result.stderr).toBe("");
  expect(readdirSync(root)).toEqual([]);
});

for (const [file, inverse] of COPIED_CLI_INVERSES) for (const mode of ["stale", "unknown", "missing"] as const) test(`phase165 copied CLI generation direct supplied source refuses ${mode}: ${file}`, () => {
  // Arrange: establish valid unchanged H/E/Rust/filter inputs before changing one supplied copier.
  const sources = currentBoundInputs();
  const current = sources.get(file)!;
  expect(rawSha(current)).toBe(inverse.currentRawSha);
  const original = [...sources];
  expect(() => historicalFixtureAdapter(sources)).not.toThrow();
  expect([...sources]).toEqual(original);
  const maybeBefore = maybeRestoreReviewedSource(current, inverse);
  expect(maybeBefore).toBeDefined();
  // Act
  if (mode === "missing") sources.delete(file);
  else sources.set(file, mode === "stale" ? maybeBefore! : current + "\n");
  // Assert: imported current code cannot qualify stale, missing or unknown PASSED bytes.
  expect(() => historicalFixtureAdapter(sources)).toThrow(`${file}: reviewed current copied CLI generation`);
});

for (const [file, inverse] of COPIED_CLI_INVERSES) {
  test(`phase165 copied CLI inverse reconstructs complete historical chains: ${file}`, () => {
    // Arrange
    const sources = loadSources(), current = sources.get(file)!;
    const binding = manifest.bindings.find(row => row.file === file)!;
    const snapshot = sources.get(binding.snapshot)!;
    // Act
    const maybeBefore = maybeRestoreReviewedSource(current, inverse);
    // Assert: complete returned strings retain authenticated before and original generations.
    expect(maybeBefore).toBeDefined();
    expect(rawSha(maybeBefore!)).toBe(inverse.restoredRawSha);
    expect(legacyToolingBase(file, current)).toBe(legacyToolingBase(file, maybeBefore!));
    expect(rawSha(legacyToolingBase(file, current))).toBe(binding.current_sha);
    for (const candidate of [current, maybeBefore!, legacyToolingBase(file, current), snapshot]) {
      const supplied = new Map(sources).set(file, candidate);
      for (const optional of [TOOLING_SOURCE, ADAPTER_SOURCE, "scripts/check-phase159-filter-rpcs.ts", "scripts/check-phase161-operator-index-evidence.ts"]) supplied.delete(optional);
      const original = [...supplied];
      expect(historicalToolingView(supplied).get(file)).toBe(snapshot);
      expect([...supplied]).toEqual(original);
    }
  });
  test(`phase165 copied CLI inverse refuses unknown and missing data without checkout fallback: ${file}`, () => {
    // Arrange: authenticate every other owner, so its fingerprint cannot mask this refusal.
    const sources = loadSources(), current = sources.get(file)!;
    for (const binding of manifest.bindings) sources.set(binding.file, sources.get(binding.snapshot)!);
    // Act / Assert
    for (const candidate of [current, maybeRestoreReviewedSource(current, inverse)!, sources.get(file)!]) {
      const unproved = candidate + "\n";
      sources.set(file, unproved);
      expect(legacyToolingBase(file, unproved)).toBe(unproved);
      expect(() => historicalToolingView(sources)).toThrow(`${file}: unreviewed current tooling fingerprint`);
    }
    sources.delete(file);
    expect(() => historicalToolingView(sources)).toThrow(`${file}: unreviewed current tooling fingerprint`);
  });
  for (const [name, invalid] of [
    ["wrong current hash", { ...inverse, currentRawSha: "0".repeat(64) }],
    ["wrong before hash", { ...inverse, restoredRawSha: "0".repeat(64) }],
    ["missing hunks", { ...inverse, hunks: [] }],
    ["partial hunks", { ...inverse, hunks: inverse.hunks.slice(0, -1) }],
    ["duplicate hunk", { ...inverse, hunks: [inverse.hunks[0], ...inverse.hunks] }],
    ["reordered hunks", { ...inverse, hunks: inverse.hunks.toReversed() }],
    ["overlapping spans", { ...inverse, hunks: [inverse.hunks[0], [inverse.hunks[0][0].slice(1), ""], ...inverse.hunks.slice(1)] }],
    ["no-op span", { ...inverse, hunks: [[inverse.hunks[0][0], inverse.hunks[0][0]]] }],
    ["empty span", { ...inverse, hunks: [["", "unproved"]] }],
    ["forged descriptor", { ...inverse, unreviewed: true }],
  ] as const) test(`phase165 copied CLI inverse rejects ${name}: ${file}`, () => {
    // Arrange / Act / Assert
    expect(maybeRestoreReviewedSource(readFileSync(join(REPO, file), "utf8"), invalid)).toBeUndefined();
  });
}

for (const change of COPIED_CLI_MUTATIONS) test(`phase165 copied CLI generation fresh paired ${change[0]}`, () => {
  // Arrange / Act / Assert: both actual fixed inventories require these sources.
  expectCopiedAdapterCase(change, load165Inputs(), load165Inputs, [[CLI_MODULES[2], checkPhase164], [CLI_MODULES[0], checkPhase165]]);
});

test("phase165 copied CLI evidence mode preserves exact caller membership, order and bytes", () => {
  // Arrange
  const empty = "scripts/check-phase164-canonical-identity/fixture-source.ts";
  const sources = new Map([["README.md", "caller evidence"], [ENTRIES[0], "// caller runtime mutation\n"], [empty, ""]]);
  const before = [...sources];
  // Act
  const root = createCopiedFixture(sources, root => closureRoots.push(root), ENTRIES, "evidence-only");
  // Assert
  expect([...sources]).toEqual(before);
  for (const [file, source] of before) expect(readFileSync(join(root, file), "utf8")).toBe(source);
  expect([...loadPhase162OptionalSources(root).keys()].sort()).toEqual(before.map(([file]) => file).filter(file => PROVENANCE_FILES.includes(file)).sort());
  for (const file of [...MISSING_RUNTIME_HELPERS, "scripts/check-phase164-canonical-identity/historical-tooling.ts"]) expect(existsSync(join(root, file))).toBe(false);
});
for (const mode of ["executable", "evidence-only"] as const) test(`phase165 copied CLI evidence mode empty caller Map: ${mode}`, () => {
  // Arrange / Act / Assert: the omitted mode must retain the executable default.
  const register = (root: string) => closureRoots.push(root);
  const root = mode === "executable" ? createCopiedFixture(new Map(), register) : createCopiedFixture(new Map(), register, ENTRIES, mode);
  expect(existsSync(join(root, ENTRIES[0]))).toBe(mode === "executable");
  if (mode === "evidence-only") expect(readdirSync(root)).toEqual([]);
});
test("phase165 copied CLI evidence mode failed setup remains caller-owned", () => {
  // Arrange / Act / Assert
  expect(() => createCopiedFixture(new Map([["scripts/marker.md", "blocked"]]), root => {
    closureRoots.push(root);
    writeFileSync(join(root, "scripts"), "blocked directory");
  }, ENTRIES, "evidence-only")).toThrow("non-directory bounded fixture component");
});
test("phase165 copied CLI evidence mode refuses unknown modes before registration", () => {
  // Arrange / Act / Assert
  const registered: string[] = [];
  expect(() => Reflect.apply(createCopiedFixture, undefined, [new Map(), (root: string) => registered.push(root), ENTRIES, "unknown"])).toThrow("fixture mode");
  expect(registered).toEqual([]);
});
