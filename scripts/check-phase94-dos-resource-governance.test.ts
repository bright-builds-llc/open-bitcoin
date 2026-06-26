import { afterEach, expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase94DosResourceGovernance } from "./check-phase94-dos-resource-governance";

const SURFACE_ID = "v1-9-dos-resource-governance";
const PHASE94_REQUIREMENTS = ["DOS-01", "DOS-02", "DOS-03", "DOS-04", "DOS-05"] as const;
const PHASE93_TEST_COMMAND = "bun test scripts/check-phase93-peer-policy.test.ts";
const PHASE93_CHECKER_COMMAND = "bun run scripts/check-phase93-peer-policy.ts";
const PHASE94_TEST_COMMAND =
  "bun test scripts/check-phase94-dos-resource-governance.test.ts";
const PHASE94_CHECKER_COMMAND =
  "bun run scripts/check-phase94-dos-resource-governance.ts";
const REQUIRED_LABELS = [
  "wrong_network_magic",
  "malformed_header",
  "payload_oversized",
  "invalid_checksum",
  "unsupported_command",
  "malformed_payload",
  "trailing_payload",
  "slow_handshake",
  "idle_peer",
  "connection_churn_limited",
  "repeated_failure_limited",
  "reconnect_suppressed_banned",
  "reconnect_suppressed_discouraged",
  "resource_pressure_active",
  "read_queue_pressure",
  "write_queue_pressure",
  "request_cap_reached",
  "payload_rejected",
  "timeout_disconnect",
  "churn_rejected",
  "reconnect_suppressed",
] as const;
const REQUIRED_METRICS = [
  "inbound_resource_pressure_active_count",
  "inbound_read_queue_pressure_count",
  "inbound_write_queue_pressure_count",
  "inbound_request_cap_reached_count",
  "inbound_payload_rejected_count",
  "inbound_timeout_disconnect_count",
  "inbound_churn_rejected_count",
  "inbound_reconnect_suppressed_count",
] as const;
const STRUCTURED_LOG_FIELDS =
  "INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE inbound_resource_governance_log_record inbound_resource_governance outcome= reason= label= source= message= next_action=";
const RUNTIME_WIRING =
  "decide_queue tokio::time::timeout decide_churn decide_repeated_failure decide_reconnect record_inbound_resource_event reconnect_suppression_input_for_remote_addr";
const STRUCTURED_LOG_EMISSION =
  "append_structured_log_record maybe_resource_governance_log_dir LogRetentionPolicy record_inbound_resource_event_at record_inbound_resource_event_appends_inbound_resource_governance_log_record open-bitcoin-runtime- serde_json::from_str";
const HELPER_PROJECTION_WITHOUT_APPEND =
  "inbound_resource_governance_log_record helper projection without append_structured_log_record";
const APPEND_ON_MANAGED_CONTEXT =
  "append_structured_log_record must stay wired through ManagedRpcContext";
const TARGET_FILES = [
  "packages/open-bitcoin-network/src/resource.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs",
  "packages/open-bitcoin-rpc/src/context.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/context/resource_governance.rs",
  "packages/open-bitcoin-rpc/src/context/tests.rs",
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-node/src/status/inbound.rs",
  "packages/open-bitcoin-node/src/network/inbound.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/writer.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
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

test("passes with complete Phase 94 labels, docs anchors, metrics, logs, and verifier wiring", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase94DosResourceGovernance({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails when required Phase 94 labels or fixed metric names are missing", () => {
  // Arrange
  const roots = ["payload_oversized", "slow_handshake", "inbound_payload_rejected_count"].map(
    (missingText) =>
      createFixture({
        maybeMutateFiles(files) {
          replaceInAllFiles(files, missingText, "");
        },
      }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase94DosResourceGovernance({ rootDir: root }).join("\n"),
  );

  // Assert
  expect(failureMessages[0]).toContain("Phase 94 label coverage");
  expect(failureMessages[1]).toContain("Phase 94 label coverage");
  expect(failureMessages[2]).toContain("Phase 94 metric coverage");
});

test("fails when ManagedRpcContext structured log emission append wiring is missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "packages/open-bitcoin-rpc/src/context/resource_governance.rs",
        [
          "use open_bitcoin_node::logging::inbound_resource_governance_log_record;",
          HELPER_PROJECTION_WITHOUT_APPEND,
          "impl ManagedRpcContext {",
          "  pub fn record_inbound_resource_event(&mut self) {}",
          "}",
        ].join("\n"),
      );
      files.set(
        "packages/open-bitcoin-rpc/src/context/tests.rs",
        "fn helper_projection_only_does_not_decode_jsonl_records() {}",
      );
    },
  });

  // Act
  const failures = checkPhase94DosResourceGovernance({ rootDir: root });

  // Assert
  expect(`${APPEND_ON_MANAGED_CONTEXT}\n${failures.join("\n")}`).toContain(
    "ManagedRpcContext structured log emission",
  );
});

