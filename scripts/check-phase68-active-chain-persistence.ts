#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE_DIR =
  ".planning/phases/68-full-active-chain-validation-and-durable-persistence";
const ACTIVE_CHAIN_FIELDS = [
  "validated_active_chain_height",
  "maybe_validated_active_chain_hash",
  "maybe_validated_active_chain_work",
] as const;
const PROGRESS_FIELDS = [
  "downloaded_block_height",
  "connected_block_height",
  ...ACTIVE_CHAIN_FIELDS,
] as const;
const REQUIRED_PHASE_ARTIFACTS = [
  `${PHASE_DIR}/68-CONTEXT.md`,
  `${PHASE_DIR}/68-DISCUSSION-LOG.md`,
  `${PHASE_DIR}/68-RESEARCH.md`,
  `${PHASE_DIR}/68-01-PLAN.md`,
  `${PHASE_DIR}/68-02-PLAN.md`,
  `${PHASE_DIR}/68-03-PLAN.md`,
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl --user",
  "launchctl",
] as const;

function readText(relativePath: string): string {
  return readFileSync(path.join(REPO_ROOT, relativePath), "utf8");
}

function requireExists(relativePath: string): void {
  if (!existsSync(path.join(REPO_ROOT, relativePath))) {
    throw new Error(`missing required Phase 68 artifact: ${relativePath}`);
  }
}

function requireContains(text: string, needle: string, label: string): void {
  if (!text.includes(needle)) {
    throw new Error(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(text: string, needle: string, label: string): void {
  if (text.includes(needle)) {
    throw new Error(`${label} must not contain default-verification command: ${needle}`);
  }
}

function requireAllContains(text: string, needles: readonly string[], label: string): void {
  for (const needle of needles) {
    requireContains(text, needle, label);
  }
}

function verifyPhaseArtifacts(): void {
  for (const artifact of REQUIRED_PHASE_ARTIFACTS) {
    requireExists(artifact);
  }

  const context = readText(`${PHASE_DIR}/68-CONTEXT.md`);
  const research = readText(`${PHASE_DIR}/68-RESEARCH.md`);
  requireAllContains(
    context,
    [
      "active-chain",
      "durably persisted",
      "cumulative work",
      "downloaded block height",
      "connected block height",
    ],
    `${PHASE_DIR}/68-CONTEXT.md`,
  );
  requireAllContains(
    research,
    [
      "## RESEARCH COMPLETE",
      "Persisted-connected proof",
      "validated active-chain height",
      "Cumulative work",
    ],
    `${PHASE_DIR}/68-RESEARCH.md`,
  );
}

function verifyStatusContract(): void {
  const status = readText("packages/open-bitcoin-node/src/status.rs");
  requireAllContains(status, PROGRESS_FIELDS, "packages/open-bitcoin-node/src/status.rs");
  requireContains(status, "#[serde(default)]", "packages/open-bitcoin-node/src/status.rs");
}

function verifyRuntimeProjection(): void {
  const runtimeState = readText("packages/open-bitcoin-node/src/sync/runtime_state.rs");
  requireAllContains(
    runtimeState,
    [
      "chain_work: u128",
      "progress.validated_active_chain_height = progress.connected_block_height",
      "progress.maybe_validated_active_chain_hash",
      "progress.maybe_connected_block_hash.clone()",
      "block.chain_work.to_string()",
    ],
    "packages/open-bitcoin-node/src/sync/runtime_state.rs",
  );

  const summary = readText("packages/open-bitcoin-node/src/sync/types/summary.rs");
  requireAllContains(
    summary,
    [
      "maybe_validated_active_chain_work",
      "validated_active_chain_height: self.best_block_height",
      "maybe_validated_active_chain_hash: self.maybe_connected_block_hash.clone()",
    ],
    "packages/open-bitcoin-node/src/sync/types/summary.rs",
  );
}

function verifyTests(): void {
  const tests = readText("packages/open-bitcoin-node/src/sync/tests.rs");
  requireAllContains(
    tests,
    [
      "connected_active_chain_progress_survives_runtime_reopen",
      "maybe_validated_active_chain_work: Some(\"2\".to_string())",
      "load_chainstate_snapshot",
      "assert_eq!(active_tip.chain_work, 2)",
      "sync_progress_reports_downloaded_only_block_hash",
    ],
    "packages/open-bitcoin-node/src/sync/tests.rs",
  );
}

function verifyDocs(): void {
  for (const relativePath of [
    "docs/architecture/status-snapshot.md",
    "docs/operator/runtime-guide.md",
  ]) {
    const text = readText(relativePath);
    requireAllContains(text, PROGRESS_FIELDS, relativePath);
    requireAllContains(
      text,
      ["downloaded-only", "consensus validation", "durable"],
      relativePath,
    );
  }
}

function verifyVerifyScript(): void {
  const verifyScript = readText("scripts/verify.sh");
  requireContains(
    verifyScript,
    "bun run scripts/check-phase68-active-chain-persistence.ts",
    "scripts/verify.sh",
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh");
  }
}

function main(): void {
  verifyPhaseArtifacts();
  verifyStatusContract();
  verifyRuntimeProjection();
  verifyTests();
  verifyDocs();
  verifyVerifyScript();

  console.log("validated Phase 68 active-chain persistence evidence");
}

main();
