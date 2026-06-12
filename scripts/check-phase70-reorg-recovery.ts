#!/usr/bin/env bun

import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE_DIR = ".planning/phases/70-reorg-peer-rotation-and-no-progress-recovery";
const PLAN_FILES = [
  `${PHASE_DIR}/70-01-PLAN.md`,
  `${PHASE_DIR}/70-02-PLAN.md`,
  `${PHASE_DIR}/70-03-PLAN.md`,
  `${PHASE_DIR}/70-04-PLAN.md`,
  `${PHASE_DIR}/70-05-PLAN.md`,
  `${PHASE_DIR}/70-06-PLAN.md`,
] as const;
const REQUIREMENT_IDS = ["REC-01", "REC-02", "REC-03", "REC-04"] as const;
const SOURCE_FILES = [
  "packages/open-bitcoin-node/src/status.rs",
  "packages/open-bitcoin-node/src/sync/types.rs",
  "packages/open-bitcoin-node/src/sync/block_reconcile.rs",
  "packages/open-bitcoin-node/src/sync/progress.rs",
  "packages/open-bitcoin-node/src/sync/runtime_state.rs",
] as const;
const SOURCE_NEEDLES = [
  "SyncReorgEvidence",
  "latest_reorg",
  "reconcile_progress",
  "SyncReconcileProgress",
  "BranchCompetitionAwaitingBodies",
  "NoProgressDiagnosis",
  "no_progress_diagnosis",
  "no_progress_next_action",
] as const;
const TEST_NEEDLES = ["phase70_reorg", "phase70_peer", "phase70_no_progress"] as const;
const DOC_FILES = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/p2p.md",
] as const;
const DOC_NEEDLES = [
  "sync.latest_reorg",
  "sync.reconcile_progress",
  "sync.no_progress_diagnosis",
  "branch_competition_awaiting_bodies",
  "stale_inflight_cleanup",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "systemctl",
  "launchctl",
  "openbitcoinsync=mainnet-ibd",
] as const;

function repoPath(relativePath: string): string {
  return path.join(REPO_ROOT, relativePath);
}

async function readText(relativePath: string, failures: string[]): Promise<string> {
  const file = Bun.file(repoPath(relativePath));
  if (!(await file.exists())) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }
  return file.text();
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain default verification command: ${needle}`);
  }
}

async function readJoined(files: readonly string[], failures: string[]): Promise<string> {
  const parts = [];
  for (const file of files) {
    parts.push(await readText(file, failures));
  }
  return parts.join("\n");
}

async function verifyRequirements(failures: string[]): Promise<void> {
  const planText = await readJoined(PLAN_FILES, failures);
  for (const requirementId of REQUIREMENT_IDS) {
    requireContains(planText, requirementId, `${PHASE_DIR}/70-*-PLAN.md`, failures);
  }
}

async function verifySource(failures: string[]): Promise<void> {
  const sourceText = await readJoined(SOURCE_FILES, failures);
  for (const needle of SOURCE_NEEDLES) {
    requireContains(sourceText, needle, SOURCE_FILES.join(", "), failures);
  }
}

async function verifyTests(failures: string[]): Promise<void> {
  const tests = await readText("packages/open-bitcoin-node/src/sync/tests.rs", failures);
  for (const needle of TEST_NEEDLES) {
    requireContains(tests, needle, "packages/open-bitcoin-node/src/sync/tests.rs", failures);
  }
}

async function verifyDocs(failures: string[]): Promise<void> {
  const docs = await readJoined(DOC_FILES, failures);
  for (const needle of DOC_NEEDLES) {
    requireContains(docs, needle, DOC_FILES.join(", "), failures);
  }
}

async function verifyVerifyScript(failures: string[]): Promise<void> {
  const verifyScript = await readText("scripts/verify.sh", failures);
  const phase69 = "bun run scripts/check-phase69-tip-stay-current.ts";
  const phase70 = "bun run scripts/check-phase70-reorg-recovery.ts";
  requireContains(verifyScript, phase70, "scripts/verify.sh", failures);

  const phase69Index = verifyScript.indexOf(phase69);
  const phase70Index = verifyScript.indexOf(phase70);
  if (phase69Index === -1 || phase70Index === -1 || phase70Index < phase69Index) {
    failures.push("scripts/verify.sh must run the Phase 70 checker after the Phase 69 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh", failures);
  }
}

async function main(): Promise<void> {
  const failures: string[] = [];
  await verifyRequirements(failures);
  await verifySource(failures);
  await verifyTests(failures);
  await verifyDocs(failures);
  await verifyVerifyScript(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 70 reorg recovery evidence");
}

await main();
