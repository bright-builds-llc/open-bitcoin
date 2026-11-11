import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { v25FixtureSources } from "./check-phase163-provenance/fixture-controls.ts";
import { PHASE163, PHASE163_DOC, PROVENANCE_FILES, PROVENANCE_DIGEST_FILES, V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS, ORIGINAL_AUDITS,
  resolveV25Provenance, provenanceDigest, provenanceClaimView, provenanceMetadata,
  GAP_CLAIM, CLOSURE_CLAIM, READINESS_CLAIM, ARCHIVED_CLAIM } from "./check-phase163-provenance.ts";

const root = resolve(import.meta.dir, "..");
export function liveProvenanceFixture(maybeBase?: ReadonlyMap<string, string>): Map<string, string> {
  const loaded = new Map(maybeBase);
  if (!maybeBase) for (const file of PROVENANCE_FILES) {
    try { loaded.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
  }
  const sources = v25FixtureSources(loaded);
  const requirements = sources.get(V25_REQUIREMENTS[0])!;
  sources.set(V25_REQUIREMENTS[0], requirements.replace(/^- \[[ xX]\] \*\*CFGR-01\*\*:/m, "- [ ] **CFGR-01**:")
    .replace(/(\|\s*CFGR-01\s*\|)\s*Phase 16[23]\s*\|\s*(?:Pending|Complete)\s*\|/, "$1 Phase 163 | Pending |"));
  sources.set(V25_AUDITS[0], sources.get(ORIGINAL_AUDITS[0])!);
  for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS]) sources.delete(paths[1]);
  for (const name of ["VERIFICATION", "NATIVE-EVIDENCE", "REVIEW", "SECURITY", "CLOSURE-VERIFICATION", "04-SUMMARY"]) sources.delete(PHASE163 + `163-${name}.md`);
  sources.set(PHASE163_DOC, GAP_CLAIM);
  return sources;
}

for (const layout of ["archive-only", "next-active"]) {
  test(`Pending fixture selects archived v2.5 controls in ${layout} without live fallback`, () => {
    // Arrange
    const archived = archivedProvenanceFixture();
    if (layout === "next-active") {
      archived.set(V25_REQUIREMENTS[0], "**Milestone:** v2.6 Future\n");
      archived.set(V25_ROADMAPS[0], "## Active Milestone: v2.6 Future\n");
    }
    const before = new Map(archived);
    // Act
    const sources = liveProvenanceFixture(archived);
    // Assert
    expect(resolveV25Provenance(sources, [])).toMatchObject({ closure: "gap_pending", publication: "gaps_found" });
    expect(sources.has(V25_REQUIREMENTS[1])).toBe(false);
    expect(sources.get(V25_REQUIREMENTS[0])).toContain("**Milestone:** v2.5");
    expect(sources.get(V25_ROADMAPS[0])).toContain("### Phase 163:");
    expect(archived).toEqual(before);
  });
}

test("supplied Pending fixture cannot rescue missing v2.5 controls from the checkout", () => {
  // Arrange
  const sources = archivedProvenanceFixture();
  sources.delete(V25_REQUIREMENTS[1]);
  // Act / Assert
  expect(() => liveProvenanceFixture(sources)).toThrow();
});

test("historical complete proof and honest Phase163 Pending need no future report", () => {
  // Arrange
  const failures: string[] = [];
  // Act
  const state = resolveV25Provenance(liveProvenanceFixture(), failures);
  // Assert
  expect(failures).toEqual([]);
  expect(state).toMatchObject({ historical: "complete", closure: "gap_pending", publication: "gaps_found" });
});

test("the current provenance page is optional claim input rather than a closure digest input", () => {
  expect(PROVENANCE_FILES).toContain(PHASE163_DOC);
  expect(PROVENANCE_DIGEST_FILES).not.toContain(PHASE163_DOC);
  const sources = liveProvenanceFixture();
  sources.delete(PHASE163_DOC);
  expect(resolveV25Provenance(sources, [])?.closure).toBe("gap_pending");
});

