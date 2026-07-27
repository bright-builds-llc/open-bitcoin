import type {
  Phase125LifecycleStage,
  Phase126CloseoutStage,
} from "../check-phase124-milestone-gap-closure";
import { PHASE123_TEST, PHASE123_CHECK, PHASE124_TEST, PHASE124_CHECK, ACTIVE_TRACEABILITY_TEST, ACTIVE_TRACEABILITY_CHECK, PHASE117_TEST, PHASE117_CHECK, LIFECYCLE_ID, PHASE125_LIFECYCLE_ID, ARCHIVE_ROUTE, PHASE125_ROUTE, PHASE126_ROUTE, PHASE127_ROUTE, PHASE129_ROUTE, PHASE128_EXECUTION_ROUTE, PHASE126_LIFECYCLE_ID, PHASE127_LIFECYCLE_ID, PHASE129_LIFECYCLE_ID, VERIFICATION_FILE, SUMMARY_FILE, CONTEXT_FILE, PLAN_01_FILE, PLAN_02_FILE, SUMMARY_01_FILE, PHASE125_CONTEXT_FILE, PHASE125_VERIFICATION_FILE, PHASE126_CONTEXT_FILE, PHASE126_VERIFICATION_FILE, PHASE125_DIRECTORY, PHASE126_DIRECTORY, POST_AUDIT_PHASE_DIRECTORIES, REQUIRED_FILES, REQUIREMENT_IDS, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, RESOLVED_DEBT_IDS, PHASE127_DIRECTORY, PHASE129_DIRECTORY, PHASE129_VERIFICATION_FILE, PHASE129_REQUIREMENT_IDS, PHASE129_VERIFIED_REQUIREMENT_IDS } from "./base.ts";
import type { RequiredFile, Phase125PlanNumber, Phase126PlanNumber, Phase127PlanNumber, Phase128FixtureStage, Phase129FixtureStage, Phase129PlanNumber, FixtureFile, FixtureOptions } from "./base.ts";
import { createFixture } from "./base.ts";
import { createPhase126Requirements, createPhase126Roadmap, createPhase126Routing, createPhase126State, createPhase126Audit, addPhase126Artifacts, phase126Artifact, phase126SummaryCount, phase126Promoted } from "./phase126.ts";
import { createPostAuditGapPlanningRequirements, createPostAuditGapPlanningRoadmap, createPostAuditGapPlanningAudit, addPhase127Artifacts, phase127Artifact, postAuditGapOwners } from "./phase127.ts";
import { createPhase128Requirements, createPhase128Roadmap, createPhase128State } from "./phase128.ts";
import { createArchiveReadyRequirements, createArchiveReadyRoadmap, createArchiveReadyAudit, addPhase129Artifacts, phase129Artifact } from "./phase129.ts";
import { replace, append } from "./mutations.ts";

export function lifecycleArtifact(generatedBy: string, generatedAt: string): string {
  return [
    "---",
    `generated_by: ${generatedBy}`,
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${LIFECYCLE_ID}`,
    `generated_at: "${generatedAt}"`,
    "---",
    "fixture artifact",
  ].join("\n");
}

export function addPhase125Artifacts(
  files: Map<FixtureFile, string>,
  stage: Phase125LifecycleStage["kind"],
): void {
  files.set(
    PHASE125_CONTEXT_FILE,
    phase125Artifact(["generated_by: gsd-discuss-phase"]),
  );
  for (const planNumber of phase125PlanNumbers()) {
    files.set(
      `${PHASE125_DIRECTORY}/125-${planNumber}-PLAN.md`,
      phase125Artifact([
        "phase: 125-compact-download-verification-traceability-closure",
        `plan: "${planNumber}"`,
        "generated_by: gsd-plan-phase",
      ]),
    );
  }
  const summaryCount = phase125SummaryCount(stage);
  for (const planNumber of phase125PlanNumbers().slice(0, summaryCount)) {
    files.set(
      `${PHASE125_DIRECTORY}/125-${planNumber}-SUMMARY.md`,
      phase125Artifact([
        "phase: 125-compact-download-verification-traceability-closure",
        `plan: "${planNumber}"`,
        "requirements-completed: []",
        "generated_by: gsd-execute-plan",
      ]),
    );
  }
  if (phase125VerificationPresent(stage)) {
    files.set(
      PHASE125_VERIFICATION_FILE,
      phase125Artifact([
        "phase: 125-compact-download-verification-traceability-closure",
        "status: passed",
        "lifecycle_validated: true",
        "generated_by: gsd-verifier",
      ]),
    );
  }
}

export function phase125Artifact(fields: readonly string[]): string {
  return [
    "---",
    ...fields,
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${PHASE125_LIFECYCLE_ID}`,
    'generated_at: "2026-07-17T15:00:00Z"',
    "---",
    "fixture artifact",
  ].join("\n");
}

