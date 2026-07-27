import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { PHASE125_DIRECTORY, PHASE126_DIRECTORY, PHASE125_CONTEXT, PHASE125_VERIFICATION, PHASE126_CONTEXT, PHASE126_VERIFICATION, PHASE125_NAME, PHASE126_NAME, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, GAP_REQUIREMENT_PHASES, EXPECTED_PLAN_NUMBERS, PHASE125_ROUTE, PHASE126_ROUTE, ARCHIVE_ROUTE, ROUTING_FILES } from "./constants.ts";
import type { Phase125LifecycleStage, Phase126CloseoutStage, RequirementEntry, TraceabilityEntry, LifecycleIdentity, Phase125Artifacts, ProjectionState } from "./constants.ts";
import { maybeParsePhase126Stage, verifyPhase126Projection, verifyPhase126CoverageCounts, verifyPhase126Roadmap, verifyPhase126Audit, verifyPhase126Routing, routeLabel } from "./projection.ts";
import { maybeParsePhase125LifecycleStage, maybeParseProjectionState, maybeParsePhase125Artifacts, maybeReadLifecycleIdentity, verifyExactPlanSet, verifyNumberedArtifact, verifyVerificationArtifact, verifyLifecycleMatches, maybeReadFrontmatter, verifyRequirementOwnership } from "./routing.ts";
import { verifyProjection, expectedRequirementComplete, verifyCoverageCounts, verifyRoadmapPhases, expectedPhase125Progress, verifyAudit, verifyRouting, verifyPostSummaryNarrative, verifyPhaseDirectories, parseRequirementEntries, parseTraceabilityEntries, maybeExtractFrontmatter } from "./filesystem.ts";
import { maybeExactScalar, requireScalar, isPreVerificationSummaryCount, isPhase126CandidateSummaryCount, isPromoted, isPhase126Promoted, phaseSection, requireContains, requireAbsent, requireExactNumber, countOccurrences, stripYamlQuotes, escapeRegExp } from "./parsing.ts";
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

  if (maybeStage.kind === "post_summary" && phase126LifecycleStarted(repoRoot)) {
    verifyPhase126CloseoutStage(
      repoRoot,
      requirements,
      roadmap,
      audit,
      entries,
      traceability,
      failures,
    );
    return;
  }

  verifyProjection(maybeStage, requirements, roadmap, audit, entries, traceability, failures);
  verifyRouting(repoRoot, maybeStage, roadmap, audit, failures);
}

export function verifyCompletedGapClosureLifecycleArtifacts(
  repoRoot: string,
  failures: string[],
): void {
  const maybePhase125Artifacts = maybeParsePhase125Artifacts(repoRoot, failures);
  const maybePhase126Artifacts = maybeParsePhase126Artifacts(repoRoot, failures);
  for (const [phase, maybeArtifacts] of [
    [125, maybePhase125Artifacts],
    [126, maybePhase126Artifacts],
  ] as const) {
    if (maybeArtifacts === null) continue;
    if (
      maybeArtifacts.planCount !== 4 ||
      maybeArtifacts.summaryCount !== 4 ||
      !maybeArtifacts.verificationPresent
    ) {
      failures.push(
        `P124 post-audit Phase ${phase} lifecycle must remain complete at 4 plans, 4 summaries, and passed verification`,
      );
    }
  }
}

export function phase126LifecycleStarted(repoRoot: string): boolean {
  const absoluteDirectory = path.join(repoRoot, PHASE126_DIRECTORY);
  if (!existsSync(absoluteDirectory)) return false;
  return readdirSync(absoluteDirectory).some((name) =>
    /^126-\d{2}-(?:PLAN|SUMMARY)\.md$/.test(name),
  );
}

export function verifyPhase126CloseoutStage(
  repoRoot: string,
  requirements: string,
  roadmap: string,
  audit: string,
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): void {
  const maybeProjection = maybeParsePhase126Projection(entries, traceability, failures);
  const maybeArtifacts = maybeParsePhase126Artifacts(repoRoot, failures);
  if (maybeProjection === null || maybeArtifacts === null) return;

  const maybeStage = maybeParsePhase126Stage(maybeProjection, maybeArtifacts, failures);
  if (maybeStage === null) return;

  verifyPhase126Projection(
    maybeStage,
    requirements,
    roadmap,
    audit,
    entries,
    traceability,
    failures,
  );
  verifyPhase126Routing(repoRoot, maybeStage, roadmap, audit, failures);
}

export function maybeParsePhase126Projection(
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): ProjectionState | null {
  const checklistStates = PHASE126_REQUIREMENTS.map((id) =>
    entries.find((entry) => entry.id === id)?.checked,
  );
  const traceabilityStates = PHASE126_REQUIREMENTS.map(
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
    "P124 Phase 126 requirement projection must be uniformly pending or promoted",
  );
  return null;
}

