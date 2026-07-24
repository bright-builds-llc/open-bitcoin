#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-mempool-chainstate-lifecycle-durable-recovery";
const PHASE102_TEST_COMMAND = "bun test scripts/check-phase102-orphan-admission-bridge.test.ts";
const PHASE102_CHECKER_COMMAND = "bun run scripts/check-phase102-orphan-admission-bridge.ts";
const PHASE103_TEST_COMMAND = "bun test scripts/check-phase103-mempool-lifecycle.test.ts";
const PHASE103_CHECKER_COMMAND = "bun run scripts/check-phase103-mempool-lifecycle.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_REQUIREMENTS = ["MEM-03", "MEM-04", "MEM-05", "MEM-06"] as const;
const TARGET_FILES = [
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  "packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/storage.rs",
  "packages/open-bitcoin-node/src/storage/mempool_snapshot.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/tests.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs",
  "scripts/verify.sh",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-01-SUMMARY.md",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-02-SUMMARY.md",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-03-SUMMARY.md",
] as const;
const REQUIRED_EVIDENCE_ROOTS = [
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  "packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/storage/mempool_snapshot.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/tests.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs",
  "scripts/check-phase103-mempool-lifecycle.ts",
  "scripts/check-phase103-mempool-lifecycle.test.ts",
  "scripts/verify.sh",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-01-SUMMARY.md",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-02-SUMMARY.md",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-03-SUMMARY.md",
] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/txmempool.h",
  "packages/bitcoin-knots/src/txmempool.cpp",
  "packages/bitcoin-knots/src/validation.cpp",
  "packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp",
  "packages/bitcoin-knots/src/node/mempool_persist.cpp",
  "packages/bitcoin-knots/test/functional/mempool_limit.py",
  "packages/bitcoin-knots/test/functional/mempool_reorg.py",
  "packages/bitcoin-knots/test/functional/mempool_persist.py",
] as const;
const REQUIRED_SYMBOLS = [
  "MempoolPressureSummary",
  "MempoolRemovalCause",
  "MempoolRemovalRole",
  "RollingFeeParityStatus",
  "MempoolCapacityStatus",
  "remove_for_connected_block",
  "remove_for_connected_transactions",
  "apply_connected_block_mempool_lifecycle",
  "apply_reorg_mempool_lifecycle",
  "remove_stored_transactions",
  "submit_transaction_outcome",
  "StorageNamespace::Mempool",
  "MempoolSnapshot",
  "MempoolSnapshotRecord",
  "MempoolRecoveryStatus",
  "replay_into_mempool",
  "save_mempool_snapshot",
  "load_mempool_snapshot",
  "clear_mempool_snapshot",
] as const;
const REQUIRED_BEHAVIOR_TESTS = [
  "lifecycle_pressure_summary_reports_capacity_and_fee_floor",
  "block_connect_removes_confirmed_transaction_and_recomputes_indexes",
  "block_connect_removes_conflict_and_descendants",
  "managed_block_connect_removes_confirmed_mempool_transaction_and_runtime_caches",
  "managed_block_connect_removes_conflict_and_descendant_caches",
  "managed_reorg_reacceptance_uses_explicit_event_time",
  "mempool_snapshot_replay_recovers_accepted_records",
  "mempool_snapshot_replay_drops_confirmed_records_with_evidence",
  "mempool_snapshot_replay_drops_policy_incompatible_records_with_evidence",
  "mempool_snapshot_codec_rejects_schema_mismatch",
  "mempool_snapshot_codec_rejects_corrupt_bytes",
  "fjall_mempool_snapshot_round_trips_after_reopen",
  "fjall_mempool_snapshot_remove_clears_persisted_state",
  "fjall_mempool_snapshot_reports_corruption",
] as const;
const REQUIRED_BREADCRUMB_GROUPS = [
  "mempool-lifecycle",
  "node-mempool-lifecycle",
  "node-mempool-storage",
] as const;
const FORBIDDEN_CLAIMS = [
  "full knots rolling minimum fee decay",
  "knots mempool.dat binary compatibility",
  "relay serving",
  "relay fanout",
  "rebroadcast",
  "rpc/operator/support evidence",
  "support-bundle redaction",
  "release-boundary closeout",
  "compact block relay",
  "package relay",
  "bloom/filter serving",
  "public relay defaults",
  "public relay by default",
  "public-network relay ci",
  "production full-node readiness",
  "production service operation",
  "production-funds wallet use",
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
  "later",
  "remain",
  "remains",
  "no claim",
  "not claim",
  "not supported",
  "only",
  "require separate evidence",
  "requires separate evidence",
] as const;
const POSITIVE_CLAIM_PATTERNS = [
  /\bsupports?\b/,
  /\bprovides?\b/,
  /\benables?\b/,
  /\badds?\b/,
  /\bimplements?\b/,
  /\bships?\b/,
  /\bproves?\b/,
  /\bis supported\b/,
  /\bis enabled\b/,
  /\bis available\b/,
  /\bis complete\b/,
  /\bis ready\b/,
] as const;
const LATER_PHASE_MARKERS = Array.from({ length: 14 }, (_, index) => `phase ${104 + index}`);
const LATER_PHASE_OWNED_CLAIMS = new Set([
  "relay serving",
  "relay fanout",
  "rebroadcast",
  "rpc/operator/support evidence",
  "support-bundle redaction",
  "release-boundary closeout",
  "compact block relay",
]);

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type ParitySurface = {
  evidence?: unknown;
  id?: unknown;
  known_gaps?: unknown;
  requirements?: unknown;
  status?: unknown;
  suspected_unknowns?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type BreadcrumbGroup = { breadcrumbs?: unknown; files?: unknown; label?: unknown };

export function checkPhase103MempoolLifecycle(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE103_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  checkParitySurface(texts, failures);
  checkRequiredText(texts, failures);
  checkBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  checkVerifierOrder(texts.get("scripts/verify.sh") ?? "", failures);
  checkForbiddenClaims(texts, failures);

  return failures;
}

function checkParitySurface(texts: TextCorpus, failures: string[]): void {
  const raw = texts.get("docs/parity/index.json") ?? "";
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(raw) as ParityIndex;
  } catch (error) {
    failures.push(`docs/parity/index.json is not valid JSON: ${String(error)}`);
    return;
  }

  const surfaces = Array.isArray(parsed.checklist?.surfaces)
    ? (parsed.checklist.surfaces as ParitySurface[])
    : [];
  const maybeSurface = surfaces.find((surface) => surface.id === SURFACE_ID);
  if (!maybeSurface) {
    failures.push(`missing parity checklist surface ${SURFACE_ID}`);
    return;
  }
  if (maybeSurface.status !== "done") {
    failures.push(`${SURFACE_ID}: expected status done`);
  }

  const requirements = asStringArray(maybeSurface.requirements);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    if (!requirements.includes(requirement)) {
      failures.push(`${SURFACE_ID}: missing requirement ${requirement}`);
    }
  }

  const evidence = asStringArray(maybeSurface.evidence);
  for (const root of REQUIRED_EVIDENCE_ROOTS) {
    if (!evidence.includes(root)) {
      failures.push(`${SURFACE_ID}: missing evidence root ${root}`);
    }
  }

  const anchors = [
    ...asStringArray(maybeSurface.upstream?.sources),
    ...asStringArray(maybeSurface.upstream?.tests),
  ];
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!anchors.includes(anchor)) {
      failures.push(`${SURFACE_ID}: missing Knots anchor ${anchor}`);
    }
  }
}

