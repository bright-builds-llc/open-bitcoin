import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

const PHASE125_DIRECTORY =
  ".planning/phases/125-compact-download-verification-traceability-closure";
const PHASE126_DIRECTORY =
  ".planning/phases/126-compact-relay-residual-hardening";
const PHASE125_CONTEXT = `${PHASE125_DIRECTORY}/125-CONTEXT.md`;
const PHASE125_VERIFICATION = `${PHASE125_DIRECTORY}/125-VERIFICATION.md`;
const PHASE125_NAME = "Compact Download Verification Traceability Closure";
const PHASE126_NAME = "Compact Relay Residual Hardening";
const PHASE125_REQUIREMENTS = ["RCN-04", "RCN-05", "RCN-06"] as const;
const PHASE126_REQUIREMENTS = [
  "CMP-05",
  "RCN-02",
  "RCN-03",
  "GOV-04",
  "BOUND-01",
  "HARD-05",
] as const;
const GAP_REQUIREMENT_PHASES = new Map([
  ...PHASE125_REQUIREMENTS.map((id) => [id, 125] as const),
  ...PHASE126_REQUIREMENTS.map((id) => [id, 126] as const),
]);
const EXPECTED_PLAN_NUMBERS = ["01", "02", "03", "04"] as const;
const PHASE125_ROUTE = "/gsd-execute-phase 125";
const PHASE126_ROUTE = "/gsd-execute-phase 126";
const ARCHIVE_ROUTE = "/gsd-complete-milestone v2.1";
const ROUTING_FILES = [
  ".planning/ROADMAP.md",
  ".planning/PROJECT.md",
  ".planning/STATE.md",
  ".planning/v2.1-MILESTONE-AUDIT.md",
] as const;

export type Phase125LifecycleStage =
  | {
      kind: "planned";
      planCount: 4;
      summaryCount: 0;
      verificationPresent: false;
    }
  | {
      kind: "pre_verification";
      planCount: 4;
      summaryCount: 1 | 2 | 3;
      verificationPresent: false;
    }
  | {
      kind: "verification_written_pre_promotion";
      planCount: 4;
      summaryCount: 3;
      verificationPresent: true;
    }
  | {
      kind: "post_verification";
      planCount: 4;
      summaryCount: 3;
      verificationPresent: true;
    }
  | {
      kind: "post_summary";
      planCount: 4;
      summaryCount: 4;
      verificationPresent: true;
    };

type RequirementEntry = { checked: boolean; id: string };
type TraceabilityEntry = { id: string; phase: number; status: string };
type LifecycleIdentity = { mode: string; phaseLifecycleId: string };
type Phase125Artifacts = {
  planCount: number;
  summaryCount: number;
  verificationPresent: boolean;
};
type ProjectionState = "pending" | "promoted";

export function isPhase124GapClosureStage(roadmap: string, audit: string): boolean {
  return (
    /^status:\s*gaps_found\s*$/m.test(audit) ||
    roadmap.includes("#### Phase 125:") ||
    roadmap.includes("#### Phase 126:")
  );
}

export function verifyPhase124GapClosureStage(
  repoRoot: string,
  requirements: string,
  roadmap: string,
  audit: string,
  failures: string[],
): void {
  const entries = parseRequirementEntries(requirements);
  const traceability = parseTraceabilityEntries(requirements);
  const maybeStage = maybeParsePhase125LifecycleStage(
    repoRoot,
    entries,
    traceability,
    failures,
  );

  verifyRequirementOwnership(entries, traceability, failures);
  verifyPhaseDirectories(repoRoot, failures);
  if (maybeStage === null) {
    return;
  }

  verifyProjection(maybeStage, requirements, roadmap, audit, entries, traceability, failures);
  verifyRouting(repoRoot, maybeStage, roadmap, audit, failures);
}

