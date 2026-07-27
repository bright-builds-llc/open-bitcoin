import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import {
  detectPhase129ReconciliationStage,
  verifyArchiveReady,
  verifyVerifiedPrePromotion,
} from "../check-phase124-archive-ready";
import { GAP_PHASES, GAP_REQUIREMENTS, ROUTING_FILES, PHASE127_ROUTE, PHASE128_ROUTE, PHASE129_ROUTE, PHASE128_EXECUTION_ROUTE, PHASE127_DIRECTORY, PHASE127_REQUIREMENTS } from "./constants.ts";
import type { RequirementEntry, TraceabilityEntry, Phase127Lifecycle, Phase128LifecycleStage } from "./constants.ts";
import { isPostAuditGapPlanningStage, verifyPostAuditGapPlanningStage, verifyRequirementProjection, verifyCoverage, verifyRoadmapTopology, verifyAudit, verifyRouting, phase128LifecycleStage } from "./projection.ts";

export function verifyPhase127Lifecycle(
  repoRoot: string,
  roadmap: string,
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): Phase127Lifecycle {
  const absoluteDirectory = path.join(repoRoot, PHASE127_DIRECTORY);
  if (!existsSync(absoluteDirectory)) {
    return { complete: false, promoted: false, summaryCount: 0 };
  }
  const names = readdirSync(absoluteDirectory).sort();
  const planNames = names.filter((name) => /^127-\d\d-PLAN\.md$/.test(name));
  const summaryNames = names.filter((name) => /^127-\d\d-SUMMARY\.md$/.test(name));
  const verificationNames = names.filter((name) => name === "127-VERIFICATION.md");
  const phaseSectionText = phaseSection(roadmap, 127);
  const complete = roadmap.includes(
    "- [x] **Phase 127: Authoritative Network State Unification**",
  );
  const projected = PHASE127_REQUIREMENTS.map((id) => ({
    checklist: entries.find((entry) => entry.id === id)?.checked === true,
    traceability:
      traceability.find((entry) => entry.id === id)?.status === "Complete",
  }));
  const promoted = projected.every(
    (state) => state.checklist && state.traceability,
  );
  const pending = projected.every(
    (state) => !state.checklist && !state.traceability,
  );
  if (!promoted && !pending) {
    failures.push(
      "P124 Phase 127 requirement projection must be uniformly pending or promoted",
    );
  }

  if (planNames.length === 0) {
    requireContains(
      phaseSectionText,
      "**Plans:** 0 plans",
      "post-audit Phase 127 plan state",
      failures,
    );
    if (summaryNames.length > 0 || verificationNames.length > 0) {
      failures.push(
        "P124 Phase 127 lifecycle artifacts require the exact four-plan set",
      );
    }
    return { complete, promoted, summaryCount: summaryNames.length };
  }

  const expectedPlans = ["127-01-PLAN.md", "127-02-PLAN.md", "127-03-PLAN.md", "127-04-PLAN.md"];
  if (JSON.stringify(planNames) !== JSON.stringify(expectedPlans)) {
    failures.push("P124 Phase 127 lifecycle requires exactly plans 01 through 04");
  }
  const expectedSummaries = expectedPlans
    .slice(0, summaryNames.length)
    .map((name) => name.replace("-PLAN.md", "-SUMMARY.md"));
  if (
    summaryNames.length > 4 ||
    JSON.stringify(summaryNames) !== JSON.stringify(expectedSummaries)
  ) {
    failures.push(
      "P124 Phase 127 summaries must form a contiguous 01 through 04 prefix",
    );
  }
  if (verificationNames.length > 1) {
    failures.push("P124 Phase 127 requires at most one verification artifact");
  }

  const contextPath = path.join(PHASE127_DIRECTORY, "127-CONTEXT.md");
  const maybeLifecycle = readLifecycleIdentity(
    repoRoot,
    contextPath,
    "gsd-discuss-phase",
    failures,
  );
  for (const name of planNames) {
    verifyLifecycleArtifact(
      repoRoot,
      path.join(PHASE127_DIRECTORY, name),
      maybeLifecycle,
      "gsd-plan-phase",
      name.slice(4, 6),
      failures,
    );
  }
  for (const name of summaryNames) {
    verifyLifecycleArtifact(
      repoRoot,
      path.join(PHASE127_DIRECTORY, name),
      maybeLifecycle,
      "gsd-execute-plan",
      name.slice(4, 6),
      failures,
    );
  }
  const verificationPresent = verificationNames.length === 1;
  if (verificationPresent) {
    const relativePath = path.join(PHASE127_DIRECTORY, verificationNames[0] ?? "");
    verifyLifecycleArtifact(
      repoRoot,
      relativePath,
      maybeLifecycle,
      "gsd-verifier",
      null,
      failures,
    );
    const frontmatter = extractFrontmatter(
      readFileSync(path.join(repoRoot, relativePath), "utf8"),
      relativePath,
      failures,
    );
    if (frontmatter !== null && exactScalar(frontmatter, "status") !== "passed") {
      failures.push(`${relativePath} requires status: passed`);
    }
  }
  if (verificationPresent && summaryNames.length !== 4) {
    failures.push("P124 Phase 127 verification requires all four summaries");
  }
  if (promoted && !verificationPresent) {
    failures.push(
      "P124 Phase 127 promoted projection requires lifecycle-valid verification",
    );
  }
  if (complete && (!promoted || summaryNames.length !== 4)) {
    failures.push(
      "P124 completed Phase 127 requires four summaries and promoted requirements",
    );
  }

  const expectedProgress =
    summaryNames.length === 4
      ? ["**Plans:** 3/4 plans executed", "**Plans:** 4/4 plans complete"]
      : summaryNames.length === 0
        ? ["**Plans:** 4 plans", "**Plans:** 0/4 plans executed"]
        : [`**Plans:** ${summaryNames.length}/4 plans executed`];
  if (!expectedProgress.some((progress) => phaseSectionText.includes(progress))) {
    failures.push(
      `P124 Phase 127 roadmap progress does not match ${summaryNames.length} summaries`,
    );
  }

  return { complete, promoted, summaryCount: summaryNames.length };
}

