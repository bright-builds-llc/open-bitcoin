import { existsSync } from "node:fs";
import path from "node:path";

const GAP_REQUIREMENT_PHASES = new Map([
  ["RCN-04", 125],
  ["RCN-05", 125],
  ["RCN-06", 125],
  ["CMP-05", 126],
  ["RCN-02", 126],
  ["RCN-03", 126],
  ["GOV-04", 126],
  ["BOUND-01", 126],
  ["HARD-05", 126],
] as const);
const PHASE_DIRECTORIES = [
  ".planning/phases/125-compact-download-verification-traceability-closure",
  ".planning/phases/126-compact-relay-residual-hardening",
] as const;

type RequirementEntry = { checked: boolean; id: string };
type TraceabilityEntry = { id: string; phase: number; status: string };

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

  requireExactNumber(entries.length, 39, "P124 gap-closure requirement checklist total", failures);
  requireExactNumber(traceability.length, 39, "P124 gap-closure traceability total", failures);
  requireExactNumber(
    entries.filter((entry) => entry.checked).length,
    30,
    "P124 gap-closure checked requirement count",
    failures,
  );
  requireExactNumber(
    traceability.filter((entry) => entry.status === "Complete").length,
    30,
    "P124 gap-closure complete traceability count",
    failures,
  );
  requireExactNumber(
    traceability.filter((entry) => entry.status === "Pending").length,
    9,
    "P124 gap-closure pending traceability count",
    failures,
  );

  for (const entry of entries) {
    const maybeGapPhase = GAP_REQUIREMENT_PHASES.get(entry.id);
    if (entry.checked === (maybeGapPhase !== undefined)) {
      failures.push(`P124 gap-closure checklist state is invalid for ${entry.id}`);
    }
  }
  for (const entry of traceability) {
    const maybeGapPhase = GAP_REQUIREMENT_PHASES.get(entry.id);
    const expectedStatus = maybeGapPhase === undefined ? "Complete" : "Pending";
    if (entry.status !== expectedStatus) {
      failures.push(`P124 gap-closure traceability status is invalid for ${entry.id}`);
    }
    if (maybeGapPhase !== undefined && entry.phase !== maybeGapPhase) {
      failures.push(
        `P124 gap-closure ${entry.id} must be owned by Phase ${maybeGapPhase}`,
      );
    }
  }

  verifyCoverageCounts(requirements, "Complete", failures);
  verifyCoverageCounts(roadmap, "Satisfied", failures);
  verifyRoadmapPhases(roadmap, failures);
  verifyAudit(audit, failures);

  for (const directory of PHASE_DIRECTORIES) {
    if (!existsSync(path.join(repoRoot, directory))) {
      failures.push(`P124 gap-closure missing phase directory ${directory}`);
    }
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

function verifyCoverageCounts(
  text: string,
  completeLabel: "Complete" | "Satisfied",
  failures: string[],
): void {
  for (const line of [
    "v2.1 requirements: 39 total",
    "Mapped to phases: 39",
    `${completeLabel}: 30`,
    "Pending hardening and closeout: 9",
    "Unmapped: 0",
  ]) {
    requireContains(text, line, "P124 gap-closure coverage counts", failures);
  }
}

function verifyRoadmapPhases(roadmap: string, failures: string[]): void {
  requireContains(roadmap, "- [x] **Phase 124:", "P124 gap-closure Phase 124 state", failures);
  requireContains(phaseSection(roadmap, 124), "**Plans:** 2/2 plans complete", "P124 gap-closure Phase 124 plans", failures);
  for (const [phase, name, dependency, requirements] of [
    [125, "Compact Download Verification Traceability Closure", 124, "RCN-04, RCN-05, RCN-06"],
    [126, "Compact Relay Residual Hardening", 125, "CMP-05, RCN-02, RCN-03, GOV-04, BOUND-01, HARD-05"],
  ] as const) {
    const section = phaseSection(roadmap, phase);
    requireContains(roadmap, `- [ ] **Phase ${phase}: ${name}**`, `P124 gap-closure Phase ${phase} state`, failures);
    requireContains(section, `**Depends on:** Phase ${dependency}`, `P124 gap-closure Phase ${phase} dependency`, failures);
    requireContains(section, `**Requirements:** ${requirements}`, `P124 gap-closure Phase ${phase} requirements`, failures);
    requireContains(section, "**Plans:** 0 plans", `P124 gap-closure Phase ${phase} plans`, failures);
  }
  requireContains(roadmap, "/gsd-plan-phase 125", "P124 gap-closure next route", failures);
}

function verifyAudit(audit: string, failures: string[]): void {
  for (const marker of [
    "status: gaps_found",
    'requirements: "36/39"',
    'phases: "15/15"',
    "integration: []",
    "flows: []",
  ]) {
    requireContains(audit, marker, "P124 gap-closure audit", failures);
  }
  for (const requirement of ["RCN-04", "RCN-05", "RCN-06"]) {
    requireExactNumber(
      countOccurrences(audit, `- id: ${requirement}`),
      1,
      `P124 gap-closure audit orphan ${requirement}`,
      failures,
    );
  }
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
  if (!text.includes(needle)) failures.push(`${label} missing ${needle}`);
}

function requireExactNumber(
  actual: number,
  expected: number,
  label: string,
  failures: string[],
): void {
  if (actual !== expected) failures.push(`${label}: expected ${expected}, found ${actual}`);
}

function countOccurrences(text: string, needle: string): number {
  return text.split(needle).length - 1;
}
