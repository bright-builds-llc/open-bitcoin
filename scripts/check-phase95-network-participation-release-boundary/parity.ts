import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { DEFAULT_REPO_ROOT, SURFACE_ID, PHASE94_TEST_COMMAND, PHASE94_CHECKER_COMMAND, PHASE95_TEST_COMMAND, PHASE95_CHECKER_COMMAND, PURE_CORE_COMMAND, REQUIRED_PHASE95_REQUIREMENTS, PHASE_REQUIREMENTS, REQUIRED_V1_9_REQUIREMENTS, REQUIREMENT_PHASE_ASSIGNMENTS, ROADMAP_TRACEABILITY_ROWS, REQUIRED_KNOTS_ANCHORS, REQUIRED_PHASE95_EVIDENCE, REQUIRED_UAT_COMMANDS, REQUIRED_SUPPORT_REDACTION_ROOTS, FORBIDDEN_POSITIVE_CLAIMS, POSITIVE_CLAIM_VERBS, FORBIDDEN_VERIFY_STRINGS, CLAIM_SCAN_FILES, TARGET_FILES } from "./constants.ts";
import type { TargetFile, ChecklistSurface, ParityIndex, ParitySurface, CheckPhase95Options } from "./constants.ts";
import { checkPhase95NetworkParticipationReleaseBoundary, readText, verifyParityIndex, verifyTopLevelSurface, verifyPhaseRequirementSurfaces, verifyPhase95ChecklistSurface, requireExactRequirements, requireArrayIncludes, verifyKnotsAnchors, verifyNoClaimBoundary, verifyNoForbiddenClaim, isPositiveClaim, containsUnnegatedVerbClaim, isExplicitDeferredMatrixRow, verifyUatCommands, verifySupportRedactionRoots, verifyVerifierWiring, executableVerifyText, verifyOrderedCommands, verifyRequirementTraceability, verifyChecklistMarkdown } from "./checks.ts";
export function extractRequirementIdsFromSurfaceRows(text: string): string[] {
  const ids: string[] = [];
  for (const surfaceId of Object.keys(PHASE_REQUIREMENTS)) {
    const row = text
      .split("\n")
      .find((line) => line.startsWith("|") && line.includes(surfaceId));
    if (row === undefined) {
      continue;
    }
    ids.push(...requirementIds(row));
  }
  return ids;
}

export function verifyRequirementsTable(text: string, failures: string[]): void {
  requireContains(text, "v1.9 requirements: 28 total", "BOUND-06 requirements coverage", failures);
  requireContains(text, "Mapped to phases: 28", "BOUND-06 requirements coverage", failures);
  requireContains(text, "Unmapped: 0", "BOUND-06 requirements coverage", failures);
  verifyRequirementCountsFromArrays(
    Object.keys(REQUIREMENT_PHASE_ASSIGNMENTS),
    "BOUND-06 requirements traceability assignment map",
    failures,
  );
  for (const [requirement, phase] of Object.entries(REQUIREMENT_PHASE_ASSIGNMENTS)) {
    const rowPattern = new RegExp(`\\|\\s*${escapeRegExp(requirement)}\\s*\\|\\s*Phase ${phase}\\s*\\|`);
    if (!rowPattern.test(text)) {
      failures.push(`BOUND-06 requirements traceability missing ${requirement} -> Phase ${phase}`);
    }
  }
}

export function verifyRoadmapTraceability(text: string, failures: string[]): void {
  requireContains(
    text,
    "Coverage:** 28/28 v1.9 requirements mapped, 0 unmapped",
    "BOUND-06 roadmap coverage",
    failures,
  );
  verifyRequirementCountsFromArrays(
    ROADMAP_TRACEABILITY_ROWS.flatMap(({ requirements }) => requirements),
    "BOUND-06 roadmap traceability rows",
    failures,
  );
  for (const { phase, requirements } of ROADMAP_TRACEABILITY_ROWS) {
    const requirementText = requirements.length === 0 ? "—" : requirements.join(", ");
    const expected = `| Phase ${phase} | ${requirementText} | ${requirements.length} |`;
    requireContains(text, expected, "BOUND-06 roadmap phase traceability", failures);
  }
}

export function verifyRequirementCountsFromArrays(
  ids: readonly string[],
  label: string,
  failures: string[],
): void {
  const counts = new Map<string, number>();
  for (const id of ids) {
    if (!REQUIRED_V1_9_REQUIREMENTS.includes(id)) {
      continue;
    }
    counts.set(id, (counts.get(id) ?? 0) + 1);
  }
  for (const id of REQUIRED_V1_9_REQUIREMENTS) {
    const count = counts.get(id) ?? 0;
    if (count !== 1) {
      failures.push(`${label} BOUND-06 expected ${id} exactly once, found ${count}`);
    }
  }
}

export function requirementIds(text: string): string[] {
  return text.match(/\b(?:INB|PERM|ADDR|EVICT|DOS|BOUND)-\d{2}\b/g) ?? [];
}

export function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

export function contextUnits(text: string): string[] {
  const units: string[] = [];
  for (const block of text.replaceAll("\r\n", "\n").split(/\n\s*\n/)) {
    const lines = block
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
    if (lines.length === 0) {
      continue;
    }

    const tableRows = lines.filter((line) => line.startsWith("|") && !/^\|\s*-/.test(line));
    if (tableRows.length > 0) {
      units.push(...tableRows.map(normalizeWhitespace));
      units.push(...sentenceUnits(lines.filter((line) => !line.startsWith("|")).join(" ")));
      continue;
    }

    units.push(...sentenceUnits(lines.join(" ")));
  }
  return units.map(normalizeWhitespace).filter((unit) => unit.length > 0);
}

export function sentenceUnits(text: string): string[] {
  const normalized = normalizeWhitespace(text);
  return normalized.length === 0 ? [] : normalized.split(/(?<=[.!?])\s+/);
}

export function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

export function normalizedLower(text: string): string {
  return normalizeWhitespace(text).toLowerCase();
}

export function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