export function createRequirements(finalStage: boolean): string {
  const completeCount = finalStage ? 39 : 38;
  const pendingCount = finalStage ? 0 : 1;
  const checklist = REQUIREMENT_IDS.map((id) => {
    const checked = id !== "HARD-05" || finalStage;
    return `- [${checked ? "x" : " "}] **${id}**: fixture requirement`;
  });
  const traceability = REQUIREMENT_IDS.map((id) => {
    const status = id === "HARD-05" && !finalStage ? "Pending" : "Complete";
    return `| ${id} | Phase ${phaseFor(id)} | ${status} |`;
  });
  return [
    ...checklist,
    ...traceability,
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    `- Complete: ${completeCount}`,
    `- Pending hardening and closeout: ${pendingCount}`,
    "- Unmapped: 0",
  ].join("\n");
}

export function createGapClosureRequirements(stage: Phase125LifecycleStage["kind"]): string {
  const promoted = phase125Promoted(stage);
  const pending = new Set<string>(PHASE126_REQUIREMENTS);
  if (!promoted) {
    for (const requirement of PHASE125_REQUIREMENTS) {
      pending.add(requirement);
    }
  }
  const completeCount = promoted ? 33 : 30;
  return [
    ...REQUIREMENT_IDS.map(
      (id) => `- [${pending.has(id) ? " " : "x"}] **${id}**: fixture requirement`,
    ),
    ...REQUIREMENT_IDS.map((id) => {
      const maybePhase = phase125GapPhase(id);
      return `| ${id} | Phase ${maybePhase ?? phaseFor(id)} | ${pending.has(id) ? "Pending" : "Complete"} |`;
    }),
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    `- Complete: ${completeCount}`,
    `- Pending hardening and closeout: ${39 - completeCount}`,
    "- Unmapped: 0",
  ].join("\n");
}

export function createRoadmap(phaseComplete: boolean): string {
  const completeCount = phaseComplete ? 39 : 38;
  const pendingCount = phaseComplete ? 0 : 1;
  const phase124State = phaseComplete ? "x" : " ";
  const phase124Plans = phaseComplete ? "2/2 plans complete" : "1/2 plans executed";
  const maybeRoute = phaseComplete ? `\n## Next Step\n${ARCHIVE_ROUTE}` : "";
  return [
    `- [${phase124State}] **Phase 124: Milestone Closeout Reconciliation**`,
    "#### Phase 122: Compact Relay Peer Completion",
    "**Plans:** 1/1 plans complete",
    "#### Phase 123: Runtime Timing and Evidence Integrity",
    "**Plans:** 7/7 plans complete",
    "#### Phase 124: Milestone Closeout Reconciliation",
    `**Plans:** ${phase124Plans}`,
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    `- Satisfied: ${completeCount}`,
    `- Pending hardening and closeout: ${pendingCount}`,
    "- Unmapped: 0",
    maybeRoute,
  ].join("\n");
}

export function createGapClosureRoadmap(stage: Phase125LifecycleStage["kind"]): string {
  const promoted = phase125Promoted(stage);
  const phase125Complete = stage === "post_summary";
  const summaryCount = phase125SummaryCount(stage);
  const plans =
    stage === "planned"
      ? "4 plans"
      : phase125Complete
        ? "4/4 plans complete"
        : `${summaryCount}/4 plans executed`;
  const completeCount = promoted ? 33 : 30;
  return [
    "- [x] **Phase 124: Milestone Closeout Reconciliation**",
    `- [${phase125Complete ? "x" : " "}] **Phase 125: Compact Download Verification Traceability Closure**`,
    "- [ ] **Phase 126: Compact Relay Residual Hardening**",
    "#### Phase 122: Compact Relay Peer Completion",
    "**Plans:** 1/1 plans complete",
    "#### Phase 123: Runtime Timing and Evidence Integrity",
    "**Plans:** 7/7 plans complete",
    "#### Phase 124: Milestone Closeout Reconciliation",
    "**Plans:** 2/2 plans complete",
    "#### Phase 125: Compact Download Verification Traceability Closure",
    "**Depends on:** Phase 124",
    "**Requirements:** RCN-04, RCN-05, RCN-06",
    `**Plans:** ${plans}`,
    "#### Phase 126: Compact Relay Residual Hardening",
    "**Depends on:** Phase 125",
    "**Requirements:** CMP-05, RCN-02, RCN-03, GOV-04, BOUND-01, HARD-05",
    "**Plans:** 0 plans",
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    `- Satisfied: ${completeCount}`,
    `- Pending hardening and closeout: ${39 - completeCount}`,
    "- Unmapped: 0",
    "## Next Step",
    createGapClosureRouting(stage),
  ].join("\n");
}

export function createState(finalStage: boolean): string {
  if (finalStage) return `Phase 124 verified. Next action: ${ARCHIVE_ROUTE}`;
  return "Phase 124 evidence reconciled; HARD-05 pending";
}

