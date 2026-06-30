#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-transaction-inventory-download-scheduling";
const PHASE100_TEST_COMMAND =
  "bun test scripts/check-phase100-relay-activation-boundary.test.ts";
const PHASE100_CHECKER_COMMAND =
  "bun run scripts/check-phase100-relay-activation-boundary.ts";
const PHASE101_TEST_COMMAND =
  "bun test scripts/check-phase101-transaction-inventory-download-scheduling.test.ts";
const PHASE101_CHECKER_COMMAND =
  "bun run scripts/check-phase101-transaction-inventory-download-scheduling.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_REQUIREMENTS = ["INV-01", "INV-02", "INV-03", "INV-04", "DL-01", "DL-02"] as const;
const TARGET_FILES = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs",
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-network/src/peer/inventory_state.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/tests.rs",
  "scripts/verify.sh",
] as const;
const REQUIRED_EVIDENCE_ROOTS = [
  "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs",
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-network/src/peer/inventory_state.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/tests.rs",
  "docs/parity/source-breadcrumbs.json",
  ".planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-01-SUMMARY.md",
  ".planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-02-SUMMARY.md",
] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/protocol.h",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/node/txdownloadman.h",
  "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
  "packages/bitcoin-knots/src/txrequest.h",
  "packages/bitcoin-knots/src/txrequest.cpp",
  "packages/bitcoin-knots/test/functional/p2p_tx_download.py",
  "packages/bitcoin-knots/test/functional/p2p_getdata.py",
] as const;
const REQUIRED_TYPES = [
  "TxRelayId",
  "TxRelayPeerMode",
  "TxDownloadScheduler",
  "TxDownloadPolicy",
  "TxDownloadLocalFacts",
  "TxDownloadAction",
  "TxDownloadSuppressionReason",
  "PeerAction::TransactionRelay",
] as const;
const REQUIRED_CONSTANTS = [
  "PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER",
  "PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER",
  "PHASE101_TXID_RELAY_DELAY_SECONDS",
  "PHASE101_NONPREF_PEER_TX_DELAY_SECONDS",
  "PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS",
  "PHASE101_GETDATA_TX_INTERVAL_SECONDS",
] as const;
const REQUIRED_ACTION_LABELS = [
  "request_getdata",
  "suppress_duplicate",
  "suppress_already_have",
  "suppress_recent_reject",
  "suppress_mempool_known",
  "suppress_identity_mismatch",
  "suppress_request_cap",
  "fallback_request",
  "request_expired",
  "notfound_cleanup",
  "received_tx_cleanup",
  "peer_cleanup",
] as const;
const REQUIRED_BEHAVIOR_TESTS = [
  "txid_announcement_requests_transaction_inventory",
  "wtxid_announcement_requests_witness_transaction_inventory",
  "identity_mismatch_suppresses_without_candidate_or_inflight_state",
  "duplicate_announcement_retains_fallback_candidate_without_second_request",
  "already_have_recent_reject_and_mempool_known_suppress_requests",
  "inflight_cap_suppresses_additional_ready_requests",
  "txid_delay_waits_until_fake_clock_reaches_ready_time",
  "non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time",
  "overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time",
  "expiry_fallback_waits_until_fake_clock_reaches_getdata_interval",
  "timeout_expires_request_and_falls_back_to_duplicate_announcer",
  "notfound_clears_matching_request_and_falls_back",
  "disconnect_cleanup_removes_peer_state_and_falls_back",
  "received_transaction_cleanup_marks_txid_and_wtxid_already_have",
  "peer_manager_transaction_relay",
  "managed_network_transaction_relay",
] as const;
const FORBIDDEN_CLAIMS = [
  "orphan handling",
  "parent request behavior",
  "mempool admission",
  "standardness",
  "fee policy",
  "rbf",
  "ancestor policy",
  "descendant policy",
  "mempool lifecycle",
  "mempool persistence",
  "relay serving",
  "relay fanout",
  "rebroadcast",
  "compact block relay",
  "bloom filter serving",
  "bloom/filter serving",
  "compact filter serving",
  "package relay",
  "public relay by default",
  "public-network relay ci",
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
  "only;",
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
const FORBIDDEN_VERIFIER_SCOPE = [
  "public-network relay",
  "public relay ci",
  "sleep ",
  "service-manager",
  "systemctl",
  "launchctl",
  "wall-clock",
  "production-deployment",
  "production full-node readiness",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type ParitySurface = {
  evidence?: unknown;
  id?: unknown;
  known_gaps?: unknown;
  name?: unknown;
  requirements?: unknown;
  status?: unknown;
  suspected_unknowns?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type BreadcrumbGroup = { breadcrumbs?: unknown; files?: unknown; label?: unknown };

export function checkPhase101TransactionInventoryDownloadScheduling(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE101_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyParityDocs(texts, failures);
  verifySchedulerEvidence(texts, failures);
  verifyBehaviorTests(texts, failures);
  verifySourceBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  verifyNoBareRequestSets(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`Phase 101 missing required corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`Phase 101 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("Phase 101 parity index surfaces must be an array");
    return;
  }

  const matches = parsed.surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`Phase 101 parity index must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }
  if (matches[0]?.status !== "done") {
    failures.push(`Phase 101 parity index surface must be done: ${SURFACE_ID}`);
  }
}

function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("Phase 101 parity checklist surfaces must be an array");
    return;
  }

  const matches = surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`Phase 101 parity checklist must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }

  const surface = matches[0]!;
  if (surface.status !== "done") {
    failures.push(`Phase 101 checklist surface must be done: ${SURFACE_ID}`);
  }
  requireExactRequirements(surface.requirements, REQUIRED_REQUIREMENTS, "Phase 101 checklist", failures);
  for (const root of REQUIRED_EVIDENCE_ROOTS) {
    requireArrayIncludes(surface.evidence, root, `Phase 101 evidence root missing: ${root}`, failures);
  }
  for (const source of REQUIRED_KNOTS_ANCHORS.slice(0, 6)) {
    requireArrayIncludes(surface.upstream?.sources, source, `Phase 101 Knots source missing: ${source}`, failures);
  }
  for (const test of REQUIRED_KNOTS_ANCHORS.slice(6)) {
    requireArrayIncludes(surface.upstream?.tests, test, `Phase 101 Knots test missing: ${test}`, failures);
  }
}

function verifyParityDocs(texts: TextCorpus, failures: string[]): void {
  const docs = [
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/index.json") ?? "",
  ].join("\n");

  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireContains(docs, requirement, `Phase 101 requirement missing: ${requirement}`, failures);
  }
  requireContains(docs, SURFACE_ID, `Phase 101 surface id missing: ${SURFACE_ID}`, failures);
  for (const root of REQUIRED_EVIDENCE_ROOTS) {
    requireContains(docs, root, `Phase 101 evidence root missing: ${root}`, failures);
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireContains(docs, anchor, `Phase 101 Knots anchor missing: ${anchor}`, failures);
  }
  for (const label of [...REQUIRED_TYPES, ...REQUIRED_CONSTANTS]) {
    requireContains(docs, label, `Phase 101 docs label missing: ${label}`, failures);
  }
  for (const actionLabel of REQUIRED_ACTION_LABELS) {
    requireContains(docs, actionLabel, `required action label missing: ${actionLabel}`, failures);
  }
  requireContains(
    docs,
    "Phase 101 does not claim orphan handling, parent request behavior, mempool admission outcomes",
    "Phase 101 no-claim boundary sentence missing",
    failures,
  );
}

function verifySchedulerEvidence(texts: TextCorpus, failures: string[]): void {
  const source = [
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/inventory_state.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/network.rs") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/index.json") ?? "",
  ].join("\n");

  for (const label of REQUIRED_TYPES) {
    requireContains(source, label, `missing scheduler evidence: ${label}`, failures);
  }
  for (const constant of REQUIRED_CONSTANTS) {
    requireContains(source, constant, `missing scheduler evidence: ${constant}`, failures);
  }
  for (const actionLabel of REQUIRED_ACTION_LABELS) {
    requireContains(source, actionLabel, `required action label missing: ${actionLabel}`, failures);
  }
  for (const needle of [
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs",
    "TxDownloadSuppressionReason::MempoolKnown",
    "mempool_known",
    "Suppress {",
    "peer_id",
    "relay_id",
    "reason",
    "TxRelayPeerMode::from_remote_wtxidrelay",
    "record_notfound",
    "record_received_transaction",
    "TxRelayId::Txid",
    "TxRelayId::Wtxid",
    "process_transaction_relay_action",
  ]) {
    requireContains(source, needle, `missing scheduler evidence: ${needle}`, failures);
  }
}

