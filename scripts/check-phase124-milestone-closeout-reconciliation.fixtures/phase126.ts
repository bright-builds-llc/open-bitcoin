import type {
  Phase125LifecycleStage,
  Phase126CloseoutStage,
} from "../check-phase124-milestone-gap-closure";
import { PHASE123_TEST, PHASE123_CHECK, PHASE124_TEST, PHASE124_CHECK, ACTIVE_TRACEABILITY_TEST, ACTIVE_TRACEABILITY_CHECK, PHASE117_TEST, PHASE117_CHECK, LIFECYCLE_ID, PHASE125_LIFECYCLE_ID, ARCHIVE_ROUTE, PHASE125_ROUTE, PHASE126_ROUTE, PHASE127_ROUTE, PHASE129_ROUTE, PHASE128_EXECUTION_ROUTE, PHASE126_LIFECYCLE_ID, PHASE127_LIFECYCLE_ID, PHASE129_LIFECYCLE_ID, VERIFICATION_FILE, SUMMARY_FILE, CONTEXT_FILE, PLAN_01_FILE, PLAN_02_FILE, SUMMARY_01_FILE, PHASE125_CONTEXT_FILE, PHASE125_VERIFICATION_FILE, PHASE126_CONTEXT_FILE, PHASE126_VERIFICATION_FILE, PHASE125_DIRECTORY, PHASE126_DIRECTORY, POST_AUDIT_PHASE_DIRECTORIES, REQUIRED_FILES, REQUIREMENT_IDS, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, RESOLVED_DEBT_IDS, PHASE127_DIRECTORY, PHASE129_DIRECTORY, PHASE129_VERIFICATION_FILE, PHASE129_REQUIREMENT_IDS, PHASE129_VERIFIED_REQUIREMENT_IDS } from "./base.ts";
import type { RequiredFile, Phase125PlanNumber, Phase126PlanNumber, Phase127PlanNumber, Phase128FixtureStage, Phase129FixtureStage, Phase129PlanNumber, FixtureFile, FixtureOptions } from "./base.ts";
import { createFixture } from "./base.ts";
import { lifecycleArtifact, addPhase125Artifacts, phase125Artifact, createRequirements, createGapClosureRequirements, createRoadmap, createGapClosureRoadmap, createState, createGapClosureRouting, createAudit, createGapClosureAudit, createVerifyScript, phase125SummaryCount, phase125VerificationPresent, phase125Promoted, phase125GapPhase, phase125PlanNumbers, phaseFor, range } from "./phase125.ts";
import { createPostAuditGapPlanningRequirements, createPostAuditGapPlanningRoadmap, createPostAuditGapPlanningAudit, addPhase127Artifacts, phase127Artifact, postAuditGapOwners } from "./phase127.ts";
import { createPhase128Requirements, createPhase128Roadmap, createPhase128State } from "./phase128.ts";
import { createArchiveReadyRequirements, createArchiveReadyRoadmap, createArchiveReadyAudit, addPhase129Artifacts, phase129Artifact } from "./phase129.ts";
import { replace, append } from "./mutations.ts";