function maybeParsePhase125LifecycleStage(
  repoRoot: string,
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): Phase125LifecycleStage | null {
  const maybeProjection = maybeParseProjectionState(entries, traceability, failures);
  const maybeArtifacts = maybeParsePhase125Artifacts(repoRoot, failures);
  if (maybeProjection === null || maybeArtifacts === null) {
    return null;
  }

  const { planCount, summaryCount, verificationPresent } = maybeArtifacts;
  if (planCount !== 4) {
    failures.push(
      `P124 Phase 125 lifecycle requires exactly four plans; found ${planCount}`,
    );
    return null;
  }

  if (maybeProjection === "pending" && !verificationPresent && summaryCount === 0) {
    return { kind: "planned", planCount: 4, summaryCount: 0, verificationPresent: false };
  }
  if (
    maybeProjection === "pending" &&
    !verificationPresent &&
    isPreVerificationSummaryCount(summaryCount)
  ) {
    return {
      kind: "pre_verification",
      planCount: 4,
      summaryCount,
      verificationPresent: false,
    };
  }
  if (maybeProjection === "pending" && verificationPresent && summaryCount === 3) {
    return {
      kind: "verification_written_pre_promotion",
      planCount: 4,
      summaryCount: 3,
      verificationPresent: true,
    };
  }
  if (maybeProjection === "promoted" && verificationPresent && summaryCount === 3) {
    return {
      kind: "post_verification",
      planCount: 4,
      summaryCount: 3,
      verificationPresent: true,
    };
  }
  if (maybeProjection === "promoted" && verificationPresent && summaryCount === 4) {
    return {
      kind: "post_summary",
      planCount: 4,
      summaryCount: 4,
      verificationPresent: true,
    };
  }

  if (maybeProjection === "promoted" && !verificationPresent) {
    failures.push("P124 promoted projection requires lifecycle-valid verification");
  }
  failures.push(
    `P124 Phase 125 artifact combination does not match a legal lifecycle stage: ${planCount} plans, ${summaryCount} summaries, verification ${verificationPresent ? "present" : "absent"}, projection ${maybeProjection}`,
  );
  return null;
}

function maybeParseProjectionState(
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): ProjectionState | null {
  const checklistStates = PHASE125_REQUIREMENTS.map((id) =>
    entries.find((entry) => entry.id === id)?.checked,
  );
  const traceabilityStates = PHASE125_REQUIREMENTS.map(
    (id) => traceability.find((entry) => entry.id === id)?.status,
  );
  const pending =
    checklistStates.every((checked) => checked === false) &&
    traceabilityStates.every((status) => status === "Pending");
  const promoted =
    checklistStates.every((checked) => checked === true) &&
    traceabilityStates.every((status) => status === "Complete");

  if (pending) return "pending";
  if (promoted) return "promoted";
  failures.push(
    "P124 Phase 125 requirement projection must be uniformly pending or promoted",
  );
  return null;
}

function maybeParsePhase125Artifacts(
  repoRoot: string,
  failures: string[],
): Phase125Artifacts | null {
  const absoluteDirectory = path.join(repoRoot, PHASE125_DIRECTORY);
  if (!existsSync(absoluteDirectory)) {
    failures.push(`P124 gap-closure missing phase directory ${PHASE125_DIRECTORY}`);
    return null;
  }

  const names = readdirSync(absoluteDirectory).sort();
  const planNames = names.filter((name) => /^125-\d{2}-PLAN\.md$/.test(name));
  const summaryNames = names.filter((name) => /^125-\d{2}-SUMMARY\.md$/.test(name));
  for (const name of names) {
    if (
      /^125-.*-(?:PLAN|SUMMARY)\.md$/.test(name) &&
      !/^125-\d{2}-(?:PLAN|SUMMARY)\.md$/.test(name)
    ) {
      failures.push(`P124 Phase 125 has malformed lifecycle artifact ${name}`);
    }
  }

  const maybeContextIdentity = maybeReadLifecycleIdentity(
    repoRoot,
    PHASE125_CONTEXT,
    "gsd-discuss-phase",
    failures,
  );
  if (maybeContextIdentity === null) {
    return null;
  }

  verifyExactPlanSet(planNames, failures);
  for (const planName of planNames) {
    verifyNumberedArtifact(
      repoRoot,
      path.join(PHASE125_DIRECTORY, planName),
      planName,
      "PLAN",
      "gsd-plan-phase",
      maybeContextIdentity,
      failures,
    );
  }
  for (const summaryName of summaryNames) {
    verifyNumberedArtifact(
      repoRoot,
      path.join(PHASE125_DIRECTORY, summaryName),
      summaryName,
      "SUMMARY",
      "gsd-execute-plan",
      maybeContextIdentity,
      failures,
    );
  }

  const verificationPresent = names.includes("125-VERIFICATION.md");
  if (verificationPresent) {
    verifyVerificationArtifact(repoRoot, maybeContextIdentity, failures);
  }

  return {
    planCount: planNames.length,
    summaryCount: summaryNames.length,
    verificationPresent,
  };
}