function original(state: "pending" | "implementation" | "complete"): Map<string, string> {
  const sources = liveProvenanceFixture();
  sources.delete(PHASE163_DOC);
  const status = state === "complete" ? "Complete" : "Pending";
  sources.set(V25_REQUIREMENTS[0], sources.get(V25_REQUIREMENTS[0])!
    .replace(/^- \[[ x]\] \*\*(CFPR-02|CFGR-01|CFGR-02)\*\*:/gm, `- [${state === "complete" ? "x" : " "}] **$1**:`)
    .replace(/(\|\s*(?:CFPR-02|CFGR-01|CFGR-02)\s*\|)\s*Phase 16[23]\s*\|\s*(?:Pending|Complete)\s*\|/g, `$1 Phase 162 | ${status} |`));
  sources.set(V25_ROADMAPS[0], sources.get(V25_ROADMAPS[0])!.replace("**Requirements**: CFPR-02, CFGR-02", "**Requirements**: CFPR-02, CFGR-01, CFGR-02").replace("**Requirements**: CFGR-01", "**Requirements**: none"));
  if (state === "pending") sources.delete("docs/parity/v2-5-integrated-filter-proof.md");
  return sources;
}

for (const state of ["pending", "implementation", "complete"] as const) {
  test(`explicit original ${state} retains its historical state`, () => {
    const failures: string[] = [];
    expect(resolveV25Provenance(original(state), failures)).toMatchObject({ historical: state, closure: "original" });
    expect(failures).toEqual([]);
  });
}

export function completedProvenanceFixture(final = false): Map<string, string> {
  const sources = liveProvenanceFixture();
  sources.set(PHASE163_DOC, final ? `${CLOSURE_CLAIM}\n${READINESS_CLAIM}` : CLOSURE_CLAIM);
  sources.set(V25_REQUIREMENTS[0], sources.get(V25_REQUIREMENTS[0])!.replace("[ ] **CFGR-01**", "[x] **CFGR-01**")
    .replace(/(\|\s*CFGR-01\s*\|\s*Phase 163\s*\|)\s*Pending\s*\|/, "$1 Complete |"));
  const digest = provenanceDigest(sources);
  const fields = `lifecycle_mode: yolo\nphase_lifecycle_id: 163-2026-10-12T21-09-41\ngenerated_at: 2026-10-12T23:00:00Z\nsource_digest: ${digest}\n`;
  const report = (name: string, status: string, generator: string, extra: string) => sources.set(PHASE163 + `163-${name}.md`, `---\n${fields}status: ${status}\ngenerated_by: ${generator}\n${extra}---\n`);
  report("04-SUMMARY", final ? "complete" : "in_progress", "gsd-execute-plan", `requirements-completed: [CFGR-01]\nexecution_status: ${final ? "complete" : "in_progress"}\n`);
  report("VERIFICATION", "passed", "gsd-verifier", `lifecycle_validated: true\nrequirements: [CFGR-01]\nsuccess_criteria: [SC1, SC2, SC3, SC4${final ? ", SC5" : ""}]\nreadiness_gate: enforced\nverification_scope: ${final ? "phase" : "functional-closure"}\naudit_status: ${final ? "passed" : "pending"}\nreadiness: ${final ? "established" : "not_asserted"}\nplan04_status: ${final ? "complete" : "in_progress"}\nphase_status: ${final ? "complete" : "in_progress"}\n`);
  report("NATIVE-EVIDENCE", "passed", "gsd-execute-phase", "exit_code: 0\n");
  sources.set(PHASE163 + "163-NATIVE-EVIDENCE.md", sources.get(PHASE163 + "163-NATIVE-EVIDENCE.md")! + "bash scripts/verify.sh\n");
  report("REVIEW", "clean", "gsd-code-reviewer", "");
  report("SECURITY", "verified", "gsd-security-auditor", "threats_open: 0\n");
  if (final) sources.set(V25_AUDITS[0], `---\nmilestone: v2.5\ngenerated_by: gsd-audit-milestone\nstatus: passed\naudited: 2026-10-12T23:01:00Z\nphase_lifecycle_id: 163-2026-10-12T21-09-41\nsource_digest: ${digest}\nscores:\n  requirements: 22/22\ngaps:\n  requirements: []\n  integration: []\n  flows: []\n---\n`);
  return sources;
}