export function createGapClosureRouting(stage: Phase125LifecycleStage["kind"]): string {
  return phase125Promoted(stage) ? PHASE126_ROUTE : PHASE125_ROUTE;
}

export function createAudit(finalStage: boolean): string {
  if (!finalStage) {
    return [
      "---",
      "status: tech_debt",
      "scores:",
      '  requirements: "34/34"',
      '  phases: "12/12"',
      "---",
      "Phase 124 closeout verification pending; do not archive.",
    ].join("\n");
  }
  return [
    "---",
    "status: passed",
    "scores:",
    '  requirements: "39/39"',
    '  phases: "15/15"',
    "gaps:",
    "  requirements: []",
    "  integration: []",
    "  flows: []",
    "tech_debt: []",
    "---",
    "## Resolved Hardening Debt",
    ...RESOLVED_DEBT_IDS.map((id) => `- ${id}: resolved with current evidence.`),
    `## Next Step\n${ARCHIVE_ROUTE}`,
  ].join("\n");
}

export function createGapClosureAudit(stage: Phase125LifecycleStage["kind"]): string {
  const promoted = phase125Promoted(stage);
  return [
    "---",
    "status: gaps_found",
    "scores:",
    `  requirements: "${promoted ? 33 : 30}/39"`,
    `  phases: "${promoted ? 16 : 15}/17"`,
    "gaps:",
    "  requirements:",
    ...(promoted ? [] : PHASE125_REQUIREMENTS.map((id) => `    - id: ${id}`)),
    "  integration: []",
    "  flows: []",
    "---",
    "## Next Action",
    createGapClosureRouting(stage),
  ].join("\n");
}

export function createVerifyScript(
  maybePhase125Stage?: Phase125LifecycleStage["kind"],
): string {
  const maybeActiveTraceabilityCommands =
    maybePhase125Stage !== undefined &&
    phase125VerificationPresent(maybePhase125Stage)
      ? [ACTIVE_TRACEABILITY_TEST, ACTIVE_TRACEABILITY_CHECK]
      : [];
  const commands = [
    PHASE123_TEST,
    PHASE123_CHECK,
    PHASE124_TEST,
    PHASE124_CHECK,
    ...maybeActiveTraceabilityCommands,
    PHASE117_TEST,
    PHASE117_CHECK,
  ];
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    ...commands,
    "VERIFY_COMMAND_ORDER",
    `run_step "test Phase 123" ${PHASE123_TEST}`,
    `run_step "check Phase 123" ${PHASE123_CHECK}`,
    `run_step "test Phase 124" ${PHASE124_TEST}`,
    `run_step "check Phase 124" ${PHASE124_CHECK}`,
    ...(maybeActiveTraceabilityCommands.length === 0
      ? []
      : [
          `run_step "test active traceability" ${ACTIVE_TRACEABILITY_TEST}`,
          `run_step "check active traceability" ${ACTIVE_TRACEABILITY_CHECK}`,
        ]),
    `run_step "test Phase 117" ${PHASE117_TEST}`,
    `run_step "check Phase 117" ${PHASE117_CHECK}`,
  ].join("\n");
}

export function phase125SummaryCount(stage: Phase125LifecycleStage["kind"]): number {
  if (stage === "planned") return 0;
  if (stage === "pre_verification") return 2;
  if (stage === "post_summary") return 4;
  return 3;
}

export function phase125VerificationPresent(stage: Phase125LifecycleStage["kind"]): boolean {
  return !["planned", "pre_verification"].includes(stage);
}

export function phase125Promoted(stage: Phase125LifecycleStage["kind"]): boolean {
  return stage === "post_verification" || stage === "post_summary";
}

export function phase125GapPhase(id: string): number | undefined {
  if (PHASE125_REQUIREMENTS.includes(id as (typeof PHASE125_REQUIREMENTS)[number])) {
    return 125;
  }
  if (PHASE126_REQUIREMENTS.includes(id as (typeof PHASE126_REQUIREMENTS)[number])) {
    return 126;
  }
  return undefined;
}

export function phase125PlanNumbers(): Phase125PlanNumber[] {
  return ["01", "02", "03", "04"];
}

export function phaseFor(id: string): number {
  if (id === "HARD-05") return 124;
  if (id === "HARD-01") return 122;
  if (id.startsWith("HARD-")) return 123;
  if (id.startsWith("BOUND-")) return 117;
  if (id.startsWith("OBS-")) return id === "OBS-03" ? 121 : 116;
  if (id.startsWith("GOV-")) return 111;
  if (id.startsWith("RCN-")) return 115;
  if (id.startsWith("CMP-")) return 113;
  return 110;
}

export function range(prefix: string, count: number): string[] {
  return Array.from({ length: count }, (_, index) =>
    `${prefix}-${String(index + 1).padStart(2, "0")}`,
  );
}
