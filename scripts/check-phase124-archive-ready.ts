import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

const PHASE129_DIRECTORY =
  ".planning/phases/129-integration-guardrails-and-milestone-reconciliation";
const PHASE129_LIFECYCLE_ID = "129-2026-07-20T19-28-06";
const ARCHIVE_ROUTE = "/gsd-complete-milestone v2.1";
const REQUIREMENTS_FILE = ".planning/REQUIREMENTS.md";
const ROADMAP_FILE = ".planning/ROADMAP.md";
const AUDIT_FILE = ".planning/v2.1-MILESTONE-AUDIT.md";
const ARCHIVED_MILESTONE_FILES = {
  [REQUIREMENTS_FILE]: ".planning/milestones/v2.1-REQUIREMENTS.md",
  [ROADMAP_FILE]: ".planning/milestones/v2.1-ROADMAP.md",
  [AUDIT_FILE]: ".planning/milestones/v2.1-MILESTONE-AUDIT.md",
} as const;
const PHASE129_REQUIREMENTS = ["OBS-01", "BOUND-02", "HARD-05"] as const;
const PHASE129_ROW_CHECKED =
  "- [x] **Phase 129: Integration Guardrails and Milestone Reconciliation**";
const PHASE129_AUDIT_GAP_IDS = [
  "BSRV-03",
  "BSRV-04",
  "CMP-04",
  "CMP-05",
  "OBS-01",
  "OBS-02",
  "OBS-03",
  "OBS-04",
  "BOUND-02",
  "HARD-05",
] as const;
const ARCHIVE_READY_STALE_ROUTES = [
  "/gsd-plan-phase 129",
  "/gsd-plan-phase 128",
  "/gsd-execute-phase 129",
] as const;
const ARCHIVE_READY_ROUTED_FILES = [
  ".planning/STATE.md",
  ".planning/MILESTONES.md",
  ".planning/PROJECT.md",
] as const;
const ARCHIVE_READY_PHASE_DIRECTORIES = [
  ".planning/phases/127-authoritative-network-state-unification",
  ".planning/phases/128-production-compact-announcement-transport",
  PHASE129_DIRECTORY,
] as const;
const PHASE129_VERIFICATION_FRONTMATTER = [
  ["status", "passed"],
  ["lifecycle_validated", "true"],
  ["generated_by", "gsd-verifier"],
  ["lifecycle_mode", "yolo"],
  ["phase_lifecycle_id", PHASE129_LIFECYCLE_ID],
] as const;
const RETAINED_TECH_DEBT_ITEM =
  "scripts/check-phase124-milestone-gap-closure.ts exceeds 1,500 lines and concentrates unrelated lifecycle assertions.";
const RESOLVED_TECH_DEBT_ENTRY = "phase: cross-cutting-verification";

export type Phase129ReconciliationStage =
  | "gaps_open"
  | "verified_pre_promotion"
  | "archive_ready";

/**
 * Detects which of the three legal Phase 129 reconciliation states the
 * planning artifacts claim. Any archive-ready evidence (audit passed, a
 * promoted Phase 129 requirement, or a checked roadmap row) claims the
 * archive-ready stage, whose full condition set is then enforced fail-closed
 * by `verifyArchiveReady` so mixtures cannot pass.
 */
export function detectPhase129ReconciliationStage(
  repoRoot: string,
  failures: string[],
): Phase129ReconciliationStage {
  const requirements = readOptional(repoRoot, REQUIREMENTS_FILE);
  const roadmap = readOptional(repoRoot, ROADMAP_FILE);
  const audit = readOptional(repoRoot, AUDIT_FILE);
  const entries = parseRequirementEntries(requirements);
  const anyPromoted = PHASE129_REQUIREMENTS.some(
    (id) => entries.find((entry) => entry.id === id)?.checked === true,
  );
  const maybeAuditFrontmatter = extractFrontmatter(audit, AUDIT_FILE, failures);
  const auditPassed =
    maybeAuditFrontmatter !== null &&
    exactScalar(maybeAuditFrontmatter, "status") === "passed";
  if (auditPassed || anyPromoted || roadmap.includes(PHASE129_ROW_CHECKED)) {
    return "archive_ready";
  }
  const verificationPath = path.join(
    repoRoot,
    PHASE129_DIRECTORY,
    "129-VERIFICATION.md",
  );
  if (existsSync(verificationPath)) return "verified_pre_promotion";
  return "gaps_open";
}