export function maybeParsePhase126Artifacts(
  repoRoot: string,
  failures: string[],
): Phase125Artifacts | null {
  const absoluteDirectory = path.join(repoRoot, PHASE126_DIRECTORY);
  if (!existsSync(absoluteDirectory)) {
    failures.push(`P124 gap-closure missing phase directory ${PHASE126_DIRECTORY}`);
    return null;
  }

  const names = readdirSync(absoluteDirectory).sort();
  const planNames = names.filter((name) => /^126-\d{2}-PLAN\.md$/.test(name));
  const summaryNames = names.filter((name) => /^126-\d{2}-SUMMARY\.md$/.test(name));
  for (const name of names) {
    if (
      /^126-.*-(?:PLAN|SUMMARY)\.md$/.test(name) &&
      !/^126-\d{2}-(?:PLAN|SUMMARY)\.md$/.test(name)
    ) {
      failures.push(`P124 Phase 126 has malformed lifecycle artifact ${name}`);
    }
  }

  const maybeContextIdentity = maybeReadLifecycleIdentity(
    repoRoot,
    PHASE126_CONTEXT,
    "gsd-discuss-phase",
    failures,
  );
  if (maybeContextIdentity === null) return null;

  verifyExactPhase126PlanSet(planNames, failures);
  for (const planName of planNames) {
    verifyPhase126NumberedArtifact(
      repoRoot,
      path.join(PHASE126_DIRECTORY, planName),
      planName,
      "PLAN",
      "gsd-plan-phase",
      maybeContextIdentity,
      failures,
    );
  }
  for (const summaryName of summaryNames) {
    verifyPhase126NumberedArtifact(
      repoRoot,
      path.join(PHASE126_DIRECTORY, summaryName),
      summaryName,
      "SUMMARY",
      "gsd-execute-plan",
      maybeContextIdentity,
      failures,
    );
  }

  const verificationPresent = names.includes("126-VERIFICATION.md");
  if (verificationPresent) {
    verifyPhase126VerificationArtifact(repoRoot, maybeContextIdentity, failures);
  }

  return {
    planCount: planNames.length,
    summaryCount: summaryNames.length,
    verificationPresent,
  };
}

export function verifyExactPhase126PlanSet(planNames: string[], failures: string[]): void {
  const expected = EXPECTED_PLAN_NUMBERS.map((number) => `126-${number}-PLAN.md`);
  for (const name of expected) {
    requireExactNumber(
      planNames.filter((candidate) => candidate === name).length,
      1,
      `P124 Phase 126 plan artifact ${name}`,
      failures,
    );
  }
  for (const name of planNames) {
    if (!expected.includes(name)) {
      failures.push(`P124 Phase 126 plan number is outside 01 through 04: ${name}`);
    }
  }
}

export function verifyPhase126NumberedArtifact(
  repoRoot: string,
  relativePath: string,
  name: string,
  kind: "PLAN" | "SUMMARY",
  expectedGenerator: string,
  expectedLifecycle: LifecycleIdentity,
  failures: string[],
): void {
  const maybeMatch = name.match(/^126-(\d{2})-(?:PLAN|SUMMARY)\.md$/);
  const planNumber = maybeMatch?.[1] ?? "";
  if (!EXPECTED_PLAN_NUMBERS.includes(planNumber as (typeof EXPECTED_PLAN_NUMBERS)[number])) {
    failures.push(`P124 Phase 126 ${kind.toLowerCase()} number is outside 01 through 04: ${name}`);
  }
  const maybeFrontmatter = maybeReadFrontmatter(repoRoot, relativePath, failures);
  if (maybeFrontmatter === null) return;

  requireScalar(
    maybeFrontmatter,
    "phase",
    "126-compact-relay-residual-hardening",
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
  verifyPhase126LifecycleMatches(
    maybeFrontmatter,
    relativePath,
    expectedGenerator,
    expectedLifecycle,
    failures,
  );
}

export function verifyPhase126VerificationArtifact(
  repoRoot: string,
  expectedLifecycle: LifecycleIdentity,
  failures: string[],
): void {
  const maybeFrontmatter = maybeReadFrontmatter(repoRoot, PHASE126_VERIFICATION, failures);
  if (maybeFrontmatter === null) return;

  requireScalar(
    maybeFrontmatter,
    "phase",
    "126-compact-relay-residual-hardening",
    PHASE126_VERIFICATION,
    failures,
  );
  requireScalar(maybeFrontmatter, "status", "passed", PHASE126_VERIFICATION, failures);
  requireScalar(
    maybeFrontmatter,
    "lifecycle_validated",
    "true",
    PHASE126_VERIFICATION,
    failures,
  );
  verifyPhase126LifecycleMatches(
    maybeFrontmatter,
    PHASE126_VERIFICATION,
    "gsd-verifier",
    expectedLifecycle,
    failures,
  );
}

export function verifyPhase126LifecycleMatches(
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
    failures.push(`${relativePath} lifecycle_mode must match Phase 126 CONTEXT`);
  }
  if (
    maybePhaseLifecycleId !== null &&
    maybePhaseLifecycleId !== expectedLifecycle.phaseLifecycleId
  ) {
    failures.push(`${relativePath} phase_lifecycle_id must match Phase 126 CONTEXT`);
  }
}
