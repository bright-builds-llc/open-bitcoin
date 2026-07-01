import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase101TransactionInventoryDownloadScheduling } from "./check-phase101-transaction-inventory-download-scheduling";

const SURFACE_ID = "v2-0-transaction-inventory-download-scheduling";
const REQUIRED_REQUIREMENTS = ["INV-01", "INV-02", "INV-03", "INV-04", "DL-01", "DL-02"] as const;
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

type TargetFile = (typeof TARGET_FILES)[number];
type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
};

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes_when_phase101_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase101TransactionInventoryDownloadScheduling(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_phase101_requirement_is_missing", () => {
  // Arrange
  const roots = REQUIRED_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase101TransactionInventoryDownloadScheduling(root).join("\n"),
  );

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("fails_when_scheduler_source_or_test_evidence_is_missing", () => {
  // Arrange
  const missingNeedles = ["TxRelayId", "TxDownloadScheduler", "transaction_relay/tests.rs"];
  const roots = missingNeedles.map((needle) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, needle);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase101TransactionInventoryDownloadScheduling(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("missing scheduler evidence");
  }
});

test("fails_when_required_action_label_is_missing", () => {
  // Arrange
  const roots = REQUIRED_ACTION_LABELS.map((label) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, label);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase101TransactionInventoryDownloadScheduling(root).join("\n"),
  );

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain("required action label");
    expect(message).toContain(REQUIRED_ACTION_LABELS[index]);
  }
});

test("fails_when_required_behavior_test_is_missing", () => {
  // Arrange
  const missingTests = [
    "txid_delay_waits_until_fake_clock_reaches_ready_time",
    "non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time",
    "overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time",
    "expiry_fallback_waits_until_fake_clock_reaches_getdata_interval",
    "managed_network_transaction_relay",
  ];
  const roots = missingTests.map((testName) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, testName);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase101TransactionInventoryDownloadScheduling(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("required behavior test");
  }
});

test("fails_when_source_breadcrumbs_are_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "network-transaction-relay-download");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "packages/bitcoin-knots/test/functional/p2p_getdata.py");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase101TransactionInventoryDownloadScheduling(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("source breadcrumb");
  }
});

test("fails_when_public_network_sleep_or_service_manager_enters_default_verifier", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 101 public-network relay CI" true');
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 101 sleep 60" sleep 60');
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 101 service-manager" systemctl status open-bitcoind');
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase101TransactionInventoryDownloadScheduling(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("verifier-scope");
  }
});

test("fails_when_docs_claim_phase102_or_later_surfaces", () => {
  // Arrange
  const claims = [
    "Phase 101 supports orphan handling.",
    "Phase 101 provides mempool admission outcomes.",
    "Phase 101 enables relay serving and relay fanout.",
    "Phase 101 implements compact block relay.",
    "Phase 101 ships production full-node readiness.",
  ];
  const roots = claims.map((claim) =>
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "docs/parity/catalog/p2p.md", claim);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase101TransactionInventoryDownloadScheduling(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("no-claim");
  }
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase101-"));
  tempRoots.push(root);

  const files = fixtureFiles();
  options.maybeMutateFiles?.(files);

  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }

  return root;
}

function fixtureFiles(): Map<TargetFile, string> {
  return new Map<TargetFile, string>([
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/index.json", parityIndexText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/source-breadcrumbs.json", sourceBreadcrumbsText()],
    ["packages/open-bitcoin-network/src/peer/transaction_relay.rs", transactionRelayText()],
    ["packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs", transactionRelayTestsText()],
    ["packages/open-bitcoin-network/src/peer.rs", peerText()],
    ["packages/open-bitcoin-network/src/peer/inventory_state.rs", inventoryStateText()],
    ["packages/open-bitcoin-network/src/peer/tests.rs", peerTestsText()],
    ["packages/open-bitcoin-node/src/network.rs", managedNetworkText()],
    ["packages/open-bitcoin-node/src/network/tests.rs", managedNetworkTestsText()],
    ["scripts/verify.sh", verifyScriptText()],
  ]);
}

function removeFromAllFiles(files: Map<TargetFile, string>, needle: string): void {
  for (const [file, current] of files) {
    files.set(file, current.replaceAll(needle, ""));
  }
}

function appendToFile(files: Map<TargetFile, string>, file: TargetFile, line: string): void {
  files.set(file, `${files.get(file) ?? ""}\n${line}\n`);
}

