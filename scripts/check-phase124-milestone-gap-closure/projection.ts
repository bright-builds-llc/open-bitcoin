import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { PHASE125_DIRECTORY, PHASE126_DIRECTORY, PHASE125_CONTEXT, PHASE125_VERIFICATION, PHASE126_CONTEXT, PHASE126_VERIFICATION, PHASE125_NAME, PHASE126_NAME, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, GAP_REQUIREMENT_PHASES, EXPECTED_PLAN_NUMBERS, PHASE125_ROUTE, PHASE126_ROUTE, ARCHIVE_ROUTE, ROUTING_FILES } from "./constants.ts";
import type { Phase125LifecycleStage, Phase126CloseoutStage, RequirementEntry, TraceabilityEntry, LifecycleIdentity, Phase125Artifacts, ProjectionState } from "./constants.ts";
import { isPhase124GapClosureStage, verifyPhase124GapClosureStage, verifyCompletedGapClosureLifecycleArtifacts, phase126LifecycleStarted, verifyPhase126CloseoutStage, maybeParsePhase126Projection, maybeParsePhase126Artifacts, verifyExactPhase126PlanSet, verifyPhase126NumberedArtifact, verifyPhase126VerificationArtifact, verifyPhase126LifecycleMatches } from "./lifecycle.ts";
import { maybeParsePhase125LifecycleStage, maybeParseProjectionState, maybeParsePhase125Artifacts, maybeReadLifecycleIdentity, verifyExactPlanSet, verifyNumberedArtifact, verifyVerificationArtifact, verifyLifecycleMatches, maybeReadFrontmatter, verifyRequirementOwnership } from "./routing.ts";
import { verifyProjection, expectedRequirementComplete, verifyCoverageCounts, verifyRoadmapPhases, expectedPhase125Progress, verifyAudit, verifyRouting, verifyPostSummaryNarrative, verifyPhaseDirectories, parseRequirementEntries, parseTraceabilityEntries, maybeExtractFrontmatter } from "./filesystem.ts";
import { maybeExactScalar, requireScalar, isPreVerificationSummaryCount, isPhase126CandidateSummaryCount, isPromoted, isPhase126Promoted, phaseSection, requireContains, requireAbsent, requireExactNumber, countOccurrences, stripYamlQuotes, escapeRegExp } from "./parsing.ts";
export function maybeParsePhase126Stage(
  projection: ProjectionState,
  artifacts: Phase125Artifacts,
  failures: string[],
): Phase126CloseoutStage | null {
  const { planCount, summaryCount, verificationPresent } = artifacts;
  if (planCount !== 4) {
    failures.push(
      `P124 Phase 126 lifecycle requires exactly four plans; found ${planCount}`,
    );
    return null;
  }

  if (
    projection === "pending" &&
    !verificationPresent &&
    isPhase126CandidateSummaryCount(summaryCount)
  ) {
    return {
      kind: "candidate",
      planCount: 4,
      summaryCount,
      verificationPresent: false,
    };
  }
  if (projection === "pending" && verificationPresent && summaryCount === 3) {
    return {
      kind: "verified_pre_promotion",
      planCount: 4,
      summaryCount: 3,
      verificationPresent: true,
    };
  }
  if (projection === "promoted" && verificationPresent && summaryCount === 3) {
    return {
      kind: "promoted_pre_summary",
      planCount: 4,
      summaryCount: 3,
      verificationPresent: true,
    };
  }
  if (projection === "promoted" && verificationPresent && summaryCount === 4) {
    return {
      kind: "archive_ready",
      planCount: 4,
      summaryCount: 4,
      verificationPresent: true,
    };
  }

  if (projection === "promoted" && !verificationPresent) {
    failures.push(
      "P124 Phase 126 promoted projection requires lifecycle-valid verification",
    );
  }
  failures.push(
    `P124 Phase 126 artifact combination does not match a legal closeout state: ${planCount} plans, ${summaryCount} summaries, verification ${verificationPresent ? "present" : "absent"}, projection ${projection}`,
  );
  return null;
}

export function verifyPhase126Projection(
  stage: Phase126CloseoutStage,
  requirements: string,
  roadmap: string,
  audit: string,
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): void {
  const promoted = isPhase126Promoted(stage);
  const expectedComplete = promoted ? 39 : 33;
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
    const expectedChecked =
      !PHASE126_REQUIREMENTS.includes(
        entry.id as (typeof PHASE126_REQUIREMENTS)[number],
      ) || promoted;
    if (entry.checked !== expectedChecked) {
      failures.push(`P124 ${stage.kind} checklist state is invalid for ${entry.id}`);
    }
  }
  for (const entry of traceability) {
    const expectedStatus =
      !PHASE126_REQUIREMENTS.includes(
        entry.id as (typeof PHASE126_REQUIREMENTS)[number],
      ) || promoted
        ? "Complete"
        : "Pending";
    if (entry.status !== expectedStatus) {
      failures.push(
        `P124 ${stage.kind} traceability status is invalid for ${entry.id}`,
      );
    }
  }

  verifyPhase126CoverageCounts(
    requirements,
    "Complete",
    expectedComplete,
    expectedPending,
    "requirements",
    failures,
  );
  verifyPhase126CoverageCounts(
    roadmap,
    "Satisfied",
    expectedComplete,
    expectedPending,
    "roadmap",
    failures,
  );
  verifyPhase126Roadmap(stage, roadmap, failures);
  verifyPhase126Audit(stage, audit, failures);
}

