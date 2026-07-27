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
import { createPhase128Requirements, createPhase128Roadmap, createPhase128State } from "./phase128.ts";
import { replace, append } from "./mutations.ts";

export function createArchiveReadyRequirements(): string {
  let requirements = createPhase128Requirements();
  for (const requirement of PHASE129_REQUIREMENT_IDS) {
    requirements = requirements
      .replace(`- [ ] **${requirement}**`, `- [x] **${requirement}**`)
      .replace(
        `| ${requirement} | Phase 129 | Pending |`,
        `| ${requirement} | Phase 129 | Complete |`,
      );
  }
  return requirements
    .replace("- Complete: 36", "- Complete: 39")
    .replace(
      "- Pending integration gap closure: 3",
      "- Pending integration gap closure: 0",
    );
}

export function createArchiveReadyRoadmap(): string {
  return createPhase128Roadmap("complete")
    .replace(
      "- [ ] **Phase 129: Integration Guardrails and Milestone Reconciliation**",
      "- [x] **Phase 129: Integration Guardrails and Milestone Reconciliation**",
    )
    .replace(
      "**Requirements:** OBS-01, BOUND-02, HARD-05\n**Plans:** 0 plans",
      "**Requirements:** OBS-01, BOUND-02, HARD-05\n**Plans:** 4/4 plans complete",
    )
    .replace("- Satisfied: 36", "- Satisfied: 39")
    .replace(
      "- Pending integration gap closure: 3",
      "- Pending integration gap closure: 0",
    )
    .replace(`Run \`${PHASE129_ROUTE}\`.`, `Run \`${ARCHIVE_ROUTE}\`.`);
}

export function createArchiveReadyAudit(): string {
  return [
    "---",
    "status: passed",
    "scores:",
    '  requirements: "39/39"',
    '  phases: "20/20"',
    '  integration: "13/13"',
    '  flows: "11/11"',
    "gaps:",
    "  requirements: []",
    "  integration: []",
    "  flows: []",
    "tech_debt:",
    "  - phase: 124-milestone-closeout-reconciliation",
    "    items:",
    '      - "scripts/check-phase124-milestone-gap-closure.ts exceeds 1,500 lines and concentrates unrelated lifecycle assertions."',
    "---",
    "## Next Action",
    "",
    `Run \`${ARCHIVE_ROUTE}\` to archive the reconciled milestone.`,
  ].join("\n");
}

export function addPhase129Artifacts(
  files: Map<FixtureFile, string>,
  stage: Phase129FixtureStage,
): void {
  if (stage === "gaps_open") return;
  files.set(
    `${PHASE129_DIRECTORY}/129-CONTEXT.md`,
    phase129Artifact(["generated_by: gsd-discuss-phase"]),
  );
  for (const planNumber of phase125PlanNumbers()) {
    files.set(
      `${PHASE129_DIRECTORY}/129-${planNumber}-PLAN.md`,
      phase129Artifact([
        "phase: 129-integration-guardrails-and-milestone-reconciliation",
        `plan: "${planNumber}"`,
        "generated_by: gsd-plan-phase",
      ]),
    );
  }
  const summaryCount = stage === "archive_ready" ? 4 : 3;
  for (const planNumber of phase125PlanNumbers().slice(0, summaryCount)) {
    const requirementsCompleted =
      stage === "archive_ready" && planNumber === "04"
        ? `requirements-completed: [${PHASE129_REQUIREMENT_IDS.join(", ")}]`
        : "requirements-completed: []";
    files.set(
      `${PHASE129_DIRECTORY}/129-${planNumber}-SUMMARY.md`,
      phase129Artifact([
        "phase: 129-integration-guardrails-and-milestone-reconciliation",
        `plan: "${planNumber}"`,
        requirementsCompleted,
        "generated_by: gsd-execute-plan",
      ]),
    );
  }
  files.set(
    PHASE129_VERIFICATION_FILE,
    phase129Artifact(
      [
        "phase: 129-integration-guardrails-and-milestone-reconciliation",
        "status: passed",
        "lifecycle_validated: true",
        "generated_by: gsd-verifier",
      ],
      `Verified requirements: ${PHASE129_VERIFIED_REQUIREMENT_IDS.join(", ")}.`,
    ),
  );
}

export function phase129Artifact(
  fields: readonly string[],
  maybeBody?: string,
): string {
  return [
    "---",
    ...fields,
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${PHASE129_LIFECYCLE_ID}`,
    'generated_at: "2026-07-20T23:00:00Z"',
    "---",
    maybeBody ?? "fixture artifact",
  ].join("\n");
}
