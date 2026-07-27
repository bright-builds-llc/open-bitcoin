import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

import {
  detectPhase129ReconciliationStage,
  verifyArchiveReady,
  verifyVerifiedPrePromotion,

} from "../check-phase124-archive-ready";

export const GAP_PHASES = [
  {
    number: 127,
    name: "Authoritative Network State Unification",
    dependsOn: 126,
    requirements: ["BSRV-03", "BSRV-04", "OBS-02", "OBS-04"],
  },
  {
    number: 128,
    name: "Production Compact Announcement Transport",
    dependsOn: 127,
    requirements: ["CMP-04", "CMP-05", "OBS-03"],
  },
  {
    number: 129,
    name: "Integration Guardrails and Milestone Reconciliation",
    dependsOn: 128,
    requirements: ["OBS-01", "BOUND-02", "HARD-05"],
  },
] as const;
export const GAP_REQUIREMENTS = new Map(
  GAP_PHASES.flatMap((phase) =>
    phase.requirements.map((requirement) => [requirement, phase.number] as const),
  ),
);
export const ROUTING_FILES = [
  ".planning/PROJECT.md",
  ".planning/STATE.md",
  ".planning/MILESTONES.md",
] as const;
export const PHASE127_ROUTE = "/gsd-plan-phase 127";
export const PHASE128_ROUTE = "/gsd-plan-phase 128";
export const PHASE129_ROUTE = "/gsd-plan-phase 129";
export const PHASE128_EXECUTION_ROUTE =
  "Execute Phase 128 Plan 04 aggregate guardrails and parity closure.";
export const PHASE127_DIRECTORY =
  ".planning/phases/127-authoritative-network-state-unification";
export const PHASE127_REQUIREMENTS = GAP_PHASES[0].requirements;

export type RequirementEntry = { checked: boolean; id: string };
export type TraceabilityEntry = { id: string; phase: number; status: string };
export type Phase127Lifecycle = {
  complete: boolean;
  promoted: boolean;
  summaryCount: number;
};
export type Phase128LifecycleStage = "planned" | "executing_plan_04" | "complete";
