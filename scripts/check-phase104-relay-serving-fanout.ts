#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-relay-serving-fanout-rebroadcast-policy";
const PHASE103_TEST_COMMAND = "bun test scripts/check-phase103-mempool-lifecycle.test.ts";
const PHASE103_CHECKER_COMMAND = "bun run scripts/check-phase103-mempool-lifecycle.ts";
const PHASE104_TEST_COMMAND = "bun test scripts/check-phase104-relay-serving-fanout.test.ts";
const PHASE104_CHECKER_COMMAND = "bun run scripts/check-phase104-relay-serving-fanout.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_REQUIREMENTS = ["REL-01", "REL-02", "REL-03", "REL-04"] as const;
const TARGET_FILES = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/serving_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs",
  "packages/open-bitcoin-node/src/network/relay_serving.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "scripts/check-phase104-relay-serving-fanout.ts",
  "scripts/check-phase104-relay-serving-fanout.test.ts",
  "scripts/verify.sh",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-01-SUMMARY.md",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-02-SUMMARY.md",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-03-SUMMARY.md",
] as const;
const REQUIRED_EVIDENCE_ROOTS = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/serving_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs",
  "packages/open-bitcoin-node/src/network/relay_serving.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "scripts/check-phase104-relay-serving-fanout.ts",
  "scripts/check-phase104-relay-serving-fanout.test.ts",
  "scripts/verify.sh",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-01-SUMMARY.md",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-02-SUMMARY.md",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-03-SUMMARY.md",
] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/node/txdownloadman.h",
  "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
  "packages/bitcoin-knots/src/protocol.h",
  "packages/bitcoin-knots/src/rpc/rawtransaction.cpp",
  "packages/bitcoin-knots/test/functional/p2p_getdata.py",
  "packages/bitcoin-knots/test/functional/p2p_tx_download.py",
] as const;
const REQUIRED_SYMBOLS = [
  "TxServeOutcomeLabel",
  "TxServingRecordStatus",
  "classify_tx_serve_request",
  "TxFanoutAction",
  "TxFanoutQueue",
  "PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER",
  "RelayServingCache",
  "ManagedRelayFanoutState",
  "LocalRelaySubmissionEvidence",
  "rebroadcast_deferred",
] as const;
const REQUIRED_BEHAVIOR_TESTS = [
  "tx_serving_policy_reports_low_cardinality_outcomes",
  "tx_fanout_policy_suppresses_origin_and_ineligible_peers",
  "tx_fanout_policy_reports_rebroadcast_deferred_without_timer",
  "managed_getdata_serves_only_accepted_relay_eligible_transaction",
  "managed_getdata_reports_unknown_confirmed_replaced_evicted_expired_notfound",
  "managed_getdata_preserves_block_serving_branch",
  "managed_fanout_announces_wtxid_to_wtxidrelay_peer",
  "managed_fanout_suppresses_origin_ineligible_and_recent_reject_peers",
  "managed_lifecycle_cleanup_removes_serving_and_fanout_state",
  "local_submission_records_queued_internal_relay_evidence",
  "local_submission_duplicate_rejected_or_orphaned_does_not_enqueue_fanout",
  "local_submission_records_rebroadcast_deferred_without_timer",
  "sendrawtransaction_queues_internal_relay_evidence_without_propagation_claim",
  "sendrawtransaction_duplicate_does_not_queue_new_fanout",
] as const;
const REQUIRED_BREADCRUMB_GROUPS = [
  "network-transaction-relay-download",
  "node-network-adapter",
  "rpc-surface",
] as const;
const FORBIDDEN_CLAIMS = [
  "periodic rebroadcast scheduling",
  "compact block relay",
  "package relay",
  "bloom/filter serving",
  "public relay defaults",
  "public relay by default",
  "public-network relay ci",
  "operator/rpc/metrics/log/support presentation",
  "support-bundle redaction",
  "release-boundary closeout",
  "production service operation",
  "production full-node readiness",
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
type ParityIndex = { checklist?: { surfaces?: unknown } };
type BreadcrumbGroup = { breadcrumbs?: unknown; files?: unknown; label?: unknown };

export function checkPhase104RelayServingFanout(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE104_REPO_ROOT ?? DEFAULT_REPO_ROOT,
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
      failures.push(`missing Phase 104 requirement ${requirement}`);
    }
  }
  for (const symbol of REQUIRED_SYMBOLS) {
    if (!corpus.includes(symbol)) {
      failures.push(`missing required Phase 104 symbol ${symbol}`);
    }
  }
  for (const testName of REQUIRED_BEHAVIOR_TESTS) {
    if (!corpus.includes(testName)) {
      failures.push(`missing required Phase 104 behavior test ${testName}`);
    }
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!corpus.includes(anchor)) {
      failures.push(`missing Phase 104 Knots anchor ${anchor}`);
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
  const visibleMarker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const visibleStart = verifyText.indexOf(visibleMarker);
  const visibleBodyStart = visibleStart + visibleMarker.length;
  const visibleEnd = verifyText.indexOf("\nVERIFY_COMMAND_ORDER", visibleBodyStart);
  const visibleText =
    visibleStart === -1 || visibleEnd === -1
      ? ""
      : verifyText.slice(visibleBodyStart, visibleEnd);
  if (
    !orderedIndexes(visibleText, [
      PHASE103_TEST_COMMAND,
      PHASE103_CHECKER_COMMAND,
      PHASE104_TEST_COMMAND,
      PHASE104_CHECKER_COMMAND,
    ])
  ) {
    failures.push("verifier-scope: Phase 104 visible order must follow Phase 103");
  }

  if (
    !orderedIndexes(verifyText, [
      'run_step "test Phase 103 mempool lifecycle checker"',
      'run_step "check Phase 103 mempool lifecycle"',
      'run_step "test Phase 104 relay serving/fanout checker"',
      'run_step "check Phase 104 relay serving/fanout"',
      'run_step "check pure-core dependencies"',
    ])
  ) {
    failures.push("verifier-scope: Phase 104 executable order must follow Phase 103 and precede pure-core checks");
  }
}

