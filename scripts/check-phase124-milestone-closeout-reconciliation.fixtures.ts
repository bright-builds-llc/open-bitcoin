import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

export const PHASE123_TEST =
  "bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts";
export const PHASE123_CHECK =
  "bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts";
export const PHASE124_TEST =
  "bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts";
export const PHASE124_CHECK =
  "bun run scripts/check-phase124-milestone-closeout-reconciliation.ts";
export const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
export const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
export const LIFECYCLE_ID = "124-2026-07-16T20-19-53";
export const ARCHIVE_ROUTE = "/gsd-complete-milestone v2.1";
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
export const RESOLVED_DEBT_IDS = [
  "DEBT-01-INBOUND-GETBLOCKTXN",
  "DEBT-02-PHASE112-TEST-VOCABULARY",
  "DEBT-03-SUCCESSFUL-BLOCK-WRITE-EVIDENCE",
  "DEBT-04-RECEIVE-INDEPENDENT-TIMEOUT",
  "DEBT-05-AUTHORITATIVE-RUNTIME-PROJECTION",
  "DEBT-06-MILESTONE-METADATA-RECONCILIATION",
] as const;

type RequiredFile = (typeof REQUIRED_FILES)[number];
export type FixtureFile =
  | RequiredFile
  | typeof CONTEXT_FILE
  | typeof PLAN_01_FILE
  | typeof PLAN_02_FILE
  | typeof SUMMARY_01_FILE
  | typeof SUMMARY_FILE
  | typeof VERIFICATION_FILE;
type FixtureOptions = {
  finalStage?: boolean;
  gapClosureStage?: boolean;
  includeVerification?: boolean;
  maybeMutate?: (files: Map<FixtureFile, string>) => void;
  promotedStage?: boolean;
};

export function createFixture(tempRoots: string[], options: FixtureOptions = {}): string {
  const gapClosureStage = options.gapClosureStage ?? false;
  const phaseComplete = (options.finalStage ?? false) || gapClosureStage;
  const finalStage = phaseComplete || (options.promotedStage ?? false);
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase124-"));
  tempRoots.push(root);
  const noClaim =
    "Package relay, filter serving, public-network CI, archive-node behavior, production full-node readiness, and production-funds wallet use remain deferred.";
  const files = new Map<FixtureFile, string>([
    [
      ".planning/REQUIREMENTS.md",
      gapClosureStage ? createGapClosureRequirements() : createRequirements(finalStage),
    ],
    [
      ".planning/ROADMAP.md",
      gapClosureStage ? createGapClosureRoadmap() : createRoadmap(phaseComplete),
    ],
    [".planning/STATE.md", createState(phaseComplete)],
    [
      ".planning/v2.1-MILESTONE-AUDIT.md",
      gapClosureStage ? createGapClosureAudit() : createAudit(finalStage),
    ],
    [".planning/PROJECT.md", noClaim],
    ["README.md", noClaim],
    ["docs/parity/release-readiness.md", noClaim],
    ["docs/parity/production-claim-boundary.md", noClaim],
    ["scripts/verify.sh", createVerifyScript()],
  ]);
  if (options.includeVerification) {
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
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${text}\n`);
  }
  if (gapClosureStage) {
    mkdirSync(
      path.join(root, ".planning/phases/125-compact-download-verification-traceability-closure"),
      { recursive: true },
    );
    mkdirSync(path.join(root, ".planning/phases/126-compact-relay-residual-hardening"), {
      recursive: true,
    });
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

function createGapClosureRequirements(): string {
  const gapPhases = new Map([
    ["RCN-04", 125],
    ["RCN-05", 125],
    ["RCN-06", 125],
    ["CMP-05", 126],
    ["RCN-02", 126],
    ["RCN-03", 126],
    ["GOV-04", 126],
    ["BOUND-01", 126],
    ["HARD-05", 126],
  ]);
  return [
    ...REQUIREMENT_IDS.map(
      (id) => `- [${gapPhases.has(id) ? " " : "x"}] **${id}**: fixture requirement`,
    ),
    ...REQUIREMENT_IDS.map((id) => {
      const maybePhase = gapPhases.get(id);
      return `| ${id} | Phase ${maybePhase ?? phaseFor(id)} | ${maybePhase ? "Pending" : "Complete"} |`;
    }),
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    "- Complete: 30",
    "- Pending hardening and closeout: 9",
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

function createGapClosureRoadmap(): string {
  return [
    "- [x] **Phase 124: Milestone Closeout Reconciliation**",
    "- [ ] **Phase 125: Compact Download Verification Traceability Closure**",
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
    "**Plans:** 0 plans",
    "#### Phase 126: Compact Relay Residual Hardening",
    "**Depends on:** Phase 125",
    "**Requirements:** CMP-05, RCN-02, RCN-03, GOV-04, BOUND-01, HARD-05",
    "**Plans:** 0 plans",
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    "- Satisfied: 30",
    "- Pending hardening and closeout: 9",
    "- Unmapped: 0",
    "## Next Step",
    "/gsd-plan-phase 125",
  ].join("\n");
}

function createState(finalStage: boolean): string {
  if (finalStage) return `Phase 124 verified. Next action: ${ARCHIVE_ROUTE}`;
  return "Phase 124 evidence reconciled; HARD-05 pending";
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

function createGapClosureAudit(): string {
  return [
    "---",
    "status: gaps_found",
    "scores:",
    '  requirements: "36/39"',
    '  phases: "15/15"',
    "gaps:",
    "  requirements:",
    "    - id: RCN-04",
    "    - id: RCN-05",
    "    - id: RCN-06",
    "  integration: []",
    "  flows: []",
    "---",
  ].join("\n");
}

function createVerifyScript(): string {
  const commands = [
    PHASE123_TEST,
    PHASE123_CHECK,
    PHASE124_TEST,
    PHASE124_CHECK,
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
    `run_step "test Phase 117" ${PHASE117_TEST}`,
    `run_step "check Phase 117" ${PHASE117_CHECK}`,
  ].join("\n");
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