test("fails positive deferred-feature and production-readiness claims", () => {
  // Arrange
  const claims = [
    "Phase 94 provides transaction relay support.",
    "Phase 94 provides compact block relay support.",
    "Phase 94 provides mempool propagation support.",
    "Phase 94 provides public inbound default behavior.",
    "Phase 94 provides production service operation.",
    "Phase 94 provides production full-node readiness.",
    "Phase 94 provides BIP37 support.",
    "Phase 94 provides compact filter support.",
  ];
  const roots = claims.map((claim) =>
    createFixture({
      maybeMutateFiles(files) {
        const current = files.get("docs/parity/catalog/p2p.md") ?? "";
        files.set("docs/parity/catalog/p2p.md", `${current}\n${claim}\n`);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase94DosResourceGovernance({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 94 no-claim boundary");
  }
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase94-"));
  tempRoots.push(root);

  const files = fixtureFiles();
  options.maybeMutateFiles?.(files);

  for (const [file, contents] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }

  return root;
}

function fixtureFiles(): Map<TargetFile, string> {
  return new Map<TargetFile, string>([
    ["packages/open-bitcoin-network/src/resource.rs", resourceText()],
    ["packages/open-bitcoin-rpc/src/inbound_listener.rs", inboundListenerText()],
    ["packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs", resourceRuntimeText()],
    ["packages/open-bitcoin-rpc/src/context.rs", rpcContextText()],
    ["packages/open-bitcoin-rpc/src/context/network.rs", rpcNetworkText()],
    ["packages/open-bitcoin-rpc/src/context/resource_governance.rs", rpcResourceText()],
    ["packages/open-bitcoin-rpc/src/context/tests.rs", rpcContextTestsText()],
    ["packages/open-bitcoin-network/src/peer.rs", peerText()],
    ["packages/open-bitcoin-node/src/status/inbound.rs", statusText()],
    ["packages/open-bitcoin-node/src/network/inbound.rs", managedInboundText()],
    ["packages/open-bitcoin-node/src/logging.rs", loggingText()],
    ["packages/open-bitcoin-node/src/logging/writer.rs", loggingWriterText()],
    ["packages/open-bitcoin-node/src/metrics.rs", metricsText()],
    ["packages/open-bitcoin-cli/src/operator/status/render/inbound.rs", statusRendererText()],
    ["packages/open-bitcoin-cli/src/operator/support/render/inbound.rs", supportRendererText()],
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/architecture/status-snapshot.md", statusSnapshotText()],
    ["docs/architecture/operator-observability.md", operatorObservabilityText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/index.json", parityIndexText()],
    ["scripts/verify.sh", verifyScriptText()],
  ]);
}

function replaceInAllFiles(files: Map<TargetFile, string>, needle: string, replacement: string): void {
  for (const [file, current] of files) {
    files.set(file, current.replaceAll(needle, replacement));
  }
}

function resourceText(): string {
  return [
    "pub fn decide_queue() {}",
    "pub fn decide_churn() {}",
    "pub fn decide_repeated_failure() {}",
    "pub fn decide_reconnect() {}",
    REQUIRED_LABELS.join(" "),
    RUNTIME_WIRING,
  ].join("\n");
}

function inboundListenerText(): string {
  return [
    "resource_policy.decide_churn(churn_input)",
    "resource_policy.decide_repeated_failure(failure_input)",
    "resource_policy.decide_reconnect(reconnect_input)",
    "context.lock().await.record_inbound_resource_event(event);",
    "reconnect_suppression_input_for_remote_addr(remote_addr, now_unix_seconds)",
  ].join("\n");
}

function resourceRuntimeText(): string {
  return [
    "resource_event_from_decision(policy.decide_queue(input))",
    "tokio::time::timeout(timeout_duration, stream.readable()).await",
  ].join("\n");
}

function rpcContextText(): string {
  return [
    "use open_bitcoin_node::LogRetentionPolicy;",
    "struct ManagedRpcContext {",
    "  maybe_resource_governance_log_dir: Option<PathBuf>,",
    "  resource_governance_log_retention: LogRetentionPolicy,",
    "}",
  ].join("\n");
}

function rpcNetworkText(): string {
  return [
    "maybe_resource_governance_log_dir: maybe_resource_governance_log_dir.clone(),",
    "pub fn reconnect_suppression_input_for_remote_addr() {}",
  ].join("\n");
}

function rpcResourceText(): string {
  return [
    "use open_bitcoin_node::logging::{StructuredLogError, writer::append_structured_log_record};",
    "impl ManagedRpcContext {",
    "  pub fn record_inbound_resource_event(&mut self, event: InboundResourceEvent) {",
    "    let _ = self.record_inbound_resource_event_at(event, current_unix_seconds());",
    "  }",
    "  pub(crate) fn record_inbound_resource_event_at(&mut self, event: InboundResourceEvent, timestamp_unix_seconds: u64) -> Result<(), StructuredLogError> {",
    "    let Some(log_dir) = &self.maybe_resource_governance_log_dir else { return Ok(()); };",
    "    append_structured_log_record(log_dir, &record, self.resource_governance_log_retention)?;",
    "    Ok(())",
    "  }",
    "}",
    STRUCTURED_LOG_FIELDS,
  ].join("\n");
}

function rpcContextTestsText(): string {
  return [
    "fn record_inbound_resource_event_appends_inbound_resource_governance_log_record() {}",
    "context.record_inbound_resource_event_at(event, 1_777_225_022).expect(\"append\");",
    "if !file_name.starts_with(\"open-bitcoin-runtime-\") { continue; }",
    "serde_json::from_str(line).expect(\"decode structured log record\");",
    STRUCTURED_LOG_EMISSION,
  ].join("\n");
}

function peerText(): string {
  return "PeerAction::Disconnect DisconnectReason::ResourceLimit request_cap_reached";
}

function statusText(): string {
  return "InboundResourceGovernanceEvent resource_pressure_events read_queue_pressure_events write_queue_pressure_events request_cap_events payload_rejections timeout_disconnects churn_rejections reconnect_suppressions latest_resource_governance_decision";
}

function managedInboundText(): string {
  return [
    "ManagedResourceGovernanceInfo",
    "record_resource_governance_event",
    "maybe_structured_log_record",
    "inbound_resource_governance_log_record",
    REQUIRED_LABELS.join(" "),
    STRUCTURED_LOG_FIELDS,
  ].join("\n");
}

function loggingText(): string {
  return [
    "pub const INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE: &str = \"inbound_resource_governance\";",
    "pub fn inbound_resource_governance_log_record() {}",
    STRUCTURED_LOG_FIELDS,
  ].join("\n");
}

function loggingWriterText(): string {
  return [
    "pub fn append_structured_log_record(log_dir: &Path, record: &StructuredLogRecord, retention: LogRetentionPolicy) {}",
    "log_dir.join(format!(\"open-bitcoin-runtime-{unix_day}.jsonl\"))",
    "let Ok(record) = serde_json::from_str::<StructuredLogRecord>(&line) else { continue; };",
  ].join("\n");
}

function metricsText(): string {
  return REQUIRED_METRICS.join("\n");
}

function statusRendererText(): string {
  return [
    "resource evidence: resource_pressure_events read_queue_pressure_events write_queue_pressure_events request_cap_events payload_rejections timeout_disconnects churn_rejections reconnect_suppressions",
    "latest resource governance decision outcome= reason= label= source= message= next_action=",
  ].join("\n");
}

function supportRendererText(): string {
  return [
    "Treat Phase 94 as bounded inbound resource-governance evidence only.",
    "Resource pressure events Read queue pressure events Write queue pressure events Request cap events Payload rejections Timeout disconnects Churn rejections Reconnect suppressions",
    "Latest resource governance decision outcome= reason= label= source= message= next_action=",
  ].join("\n");
}

function runtimeGuideText(): string {
  return [
    "# Phase 94 Resource Governance Review",
    REQUIRED_LABELS.join(", "),
    REQUIRED_METRICS.join(", "),
    STRUCTURED_LOG_FIELDS,
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -chain=regtest -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444",
    "bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -chain=regtest -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -regtest openbitcoinnetworkstatus",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -regtest openbitcoinnetworkstatus",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-resource-support",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-resource-support",
    "Default verification remains loopback/synthetic and public-network-free.",
  ].join("\n");
}

function statusSnapshotText(): string {
  return [
    "resource_pressure_events read_queue_pressure_events write_queue_pressure_events request_cap_events payload_rejections timeout_disconnects churn_rejections reconnect_suppressions latest_resource_governance_decision",
    "outcome reason label source message next_action",
    STRUCTURED_LOG_FIELDS,
    REQUIRED_METRICS.join(", "),
    "The fields document bounded resource-governance evidence only; they do not change listener defaults or widen the release boundary.",
  ].join("\n");
}

function operatorObservabilityText(): string {
  return [
    "latest_resource_governance_decision payload_rejections timeout_disconnects churn_rejections reconnect_suppressions",
    STRUCTURED_LOG_FIELDS,
    REQUIRED_METRICS.join(", "),
    "This evidence documents bounded resource-governance review only and does not expand listener exposure or release claims.",
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    `The ${SURFACE_ID} surface covers ${PHASE94_REQUIREMENTS.join(", ")}.`,
    "packages/bitcoin-knots/src/protocol.h packages/bitcoin-knots/src/net.cpp packages/bitcoin-knots/src/net_processing.cpp packages/bitcoin-knots/src/banman.cpp packages/bitcoin-knots/src/net_permissions.cpp",
    "packages/bitcoin-knots/test/functional/p2p_invalid_messages.py packages/bitcoin-knots/test/functional/p2p_dos_header_tree.py packages/bitcoin-knots/test/functional/p2p_timeouts.py packages/bitcoin-knots/test/functional/p2p_ibd_stalling.py packages/bitcoin-knots/test/functional/p2p_getdata.py",
    REQUIRED_LABELS.join(" "),
    "openbitcoinnetworkstatus OpenBitcoinStatusSnapshot.peers.inbound operator status fixed metrics structured logs support bundles",
    "Phase 94 does not claim transaction relay, compact block relay, mempool propagation, broad address relay, public inbound defaults, public-network CI, production service operation, or production full-node readiness.",
  ].join("\n");
}

function checklistText(): string {
  return [
    "# Parity Checklist",
    "| Surface | Status | Requirements | Evidence | Known Gaps |",
    "| --- | --- | --- | --- | --- |",
    `| ${SURFACE_ID} | done | ${PHASE94_REQUIREMENTS.join(", ")} | runtime guide, status snapshot, operator observability, P2P catalog, source breadcrumbs | Phase 94 does not claim transaction relay, compact block relay, mempool propagation, broad address relay, public inbound defaults, public-network CI, production service operation, or production full-node readiness. |`,
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
            status: "done",
            requirements: [...PHASE94_REQUIREMENTS],
            evidence: [
              "docs/operator/runtime-guide.md",
              "docs/architecture/status-snapshot.md",
              "docs/architecture/operator-observability.md",
              "docs/parity/catalog/p2p.md",
              "docs/parity/source-breadcrumbs.json",
            ],
          },
        ],
      },
    },
    null,
    2,
  );
}

function verifyScriptText(): string {
  return [
    "#!/usr/bin/env bash",
    "set -euo pipefail",
    "run_step() {",
    "  local label=\"$1\"",
    "  shift",
    "  \"$@\"",
    "}",
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE93_TEST_COMMAND,
    PHASE93_CHECKER_COMMAND,
    PHASE94_TEST_COMMAND,
    PHASE94_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "test Phase 93 peer policy checker" ${PHASE93_TEST_COMMAND}`,
    `run_step "check Phase 93 peer policy" ${PHASE93_CHECKER_COMMAND}`,
    `run_step "Phase 94 DoS/resource governance checker tests" ${PHASE94_TEST_COMMAND}`,
    `run_step "Phase 94 DoS/resource governance checker" ${PHASE94_CHECKER_COMMAND}`,
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
  ].join("\n");
}