function verifyBehaviorTests(texts: TextCorpus, failures: string[]): void {
  const tests = [
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/network/tests.rs") ?? "",
  ].join("\n");

  for (const testName of REQUIRED_BEHAVIOR_TESTS) {
    requireContains(tests, testName, `required behavior test missing: ${testName}`, failures);
  }
}

function verifySourceBreadcrumbs(text: string, failures: string[]): void {
  let parsed: { groups?: unknown };
  try {
    parsed = JSON.parse(text) as { groups?: unknown };
  } catch (error) {
    failures.push(`Phase 101 source breadcrumb JSON parse failed: ${String(error)}`);
    return;
  }

  if (!Array.isArray(parsed.groups)) {
    failures.push("Phase 101 source breadcrumb groups must be an array");
    return;
  }

  const matches = parsed.groups.filter((entry) => {
    const maybeGroup = entry as BreadcrumbGroup;
    return maybeGroup.label === "network-transaction-relay-download";
  }) as BreadcrumbGroup[];
  if (matches.length !== 1) {
    failures.push("source breadcrumb missing group: network-transaction-relay-download");
    return;
  }

  const group = matches[0]!;
  for (const file of [
    "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs",
  ]) {
    requireArrayIncludes(group.files, file, `source breadcrumb missing file: ${file}`, failures);
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireArrayIncludes(group.breadcrumbs, anchor, `source breadcrumb missing Knots anchor: ${anchor}`, failures);
  }
}

