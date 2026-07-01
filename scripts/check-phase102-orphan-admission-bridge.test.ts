import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase102OrphanAdmissionBridge } from "./check-phase102-orphan-admission-bridge";

const SURFACE_ID = "v2-0-orphan-handling-admission-outcome-bridge";
const REQUIRED_REQUIREMENTS = ["DL-03", "DL-04", "DL-05", "MEM-01", "MEM-02"] as const;
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
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "packages/open-bitcoin-node/src/network/action_translation.rs",
  "packages/open-bitcoin-node/src/network/admission_bridge.rs",
  "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs",
  "scripts/verify.sh",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-01-SUMMARY.md",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-02-SUMMARY.md",
  ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-03-SUMMARY.md",
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

test("passes_when_phase102_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase102OrphanAdmissionBridge(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_phase102_requirement_is_missing", () => {
  // Arrange
  const roots = REQUIRED_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase102OrphanAdmissionBridge(root).join("\n"));

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("fails_when_outcome_or_orphan_label_is_missing", () => {
  // Arrange
  const missingLabels = ["MempoolOutcomeLabel", "parent_requested"];
  const roots = missingLabels.map((label) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, label);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase102OrphanAdmissionBridge(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("required outcome label");
  expect(failureMessages[1]).toContain("required orphan label");
});

test("fails_when_cap_constant_or_bridge_symbol_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "PHASE102_MAX_ORPHAN_TRANSACTIONS");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "process_peer_transaction_admission");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase102OrphanAdmissionBridge(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("required cap constant");
  expect(failureMessages[1]).toContain("required bridge symbol");
});

test("fails_when_required_behavior_test_is_missing", () => {
  // Arrange
  const missingTests = [
    "no_partial_mutation_for_low_fee_rejection",
    "missing_parent_stage_requests_each_unique_parent_by_txid",
    "peer_manager_orphan_parent_request_respects_inflight_cap",
    "managed_admission_bridge_parent_acceptance_reconsiders_child",
    "managed_admission_bridge_disconnect_cleans_peer_orphans_and_request_state",
  ];
  const roots = missingTests.map((testName) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, testName);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase102OrphanAdmissionBridge(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("required behavior test");
  }
});

test("fails_when_managed_disconnect_cleanup_evidence_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(
        files,
        "packages/open-bitcoin-node/src/network/action_translation.rs",
        "self.orphanage.cleanup_peer(peer_id);",
      );
    },
  });

  // Act
  const failures = checkPhase102OrphanAdmissionBridge(root).join("\n");

  // Assert
  expect(failures).toContain("managed disconnect cleanup order");
});

test("fails_when_source_breadcrumbs_or_knots_anchors_are_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(files, "docs/parity/source-breadcrumbs.json", "node-network-adapter");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "packages/bitcoin-knots/src/txorphanage.cpp");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase102OrphanAdmissionBridge(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("source breadcrumb");
  expect(failureMessages[1]).toContain("Knots");
});

test("fails_when_default_verifier_wiring_is_missing_or_public_network_scoped", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(
          files,
          "scripts/verify.sh",
          "bun run scripts/check-phase102-orphan-admission-bridge.ts",
        );
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 102 public-network relay CI" true');
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase102OrphanAdmissionBridge(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("verifier-scope");
  expect(failureMessages[1]).toContain("verifier-scope forbidden");
});

test("fails_when_docs_claim_deferred_phase103_or_later_surfaces", () => {
  // Arrange
  const claims = [
    "Phase 102 supports relay serving.",
    "Phase 102 provides durable mempool persistence.",
    "Phase 102 enables support-bundle redaction.",
    "Phase 102 ships production full-node readiness.",
  ];
  const roots = claims.map((claim) =>
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "docs/parity/catalog/p2p.md", claim);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase102OrphanAdmissionBridge(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("no-claim");
  }
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase102-"));
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
    ["packages/open-bitcoin-mempool/src/outcome.rs", outcomeText()],
    ["packages/open-bitcoin-mempool/src/pool.rs", poolText()],
    ["packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs", outcomeTestsText()],
    ["packages/open-bitcoin-node/src/mempool.rs", managedMempoolText()],
    ["packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs", orphanageText()],
    ["packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs", orphanageTestsText()],
    ["packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs", schedulerText()],
    ["packages/open-bitcoin-network/src/peer.rs", peerText()],
    ["packages/open-bitcoin-network/src/peer/tests.rs", peerTestsText()],
    ["packages/open-bitcoin-node/src/network/action_translation.rs", actionTranslationText()],
    ["packages/open-bitcoin-node/src/network/admission_bridge.rs", admissionBridgeText()],
    ["packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs", admissionBridgeTestsText()],
    ["scripts/verify.sh", verifyScriptText()],
    [
      ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-01-SUMMARY.md",
      "no_partial_mutation_for_low_fee_rejection",
    ],
    [
      ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-02-SUMMARY.md",
      "missing_parent_stage_requests_each_unique_parent_by_txid peer_manager_orphan_parent_request_respects_inflight_cap",
    ],
    [
      ".planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-03-SUMMARY.md",
      "managed_admission_bridge_parent_acceptance_reconsiders_child managed_admission_bridge_disconnect_cleans_peer_orphans_and_request_state",
    ],
  ]);
}

