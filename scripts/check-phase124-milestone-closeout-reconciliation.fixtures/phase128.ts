import type {
  Phase125LifecycleStage,
  Phase126CloseoutStage,
} from "../check-phase124-milestone-gap-closure";
import { PHASE123_TEST, PHASE123_CHECK, PHASE124_TEST, PHASE124_CHECK, ACTIVE_TRACEABILITY_TEST, ACTIVE_TRACEABILITY_CHECK, PHASE117_TEST, PHASE117_CHECK, LIFECYCLE_ID, PHASE125_LIFECYCLE_ID, ARCHIVE_ROUTE, PHASE125_ROUTE, PHASE126_ROUTE, PHASE127_ROUTE, PHASE129_ROUTE, PHASE128_EXECUTION_ROUTE, PHASE126_LIFECYCLE_ID, PHASE127_LIFECYCLE_ID, PHASE129_LIFECYCLE_ID, VERIFICATION_FILE, SUMMARY_FILE, CONTEXT_FILE, PLAN_01_FILE, PLAN_02_FILE, SUMMARY_01_FILE, PHASE125_CONTEXT_FILE, PHASE125_VERIFICATION_FILE, PHASE126_CONTEXT_FILE, PHASE126_VERIFICATION_FILE, PHASE125_DIRECTORY, PHASE126_DIRECTORY, POST_AUDIT_PHASE_DIRECTORIES, REQUIRED_FILES, REQUIREMENT_IDS, PHASE125_REQUIREMENTS, PHASE126_REQUIREMENTS, RESOLVED_DEBT_IDS, PHASE127_DIRECTORY, PHASE129_DIRECTORY, PHASE129_VERIFICATION_FILE, PHASE129_REQUIREMENT_IDS, PHASE129_VERIFIED_REQUIREMENT_IDS } from "./base.ts";
import type { RequiredFile, Phase125PlanNumber, Phase126PlanNumber, Phase127PlanNumber, Phase128FixtureStage, Phase129FixtureStage, Phase129PlanNumber, FixtureFile, FixtureOptions } from "./base.ts";
import { createFixture } from "./base.ts";
import { lifecycleArtifact, addPhase125Artifacts, phase125Artifact, createRequirements, createGapClosureRequirements, createRoadmap, createGapClosureRoadmap, createState, createGapClosureRouting, createAudit, createGapClosureAudit, createVerifyScript, phase125SummaryCount, phase125VerificationPresent, phase125Promoted, phase125GapPhase, phase125PlanNumbers, phaseFor, range } from "./phase125.ts";
import { createPhase126Requirements, createPhase126Roadmap, createPhase126Routing, createPhase126State, createPhase126Audit, addPhase126Artifacts, phase126Artifact, phase126SummaryCount, phase126Promoted } from "./phase126.ts";
import { createPostAuditGapPlanningRequirements, createPostAuditGapPlanningRoadmap, createPostAuditGapPlanningAudit, addPhase127Artifacts, phase127Artifact, postAuditGapOwners } from "./phase127.ts";
import { createArchiveReadyRequirements, createArchiveReadyRoadmap, createArchiveReadyAudit, addPhase129Artifacts, phase129Artifact } from "./phase129.ts";
import { replace, append } from "./mutations.ts";

export function createPhase128Requirements(): string {
  let requirements = createPostAuditGapPlanningRequirements();
  for (const requirement of [
    "BSRV-03",
    "BSRV-04",
    "OBS-02",
    "OBS-04",
    "CMP-04",
    "CMP-05",
    "OBS-03",
  ]) {
    requirements = requirements
      .replace(`- [ ] **${requirement}**`, `- [x] **${requirement}**`)
      .replace(
        new RegExp(`\\| ${requirement} \\| Phase (127|128) \\| Pending \\|`),
        (row) => row.replace("Pending", "Complete"),
      );
  }
  return requirements
    .replace("- Complete: 29", "- Complete: 36")
    .replace(
      "- Pending integration gap closure: 10",
      "- Pending integration gap closure: 3",
    );
}

export function createPhase128Roadmap(stage: Phase128FixtureStage): string {
  const complete = stage === "complete";
  return [
    "- [x] **Phase 124: Milestone Closeout Reconciliation**",
    "- [x] **Phase 125: Compact Download Verification Traceability Closure**",
    "- [x] **Phase 126: Compact Relay Residual Hardening**",
    "- [x] **Phase 127: Authoritative Network State Unification**",
    `- [${complete ? "x" : " "}] **Phase 128: Production Compact Announcement Transport**`,
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
    "**Plans:** 4/4 plans complete",
    "#### Phase 128: Production Compact Announcement Transport",
    "**Depends on:** Phase 127",
    "**Requirements:** CMP-04, CMP-05, OBS-03",
    `**Plans:** ${complete ? "4/4 plans complete" : "3/4 plans executed"}`,
    "#### Phase 129: Integration Guardrails and Milestone Reconciliation",
    "**Depends on:** Phase 128",
    "**Requirements:** OBS-01, BOUND-02, HARD-05",
    "**Plans:** 0 plans",
    "**Execution Order:** 110 -> 126 -> 127 -> 128 -> 129",
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    "- Satisfied: 36",
    "- Pending integration gap closure: 3",
    "- Unmapped: 0",
    "## Next Step",
    "",
    complete ? `Run \`${PHASE129_ROUTE}\`.` : PHASE128_EXECUTION_ROUTE,
  ].join("\n");
}

export function createPhase128State(stage: Phase128FixtureStage): string {
  return stage === "complete"
    ? `Phase: 129\nNext action: Run \`${PHASE129_ROUTE}\`.`
    : `Phase: 128\nPlan: 4 of 4\nStatus: In progress\nNext action: ${PHASE128_EXECUTION_ROUTE}`;
}