function maybeReadLifecycleIdentity(
  repoRoot: string,
  relativePath: string,
  expectedGenerator: string,
  failures: string[],
): LifecycleIdentity | null {
  const maybeFrontmatter = maybeReadFrontmatter(repoRoot, relativePath, failures);
  if (maybeFrontmatter === null) {
    return null;
  }
  requireScalar(maybeFrontmatter, "generated_by", expectedGenerator, relativePath, failures);
  const maybeMode = maybeExactScalar(maybeFrontmatter, "lifecycle_mode", relativePath, failures);
  const maybePhaseLifecycleId = maybeExactScalar(
    maybeFrontmatter,
    "phase_lifecycle_id",
    relativePath,
    failures,
  );
  if (maybeMode === null || maybePhaseLifecycleId === null) {
    return null;
  }
  return { mode: maybeMode, phaseLifecycleId: maybePhaseLifecycleId };
}

function verifyExactPlanSet(planNames: string[], failures: string[]): void {
  const expected = EXPECTED_PLAN_NUMBERS.map((number) => `125-${number}-PLAN.md`);
  for (const name of expected) {
    requireExactNumber(
      planNames.filter((candidate) => candidate === name).length,
      1,
      `P124 Phase 125 plan artifact ${name}`,
      failures,
    );
  }
  for (const name of planNames) {
    if (!expected.includes(name)) {
      failures.push(`P124 Phase 125 plan number is outside 01 through 04: ${name}`);
    }
  }
}

function verifyNumberedArtifact(
  repoRoot: string,
  relativePath: string,
  name: string,
  kind: "PLAN" | "SUMMARY",
  expectedGenerator: string,
  expectedLifecycle: LifecycleIdentity,
  failures: string[],
): void {
  const maybeMatch = name.match(/^125-(\d{2})-(?:PLAN|SUMMARY)\.md$/);
  const planNumber = maybeMatch?.[1] ?? "";
  if (!EXPECTED_PLAN_NUMBERS.includes(planNumber as (typeof EXPECTED_PLAN_NUMBERS)[number])) {
    failures.push(`P124 Phase 125 ${kind.toLowerCase()} number is outside 01 through 04: ${name}`);
  }
  const maybeFrontmatter = maybeReadFrontmatter(repoRoot, relativePath, failures);
  if (maybeFrontmatter === null) {
    return;
  }
  requireScalar(
    maybeFrontmatter,
    "phase",
    "125-compact-download-verification-traceability-closure",
    relativePath,
    failures,
  );
  const maybeArtifactPlan = maybeExactScalar(
    maybeFrontmatter,
    "plan",
    relativePath,
    failures,
  );
  if (maybeArtifactPlan !== null && maybeArtifactPlan !== planNumber) {
    failures.push(`${relativePath} plan number must match its filename`);
  }
  verifyLifecycleMatches(
    maybeFrontmatter,
    relativePath,
    expectedGenerator,
    expectedLifecycle,
    failures,
  );
}