test("canonical functional closure earns the requirement while audit/readiness/phase stay pending", () => {
  const failures: string[] = [];
  const state = resolveV25Provenance(completedProvenanceFixture(), failures);
  expect(failures).toEqual([]);
  expect(state).toMatchObject({ historical: "complete", closure: "gap_complete", publication: "gaps_found" });
  expect(provenanceClaimView(CLOSURE_CLAIM, state)).toBe("");
  expect(provenanceClaimView(READINESS_CLAIM, state)).toBe(READINESS_CLAIM);
});

test("renewed final phase audit earns readiness without shipment", () => {
  const failures: string[] = [];
  const state = resolveV25Provenance(completedProvenanceFixture(true), failures);
  expect(failures).toEqual([]);
  expect(state?.publication).toBe("audit_passed");
  expect(provenanceClaimView(READINESS_CLAIM, state)).toBe("");
  expect(provenanceClaimView(ARCHIVED_CLAIM, state)).toBe(ARCHIVED_CLAIM);
});

test("final refreshed phase report cannot authorize a pre-lifecycle audit", () => {
  const sources = completedProvenanceFixture(true);
  sources.set(V25_AUDITS[0], sources.get(V25_AUDITS[0])!.replace("2026-10-12T23:01:00Z", "2026-10-11T23:01:00Z"));
  expect(resolveV25Provenance(sources, [])).toBeUndefined();
});

test("equivalent active/archive candidates ignore the standard archive envelope", () => {
  const sources = completedProvenanceFixture(true);
  for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS]) sources.set(paths[1], "# Milestone v2.5: Prune-Aware Compact-Filter Serving\n\n**Status:** SHIPPED\n\n" + sources.get(paths[0])!);
  sources.set(V25_AUDITS[1], sources.get(V25_AUDITS[0])!);
  const failures: string[] = [];
  expect(resolveV25Provenance(sources, failures)?.publication).toBe("audit_passed");
  expect(failures).toEqual([]);
  for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS]) sources.delete(paths[0]);
  const archived = resolveV25Provenance(sources, []);
  expect(archived?.publication).toBe("archived");
  expect(provenanceClaimView(CLOSURE_CLAIM, archived)).toBe(CLOSURE_CLAIM);
  expect(provenanceClaimView(READINESS_CLAIM, archived)).toBe(READINESS_CLAIM);
  expect(provenanceClaimView(ARCHIVED_CLAIM, archived)).toBe("");
  sources.set(V25_REQUIREMENTS[0], "**Milestone:** v2.6 Future\n");
  sources.set(V25_ROADMAPS[0], "## Active Milestone: v2.6 Future\n");
  expect(resolveV25Provenance(sources, [])?.publication).toBe("archived");
});

for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS]) {
  for (const conflict of [false, true]) {
    test(`parser-recognized active identity ${conflict ? "refuses conflict" : "accepts equivalent overlap"}: ${paths[0]}`, () => {
      // Arrange
      const sources = completedProvenanceFixture(true);
      const original = sources.get(paths[0])!;
      sources.set(paths[1], original);
      let active = paths === V25_REQUIREMENTS
        ? original.replace("**Milestone:** v2.5", "**Milestone:**  v2.5").replace("## v2.5 Requirements", "## Requirements")
        : original.replace(/^## Active Milestone: v2\.5.*$/m, "## Active Milestone: v2.5");
      if (conflict) active = paths === V25_REQUIREMENTS
        ? active.replace("A client can obtain", "A conflicting client can obtain")
        : active.replace("**Requirements**: CFPR-02, CFGR-02", "**Requirements**: CFPR-02, CFGR-01, CFGR-02");
      sources.set(paths[0], active);
      const failures: string[] = [];
      // Act
      const maybeState = resolveV25Provenance(sources, failures);
      // Assert
      if (conflict) {
        expect(maybeState).toBeUndefined();
        expect(failures.some(failure => failure.includes("conflicting v2.5 candidates"))).toBe(true);
        return;
      }
      expect(maybeState?.publication).toBe("audit_passed");
      expect(failures).toEqual([]);
    });
  }
}

function archivedProvenanceFixture(): Map<string, string> {
  const sources = completedProvenanceFixture(true);
  for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS]) {
    sources.set(paths[1], sources.get(paths[0])!);
    sources.delete(paths[0]);
  }
  sources.set(PHASE163_DOC, ARCHIVED_CLAIM);
  return sources;
}