function p2pCatalogText(): string {
  return [
    "# P2P Networking And Sync",
    `Phase 101 ${SURFACE_ID} covers INV-01, INV-02, INV-03, INV-04, DL-01, and DL-02.`,
    "TxRelayId TxRelayPeerMode TxDownloadScheduler TxDownloadPolicy TxDownloadLocalFacts TxDownloadAction TxDownloadSuppressionReason PeerAction::TransactionRelay",
    "PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER PHASE101_TXID_RELAY_DELAY_SECONDS PHASE101_NONPREF_PEER_TX_DELAY_SECONDS PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS PHASE101_GETDATA_TX_INTERVAL_SECONDS",
    "request_getdata suppress_duplicate suppress_already_have suppress_recent_reject suppress_mempool_known mempool_known suppress_identity_mismatch suppress_request_cap fallback_request request_expired notfound_cleanup received_tx_cleanup peer_cleanup",
    "packages/open-bitcoin-network/src/peer/transaction_relay.rs packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs packages/open-bitcoin-network/src/peer.rs packages/open-bitcoin-network/src/peer/inventory_state.rs packages/open-bitcoin-network/src/peer/tests.rs packages/open-bitcoin-node/src/network.rs packages/open-bitcoin-node/src/network/tests.rs docs/parity/source-breadcrumbs.json .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-01-SUMMARY.md .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-02-SUMMARY.md",
    "packages/bitcoin-knots/src/protocol.h packages/bitcoin-knots/src/net_processing.cpp packages/bitcoin-knots/src/node/txdownloadman.h packages/bitcoin-knots/src/node/txdownloadman_impl.cpp packages/bitcoin-knots/src/txrequest.h packages/bitcoin-knots/src/txrequest.cpp packages/bitcoin-knots/test/functional/p2p_tx_download.py packages/bitcoin-knots/test/functional/p2p_getdata.py",
    "Phase 101 does not claim orphan handling, parent request behavior, mempool admission outcomes, standardness or fee policy, RBF, ancestor or descendant policy, mempool lifecycle or persistence, block connect/disconnect mempool behavior, relay serving/fanout, rebroadcast, RPC/operator/support surfaces, compact block relay, package relay, bloom/filter serving, public relay by default, public-network relay CI, production service operation, production full-node readiness, or production-funds wallet use.",
  ].join("\n");
}

function parityIndexText(): string {
  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            title: "v2.0 Transaction Inventory Identity and Download Scheduling",
            status: "done",
            requirements: [...REQUIRED_REQUIREMENTS],
            evidence: [
              "docs/parity/catalog/p2p.md",
              "docs/parity/checklist.md",
              "docs/parity/source-breadcrumbs.json",
              "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
              "packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs",
              "packages/open-bitcoin-network/src/peer.rs",
              "packages/open-bitcoin-network/src/peer/inventory_state.rs",
              "packages/open-bitcoin-network/src/peer/tests.rs",
              "packages/open-bitcoin-node/src/network.rs",
              "packages/open-bitcoin-node/src/network/tests.rs",
              "scripts/check-phase101-transaction-inventory-download-scheduling.ts",
              "scripts/check-phase101-transaction-inventory-download-scheduling.test.ts",
              "scripts/verify.sh",
              ".planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-01-SUMMARY.md",
              ".planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-02-SUMMARY.md",
            ],
            rationale:
              "Phase 101 records TxRelayId, TxDownloadScheduler, TxDownloadSuppressionReason, request_getdata, suppress_mempool_known, mempool_known, peer_cleanup, PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER, and PHASE101_GETDATA_TX_INTERVAL_SECONDS evidence.",
            upstream: {
              sources: [
                "packages/bitcoin-knots/src/protocol.h",
                "packages/bitcoin-knots/src/net_processing.cpp",
                "packages/bitcoin-knots/src/node/txdownloadman.h",
                "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
                "packages/bitcoin-knots/src/txrequest.h",
                "packages/bitcoin-knots/src/txrequest.cpp",
              ],
              tests: [
                "packages/bitcoin-knots/test/functional/p2p_tx_download.py",
                "packages/bitcoin-knots/test/functional/p2p_getdata.py",
              ],
            },
            known_gaps: [
              "Phase 101 does not claim orphan handling, parent request behavior, mempool admission outcomes, standardness or fee policy, RBF, ancestor or descendant policy, mempool lifecycle or persistence, block connect/disconnect mempool behavior, relay serving/fanout, rebroadcast, RPC/operator/support surfaces, compact block relay, package relay, bloom/filter serving, public relay by default, public-network relay CI, production service operation, production full-node readiness, or production-funds wallet use.",
            ],
            suspected_unknowns: [
              "Future phases own mempool admission, relay serving/fanout, rebroadcast, compact block relay, package relay, filters, operator evidence, and release-boundary guardrails.",
            ],
          },
        ],
      },
    },
    null,
    2,
  );
}

