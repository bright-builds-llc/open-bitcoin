#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-orphan-handling-admission-outcome-bridge";
const PHASE101_TEST_COMMAND =
  "bun test scripts/check-phase101-transaction-inventory-download-scheduling.test.ts";
const PHASE101_CHECKER_COMMAND =
  "bun run scripts/check-phase101-transaction-inventory-download-scheduling.ts";
const PHASE102_TEST_COMMAND = "bun test scripts/check-phase102-orphan-admission-bridge.test.ts";
const PHASE102_CHECKER_COMMAND = "bun run scripts/check-phase102-orphan-admission-bridge.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_REQUIREMENTS = ["DL-03", "DL-04", "DL-05", "MEM-01", "MEM-02"] as const;
const TARGET_FILES = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-mempool/src/outcome.rs",
  "packages/open-bitcoin-mempool/src/pool.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs",
  "packages/open-bitcoin-node/src/mempool.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs",
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-network/src/peer/inventory_state.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "packages/open-bitcoin-node/src/network/action_translation.rs",
  "packages/open-bitcoin-node/src/network/admission_bridge.rs",
  "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs",
  "scripts/verify.sh",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-01-SUMMARY.md",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-02-SUMMARY.md",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-03-SUMMARY.md",
] as const;
const REQUIRED_EVIDENCE_ROOTS = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-mempool/src/outcome.rs",
  "packages/open-bitcoin-mempool/src/pool.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs",
  "packages/open-bitcoin-node/src/mempool.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "packages/open-bitcoin-node/src/network/action_translation.rs",
  "packages/open-bitcoin-node/src/network/admission_bridge.rs",
  "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs",
  "scripts/check-phase102-orphan-admission-bridge.ts",
  "scripts/check-phase102-orphan-admission-bridge.test.ts",
  "scripts/verify.sh",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-01-SUMMARY.md",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-02-SUMMARY.md",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-03-SUMMARY.md",
] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/txorphanage.h",
  "packages/bitcoin-knots/src/txorphanage.cpp",
  "packages/bitcoin-knots/src/node/txdownloadman.h",
  "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/validation.cpp",
  "packages/bitcoin-knots/src/txmempool.cpp",
  "packages/bitcoin-knots/src/policy/policy.cpp",
  "packages/bitcoin-knots/src/policy/rbf.cpp",
  "packages/bitcoin-knots/test/functional/p2p_orphan_handling.py",
  "packages/bitcoin-knots/test/functional/mempool_accept.py",
  "packages/bitcoin-knots/test/functional/feature_rbf.py",
] as const;
const REQUIRED_OUTCOME_LABELS = [
  "MempoolOutcome",
  "MempoolOutcomeLabel",
  "MempoolRejectionCategory",
  "accepted",
  "rejected",
  "duplicate",
  "replaced",
  "orphaned",
  "evicted",
  "expired",
] as const;
const REQUIRED_ORPHAN_LABELS = [
  "TxOrphanage",
  "OrphanPolicy",
  "OrphanEvidenceLabel",
  "parent_requested",
  "orphan_evicted",
  "orphan_expired",
  "orphan_reconsidered",
] as const;
const REQUIRED_CONSTANTS = [
  "PHASE102_MAX_ORPHAN_TRANSACTIONS",
  "PHASE102_MAX_ORPHANS_PER_PEER",
  "PHASE102_ORPHAN_TTL_SECONDS",
  "PHASE102_MAX_RECONSIDERATIONS_PER_PARENT",
] as const;
const REQUIRED_BRIDGE_SYMBOLS = [
  "request_orphan_parent",
  "process_peer_transaction_admission",
  "submit_transaction_outcome",
  "accept_transaction_outcome",
  "reconsider_orphans_after_acceptance",
  "expire_orphan_transactions",
  "remove_stored_transactions",
  "disconnect_peer_at",
  "cleanup_peer",
] as const;
const REQUIRED_BEHAVIOR_TESTS = [
  "no_partial_mutation_for_low_fee_rejection",
  "missing_parent_stage_requests_each_unique_parent_by_txid",
  "orphan_parent_request_suppresses_duplicate_pending_parent_with_fallback",
  "peer_manager_orphan_parent_request_respects_inflight_cap",
  "managed_admission_bridge_parent_acceptance_reconsiders_child",
  "managed_admission_bridge_drains_ready_orphans_after_reconsideration_cap",
  "managed_admission_bridge_disconnect_cleans_peer_orphans_and_request_state",
] as const;
const REQUIRED_BREADCRUMB_GROUPS = [
  {
    anchors: [
      "packages/bitcoin-knots/src/txmempool.h",
      "packages/bitcoin-knots/src/txmempool.cpp",
      "packages/bitcoin-knots/src/policy/policy.h",
      "packages/bitcoin-knots/src/policy/rbf.cpp",
    ],
    files: [
      "packages/open-bitcoin-mempool/src/outcome.rs",
      "packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs",
    ],
    label: "mempool-policy",
  },
  {
    anchors: [
      "packages/bitcoin-knots/src/protocol.h",
      "packages/bitcoin-knots/src/net_processing.cpp",
      "packages/bitcoin-knots/src/node/txdownloadman.h",
      "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
      "packages/bitcoin-knots/src/txorphanage.h",
      "packages/bitcoin-knots/src/txorphanage.cpp",
      "packages/bitcoin-knots/test/functional/p2p_orphan_handling.py",
    ],
    files: [
      "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
      "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
    ],
    label: "network-transaction-relay-download",
  },
  {
    anchors: [
      "packages/bitcoin-knots/src/net_processing.cpp",
      "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
      "packages/bitcoin-knots/src/node/txdownloadman.h",
      "packages/bitcoin-knots/src/protocol.h",
      "packages/bitcoin-knots/src/txorphanage.cpp",
      "packages/bitcoin-knots/src/validation.cpp",
      "packages/bitcoin-knots/test/functional/p2p_orphan_handling.py",
      "packages/bitcoin-knots/test/functional/mempool_accept.py",
    ],
    files: [
      "packages/open-bitcoin-node/src/network/action_translation.rs",
      "packages/open-bitcoin-node/src/network/admission_bridge.rs",
      "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs",
    ],
    label: "node-network-adapter",
  },
] as const;
const FORBIDDEN_CLAIMS = [
  "durable mempool persistence",
  "block connect/disconnect mempool lifecycle",
  "long-lived mempool pressure",
  "mempool pressure/trimming",
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

export function checkPhase102OrphanAdmissionBridge(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE102_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyParityDocs(texts, failures);
  verifyOutcomeAndOrphanEvidence(texts, failures);
  verifyManagedBridgeEvidence(texts, failures);
  verifyBehaviorTests(texts, failures);
  verifySourceBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`Phase 102 missing required corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`Phase 102 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("Phase 102 parity index surfaces must be an array");
    return;
  }

  const matches = parsed.surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`Phase 102 parity index must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }
  if (matches[0]?.status !== "done") {
    failures.push(`Phase 102 parity index surface must be done: ${SURFACE_ID}`);
  }
}

function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("Phase 102 parity checklist surfaces must be an array");
    return;
  }

  const matches = surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`Phase 102 parity checklist must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }

  const surface = matches[0]!;
  if (surface.status !== "done") {
    failures.push(`Phase 102 checklist surface must be done: ${SURFACE_ID}`);
  }
  requireExactRequirements(surface.requirements, REQUIRED_REQUIREMENTS, "Phase 102 checklist", failures);
  for (const root of REQUIRED_EVIDENCE_ROOTS) {
    requireArrayIncludes(surface.evidence, root, `Phase 102 evidence root missing: ${root}`, failures);
  }
  for (const source of REQUIRED_KNOTS_ANCHORS.slice(0, 9)) {
    requireArrayIncludes(surface.upstream?.sources, source, `Phase 102 Knots source missing: ${source}`, failures);
  }
  for (const test of REQUIRED_KNOTS_ANCHORS.slice(9)) {
    requireArrayIncludes(surface.upstream?.tests, test, `Phase 102 Knots test missing: ${test}`, failures);
  }
}

