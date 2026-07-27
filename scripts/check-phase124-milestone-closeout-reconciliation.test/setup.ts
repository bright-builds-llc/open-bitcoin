import { afterEach, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { rmSync } from "node:fs";
import path from "node:path";

import { checkPhase124MilestoneCloseoutReconciliation } from "../check-phase124-milestone-closeout-reconciliation";
import {
  append,
  ARCHIVE_ROUTE,
  CONTEXT_FILE,
  createFixture as createPhase124Fixture,
  LIFECYCLE_ID,
  PHASE128_EXECUTION_ROUTE,
  PHASE129_ROUTE,
  PHASE129_VERIFICATION_FILE,
  PHASE117_CHECK,
  PHASE117_TEST,
  PHASE124_CHECK,
  PHASE124_TEST,
  replace,
  RESOLVED_DEBT_IDS,
  SUMMARY_FILE,
  VERIFICATION_FILE,
} from "../check-phase124-milestone-closeout-reconciliation.fixtures";

export const tempRoots: string[] = [];

export const createFixture = (
  options?: Parameters<typeof createPhase124Fixture>[1],
): string => createPhase124Fixture(tempRoots, options);

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});











export const PHASE129_SUMMARY_04_FILE =
  ".planning/phases/129-integration-guardrails-and-milestone-reconciliation/129-04-SUMMARY.md" as const;export { spawnSync, rmSync, path, checkPhase124MilestoneCloseoutReconciliation, append, ARCHIVE_ROUTE, CONTEXT_FILE, createPhase124Fixture, LIFECYCLE_ID, PHASE128_EXECUTION_ROUTE, PHASE129_ROUTE, PHASE129_VERIFICATION_FILE, PHASE117_CHECK, PHASE117_TEST, PHASE124_CHECK, PHASE124_TEST, replace, RESOLVED_DEBT_IDS, SUMMARY_FILE, VERIFICATION_FILE };
