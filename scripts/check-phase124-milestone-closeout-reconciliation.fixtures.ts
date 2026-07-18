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
export const PHASE126_LIFECYCLE_ID = "126-2026-07-18T16-09-20";
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
export type FixtureFile =
  | RequiredFile
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
  | `${typeof PHASE126_DIRECTORY}/126-${Phase126PlanNumber}-SUMMARY.md`;
type FixtureOptions = {
  finalStage?: boolean;
  includeVerification?: boolean;
  maybeMutate?: (files: Map<FixtureFile, string>) => void;
  maybePhase125Stage?: Phase125LifecycleStage["kind"];
  maybePhase126Stage?: Phase126CloseoutStage["kind"];
  promotedStage?: boolean;
};

export function createFixture(tempRoots: string[], options: FixtureOptions = {}): string {
  const maybePhase125Stage = options.maybePhase125Stage;
  const maybePhase126Stage = options.maybePhase126Stage;
  const gapClosureStage =
    maybePhase125Stage !== undefined || maybePhase126Stage !== undefined;
  const phaseComplete = (options.finalStage ?? false) || gapClosureStage;
  const finalStage = phaseComplete || (options.promotedStage ?? false);
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase124-"));
  tempRoots.push(root);
  const noClaim =
    "Package relay, filter serving, public-network CI, archive-node behavior, production full-node readiness, and production-funds wallet use remain deferred.";
  const files = new Map<FixtureFile, string>([
    [
      ".planning/REQUIREMENTS.md",
      maybePhase126Stage !== undefined
        ? createPhase126Requirements(maybePhase126Stage)
        : gapClosureStage
        ? createGapClosureRequirements(maybePhase125Stage)
        : createRequirements(finalStage),
    ],
    [
      ".planning/ROADMAP.md",
      maybePhase126Stage !== undefined
        ? createPhase126Roadmap(maybePhase126Stage)
        : gapClosureStage
        ? createGapClosureRoadmap(maybePhase125Stage)
        : createRoadmap(phaseComplete),
    ],
    [
      ".planning/STATE.md",
      maybePhase126Stage !== undefined
        ? createPhase126State(maybePhase126Stage)
        : gapClosureStage
          ? createGapClosureRouting(maybePhase125Stage)
          : createState(phaseComplete),
    ],
    [
      ".planning/v2.1-MILESTONE-AUDIT.md",
      maybePhase126Stage !== undefined
        ? createPhase126Audit(maybePhase126Stage)
        : gapClosureStage
          ? createGapClosureAudit(maybePhase125Stage)
          : createAudit(finalStage),
    ],
    [
      ".planning/PROJECT.md",
      gapClosureStage
        ? `${noClaim}\n${
            maybePhase126Stage !== undefined
              ? createPhase126Routing(maybePhase126Stage)
              : createGapClosureRouting(maybePhase125Stage)
          }`
        : noClaim,
    ],
    ["README.md", noClaim],
    ["docs/parity/release-readiness.md", noClaim],
    ["docs/parity/production-claim-boundary.md", noClaim],
    [
      "scripts/verify.sh",
      createVerifyScript(
        maybePhase125Stage ?? (maybePhase126Stage === undefined ? undefined : "post_summary"),
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
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${text}\n`);
  }
  if (gapClosureStage) {
    mkdirSync(path.join(root, PHASE126_DIRECTORY), { recursive: true });
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

function createState(finalStage: boolean): string {
  if (finalStage) return `Phase 124 verified. Next action: ${ARCHIVE_ROUTE}`;
  return "Phase 124 evidence reconciled; HARD-05 pending";
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