function checklistText(): string {
  return [
    "# Parity Checklist",
    `| \`${SURFACE_ID}\` | \`done\` | \`INV-01\`, \`INV-02\`, \`INV-03\`, \`INV-04\`, \`DL-01\`, \`DL-02\` | packages/open-bitcoin-network/src/peer/transaction_relay.rs packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs packages/open-bitcoin-network/src/peer.rs packages/open-bitcoin-network/src/peer/inventory_state.rs packages/open-bitcoin-network/src/peer/tests.rs packages/open-bitcoin-node/src/network.rs packages/open-bitcoin-node/src/network/tests.rs docs/parity/source-breadcrumbs.json .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-01-SUMMARY.md .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-02-SUMMARY.md | TxRelayId TxRelayPeerMode TxDownloadScheduler TxDownloadPolicy TxDownloadLocalFacts TxDownloadAction TxDownloadSuppressionReason PeerAction::TransactionRelay PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER PHASE101_TXID_RELAY_DELAY_SECONDS PHASE101_NONPREF_PEER_TX_DELAY_SECONDS PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS PHASE101_GETDATA_TX_INTERVAL_SECONDS request_getdata suppress_duplicate suppress_already_have suppress_recent_reject suppress_mempool_known mempool_known suppress_identity_mismatch suppress_request_cap fallback_request request_expired notfound_cleanup received_tx_cleanup peer_cleanup. Phase 101 does not claim orphan handling, parent request behavior, mempool admission outcomes, standardness or fee policy, RBF, ancestor or descendant policy, mempool lifecycle or persistence, block connect/disconnect mempool behavior, relay serving/fanout, rebroadcast, RPC/operator/support surfaces, compact block relay, package relay, bloom/filter serving, public relay by default, public-network relay CI, production service operation, production full-node readiness, or production-funds wallet use. | Future phases own later surfaces. |`,
  ].join("\n");
}

function sourceBreadcrumbsText(): string {
  return JSON.stringify(
    {
      groups: [
        {
          label: "network-transaction-relay-download",
          files: [
            "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
            "packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs",
          ],
          breadcrumbs: [
            "packages/bitcoin-knots/src/protocol.h",
            "packages/bitcoin-knots/src/net_processing.cpp",
            "packages/bitcoin-knots/src/node/txdownloadman.h",
            "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
            "packages/bitcoin-knots/src/txrequest.h",
            "packages/bitcoin-knots/src/txrequest.cpp",
            "packages/bitcoin-knots/test/functional/p2p_tx_download.py",
            "packages/bitcoin-knots/test/functional/p2p_getdata.py",
          ],
        },
      ],
    },
    null,
    2,
  );
}

function transactionRelayText(): string {
  return [
    "pub const PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER: usize = 5000;",
    "pub const PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER: usize = 100;",
    "pub const PHASE101_TXID_RELAY_DELAY_SECONDS: i64 = 2;",
    "pub const PHASE101_NONPREF_PEER_TX_DELAY_SECONDS: i64 = 2;",
    "pub const PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS: i64 = 2;",
    "pub const PHASE101_GETDATA_TX_INTERVAL_SECONDS: i64 = 60;",
    "pub enum TxRelayId { Txid(Txid), Wtxid(Wtxid) }",
    "pub enum TxRelayPeerMode { TxidOnly, WtxidRelay }",
    "pub struct TxDownloadPolicy;",
    "pub struct TxDownloadLocalFacts;",
    "pub struct TxDownloadScheduler;",
    "pub enum TxDownloadSuppressionReason { MempoolKnown }",
    "TxDownloadSuppressionReason::MempoolKnown",
    "pub enum TxDownloadAction { Suppress { peer_id: PeerId, relay_id: TxRelayId, reason: TxDownloadSuppressionReason } }",
    "request_getdata suppress_duplicate suppress_already_have suppress_recent_reject suppress_mempool_known mempool_known suppress_identity_mismatch suppress_request_cap fallback_request request_expired notfound_cleanup received_tx_cleanup peer_cleanup",
  ].join("\n");
}