function verifyParityDocs(texts: TextCorpus, failures: string[]): void {
  const docs = [
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/index.json") ?? "",
  ].join("\n");

  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireContains(docs, requirement, `Phase 102 requirement missing: ${requirement}`, failures);
  }
  requireContains(docs, SURFACE_ID, `Phase 102 surface id missing: ${SURFACE_ID}`, failures);
  for (const root of REQUIRED_EVIDENCE_ROOTS) {
    requireContains(docs, root, `Phase 102 evidence root missing: ${root}`, failures);
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireContains(docs, anchor, `Phase 102 Knots anchor missing: ${anchor}`, failures);
  }
  for (const label of [
    ...REQUIRED_OUTCOME_LABELS,
    ...REQUIRED_ORPHAN_LABELS,
    ...REQUIRED_CONSTANTS,
    ...REQUIRED_BRIDGE_SYMBOLS,
  ]) {
    requireContains(docs, label, `Phase 102 docs label missing: ${label}`, failures);
  }
  requireContains(
    docs,
    "Phase 102 does not claim durable mempool persistence",
    "Phase 102 no-claim boundary sentence missing",
    failures,
  );
}

function verifyOutcomeAndOrphanEvidence(texts: TextCorpus, failures: string[]): void {
  const source = [
    texts.get("packages/open-bitcoin-mempool/src/outcome.rs") ?? "",
    texts.get("packages/open-bitcoin-mempool/src/pool.rs") ?? "",
    texts.get("packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/mempool.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/inventory_state.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/network/admission_bridge.rs") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/index.json") ?? "",
  ].join("\n");

  for (const label of REQUIRED_OUTCOME_LABELS) {
    requireContains(source, label, `required outcome label missing: ${label}`, failures);
  }
  for (const label of REQUIRED_ORPHAN_LABELS) {
    requireContains(source, label, `required orphan label missing: ${label}`, failures);
  }
  for (const constant of REQUIRED_CONSTANTS) {
    requireContains(source, constant, `required cap constant missing: ${constant}`, failures);
  }
  for (const symbol of REQUIRED_BRIDGE_SYMBOLS) {
    requireContains(source, symbol, `required bridge symbol missing: ${symbol}`, failures);
  }
  for (const needle of [
    "stage_missing_parent",
    "reconsider_after_parent",
    "OrphanReconsiderationStatus",
    "TxRelayId::Txid",
    "remove_stored_transactions",
  ]) {
    requireContains(source, needle, `Phase 102 orphan/admission evidence missing: ${needle}`, failures);
  }
}

