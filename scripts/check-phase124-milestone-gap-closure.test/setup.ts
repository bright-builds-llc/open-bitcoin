import { afterEach, expect, test } from "bun:test";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

import type {
  Phase125LifecycleStage,
  Phase126CloseoutStage,
} from "../check-phase124-milestone-gap-closure";
import { checkPhase124MilestoneCloseoutReconciliation } from "../check-phase124-milestone-closeout-reconciliation";
import {
  ACTIVE_TRACEABILITY_CHECK,
  ACTIVE_TRACEABILITY_TEST,
  append,
  createFixture,
  type FixtureFile,
  PHASE124_CHECK,
  PHASE125_LIFECYCLE_ID,
  PHASE125_ROUTE,
  PHASE125_VERIFICATION_FILE,
  PHASE126_ROUTE,
  PHASE126_LIFECYCLE_ID,
  PHASE126_VERIFICATION_FILE,
  PHASE127_ROUTE,
  replace,
} from "../check-phase124-milestone-closeout-reconciliation.fixtures";

export const PHASE125_DIRECTORY =
  ".planning/phases/125-compact-download-verification-traceability-closure";
export const PHASE125_SUMMARY_01 =
  `${PHASE125_DIRECTORY}/125-01-SUMMARY.md` as const;
export const PHASE125_SUMMARY_03 =
  `${PHASE125_DIRECTORY}/125-03-SUMMARY.md` as const;
export const PHASE125_SUMMARY_04 =
  `${PHASE125_DIRECTORY}/125-04-SUMMARY.md` as const;
export const PHASE127_DIRECTORY =
  ".planning/phases/127-authoritative-network-state-unification";
export const PHASE127_LIFECYCLE_ID = "127-2026-07-19T15-09-40";
export const PHASE128_ROUTE = "/gsd-plan-phase 128";
export const ROUTING_FILES = [
  ".planning/ROADMAP.md",
  ".planning/PROJECT.md",
  ".planning/STATE.md",
  ".planning/v2.1-MILESTONE-AUDIT.md",
] as const;
export const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});





























































































export function stageFixture(
  stage: Phase125LifecycleStage["kind"],
  maybeMutate?: (files: Map<FixtureFile, string>) => void,
): string {
  return createFixture(tempRoots, {
    maybePhase125Stage: stage,
    maybeMutate,
  });
}

export function phase126StageFixture(
  stage: Phase126CloseoutStage["kind"],
  maybeMutate?: (files: Map<FixtureFile, string>) => void,
): string {
  return createFixture(tempRoots, {
    maybePhase126Stage: stage,
    maybeMutate,
  });
}

export function postAuditGapPlanningFixture(
  maybeMutate?: (files: Map<FixtureFile, string>) => void,
): string {
  return createFixture(tempRoots, {
    postAuditGapPlanning: true,
    maybeMutate,
  });
}

export function check(root: string): string[] {
  return checkPhase124MilestoneCloseoutReconciliation({ rootDir: root });
}

export function replaceRoutes(
  files: Map<FixtureFile, string>,
  route: string,
  replacement: string,
): void {
  for (const file of ROUTING_FILES) {
    replace(files, file, route, replacement);
  }
}

export function promoteRequirements(files: Map<FixtureFile, string>): void {
  for (const requirement of ["RCN-04", "RCN-05", "RCN-06"]) {
    replace(files, ".planning/REQUIREMENTS.md", `- [ ] **${requirement}**`, `- [x] **${requirement}**`);
    replace(
      files,
      ".planning/REQUIREMENTS.md",
      `| ${requirement} | Phase 125 | Pending |`,
      `| ${requirement} | Phase 125 | Complete |`,
    );
  }
  replace(files, ".planning/REQUIREMENTS.md", "Complete: 30", "Complete: 33");
  replace(
    files,
    ".planning/REQUIREMENTS.md",
    "Pending hardening and closeout: 9",
    "Pending hardening and closeout: 6",
  );
}

export function promotePhase126Requirements(files: Map<FixtureFile, string>): void {
  for (const requirement of [
    "CMP-05",
    "RCN-02",
    "RCN-03",
    "GOV-04",
    "BOUND-01",
    "HARD-05",
  ]) {
    replace(
      files,
      ".planning/REQUIREMENTS.md",
      `- [ ] **${requirement}**`,
      `- [x] **${requirement}**`,
    );
    replace(
      files,
      ".planning/REQUIREMENTS.md",
      `| ${requirement} | Phase 126 | Pending |`,
      `| ${requirement} | Phase 126 | Complete |`,
    );
  }
  replace(files, ".planning/REQUIREMENTS.md", "Complete: 33", "Complete: 39");
  replace(
    files,
    ".planning/REQUIREMENTS.md",
    "Pending hardening and closeout: 6",
    "Pending hardening and closeout: 0",
  );
}

