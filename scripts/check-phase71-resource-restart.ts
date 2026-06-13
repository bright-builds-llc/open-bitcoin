#!/usr/bin/env bun

import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE_DIR = ".planning/phases/71-resource-bounds-and-durable-restart-resume";
const PLAN_FILES = [
  `${PHASE_DIR}/71-01-PLAN.md`,
  `${PHASE_DIR}/71-02-PLAN.md`,
  `${PHASE_DIR}/71-03-PLAN.md`,
  `${PHASE_DIR}/71-04-PLAN.md`,
] as const;
const REQUIREMENT_IDS = ["RES-01", "RES-02", "RES-03", "RES-04"] as const;
const SOURCE_FILES = [
  "packages/open-bitcoin-cli/src/operator/support.rs",
  "packages/open-bitcoin-cli/src/operator/support/live_smoke.rs",
  "packages/open-bitcoin-cli/src/operator/runtime/support.rs",
  "packages/open-bitcoin-node/src/storage.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store.rs",
  "packages/open-bitcoin-node/src/sync/progress.rs",
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "packages/open-bitcoin-node/src/sync/types/recovery.rs",
] as const;
const SOURCE_NEEDLES = [
  "phase71_support_redaction_names_compact_evidence_bounds",
  "phase71_live_smoke_summary_is_allowlisted_and_bounded",
  "phase71_runtime_support_resource_pressure_lists_all_configured_bounds",
  "phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight",
  "phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network",
  "StorageRecoveryAction::FreeDisk",
  "for_backend_message",
  "Free disk space for the selected datadir, then retry sync.",
] as const;
const DOC_FILES = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/architecture/storage-decision.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
] as const;
const DOC_NEEDLES = [
  "Phase 71 resource bounds and restart/resume proof",
  "peers, in-flight blocks, request queues, retry maps, cache retention, synchronous storage writes, metrics retention, structured log retention, and support evidence compactness",
  "same-datadir resume matrix: clean shutdown, unclean shutdown, mid-download interruption, mid-connect interruption, stale in-flight cleanup",
  "Free disk space for the selected datadir, then retry sync.",
  "phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network",
  "phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight",
  "SyncResourcePressure",
  "SyncRecoveryCategory::ResourceExhaustion",
  "StorageRecoveryAction::FreeDisk",
  "MetricRetentionPolicy",
  "LogRetentionPolicy",
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet claims",
  "migration apply mode",
  "signed packaging",
  "Windows service support",
  "GUI",
  "hosted dashboards",
  "broad production-node readiness",
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
    requireContains(planText, requirementId, `${PHASE_DIR}/71-*-PLAN.md`, failures);
  }
}

async function verifySource(failures: string[]): Promise<void> {
  const sourceText = await readJoined(SOURCE_FILES, failures);
  for (const needle of SOURCE_NEEDLES) {
    requireContains(sourceText, needle, SOURCE_FILES.join(", "), failures);
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
  const phase70 = "bun run scripts/check-phase70-reorg-recovery.ts";
  const phase71 = "bun run scripts/check-phase71-resource-restart.ts";
  requireContains(verifyScript, phase70, "scripts/verify.sh", failures);
  requireContains(verifyScript, phase71, "scripts/verify.sh", failures);

  const phase70Index = verifyScript.indexOf(phase70);
  const phase71Index = verifyScript.indexOf(phase71);
  if (phase70Index === -1 || phase71Index === -1 || phase71Index < phase70Index) {
    failures.push("scripts/verify.sh must run the Phase 71 checker after the Phase 70 checker");
  }

  requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "--manual-peer", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "--restart-after-progress", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "systemctl", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "launchctl", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "openbitcoinsync=mainnet-ibd", "scripts/verify.sh", failures);
}

async function main(): Promise<void> {
  const failures: string[] = [];
  await verifyRequirements(failures);
  await verifySource(failures);
  await verifyDocs(failures);
  await verifyVerifyScript(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 71 resource/restart evidence");
}

await main();
