import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v2-0-orphan-handling-admission-outcome-bridge";
export const PHASE101_TEST_COMMAND =
  "bun test scripts/check-phase101-transaction-inventory-download-scheduling.test.ts";
export const PHASE101_CHECKER_COMMAND =
  "bun run scripts/check-phase101-transaction-inventory-download-scheduling.ts";
export const PHASE102_TEST_COMMAND = "bun test scripts/check-phase102-orphan-admission-bridge.test.ts";
export const PHASE102_CHECKER_COMMAND = "bun run scripts/check-phase102-orphan-admission-bridge.ts";
export const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
export const REQUIRED_REQUIREMENTS = ["DL-03", "DL-04", "DL-05", "MEM-01", "MEM-02"] as const;
export const TARGET_FILES = [
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
export const REQUIRED_EVIDENCE_ROOTS = [
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
export const REQUIRED_KNOTS_ANCHORS = [
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
export const REQUIRED_OUTCOME_LABELS = [
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
export const REQUIRED_ORPHAN_LABELS = [
  "TxOrphanage",
  "OrphanPolicy",
  "OrphanEvidenceLabel",
  "parent_requested",
  "orphan_evicted",
  "orphan_expired",
  "orphan_reconsidered",
] as const;
export const REQUIRED_CONSTANTS = [
  "PHASE102_MAX_ORPHAN_TRANSACTIONS",
  "PHASE102_MAX_ORPHANS_PER_PEER",
  "PHASE102_ORPHAN_TTL_SECONDS",
  "PHASE102_MAX_RECONSIDERATIONS_PER_PARENT",
] as const;
export const REQUIRED_BRIDGE_SYMBOLS = [
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
export const REQUIRED_BEHAVIOR_TESTS = [
  "no_partial_mutation_for_low_fee_rejection",
  "missing_parent_stage_requests_each_unique_parent_by_txid",
  "orphan_parent_request_suppresses_duplicate_pending_parent_with_fallback",
  "peer_manager_orphan_parent_request_respects_inflight_cap",
  "managed_admission_bridge_parent_acceptance_reconsiders_child",
  "managed_admission_bridge_drains_ready_orphans_after_reconsideration_cap",
  "managed_admission_bridge_disconnect_cleans_peer_orphans_and_request_state",
] as const;
export const REQUIRED_BREADCRUMB_GROUPS = [
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
export const FORBIDDEN_CLAIMS = [
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
export const NO_CLAIM_MARKERS = [
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
export const POSITIVE_CLAIM_PATTERNS = [
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
export const FORBIDDEN_VERIFIER_SCOPE = [
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

export type TargetFile = (typeof TARGET_FILES)[number];
export type TextCorpus = Map<TargetFile, string>;
export type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
export type ParitySurface = {
  evidence?: unknown;
  id?: unknown;
  known_gaps?: unknown;
  name?: unknown;
  requirements?: unknown;
  status?: unknown;
  suspected_unknowns?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
export type BreadcrumbGroup = { breadcrumbs?: unknown; files?: unknown; label?: unknown };