function verifyVerificationArtifact(
  repoRoot: string,
  expectedLifecycle: LifecycleIdentity,
  failures: string[],
): void {
  const maybeFrontmatter = maybeReadFrontmatter(repoRoot, PHASE125_VERIFICATION, failures);
  if (maybeFrontmatter === null) {
    return;
  }
  requireScalar(
    maybeFrontmatter,
    "phase",
    "125-compact-download-verification-traceability-closure",
    PHASE125_VERIFICATION,
    failures,
  );
  requireScalar(maybeFrontmatter, "status", "passed", PHASE125_VERIFICATION, failures);
  requireScalar(
    maybeFrontmatter,
    "lifecycle_validated",
    "true",
    PHASE125_VERIFICATION,
    failures,
  );
  verifyLifecycleMatches(
    maybeFrontmatter,
    PHASE125_VERIFICATION,
    "gsd-verifier",
    expectedLifecycle,
    failures,
  );
}

function verifyLifecycleMatches(
  frontmatter: string,
  relativePath: string,
  expectedGenerator: string,
  expectedLifecycle: LifecycleIdentity,
  failures: string[],
): void {
  requireScalar(frontmatter, "generated_by", expectedGenerator, relativePath, failures);
  const maybeMode = maybeExactScalar(frontmatter, "lifecycle_mode", relativePath, failures);
  const maybePhaseLifecycleId = maybeExactScalar(
    frontmatter,
    "phase_lifecycle_id",
    relativePath,
    failures,
  );
  if (maybeMode !== null && maybeMode !== expectedLifecycle.mode) {
    failures.push(`${relativePath} lifecycle_mode must match Phase 125 CONTEXT`);
  }
  if (
    maybePhaseLifecycleId !== null &&
    maybePhaseLifecycleId !== expectedLifecycle.phaseLifecycleId
  ) {
    failures.push(`${relativePath} phase_lifecycle_id must match Phase 125 CONTEXT`);
  }
}