function checkForbiddenClaims(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts.entries()) {
    if (!file.startsWith("docs/") && !file.startsWith(".planning/")) {
      continue;
    }
    for (const paragraph of markdownParagraphs(text)) {
      const lowerText = paragraph.text.toLowerCase();
      for (const forbidden of FORBIDDEN_CLAIMS) {
        if (!lowerText.includes(forbidden)) {
          continue;
        }
        if (hasNoClaimMarker(lowerText) || !hasPositiveClaim(lowerText)) {
          continue;
        }
        failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 104 claim: ${forbidden}`);
      }
    }
  }
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

function orderedIndexes(text: string, needles: readonly string[]): boolean {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle, cursor + 1);
    if (index === -1) {
      return false;
    }
    cursor = index;
  }
  return true;
}

function markdownParagraphs(text: string): Array<{ startLine: number; text: string }> {
  const paragraphs: Array<{ startLine: number; text: string }> = [];
  let startLine = 1;
  let current: string[] = [];
  for (const [index, line] of text.split("\n").entries()) {
    if (line.trim() === "") {
      if (current.length > 0) {
        paragraphs.push({ startLine, text: current.join(" ") });
        current = [];
      }
      startLine = index + 2;
      continue;
    }
    if (current.length === 0) {
      startLine = index + 1;
    }
    current.push(line);
  }
  if (current.length > 0) {
    paragraphs.push({ startLine, text: current.join(" ") });
  }
  return paragraphs;
}

function hasNoClaimMarker(line: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => line.includes(marker));
}

function hasPositiveClaim(line: string): boolean {
  return POSITIVE_CLAIM_PATTERNS.some((patternValue) => patternValue.test(line));
}

if (import.meta.main) {
  const failures = checkPhase104RelayServingFanout();
  if (failures.length > 0) {
    console.error("Phase 104 relay serving/fanout check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log("Phase 104 relay serving/fanout evidence validated.");
}