for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS]) {
  const marker = paths === V25_REQUIREMENTS ? "**Milestone:**" : "## Active Milestone:";
  const heading = paths === V25_REQUIREMENTS ? "# Requirements" : "# Roadmap";
  const fallback = paths === V25_REQUIREMENTS ? "## v2.5 Requirements" : "# Milestone v2.5: Prior";
  for (const [name, source] of [
    ["unknown", `${heading}\n\nUnclassifiable active control.\n`],
    ["empty", ""],
    ["newline value", `${marker}\n\nv2.6 Future\n`],
    ["blank marker before foreign marker", `${marker}\n${marker} v2.6 Future\n`],
    ["duplicate foreign identity", `${marker} v2.6 Future\n${marker} v2.6 Future\n`],
    ["contradictory fallback", `${marker} v2.6 Future\n${fallback}\n`],
    ["earlier milestone", `${marker} v2.4 Prior\n`],
    ["noncanonical version", `${marker} v02.6 Future\n`],
    ["unsafe version", `${marker} v9007199254740992.0 Future\n`],
    ["malformed marker", paths === V25_REQUIREMENTS ? "**Milestone**: v2.6 Future\n" : "## Active Milestone v2.6 Future\n"],
  ]) {
    test(`present active ${name} refuses despite valid archive: ${paths[0]}`, () => {
      const sources = archivedProvenanceFixture();
      sources.set(paths[0], source);
      const failures: string[] = [];
      expect(resolveV25Provenance(sources, failures)).toBeUndefined();
      expect(failures.some(failure => failure.includes(`${paths[0]}: invalid active milestone identity`))).toBe(true);
    });
  }
  for (const version of ["v2.6", "v3.0"]) {
    test(`uniquely identified later ${version} routes to fixed archive: ${paths[0]}`, () => {
      const sources = archivedProvenanceFixture();
      sources.set(paths[0], `${heading}\n\n${marker}\t${version} Future\n`);
      const failures: string[] = [];
      expect(resolveV25Provenance(sources, failures)?.publication).toBe("archived");
      expect(failures).toEqual([]);
    });
  }
}