function checkRequiredText(texts: TextCorpus, failures: string[]): void {
  const corpus = [...texts.values()].join("\n");
  for (const requirement of REQUIRED_REQUIREMENTS) {
    if (!corpus.includes(requirement)) {
      failures.push(`missing Phase 103 requirement ${requirement}`);
    }
  }
  for (const symbol of REQUIRED_SYMBOLS) {
    if (!corpus.includes(symbol)) {
      failures.push(`missing required Phase 103 symbol ${symbol}`);
    }
  }
  for (const testName of REQUIRED_BEHAVIOR_TESTS) {
    if (!corpus.includes(testName)) {
      failures.push(`missing required Phase 103 behavior test ${testName}`);
    }
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!corpus.includes(anchor)) {
      failures.push(`missing Phase 103 Knots anchor ${anchor}`);
    }
  }
}

function checkBreadcrumbs(raw: string, failures: string[]): void {
  let parsed: { groups?: unknown };
  try {
    parsed = JSON.parse(raw) as { groups?: unknown };
  } catch (error) {
    failures.push(`docs/parity/source-breadcrumbs.json is not valid JSON: ${String(error)}`);
    return;
  }

  const groups = Array.isArray(parsed.groups) ? (parsed.groups as BreadcrumbGroup[]) : [];
  for (const label of REQUIRED_BREADCRUMB_GROUPS) {
    const maybeGroup = groups.find((group) => group.label === label);
    if (!maybeGroup) {
      failures.push(`missing source breadcrumb group ${label}`);
      continue;
    }
    const files = asStringArray(maybeGroup.files);
    const breadcrumbs = asStringArray(maybeGroup.breadcrumbs);
    if (files.length === 0 || breadcrumbs.length === 0) {
      failures.push(`source breadcrumb group ${label} must map files to Knots anchors`);
    }
  }
}

