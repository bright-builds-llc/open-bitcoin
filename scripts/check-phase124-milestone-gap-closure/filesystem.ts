import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { PHASE125_DIRECTORY, PHASE126_DIRECTORY, PHASE125_CONTEXT, PHASE125_VERIFICATION, PHASE126_CONTEXT, PHASE126_VERIFICATION, PHASE125_NAME, PHASE126_NAME, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, GAP_REQUIREMENT_PHASES, EXPECTED_PLAN_NUMBERS, PHASE125_ROUTE, PHASE126_ROUTE, ARCHIVE_ROUTE, ROUTING_FILES } from "./constants.ts";
import type { Phase125LifecycleStage, Phase126CloseoutStage, RequirementEntry, TraceabilityEntry, LifecycleIdentity, Phase125Artifacts, ProjectionState } from "./constants.ts";
import { isPhase124GapClosureStage, verifyPhase124GapClosureStage, verifyCompletedGapClosureLifecycleArtifacts, phase126LifecycleStarted, verifyPhase126CloseoutStage, maybeParsePhase126Projection, maybeParsePhase126Artifacts, verifyExactPhase126PlanSet, verifyPhase126NumberedArtifact, verifyPhase126VerificationArtifact, verifyPhase126LifecycleMatches } from "./lifecycle.ts";
import { maybeParsePhase126Stage, verifyPhase126Projection, verifyPhase126CoverageCounts, verifyPhase126Roadmap, verifyPhase126Audit, verifyPhase126Routing, routeLabel } from "./projection.ts";
import { maybeParsePhase125LifecycleStage, maybeParseProjectionState, maybeParsePhase125Artifacts, maybeReadLifecycleIdentity, verifyExactPlanSet, verifyNumberedArtifact, verifyVerificationArtifact, verifyLifecycleMatches, maybeReadFrontmatter, verifyRequirementOwnership } from "./routing.ts";
import { maybeExactScalar, requireScalar, isPreVerificationSummaryCount, isPhase126CandidateSummaryCount, isPromoted, isPhase126Promoted, phaseSection, requireContains, requireAbsent, requireExactNumber, countOccurrences, stripYamlQuotes, escapeRegExp } from "./parsing.ts";
export function verifyProjection(
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

export function expectedRequirementComplete(id: string, promoted: boolean): boolean {
  if (PHASE126_REQUIREMENTS.includes(id as (typeof PHASE126_REQUIREMENTS)[number])) {
    return false;
  }
  if (PHASE125_REQUIREMENTS.includes(id as (typeof PHASE125_REQUIREMENTS)[number])) {
    return promoted;
  }
  return true;
}

export function verifyCoverageCounts(
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

export function verifyRoadmapPhases(
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

export function expectedPhase125Progress(stage: Phase125LifecycleStage): string {
  if (stage.kind === "planned") return "**Plans:** 4 plans";
  if (stage.kind === "post_summary") return "**Plans:** 4/4 plans complete";
  return `**Plans:** ${stage.summaryCount}/4 plans executed`;
}

export function verifyAudit(
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

export function verifyRouting(
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

export function verifyPostSummaryNarrative(
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

export function verifyPhaseDirectories(repoRoot: string, failures: string[]): void {
  for (const directory of [PHASE125_DIRECTORY, PHASE126_DIRECTORY]) {
    if (!existsSync(path.join(repoRoot, directory))) {
      failures.push(`P124 gap-closure missing phase directory ${directory}`);
    }
  }
}

export function parseRequirementEntries(text: string): RequirementEntry[] {
  return [...text.matchAll(/^- \[([ x])\] \*\*([A-Z]+-\d+)\*\*:/gm)].map(
    (match) => ({ checked: match[1] === "x", id: match[2] ?? "" }),
  );
}

export function parseTraceabilityEntries(text: string): TraceabilityEntry[] {
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

export function maybeExtractFrontmatter(
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
