import type {
  Phase125LifecycleStage,
  Phase126CloseoutStage,
} from "../check-phase124-milestone-gap-closure";
import { PHASE123_TEST, PHASE123_CHECK, PHASE124_TEST, PHASE124_CHECK, ACTIVE_TRACEABILITY_TEST, ACTIVE_TRACEABILITY_CHECK, PHASE117_TEST, PHASE117_CHECK, LIFECYCLE_ID, PHASE125_LIFECYCLE_ID, ARCHIVE_ROUTE, PHASE125_ROUTE, PHASE126_ROUTE, PHASE127_ROUTE, PHASE129_ROUTE, PHASE128_EXECUTION_ROUTE, PHASE126_LIFECYCLE_ID, PHASE127_LIFECYCLE_ID, PHASE129_LIFECYCLE_ID, VERIFICATION_FILE, SUMMARY_FILE, CONTEXT_FILE, PLAN_01_FILE, PLAN_02_FILE, SUMMARY_01_FILE, PHASE125_CONTEXT_FILE, PHASE125_VERIFICATION_FILE, PHASE126_CONTEXT_FILE, PHASE126_VERIFICATION_FILE, PHASE125_DIRECTORY, PHASE126_DIRECTORY, POST_AUDIT_PHASE_DIRECTORIES, REQUIRED_FILES, REQUIREMENT_IDS, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, RESOLVED_DEBT_IDS, PHASE127_DIRECTORY, PHASE129_DIRECTORY, PHASE129_VERIFICATION_FILE, PHASE129_REQUIREMENT_IDS, PHASE129_VERIFIED_REQUIREMENT_IDS } from "./base.ts";
import type { RequiredFile, Phase125PlanNumber, Phase126PlanNumber, Phase127PlanNumber, Phase128FixtureStage, Phase129FixtureStage, Phase129PlanNumber, FixtureFile, FixtureOptions } from "./base.ts";
import { createFixture } from "./base.ts";
import { lifecycleArtifact, addPhase125Artifacts, phase125Artifact, createRequirements, createGapClosureRequirements, createRoadmap, createGapClosureRoadmap, createState, createGapClosureRouting, createAudit, createGapClosureAudit, createVerifyScript, phase125SummaryCount, phase125VerificationPresent, phase125Promoted, phase125GapPhase, phase125PlanNumbers, phaseFor, range } from "./phase125.ts";
import { createPhase126Requirements, createPhase126Roadmap, createPhase126Routing, createPhase126State, createPhase126Audit, addPhase126Artifacts, phase126Artifact, phase126SummaryCount, phase126Promoted } from "./phase126.ts";
import { createPhase128Requirements, createPhase128Roadmap, createPhase128State } from "./phase128.ts";
import { createArchiveReadyRequirements, createArchiveReadyRoadmap, createArchiveReadyAudit, addPhase129Artifacts, phase129Artifact } from "./phase129.ts";
import { replace, append } from "./mutations.ts";

export function createPostAuditGapPlanningRequirements(): string {
  const gapOwners = postAuditGapOwners();
  return [
    ...REQUIREMENT_IDS.map(
      (id) => `- [${gapOwners.has(id) ? " " : "x"}] **${id}**: fixture requirement`,
    ),
    ...REQUIREMENT_IDS.map((id) => {
      const maybeOwner = gapOwners.get(id);
      return `| ${id} | Phase ${maybeOwner ?? phase125GapPhase(id) ?? phaseFor(id)} | ${maybeOwner === undefined ? "Complete" : "Pending"} |`;
    }),
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    "- Complete: 29",
    "- Pending integration gap closure: 10",
    "- Unmapped: 0",
  ].join("\n");
}

