import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REQUIRED_OUTCOME_LABELS, REQUIRED_ORPHAN_LABELS, REQUIRED_CONSTANTS, REQUIRED_BRIDGE_SYMBOLS, REQUIRED_BEHAVIOR_TESTS, TextCorpus } from "./constants.ts";
import { requireContains, verifyOrderedCommands } from "./helpers.ts";

export function verifyOutcomeAndOrphanEvidence(texts: TextCorpus, failures: string[]): void {
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

export function verifyManagedBridgeEvidence(texts: TextCorpus, failures: string[]): void {
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

export function verifyBehaviorTests(texts: TextCorpus, failures: string[]): void {
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