export function createPhase126Requirements(stage: Phase126CloseoutStage["kind"]): string {
  const promoted = phase126Promoted(stage);
  const completeCount = promoted ? 39 : 33;
  const pending = new Set<string>(promoted ? [] : PHASE126_REQUIREMENTS);
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

export function createPhase126Roadmap(stage: Phase126CloseoutStage["kind"]): string {
  const archiveReady = stage === "archive_ready";
  const promoted = phase126Promoted(stage);
  return [
    "- [x] **Phase 124: Milestone Closeout Reconciliation**",
    "- [x] **Phase 125: Compact Download Verification Traceability Closure**",
    `- [${archiveReady ? "x" : " "}] **Phase 126: Compact Relay Residual Hardening**`,
    "#### Phase 124: Milestone Closeout Reconciliation",
    "**Plans:** 2/2 plans complete",
    "#### Phase 125: Compact Download Verification Traceability Closure",
    "**Depends on:** Phase 124",
    "**Requirements:** RCN-04, RCN-05, RCN-06",
    "**Plans:** 4/4 plans complete",
    "#### Phase 126: Compact Relay Residual Hardening",
    "**Depends on:** Phase 125",
    "**Requirements:** CMP-05, RCN-02, RCN-03, GOV-04, BOUND-01, HARD-05",
    `**Plans:** ${archiveReady ? "4/4 plans complete" : `${phase126SummaryCount(stage)}/4 plans executed`}`,
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    `- Satisfied: ${promoted ? 39 : 33}`,
    `- Pending hardening and closeout: ${promoted ? 0 : 6}`,
    "- Unmapped: 0",
    "## Next Step",
    createPhase126Routing(stage),
  ].join("\n");
}

export function createPhase126Routing(stage: Phase126CloseoutStage["kind"]): string {
  return stage === "archive_ready" ? ARCHIVE_ROUTE : PHASE126_ROUTE;
}

export function createPhase126State(stage: Phase126CloseoutStage["kind"]): string {
  if (stage === "candidate") {
    return "Phase 126 candidate execution is in progress.";
  }
  return createPhase126Routing(stage);
}

export function createPhase126Audit(stage: Phase126CloseoutStage["kind"]): string {
  const promoted = phase126Promoted(stage);
  return [
    "---",
    `status: ${promoted ? "passed" : "gaps_found"}`,
    "scores:",
    `  requirements: "${promoted ? 39 : 33}/39"`,
    `  phases: "${promoted ? 17 : 16}/17"`,
    "gaps:",
    ...(promoted
      ? ["  requirements: []"]
      : [
          "  requirements:",
          ...PHASE126_REQUIREMENTS.map((id) => `    - id: ${id}`),
        ]),
    "  integration: []",
    "  flows: []",
    ...(promoted ? ["tech_debt: []"] : []),
    "---",
    "## Next Action",
    createPhase126Routing(stage),
  ].join("\n");
}

export function addPhase126Artifacts(
  files: Map<FixtureFile, string>,
  stage: Phase126CloseoutStage["kind"],
): void {
  files.set(
    PHASE126_CONTEXT_FILE,
    phase126Artifact(["generated_by: gsd-discuss-phase"]),
  );
  for (const planNumber of phase125PlanNumbers()) {
    files.set(
      `${PHASE126_DIRECTORY}/126-${planNumber}-PLAN.md`,
      phase126Artifact([
        "phase: 126-compact-relay-residual-hardening",
        `plan: "${planNumber}"`,
        "generated_by: gsd-plan-phase",
      ]),
    );
  }
  for (const planNumber of phase125PlanNumbers().slice(0, phase126SummaryCount(stage))) {
    files.set(
      `${PHASE126_DIRECTORY}/126-${planNumber}-SUMMARY.md`,
      phase126Artifact([
        "phase: 126-compact-relay-residual-hardening",
        `plan: "${planNumber}"`,
        "requirements-completed: []",
        "generated_by: gsd-execute-plan",
      ]),
    );
  }
  if (stage !== "candidate") {
    files.set(
      PHASE126_VERIFICATION_FILE,
      phase126Artifact([
        "phase: 126-compact-relay-residual-hardening",
        "status: passed",
        "lifecycle_validated: true",
        "generated_by: gsd-verifier",
      ]),
    );
  }
}

export function phase126Artifact(fields: readonly string[]): string {
  return [
    "---",
    ...fields,
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${PHASE126_LIFECYCLE_ID}`,
    'generated_at: "2026-07-18T20:00:00Z"',
    "---",
    "fixture artifact",
  ].join("\n");
}

export function phase126SummaryCount(stage: Phase126CloseoutStage["kind"]): number {
  if (stage === "candidate") return 1;
  if (stage === "archive_ready") return 4;
  return 3;
}

export function phase126Promoted(stage: Phase126CloseoutStage["kind"]): boolean {
  return stage === "promoted_pre_summary" || stage === "archive_ready";
}
