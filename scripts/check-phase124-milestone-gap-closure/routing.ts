import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { PHASE125_DIRECTORY, PHASE126_DIRECTORY, PHASE125_CONTEXT, PHASE125_VERIFICATION, PHASE126_CONTEXT, PHASE126_VERIFICATION, PHASE125_NAME, PHASE126_NAME, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, GAP_REQUIREMENT_PHASES, EXPECTED_PLAN_NUMBERS, PHASE125_ROUTE, PHASE126_ROUTE, ARCHIVE_ROUTE, ROUTING_FILES } from "./constants.ts";
import type { Phase125LifecycleStage, Phase126CloseoutStage, RequirementEntry, TraceabilityEntry, LifecycleIdentity, Phase125Artifacts, ProjectionState } from "./constants.ts";
import { isPhase124GapClosureStage, verifyPhase124GapClosureStage, verifyCompletedGapClosureLifecycleArtifacts, phase126LifecycleStarted, verifyPhase126CloseoutStage, maybeParsePhase126Projection, maybeParsePhase126Artifacts, verifyExactPhase126PlanSet, verifyPhase126NumberedArtifact, verifyPhase126VerificationArtifact, verifyPhase126LifecycleMatches } from "./lifecycle.ts";
import { maybeParsePhase126Stage, verifyPhase126Projection, verifyPhase126CoverageCounts, verifyPhase126Roadmap, verifyPhase126Audit, verifyPhase126Routing, routeLabel } from "./projection.ts";
import { verifyProjection, expectedRequirementComplete, verifyCoverageCounts, verifyRoadmapPhases, expectedPhase125Progress, verifyAudit, verifyRouting, verifyPostSummaryNarrative, verifyPhaseDirectories, parseRequirementEntries, parseTraceabilityEntries, maybeExtractFrontmatter } from "./filesystem.ts";
import { maybeExactScalar, requireScalar, isPreVerificationSummaryCount, isPhase126CandidateSummaryCount, isPromoted, isPhase126Promoted, phaseSection, requireContains, requireAbsent, requireExactNumber, countOccurrences, stripYamlQuotes, escapeRegExp } from "./parsing.ts";
export function maybeParsePhase125LifecycleStage(
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

export function maybeParseProjectionState(
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

export function maybeParsePhase125Artifacts(
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

export function maybeReadLifecycleIdentity(
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

export function verifyExactPlanSet(planNames: string[], failures: string[]): void {
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

export function verifyNumberedArtifact(
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

export function verifyVerificationArtifact(
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

export function verifyLifecycleMatches(
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

export function maybeReadFrontmatter(
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

export function verifyRequirementOwnership(
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
