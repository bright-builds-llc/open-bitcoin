import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import {
  detectPhase129ReconciliationStage,
  verifyArchiveReady,
  verifyVerifiedPrePromotion,
} from "../check-phase124-archive-ready";
import { GAP_PHASES, GAP_REQUIREMENTS, ROUTING_FILES, PHASE127_ROUTE, PHASE128_ROUTE, PHASE129_ROUTE, PHASE128_EXECUTION_ROUTE, PHASE127_DIRECTORY, PHASE127_REQUIREMENTS } from "./constants.ts";
import type { RequirementEntry, TraceabilityEntry, Phase127Lifecycle, Phase128LifecycleStage } from "./constants.ts";
import { verifyPhase127Lifecycle, readLifecycleIdentity, verifyLifecycleArtifact, extractFrontmatter, exactScalar, parseRequirementEntries, parseTraceabilityEntries, phaseSection, slugify, requireContains, requireAbsent, requireCount, countOccurrences } from "./routing.ts";

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
  const phase129Stage = detectPhase129ReconciliationStage(repoRoot, failures);
  const entries = parseRequirementEntries(requirements);
  const traceability = parseTraceabilityEntries(requirements);
  const lifecycle = verifyPhase127Lifecycle(
    repoRoot,
    roadmap,
    entries,
    traceability,
    failures,
  );
  const phase128Stage = phase128LifecycleStage(roadmap, failures);
  if (phase129Stage === "archive_ready") {
    verifyArchiveReady(repoRoot, failures);
    return;
  }

  verifyRequirementProjection(
    entries,
    traceability,
    lifecycle.promoted,
    phase128Stage,
    failures,
  );
  verifyCoverage(requirements, roadmap, lifecycle.promoted, phase128Stage, failures);
  verifyRoadmapTopology(repoRoot, roadmap, lifecycle, phase128Stage, failures);
  verifyAudit(audit, failures);
  verifyRouting(repoRoot, roadmap, audit, lifecycle.complete, phase128Stage, failures);
  if (phase129Stage === "verified_pre_promotion") {
    verifyVerifiedPrePromotion(repoRoot, failures);
  }
}

export function verifyRequirementProjection(
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  phase127Promoted: boolean,
  phase128Stage: Phase128LifecycleStage,
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
    phase128Stage === "planned" ? (phase127Promoted ? 33 : 29) : 36,
    "post-audit checked requirement count",
    failures,
  );
  requireCount(
    traceability.filter((entry) => entry.status === "Complete").length,
    phase128Stage === "planned" ? (phase127Promoted ? 33 : 29) : 36,
    "post-audit complete traceability count",
    failures,
  );
  if (phase128Stage !== "planned" && !phase127Promoted) {
    failures.push("P124 Phase 128 execution requires promoted Phase 127 requirements");
  }

  for (const entry of entries) {
    const maybeGapOwner = GAP_REQUIREMENTS.get(entry.id);
    const shouldBePending =
      maybeGapOwner !== undefined &&
      !(
        (phase127Promoted && maybeGapOwner === 127) ||
        (phase128Stage !== "planned" && maybeGapOwner === 128)
      );
    if (entry.checked === shouldBePending) {
      failures.push(`P124 post-audit checklist state is invalid for ${entry.id}`);
    }
  }
  for (const entry of traceability) {
    const maybeGapOwner = GAP_REQUIREMENTS.get(entry.id);
    const expectedStatus =
      maybeGapOwner === undefined ||
      (phase127Promoted && maybeGapOwner === 127) ||
      (phase128Stage !== "planned" && maybeGapOwner === 128)
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

export function verifyCoverage(
  requirements: string,
  roadmap: string,
  phase127Promoted: boolean,
  phase128Stage: Phase128LifecycleStage,
  failures: string[],
): void {
  const complete =
    phase128Stage === "planned" ? (phase127Promoted ? 33 : 29) : 36;
  const pending = 39 - complete;
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

export function verifyRoadmapTopology(
  repoRoot: string,
  roadmap: string,
  lifecycle: Phase127Lifecycle,
  phase128Stage: Phase128LifecycleStage,
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
    } else if (phase.number === 128) {
      const expected = phase128Stage === "complete" ? "x" : " ";
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
    if (phase.number === 128) {
      const expectedProgress =
        phase128Stage === "planned"
          ? "**Plans:** 0 plans"
          : phase128Stage === "executing_plan_04"
            ? "**Plans:** 3/4 plans executed"
            : "**Plans:** 4/4 plans complete";
      requireContains(
        section,
        expectedProgress,
        "post-audit Phase 128 plan state",
        failures,
      );
    } else if (phase.number !== 127) {
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

export function verifyAudit(audit: string, failures: string[]): void {
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

export function verifyRouting(
  repoRoot: string,
  roadmap: string,
  audit: string,
  phase127Complete: boolean,
  phase128Stage: Phase128LifecycleStage,
  failures: string[],
): void {
  const primaryRoute =
    phase128Stage === "complete"
      ? PHASE129_ROUTE
      : phase127Complete
        ? PHASE128_ROUTE
        : PHASE127_ROUTE;
  const roadmapRoute =
    phase128Stage === "executing_plan_04"
      ? PHASE128_EXECUTION_ROUTE
      : `Run \`${primaryRoute}\`.`;
  requireContains(
    roadmap,
    `## Next Step\n\n${roadmapRoute}`,
    "post-audit roadmap route",
    failures,
  );
  requireContains(
    audit,
    `## Next Action\n\nRun \`${PHASE127_ROUTE}\``,
    "post-audit audit route",
    failures,
  );
  if (phase128Stage === "executing_plan_04") {
    for (const relativePath of [".planning/ROADMAP.md", ".planning/STATE.md"] as const) {
      const text = readFileSync(path.join(repoRoot, relativePath), "utf8");
      requireContains(
        text,
        PHASE128_EXECUTION_ROUTE,
        `post-audit Phase 128 execution route ${relativePath}`,
        failures,
      );
      for (const staleRoute of [PHASE127_ROUTE, PHASE128_ROUTE, PHASE129_ROUTE]) {
        requireAbsent(
          text,
          staleRoute,
          `post-audit Phase 128 execution route ${relativePath}`,
          failures,
        );
      }
    }
    return;
  }
  const routingFiles =
    phase128Stage === "complete"
      ? [".planning/ROADMAP.md", ".planning/STATE.md"] as const
      : ROUTING_FILES;
  for (const relativePath of routingFiles) {
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

export function phase128LifecycleStage(
  roadmap: string,
  failures: string[],
): Phase128LifecycleStage {
  const section = phaseSection(roadmap, 128);
  const stages = [
    ["planned", section.includes("**Plans:** 0 plans")],
    ["executing_plan_04", section.includes("**Plans:** 3/4 plans executed")],
    ["complete", section.includes("**Plans:** 4/4 plans complete")],
  ] as const;
  const matches = stages.filter(([, matchesStage]) => matchesStage);
  if (matches.length !== 1) {
    failures.push("P124 Phase 128 lifecycle must match exactly one supported stage");
    return "planned";
  }
  return matches[0]?.[0] ?? "planned";
}