const ARCHIVE_INDEX = "# Roadmap\n\nv2.5 controls: .planning/milestones/v2.5-ROADMAP.md\n";
test("the exact whole archive index routes only to an existing fixed archive", () => {
  const sources = archivedProvenanceFixture();
  sources.set(V25_ROADMAPS[0], ARCHIVE_INDEX);
  expect(resolveV25Provenance(sources, [])?.roadmapPath).toBe(V25_ROADMAPS[1]);
  sources.delete(V25_ROADMAPS[1]);
  const failures: string[] = [];
  expect(resolveV25Provenance(sources, failures)).toBeUndefined();
  expect(failures.join("\n")).toContain("missing v2.5 source");
});
for (const [name, source] of [
  ["wrong pointer", ARCHIVE_INDEX.replace("v2.5-ROADMAP.md", "v2.6-ROADMAP.md")],
  ["missing terminal newline", ARCHIVE_INDEX.slice(0, -1)],
  ["leading comment", "<!-- extra -->\n" + ARCHIVE_INDEX],
  ["extra control", ARCHIVE_INDEX + "**Requirements**: CFGR-01\n"],
  ["mixed milestones", ARCHIVE_INDEX + "## Active Milestone: v2.5 Prior\n## Active Milestone: v2.6 Future\n"],
  ["near-match heading", ARCHIVE_INDEX.replace("# Roadmap", "# Roadmaps")],
]) {
  test(`archive index ${name} refuses`, () => {
    const sources = archivedProvenanceFixture();
    sources.set(V25_ROADMAPS[0], source);
    const failures: string[] = [];
    expect(resolveV25Provenance(sources, failures)).toBeUndefined();
    expect(failures.some(failure => failure.includes(`${V25_ROADMAPS[0]}: invalid active milestone identity`))).toBe(true);
  });
}
test("an explicit later milestone may retain a historical archive link", () => {
  const sources = archivedProvenanceFixture();
  sources.set(V25_ROADMAPS[0], ARCHIVE_INDEX + "## Active Milestone: v2.6 Future\n");
  expect(resolveV25Provenance(sources, [])?.publication).toBe("archived");
});

test("renewed audit earns readiness before the final phase refresh without claiming phase completion", () => {
  const functional = completedProvenanceFixture();
  functional.set(V25_AUDITS[0], completedProvenanceFixture(true).get(V25_AUDITS[0])!);
  const failures: string[] = [];
  const state = resolveV25Provenance(functional, failures);
  expect(failures).toEqual([]);
  expect(state?.publication).toBe("audit_passed");
  expect(provenanceClaimView(READINESS_CLAIM, state)).toBe("");
  expect(provenanceMetadata(functional.get(PHASE163 + "163-VERIFICATION.md")!).phase_status).toBe("in_progress");
});

for (const [name, file, before, after] of [
  ["wrong milestone", V25_REQUIREMENTS[0], "**Milestone:** v2.5", "**Milestone:** v2.6"],
  ["wrong baseline", V25_REQUIREMENTS[0], "29.3.knots20260210", "29.3.knots20260101"],
  ["wrong owner", V25_REQUIREMENTS[0], "Phase 163 | Pending", "Phase 164 | Pending"],
  ["checkbox alone", V25_REQUIREMENTS[0], "[ ] **CFGR-01**", "[x] **CFGR-01**"],
  ["wrong gap dependency", PHASE163 + "163-GAP-SCOPE.md", "depends_on: [162]", "depends_on: [161]"],
  ["wrong creation lifecycle", PHASE163 + "163-GAP-SCOPE.md", "163-2026-10-12T21-03-37", "163-2026-10-12T21-09-41"],
  ["wrong context lifecycle", PHASE163 + "163-CONTEXT.md", "163-2026-10-12T21-09-41", "163-stale"],
  ["stale snapshot", ORIGINAL_AUDITS[0], "status: gaps_found", "status: passed"],
  ["duplicate metadata", PHASE163 + "163-CONTEXT.md", "status: approved-scope", "status: approved-scope\nstatus: approved-scope"],
  ["quoted duplicate metadata", PHASE163 + "163-CONTEXT.md", "status: approved-scope", 'status: denied\n"status": approved-scope'],
  ["spaced duplicate metadata", PHASE163 + "163-CONTEXT.md", "status: approved-scope", "status: denied\nstatus : approved-scope"],
  ["tabbed duplicate metadata", PHASE163 + "163-CONTEXT.md", "status: approved-scope", "status: denied\nstatus\t: approved-scope"],
  ["tagged duplicate metadata", PHASE163 + "163-CONTEXT.md", "status: approved-scope", "status: denied\n!!str status: approved-scope"],
  ["sequence-key duplicate metadata", PHASE163 + "163-CONTEXT.md", "status: approved-scope", "status: denied\n[status]: approved-scope"],
  ["altered live gap audit", V25_AUDITS[0], "id: CFGR-01", "id: CFGR-02"],
  ["duplicate milestone identity", V25_REQUIREMENTS[0], "## v2.5 Requirements", "**Milestone:** v2.6 Future\n\n## v2.5 Requirements"],
  ["duplicate scoped roadmap owner", V25_ROADMAPS[0], "**Requirements**: CFPR-02, CFGR-02", "**Requirements**: CFPR-02, CFGR-01, CFGR-02"],
] as const) {
  test(`refuses ${name}`, () => {
    const sources = liveProvenanceFixture();
    expect(sources.get(file)).toContain(before);
    sources.set(file, sources.get(file)!.replace(before, after));
    const failures: string[] = [];
    expect(resolveV25Provenance(sources, failures)).toBeUndefined();
    expect(failures.length).toBe(1);
  });
}

