import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { PHASE125_DIRECTORY, PHASE126_DIRECTORY, PHASE125_CONTEXT, PHASE125_VERIFICATION, PHASE126_CONTEXT, PHASE126_VERIFICATION, PHASE125_NAME, PHASE126_NAME, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, GAP_REQUIREMENT_PHASES, EXPECTED_PLAN_NUMBERS, PHASE125_ROUTE, PHASE126_ROUTE, ARCHIVE_ROUTE, ROUTING_FILES } from "./constants.ts";
import type { Phase125LifecycleStage, Phase126CloseoutStage, RequirementEntry, TraceabilityEntry, LifecycleIdentity, Phase125Artifacts, ProjectionState } from "./constants.ts";
import { isPhase124GapClosureStage, verifyPhase124GapClosureStage, verifyCompletedGapClosureLifecycleArtifacts, phase126LifecycleStarted, verifyPhase126CloseoutStage, maybeParsePhase126Projection, maybeParsePhase126Artifacts, verifyExactPhase126PlanSet, verifyPhase126NumberedArtifact, verifyPhase126VerificationArtifact, verifyPhase126LifecycleMatches } from "./lifecycle.ts";
import { maybeParsePhase126Stage, verifyPhase126Projection, verifyPhase126CoverageCounts, verifyPhase126Roadmap, verifyPhase126Audit, verifyPhase126Routing, routeLabel } from "./projection.ts";
import { maybeParsePhase125LifecycleStage, maybeParseProjectionState, maybeParsePhase125Artifacts, maybeReadLifecycleIdentity, verifyExactPlanSet, verifyNumberedArtifact, verifyVerificationArtifact, verifyLifecycleMatches, maybeReadFrontmatter, verifyRequirementOwnership } from "./routing.ts";
import { verifyProjection, expectedRequirementComplete, verifyCoverageCounts, verifyRoadmapPhases, expectedPhase125Progress, verifyAudit, verifyRouting, verifyPostSummaryNarrative, verifyPhaseDirectories, parseRequirementEntries, parseTraceabilityEntries, maybeExtractFrontmatter } from "./filesystem.ts";
export function maybeExactScalar(
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

export function requireScalar(
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

export function isPreVerificationSummaryCount(value: number): value is 1 | 2 | 3 {
  return value === 1 || value === 2 || value === 3;
}

export function isPhase126CandidateSummaryCount(value: number): value is 1 | 2 | 3 {
  return value === 1 || value === 2 || value === 3;
}

export function isPromoted(stage: Phase125LifecycleStage): boolean {
  return stage.kind === "post_verification" || stage.kind === "post_summary";
}

export function isPhase126Promoted(stage: Phase126CloseoutStage): boolean {
  return stage.kind === "promoted_pre_summary" || stage.kind === "archive_ready";
}

export function phaseSection(roadmap: string, phase: number): string {
  const marker = `#### Phase ${phase}:`;
  const start = roadmap.indexOf(marker);
  if (start === -1) return "";
  const end = roadmap.indexOf("\n#### Phase ", start + marker.length);
  return roadmap.slice(start, end === -1 ? roadmap.length : end);
}

export function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) failures.push(`${label} missing ${needle}`);
}

export function requireAbsent(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) failures.push(`${label} must not contain ${needle}`);
}

export function requireExactNumber(
  actual: number,
  expected: number,
  label: string,
  failures: string[],
): void {
  if (actual !== expected) failures.push(`${label}: expected ${expected}, found ${actual}`);
}

export function countOccurrences(text: string, needle: string): number {
  return text.split(needle).length - 1;
}

export function stripYamlQuotes(value: string): string {
  const quoted =
    (value.startsWith('"') && value.endsWith('"')) ||
    (value.startsWith("'") && value.endsWith("'"));
  return quoted ? value.slice(1, -1) : value;
}

export function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