function maybeReadFrontmatter(
  repoRoot: string,
  relativePath: string,
  failures: string[],
): string | null {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P124 Phase 125 lifecycle missing ${relativePath}`);
    return null;
  }
  const text = readFileSync(absolutePath, "utf8");
  const delimiterCount = text.split("\n").filter((line) => line.trim() === "---").length;
  const maybeMatch = text.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/);
  if (maybeMatch === null || delimiterCount !== 2) {
    failures.push(`${relativePath} must contain exactly one YAML frontmatter block`);
    return null;
  }
  return maybeMatch[1] ?? "";
}

function verifyRequirementOwnership(
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): void {
  requireExactNumber(entries.length, 39, "P124 gap-closure requirement checklist total", failures);
  requireExactNumber(traceability.length, 39, "P124 gap-closure traceability total", failures);
  requireExactNumber(
    new Set(entries.map((entry) => entry.id)).size,
    39,
    "P124 gap-closure unique checklist total",
    failures,
  );
  requireExactNumber(
    new Set(traceability.map((entry) => entry.id)).size,
    39,
    "P124 gap-closure unique traceability total",
    failures,
  );
  for (const [requirement, expectedPhase] of GAP_REQUIREMENT_PHASES) {
    const owners = traceability.filter((entry) => entry.id === requirement);
    if (owners.length !== 1 || owners[0]?.phase !== expectedPhase) {
      failures.push(`P124 gap-closure ${requirement} must be owned by Phase ${expectedPhase}`);
    }
  }
}

function verifyProjection(
  stage: Phase125LifecycleStage,
  requirements: string,
  roadmap: string,
  audit: string,
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): void {
  const promoted = isPromoted(stage);
  const expectedComplete = promoted ? 33 : 30;
  const expectedPending = 39 - expectedComplete;
  requireExactNumber(
    entries.filter((entry) => entry.checked).length,
    expectedComplete,
    `P124 ${stage.kind} checked requirement count`,
    failures,
  );
  requireExactNumber(
    traceability.filter((entry) => entry.status === "Complete").length,
    expectedComplete,
    `P124 ${stage.kind} complete traceability count`,
    failures,
  );
  requireExactNumber(
    traceability.filter((entry) => entry.status === "Pending").length,
    expectedPending,
    `P124 ${stage.kind} pending traceability count`,
    failures,
  );

  for (const entry of entries) {
    const expectedChecked = expectedRequirementComplete(entry.id, promoted);
    if (entry.checked !== expectedChecked) {
      failures.push(`P124 ${stage.kind} checklist state is invalid for ${entry.id}`);
    }
  }
  for (const entry of traceability) {
    const expectedStatus = expectedRequirementComplete(entry.id, promoted)
      ? "Complete"
      : "Pending";
    if (entry.status !== expectedStatus) {
      failures.push(
        `P124 ${stage.kind} traceability status is invalid for ${entry.id}`,
      );
    }
  }

  verifyCoverageCounts(
    requirements,
    "Complete",
    expectedComplete,
    expectedPending,
    "requirements",
    failures,
  );
  verifyCoverageCounts(
    roadmap,
    "Satisfied",
    expectedComplete,
    expectedPending,
    "roadmap",
    failures,
  );
  verifyRoadmapPhases(stage, roadmap, failures);
  verifyAudit(stage, audit, expectedComplete, failures);
}

function expectedRequirementComplete(id: string, promoted: boolean): boolean {
  if (PHASE126_REQUIREMENTS.includes(id as (typeof PHASE126_REQUIREMENTS)[number])) {
    return false;
  }
  if (PHASE125_REQUIREMENTS.includes(id as (typeof PHASE125_REQUIREMENTS)[number])) {
    return promoted;
  }
  return true;
}

function verifyCoverageCounts(
  text: string,
  completeLabel: "Complete" | "Satisfied",
  completeCount: number,
  pendingCount: number,
  corpusLabel: string,
  failures: string[],
): void {
  for (const line of [
    "v2.1 requirements: 39 total",
    "Mapped to phases: 39",
    `${completeLabel}: ${completeCount}`,
    `Pending hardening and closeout: ${pendingCount}`,
    "Unmapped: 0",
  ]) {
    requireContains(
      text,
      line,
      `P124 Phase 125 ${corpusLabel} coverage counts`,
      failures,
    );
  }
}

function verifyRoadmapPhases(
  stage: Phase125LifecycleStage,
  roadmap: string,
  failures: string[],
): void {
  requireContains(
    roadmap,
    "- [x] **Phase 124:",
    "P124 Phase 125 lifecycle Phase 124 state",
    failures,
  );
  requireContains(
    phaseSection(roadmap, 124),
    "**Plans:** 2/2 plans complete",
    "P124 Phase 125 lifecycle Phase 124 plans",
    failures,
  );

  const phase125 = phaseSection(roadmap, 125);
  const phase126 = phaseSection(roadmap, 126);
  const phase125Checked = stage.kind === "post_summary";
  requireContains(
    roadmap,
    `- [${phase125Checked ? "x" : " "}] **Phase 125: ${PHASE125_NAME}**`,
    `P124 ${stage.kind} Phase 125 state`,
    failures,
  );
  requireContains(
    phase125,
    "**Depends on:** Phase 124",
    `P124 ${stage.kind} Phase 125 dependency`,
    failures,
  );
  requireContains(
    phase125,
    "**Requirements:** RCN-04, RCN-05, RCN-06",
    `P124 ${stage.kind} Phase 125 requirements`,
    failures,
  );
  requireContains(
    phase125,
    expectedPhase125Progress(stage),
    `P124 ${stage.kind} Phase 125 plans`,
    failures,
  );

  requireContains(
    roadmap,
    `- [ ] **Phase 126: ${PHASE126_NAME}**`,
    `P124 ${stage.kind} Phase 126 state`,
    failures,
  );
  requireContains(
    phase126,
    "**Depends on:** Phase 125",
    `P124 ${stage.kind} Phase 126 dependency`,
    failures,
  );
  requireContains(
    phase126,
    "**Requirements:** CMP-05, RCN-02, RCN-03, GOV-04, BOUND-01, HARD-05",
    `P124 ${stage.kind} Phase 126 requirements`,
    failures,
  );
  requireContains(
    phase126,
    "**Plans:** 0 plans",
    `P124 ${stage.kind} Phase 126 plans`,
    failures,
  );
}

function expectedPhase125Progress(stage: Phase125LifecycleStage): string {
  if (stage.kind === "planned") return "**Plans:** 4 plans";
  if (stage.kind === "post_summary") return "**Plans:** 4/4 plans complete";
  return `**Plans:** ${stage.summaryCount}/4 plans executed`;
}

function verifyAudit(
  stage: Phase125LifecycleStage,
  audit: string,
  completeCount: number,
  failures: string[],
): void {
  const relativePath = ".planning/v2.1-MILESTONE-AUDIT.md";
  const maybeFrontmatter = maybeExtractFrontmatter(audit, relativePath, failures);
  if (maybeFrontmatter !== null) {
    requireScalar(maybeFrontmatter, "status", "gaps_found", relativePath, failures);
  }
  requireContains(
    audit,
    `requirements: "${completeCount}/39"`,
    `P124 ${stage.kind} audit requirements`,
    failures,
  );
  requireContains(
    audit,
    `phases: "${isPromoted(stage) ? 16 : 15}/17"`,
    `P124 ${stage.kind} audit phases`,
    failures,
  );
  requireContains(audit, "integration: []", `P124 ${stage.kind} audit`, failures);
  requireContains(audit, "flows: []", `P124 ${stage.kind} audit`, failures);
  for (const requirement of PHASE125_REQUIREMENTS) {
    requireExactNumber(
      countOccurrences(audit, `- id: ${requirement}`),
      isPromoted(stage) ? 0 : 1,
      `P124 ${stage.kind} audit orphan ${requirement}`,
      failures,
    );
  }
}

function verifyRouting(
  repoRoot: string,
  stage: Phase125LifecycleStage,
  roadmap: string,
  audit: string,
  failures: string[],
): void {
  const texts = new Map<string, string>([
    [".planning/ROADMAP.md", roadmap],
    [".planning/v2.1-MILESTONE-AUDIT.md", audit],
  ]);
  for (const relativePath of [".planning/PROJECT.md", ".planning/STATE.md"] as const) {
    const absolutePath = path.join(repoRoot, relativePath);
    if (!existsSync(absolutePath)) {
      failures.push(`P124 Phase 125 routing missing ${relativePath}`);
      texts.set(relativePath, "");
      continue;
    }
    texts.set(relativePath, readFileSync(absolutePath, "utf8"));
  }

  const promoted = isPromoted(stage);
  const expectedRoute = promoted ? PHASE126_ROUTE : PHASE125_ROUTE;
  const forbiddenRoute = promoted ? PHASE125_ROUTE : PHASE126_ROUTE;
  for (const relativePath of ROUTING_FILES) {
    const text = texts.get(relativePath) ?? "";
    requireAbsent(
      text,
      ARCHIVE_ROUTE,
      `P124 ${stage.kind} milestone completion route ${relativePath}`,
      failures,
    );
    requireAbsent(
      text,
      forbiddenRoute,
      promoted
        ? `P124 ${stage.kind} stale Phase 125 route ${relativePath}`
        : `P124 ${stage.kind} premature Phase 126 route ${relativePath}`,
      failures,
    );
    requireContains(
      text,
      expectedRoute,
      promoted
        ? `P124 ${stage.kind} Phase 126 primary route ${relativePath}`
        : `P124 ${stage.kind} Phase 125 primary route ${relativePath}`,
      failures,
    );
  }
  if (stage.kind === "post_summary") {
    verifyPostSummaryNarrative(texts, failures);
  }
}

function verifyPostSummaryNarrative(
  texts: ReadonlyMap<string, string>,
  failures: string[],
): void {
  const staleNarratives = [
    {
      pattern: /\b3\/4\s+plans?\s+(?:complete|executed)\b/i,
      label: "3/4 progress",
    },
    {
      pattern:
        /\b(?:awaits?|awaiting)\b[^\n]{0,80}\bsummary bookkeeping\b|\bsummary bookkeeping\b[^\n]{0,80}\b(?:pending|still pending)\b/i,
      label: "summary bookkeeping pending",
    },
    { pattern: /\bpromoted-pre-summary\b/i, label: "promoted-pre-summary projection" },
    {
      pattern: /\bcurrent focus:\*{0,2}\s*phase 125\b/i,
      label: "Phase 125 current focus",
    },
  ] as const;

  for (const [relativePath, text] of texts) {
    for (const staleNarrative of staleNarratives) {
      if (staleNarrative.pattern.test(text)) {
        failures.push(
          `P124 post_summary contradictory Phase 125 narrative ${relativePath}: ${staleNarrative.label}`,
        );
      }
    }
  }
}

function verifyPhaseDirectories(repoRoot: string, failures: string[]): void {
  for (const directory of [PHASE125_DIRECTORY, PHASE126_DIRECTORY]) {
    if (!existsSync(path.join(repoRoot, directory))) {
      failures.push(`P124 gap-closure missing phase directory ${directory}`);
    }
  }
}

function parseRequirementEntries(text: string): RequirementEntry[] {
  return [...text.matchAll(/^- \[([ x])\] \*\*([A-Z]+-\d+)\*\*:/gm)].map(
    (match) => ({ checked: match[1] === "x", id: match[2] ?? "" }),
  );
}

function parseTraceabilityEntries(text: string): TraceabilityEntry[] {
  return [
    ...text.matchAll(
      /^\|\s*([A-Z]+-\d+)\s*\|\s*Phase\s+(\d+)\s*\|\s*(Complete|Pending)\s*\|$/gm,
    ),
  ].map((match) => ({
    id: match[1] ?? "",
    phase: Number(match[2]),
    status: match[3] ?? "",
  }));
}

function maybeExtractFrontmatter(
  text: string,
  relativePath: string,
  failures: string[],
): string | null {
  const delimiterCount = text.split("\n").filter((line) => line.trim() === "---").length;
  const maybeMatch = text.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/);
  if (maybeMatch === null || delimiterCount !== 2) {
    failures.push(`${relativePath} must contain exactly one YAML frontmatter block`);
    return null;
  }
  return maybeMatch[1] ?? "";
}

function maybeExactScalar(
  frontmatter: string,
  key: string,
  relativePath: string,
  failures: string[],
): string | null {
  const pattern = new RegExp(`^${escapeRegExp(key)}[ \\t]*:[ \\t]*(.*?)[ \\t]*$`, "gm");
  const matches = [...frontmatter.matchAll(pattern)];
  if (matches.length !== 1) {
    failures.push(`${relativePath} requires exactly one ${key} field; found ${matches.length}`);
    return null;
  }
  const value = stripYamlQuotes((matches[0]?.[1] ?? "").trim());
  if (value === "") {
    failures.push(`${relativePath} requires a non-empty ${key} field`);
    return null;
  }
  return value;
}

function requireScalar(
  frontmatter: string,
  key: string,
  expected: string,
  relativePath: string,
  failures: string[],
): void {
  const maybeValue = maybeExactScalar(frontmatter, key, relativePath, failures);
  if (maybeValue !== null && maybeValue !== expected) {
    failures.push(`${relativePath} requires ${key}: ${expected}`);
  }
}

function isPreVerificationSummaryCount(value: number): value is 1 | 2 | 3 {
  return value === 1 || value === 2 || value === 3;
}

function isPromoted(stage: Phase125LifecycleStage): boolean {
  return stage.kind === "post_verification" || stage.kind === "post_summary";
}

function phaseSection(roadmap: string, phase: number): string {
  const marker = `#### Phase ${phase}:`;
  const start = roadmap.indexOf(marker);
  if (start === -1) return "";
  const end = roadmap.indexOf("\n#### Phase ", start + marker.length);
  return roadmap.slice(start, end === -1 ? roadmap.length : end);
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) failures.push(`${label} missing ${needle}`);
}

function requireAbsent(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) failures.push(`${label} must not contain ${needle}`);
}

function requireExactNumber(
  actual: number,
  expected: number,
  label: string,
  failures: string[],
): void {
  if (actual !== expected) failures.push(`${label}: expected ${expected}, found ${actual}`);
}

function countOccurrences(text: string, needle: string): number {
  return text.split(needle).length - 1;
}

function stripYamlQuotes(value: string): string {
  const quoted =
    (value.startsWith('"') && value.endsWith('"')) ||
    (value.startsWith("'") && value.endsWith("'"));
  return quoted ? value.slice(1, -1) : value;
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
