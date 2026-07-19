import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

const GAP_PHASES = [
  {
    number: 127,
    name: "Authoritative Network State Unification",
    dependsOn: 126,
    requirements: ["BSRV-03", "BSRV-04", "OBS-02", "OBS-04"],
  },
  {
    number: 128,
    name: "Production Compact Announcement Transport",
    dependsOn: 127,
    requirements: ["CMP-04", "CMP-05", "OBS-03"],
  },
  {
    number: 129,
    name: "Integration Guardrails and Milestone Reconciliation",
    dependsOn: 128,
    requirements: ["OBS-01", "BOUND-02", "HARD-05"],
  },
] as const;
const GAP_REQUIREMENTS = new Map(
  GAP_PHASES.flatMap((phase) =>
    phase.requirements.map((requirement) => [requirement, phase.number] as const),
  ),
);
const ROUTING_FILES = [
  ".planning/PROJECT.md",
  ".planning/STATE.md",
  ".planning/MILESTONES.md",
] as const;
const PHASE127_ROUTE = "/gsd-plan-phase 127";
const PHASE128_ROUTE = "/gsd-plan-phase 128";
const PHASE127_DIRECTORY =
  ".planning/phases/127-authoritative-network-state-unification";
const PHASE127_REQUIREMENTS = GAP_PHASES[0].requirements;

type RequirementEntry = { checked: boolean; id: string };
type TraceabilityEntry = { id: string; phase: number; status: string };
type Phase127Lifecycle = {
  complete: boolean;
  promoted: boolean;
  summaryCount: number;
};

export function isPostAuditGapPlanningStage(roadmap: string): boolean {
  return GAP_PHASES.some((phase) =>
    roadmap.includes(`#### Phase ${phase.number}: ${phase.name}`),
  );
}

export function verifyPostAuditGapPlanningStage(
  repoRoot: string,
  requirements: string,
  roadmap: string,
  audit: string,
  failures: string[],
): void {
  const entries = parseRequirementEntries(requirements);
  const traceability = parseTraceabilityEntries(requirements);
  const lifecycle = verifyPhase127Lifecycle(
    repoRoot,
    roadmap,
    entries,
    traceability,
    failures,
  );

  verifyRequirementProjection(entries, traceability, lifecycle.promoted, failures);
  verifyCoverage(requirements, roadmap, lifecycle.promoted, failures);
  verifyRoadmapTopology(repoRoot, roadmap, lifecycle, failures);
  verifyAudit(audit, failures);
  verifyRouting(repoRoot, roadmap, audit, lifecycle.complete, failures);
}

function verifyRequirementProjection(
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  phase127Promoted: boolean,
  failures: string[],
): void {
  requireCount(entries.length, 39, "post-audit checklist total", failures);
  requireCount(traceability.length, 39, "post-audit traceability total", failures);
  requireCount(new Set(entries.map((entry) => entry.id)).size, 39, "post-audit unique checklist total", failures);
  requireCount(
    new Set(traceability.map((entry) => entry.id)).size,
    39,
    "post-audit unique traceability total",
    failures,
  );
  const traceabilityIds = new Set(traceability.map((entry) => entry.id));
  for (const entry of entries) {
    if (!traceabilityIds.has(entry.id)) {
      failures.push(`P124 post-audit traceability is missing ${entry.id}`);
    }
  }
  requireCount(
    entries.filter((entry) => entry.checked).length,
    phase127Promoted ? 33 : 29,
    "post-audit checked requirement count",
    failures,
  );
  requireCount(
    traceability.filter((entry) => entry.status === "Complete").length,
    phase127Promoted ? 33 : 29,
    "post-audit complete traceability count",
    failures,
  );

  for (const entry of entries) {
    const maybeGapOwner = GAP_REQUIREMENTS.get(entry.id);
    const shouldBePending =
      maybeGapOwner !== undefined &&
      !(phase127Promoted && maybeGapOwner === 127);
    if (entry.checked === shouldBePending) {
      failures.push(`P124 post-audit checklist state is invalid for ${entry.id}`);
    }
  }
  for (const entry of traceability) {
    const maybeGapOwner = GAP_REQUIREMENTS.get(entry.id);
    const expectedStatus =
      maybeGapOwner === undefined ||
      (phase127Promoted && maybeGapOwner === 127)
        ? "Complete"
        : "Pending";
    if (entry.status !== expectedStatus) {
      failures.push(`P124 post-audit traceability status is invalid for ${entry.id}`);
    }
    if (maybeGapOwner !== undefined && entry.phase !== maybeGapOwner) {
      failures.push(
        `P124 post-audit ${entry.id} must be owned by Phase ${maybeGapOwner}`,
      );
    }
  }
  for (const [requirement, expectedPhase] of GAP_REQUIREMENTS) {
    const owners = traceability.filter((entry) => entry.id === requirement);
    if (owners.length !== 1 || owners[0]?.phase !== expectedPhase) {
      failures.push(
        `P124 post-audit ${requirement} must be owned by Phase ${expectedPhase}`,
      );
    }
  }
}