for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS]) {
  test(`conflicting parsed candidates refuse ${paths[0]}`, () => {
    const sources = completedProvenanceFixture(true);
    sources.set(paths[1], sources.get(paths[0])!.replace(paths === V25_REQUIREMENTS ? "A client can obtain" : paths === V25_ROADMAPS ? "Phase 162\n" : "22/22", "conflict"));
    expect(resolveV25Provenance(sources, [])).toBeUndefined();
  });
}

for (const file of [V25_REQUIREMENTS[0], V25_ROADMAPS[0], V25_AUDITS[0], ...ORIGINAL_AUDITS, PHASE163 + "163-GAP-SCOPE.md", PHASE163 + "163-CONTEXT.md"]) {
  test(`missing supplied key refuses ${file}`, () => {
    const sources = liveProvenanceFixture();
    sources.delete(file);
    expect(resolveV25Provenance(sources, [])).toBeUndefined();
  });
}

for (const mutation of ["wrong generator", "wrong scope", "old owner", "missing activation", "internal substitute", "security open", "native failed", "premature readiness", "premature phase", "premature summary stage"]) {
  test(`completion refuses ${mutation}`, () => {
    const sources = completedProvenanceFixture();
    const verification = PHASE163 + "163-VERIFICATION.md";
    if (mutation === "wrong generator") sources.set(verification, sources.get(verification)!.replace("gsd-verifier", "gsd-executor"));
    if (mutation === "wrong scope") sources.set(verification, sources.get(verification)!.replace("functional-closure", "internal"));
    if (mutation === "old owner") sources.set(verification, sources.get(".planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-VERIFICATION.md")!);
    if (mutation === "missing activation") sources.delete(PHASE163 + "163-04-SUMMARY.md");
    if (mutation === "internal substitute") { sources.set(PHASE163 + "163-CLOSURE-VERIFICATION.md", sources.get(verification)!); sources.delete(verification); }
    if (mutation === "security open") sources.set(PHASE163 + "163-SECURITY.md", sources.get(PHASE163 + "163-SECURITY.md")!.replace("threats_open: 0", "threats_open: 1"));
    if (mutation === "native failed") sources.set(PHASE163 + "163-NATIVE-EVIDENCE.md", sources.get(PHASE163 + "163-NATIVE-EVIDENCE.md")!.replace("exit_code: 0", "exit_code: 1"));
    if (mutation === "premature readiness") sources.set(verification, sources.get(verification)!.replace("readiness: not_asserted", "readiness: established"));
    if (mutation === "premature phase") sources.set(verification, sources.get(verification)!.replace("phase_status: in_progress", "phase_status: complete"));
    if (mutation === "premature summary stage") sources.set(PHASE163 + "163-04-SUMMARY.md", sources.get(PHASE163 + "163-04-SUMMARY.md")!.replace("execution_status: in_progress", "execution_status: complete"));
    expect(resolveV25Provenance(sources, [])).toBeUndefined();
  });
}

test("first frontmatter wins and nested duplicate metadata refuses", () => {
  expect(provenanceMetadata("---\nstatus: passed\n---\nbody\n---\nstatus: failed\n---\n").status).toBe("passed");
  expect(() => provenanceMetadata("---\nscores:\n  requirements: 22/22\n  requirements: 0/22\n---\n")).toThrow("duplicate metadata");
});

