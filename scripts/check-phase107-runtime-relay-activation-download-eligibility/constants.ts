import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v2-0-runtime-relay-activation-download-eligibility";
export const PHASE106_TEST_COMMAND = "bun test scripts/check-phase106-parity-uat-release-boundary.test.ts";
export const PHASE106_CHECKER_COMMAND = "bun run scripts/check-phase106-parity-uat-release-boundary.ts";
export const PHASE107_TEST_COMMAND =
  "bun test scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts";
export const PHASE107_CHECKER_COMMAND =
  "bun run scripts/check-phase107-runtime-relay-activation-download-eligibility.ts";
export const REQUIRED_REQUIREMENTS = ["ACT-01", "ACT-02", "INV-02", "INV-03", "DL-01", "DL-02", "REL-03"] as const;
export const TARGET_FILES = [
  "README.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-node/src/network/relay_serving.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-network/src/peer/relay_download.rs",
  "packages/open-bitcoin-network/src/peer/inventory_state.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.ts",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts",
  "scripts/verify.sh",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-RESEARCH.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-01-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-02-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-03-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-04-SUMMARY.md",
] as const;
export const REQUIRED_EVIDENCE_ROOTS = [
  "README.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-node/src/network/relay_serving.rs",
  "packages/open-bitcoin-network/src/peer/relay_download.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.ts",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts",
  "scripts/verify.sh",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-RESEARCH.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-01-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-02-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-03-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-04-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-VERIFICATION.md",
] as const;
export const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net_permissions.h",
  "packages/bitcoin-knots/src/net_permissions.cpp",
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/node/txdownloadman.h",
  "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
  "packages/bitcoin-knots/test/functional/p2p_permissions.py",
  "packages/bitcoin-knots/test/functional/p2p_tx_download.py",
  "packages/bitcoin-knots/test/functional/p2p_getdata.py",
  "packages/bitcoin-knots/test/functional/rpc_rawtransaction.py",
] as const;
export const REQUIRED_SUPPRESSION_LABELS = [
  "relay_disabled",
  "not_relay_eligible",
  "inbound_serving_required",
  "permission_required",
  "protected_not_relay",
] as const;
export const REQUIRED_STATUS_SYMBOLS = [
  "RelayActivationEvidence",
  "RelayDownloadEligibilityCounters",
  "with_activation_and_counters",
  "activation: RelayEvidenceField<RelayActivationEvidence>",
  "download_eligibility: RelayEvidenceField<RelayDownloadEligibilityCounters>",
  "eligible_peer_count",
  "ineligible_peer_count",
  "relay_disabled_count",
  "inbound_serving_required_count",
  "permission_required_count",
  "protected_not_relay_count",
] as const;
export const REQUIRED_POLICY_SYMBOLS = [
  "RelayDownloadPolicy",
  "set_relay_download_policy",
  "relay_download_eligibility",
  "RelayEligibilityDecision",
  "TxParentRequestInput",
  "relay_eligibility: RelayEligibilityDecision",
  "relay_eligibility: relay_eligibility.clone()",
  "relay_eligibility,",
] as const;
export const REQUIRED_RUNTIME_GUIDE_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
  "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --",
  "-openbitcoinrelay=1",
  "-openbitcoininbound=1",
  "openbitcoinnetworkstatus",
  "support bundle --output-dir=/tmp/open-bitcoin-relay-enabled-support",
  "bash scripts/verify.sh",
] as const;
export const REQUIRED_DOC_NEEDLES = [
  "resolved `RuntimeConfig.relay`",
  "resolved `config.inbound.enabled`",
  "transaction download scheduling requires relay eligibility before request-state mutation",
  "aggregate, sanitized, and fixed-label only",
  "`RelayDownloadEligibilityCounters`",
  "Support evidence must not include peer ids, endpoints, permission strings, class names, txids, wtxids, raw transaction hex, credentials, or dynamic",
] as const;
export const REQUIRED_GAP_TERMS = [
  "public relay by default",
  "compact block relay",
  "package relay",
  "bloom/filter serving",
  "public-network relay CI",
  "production service operation",
  "production full-node readiness",
  "production-funds wallet safety",
  "production-funds wallet use",
  "durable mempool recovery",
] as const;
export const FORBIDDEN_CLAIMS = [
  "public relay by default",
  "compact block relay",
  "compact-block relay",
  "package relay",
  "bloom/filter serving",
  "public-network relay ci",
  "production service operation",
  "production-service operation",
  "production full-node readiness",
  "production-readiness proof",
  "production-funds wallet safety",
  "production-funds wallet use",
  "durable mempool recovery",
  "public propagation",
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
  "bounded",
  "opt-in",
  "separate",
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
export const FORBIDDEN_DEFAULT_VERIFIER_GATES = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 86400",
  "sleep 259200",
  "public-network",
  "wall-clock",
  "service-manager",
  "production-deployment",
  "production-funds",
] as const;
export const SENSITIVE_PUBLIC_EVIDENCE_PATTERNS = [
  /txid=[0-9a-f]{64}/i,
  /wtxid=[0-9a-f]{64}/i,
  /\bpeer_id=\d+/i,
  /020000000001/i,
  /\bpermission_string=/i,
  /\bcredential=/i,
  /\bsecret=/i,
  /\bcookie=/i,
  /\bdynamic_label=/i,
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
