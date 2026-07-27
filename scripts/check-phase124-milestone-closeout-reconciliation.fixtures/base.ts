import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import type {
  Phase125LifecycleStage,
  Phase126CloseoutStage,
} from "../check-phase124-milestone-gap-closure";
import { lifecycleArtifact, addPhase125Artifacts, phase125Artifact, createRequirements, createGapClosureRequirements, createRoadmap, createGapClosureRoadmap, createState, createGapClosureRouting, createAudit, createGapClosureAudit, createVerifyScript, phase125SummaryCount, phase125VerificationPresent, phase125Promoted, phase125GapPhase, phase125PlanNumbers, phaseFor, range } from "./phase125.ts";
import { createPhase126Requirements, createPhase126Roadmap, createPhase126Routing, createPhase126State, createPhase126Audit, addPhase126Artifacts, phase126Artifact, phase126SummaryCount, phase126Promoted } from "./phase126.ts";
import { createPostAuditGapPlanningRequirements, createPostAuditGapPlanningRoadmap, createPostAuditGapPlanningAudit, addPhase127Artifacts, phase127Artifact, postAuditGapOwners } from "./phase127.ts";
import { createPhase128Requirements, createPhase128Roadmap, createPhase128State } from "./phase128.ts";
import { createArchiveReadyRequirements, createArchiveReadyRoadmap, createArchiveReadyAudit, addPhase129Artifacts, phase129Artifact } from "./phase129.ts";
import { replace, append } from "./mutations.ts";

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
export const PHASE127_LIFECYCLE_ID = "127-2026-07-19T17-54-42";
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

export const PHASE125_DIRECTORY =
  ".planning/phases/125-compact-download-verification-traceability-closure";
export const PHASE126_DIRECTORY =
  ".planning/phases/126-compact-relay-residual-hardening";
export const POST_AUDIT_PHASE_DIRECTORIES = [
  ".planning/phases/127-authoritative-network-state-unification",
  ".planning/phases/128-production-compact-announcement-transport",
  ".planning/phases/129-integration-guardrails-and-milestone-reconciliation",
] as const;
export const REQUIRED_FILES = [
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
export const REQUIREMENT_IDS = [
  ...range("BSRV", 6),
  ...range("CMP", 6),
  ...range("RCN", 7),
  ...range("GOV", 5),
  ...range("OBS", 5),
  ...range("BOUND", 5),
  ...range("HARD", 5),
] as const;
export const PHASE125_REQUIREMENTS = ["RCN-04", "RCN-05", "RCN-06"] as const;
export const PHASE126_REQUIREMENTS = [
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

export type RequiredFile = (typeof REQUIRED_FILES)[number];
export type Phase125PlanNumber = "01" | "02" | "03" | "04";
export type Phase126PlanNumber = Phase125PlanNumber;
export type Phase127PlanNumber = Phase125PlanNumber;
export type Phase128FixtureStage = "executing_plan_04" | "complete";
export type Phase129FixtureStage =
  | "gaps_open"
  | "verified_pre_promotion"
  | "archive_ready";
export type Phase129PlanNumber = Phase125PlanNumber;
export const PHASE127_DIRECTORY =
  ".planning/phases/127-authoritative-network-state-unification";
export const PHASE129_DIRECTORY =
  ".planning/phases/129-integration-guardrails-and-milestone-reconciliation";
export const PHASE129_VERIFICATION_FILE =
  `${PHASE129_DIRECTORY}/129-VERIFICATION.md` as const;
export const PHASE129_REQUIREMENT_IDS = ["OBS-01", "BOUND-02", "HARD-05"] as const;
export const PHASE129_VERIFIED_REQUIREMENT_IDS = [
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
export type FixtureOptions = {
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
