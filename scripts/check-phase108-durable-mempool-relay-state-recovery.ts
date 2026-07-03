#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-durable-mempool-relay-state-recovery";
const PHASE107_TEST_COMMAND =
  "bun test scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts";
const PHASE107_CHECKER_COMMAND =
  "bun run scripts/check-phase107-runtime-relay-activation-download-eligibility.ts";
const PHASE108_TEST_COMMAND = "bun test scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts";
const PHASE108_CHECKER_COMMAND = "bun run scripts/check-phase108-durable-mempool-relay-state-recovery.ts";

const TARGET_FILES = [
  "README.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-node/src/network/recovery.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  "packages/open-bitcoin-node/src/network/tests/recovery_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/context/tests.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/verify.sh",
] as const;

const REQUIRED_REQUIREMENTS = ["MEM-04", "MEM-05", "MEM-06", "REL-01", "REL-02"] as const;
const REQUIRED_NEEDLES = [
  "ManagedMempoolRecoverySummary",
  "recover_mempool_snapshot",
  "record_mempool_recovery_storage_error",
  "seed_recovered_transaction",
  "RelayRecoveryCounters",
  "recovery_counters",
  "Relay recovery",
  "redacted_relay_mempool_evidence",
  "relay_recovery_recovered_count",
  "relay_mempool_log_record",
  "support bundle --output-dir=/tmp/open-bitcoin-recovery-support",
] as const;

const RECOVERY_FIELDS = [
  "recovered_count",
  "dropped_confirmed_count",
  "dropped_duplicate_count",
  "dropped_missing_parent_count",
  "dropped_policy_incompatible_count",
  "dropped_evicted_count",
] as const;

const FILE_NEEDLES = [
  {
    file: "packages/open-bitcoin-node/src/network/recovery.rs",
    needles: ["ManagedMempoolRecoverySummary", "recover_mempool_snapshot", "record_mempool_recovery_storage_error"],
  },
  {
    file: "packages/open-bitcoin-node/src/network/relay_fanout.rs",
    needles: ["seed_recovered_transaction"],
  },
] as const;

const FORBIDDEN_CLAIMS = [
  "public relay by default",
  "broadcasted to network",
  "public propagation",
  "compact block relay",
  "compact-block relay",
  "package relay",
  "bloom/filter serving",
  "public-network relay CI",
  "production service operation",
  "production-service operation",
  "production full-node readiness",
  "production-readiness proof",
  "production-funds wallet safety",
  "production-funds wallet use",
  "destructive repair",
  "source datadir mutation",
  "compaction",
  "reindex",
  "store surgery",
  "automatic support upload",
] as const;

const PHASE108_CONTEXT_MARKERS = [
  "phase 108",
  "durable mempool relay state recovery",
  "relay recovery",
  "mempool recovery",
  "recovered relay",
  "recovered transaction",
  "recovery counters",
] as const;

const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "must not",
  "not ",
  "without",
  "outside",
  "out of scope",
  "deferred",
  "future",
  "remain",
  "remains",
  "no claim",
  "not claim",
  "only",
  "bounded",
] as const;

export function checkPhase108DurableMempoolRelayStateRecovery(repoRoot = DEFAULT_REPO_ROOT): string[] {
  const failures: string[] = [];
  const files = readTargetFiles(repoRoot, failures);
  const allText = [...files.values()].join("\n");

  requireNeedle(files, failures, "docs/parity/index.json", SURFACE_ID);
  requireNeedle(files, failures, "docs/parity/checklist.md", SURFACE_ID);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireNeedle(files, failures, "docs/parity/index.json", requirement);
    requireNeedle(files, failures, "docs/parity/checklist.md", requirement);
  }
  for (const needle of REQUIRED_NEEDLES) {
    if (!allText.includes(needle)) {
      failures.push(`missing Phase 108 evidence needle: ${needle}`);
    }
  }
  for (const fileNeedle of FILE_NEEDLES) {
    for (const needle of fileNeedle.needles) {
      requireNeedle(files, failures, fileNeedle.file, needle);
    }
  }
  for (const field of RECOVERY_FIELDS) {
    if (!allText.includes(field)) {
      failures.push(`missing Relay recovery field: ${field}`);
    }
  }
  checkVerifierOrder(files.get("scripts/verify.sh") ?? "", failures);
  checkForbiddenClaims(files, failures);

  return failures;
}

function readTargetFiles(repoRoot: string, failures: string[]): Map<string, string> {
  const files = new Map<string, string>();
  for (const file of TARGET_FILES) {
    const absolutePath = path.join(repoRoot, file);
    if (!existsSync(absolutePath)) {
      failures.push(`missing required Phase 108 file: ${file}`);
      files.set(file, "");
      continue;
    }
    files.set(file, readFileSync(absolutePath, "utf8"));
  }
  return files;
}

function requireNeedle(files: Map<string, string>, failures: string[], file: string, needle: string): void {
  if (!(files.get(file) ?? "").includes(needle)) {
    failures.push(`${file} missing ${needle}`);
  }
}

function checkVerifierOrder(verifyScript: string, failures: string[]): void {
  for (const command of [PHASE107_TEST_COMMAND, PHASE107_CHECKER_COMMAND, PHASE108_TEST_COMMAND, PHASE108_CHECKER_COMMAND]) {
    if (!verifyScript.includes(command)) {
      failures.push(`scripts/verify.sh missing ${command}`);
    }
  }
  requireOrdered(verifyScript, PHASE107_TEST_COMMAND, PHASE108_TEST_COMMAND, failures);
  requireOrdered(verifyScript, PHASE107_CHECKER_COMMAND, PHASE108_CHECKER_COMMAND, failures);
}

function requireOrdered(text: string, before: string, after: string, failures: string[]): void {
  const beforeIndex = text.indexOf(before);
  const afterIndex = text.indexOf(after);
  if (beforeIndex < 0 || afterIndex < 0 || beforeIndex >= afterIndex) {
    failures.push(`expected ${before} before ${after}`);
  }
}

function checkForbiddenClaims(files: Map<string, string>, failures: string[]): void {
  for (const [file, text] of files) {
    if (!file.endsWith(".md")) {
      continue;
    }
    for (const paragraph of text.split(/\r?\n\s*\r?\n/)) {
      const lowerParagraph = paragraph.toLowerCase();
      if (!PHASE108_CONTEXT_MARKERS.some((marker) => lowerParagraph.includes(marker))) {
        continue;
      }
      for (const claim of FORBIDDEN_CLAIMS) {
        if (!lowerParagraph.includes(claim.toLowerCase())) {
          continue;
        }
        if (NO_CLAIM_MARKERS.some((marker) => lowerParagraph.includes(marker))) {
          continue;
        }
        failures.push(`${file} has positive or ambiguous forbidden claim: ${claim}`);
      }
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase108DurableMempoolRelayStateRecovery();
  if (failures.length > 0) {
    console.error(failures.join("\n"));
    process.exit(1);
  }
  console.log("Phase 108 durable mempool relay state recovery checks passed.");
}
