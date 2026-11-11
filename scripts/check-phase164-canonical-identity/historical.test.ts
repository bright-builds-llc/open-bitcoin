import { afterEach, expect, setDefaultTimeout, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import manifest from "../../packages/testdata/phase164/historical-tooling/manifest.json";
import { historicalToolingView, MANIFEST_FILE } from "./historical-tooling.ts";
import { loadSources } from "../check-phase162-integrated-filter-proof.ts";
import { PROVENANCE_DIGEST_FILES, PROVENANCE_FILES, PHASE163, provenanceMetadata } from "../check-phase163-provenance.ts";
import { checkPhase164, historicalFixtureAdapter, loadInputs } from "../check-phase164-canonical-identity.ts";
import { createHash } from "node:crypto";
import { FIXTURE_ADAPTER_INVERSE, PHASE162_SOURCE_INVERSE, maybeRestoreReviewedSource } from "./contracts.ts";
import { MIGRATED_FIXTURE } from "./fixture-source.ts";
import { checkPhase165, loadInputs as load165Inputs } from "../check-phase165-shared-exact-work.ts";
import { CLI_MODULES, COPIED_CLI_MUTATIONS, expectCopiedAdapterCase } from "../check-phase165-shared-exact-work/test-support.ts";
import { DOC, loadPhase162OptionalSources } from "../check-phase158-validated-reorg.ts";
import { ENTRIES, REPO, assertLiveUnchanged, cleanedRoots, cleanupFixtures, copiedCliSources, createCopiedFixture, runtimeSources } from "../check-phase163-provenance/fixtures.ts";
setDefaultTimeout(20_000); // Allow slower CI child-process inverse checks.
for (const row of manifest.bindings) {
  for (const file of [row.snapshot]) {
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

const ADAPTER_SOURCE = "scripts/check-phase164-canonical-identity/fixture-source.ts";
test("phase165 filter guard current generation qualifies complete actual PASSED inputs", () => {
  // Arrange
  const sources = loadInputs();
  const original = [...sources];
  // Act
  const adapter = historicalFixtureAdapter(sources);
  // Assert
  expect(rawSha(adapter)).toBe("b92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3");
  expect([...sources]).toEqual(original);
});

const CURRENT_GENERATIONS = [
  [ADAPTER_SOURCE, FIXTURE_ADAPTER_INVERSE, "paired current raw fixture/adapter generation"],
  [MIGRATED_FIXTURE, PHASE162_SOURCE_INVERSE, "paired current raw fixture/adapter generation"],
] as const;
for (const [file, inverse, detail] of CURRENT_GENERATIONS) for (const mode of ["historical", "unknown"] as const) test(`phase165 filter guard current generation refuses ${mode} PASSED source: ${file}`, () => {
  // Arrange: the imported adapter remains current while the explicit map generation changes.
  const sources = loadInputs();
  expect(() => historicalFixtureAdapter(sources)).not.toThrow();
  const current = sources.get(file)!;
  const maybeHistorical = maybeRestoreReviewedSource(current, inverse);
  expect(maybeHistorical).toBeDefined();
  // Act
  sources.set(file, mode === "historical" ? maybeHistorical! : current + "\n");
  // Assert
  expect(() => historicalFixtureAdapter(sources)).toThrow(detail);
});

for (const file of [ADAPTER_SOURCE, MIGRATED_FIXTURE]) test(`phase165 filter guard current generation requires PASSED source: ${file}`, () => {
  // Arrange
  const sources = loadInputs();
  sources.delete(file);
  // Act / Assert
  expect(() => historicalFixtureAdapter(sources)).toThrow();
});

for (const [phase, load, check] of [[164, loadInputs, checkPhase164], [165, load165Inputs, checkPhase165]] as const) {
  for (const [file, inverse, detail] of CURRENT_GENERATIONS) test(`phase165 filter guard full${phase} rejects stale PASSED generation: ${file}`, () => {
    // Arrange: pending ordinary SOURCE rows are separate from this history-specific gate.
    const sources = load();
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

const rawSha = (source: string) => createHash("sha256").update(source).digest("hex");

test("phase165 filter guard historical data preserves the complete original receipt digest", () => {
  // Arrange
  const sources = loadSources();
  const original = [...sources];
  // Act
  const view = historicalToolingView(sources);
  const historicalDigest = rawSha(PROVENANCE_DIGEST_FILES.map(file => `${file}:${rawSha(view.get(file) ?? "")}`).join("\n"));
  // Assert
  expect(historicalDigest).toBe(provenanceMetadata(sources.get(PHASE163 + "163-VERIFICATION.md")!).source_digest);
  expect([...sources]).toEqual(original);
});

for (const file of [MANIFEST_FILE, "scripts/check-phase164-canonical-identity/fixture-source.ts"]) {
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
