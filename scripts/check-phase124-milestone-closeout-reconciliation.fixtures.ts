import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import type {
  Phase125LifecycleStage,
  Phase126CloseoutStage,
} from "./check-phase124-milestone-gap-closure";

export const PHASE123_TEST =
  "bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts";
export const PHASE123_CHECK =
  "bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts";
export const PHASE124_TEST =
  "bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts";
export const PHASE124_CHECK =
  "bun run scripts/check-phase124-milestone-closeout-reconciliation.ts";
export const ACTIVE_TRACEABILITY_TEST =
  "bun test scripts/check-active-milestone-verification-traceability.test.ts";
export const ACTIVE_TRACEABILITY_CHECK =
  "bun run scripts/check-active-milestone-verification-traceability.ts";
export const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
export const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
export const LIFECYCLE_ID = "124-2026-07-16T20-19-53";
export const PHASE125_LIFECYCLE_ID = "125-2026-07-17T13-21-01";
export const ARCHIVE_ROUTE = "/gsd-complete-milestone v2.1";
export const PHASE125_ROUTE = "/gsd-execute-phase 125";
export const PHASE126_ROUTE = "/gsd-execute-phase 126";
export const PHASE127_ROUTE = "/gsd-plan-phase 127";
export const PHASE129_ROUTE = "/gsd-plan-phase 129";
export const PHASE128_EXECUTION_ROUTE =
  "Execute Phase 128 Plan 04 aggregate guardrails and parity closure.";
export const PHASE126_LIFECYCLE_ID = "126-2026-07-18T16-09-20";
const PHASE127_LIFECYCLE_ID = "127-2026-07-19T17-54-42";
export const PHASE129_LIFECYCLE_ID = "129-2026-07-20T19-28-06";
export const VERIFICATION_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-VERIFICATION.md";
export const SUMMARY_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-02-SUMMARY.md";
export const CONTEXT_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-CONTEXT.md";
export const PLAN_01_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-01-PLAN.md";
export const PLAN_02_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-02-PLAN.md";
export const SUMMARY_01_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-01-SUMMARY.md";
export const PHASE125_CONTEXT_FILE =
  ".planning/phases/125-compact-download-verification-traceability-closure/125-CONTEXT.md";
export const PHASE125_VERIFICATION_FILE =
  ".planning/phases/125-compact-download-verification-traceability-closure/125-VERIFICATION.md";
export const PHASE126_CONTEXT_FILE =
  ".planning/phases/126-compact-relay-residual-hardening/126-CONTEXT.md";
export const PHASE126_VERIFICATION_FILE =
  ".planning/phases/126-compact-relay-residual-hardening/126-VERIFICATION.md";

const PHASE125_DIRECTORY =
  ".planning/phases/125-compact-download-verification-traceability-closure";
const PHASE126_DIRECTORY =
  ".planning/phases/126-compact-relay-residual-hardening";
const POST_AUDIT_PHASE_DIRECTORIES = [
  ".planning/phases/127-authoritative-network-state-unification",
  ".planning/phases/128-production-compact-announcement-transport",
  ".planning/phases/129-integration-guardrails-and-milestone-reconciliation",
] as const;
const REQUIRED_FILES = [
  ".planning/REQUIREMENTS.md",
  ".planning/ROADMAP.md",
  ".planning/STATE.md",
  ".planning/v2.1-MILESTONE-AUDIT.md",
  ".planning/PROJECT.md",
  "README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "scripts/verify.sh",
] as const;
const REQUIREMENT_IDS = [
  ...range("BSRV", 6),
  ...range("CMP", 6),
  ...range("RCN", 7),
  ...range("GOV", 5),
  ...range("OBS", 5),
  ...range("BOUND", 5),
  ...range("HARD", 5),
] as const;
const PHASE125_REQUIREMENTS = ["RCN-04", "RCN-05", "RCN-06"] as const;
const PHASE126_REQUIREMENTS = [
  "CMP-05",
  "RCN-02",
  "RCN-03",
  "GOV-04",
  "BOUND-01",
  "HARD-05",
] as const;
export const RESOLVED_DEBT_IDS = [
  "DEBT-01-INBOUND-GETBLOCKTXN",
  "DEBT-02-PHASE112-TEST-VOCABULARY",
  "DEBT-03-SUCCESSFUL-BLOCK-WRITE-EVIDENCE",
  "DEBT-04-RECEIVE-INDEPENDENT-TIMEOUT",
  "DEBT-05-AUTHORITATIVE-RUNTIME-PROJECTION",
  "DEBT-06-MILESTONE-METADATA-RECONCILIATION",
] as const;