function verifyManagedBridgeEvidence(texts: TextCorpus, failures: string[]): void {
  const actionTranslation = texts.get("packages/open-bitcoin-node/src/network/action_translation.rs") ?? "";
  const admissionBridge = texts.get("packages/open-bitcoin-node/src/network/admission_bridge.rs") ?? "";
  const peerManager = texts.get("packages/open-bitcoin-network/src/peer.rs") ?? "";
  const peerInventoryState =
    texts.get("packages/open-bitcoin-network/src/peer/inventory_state.rs") ?? "";
  const peerManagerBridge = `${peerManager}\n${peerInventoryState}`;

  verifyOrderedCommands(
    actionTranslation,
    [
      "pub fn disconnect_peer_at",
      "remove_peer_with_transaction_cleanup(peer_id, now_unix_seconds)?",
      "self.known_peers.remove(&peer_id);",
    ],
    "Phase 102 managed disconnect cleanup order",
    failures,
  );
  verifyOrderedCommands(
    peerInventoryState,
    [
      "pub fn remove_peer_with_transaction_cleanup",
      "self.peers.remove(&peer_id)",
      "self.orphanage.cleanup_peer(peer_id);",
      "self.tx_download.cleanup_peer(peer_id, now_unix_seconds)",
    ],
    "Phase 102 PeerManager disconnect cleanup order",
    failures,
  );
  if (actionTranslation.includes("self.orphanage")) {
    failures.push("Phase 102 node action translation must delegate orphan cleanup to PeerManager");
  }
  const maybeAdmissionCommand = [
    "submit_transaction_transition_with_context",
    "submit_transaction_outcome",
  ].find((command) => admissionBridge.includes(command));
  if (maybeAdmissionCommand === undefined) {
    failures.push(
      "Phase 102 admission bridge must use an outcome or transition admission command",
    );
  } else {
    verifyOrderedCommands(
      admissionBridge,
      [
        "process_peer_transaction_admission",
        maybeAdmissionCommand,
        "stage_missing_parent",
        "request_orphan_parent",
      ],
      "Phase 102 admission bridge must stage before scheduler-mediated parent requests",
      failures,
    );
  }
  requireContains(
    peerManagerBridge,
    "request_orphan_parent",
    "Phase 102 PeerManager parent request bridge missing",
    failures,
  );
  for (const forbidden of [
    "submit_transaction_transition_with_context",
    "submit_transaction_outcome",
    "accept_transaction_outcome",
    "process_peer_transaction_admission",
    "stage_missing_parent(",
  ]) {
    if (peerManagerBridge.includes(forbidden)) {
      failures.push(`Phase 102 peer/socket code mutates mempool or orphanage directly: ${forbidden}`);
    }
  }
}

