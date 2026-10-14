import { afterEach, afterAll, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { checkPhase158ValidatedReorg, PHASE162, PHASE162_DOC, PHASE162_EVIDENCE } from "../check-phase158-validated-reorg.ts";
import { checkPhase162IntegratedFilterProof } from "../check-phase162-integrated-filter-proof.ts";
import { PHASE163, PHASE163_DOC, ORIGINAL_AUDITS, V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS,
  GAP_CLAIM, CLOSURE_CLAIM, READINESS_CLAIM, ARCHIVED_CLAIM, resolveV25Provenance } from "../check-phase163-provenance.ts";
import { fixtureSources, withFixture, checkAllBoundaries, boundaries, change, writeSources,
  cleanupFixtures, assertLiveUnchanged, cleanedRoots, LAYOUTS, REPORT_NAMES, CURRENT_CLAIM_FILES,
  archiveOverlap, archiveOnly, nextActive, CORPUS_MANIFEST, CORPUS_DIGEST, CORPUS_BYTES, sha } from "./fixtures.ts";

afterEach(cleanupFixtures);
afterAll(() => {
  assertLiveUnchanged();
  expect(cleanedRoots.length).toBeGreaterThan(0);
  for (const root of cleanedRoots) expect(existsSync(root)).toBe(false);
  console.log(`Phase163 native fixtures: ${cleanedRoots.length} owned roots removed; ${CORPUS_MANIFEST.length} finite files/${CORPUS_BYTES} bytes; corpus ${CORPUS_DIGEST}; live controls/proof hashes unchanged.`);
});

test("actual native entrypoints and hermetic maps accept linked Pending without future receipts", () => {
  // Arrange
  const sources = fixtureSources("gap-pending");
  // Act / Assert
  withFixture(sources, root => expect(checkAllBoundaries(root, sources)).toEqual([]));
}, 30_000);

for (const layout of LAYOUTS.filter(layout => layout !== "gap-pending")) {
  test(`actual CLI/filesystem/map agreement: ${layout}`, () => {
    // Arrange
    const sources = fixtureSources(layout);
    if (layout.startsWith("original-")) expect(sources.has(PHASE163_DOC)).toBe(false);
    // Act / Assert
    withFixture(sources, root => expect(checkAllBoundaries(root, sources)).toEqual([]));
  }, 30_000);
}

for (const layout of ["archive-only", "next-active"] as const) {
  test(`historical native builders execute all ten layouts from ${layout} checkout controls`, () => {
    // Arrange
    const sources = fixtureSources(layout);
    // Act / Assert
    withFixture(sources, root => {
      const result = spawnSync(process.execPath, ["test", "./scripts/check-phase163-provenance/native.test.ts",
        "--test-name-pattern", "^actual CLI/filesystem/map agreement: |^actual native entrypoints and hermetic maps accept linked Pending without future receipts$"],
      { cwd: root, encoding: "utf8", timeout: 55_000, maxBuffer: 256 * 1024 });
      expect(result.error).toBeUndefined();
      expect(result.signal).toBeNull();
      expect(result.status, result.stderr).toBe(0);
      expect(result.stderr).toContain("10 pass");
      expect(result.stderr).toContain("0 fail");
    });
  }, 60_000);
}

for (const [name, file, operation] of [
  ["absent active control creation", V25_REQUIREMENTS[0], "create"],
  ["present archived control deletion", V25_REQUIREMENTS[1], "delete"],
  ["present active roadmap bytes", V25_ROADMAPS[0], "change"],
  ["mandatory historical proof deletion", PHASE162_DOC, "delete"],
] as const) {
  test(`archived fixture live witness refuses ${name}`, () => {
    // Arrange
    const sources = fixtureSources("archive-only");
    const program = `
      import { writeFileSync, unlinkSync } from "node:fs";
      import { assertLiveUnchanged } from "./scripts/check-phase163-provenance/fixtures.ts";
      assertLiveUnchanged();
      const file = ${JSON.stringify(file)};
      if (${JSON.stringify(operation)} === "delete") unlinkSync(file);
      else writeFileSync(file, "unexpected fixture mutation\\n");
      let refused = false;
      try { assertLiveUnchanged(); } catch (error) {
        refused = String(error).includes(file);
      }
      if (!refused) throw new Error("live witness accepted mutation: " + file);
    `;
    // Act / Assert
    withFixture(sources, root => {
      const result = spawnSync(process.execPath, ["--eval", program],
        { cwd: root, encoding: "utf8", timeout: 15_000, maxBuffer: 256 * 1024 });
      expect(result.error).toBeUndefined();
      expect(result.signal).toBeNull();
      expect(result.status, result.stderr).toBe(0);
    });
  }, 30_000);
}

test("real closeout sequence preserves safety-copy overlap, archive-only and next-active controls", () => {
  // Arrange
  const sources = fixtureSources("final");
  const historical = new Map([PHASE162_DOC, ...PHASE162_EVIDENCE, ...REPORT_NAMES.slice(0, 4).map(name => PHASE162 + `162-${name}.md`)]
    .map(file => [file, sha(sources.get(file)!)]));
  // Act / Assert
  withFixture(sources, root => {
    expect(checkAllBoundaries(root, sources)).toEqual([]);
    archiveOverlap(sources);
    writeSources(root, sources);
    unlinkSync(join(root, V25_AUDITS[0]));
    expect(existsSync(join(root, V25_REQUIREMENTS[0]))).toBe(true);
    expect(readFileSync(join(root, V25_REQUIREMENTS[1]), "utf8")).toContain("**Status:** SHIPPED");
    expect(checkAllBoundaries(root, sources)).toEqual([]);
    archiveOnly(sources);
    unlinkSync(join(root, V25_REQUIREMENTS[0]));
    writeSources(root, sources);
    for (const file of CURRENT_CLAIM_FILES) expect(sources.get(file)).toContain(ARCHIVED_CLAIM);
    expect(checkAllBoundaries(root, sources)).toEqual([]);
    nextActive(sources);
    writeSources(root, sources);
    expect(checkAllBoundaries(root, sources)).toEqual([]);
    for (const [file, hash] of historical) expect(sha(readFileSync(join(root, file), "utf8"))).toBe(hash);
  });
}, 60_000);

test("optional current parity page may be absent before its creation", () => {
  // Arrange
  const sources = fixtureSources("gap-pending");
  expect(sources.delete(PHASE163_DOC)).toBe(true);
  // Act / Assert
  withFixture(sources, root => expect(checkAllBoundaries(root, sources)).toEqual([]));
}, 30_000);

for (const [layout, claim] of [
  ["gap-pending", GAP_CLAIM], ["functional-closure", CLOSURE_CLAIM], ["audit-in-progress", READINESS_CLAIM],
  ["final", READINESS_CLAIM], ["archive-only", ARCHIVED_CLAIM], ["next-active", ARCHIVED_CLAIM],
] as const) {
  test(`optional current parity page accepts earned ${layout} wording at every boundary`, () => {
    // Arrange
    const sources = fixtureSources(layout);
    expect(sources.get(PHASE163_DOC)).toContain(claim);
    // Act / Assert
    withFixture(sources, root => expect(checkAllBoundaries(root, sources)).toEqual([]));
  }, 30_000);
}

for (const [name, layout, before, after] of [
  ["premature closure", "gap-pending", GAP_CLAIM, CLOSURE_CLAIM],
  ["premature readiness", "gap-pending", GAP_CLAIM, READINESS_CLAIM],
  ["premature archive", "gap-pending", GAP_CLAIM, ARCHIVED_CLAIM],
  ["functional closure cannot assert readiness", "functional-closure", CLOSURE_CLAIM, READINESS_CLAIM],
  ["shipment", "gap-pending", GAP_CLAIM, "v2.5 is shipped."],
  ["production", "gap-pending", GAP_CLAIM, "Production service is ready."],
  ["quoted shipment", "gap-pending", GAP_CLAIM, "`v2.5 is shipped.`"],
  ["blockquote production", "gap-pending", GAP_CLAIM, "> Production service is ready."],
  ["fenced shipment", "gap-pending", GAP_CLAIM, "```text\nv2.5 is shipped.\n```"],
  ["stale archived closure", "archive-only", ARCHIVED_CLAIM, CLOSURE_CLAIM],
  ["stale archived readiness", "archive-only", ARCHIVED_CLAIM, READINESS_CLAIM],
] as const) {
  test(`optional current parity page targeted refusal: ${name}`, () => {
    // Arrange
    const sources = fixtureSources(layout);
    change(sources, PHASE163_DOC, source => source.replace(before, after));
    // Act / Assert
    refuses(sources, new RegExp(`${PHASE163_DOC.replaceAll(".", "\\.")}: unsupported claim`));
  }, 30_000);
}

type Mutation = { name: string; file: string; before: string; after: string; reason: RegExp; maybeLayout?: "gap-pending" | "functional-closure" | "final" | "archive-only" | "overlap" };
const scope = PHASE163 + "163-GAP-SCOPE.md";
const context = PHASE163 + "163-CONTEXT.md";
const verification = PHASE163 + "163-VERIFICATION.md";
const mutations: Mutation[] = [
  { name: "wrong milestone", file: V25_REQUIREMENTS[0], before: "**Milestone:** v2.5", after: "**Milestone:** v2.6", reason: /identity\/baseline/ },
  { name: "wrong baseline", file: V25_REQUIREMENTS[0], before: "29.3.knots20260210", after: "29.3.knots20260101", reason: /identity\/baseline/ },
  { name: "wrong owner", file: V25_REQUIREMENTS[0], before: "Phase 163 | Pending", after: "Phase 164 | Pending", reason: /wrong owning phase/ },
  { name: "checkbox-only completion", file: V25_REQUIREMENTS[0], before: "[ ] **CFGR-01**", after: "[x] **CFGR-01**", reason: /checkbox\/traceability consistency/ },
  { name: "duplicate checkbox", file: V25_REQUIREMENTS[0], before: "- [ ] **CFGR-01**:", after: "- [ ] **CFGR-01**: duplicate\n- [ ] **CFGR-01**:", reason: /22 unique IDs\/owners/ },
  { name: "duplicate roadmap owner", file: V25_ROADMAPS[0], before: "**Requirements**: CFPR-02, CFGR-02", after: "**Requirements**: CFPR-02, CFGR-01, CFGR-02", reason: /unique roadmap\/requirements owner/ },
  { name: "wrong dependency", file: scope, before: "depends_on: [162]", after: "depends_on: [161]", reason: /approved Phase163 scope\/current lifecycle/ },
  { name: "wrong scope", file: scope, before: "audit_gaps: [AG01, AF01]", after: "audit_gaps: [AG01]", reason: /approved Phase163 scope\/current lifecycle/ },
  { name: "wrong creation lifecycle", file: scope, before: "163-2026-10-12T21-03-37", after: "163-stale", reason: /approved Phase163 scope\/current lifecycle/ },
  { name: "wrong current lifecycle", file: context, before: "163-2026-10-12T21-09-41", after: "163-stale", reason: /approved Phase163 scope\/current lifecycle/ },
  { name: "duplicate metadata", file: context, before: "status: approved-scope", after: "status: approved-scope\nstatus: approved-scope", reason: /duplicate metadata status/ },
  { name: "quoted duplicate metadata", file: context, before: "status: approved-scope", after: 'status: denied\n"status": approved-scope', reason: /noncanonical metadata/ },
  { name: "malformed metadata", file: context, before: "requirements: [CFGR-01]", after: "requirements: [CFGR-01", reason: /YAML|parse|end|expected/i },
  { name: "current gap linkage", file: V25_AUDITS[0], before: "id: CFGR-01", after: "id: CFGR-02", reason: /current audit CFGR-01\/Phase162\/AG01\/AF01 gap linkage/ },
  { name: "wrong audit generator", file: V25_AUDITS[0], before: "gsd-audit-milestone", after: "gsd-executor", reason: /invalid v2.5 milestone audit/ },
  { name: "original lineage tamper", file: ORIGINAL_AUDITS[0], before: "status: gaps_found", after: "status: passed", reason: /original audit lineage/ },
  { name: "wrong closure generator", file: verification, before: "gsd-verifier", after: "gsd-executor", reason: /fresh Phase163 VERIFICATION gate/, maybeLayout: "functional-closure" },
  { name: "wrong canonical scope", file: verification, before: "functional-closure", after: "internal", reason: /fresh Phase163 04-SUMMARY gate|scope\/coverage/, maybeLayout: "functional-closure" },
  { name: "stale report lifecycle", file: verification, before: "163-2026-10-12T21-09-41", after: "163-stale", reason: /fresh Phase163 VERIFICATION gate/, maybeLayout: "functional-closure" },
  { name: "stale report digest", file: verification, before: "source_digest: ", after: "source_digest: stale", reason: /fresh Phase163 VERIFICATION gate/, maybeLayout: "functional-closure" },
  { name: "stale report timestamp", file: verification, before: "2026-10-12T23:00:00Z", after: "2026-10-11T23:00:00Z", reason: /fresh Phase163 VERIFICATION gate/, maybeLayout: "functional-closure" },
  { name: "premature readiness", file: verification, before: "readiness: not_asserted", after: "readiness: established", reason: /functional-closure must leave audit\/readiness\/Phase163 pending/, maybeLayout: "functional-closure" },
  { name: "premature phase completion", file: verification, before: "phase_status: in_progress", after: "phase_status: complete", reason: /functional-closure must leave audit\/readiness\/Phase163 pending/, maybeLayout: "functional-closure" },
  { name: "unearned security", file: PHASE163 + "163-SECURITY.md", before: "threats_open: 0", after: "threats_open: 1", reason: /actual full native\/security gate/, maybeLayout: "functional-closure" },
  { name: "unearned native", file: PHASE163 + "163-NATIVE-EVIDENCE.md", before: "exit_code: 0", after: "exit_code: 1", reason: /actual full native\/security gate/, maybeLayout: "functional-closure" },
  { name: "wrong summary activation", file: PHASE163 + "163-04-SUMMARY.md", before: "requirements-completed: [CFGR-01]", after: "requirements-completed: []", reason: /SUMMARY activation/, maybeLayout: "functional-closure" },
  { name: "gaps audit cannot authorize final", file: V25_AUDITS[0], before: "status: passed", after: "status: gaps_found", reason: /current audit CFGR-01\/Phase162\/AG01\/AF01 gap linkage/, maybeLayout: "final" },
  { name: "audit gaps cannot authorize readiness", file: V25_AUDITS[0], before: "requirements: []", after: "requirements: [CFGR-01]", reason: /renewed passing v2.5 audit/, maybeLayout: "final" },
  { name: "stale audit digest", file: V25_AUDITS[0], before: "source_digest: ", after: "source_digest: stale", reason: /renewed passing v2.5 audit/, maybeLayout: "final" },
  { name: "stale audit date", file: V25_AUDITS[0], before: "2026-10-12T23:01:00Z", after: "2026-10-11T23:01:00Z", reason: /renewed passing v2.5 audit/, maybeLayout: "final" },
  { name: "archived stale closure prose", file: "docs/parity/index.json", before: ARCHIVED_CLAIM, after: CLOSURE_CLAIM, reason: /unsupported claim/, maybeLayout: "archive-only" },
  { name: "archived stale readiness prose", file: "docs/parity/index.json", before: ARCHIVED_CLAIM, after: READINESS_CLAIM, reason: /unsupported claim/, maybeLayout: "archive-only" },
  { name: "excluded production claim", file: "docs/parity/index.json", before: GAP_CLAIM, after: "Production service is ready.", reason: /unsupported claim/ },
  { name: "excluded shipment claim", file: "docs/parity/index.json", before: GAP_CLAIM, after: "v2.5 is shipped.", reason: /unsupported claim/ },
  { name: "historical proof lifecycle", file: PHASE162_DOC, before: "162-2026-10-11T16-25-37", after: "162-stale", reason: /linked current-lifecycle Phase 162 implementation/ },
  { name: "historical certificate owner", file: PHASE162 + "162-VERIFICATION.md", before: "gsd-verifier", after: "gsd-executor", reason: /fresh scoped Phase 162 VERIFICATION/ },
  { name: "frozen helper", file: "scripts/check-phase163-provenance.ts", before: "import { createHash }", after: "// fixture tamper\nimport { createHash }", reason: /fresh Phase163 VERIFICATION gate|frozen Plan15/, maybeLayout: "functional-closure" },
  { name: "frozen guard suite", file: "scripts/check-phase158-validated-reorg.test.ts", before: 'import { afterEach', after: '// fixture tamper\nimport { afterEach', reason: /fresh Phase163 VERIFICATION gate|frozen Plan15/, maybeLayout: "functional-closure" },
];

function refuses(sources: Map<string, string>, reason: RegExp, maybeMissingFile?: string): void {
  withFixture(sources, root => {
    for (const result of boundaries(root, sources)) {
      expect(result.failures.length, result.name).toBeGreaterThan(0);
      const missingNative = maybeMissingFile && (result.name === "162-filesystem" || result.name === "scripts/check-phase162-integrated-filter-proof.ts");
      const expected = missingNative ? new RegExp(`ENOENT[^\\n]*${maybeMissingFile.replaceAll(".", "\\.")}`) : reason;
      expect(result.failures.join("\n"), result.name).toMatch(expected);
    }
  });
}

for (const mutation of mutations) {
  test(`targeted native/map refusal: ${mutation.name}`, () => {
    // Arrange
    const sources = fixtureSources(mutation.maybeLayout ?? "gap-pending");
    change(sources, mutation.file, source => source.replace(mutation.before, mutation.after));
    // Act / Assert
    refuses(sources, mutation.reason);
  }, 30_000);
}

test("duplicate requirement rows refuse at every real boundary", () => {
  // Arrange
  const sources = fixtureSources("gap-pending");
  change(sources, V25_REQUIREMENTS[0], source => source + "\n| CFGR-01 | Phase 163 | Pending |\n");
  // Act / Assert
  refuses(sources, /22 unique IDs\/owners/);
}, 30_000);

test("plan-only completion cannot activate a Pending requirement", () => {
  // Arrange
  const sources = fixtureSources("gap-pending");
  change(sources, V25_ROADMAPS[0], source => source + "\n- [x] Plan163 complete\n");
  change(sources, "docs/parity/index.json", source => source.replace(GAP_CLAIM, CLOSURE_CLAIM));
  // Act / Assert
  refuses(sources, /unsupported claim/);
}, 30_000);

const missingKeys = [V25_REQUIREMENTS[0], V25_ROADMAPS[0], V25_AUDITS[0], ...ORIGINAL_AUDITS, scope, context,
  PHASE162_DOC, ...PHASE162_EVIDENCE, ...REPORT_NAMES.slice(0, 4).map(name => PHASE162 + `162-${name}.md`),
  ...REPORT_NAMES.map(name => PHASE163 + `163-${name}.md`)];
for (const file of missingKeys) {
  const reason = file === V25_REQUIREMENTS[0] || file === V25_ROADMAPS[0] ? /missing v2.5 source/
    : file === V25_AUDITS[0] ? /missing current v2.5 gap audit/ : ORIGINAL_AUDITS.includes(file) ? /original audit lineage/
    : file === scope || file === context || file.startsWith(PHASE163) ? /missing first frontmatter/
    : file === PHASE162_DOC ? /premature Phase 162 activation/ : PHASE162_EVIDENCE.includes(file) ? /Phase 162 source hash/
    : /fresh scoped Phase 162/;
  test(`missing selected key refuses filesystem/CLI/maps: ${file}`, () => {
    // Arrange
    const sources = fixtureSources("functional-closure");
    expect(sources.delete(file)).toBe(true);
    // Act / Assert
    refuses(sources, reason, PHASE162_EVIDENCE.includes(file) ? file : undefined);
  }, 30_000);
  test(`omitted map key cannot borrow valid disk evidence: ${file}`, () => {
    // Arrange
    const sources = fixtureSources("functional-closure");
    withFixture(sources, root => {
      const omitted = new Map(sources);
      expect(omitted.delete(file)).toBe(true);
      expect(existsSync(join(root, file))).toBe(true);
      // Act
      const failures = [checkPhase158ValidatedReorg(root, omitted), checkPhase162IntegratedFilterProof(omitted)];
      // Assert
      for (const result of failures) expect(result.join("\n")).toMatch(reason);
    });
  }, 30_000);
}

test("active whitespace requirements conflict cannot hide behind a valid archive", () => {
  // Arrange
  const sources = fixtureSources("final");
  sources.set(V25_REQUIREMENTS[1], sources.get(V25_REQUIREMENTS[0])!);
  change(sources, V25_REQUIREMENTS[0], source => source.replace("**Milestone:** v2.5", "**Milestone:**  v2.5"));
  change(sources, V25_REQUIREMENTS[0], source => source.replace("## v2.5 Requirements", "## Requirements"));
  change(sources, V25_REQUIREMENTS[0], source => source.replace("A client can obtain", "Conflicting client description"));
  // Act / Assert
  refuses(sources, /conflicting v2.5 candidates .*REQUIREMENTS\.md/);
}, 30_000);

test("active bare milestone roadmap conflict cannot hide behind a valid archive", () => {
  // Arrange
  const sources = fixtureSources("final");
  sources.set(V25_ROADMAPS[1], sources.get(V25_ROADMAPS[0])!);
  change(sources, V25_ROADMAPS[0], source => source.replace(/^## Active Milestone: v2\.5 .+$/m, "## Active Milestone: v2.5"));
  change(sources, V25_ROADMAPS[0], source => source.replace("**Requirements**: CFPR-02, CFGR-02", "**Requirements**: CFPR-02, CFGR-01, CFGR-02"));
  // Act / Assert
  refuses(sources, /conflicting v2.5 candidates .*ROADMAP\.md/);
}, 30_000);

type ActiveIdentityCase = { name: string; paths: string[]; transform: (source: string) => string; maybeArchiveOnly?: boolean };
const activeIdentityCases: ActiveIdentityCase[] = [
  { name: "requirements missing identity", paths: V25_REQUIREMENTS, transform: source => source.replace(/^\*\*Milestone:\*\*.*\n/m, "").replace("## v2.5 Requirements", "## Requirements") },
  { name: "roadmap missing identity", paths: V25_ROADMAPS, transform: source => source.replace(/^## Active Milestone:.*\n/m, "") },
  { name: "requirements duplicate unrelated identity", paths: V25_REQUIREMENTS, transform: () => "# Requirements\n\n**Milestone:** v2.6 Future\n**Milestone:** v2.6 Future\n" },
  { name: "roadmap duplicate unrelated identity", paths: V25_ROADMAPS, transform: () => "# Roadmap\n\n## Active Milestone: v2.6 Future\n## Active Milestone: v2.6 Future\n" },
  { name: "requirements blank identity cannot consume next-line version", paths: V25_REQUIREMENTS, transform: () => "# Requirements\n\n**Milestone:**\n\nv2.6 Future\n" },
  { name: "roadmap blank identity cannot consume next-line version", paths: V25_ROADMAPS, transform: () => "# Roadmap\n\n## Active Milestone:\n\nv2.6 Future\n" },
  { name: "requirements blank marker cannot hide later foreign marker", paths: V25_REQUIREMENTS, transform: () => "# Requirements\n\n**Milestone:**\n**Milestone:** v2.6 Future\n" },
  { name: "roadmap blank marker cannot hide later foreign marker", paths: V25_ROADMAPS, transform: () => "# Roadmap\n\n## Active Milestone:\n## Active Milestone: v2.6 Future\n" },
  { name: "requirements foreign primary cannot contradict versioned section", paths: V25_REQUIREMENTS, transform: () => "# Requirements\n\n**Milestone:** v2.6 Future\n\n## v2.5 Requirements\n" },
  { name: "roadmap foreign active identity cannot contradict archive title", paths: V25_ROADMAPS, transform: () => "# Roadmap\n\n## Active Milestone: v2.6 Future\n\n# Milestone v2.5: Archived\n" },
  { name: "requirements earlier milestone cannot masquerade as later active", paths: V25_REQUIREMENTS, transform: () => "# Requirements\n\n**Milestone:** v2.4 Previous\n" },
  { name: "roadmap earlier milestone cannot masquerade as later active", paths: V25_ROADMAPS, transform: () => "# Roadmap\n\n## Active Milestone: v2.4 Previous\n" },
  { name: "archive index wrong fixed pointer", paths: V25_ROADMAPS, maybeArchiveOnly: true, transform: source => source.replace("v2.5-ROADMAP.md", "v2.6-ROADMAP.md") },
  { name: "archive index extra scoped control", paths: V25_ROADMAPS, maybeArchiveOnly: true, transform: source => source + "\n**Requirements**: CFGR-01\n" },
  { name: "archive index ambiguous unrelated identities", paths: V25_ROADMAPS, maybeArchiveOnly: true, transform: source => source + "\n## Active Milestone: v2.6 Future\n## Active Milestone: v2.7 Future\n" },
];

for (const row of activeIdentityCases) {
  test(`active identity routing refuses: ${row.name}`, () => {
    // Arrange
    const sources = fixtureSources(row.maybeArchiveOnly ? "archive-only" : "final");
    if (!row.maybeArchiveOnly) sources.set(row.paths[1], sources.get(row.paths[0])!);
    change(sources, row.paths[0], row.transform);
    expect(sources.has(row.paths[1])).toBe(true);
    // Act / Assert
    withFixture(sources, root => {
      for (const result of boundaries(root, sources)) {
        expect(result.failures.length, result.name).toBeGreaterThan(0);
        const maybeRefusal = result.failures.find(failure => failure.includes(`${row.paths[0]}:`) && failure.includes("active milestone identity"));
        expect(maybeRefusal, result.name).toBeDefined();
      }
    });
  }, 30_000);
}

for (const file of [V25_REQUIREMENTS[0], V25_ROADMAPS[0]]) {
  test(`present empty active control cannot become absent in adapters: ${file}`, () => {
    // Arrange
    const sources = fixtureSources("archive-only");
    sources.set(file, "");
    expect(sources.has(file)).toBe(true);
    const failures: string[] = [];
    // Act
    const maybePureState = resolveV25Provenance(sources, failures);
    // Assert
    expect(maybePureState).toBeUndefined();
    expect(failures.join("\n")).toContain(`${file}: invalid active milestone identity`);
    withFixture(sources, root => {
      expect(readFileSync(join(root, file), "utf8")).toBe("");
      for (const result of boundaries(root, sources)) {
        expect(result.failures.length, result.name).toBeGreaterThan(0);
        expect(result.failures.join("\n"), result.name).toContain(`${file}: invalid active milestone identity`);
      }
    });
  }, 30_000);
}

for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS]) {
  test(`equivalent candidate conflict refuses: ${paths[1]}`, () => {
    // Arrange
    const sources = fixtureSources("final");
    sources.set(paths[1], sources.get(paths[0])!);
    const before = paths === V25_REQUIREMENTS ? "A client can obtain" : paths === V25_ROADMAPS ? "### Phase 162:" : "22/22";
    change(sources, paths[1], source => source.replace(before, "conflict"));
    // Act / Assert
    refuses(sources, /conflicting v2.5 candidates/);
  }, 30_000);
  test(`malformed candidate refuses alongside a valid copy: ${paths[1]}`, () => {
    // Arrange
    const sources = fixtureSources("final");
    sources.set(paths[1], "malformed fixture control\n");
    // Act / Assert
    refuses(sources, /identity\/baseline|roadmap identity|missing first frontmatter/);
  }, 30_000);
}

test("archive SHIPPED envelope cannot manufacture closure or audit readiness", () => {
  // Arrange
  const sources = fixtureSources("gap-pending");
  archiveOverlap(sources);
  archiveOnly(sources);
  // Act / Assert
  refuses(sources, /archived controls require final phase closure\/passing audit/);
}, 30_000);

test("internal report cannot replace the exact canonical verification path", () => {
  // Arrange
  const sources = fixtureSources("functional-closure");
  sources.set(PHASE163 + "163-CLOSURE-VERIFICATION.md", sources.get(verification)!);
  sources.delete(verification);
  // Act / Assert
  refuses(sources, /missing first frontmatter/);
}, 30_000);

test("wrong historical owner certificate cannot replace canonical current verification", () => {
  // Arrange
  const sources = fixtureSources("functional-closure");
  sources.set(verification, sources.get(PHASE162 + "162-VERIFICATION.md")!);
  // Act / Assert
  refuses(sources, /fresh Phase163 VERIFICATION gate/);
}, 30_000);

test("source-bound executable assertion removal refuses even after proof hashes are rebound", () => {
  // Arrange
  const sources = fixtureSources("gap-pending");
  const file = PHASE162_EVIDENCE[1];
  change(sources, file, source => source.replace(".load_undo(hash)", ".load_block(hash)"));
  change(sources, PHASE162_DOC, source => source.replace(/\| `([^`]+)`\s*\|\s*`[a-f0-9]{64}`\s*\|/g,
    (_row, file) => `| \`${file}\` | \`${sha(sources.get(file)!)}\` |`));
  // Act / Assert
  refuses(sources, /actual evidence .*retention.rs:assert_pair/);
}, 30_000);

test("required source hash tampering refuses with its exact proof path", () => {
  // Arrange
  const sources = fixtureSources("gap-pending");
  const file = PHASE162_EVIDENCE[0];
  change(sources, file, source => source + "\n// stale-source control\n");
  // Act / Assert
  refuses(sources, new RegExp(`Phase 162 source hash ${file.replaceAll(".", "\\.")}`));
}, 30_000);

for (const file of [V25_REQUIREMENTS[1], V25_ROADMAPS[1], V25_AUDITS[1]]) {
  const reason = file === V25_AUDITS[1] ? /missing current v2.5 gap audit/ : /missing v2.5 source/;
  test(`missing selected archive control refuses: ${file}`, () => {
    // Arrange
    const sources = fixtureSources("archive-only");
    expect(sources.delete(file)).toBe(true);
    // Act / Assert
    refuses(sources, reason);
  }, 30_000);
  test(`omitted archived map key cannot borrow disk evidence: ${file}`, () => {
    // Arrange
    const sources = fixtureSources("next-active");
    withFixture(sources, root => {
      const omitted = new Map(sources);
      expect(omitted.delete(file)).toBe(true);
      expect(existsSync(join(root, file))).toBe(true);
      // Act / Assert
      for (const failures of [checkPhase158ValidatedReorg(root, omitted), checkPhase162IntegratedFilterProof(omitted)]) expect(failures.join("\n")).toMatch(reason);
    });
  }, 30_000);
}

test("removing the actual exclusion guard cannot escape bound source validation", () => {
  // Arrange
  const sources = fixtureSources("functional-closure");
  change(sources, "scripts/check-phase158-validated-reorg.ts", source => source.replace("PROVENANCE_CLAIMS.some(claim => view.includes(claim))", "false"));
  // Act / Assert
  refuses(sources, /fresh Phase163 VERIFICATION gate|frozen Plan15/);
}, 30_000);

test("removing executable assertion macros cannot be rescued by new proof hashes", () => {
  // Arrange
  const sources = fixtureSources("gap-pending");
  const file = PHASE162_EVIDENCE[1];
  change(sources, file, source => source.replaceAll("assert_eq!", "removed_assertion!"));
  change(sources, PHASE162_DOC, source => source.replace(/\| `([^`]+)`\s*\|\s*`[a-f0-9]{64}`\s*\|/g,
    (_row, file) => `| \`${file}\` | \`${sha(sources.get(file)!)}\` |`));
  // Act / Assert
  refuses(sources, /actual evidence .*retention.rs:assert_pair/);
}, 30_000);

test("cleanup runs when a fixture action throws", () => {
  // Arrange
  let owned = "";
  // Act
  expect(() => withFixture(fixtureSources("gap-pending"), root => { owned = root; throw new Error("intentional cleanup control"); })).toThrow("intentional cleanup control");
  // Assert
  expect(owned.length).toBeGreaterThan(0);
  expect(existsSync(owned)).toBe(false);
  assertLiveUnchanged();
});