/**
 * Asserts the Phase 129 verified pre-promotion stage: a lifecycle-valid
 * gsd-verifier verification exists while every other artifact remains in the
 * gaps-open projection (which the caller keeps asserting unchanged).
 */
export function verifyVerifiedPrePromotion(
  repoRoot: string,
  failures: string[],
): void {
  verifyPhase129VerificationFrontmatter(repoRoot, failures);
  for (const name of [
    "129-01-SUMMARY.md",
    "129-02-SUMMARY.md",
    "129-03-SUMMARY.md",
  ]) {
    if (!existsSync(path.join(repoRoot, PHASE129_DIRECTORY, name))) {
      failures.push(
        `P124 verified pre-promotion is missing ${PHASE129_DIRECTORY}/${name}`,
      );
    }
  }
  if (existsSync(path.join(repoRoot, PHASE129_DIRECTORY, "129-04-SUMMARY.md"))) {
    failures.push(
      "P124 verified pre-promotion requires 129-04-SUMMARY.md to be absent",
    );
  }
}

/**
 * Asserts the reconciled archive-ready end-state exactly (129-CONTEXT.md
 * D-13): all planning artifacts agree on 39/39, the passed audit, the archive
 * route, Phase 129 ownership of OBS-01/BOUND-02/HARD-05 (D-09), and the
 * complete lifecycle-valid Phase 129 artifact chain. Any missing condition is
 * a mixture and fails (D-10).
 */
export function verifyArchiveReady(repoRoot: string, failures: string[]): void {
  const requirements = readOptional(repoRoot, REQUIREMENTS_FILE);
  const roadmap = readOptional(repoRoot, ROADMAP_FILE);
  const audit = readOptional(repoRoot, AUDIT_FILE);
  verifyArchiveReadyRequirements(requirements, failures);
  verifyArchiveReadyRoadmap(roadmap, failures);
  verifyArchiveReadyAudit(audit, failures);
  verifyArchiveReadyRouting(repoRoot, failures);
  verifyArchiveReadyLifecycle(repoRoot, failures);
  for (const directory of ARCHIVE_READY_PHASE_DIRECTORIES) {
    if (!existsSync(path.join(repoRoot, directory))) {
      failures.push(`P124 archive-ready missing phase directory ${directory}`);
    }
  }
}

function verifyArchiveReadyRequirements(
  requirements: string,
  failures: string[],
): void {
  const entries = parseRequirementEntries(requirements);
  const traceability = parseTraceabilityEntries(requirements);
  requireCount(entries.length, 39, "archive-ready checklist total", failures);
  requireCount(
    entries.filter((entry) => entry.checked).length,
    39,
    "archive-ready checked requirement count",
    failures,
  );
  requireCount(
    traceability.filter((entry) => entry.status === "Complete").length,
    39,
    "archive-ready complete traceability count",
    failures,
  );
  for (const id of PHASE129_REQUIREMENTS) {
    requireContains(
      requirements,
      `- [x] **${id}**`,
      "archive-ready requirements checklist",
      failures,
    );
  }
  for (const row of [
    "| OBS-01 | Phase 129 | Complete |",
    "| BOUND-02 | Phase 129 | Complete |",
    "| HARD-05 | Phase 129 | Complete |",
  ]) {
    requireContains(
      requirements,
      row,
      "archive-ready requirement ownership",
      failures,
    );
  }
  for (const line of ["Complete: 39", "Pending integration gap closure: 0"]) {
    requireContains(
      requirements,
      line,
      "archive-ready requirements coverage",
      failures,
    );
  }
}

function verifyArchiveReadyRoadmap(roadmap: string, failures: string[]): void {
  for (const row of [
    "- [x] **Phase 127: Authoritative Network State Unification**",
    "- [x] **Phase 128: Production Compact Announcement Transport**",
    PHASE129_ROW_CHECKED,
  ]) {
    requireContains(roadmap, row, "archive-ready roadmap phase state", failures);
  }
  requireContains(
    phaseSection(roadmap, 129),
    "**Plans:** 4/4 plans complete",
    "archive-ready Phase 129 plan state",
    failures,
  );
  for (const line of [
    "126 -> 127 -> 128 -> 129",
    "#### Phase 125:",
    "#### Phase 126:",
    "Satisfied: 39",
    "Pending integration gap closure: 0",
  ]) {
    requireContains(roadmap, line, "archive-ready roadmap topology", failures);
  }
  requireContains(
    roadmap,
    `## Next Step\n\nRun \`${ARCHIVE_ROUTE}\`.`,
    "archive-ready roadmap route",
    failures,
  );
}