export function createPostAuditGapPlanningRoadmap(): string {
  return [
    "- [x] **Phase 124: Milestone Closeout Reconciliation**",
    "- [x] **Phase 125: Compact Download Verification Traceability Closure**",
    "- [x] **Phase 126: Compact Relay Residual Hardening**",
    "- [ ] **Phase 127: Authoritative Network State Unification**",
    "- [ ] **Phase 128: Production Compact Announcement Transport**",
    "- [ ] **Phase 129: Integration Guardrails and Milestone Reconciliation**",
    "#### Phase 124: Milestone Closeout Reconciliation",
    "**Plans:** 2/2 plans complete",
    "#### Phase 125: Compact Download Verification Traceability Closure",
    "**Plans:** 4/4 plans complete",
    "#### Phase 126: Compact Relay Residual Hardening",
    "**Plans:** 4/4 plans complete",
    "#### Phase 127: Authoritative Network State Unification",
    "**Depends on:** Phase 126",
    "**Requirements:** BSRV-03, BSRV-04, OBS-02, OBS-04",
    "**Plans:** 0 plans",
    "#### Phase 128: Production Compact Announcement Transport",
    "**Depends on:** Phase 127",
    "**Requirements:** CMP-04, CMP-05, OBS-03",
    "**Plans:** 0 plans",
    "#### Phase 129: Integration Guardrails and Milestone Reconciliation",
    "**Depends on:** Phase 128",
    "**Requirements:** OBS-01, BOUND-02, HARD-05",
    "**Plans:** 0 plans",
    "**Execution Order:** 110 -> 126 -> 127 -> 128 -> 129",
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    "- Satisfied: 29",
    "- Pending integration gap closure: 10",
    "- Unmapped: 0",
    "## Next Step",
    "",
    `Run \`${PHASE127_ROUTE}\`.`,
  ].join("\n");
}

export function createPostAuditGapPlanningAudit(): string {
  return [
    "---",
    "status: gaps_found",
    "scores:",
    '  requirements: "29/39"',
    '  phases: "17/17"',
    '  integration: "9/13"',
    '  flows: "7/11"',
    "gaps:",
    "  requirements:",
    ...Array.from(postAuditGapOwners().keys(), (id) => `    - id: ${id}`),
    "  integration:",
    "    - id: GAP-01",
    "    - id: GAP-02",
    "    - id: GAP-03",
    "  flows:",
    "    - id: FLOW-01",
    "    - id: FLOW-02",
    "    - id: FLOW-03",
    "    - id: FLOW-04",
    "---",
    "## Next Action",
    "",
    `Run \`${PHASE127_ROUTE}\` to begin gap closure.`,
  ].join("\n");
}

export function addPhase127Artifacts(files: Map<FixtureFile, string>): void {
  files.set(
    `${PHASE127_DIRECTORY}/127-CONTEXT.md`,
    phase127Artifact(["generated_by: gsd-discuss-phase"]),
  );
  for (const planNumber of phase125PlanNumbers()) {
    files.set(
      `${PHASE127_DIRECTORY}/127-${planNumber}-PLAN.md`,
      phase127Artifact([
        "phase: 127-authoritative-network-state-unification",
        `plan: "${planNumber}"`,
        "generated_by: gsd-plan-phase",
      ]),
    );
    files.set(
      `${PHASE127_DIRECTORY}/127-${planNumber}-SUMMARY.md`,
      phase127Artifact([
        "phase: 127-authoritative-network-state-unification",
        `plan: "${planNumber}"`,
        "requirements-completed: []",
        "generated_by: gsd-execute-plan",
      ]),
    );
  }
  files.set(
    `${PHASE127_DIRECTORY}/127-VERIFICATION.md`,
    phase127Artifact([
      "phase: 127-authoritative-network-state-unification",
      "status: passed",
      "lifecycle_validated: true",
      "generated_by: gsd-verifier",
    ]),
  );
}

export function phase127Artifact(fields: readonly string[]): string {
  return [
    "---",
    ...fields,
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${PHASE127_LIFECYCLE_ID}`,
    'generated_at: "2026-07-19T20:00:00Z"',
    "---",
    "fixture artifact",
  ].join("\n");
}

export function postAuditGapOwners(): Map<string, number> {
  return new Map([
    ["BSRV-03", 127],
    ["BSRV-04", 127],
    ["OBS-02", 127],
    ["OBS-04", 127],
    ["CMP-04", 128],
    ["CMP-05", 128],
    ["OBS-03", 128],
    ["OBS-01", 129],
    ["BOUND-02", 129],
    ["HARD-05", 129],
  ]);
}