type RequiredFile = (typeof REQUIRED_FILES)[number];
type Phase125PlanNumber = "01" | "02" | "03" | "04";
type Phase126PlanNumber = Phase125PlanNumber;
type Phase127PlanNumber = Phase125PlanNumber;
type Phase128FixtureStage = "executing_plan_04" | "complete";
export type Phase129FixtureStage =
  | "gaps_open"
  | "verified_pre_promotion"
  | "archive_ready";
type Phase129PlanNumber = Phase125PlanNumber;
const PHASE127_DIRECTORY =
  ".planning/phases/127-authoritative-network-state-unification";
const PHASE129_DIRECTORY =
  ".planning/phases/129-integration-guardrails-and-milestone-reconciliation";
export const PHASE129_VERIFICATION_FILE =
  `${PHASE129_DIRECTORY}/129-VERIFICATION.md` as const;
const PHASE129_REQUIREMENT_IDS = ["OBS-01", "BOUND-02", "HARD-05"] as const;
const PHASE129_VERIFIED_REQUIREMENT_IDS = [
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
export type FixtureFile =
  | RequiredFile
  | ".planning/MILESTONES.md"
  | typeof CONTEXT_FILE
  | typeof PLAN_01_FILE
  | typeof PLAN_02_FILE
  | typeof SUMMARY_01_FILE
  | typeof SUMMARY_FILE
  | typeof VERIFICATION_FILE
  | typeof PHASE125_CONTEXT_FILE
  | typeof PHASE125_VERIFICATION_FILE
  | typeof PHASE126_CONTEXT_FILE
  | typeof PHASE126_VERIFICATION_FILE
  | `${typeof PHASE125_DIRECTORY}/125-${Phase125PlanNumber}-PLAN.md`
  | `${typeof PHASE125_DIRECTORY}/125-${Phase125PlanNumber}-SUMMARY.md`
  | `${typeof PHASE126_DIRECTORY}/126-${Phase126PlanNumber}-PLAN.md`
  | `${typeof PHASE126_DIRECTORY}/126-${Phase126PlanNumber}-SUMMARY.md`
  | `${typeof PHASE127_DIRECTORY}/127-CONTEXT.md`
  | `${typeof PHASE127_DIRECTORY}/127-VERIFICATION.md`
  | `${typeof PHASE127_DIRECTORY}/127-${Phase127PlanNumber}-PLAN.md`
  | `${typeof PHASE127_DIRECTORY}/127-${Phase127PlanNumber}-SUMMARY.md`
  | `${typeof PHASE129_DIRECTORY}/129-CONTEXT.md`
  | `${typeof PHASE129_DIRECTORY}/129-VERIFICATION.md`
  | `${typeof PHASE129_DIRECTORY}/129-${Phase129PlanNumber}-PLAN.md`
  | `${typeof PHASE129_DIRECTORY}/129-${Phase129PlanNumber}-SUMMARY.md`;
type FixtureOptions = {
  finalStage?: boolean;
  includeVerification?: boolean;
  maybeMutate?: (files: Map<FixtureFile, string>) => void;
  postAuditGapPlanning?: boolean;
  maybePhase125Stage?: Phase125LifecycleStage["kind"];
  maybePhase126Stage?: Phase126CloseoutStage["kind"];
  maybePhase128Stage?: Phase128FixtureStage;
  maybePhase129Stage?: Phase129FixtureStage;
  promotedStage?: boolean;
};

export function createFixture(tempRoots: string[], options: FixtureOptions = {}): string {
  const maybePhase125Stage = options.maybePhase125Stage;
  const maybePhase126Stage = options.maybePhase126Stage;
  const maybePhase129Stage = options.maybePhase129Stage;
  const archiveReady = maybePhase129Stage === "archive_ready";
  const maybePhase128Stage =
    options.maybePhase128Stage ??
    (maybePhase129Stage === undefined || archiveReady ? undefined : "complete");
  const postAuditGapPlanning =
    (options.postAuditGapPlanning ?? false) ||
    maybePhase128Stage !== undefined ||
    maybePhase129Stage !== undefined;
  const gapClosureStage =
    postAuditGapPlanning ||
    maybePhase125Stage !== undefined ||
    maybePhase126Stage !== undefined;
  const phaseComplete = (options.finalStage ?? false) || gapClosureStage;
  const finalStage = phaseComplete || (options.promotedStage ?? false);
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase124-"));
  tempRoots.push(root);
  const noClaim =
    "Package relay, filter serving, public-network CI, archive-node behavior, production full-node readiness, and production-funds wallet use remain deferred.";
  const files = new Map<FixtureFile, string>([
    [
      ".planning/REQUIREMENTS.md",
      archiveReady
        ? createArchiveReadyRequirements()
        : maybePhase128Stage !== undefined
        ? createPhase128Requirements()
        : postAuditGapPlanning
        ? createPostAuditGapPlanningRequirements()
        : maybePhase126Stage !== undefined
        ? createPhase126Requirements(maybePhase126Stage)
        : gapClosureStage
        ? createGapClosureRequirements(maybePhase125Stage)
        : createRequirements(finalStage),
    ],
    [
      ".planning/ROADMAP.md",
      archiveReady
        ? createArchiveReadyRoadmap()
        : maybePhase128Stage !== undefined
        ? createPhase128Roadmap(maybePhase128Stage)
        : postAuditGapPlanning
        ? createPostAuditGapPlanningRoadmap()
        : maybePhase126Stage !== undefined
        ? createPhase126Roadmap(maybePhase126Stage)
        : gapClosureStage
        ? createGapClosureRoadmap(maybePhase125Stage)
        : createRoadmap(phaseComplete),
    ],
    [
      ".planning/STATE.md",
      archiveReady
        ? `Phase: 129 complete\nNext action: Run \`${ARCHIVE_ROUTE}\`.`
        : maybePhase128Stage !== undefined
        ? createPhase128State(maybePhase128Stage)
        : postAuditGapPlanning
        ? `status: planning\nNext action: Run \`${PHASE127_ROUTE}\`.`
        : maybePhase126Stage !== undefined
        ? createPhase126State(maybePhase126Stage)
        : gapClosureStage
          ? createGapClosureRouting(maybePhase125Stage)
          : createState(phaseComplete),
    ],
    [
      ".planning/v2.1-MILESTONE-AUDIT.md",
      archiveReady
        ? createArchiveReadyAudit()
        : postAuditGapPlanning
        ? createPostAuditGapPlanningAudit()
        : maybePhase126Stage !== undefined
        ? createPhase126Audit(maybePhase126Stage)
        : gapClosureStage
          ? createGapClosureAudit(maybePhase125Stage)
          : createAudit(finalStage),
    ],
    [
      ".planning/PROJECT.md",
      archiveReady
        ? `${noClaim}\nNext action: Run \`${ARCHIVE_ROUTE}\`.`
        : postAuditGapPlanning
        ? `${noClaim}\nNext action: Run \`${PHASE127_ROUTE}\`.`
        : gapClosureStage
        ? `${noClaim}\n${
            maybePhase126Stage !== undefined
              ? createPhase126Routing(maybePhase126Stage)
              : createGapClosureRouting(maybePhase125Stage)
          }`
        : noClaim,
    ],
    [
      ".planning/MILESTONES.md",
      archiveReady
        ? `${noClaim}\n**What's next:** Run \`${ARCHIVE_ROUTE}\`.`
        : postAuditGapPlanning
        ? `${noClaim}\n**What's next:** Run \`${PHASE127_ROUTE}\`.`
        : noClaim,
    ],
    ["README.md", noClaim],
    ["docs/parity/release-readiness.md", noClaim],
    ["docs/parity/production-claim-boundary.md", noClaim],
    [
      "scripts/verify.sh",
      createVerifyScript(
        postAuditGapPlanning
          ? "post_summary"
          : maybePhase125Stage ??
              (maybePhase126Stage === undefined ? undefined : "post_summary"),
      ),
    ],
  ]);
  if (options.includeVerification || gapClosureStage) {
    files.set(
      VERIFICATION_FILE,
      [
        "---",
        "phase: 124-milestone-closeout-reconciliation",
        "status: passed",
        "lifecycle_validated: true",
        "generated_by: gsd-verifier",
        "lifecycle_mode: yolo",
        `phase_lifecycle_id: ${LIFECYCLE_ID}`,
        'generated_at: "2026-07-16T22:21:10Z"',
        "---",
      ].join("\n"),
    );
  }
  if (phaseComplete) {
    files.set(
      CONTEXT_FILE,
      lifecycleArtifact("gsd-discuss-phase", "2026-07-16T20:26:30Z"),
    );
    files.set(PLAN_01_FILE, lifecycleArtifact("gsd-planner", "2026-07-16T20:57:35Z"));
    files.set(PLAN_02_FILE, lifecycleArtifact("gsd-planner", "2026-07-16T20:57:35Z"));
    files.set(
      SUMMARY_01_FILE,
      lifecycleArtifact("gsd-execute-plan", "2026-07-16T21:25:00Z"),
    );
    files.set(
      SUMMARY_FILE,
      lifecycleArtifact("gsd-execute-plan", "2026-07-16T21:56:00Z"),
    );
  }
  if (maybePhase125Stage !== undefined) {
    addPhase125Artifacts(files, maybePhase125Stage);
  }
  if (maybePhase126Stage !== undefined) {
    addPhase125Artifacts(files, "post_summary");
    addPhase126Artifacts(files, maybePhase126Stage);
  }
  if (postAuditGapPlanning) {
    addPhase125Artifacts(files, "post_summary");
    addPhase126Artifacts(files, "archive_ready");
  }
  if (maybePhase128Stage !== undefined || archiveReady) {
    addPhase127Artifacts(files);
  }
  if (maybePhase129Stage === "verified_pre_promotion" || archiveReady) {
    addPhase129Artifacts(files, maybePhase129Stage ?? "archive_ready");
  }
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${text}\n`);
  }
  if (gapClosureStage) {
    mkdirSync(path.join(root, PHASE126_DIRECTORY), { recursive: true });
  }
  if (postAuditGapPlanning) {
    for (const directory of POST_AUDIT_PHASE_DIRECTORIES) {
      mkdirSync(path.join(root, directory), { recursive: true });
    }
  }
  return root;
}

function lifecycleArtifact(generatedBy: string, generatedAt: string): string {
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

function addPhase125Artifacts(
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

function phase125Artifact(fields: readonly string[]): string {
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

function createRequirements(finalStage: boolean): string {
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

function createGapClosureRequirements(stage: Phase125LifecycleStage["kind"]): string {
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

function createPhase126Requirements(stage: Phase126CloseoutStage["kind"]): string {
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

function createPostAuditGapPlanningRequirements(): string {
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

function createPhase128Requirements(): string {
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

function createRoadmap(phaseComplete: boolean): string {
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

function createGapClosureRoadmap(stage: Phase125LifecycleStage["kind"]): string {
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

function createPhase126Roadmap(stage: Phase126CloseoutStage["kind"]): string {
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

function createPostAuditGapPlanningRoadmap(): string {
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

function createPhase128Roadmap(stage: Phase128FixtureStage): string {
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

function createArchiveReadyRequirements(): string {
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

function createArchiveReadyRoadmap(): string {
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

function createArchiveReadyAudit(): string {
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
    '      - "scripts/check-phase124-milestone-gap-closure.ts is 1,505 lines and concentrates unrelated lifecycle assertions."',
    "---",
    "## Next Action",
    "",
    `Run \`${ARCHIVE_ROUTE}\` to archive the reconciled milestone.`,
  ].join("\n");
}