export function phase125Summary(planNumber: "01" | "03" | "04"): string {
  return [
    "---",
    "phase: 125-compact-download-verification-traceability-closure",
    `plan: "${planNumber}"`,
    "requirements-completed: []",
    "generated_by: gsd-execute-plan",
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${PHASE125_LIFECYCLE_ID}`,
    'generated_at: "2026-07-17T15:00:00Z"',
    "---",
    "fixture summary",
  ].join("\n");
}

export function addPhase127Artifacts(
  root: string,
  summaryCount: number,
  includeVerification: boolean,
): void {
  writeRootFile(
    root,
    `${PHASE127_DIRECTORY}/127-CONTEXT.md`,
    phase127Artifact(["generated_by: gsd-discuss-phase"]),
  );
  for (const planNumber of ["01", "02", "03", "04"]) {
    writeRootFile(
      root,
      `${PHASE127_DIRECTORY}/127-${planNumber}-PLAN.md`,
      phase127Artifact([
        "phase: 127-authoritative-network-state-unification",
        `plan: "${planNumber}"`,
        "generated_by: gsd-plan-phase",
      ]),
    );
  }
  for (const planNumber of ["01", "02", "03", "04"].slice(0, summaryCount)) {
    writeRootFile(
      root,
      `${PHASE127_DIRECTORY}/127-${planNumber}-SUMMARY.md`,
      phase127Artifact([
        "phase: 127-authoritative-network-state-unification",
        `plan: "${planNumber}"`,
        "requirements-completed: []",
        "generated_by: gsd-execute-plan",
      ]),
    );
  }
  if (includeVerification) {
    writeRootFile(
      root,
      `${PHASE127_DIRECTORY}/127-VERIFICATION.md`,
      phase127Artifact([
        "phase: 127-authoritative-network-state-unification",
        "status: passed",
        "lifecycle_validated: true",
        "generated_by: gsd-verifier",
      ]),
    );
  }
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

export function promotePhase127Requirements(root: string): void {
  for (const requirement of ["BSRV-03", "BSRV-04", "OBS-02", "OBS-04"]) {
    replaceRootFile(
      root,
      ".planning/REQUIREMENTS.md",
      `- [ ] **${requirement}**`,
      `- [x] **${requirement}**`,
    );
    replaceRootFile(
      root,
      ".planning/REQUIREMENTS.md",
      `| ${requirement} | Phase 127 | Pending |`,
      `| ${requirement} | Phase 127 | Complete |`,
    );
  }
  for (const file of [".planning/REQUIREMENTS.md", ".planning/ROADMAP.md"]) {
    replaceRootFile(
      root,
      file,
      file.endsWith("REQUIREMENTS.md") ? "Complete: 29" : "Satisfied: 29",
      file.endsWith("REQUIREMENTS.md") ? "Complete: 33" : "Satisfied: 33",
    );
    replaceRootFile(
      root,
      file,
      "Pending integration gap closure: 10",
      "Pending integration gap closure: 6",
    );
  }
}

export function writeRootFile(root: string, relativePath: string, text: string): void {
  const absolutePath = path.join(root, relativePath);
  mkdirSync(path.dirname(absolutePath), { recursive: true });
  writeFileSync(absolutePath, text);
}

export function replaceRootFile(
  root: string,
  relativePath: string,
  needle: string,
  replacement: string,
): void {
  const absolutePath = path.join(root, relativePath);
  const text = readFileSync(absolutePath, "utf8");
  if (!text.includes(needle)) {
    throw new Error(`fixture needle missing in ${relativePath}: ${needle}`);
  }
  writeFileSync(absolutePath, text.replace(needle, replacement));
}export { mkdirSync, readFileSync, rmSync, writeFileSync, path, checkPhase124MilestoneCloseoutReconciliation, ACTIVE_TRACEABILITY_CHECK, ACTIVE_TRACEABILITY_TEST, append, createFixture, PHASE124_CHECK, PHASE125_LIFECYCLE_ID, PHASE125_ROUTE, PHASE125_VERIFICATION_FILE, PHASE126_ROUTE, PHASE126_LIFECYCLE_ID, PHASE126_VERIFICATION_FILE, PHASE127_ROUTE, replace };
export type { Phase125LifecycleStage, Phase126CloseoutStage, FixtureFile };