test("current audit rejects duplicate mapping keys nested in a flow array", () => {
  const sources = liveProvenanceFixture();
  const file = V25_AUDITS[0];
  const audit = sources.get(file)!;
  const original = provenanceMetadata(audit);
  const requirement = (original.gaps as { requirements: unknown[] }).requirements[0];
  const flow = JSON.stringify(requirement).replace('"id":"CFGR-01"', '"id":"CFGR-02","id":"CFGR-01"');
  sources.set(file, audit.replace(/  requirements:\n[\s\S]*?(?=  integration:)/, `  requirements: [${flow}]\n`));
  expect(resolveV25Provenance(sources, [])).toBeUndefined();
});

test("ordinary scalar arrays and quoted list scalar braces remain valid metadata", () => {
  expect(provenanceMetadata('---\nrequirements: [CFGR-01]\nnotes: ["{quoted}"]\nitems:\n  - "{quoted}: scalar"\n---\n').requirements).toEqual(["CFGR-01"]);
});

test("only earned exact current sentences are removed", () => {
  const pending = resolveV25Provenance(liveProvenanceFixture(), []);
  expect(provenanceClaimView(GAP_CLAIM, pending)).toBe("");
  for (const claim of [CLOSURE_CLAIM, READINESS_CLAIM, ARCHIVED_CLAIM]) expect(provenanceClaimView(claim, pending)).toBe(claim);
  expect(provenanceClaimView(GAP_CLAIM, undefined)).toBe(GAP_CLAIM);
  expect(provenanceClaimView("v2.5 is shipped.", pending)).toBe("v2.5 is shipped.");
});

test("Pending fixtures remain explicit after actual completed publication exists", () => {
  const sources = liveProvenanceFixture(completedProvenanceFixture(true));
  expect(sources.has(PHASE163 + "163-VERIFICATION.md")).toBe(false);
  expect(resolveV25Provenance(sources, [])).toMatchObject({ closure: "gap_pending", publication: "gaps_found" });
});

for (const replacement of ["generated_at: invalid", "", "generated_at: 2026-10-11T21:09:41Z"]) {
  test(`context timestamp refuses ${replacement || "missing"} even with rebound digest`, () => {
    const sources = completedProvenanceFixture();
    const context = PHASE163 + "163-CONTEXT.md";
    sources.set(context, sources.get(context)!.replace(/^generated_at:.*$/m, replacement));
    const digest = provenanceDigest(sources);
    for (const [file, source] of sources) if (file.startsWith(PHASE163) && file !== context) sources.set(file, source.replace(/^source_digest:.*$/m, `source_digest: ${digest}`));
    expect(resolveV25Provenance(sources, [])).toBeUndefined();
  });
}

for (const file of [
  ...["159-filter-rpcs", "160-peer-filter-serving", "161-operator-index-evidence"].flatMap(name =>
    [`scripts/check-phase${name}.ts`, `scripts/check-phase${name}.test.ts`]),
  "scripts/check-phase163-provenance/native.test.ts", "scripts/check-phase163-provenance/fixtures.ts",
  "scripts/check-phase162-integrated-filter-proof/contracts.ts", "scripts/verify.sh",
]) {
  for (const mutation of ["missing", "empty", "tampered"]) {
    test(`closure source binding refuses ${mutation} ${file}`, () => {
      // Arrange: receipts describe the exact unmodified implementation inputs.
      const sources = completedProvenanceFixture();
      const source = readFileSync(resolve(root, file), "utf8");
      sources.set(file, source);
      if (mutation === "missing") sources.delete(file);
      if (mutation === "empty") sources.set(file, "");
      if (mutation === "tampered") sources.set(file, source + "\n# source-binding mutation\n");
      const failures: string[] = [];
      // Act
      const maybeState = resolveV25Provenance(sources, failures);
      // Assert
      expect(maybeState).toBeUndefined();
      expect(failures.some(failure => failure.includes("gate/lifecycle/digest"))).toBe(true);
    });
  }
}