function verifyArchiveReadyAudit(audit: string, failures: string[]): void {
  for (const line of [
    "status: passed",
    'requirements: "39/39"',
    'phases: "20/20"',
    'integration: "13/13"',
    'flows: "11/11"',
    "gaps:\n  requirements: []\n  integration: []\n  flows: []",
    RETAINED_TECH_DEBT_ITEM,
  ]) {
    requireContains(audit, line, "archive-ready audit", failures);
  }
  for (const stale of [
    "status: gaps_found",
    "- id: GAP-0",
    "- id: FLOW-0",
    ...PHASE129_AUDIT_GAP_IDS.map((id) => `- id: ${id}`),
    RESOLVED_TECH_DEBT_ENTRY,
  ]) {
    requireAbsent(audit, stale, "archive-ready audit", failures);
  }
  requireContains(
    audit,
    `## Next Action\n\nRun \`${ARCHIVE_ROUTE}\``,
    "archive-ready audit route",
    failures,
  );
  for (const staleRoute of ["/gsd-plan-phase", "/gsd-execute-phase"]) {
    requireAbsent(audit, staleRoute, "archive-ready audit stale route", failures);
  }
}

function verifyArchiveReadyRouting(repoRoot: string, failures: string[]): void {
  const milestoneArchived = milestoneArchivePresent(repoRoot);
  for (const relativePath of ARCHIVE_READY_ROUTED_FILES) {
    const absolutePath = path.join(repoRoot, relativePath);
    if (!existsSync(absolutePath)) {
      failures.push(`P124 archive-ready routing missing ${relativePath}`);
      continue;
    }
    const text = readFileSync(absolutePath, "utf8");
    if (milestoneArchived) {
      requireContains(
        text,
        "/gsd-new-milestone",
        `archived milestone routing ${relativePath}`,
        failures,
      );
      continue;
    }
    requireContains(
      text,
      ARCHIVE_ROUTE,
      `archive-ready routing ${relativePath}`,
      failures,
    );
    for (const staleRoute of ARCHIVE_READY_STALE_ROUTES) {
      requireAbsent(
        text,
        staleRoute,
        `archive-ready routing ${relativePath}`,
        failures,
      );
    }
  }
}

function verifyArchiveReadyLifecycle(
  repoRoot: string,
  failures: string[],
): void {
  const absoluteDirectory = path.join(repoRoot, PHASE129_DIRECTORY);
  if (!existsSync(absoluteDirectory)) {
    failures.push(
      `P124 archive-ready missing phase directory ${PHASE129_DIRECTORY}`,
    );
    return;
  }
  const names = readdirSync(absoluteDirectory).sort();
  const planNames = names.filter((name) => /^129-\d\d-PLAN\.md$/.test(name));
  const summaryNames = names.filter((name) =>
    /^129-\d\d-SUMMARY\.md$/.test(name),
  );
  const expectedPlans = [
    "129-01-PLAN.md",
    "129-02-PLAN.md",
    "129-03-PLAN.md",
    "129-04-PLAN.md",
  ];
  if (JSON.stringify(planNames) !== JSON.stringify(expectedPlans)) {
    failures.push(
      "P124 archive-ready lifecycle requires exactly plans 01 through 04",
    );
  }
  const expectedSummaries = expectedPlans.map((name) =>
    name.replace("-PLAN.md", "-SUMMARY.md"),
  );
  if (JSON.stringify(summaryNames) !== JSON.stringify(expectedSummaries)) {
    failures.push("P124 archive-ready lifecycle requires all four summaries");
  }
  if (names.filter((name) => name === "129-VERIFICATION.md").length !== 1) {
    failures.push(
      "P124 archive-ready lifecycle requires exactly one verification artifact",
    );
  } else {
    verifyPhase129VerificationFrontmatter(repoRoot, failures);
  }
  verifyPhase129Artifact(
    repoRoot,
    `${PHASE129_DIRECTORY}/129-CONTEXT.md`,
    "gsd-discuss-phase",
    failures,
  );
  for (const name of planNames) {
    verifyPhase129Artifact(
      repoRoot,
      `${PHASE129_DIRECTORY}/${name}`,
      "gsd-plan-phase",
      failures,
    );
  }
  let activationSummaryPresent = false;
  for (const name of summaryNames) {
    const maybeFrontmatter = verifyPhase129Artifact(
      repoRoot,
      `${PHASE129_DIRECTORY}/${name}`,
      "gsd-execute-plan",
      failures,
    );
    if (
      maybeFrontmatter !== null &&
      listsPhase129Requirements(maybeFrontmatter)
    ) {
      activationSummaryPresent = true;
    }
  }
  if (!activationSummaryPresent) {
    failures.push(
      "P124 archive-ready lifecycle requires a summary listing OBS-01, BOUND-02, HARD-05 in requirements-completed",
    );
  }
}