function verifyBehaviorTests(texts: TextCorpus, failures: string[]): void {
  const tests = [
    texts.get("packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs") ?? "",
    texts.get(".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-01-SUMMARY.md") ?? "",
    texts.get(".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-02-SUMMARY.md") ?? "",
    texts.get(".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-03-SUMMARY.md") ?? "",
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
    failures.push(`Phase 102 source breadcrumb JSON parse failed: ${String(error)}`);
    return;
  }

  if (!Array.isArray(parsed.groups)) {
    failures.push("Phase 102 source breadcrumb groups must be an array");
    return;
  }

  for (const expectedGroup of REQUIRED_BREADCRUMB_GROUPS) {
    const matches = parsed.groups.filter((entry) => {
      const maybeGroup = entry as BreadcrumbGroup;
      return maybeGroup.label === expectedGroup.label;
    }) as BreadcrumbGroup[];
    if (matches.length !== 1) {
      failures.push(`source breadcrumb missing group: ${expectedGroup.label}`);
      continue;
    }

    const group = matches[0]!;
    for (const file of expectedGroup.files) {
      requireArrayIncludes(group.files, file, `source breadcrumb missing file: ${file}`, failures);
    }
    for (const anchor of expectedGroup.anchors) {
      requireArrayIncludes(group.breadcrumbs, anchor, `source breadcrumb missing Knots anchor: ${anchor}`, failures);
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
      [PHASE101_TEST_COMMAND, PHASE101_CHECKER_COMMAND, PHASE102_TEST_COMMAND, PHASE102_CHECKER_COMMAND],
      "verifier-scope visible order must place Phase 102 immediately after Phase 101",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    text,
    "Phase 101 is followed by Phase 102",
    "verifier-scope ordering comment missing Phase 102",
    failures,
  );
  requireContains(
    executableText,
    `run_step "test Phase 102 orphan admission bridge checker" ${PHASE102_TEST_COMMAND}`,
    "verifier-scope executable Phase 102 checker tests",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 102 orphan admission bridge" ${PHASE102_CHECKER_COMMAND}`,
    "verifier-scope executable Phase 102 checker",
    failures,
  );
  verifyOrderedCommands(
    executableText,
    [
      PHASE101_TEST_COMMAND,
      PHASE101_CHECKER_COMMAND,
      PHASE102_TEST_COMMAND,
      PHASE102_CHECKER_COMMAND,
      PURE_CORE_COMMAND,
    ],
    "verifier-scope executable order must run Phase 102 after Phase 101 and before pure-core checks",
    failures,
  );

  for (const line of executableText.split(/\r?\n/)) {
    const lower = line.toLowerCase();
    if (!lower.includes("phase 102") && !lower.includes("check-phase102")) {
      continue;
    }
    for (const forbidden of FORBIDDEN_VERIFIER_SCOPE) {
      if (lower.includes(forbidden)) {
        failures.push(`verifier-scope forbidden Phase 102 gate '${forbidden}': ${line.trim()}`);
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
  const failures = checkPhase102OrphanAdmissionBridge();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }
  console.log("validated Phase 102 orphan handling admission outcome bridge evidence");
}
