import { existsSync, readFileSync } from "node:fs";
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

type RequirementEntry = { checked: boolean; id: string };
type TraceabilityEntry = { id: string; phase: number; status: string };

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

  verifyRequirementProjection(entries, traceability, failures);
  verifyCoverage(requirements, roadmap, failures);
  verifyRoadmapTopology(repoRoot, roadmap, failures);
  verifyAudit(audit, failures);
  verifyRouting(repoRoot, roadmap, audit, failures);
}

function verifyRequirementProjection(
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
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
    29,
    "post-audit checked requirement count",
    failures,
  );
  requireCount(
    traceability.filter((entry) => entry.status === "Complete").length,
    29,
    "post-audit complete traceability count",
    failures,
  );

  for (const entry of entries) {
    const shouldBePending = GAP_REQUIREMENTS.has(entry.id);
    if (entry.checked === shouldBePending) {
      failures.push(`P124 post-audit checklist state is invalid for ${entry.id}`);
    }
  }
  for (const entry of traceability) {
    const maybeGapOwner = GAP_REQUIREMENTS.get(entry.id);
    const expectedStatus = maybeGapOwner === undefined ? "Complete" : "Pending";
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
  failures: string[],
): void {
  for (const [text, completeLabel, label] of [
    [requirements, "Complete", "requirements"],
    [roadmap, "Satisfied", "roadmap"],
  ] as const) {
    for (const line of [
      "v2.1 requirements: 39 total",
      "Mapped to phases: 39",
      `${completeLabel}: 29`,
      "Pending integration gap closure: 10",
      "Unmapped: 0",
    ]) {
      requireContains(text, line, `post-audit ${label} coverage`, failures);
    }
  }
}

function verifyRoadmapTopology(
  repoRoot: string,
  roadmap: string,
  failures: string[],
): void {
  requireContains(
    roadmap,
    "126 -> 127 -> 128 -> 129",
    "post-audit execution order",
    failures,
  );
  for (const phase of GAP_PHASES) {
    requireContains(
      roadmap,
      `- [ ] **Phase ${phase.number}: ${phase.name}**`,
      `post-audit Phase ${phase.number} pending state`,
      failures,
    );
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
    requireContains(
      section,
      "**Plans:** 0 plans",
      `post-audit Phase ${phase.number} plan state`,
      failures,
    );
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
  failures: string[],
): void {
  requireContains(
    roadmap,
    `## Next Step\n\nRun \`${PHASE127_ROUTE}\`.`,
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
    requireContains(
      readFileSync(absolutePath, "utf8"),
      PHASE127_ROUTE,
      `post-audit primary route ${relativePath}`,
      failures,
    );
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