function transactionRelayTestsText(): string {
  return [
    "fn txid_announcement_requests_transaction_inventory() {}",
    "fn wtxid_announcement_requests_witness_transaction_inventory() {}",
    "fn identity_mismatch_suppresses_without_candidate_or_inflight_state() {}",
    "fn duplicate_announcement_retains_fallback_candidate_without_second_request() {}",
    "fn already_have_recent_reject_and_mempool_known_suppress_requests() {}",
    "fn inflight_cap_suppresses_additional_ready_requests() {}",
    "fn txid_delay_waits_until_fake_clock_reaches_ready_time() {}",
    "fn non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time() {}",
    "fn overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time() {}",
    "fn expiry_fallback_waits_until_fake_clock_reaches_getdata_interval() {}",
    "fn timeout_expires_request_and_falls_back_to_duplicate_announcer() {}",
    "fn notfound_clears_matching_request_and_falls_back() {}",
    "fn disconnect_cleanup_removes_peer_state_and_falls_back() {}",
    "fn received_transaction_cleanup_waits_for_admission_before_already_have() {}",
  ].join("\n");
}

function peerText(): string {
  return [
    "use transaction_relay::{TxDownloadAction, TxDownloadLocalFacts, TxDownloadPolicy, TxDownloadScheduler, TxRelayId, TxRelayPeerMode};",
    "pub enum PeerAction { TransactionRelay(TxDownloadAction) }",
    "PeerAction::TransactionRelay(action)",
    "tx_download: TxDownloadScheduler",
    "recent_rejects: BTreeSet<TxRelayId>",
    "fn note_recent_reject(&mut self, relay_id: TxRelayId) {}",
    "fn transaction_request_snapshot(&self) {}",
    "fn expire_transaction_requests(&mut self) {}",
    "fn remove_peer_with_transaction_cleanup(&mut self) {}",
  ].join("\n");
}

function inventoryStateText(): string {
  return [
    "TxRelayPeerMode::from_remote_wtxidrelay(peer.remote_wtxidrelay)",
    "TxRelayId::Txid(txid)",
    "TxRelayId::Wtxid(wtxid)",
    "record_notfound(peer_id, relay_id, timestamp)",
    "record_received_transaction(peer_id, txid, wtxid)",
    "PeerAction::TransactionRelay(action)",
    "TxDownloadLocalFacts { mempool_known }",
  ].join("\n");
}

function peerTestsText(): string {
  return [
    "fn peer_manager_transaction_relay_txid_inv_emits_typed_request_action() {}",
    "fn peer_manager_transaction_relay_wtxid_inv_emits_typed_request_action() {}",
    "fn peer_manager_transaction_relay_mismatch_emits_suppression_without_state() {}",
    "fn peer_manager_transaction_relay_duplicate_inv_suppresses_second_getdata_but_keeps_fallback() {}",
    "fn peer_manager_transaction_relay_already_have_and_recent_reject_suppress_requests() {}",
    "fn peer_manager_transaction_relay_notfound_timeout_and_disconnect_cleanup_fallback() {}",
    "fn peer_manager_transaction_relay_received_transaction_cleanup_waits_for_admission() {}",
  ].join("\n");
}

function managedNetworkText(): string {
  return [
    "fn process_transaction_relay_action(action: TxDownloadAction) {}",
    "PeerAction::TransactionRelay(action)",
  ].join("\n");
}

function managedNetworkTestsText(): string {
  return [
    "fn managed_network_transaction_relay_inv_translates_request_action_to_getdata() {}",
    "fn managed_network_transaction_relay_duplicate_suppression_emits_no_extra_getdata() {}",
    "fn managed_network_transaction_relay_timeout_fallback_returns_getdata_for_alternate_peer() {}",
    "fn managed_network_transaction_relay_notfound_fallback_returns_getdata_for_alternate_peer() {}",
    "fn managed_network_transaction_relay_disconnect_fallback_returns_getdata_for_alternate_peer() {}",
  ].join("\n");
}

function verifyScriptText(): string {
  return [
    "# Phase 99 is followed by Phase 100. Phase 100 is followed by Phase 101.",
    ": <<'VERIFY_COMMAND_ORDER'",
    "bun test scripts/check-phase100-relay-activation-boundary.test.ts",
    "bun run scripts/check-phase100-relay-activation-boundary.ts",
    "bun test scripts/check-phase101-transaction-inventory-download-scheduling.test.ts",
    "bun run scripts/check-phase101-transaction-inventory-download-scheduling.ts",
    "VERIFY_COMMAND_ORDER",
    'run_step "test Phase 100 relay activation boundary checker" bun test scripts/check-phase100-relay-activation-boundary.test.ts',
    'run_step "check Phase 100 relay activation boundary" bun run scripts/check-phase100-relay-activation-boundary.ts',
    'run_step "test Phase 101 transaction inventory download scheduling checker" bun test scripts/check-phase101-transaction-inventory-download-scheduling.test.ts',
    'run_step "check Phase 101 transaction inventory download scheduling" bun run scripts/check-phase101-transaction-inventory-download-scheduling.ts',
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
  ].join("\n");
}