function verifyCoverage(
  requirements: string,
  roadmap: string,
  phase127Promoted: boolean,
  failures: string[],
): void {
  const complete = phase127Promoted ? 33 : 29;
  const pending = phase127Promoted ? 6 : 10;
  for (const [text, completeLabel, label] of [
    [requirements, "Complete", "requirements"],
    [roadmap, "Satisfied", "roadmap"],
  ] as const) {
    for (const line of [
      "v2.1 requirements: 39 total",
      "Mapped to phases: 39",
      `${completeLabel}: ${complete}`,
      `Pending integration gap closure: ${pending}`,
      "Unmapped: 0",
    ]) {
      requireContains(text, line, `post-audit ${label} coverage`, failures);
    }
  }
}

function verifyRoadmapTopology(
  repoRoot: string,
  roadmap: string,
  lifecycle: Phase127Lifecycle,
  failures: string[],
): void {
  requireContains(
    roadmap,
    "126 -> 127 -> 128 -> 129",
    "post-audit execution order",
    failures,
  );
  for (const phase of GAP_PHASES) {
    if (phase.number === 127) {
      const expected = lifecycle.complete ? "x" : " ";
      requireContains(
        roadmap,
        `- [${expected}] **Phase ${phase.number}: ${phase.name}**`,
        `post-audit Phase ${phase.number} lifecycle state`,
        failures,
      );
    } else {
      requireContains(
        roadmap,
        `- [ ] **Phase ${phase.number}: ${phase.name}**`,
        `post-audit Phase ${phase.number} pending state`,
        failures,
      );
    }
    const section = phaseSection(roadmap, phase.number);
    requireContains(
      section,
      `**Depends on:** Phase ${phase.dependsOn}`,
      `post-audit Phase ${phase.number} dependency`,
      failures,
    );
    requireContains(
      section,
      `**Requirements:** ${phase.requirements.join(", ")}`,
      `post-audit Phase ${phase.number} requirements`,
      failures,
    );
    if (phase.number !== 127) {
      requireContains(
        section,
        "**Plans:** 0 plans",
        `post-audit Phase ${phase.number} plan state`,
        failures,
      );
    }
    const directory = `.planning/phases/${phase.number}-${slugify(phase.name)}`;
    if (!existsSync(path.join(repoRoot, directory))) {
      failures.push(`P124 post-audit missing phase directory ${directory}`);
    }
  }
}

function verifyAudit(audit: string, failures: string[]): void {
  for (const line of [
    "status: gaps_found",
    'requirements: "29/39"',
    'phases: "17/17"',
    'integration: "9/13"',
    'flows: "7/11"',
  ]) {
    requireContains(audit, line, "post-audit audit score", failures);
  }
  for (const requirement of GAP_REQUIREMENTS.keys()) {
    requireCount(
      countOccurrences(audit, `- id: ${requirement}`),
      1,
      `post-audit audit gap ${requirement}`,
      failures,
    );
  }
  for (const id of ["GAP-01", "GAP-02", "GAP-03", "FLOW-01", "FLOW-02", "FLOW-03", "FLOW-04"]) {
    requireCount(
      countOccurrences(audit, `- id: ${id}`),
      1,
      `post-audit audit gap ${id}`,
      failures,
    );
  }
}

function verifyRouting(
  repoRoot: string,
  roadmap: string,
  audit: string,
  phase127Complete: boolean,
  failures: string[],
): void {
  const primaryRoute = phase127Complete ? PHASE128_ROUTE : PHASE127_ROUTE;
  requireContains(
    roadmap,
    `## Next Step\n\nRun \`${primaryRoute}\`.`,
    "post-audit roadmap route",
    failures,
  );
  requireContains(
    audit,
    `## Next Action\n\nRun \`${PHASE127_ROUTE}\``,
    "post-audit audit route",
    failures,
  );
  for (const relativePath of ROUTING_FILES) {
    const absolutePath = path.join(repoRoot, relativePath);
    if (!existsSync(absolutePath)) {
      failures.push(`P124 post-audit routing missing ${relativePath}`);
      continue;
    }
    const text = readFileSync(absolutePath, "utf8");
    requireContains(
      text,
      primaryRoute,
      `post-audit primary route ${relativePath}`,
      failures,
    );
    if (phase127Complete && text.includes(PHASE127_ROUTE)) {
      failures.push(
        `P124 completed Phase 127 routing ${relativePath} must not retain ${PHASE127_ROUTE}`,
      );
    }
  }
}

function verifyPhase127Lifecycle(
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

function readLifecycleIdentity(
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

function verifyLifecycleArtifact(
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

function phaseSection(roadmap: string, phase: number): string {
  const marker = `#### Phase ${phase}:`;
  const start = roadmap.indexOf(marker);
  if (start === -1) return "";
  const end = roadmap.indexOf("\n#### Phase ", start + marker.length);
  return roadmap.slice(start, end === -1 ? roadmap.length : end);
}

function slugify(name: string): string {
  return name.toLowerCase().replaceAll(/[^a-z0-9]+/g, "-").replaceAll(/(^-|-$)/g, "");
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) failures.push(`P124 ${label} is missing ${needle}`);
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

function countOccurrences(text: string, needle: string): number {
  return text.split(needle).length - 1;
}