function verifyNoBareRequestSets(texts: TextCorpus, failures: string[]): void {
  for (const file of [
    "packages/open-bitcoin-network/src/peer.rs",
    "packages/open-bitcoin-network/src/peer/inventory_state.rs",
    "packages/open-bitcoin-network/src/peer/tests.rs",
  ] as const) {
    const text = texts.get(file) ?? "";
    for (const pattern of [/requested_txids\s*:/, /requested_wtxids\s*:/]) {
      if (pattern.test(text)) {
        failures.push(`Phase 101 reintroduced bare transaction request set in ${file}`);
      }
    }
  }
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("verifier-scope missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [
        PHASE100_TEST_COMMAND,
        PHASE100_CHECKER_COMMAND,
        PHASE101_TEST_COMMAND,
        PHASE101_CHECKER_COMMAND,
      ],
      "verifier-scope visible order must place Phase 101 immediately after Phase 100",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    text,
    "Phase 100 is followed by Phase 101",
    "verifier-scope ordering comment missing Phase 101",
    failures,
  );
  requireContains(
    executableText,
    `run_step "test Phase 101 transaction inventory download scheduling checker" ${PHASE101_TEST_COMMAND}`,
    "verifier-scope executable Phase 101 checker tests",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 101 transaction inventory download scheduling" ${PHASE101_CHECKER_COMMAND}`,
    "verifier-scope executable Phase 101 checker",
    failures,
  );
  verifyOrderedCommands(
    executableText,
    [
      PHASE100_TEST_COMMAND,
      PHASE100_CHECKER_COMMAND,
      PHASE101_TEST_COMMAND,
      PHASE101_CHECKER_COMMAND,
      PURE_CORE_COMMAND,
    ],
    "verifier-scope executable order must run Phase 101 after Phase 100 and before pure-core checks",
    failures,
  );

  for (const line of executableText.split(/\r?\n/)) {
    const lower = line.toLowerCase();
    if (!lower.includes("phase 101") && !lower.includes("check-phase101")) {
      continue;
    }
    for (const forbidden of FORBIDDEN_VERIFIER_SCOPE) {
      if (lower.includes(forbidden)) {
        failures.push(`verifier-scope forbidden Phase 101 gate '${forbidden}': ${line.trim()}`);
      }
    }
  }
}

function verifyNoClaimBoundary(texts: TextCorpus, failures: string[]): void {
  const docs = [
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/index.json") ?? "",
  ].join("\n");

  const units = docs.replace(/\s+/g, " ").split(/(?<=[.!?])\s+/);
  for (const unit of units) {
    const lower = unit.toLowerCase();
    for (const claim of FORBIDDEN_CLAIMS) {
      if (!lower.includes(claim)) {
        continue;
      }
      if (hasNoClaimMarker(lower)) {
        continue;
      }
      if (POSITIVE_CLAIM_PATTERNS.some((pattern) => pattern.test(lower))) {
        failures.push(`no-claim boundary violation for ${claim}: ${unit.trim()}`);
      }
    }
  }
}

function requireExactRequirements(
  value: unknown,
  expected: readonly string[],
  label: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const wanted = JSON.stringify(expected);
  if (actual !== wanted) {
    failures.push(`${label} requirements mismatch: expected ${wanted}, got ${actual}`);
  }
}

function requireArrayIncludes(
  value: unknown,
  needle: string,
  message: string,
  failures: string[],
): void {
  if (!Array.isArray(value) || !value.includes(needle)) {
    failures.push(message);
  }
}

function requireContains(
  text: string,
  needle: string,
  message: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(message);
  }
}

function verifyOrderedCommands(
  text: string,
  commands: readonly string[],
  label: string,
  failures: string[],
): void {
  let lastIndex = -1;
  for (const command of commands) {
    const index = text.indexOf(command);
    if (index < 0) {
      failures.push(`${label}: missing command ${command}`);
      return;
    }
    if (index <= lastIndex) {
      failures.push(`${label}: command out of order ${command}`);
      return;
    }
    lastIndex = index;
  }
}

function executableVerifyText(text: string): string {
  return text.replace(/^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m, "");
}

function hasNoClaimMarker(lowerUnit: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => lowerUnit.includes(marker));
}

if (import.meta.main) {
  const failures = checkPhase101TransactionInventoryDownloadScheduling();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }
  console.log("validated Phase 101 transaction inventory download scheduling evidence");
}
