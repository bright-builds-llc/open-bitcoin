import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

export const PHASE125_DIRECTORY =
  ".planning/phases/125-compact-download-verification-traceability-closure";
export const PHASE126_DIRECTORY =
  ".planning/phases/126-compact-relay-residual-hardening";
export const PHASE125_CONTEXT = `${PHASE125_DIRECTORY}/125-CONTEXT.md`;
export const PHASE125_VERIFICATION = `${PHASE125_DIRECTORY}/125-VERIFICATION.md`;
export const PHASE126_CONTEXT = `${PHASE126_DIRECTORY}/126-CONTEXT.md`;
export const PHASE126_VERIFICATION = `${PHASE126_DIRECTORY}/126-VERIFICATION.md`;
export const PHASE125_NAME = "Compact Download Verification Traceability Closure";
export const PHASE126_NAME = "Compact Relay Residual Hardening";
export const PHASE125_REQUIREMENTS = ["RCN-04", "RCN-05", "RCN-06"] as const;
export const PHASE126_REQUIREMENTS = [
  "CMP-05",
  "RCN-02",
  "RCN-03",
  "GOV-04",
  "BOUND-01",
  "HARD-05",
] as const;
export const GAP_REQUIREMENT_PHASES = new Map([
  ...PHASE125_REQUIREMENTS.map((id) => [id, 125] as const),
  ...PHASE126_REQUIREMENTS.map((id) => [id, 126] as const),
]);
export const EXPECTED_PLAN_NUMBERS = ["01", "02", "03", "04"] as const;
export const PHASE125_ROUTE = "/gsd-execute-phase 125";
export const PHASE126_ROUTE = "/gsd-execute-phase 126";
export const ARCHIVE_ROUTE = "/gsd-complete-milestone v2.1";
export const ROUTING_FILES = [
  ".planning/ROADMAP.md",
  ".planning/PROJECT.md",
  ".planning/STATE.md",
  ".planning/v2.1-MILESTONE-AUDIT.md",
] as const;

export type Phase125LifecycleStage =
  | {
      kind: "planned";
      planCount: 4;
      summaryCount: 0;
      verificationPresent: false;
    }
  | {
      kind: "pre_verification";
      planCount: 4;
      summaryCount: 1 | 2 | 3;
      verificationPresent: false;
    }
  | {
      kind: "verification_written_pre_promotion";
      planCount: 4;
      summaryCount: 3;
      verificationPresent: true;
    }
  | {
      kind: "post_verification";
      planCount: 4;
      summaryCount: 3;
      verificationPresent: true;
    }
  | {
      kind: "post_summary";
      planCount: 4;
      summaryCount: 4;
      verificationPresent: true;
    };

export type Phase126CloseoutStage =
  | {
      kind: "candidate";
      planCount: 4;
      summaryCount: 1 | 2 | 3;
      verificationPresent: false;
    }
  | {
      kind: "verified_pre_promotion";
      planCount: 4;
      summaryCount: 3;
      verificationPresent: true;
    }
  | {
      kind: "promoted_pre_summary";
      planCount: 4;
      summaryCount: 3;
      verificationPresent: true;
    }
  | {
      kind: "archive_ready";
      planCount: 4;
      summaryCount: 4;
      verificationPresent: true;
    };

export type RequirementEntry = { checked: boolean; id: string };
export type TraceabilityEntry = { id: string; phase: number; status: string };
export type LifecycleIdentity = { mode: string; phaseLifecycleId: string };
export type Phase125Artifacts = {
  planCount: number;
  summaryCount: number;
  verificationPresent: boolean;
};
export type ProjectionState = "pending" | "promoted";