function verifyPhase129VerificationFrontmatter(
  repoRoot: string,
  failures: string[],
): void {
  const relativePath = `${PHASE129_DIRECTORY}/129-VERIFICATION.md`;
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P124 Phase 129 lifecycle missing ${relativePath}`);
    return;
  }
  const maybeFrontmatter = extractFrontmatter(
    readFileSync(absolutePath, "utf8"),
    relativePath,
    failures,
  );
  if (maybeFrontmatter === null) return;
  for (const [key, expected] of PHASE129_VERIFICATION_FRONTMATTER) {
    if (exactScalar(maybeFrontmatter, key) !== expected) {
      failures.push(
        `P124 ${relativePath} requires exactly one ${key}: ${expected}`,
      );
    }
  }
}

function verifyPhase129Artifact(
  repoRoot: string,
  relativePath: string,
  expectedGenerator: string,
  failures: string[],
): string | null {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P124 Phase 129 lifecycle missing ${relativePath}`);
    return null;
  }
  const maybeFrontmatter = extractFrontmatter(
    readFileSync(absolutePath, "utf8"),
    relativePath,
    failures,
  );
  if (maybeFrontmatter === null) return null;
  if (exactScalar(maybeFrontmatter, "generated_by") !== expectedGenerator) {
    failures.push(`P124 ${relativePath} requires generated_by: ${expectedGenerator}`);
  }
  if (exactScalar(maybeFrontmatter, "lifecycle_mode") !== "yolo") {
    failures.push(`P124 ${relativePath} requires lifecycle_mode: yolo`);
  }
  if (
    exactScalar(maybeFrontmatter, "phase_lifecycle_id") !== PHASE129_LIFECYCLE_ID
  ) {
    failures.push(
      `P124 ${relativePath} requires phase_lifecycle_id: ${PHASE129_LIFECYCLE_ID}`,
    );
  }
  return maybeFrontmatter;
}

function listsPhase129Requirements(frontmatter: string): boolean {
  const block = requirementsCompletedBlock(frontmatter);
  return PHASE129_REQUIREMENTS.every((id) => block.includes(id));
}

function requirementsCompletedBlock(frontmatter: string): string {
  const lines = frontmatter.split("\n");
  const startIndex = lines.findIndex((line) =>
    line.startsWith("requirements-completed:"),
  );
  if (startIndex === -1) return "";
  const collected = [lines[startIndex] ?? ""];
  for (const line of lines.slice(startIndex + 1)) {
    if (!/^\s+- /.test(line)) break;
    collected.push(line);
  }
  return collected.join("\n");
}

function readOptional(repoRoot: string, relativePath: string): string {
  const sourcePath = milestoneArchivePresent(repoRoot)
    ? ARCHIVED_MILESTONE_FILES[
        relativePath as keyof typeof ARCHIVED_MILESTONE_FILES
      ] ?? relativePath
    : relativePath;
  const absolutePath = path.join(repoRoot, sourcePath);
  return existsSync(absolutePath) ? readFileSync(absolutePath, "utf8") : "";
}

function milestoneArchivePresent(repoRoot: string): boolean {
  return Object.values(ARCHIVED_MILESTONE_FILES).every((file) =>
    existsSync(path.join(repoRoot, file)),
  );
}

function extractFrontmatter(
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

function exactScalar(frontmatter: string, key: string): string | null {
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

function parseRequirementEntries(
  text: string,
): { checked: boolean; id: string }[] {
  return [...text.matchAll(/^- \[([ x])\] \*\*([A-Z]+-\d+)\*\*:/gm)].map(
    (match) => ({ checked: match[1] === "x", id: match[2] ?? "" }),
  );
}

function parseTraceabilityEntries(
  text: string,
): { id: string; phase: number; status: string }[] {
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
  if (!text.includes(needle)) failures.push(`P124 ${label} is missing ${needle}`);
}

function requireAbsent(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) failures.push(`P124 ${label} must not contain ${needle}`);
}

function requireCount(
  actual: number,
  expected: number,
  label: string,
  failures: string[],
): void {
  if (actual !== expected) {
    failures.push(`P124 ${label} must be ${expected}; found ${actual}`);
  }
}