function checkVerifierOrder(verifyText: string, failures: string[]): void {
  const phase102TestIndex = verifyText.indexOf(PHASE102_TEST_COMMAND);
  const phase102CheckerIndex = verifyText.indexOf(PHASE102_CHECKER_COMMAND);
  const phase103TestIndex = verifyText.indexOf(PHASE103_TEST_COMMAND);
  const phase103CheckerIndex = verifyText.indexOf(PHASE103_CHECKER_COMMAND);
  const pureCoreIndex = verifyText.indexOf(PURE_CORE_COMMAND);

  if (
    phase102TestIndex === -1 ||
    phase102CheckerIndex === -1 ||
    phase103TestIndex === -1 ||
    phase103CheckerIndex === -1 ||
    pureCoreIndex === -1 ||
    !(phase102TestIndex < phase102CheckerIndex) ||
    !(phase102CheckerIndex < phase103TestIndex) ||
    !(phase103TestIndex < phase103CheckerIndex) ||
    !(phase103CheckerIndex < pureCoreIndex)
  ) {
    failures.push("verifier-scope: Phase 103 checker must run after Phase 102 and before pure-core checks");
  }

  for (const forbidden of [
    "public-network relay",
    "public relay ci",
    "service-manager",
    "systemctl",
    "launchctl",
    "wall-clock",
    "production-deployment",
  ]) {
    if (verifyText.toLowerCase().includes(forbidden)) {
      failures.push(`verifier-scope forbidden default gate: ${forbidden}`);
    }
  }
}

function checkForbiddenClaims(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts.entries()) {
    if (!file.startsWith("docs/") && !file.startsWith(".planning/")) {
      continue;
    }
    for (const [lineIndex, line] of text.split("\n").entries()) {
      const lowerLine = line.toLowerCase();
      for (const forbidden of FORBIDDEN_CLAIMS) {
        if (!lowerLine.includes(forbidden)) {
          continue;
        }
        if (isExplicitLaterPhaseLine(lowerLine) && LATER_PHASE_OWNED_CLAIMS.has(forbidden)) {
          continue;
        }
        if (hasNoClaimMarker(lowerLine) || !hasPositiveClaim(lowerLine)) {
          continue;
        }
        failures.push(`${file}:${lineIndex + 1}: forbidden positive Phase 103 claim: ${forbidden}`);
      }
    }
  }
}

function isExplicitLaterPhaseLine(lowerLine: string): boolean {
  return !lowerLine.includes("phase 103")
    && LATER_PHASE_MARKERS.some((marker) => lowerLine.includes(marker));
}

function readText(repoRoot: string, filePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, filePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing target file ${filePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function asStringArray(value: unknown): string[] {
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
}

function hasNoClaimMarker(line: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => line.includes(marker));
}

function hasPositiveClaim(line: string): boolean {
  return POSITIVE_CLAIM_PATTERNS.some((patternValue) => patternValue.test(line));
}

if (import.meta.main) {
  const failures = checkPhase103MempoolLifecycle();
  if (failures.length > 0) {
    console.error("Phase 103 mempool lifecycle check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log("Phase 103 mempool lifecycle evidence validated.");
}