function createState(finalStage: boolean): string {
  if (finalStage) return `Phase 124 verified. Next action: ${ARCHIVE_ROUTE}`;
  return "Phase 124 evidence reconciled; HARD-05 pending";
}

function createPhase128State(stage: Phase128FixtureStage): string {
  return stage === "complete"
    ? `Phase: 129\nNext action: Run \`${PHASE129_ROUTE}\`.`
    : `Phase: 128\nPlan: 4 of 4\nStatus: In progress\nNext action: ${PHASE128_EXECUTION_ROUTE}`;
}

function createGapClosureRouting(stage: Phase125LifecycleStage["kind"]): string {
  return phase125Promoted(stage) ? PHASE126_ROUTE : PHASE125_ROUTE;
}

function createPhase126Routing(stage: Phase126CloseoutStage["kind"]): string {
  return stage === "archive_ready" ? ARCHIVE_ROUTE : PHASE126_ROUTE;
}

function createPhase126State(stage: Phase126CloseoutStage["kind"]): string {
  if (stage === "candidate") {
    return "Phase 126 candidate execution is in progress.";
  }
  return createPhase126Routing(stage);
}

function createAudit(finalStage: boolean): string {
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

function createGapClosureAudit(stage: Phase125LifecycleStage["kind"]): string {
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

function createPhase126Audit(stage: Phase126CloseoutStage["kind"]): string {
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

function createPostAuditGapPlanningAudit(): string {
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

function createVerifyScript(
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

function phase125SummaryCount(stage: Phase125LifecycleStage["kind"]): number {
  if (stage === "planned") return 0;
  if (stage === "pre_verification") return 2;
  if (stage === "post_summary") return 4;
  return 3;
}

function phase125VerificationPresent(stage: Phase125LifecycleStage["kind"]): boolean {
  return !["planned", "pre_verification"].includes(stage);
}

function phase125Promoted(stage: Phase125LifecycleStage["kind"]): boolean {
  return stage === "post_verification" || stage === "post_summary";
}

function addPhase126Artifacts(
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

function addPhase127Artifacts(files: Map<FixtureFile, string>): void {
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

function addPhase129Artifacts(
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

function phase129Artifact(
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

function phase127Artifact(fields: readonly string[]): string {
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

function phase126Artifact(fields: readonly string[]): string {
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

function phase126SummaryCount(stage: Phase126CloseoutStage["kind"]): number {
  if (stage === "candidate") return 1;
  if (stage === "archive_ready") return 4;
  return 3;
}

function phase126Promoted(stage: Phase126CloseoutStage["kind"]): boolean {
  return stage === "promoted_pre_summary" || stage === "archive_ready";
}

function phase125GapPhase(id: string): number | undefined {
  if (PHASE125_REQUIREMENTS.includes(id as (typeof PHASE125_REQUIREMENTS)[number])) {
    return 125;
  }
  if (PHASE126_REQUIREMENTS.includes(id as (typeof PHASE126_REQUIREMENTS)[number])) {
    return 126;
  }
  return undefined;
}

function postAuditGapOwners(): Map<string, number> {
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

function phase125PlanNumbers(): Phase125PlanNumber[] {
  return ["01", "02", "03", "04"];
}

function phaseFor(id: string): number {
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

function range(prefix: string, count: number): string[] {
  return Array.from({ length: count }, (_, index) =>
    `${prefix}-${String(index + 1).padStart(2, "0")}`,
  );
}

export function replace(
  files: Map<FixtureFile, string>,
  file: FixtureFile,
  needle: string,
  replacement: string,
): void {
  files.set(file, (files.get(file) ?? "").replace(needle, replacement));
}

export function append(
  files: Map<FixtureFile, string>,
  file: FixtureFile,
  value: string,
): void {
  files.set(file, `${files.get(file) ?? ""}\n${value}`);
}