function removeFromAllFiles(files: Map<TargetFile, string>, needle: string): void {
  for (const [file, current] of files) {
    files.set(file, current.replaceAll(needle, ""));
  }
}

function removeFromFile(files: Map<TargetFile, string>, file: TargetFile, needle: string): void {
  files.set(file, (files.get(file) ?? "").replaceAll(needle, ""));
}

function appendToFile(files: Map<TargetFile, string>, file: TargetFile, line: string): void {
  files.set(file, `${files.get(file) ?? ""}\n${line}\n`);
}

function p2pCatalogText(): string {
  return [
    "# P2P Networking And Sync",
    `Phase 102 ${SURFACE_ID} covers DL-03, DL-04, DL-05, MEM-01, and MEM-02.`,
    REQUIRED_EVIDENCE_ROOTS.join(" "),
    REQUIRED_KNOTS_ANCHORS.join(" "),
    "MempoolOutcome MempoolOutcomeLabel MempoolRejectionCategory accepted rejected duplicate replaced orphaned evicted expired",
    "TxOrphanage OrphanPolicy OrphanEvidenceLabel parent_requested orphan_evicted orphan_expired orphan_reconsidered",
    "PHASE102_MAX_ORPHAN_TRANSACTIONS PHASE102_MAX_ORPHANS_PER_PEER PHASE102_ORPHAN_TTL_SECONDS PHASE102_MAX_RECONSIDERATIONS_PER_PARENT",
    "request_orphan_parent process_peer_transaction_admission submit_transaction_outcome accept_transaction_outcome reconsider_orphans_after_acceptance expire_orphan_transactions remove_stored_transactions disconnect_peer_at cleanup_peer",
    "Phase 102 does not claim durable mempool persistence, block connect/disconnect mempool lifecycle, long-lived mempool pressure/trimming evidence, relay serving, relay fanout, rebroadcast, RPC/operator/support evidence, support-bundle redaction for transaction material, release-boundary closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, production service operation, or production-funds wallet use.",
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
            title: "v2.0 Orphan Handling and Admission Outcome Bridge",
            status: "done",
            requirements: [...REQUIRED_REQUIREMENTS],
            evidence: [...REQUIRED_EVIDENCE_ROOTS],
            rationale:
              "Phase 102 records MempoolOutcome, TxOrphanage, request_orphan_parent, process_peer_transaction_admission, disconnect_peer_at, cleanup_peer, and deterministic checker coverage.",
            upstream: {
              sources: [
                "packages/bitcoin-knots/src/txorphanage.h",
                "packages/bitcoin-knots/src/txorphanage.cpp",
                "packages/bitcoin-knots/src/node/txdownloadman.h",
                "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
                "packages/bitcoin-knots/src/net_processing.cpp",
                "packages/bitcoin-knots/src/validation.cpp",
                "packages/bitcoin-knots/src/txmempool.cpp",
                "packages/bitcoin-knots/src/policy/policy.cpp",
                "packages/bitcoin-knots/src/policy/rbf.cpp",
              ],
              tests: [
                "packages/bitcoin-knots/test/functional/p2p_orphan_handling.py",
                "packages/bitcoin-knots/test/functional/mempool_accept.py",
                "packages/bitcoin-knots/test/functional/feature_rbf.py",
              ],
            },
            known_gaps: [
              "Phase 102 does not claim durable mempool persistence, block connect/disconnect mempool lifecycle, long-lived mempool pressure/trimming evidence, relay serving, relay fanout, rebroadcast, RPC/operator/support evidence, support-bundle redaction for transaction material, release-boundary closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, production service operation, or production-funds wallet use.",
            ],
            suspected_unknowns: [
              "Future v2.0 phases own long-lived mempool pressure, block lifecycle, durable persistence, relay serving/fanout, rebroadcast, operator evidence, support-bundle redaction, and release-boundary guardrails.",
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
    `| \`${SURFACE_ID}\` | \`done\` | \`DL-03\`, \`DL-04\`, \`DL-05\`, \`MEM-01\`, \`MEM-02\` | ${REQUIRED_EVIDENCE_ROOTS.join(" ")} | MempoolOutcome MempoolOutcomeLabel MempoolRejectionCategory accepted rejected duplicate replaced orphaned evicted expired TxOrphanage OrphanPolicy OrphanEvidenceLabel parent_requested orphan_evicted orphan_expired orphan_reconsidered PHASE102_MAX_ORPHAN_TRANSACTIONS PHASE102_MAX_ORPHANS_PER_PEER PHASE102_ORPHAN_TTL_SECONDS PHASE102_MAX_RECONSIDERATIONS_PER_PARENT request_orphan_parent process_peer_transaction_admission submit_transaction_outcome accept_transaction_outcome reconsider_orphans_after_acceptance expire_orphan_transactions remove_stored_transactions disconnect_peer_at cleanup_peer. Phase 102 does not claim durable mempool persistence, block connect/disconnect mempool lifecycle, long-lived mempool pressure/trimming evidence, relay serving, relay fanout, rebroadcast, RPC/operator/support evidence, support-bundle redaction for transaction material, release-boundary closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, production service operation, or production-funds wallet use. | Future v2.0 phases own later surfaces. |`,
  ].join("\n");
}

function sourceBreadcrumbsText(): string {
  return JSON.stringify(
    {
      groups: [
        {
          label: "mempool-policy",
          files: [
            "packages/open-bitcoin-mempool/src/outcome.rs",
            "packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs",
          ],
          breadcrumbs: [
            "packages/bitcoin-knots/src/txmempool.h",
            "packages/bitcoin-knots/src/txmempool.cpp",
            "packages/bitcoin-knots/src/policy/policy.h",
            "packages/bitcoin-knots/src/policy/rbf.cpp",
          ],
        },
        {
          label: "network-transaction-relay-download",
          files: [
            "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
            "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
          ],
          breadcrumbs: [
            "packages/bitcoin-knots/src/protocol.h",
            "packages/bitcoin-knots/src/net_processing.cpp",
            "packages/bitcoin-knots/src/node/txdownloadman.h",
            "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
            "packages/bitcoin-knots/src/txorphanage.h",
            "packages/bitcoin-knots/src/txorphanage.cpp",
            "packages/bitcoin-knots/test/functional/p2p_orphan_handling.py",
          ],
        },
        {
          label: "node-network-adapter",
          files: [
            "packages/open-bitcoin-node/src/network/action_translation.rs",
            "packages/open-bitcoin-node/src/network/admission_bridge.rs",
            "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs",
          ],
          breadcrumbs: [
            "packages/bitcoin-knots/src/net_processing.cpp",
            "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
            "packages/bitcoin-knots/src/node/txdownloadman.h",
            "packages/bitcoin-knots/src/protocol.h",
            "packages/bitcoin-knots/src/txorphanage.cpp",
            "packages/bitcoin-knots/src/validation.cpp",
            "packages/bitcoin-knots/test/functional/p2p_orphan_handling.py",
            "packages/bitcoin-knots/test/functional/mempool_accept.py",
          ],
        },
      ],
    },
    null,
    2,
  );
}

function outcomeText(): string {
  return [
    "pub enum MempoolOutcomeLabel { Accepted, Rejected, Duplicate, Replaced, Orphaned, Evicted, Expired }",
    '"accepted" "rejected" "duplicate" "replaced" "orphaned" "evicted" "expired"',
    "pub enum MempoolRejectionCategory { RelayFeeTooLow }",
    "pub enum MempoolOutcome { Accepted, Rejected, Duplicate, Replaced, Orphaned, Evicted, Expired }",
  ].join("\n");
}

function poolText(): string {
  return [
    "pub fn accept_transaction_outcome() -> MempoolOutcome {}",
    "fn remove_stored_transactions() {}",
  ].join("\n");
}

function outcomeTestsText(): string {
  return "fn no_partial_mutation_for_low_fee_rejection() {}";
}

function managedMempoolText(): string {
  return "pub fn submit_transaction_outcome() -> MempoolOutcome {}";
}

function orphanageText(): string {
  return [
    "pub const PHASE102_MAX_ORPHAN_TRANSACTIONS: usize = 100;",
    "pub const PHASE102_MAX_ORPHANS_PER_PEER: usize = 25;",
    "pub const PHASE102_ORPHAN_TTL_SECONDS: i64 = 1200;",
    "pub const PHASE102_MAX_RECONSIDERATIONS_PER_PARENT: usize = 32;",
    "pub struct TxOrphanage;",
    "pub struct OrphanPolicy;",
    "pub enum OrphanEvidenceLabel { ParentRequested, OrphanEvicted, OrphanExpired, OrphanReconsidered }",
    '"parent_requested" "orphan_evicted" "orphan_expired" "orphan_reconsidered"',
    "pub enum OrphanReconsiderationStatus { Accepted }",
    "fn stage_missing_parent() {}",
    "fn reconsider_after_parent_acceptance() {}",
    "fn reconsider_after_parent() {}",
    "fn cleanup_peer() {}",
    "TxRelayId::Txid(parent)",
  ].join("\n");
}

function orphanageTestsText(): string {
  return "fn missing_parent_stage_requests_each_unique_parent_by_txid() {}";
}

function schedulerText(): string {
  return "fn cleanup_peer() {}";
}

function peerText(): string {
  return "pub fn request_orphan_parent(peer_id: PeerId, parent_txid: Txid, now_unix_seconds: i64) {}";
}

function peerTestsText(): string {
  return "fn peer_manager_orphan_parent_request_respects_inflight_cap() {}";
}

function actionTranslationText(): string {
  return [
    "pub fn disconnect_peer_at(&mut self, peer_id: PeerId, now_unix_seconds: i64) {",
    "  let actions = self.peer_manager.remove_peer_with_transaction_cleanup(peer_id, now_unix_seconds)?;",
    "  self.orphanage.cleanup_peer(peer_id);",
    "  self.known_peers.remove(&peer_id);",
    "}",
  ].join("\n");
}

function admissionBridgeText(): string {
  return [
    "pub fn process_peer_transaction_admission() {",
    "  self.mempool.submit_transaction_outcome();",
    "  self.orphanage.stage_missing_parent();",
    "  self.peer_manager.request_orphan_parent();",
    "}",
    "fn reconsider_orphans_after_acceptance() {}",
    "fn expire_orphan_transactions() {}",
    "fn remove_stored_transactions() {}",
    "fn accept_transaction_outcome() {}",
  ].join("\n");
}

function admissionBridgeTestsText(): string {
  return [
    "fn managed_admission_bridge_parent_acceptance_reconsiders_child() {}",
    "fn managed_admission_bridge_disconnect_cleans_peer_orphans_and_request_state() {}",
  ].join("\n");
}

function verifyScriptText(): string {
  return [
    "# Phase 100 is followed by Phase 101. Phase 101 is followed by Phase 102.",
    ": <<'VERIFY_COMMAND_ORDER'",
    "bun test scripts/check-phase101-transaction-inventory-download-scheduling.test.ts",
    "bun run scripts/check-phase101-transaction-inventory-download-scheduling.ts",
    "bun test scripts/check-phase102-orphan-admission-bridge.test.ts",
    "bun run scripts/check-phase102-orphan-admission-bridge.ts",
    "VERIFY_COMMAND_ORDER",
    'run_step "test Phase 101 transaction inventory download scheduling checker" bun test scripts/check-phase101-transaction-inventory-download-scheduling.test.ts',
    'run_step "check Phase 101 transaction inventory download scheduling" bun run scripts/check-phase101-transaction-inventory-download-scheduling.ts',
    'run_step "test Phase 102 orphan admission bridge checker" bun test scripts/check-phase102-orphan-admission-bridge.test.ts',
    'run_step "check Phase 102 orphan admission bridge" bun run scripts/check-phase102-orphan-admission-bridge.ts',
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
  ].join("\n");
}