export function readLifecycleIdentity(
  repoRoot: string,
  relativePath: string,
  expectedGenerator: string,
  failures: string[],
): { lifecycleId: string; mode: string } | null {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P124 Phase 127 lifecycle missing ${relativePath}`);
    return null;
  }
  const frontmatter = extractFrontmatter(
    readFileSync(absolutePath, "utf8"),
    relativePath,
    failures,
  );
  if (frontmatter === null) return null;
  if (exactScalar(frontmatter, "generated_by") !== expectedGenerator) {
    failures.push(`${relativePath} requires generated_by: ${expectedGenerator}`);
  }
  const mode = exactScalar(frontmatter, "lifecycle_mode");
  const lifecycleId = exactScalar(frontmatter, "phase_lifecycle_id");
  if (mode === null || lifecycleId === null) {
    failures.push(`${relativePath} requires Phase 127 lifecycle identity`);
    return null;
  }
  return { lifecycleId, mode };
}

export function verifyLifecycleArtifact(
  repoRoot: string,
  relativePath: string,
  expected: { lifecycleId: string; mode: string } | null,
  expectedGenerator: string,
  maybePlan: string | null,
  failures: string[],
): void {
  const actual = readLifecycleIdentity(
    repoRoot,
    relativePath,
    expectedGenerator,
    failures,
  );
  if (actual === null || expected === null) return;
  if (actual.mode !== expected.mode) {
    failures.push(`${relativePath} lifecycle_mode must match Phase 127 CONTEXT`);
  }
  if (actual.lifecycleId !== expected.lifecycleId) {
    failures.push(
      `${relativePath} phase_lifecycle_id must match Phase 127 CONTEXT`,
    );
  }
  if (maybePlan !== null) {
    const text = readFileSync(path.join(repoRoot, relativePath), "utf8");
    const frontmatter = extractFrontmatter(text, relativePath, failures);
    if (frontmatter !== null && exactScalar(frontmatter, "plan") !== maybePlan) {
      failures.push(`${relativePath} plan number must match its filename`);
    }
  }
}

export function extractFrontmatter(
  text: string,
  relativePath: string,
  failures: string[],
): string | null {
  const delimiters = text.match(/^---$/gm)?.length ?? 0;
  const maybeMatch = text.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/);
  if (maybeMatch === null || delimiters !== 2) {
    failures.push(
      `${relativePath} must contain exactly one YAML frontmatter block`,
    );
    return null;
  }
  return maybeMatch[1] ?? "";
}

export function exactScalar(frontmatter: string, key: string): string | null {
  const matches = [
    ...frontmatter.matchAll(
      new RegExp(`^${key}[ \\t]*:[ \\t]*(.*?)[ \\t]*$`, "gm"),
    ),
  ];
  if (matches.length !== 1) return null;
  const value = (matches[0]?.[1] ?? "").trim();
  const quoted =
    (value.startsWith('"') && value.endsWith('"')) ||
    (value.startsWith("'") && value.endsWith("'"));
  return quoted ? value.slice(1, -1) : value;
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

export function phaseSection(roadmap: string, phase: number): string {
  const marker = `#### Phase ${phase}:`;
  const start = roadmap.indexOf(marker);
  if (start === -1) return "";
  const end = roadmap.indexOf("\n#### Phase ", start + marker.length);
  return roadmap.slice(start, end === -1 ? roadmap.length : end);
}

export function slugify(name: string): string {
  return name.toLowerCase().replaceAll(/[^a-z0-9]+/g, "-").replaceAll(/(^-|-$)/g, "");
}

export function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) failures.push(`P124 ${label} is missing ${needle}`);
}

export function requireAbsent(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) failures.push(`P124 ${label} must not contain ${needle}`);
}

export function requireCount(
  actual: number,
  expected: number,
  label: string,
  failures: string[],
): void {
  if (actual !== expected) {
    failures.push(`P124 ${label} must be ${expected}; found ${actual}`);
  }
}

export function countOccurrences(text: string, needle: string): number {
  return text.split(needle).length - 1;
}