export function verifyPhase126CoverageCounts(
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
      `P124 Phase 126 ${corpusLabel} coverage counts`,
      failures,
    );
  }
}

export function verifyPhase126Roadmap(
  stage: Phase126CloseoutStage,
  roadmap: string,
  failures: string[],
): void {
  const phase125 = phaseSection(roadmap, 125);
  const phase126 = phaseSection(roadmap, 126);
  requireContains(
    roadmap,
    `- [x] **Phase 125: ${PHASE125_NAME}**`,
    `P124 ${stage.kind} Phase 125 state`,
    failures,
  );
  requireContains(
    phase125,
    "**Plans:** 4/4 plans complete",
    `P124 ${stage.kind} Phase 125 plans`,
    failures,
  );

  const archiveReady = stage.kind === "archive_ready";
  requireContains(
    roadmap,
    `- [${archiveReady ? "x" : " "}] **Phase 126: ${PHASE126_NAME}**`,
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
  const expectedProgress = archiveReady
    ? "**Plans:** 4/4 plans complete"
    : `**Plans:** ${stage.summaryCount}/4 plans executed`;
  requireContains(
    phase126,
    expectedProgress,
    `P124 ${stage.kind} Phase 126 plans`,
    failures,
  );
}

export function verifyPhase126Audit(
  stage: Phase126CloseoutStage,
  audit: string,
  failures: string[],
): void {
  const relativePath = ".planning/v2.1-MILESTONE-AUDIT.md";
  const promoted = isPhase126Promoted(stage);
  const maybeFrontmatter = maybeExtractFrontmatter(audit, relativePath, failures);
  if (maybeFrontmatter !== null) {
    requireScalar(
      maybeFrontmatter,
      "status",
      promoted ? "passed" : "gaps_found",
      relativePath,
      failures,
    );
  }
  requireContains(
    audit,
    `requirements: "${promoted ? 39 : 33}/39"`,
    `P124 ${stage.kind} audit requirements`,
    failures,
  );
  requireContains(
    audit,
    `phases: "${promoted ? 17 : 16}/17"`,
    `P124 ${stage.kind} audit phases`,
    failures,
  );
  requireContains(audit, "integration: []", `P124 ${stage.kind} audit`, failures);
  requireContains(audit, "flows: []", `P124 ${stage.kind} audit`, failures);
  if (promoted) {
    requireContains(
      audit,
      "requirements: []",
      `P124 ${stage.kind} audit requirement gaps`,
      failures,
    );
    requireContains(audit, "tech_debt: []", `P124 ${stage.kind} audit debt`, failures);
  }
  for (const requirement of PHASE126_REQUIREMENTS) {
    requireExactNumber(
      countOccurrences(audit, `- id: ${requirement}`),
      promoted ? 0 : 1,
      `P124 ${stage.kind} audit gap ${requirement}`,
      failures,
    );
  }
}

export function verifyPhase126Routing(
  repoRoot: string,
  stage: Phase126CloseoutStage,
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
      failures.push(`P124 Phase 126 routing missing ${relativePath}`);
      texts.set(relativePath, "");
      continue;
    }
    texts.set(relativePath, readFileSync(absolutePath, "utf8"));
  }

  const archiveReady = stage.kind === "archive_ready";
  const expectedRoute = archiveReady ? ARCHIVE_ROUTE : PHASE126_ROUTE;
  const forbiddenRoutes = archiveReady
    ? [PHASE125_ROUTE, PHASE126_ROUTE]
    : [PHASE125_ROUTE, ARCHIVE_ROUTE];
  for (const relativePath of ROUTING_FILES) {
    const text = texts.get(relativePath) ?? "";
    for (const forbiddenRoute of forbiddenRoutes) {
      requireAbsent(
        text,
        forbiddenRoute,
        `P124 ${stage.kind} stale ${routeLabel(forbiddenRoute)} route ${relativePath}`,
        failures,
      );
    }
  }

  const routeRequiredIn = archiveReady
    ? [".planning/ROADMAP.md", ".planning/STATE.md", ".planning/v2.1-MILESTONE-AUDIT.md"]
    : [".planning/ROADMAP.md", ".planning/PROJECT.md", ".planning/v2.1-MILESTONE-AUDIT.md"];
  for (const relativePath of routeRequiredIn) {
    requireContains(
      texts.get(relativePath) ?? "",
      expectedRoute,
      `P124 ${stage.kind} primary route ${relativePath}`,
      failures,
    );
  }
}

export function routeLabel(route: string): string {
  if (route === PHASE125_ROUTE) return "Phase 125";
  if (route === PHASE126_ROUTE) return "Phase 126";
  return "milestone completion";
}
