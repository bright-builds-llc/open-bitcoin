import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { checkPhase123RuntimeTimingEvidenceIntegrity } from "./check-phase123-runtime-timing-evidence-integrity";
import { readText } from "./check-phase123-runtime-timing-evidence-integrity/filesystem.ts";
import { TARGET_FILES, type TargetFile } from "./check-phase123-runtime-timing-evidence-integrity/constants.ts";
import { completeFiles, type CorpusFile } from "./check-phase123-runtime-timing-evidence-integrity/test-fixtures.ts";
import { maybeFunction } from "./check-phase157-index-catch-up/rust-evidence.ts";
const daemonFile = "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs";
const canonicalFile = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs";
type Mutator = (files: Map<CorpusFile, string>) => void;
const tempRoots: string[] = [];
afterEach(() => {
  for (const root of tempRoots.splice(0)) rmSync(root, { force: true, recursive: true });
});
test("complete synthetic Phase 123 corpus passes", () => {
  // Arrange
  const root = createFixture();
  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: root });
  // Assert
  expect(failures).toEqual([]);
});
test("phase123_real_repository_corpus_passes", () => {
  // Arrange
  const repositoryRoot = path.resolve(import.meta.dir, "..");
  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({
    rootDir: repositoryRoot,
  });
  // Assert
  expect(failures).toEqual([]);
});
test("Phase 123 immutable live source snapshot passes", () => {
  // Arrange
  const root = writeFixture(liveFiles());
  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: root });
  // Assert
  expect(failures).toEqual([]);
});
const terminalFile = "packages/open-bitcoin-node/src/sync/session/emission_terminal.rs";
const inboundFile = "packages/open-bitcoin-rpc/src/context/inbound_wire.rs";
const listenerFile = "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs";
const tcpFile = "packages/open-bitcoin-node/src/sync/tcp.rs";
const exactSend = "session.send_encoded(buffer.bytes(), self.config.network.magic())?;";
const blockAck = "self.network.acknowledge_wire_message_written(message)?;";
test.each([
  ["ordinary ack before exact send", terminalFile, `${exactSend}\n            ${blockAck}`, `${blockAck}\n            ${exactSend}`, "P123 sync send-before-ack"],
  ["missing ordinary write", terminalFile, exactSend, "", "P123 sync send-before-ack"],
  ["ignored failed ordinary write", terminalFile, exactSend, "let _ = session.send_encoded(buffer.bytes(), self.config.network.magic());", "P123 sync send-before-ack"],
  ["fabricated outgoing bytes", terminalFile, "send_encoded(buffer.bytes(), self.config.network.magic())", "send_encoded(&fabricated, self.config.network.magic())", "P123 sync send-before-ack"],
  ["ungoverned outgoing encoding", terminalFile, ".encode_pending_network_message(peer_id, message)", ".encode_network_message(message)", "P123 sync send-before-ack"],
  ["incomplete outgoing batch", terminalFile, ".collect::<Result<Vec<_>, _>>()", ".next()", "P123 sync send-before-ack"],
  ["ordinary caller bypass", terminalFile, "self.send_all_with_cancel(session, peer_id, messages, should_cancel)?;", "self.send_untracked(session, messages)?;", "P123 governed outbound production delegation"],
  ["test-only outgoing helper", terminalFile, "    fn send_all_with_cancel", "    #[cfg(test)]\n    fn send_all_with_cancel", "P123 sync send-before-ack"],
  ["disconnected outgoing owner", terminalFile, "impl super::DurableSyncRuntime", "impl DisconnectedRuntime", "P123 ordinary transport runtime owner"],
  ["test-only outgoing owner", terminalFile, "impl super::DurableSyncRuntime", "#[cfg(test)]\nimpl super::DurableSyncRuntime", "P123 ordinary transport runtime owner"],
  ["TCP exact override removed", tcpFile, "fn send_encoded(", "fn unused_send_encoded(", "P123 concrete exact transport write"],
  ["TCP byte substitution", tcpFile, ".write_all(bytes)", ".write_all(&fabricated)", "P123 concrete exact transport write"],
  ["test-only TCP owner", tcpFile, "impl SyncPeerSession for TcpPeerSession", "#[cfg(test)]\nimpl SyncPeerSession for TcpPeerSession", "P123 concrete exact transport owner"],
  ["unguarded inbound carrier", inboundFile, "pub(crate) bytes: GenericPendingBuffer", "pub(crate) bytes: Vec<u8>", "P123 typed inbound carrier"],
  ["test-only inbound carrier", inboundFile, "pub(crate) struct EncodedWireResponse", "#[cfg(test)]\npub(crate) struct EncodedWireResponse", "P123 typed inbound carrier"],
  ["test-only inbound carrier with intervening attribute", inboundFile, "pub(crate) struct EncodedWireResponse", "#[cfg(test)]\n#[allow(dead_code)]\npub(crate) struct EncodedWireResponse", "P123 typed inbound carrier"],
  ["fabricated inbound encoding", inboundFile, "network.encode_pending_network_message(peer_id, &message)", "fabricate_bytes(&message)", "P123 pre-write complete-batch encoding"],
  ["test-only inbound factory", inboundFile, "    fn push_encoded", "    #[cfg(test)]\n    fn push_encoded", "P123 pre-write complete-batch encoding"],
  ["disconnected inbound factory", inboundFile, "impl ResolvedInboundWireResponses", "impl DisconnectedResponses", "P123 pre-write complete-batch encoding"],
  ["partial plan bypass", inboundFile, "let mut resolved = plan.resolve();", "let mut resolved = plan.partial();", "P123 inbound complete-batch resolution"],
  ["durable block encoding bypass", inboundFile, "resolved.push_encoded(response, Some(intent), None, &self.network, self.peer_id);", "resolved.push_untracked(response);", "P123 durable inbound governed encoding"],
  ["listener delegation removed", listenerFile, "if !acknowledge_inbound_response_write(&write_result, &mut response, &context).await", "if false", "P123 inbound Written-only acknowledgement"],
  ["listener exact byte substitution", listenerFile, "response.bytes.bytes(),\n                &resource_policy", "&fabricated,\n                &resource_policy", "P123 inbound Written-only acknowledgement"],
  ["Written gate made unconditional", listenerFile, "matches!(write_result, Ok(WriteWireMessageOutcome::Written))", "true", "P123 inbound Written-only delegation"],
  ["ordinary false completion gate", inboundFile, "return !was_written", "return was_written", "P123 inbound Written-only acknowledgement"],
  ["capability gate made unconditional", inboundFile, "return if was_written", "return if true", "P123 inbound Written-only acknowledgement"],
  ["capability gate widened", inboundFile, "return if was_written", "return if was_written || true", "P123 inbound Written-only acknowledgement"],
  ["unreachable Written delegation", listenerFile, "    acknowledge_encoded_wire_response(\n", "    return true;\n    acknowledge_encoded_wire_response(\n", "P123 inbound Written-only delegation"],
  ["intent gate made unconditional", inboundFile, "let outcome = if was_written", "let outcome = if true", "P123 inbound Written-only acknowledgement"],
  ["early duplicate credit", terminalFile, exactSend, `${blockAck}\n            ${exactSend}`, "P123 ordinary single post-write acknowledgement"],
  ["idle cancellation forwarding removed", "packages/open-bitcoin-node/src/sync/session.rs", "&outbound,\n                        controls.1,", "&outbound,\n                        &mut || false,", "P123 idle clock target send order"],
] as const)("rejects current production transport mutation: %s", (_label, file, from, to, diagnostic) => {
  // Arrange
  const files = liveFiles();
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toEqual([]);
  replaceRequired(files, file, from, to);
  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) });
  // Assert
  expect(failures.some((failure) => failure.startsWith(diagnostic))).toBe(true);
});
test.each(["comment", "literal", "disconnected", "test-only"] as const)(
  "rejects exact-send %s decoy in current production corpus", (kind) => {
    // Arrange
    const files = liveFiles();
    expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toEqual([]);
    const decoy = kind === "comment" ? `/* ${exactSend} */`
      : kind === "literal" ? `let decoy = r#"${exactSend}"#;`
      : kind === "test-only" ? `#[cfg(test)] fn disconnected() { ${exactSend} }`
      : `fn disconnected() { ${exactSend} }`;
    replaceRequired(files, terminalFile, exactSend, "");
    files.set(terminalFile, `${files.get(terminalFile)}\n${decoy}`);
    // Act
    const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) });
    // Assert
    expect(failures.some((failure) => failure.startsWith("P123 sync send-before-ack"))).toBe(true);
  },
);
test.each([
  ["configured daemon constructor removed", "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs", "DurableSyncRuntime::open_with_configured_runtime_activation(", "DurableSyncRuntime::open_with_runtime_activation("],
  ["daemon relay policy defaulted", "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs", "runtime.relay,", "RelayActivationConfig::default(),"],
  ["daemon block-serving policy defaulted", "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs", "runtime.block_serving,", "BlockRelayActivationPolicy::default(),"],
  ["daemon relay and block-serving arguments reordered", "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs", "runtime.relay,\n            runtime.block_serving,", "runtime.block_serving,\n            runtime.relay,"],
  ["daemon inbound activation disabled", "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs", "runtime.inbound.enabled,", "false,"],
  ["daemon index startup policy bypassed", "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs", "            basic_filter_startup_mode(runtime),", "            BasicFilterStartupMode::PreserveSaved,"],
  ["configured initialization removed", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "initialize_configured(&store, now, now, 0, false, u64::MAX, basic_filter_mode)?;", "initialize(&store, now, now, 0, false, u64::MAX)?;"],
  ["configured startup policy bypassed", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "u64::MAX, basic_filter_mode)?;", "u64::MAX, BasicFilterStartupMode::PreserveSaved)?;"],
  ["configured network relay policy removed", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            relay_activation,\n            block_relay_activation,\n            inbound_enabled,\n        );", "            RelayActivationConfig::default(),\n            block_relay_activation,\n            inbound_enabled,\n        );"],
  ["configured network block policy removed", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            block_relay_activation,\n            inbound_enabled,\n        );", "            BlockRelayActivationPolicy::default(),\n            inbound_enabled,\n        );"],
  ["configured network inbound activation removed", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            inbound_enabled,\n        );", "            false,\n        );"],
  ["compatibility configured delegation removed", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "Self::open_with_configured_runtime_activation(", "Self::open_configured("],
  ["compatibility saved policy disabled", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "BasicFilterStartupMode::PreserveSaved,", "BasicFilterStartupMode::Disabled,"],
  ["block-relay compatibility delegation removed", "packages/open-bitcoin-node/src/sync.rs", "Self::open_with_runtime_activation(", "Self::open_configured("],
  ["initialized preparation removed", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "Self::prepare_initialized_network(", "Self::disconnected_network("],
  ["initialized preparation commented", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "let network = Self::prepare_initialized_network(", "let network = /* Self::prepare_initialized_network( */ Self::disconnected_network("],
  ["initialized store substituted", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            &store,\n            managed,", "            &other_store,\n            managed,"],
  ["initialized managed authority substituted", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            managed,\n            &config,", "            other_managed,\n            &config,"],
  ["initialized config substituted", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            &config,\n            basic_filter_mode,", "            &other_config,\n            basic_filter_mode,"],
  ["initialized startup policy defaulted", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            basic_filter_mode,\n            relay_activation,", "            BasicFilterStartupMode::PreserveSaved,\n            relay_activation,"],
  ["runtime finish removed", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "Self::finish_initialized_runtime(store, network, config, consensus_params)", "Self::disconnected_runtime(store, network, config, consensus_params)"],
  ["runtime finish wrong authority", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "Self::finish_initialized_runtime(store, network, config, consensus_params)", "Self::finish_initialized_runtime(store, other_network, config, consensus_params)"],
  ["runtime finish before preparation", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "let network = Self::prepare_initialized_network(", "Self::finish_initialized_runtime(store, network, config, consensus_params);\n        let network = Self::prepare_initialized_network("],
  ["initialized local peer config substituted", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            local_config,\n            PolicyConfig::default(),", "            other_local_config,\n            PolicyConfig::default(),"],
  ["initialized network policy substituted", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            PolicyConfig::default(),\n            config.max_blocks_in_flight_per_peer,", "            other_policy,\n            config.max_blocks_in_flight_per_peer,"],
  ["initialized network bound substituted", "packages/open-bitcoin-node/src/sync/open_runtime.rs", "            config.max_blocks_in_flight_per_peer,\n            relay_activation,", "            usize::MAX,\n            relay_activation,"],
] as const)("rejects live snapshot %s", (_label, file, from, to) => {
  // Arrange
  const files = liveFiles();
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toEqual([]);
  replaceRequired(files, file, from, to);
  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) });
  // Assert
  expect(failures.some((failure) => /P123 .*activation/.test(failure))).toBe(true);
});
test("rejects initialization moved after configured network construction", () => {
  // Arrange
  const files = liveFiles();
  const file = "packages/open-bitcoin-node/src/sync/open_runtime.rs";
  const initialization = "let (lifecycle, _view, cache) =\n            initialize_configured(&store, now, now, 0, false, u64::MAX, basic_filter_mode)?;";
  replaceRequired(files, file, initialization, "");
  replaceRequired(files, file, "        if let Some(header_store)", `${initialization}\n        if let Some(header_store)`);
  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) });
  // Assert
  expect(failures.some((failure) => failure.startsWith("P123 configured sync production activation"))).toBe(true);
});
test("commented configured initialization cannot satisfy the production guard", () => {
  // Arrange
  const files = liveFiles();
  const call = "initialize_configured(&store, now, now, 0, false, u64::MAX, basic_filter_mode)?;";
  replaceRequired(files, "packages/open-bitcoin-node/src/sync/open_runtime.rs", call, `/* ${call} */ initialize(&store, now, now, 0, false, u64::MAX)?;`);
  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) });
  // Assert
  expect(failures.some((failure) => failure.startsWith("P123 configured sync production activation"))).toBe(true);
});
test.each([
  ["Idle collapsed into Closed", "P123 typed receive outcomes missing Idle", mutate("packages/open-bitcoin-node/src/sync/types.rs", "Idle", "Closed")],
  ["idle clock removed", "P123 idle clock target send order missing or out of order lettimestamp=(controls.0)();", mutateFirst("packages/open-bitcoin-node/src/sync/session.rs", "SyncPeerReceiveOutcome::Idle => { let timestamp =\n(controls.0)();", "SyncPeerReceiveOutcome::Idle => { let timestamp = timestamp;")],
  ["message receive clock removed", "P123 message receive clock missing or out of order lettimestamp=(controls.0)();", mutateFirst("packages/open-bitcoin-node/src/sync/session.rs", "SyncPeerReceiveOutcome::Message(message) => {\nlet timestamp = (controls.0)();", "SyncPeerReceiveOutcome::Message(message) => {\nlet timestamp = timestamp;")],
  ["idle target guard removed", "P123 whole-batch request accounting missing or out of order targeted.iter().any(|(target,_)|*target!=owner.peer())", mutate("packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "targeted.iter().any(|(target, _)| *target != owner.peer())", "false")],
  ["session timestamp propagation removed", "P123 manager receive ordering missing or out of order progress.record_activity(timestamp)", mutateFirst("packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "progress.record_activity(timestamp);", "progress.record_activity(other_timestamp);")],
  ["response-work session retention removed", "P123 idle clock target send order missing or out of order turn.requests.pending(self)", mutate("packages/open-bitcoin-node/src/sync/session.rs", "turn.requests.pending(self)", "false")],
  ["compact fallback ordinary tracking removed", "P123 timeout packets must flow unchanged into handoff", mutate("packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let targeted = owner.expire(timestamp).map_err(ManagedNetworkAuthorityError::from)?;\n    handoff_core(", "let targeted = owner.expire(timestamp).map_err(ManagedNetworkAuthorityError::from)?;\n    let targeted = Vec::new(); handoff_core(")],
  ["peer-targeted fallback send removed", "P123 idle clock target send order missing or out of order self.send_all_for_registered_peer_with_cancel(", mutateFirst("packages/open-bitcoin-node/src/sync/session.rs", "!self.send_all_for_registered_peer_with_cancel( session, turn.peer_id, &outbound, controls.1, turn.registration, turn.requests, )?", "!self.send_untracked(session, &outbound)?")],
  ["live-session reconciliation removed", "P123 post-manager message wake missing or out of order block_reconcile::reconcile_best_chain_for_live_session(self,current_timestamp)?", mutate("packages/open-bitcoin-node/src/sync/session.rs", "block_reconcile::reconcile_best_chain_for_live_session(self, current_timestamp)?;", "block_reconcile::reconcile_best_chain(self, current_timestamp)?;")],
  ["post-durable progress dispatch removed", "P123 post-manager message wake missing or out of order self.persist_progress_and_dispatch_tip()?;", mutate("packages/open-bitcoin-node/src/sync/session.rs", "self.persist_progress_and_dispatch_tip()?;", "self.persist_progress()?;")],
  ["peer emission completion moved before write", "P123 sync peer emission post-write completion missing or out of order capability.acknowledge_write()", mutate(terminalFile, "capability.acknowledge_write()", "drop(capability)")],
  ["ordinary response retention removed", "P123 pending response retention missing or out of order runtime.inflight_blocks.contains(hash)", mutate("packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "runtime.inflight_blocks.contains(hash)", "false")],
  ["compact fallback active-tip classification removed", "P123 session timestamp propagation missing self.block_extends_active_tip(block)?", mutate("packages/open-bitcoin-node/src/sync/session.rs", "self.block_extends_active_tip(block)?", "false")],
  [
    "five-second compact cadence regression removed",
    "P123 runtime timing tests missing phase123_compact_download_survives_five_second_idle_cadence_until_timeout",
    mutate(
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
      "phase123_compact_download_survives_five_second_idle_cadence_until_timeout",
      "phase123_compact_download_cadence_test_removed",
    ),
  ],
  [
    "slow-message timestamp regression removed",
    "P123 runtime timing tests missing phase123_slow_messages_without_idle_timestamp_compact_at_receipt",
    mutate(
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
      "phase123_slow_messages_without_idle_timestamp_compact_at_receipt",
      "phase123_slow_message_timestamp_test_removed",
    ),
  ],
  [
    "compact fallback response regression removed",
    "P123 runtime timing tests missing phase123_compact_timeout_fallback_consumes_matching_block_before_yield",
    mutate(
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
      "phase123_compact_timeout_fallback_consumes_matching_block_before_yield",
      "phase123_compact_fallback_response_test_removed",
    ),
  ],
  [
    "compact fallback persistence assertion removed",
    "P123 compact fallback response regression missing .load_block(expected_hash)",
    mutate(
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
      ".load_block(expected_hash)",
      ".load_block(BlockHash::from_byte_array([0_u8; 32]))",
    ),
  ],
  [
    "daemon live-session cancellation removed",
    "P123 daemon live-session cancellation missing or out of order sync_until_idle_with_clock_and_cancel(",
    mutate(
      "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
      "sync_until_idle_with_clock_and_cancel(",
      "sync_until_idle_with_clock(",
    ),
  ],
  [
    "bench restores old Option receive",
    "P123 first-party receive migration packages/open-bitcoin-bench/src/runtime_fixtures.rs missing Result<SyncPeerReceiveOutcome, SyncRuntimeError>",
    mutate("packages/open-bitcoin-bench/src/runtime_fixtures.rs", "Result<SyncPeerReceiveOutcome, SyncRuntimeError>", "Result<Option<WireNetworkMessage>, SyncRuntimeError>"),
  ],
  [
    "public receive re-export removed",
    "P123 public receive re-export missing SyncPeerReceiveOutcome",
    mutate("packages/open-bitcoin-node/src/lib.rs", "SyncPeerReceiveOutcome", "SyncPeerSession"),
  ],
  [
    "sync acknowledgement moved before send",
    "P123 sync send-before-ack missing or out of order self.network.acknowledge_wire_message_written(message)?;",
    mutate(terminalFile, `${exactSend}\n            ${blockAck}`, `${blockAck}\n            ${exactSend}`),
  ],
  [
    "inbound Written acknowledgement removed",
    "P123 inbound Written-only acknowledgement missing or out of order return !was_written || context .acknowledge_wire_message_written(&response.message) .is_ok();",
    mutate("packages/open-bitcoin-rpc/src/context/inbound_wire.rs", ".acknowledge_wire_message_written(&response.message)", ".record_response_intent(&response.message)"),
  ],
  [
    "encoding bypassed with fabricated bytes before write",
    "P123 pre-write complete-batch encoding missing or out of order match network.encode_pending_network_message(peer_id, &message)",
    mutate("packages/open-bitcoin-rpc/src/context/inbound_wire.rs", "match network.encode_pending_network_message(peer_id, &message)", "match fabricated_bytes(&message)"),
  ],
  [
    "acknowledgement moved before encoding",
    "P123 inbound complete-batch resolution missing or out of order let mut resolved = plan.resolve();",
    mutate("packages/open-bitcoin-rpc/src/context/inbound_wire.rs", "let mut resolved = plan.resolve();", "let mut resolved = fabricated_responses();"),
  ],
  [
    "exact encoding failure test removed",
    "P123 inbound encoding failure guard test missing phase123_inbound_encoding_failure_does_not_increment_served",
    mutate("packages/open-bitcoin-rpc/src/inbound_listener/tests.rs", "phase123_inbound_encoding_failure_does_not_increment_served", "phase123_inbound_encoding_failure_is_ignored"),
  ],
  [
    "encoded bytes are decoded at acknowledgement",
    "P123 inbound byte decoding must not contain decode_wire(&response.bytes)",
    append("packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs", "decode_wire(&response.bytes);"),
  ],
  [
    "eligible peer proxy restored",
    "P123 served metric eligibility proxy must not contain eligible_peer_count as f64",
    mutate("packages/open-bitcoin-node/src/metrics/block_relay.rs", "served_count as f64", "eligible_peer_count as f64"),
  ],
  [
    "served_count added to public status",
    "P123 unchanged public block-relay status must not contain served_count",
    append("packages/open-bitcoin-node/src/status/block_relay_evidence.rs", "pub served_count: u64,"),
  ],
  [
    "obsolete provider restored",
    "P123 obsolete block-relay provider must not contain set_block_relay_metric_status_provider",
    append("packages/open-bitcoin-node/src/sync.rs", "set_block_relay_metric_status_provider(provider);"),
  ],
  [
    "daemon sync activation removed",
    "P123 daemon sync activation wiring must preserve configured activation arguments",
    mutate(
      "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
      "open_configured_history_runtime(&runtime)?",
      "None",
    ),
  ],
  [
    "inbound block-serving activation removed",
    "P123 inbound production activation wiring missing or out of order config.block_serving",
    mutate(
      "packages/open-bitcoin-rpc/src/context/canonical_chain.rs",
      "config.block_serving",
      "BlockRelayActivationPolicy::default()",
    ),
  ],
  [
    "structured log resamples",
    "P123 shared metric/log snapshot missing or out of order self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);",
    mutate("packages/open-bitcoin-node/src/sync.rs", "self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);", "let second = self.maybe_authoritative_block_relay_snapshot(); self.write_block_relay_log(&mut summary, second.as_ref(), timestamp);"),
  ],
  [
    "unavailable omission test removed",
    "P123 runtime projection tests missing phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log",
    mutate("packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs", "phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log", "phase123_unobserved_authoritative_network_emits_zero_block_relay_metrics_and_log"),
  ],
  [
    "one HARD requirement mutated",
    "P123 parity requirements must be exactly HARD-02,HARD-03,HARD-04",
    mutate("docs/parity/index.json", "HARD-03", "HARD-05"),
  ],
] as const)("fails the %s mutation", (_label, expectedFailure, maybeMutate) => {
  // Arrange
  const root = createFixture(maybeMutate as Mutator);
  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: root });
  // Assert
  expect(failures).toContain(expectedFailure);
});
function createFixture(maybeMutate?: Mutator): string {
  const files = completeFiles();
  maybeMutate?.(files);
  return writeFixture(files);
}
test.each([
  ["dispatcher", daemonFile, "    run_application()\n}", "    unused_application()\n}"],
  ["missing declaration", daemonFile, '#[path = "open_bitcoind/canonical_startup.rs"]\nmod canonical_startup;', ""],
  ["wrong declaration", daemonFile, 'path = "open_bitcoind/canonical_startup.rs"', 'path = "open_bitcoind/wrong.rs"'],
  ["declaration comment decoy", daemonFile, '#[path = "open_bitcoind/canonical_startup.rs"]\nmod canonical_startup;', '/* #[path = "open_bitcoind/canonical_startup.rs"] mod canonical_startup; */\n#[path = "open_bitcoind/wrong.rs"] mod canonical_startup;'],
  ["missing import", daemonFile, "use canonical_startup::{\n    open_authoritative_network_runtime, open_configured_history_runtime, open_runtime_store,\n};", ""],
  ["disabled declaration", daemonFile, '#[path = "open_bitcoind/canonical_startup.rs"]\nmod canonical_startup;', '#[cfg(any())]\n#[path = "open_bitcoind/canonical_startup.rs"]\nmod canonical_startup;'],
  ["disabled import", daemonFile, "use canonical_startup::{\n    open_authoritative_network_runtime, open_configured_history_runtime, open_runtime_store,\n};", "#[cfg(any())]\nuse canonical_startup::{\n    open_authoritative_network_runtime, open_configured_history_runtime, open_runtime_store,\n};"],
  ["wrong import", daemonFile, "use canonical_startup::{\n    open_authoritative_network_runtime, open_configured_history_runtime, open_runtime_store,\n};", "use canonical_startup::{wrong_runtime, open_runtime_store};"],
  ["disabled application", daemonFile, "#[tokio::main]", "#[cfg(any())]\n#[tokio::main]"],
  ["borrowed capability", canonicalFile, "maybe_opened: Option<OpenedCanonicalGenesis>", "maybe_opened: Option<&OpenedCanonicalGenesis>"],
  ["missing opener preflight", canonicalFile, "let chain = preflight(runtime)?;\n    let Some(opened)", "let chain = selected_canonical_chain(runtime)?;\n    let Some(opened)"],
  ["missing capability refusal", canonicalFile, "let Some(opened) = maybe_opened else {\n        // There is no earned transient canonical owner in this phase.\n        return Err(compatibility_refusal(runtime, chain));\n    };", "let opened = maybe_opened.unwrap_or_default();"],
  ["wrong factory", canonicalFile, "ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)", "ManagedRpcContext::from_runtime_config_with_network_handle(runtime, opened)"],
  ["test-only decoy with raw factory", canonicalFile, "ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)", "ManagedRpcContext::from_runtime_config_with_network_handle(runtime, opened)", true],
  ["test-only decoy with default factory", canonicalFile, "ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)", "ManagedRpcContext::from_runtime_config(runtime, opened)", true],
  ["test-only decoy with fixture factory", canonicalFile, "ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)", "ManagedRpcContext::from_fixture_runtime_config_with_network_handle(runtime, opened)", true],
  ["wrong factory owner", canonicalFile, "from_canonical_genesis_runtime_config(runtime, opened)", "from_canonical_genesis_runtime_config(runtime, borrowed)"],
  ["wrong assembled runtime", canonicalFile, "assemble_durable_runtime(runtime, sync_runtime)?", "assemble_durable_runtime(runtime, other_runtime)?"],
  ["wrong prebuilt context", canonicalFile, "authoritative.maybe_prebuilt_context = Some(context);", "authoritative.maybe_prebuilt_context = Some(other_context);"],
  ["ordinary runtime arguments", daemonFile, "&runtime,\n            authoritative_runtime.network.clone(),", "&RuntimeConfig::default(),\n            authoritative_runtime.network.clone(),"],
  ["ordinary network arguments", daemonFile, "authoritative_runtime.network.clone(),\n            maybe_runtime_store.clone(),", "other_network.clone(),\n            maybe_runtime_store.clone(),"],
  ["ordinary store arguments", daemonFile, "maybe_runtime_store.clone(),\n        )?,", "None,\n        )?,"],
  ["context substitution", daemonFile, "Some(context) => context,", "Some(context) => other_context,"],
  ["activation bypass", canonicalFile, "if activation {", "if false {"],
  ["test-only owner", canonicalFile, "pub(super) fn open_authoritative_network_runtime(", "#[cfg(test)]\npub(super) fn open_authoritative_network_runtime("],
  ["projected store substitution", daemonFile, ".map(|sync| sync.store().clone())", ".map(|sync| other_store.clone())"],
] as const)("current canonical startup refuses %s", (...args) => {
  const [_label, file, from, to, maybeDecoy] = args;
  // Arrange: each mutation targets one actual live span after a clean baseline.
  const files = liveFiles();
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toEqual([]);
  const source = files.get(file)!;
  const maybeOwner = file === canonicalFile ? maybeFunction(source, "open_authoritative_network_runtime", true) : undefined;
  const target = maybeOwner?.body.includes(from) ? source.slice(maybeOwner.start, maybeOwner.end) : source;
  expect(target.split(from)).toHaveLength(2);
  expect(from).not.toBe(to);
  const changed = target.replace(from, () => to);
  files.set(file, target === source ? changed : source.slice(0, maybeOwner!.start) + changed + source.slice(maybeOwner!.end));
  if (maybeDecoy === true) {
    const owner = maybeFunction(source, "open_authoritative_network_runtime", true)!;
    files.set(file, "#[cfg(test)]\npub(super) " + source.slice(owner.start, owner.end) + "\n" + files.get(file)!);
    expect(maybeFunction(files.get(file)!, "open_authoritative_network_runtime", true)!.body).toContain(to);
  }
  // Act / Assert
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toContain("P123 daemon sync activation wiring must preserve configured activation arguments");
});
test("declared canonical startup requires its actual fixed physical child", () => {
  // Arrange
  const files = liveFiles();
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toEqual([]);
  expect(files.delete(canonicalFile)).toBe(true);
  // Act / Assert
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toContain(`P123 missing required declared canonical startup child: ${canonicalFile}`);
});
for (const [name, file, from, to, diagnostic] of [
  ["unrequested membership bypass", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "if !requested.contains(&hash) {", "if false && !requested.contains(&hash) {", "P123 whole-batch request membership: exact reachable branch required"],
  ["new-at-cap bypass", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "if hashes.difference(inflight).count() > cap.saturating_sub(inflight.len()) {", "if false && hashes.difference(inflight).count() > cap.saturating_sub(inflight.len()) {", "P123 whole-batch request capacity: exact reachable branch required"],
  ["all-peer release bypass", "packages/open-bitcoin-node/src/sync/block_reconcile.rs", "if !retained.contains(hash) {", "if false && !retained.contains(hash) {", "P123 all-peer post-manager release: exact reachable branch required"],
  ["Regtest activation bypass", canonicalFile, "if chain != CanonicalChain::Regtest || !trigger {", "if false { if chain != CanonicalChain::Regtest || !trigger { return Ok(None); } } if false {", "P123 daemon bounded Regtest activation gate: exact reachable branch required"],
  ["wrong target bypass", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "if targeted.iter().any(|(target, _)| *target != owner.peer()) {", "if false && targeted.iter().any(|(target, _)| *target != owner.peer()) {", "P123 whole-batch request target: exact reachable branch required"],
  ["duplicate slot charge", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "hashes.difference(inflight).count() > cap.saturating_sub(inflight.len())", "hashes.len() > cap.saturating_sub(inflight.len())", "P123 whole-batch request capacity: exact reachable branch required"],
  ["already-accounted resend denied at cap", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let requested: BTreeSet<_> = owner.requested()?.into_iter().collect();", "if cap.saturating_sub(inflight.len()) == 0 { return Err(SyncRuntimeError::ResourceLimit { message: \"full\".to_owned(), }); }\n    let requested: BTreeSet<_> = owner.requested()?.into_iter().collect();", "P123 already-accounted resend must remain available at capacity"],
  ["already-accounted resend denied by length precheck", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let requested: BTreeSet<_> = owner.requested()?.into_iter().collect();", "if inflight.len() >= cap { return Err(SyncRuntimeError::ResourceLimit { message: \"full\".to_owned(), }); }\n    let requested: BTreeSet<_> = owner.requested()?.into_iter().collect();", "P123 already-accounted resend must remain available at capacity"],
  ["already-accounted resend dropped by empty result", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let requested: BTreeSet<_> = owner.requested()?.into_iter().collect();", "let requested: BTreeSet<_> = owner.requested()?.into_iter().collect(); if inflight.len() >= cap { return Ok(Vec::new()); }", "P123 whole-batch request accounting: reviewed production body changed"],
  ["target packet payload changed", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "targeted.into_iter().map(|(_, message)| message).collect()", "targeted.into_iter().filter_map(|(_, message)| Some(message)).collect()", "P123 whole-batch request accounting missing or out of order targeted.into_iter().map(|(_,message)|message).collect()"],
  ["witness block membership omitted", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "InventoryType::Block | InventoryType::WitnessBlock", "InventoryType::Block", "P123 whole-batch request accounting missing or out of order InventoryType::Block|InventoryType::WitnessBlock"],
  ["ordinary nonempty batch exits before write", terminalFile, "let buffers = messages", "if !messages.is_empty() { return Ok(()); } let buffers = messages", "P123 ordinary reviewed send body changed before exact write"],
] as const) {
  test(`current reviewed branch refuses ${name}`, () => {
    const files = liveFiles();
    expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toEqual([]);
    replaceRequired(files, file, from, to);
    expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toContain(diagnostic);
  });
}
for (const kind of ["comment", "literal", "disconnected", "test-only"] as const) {
  test(`current request membership refuses ${kind} decoy`, () => {
    const files = liveFiles();
    const file = "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs";
    const predicate = "if !requested.contains(&hash) {";
    replaceRequired(files, file, predicate, "if false && !requested.contains(&hash) {");
    const decoy = kind === "comment" ? `/* ${predicate} */`
      : kind === "literal" ? `let decoy = r#"${predicate}"#;`
      : kind === "test-only" ? `#[cfg(test)] fn decoy() { ${predicate} } }`
      : `fn decoy() { ${predicate} } }`;
    files.set(file, `${files.get(file)}\n${decoy}`);
    expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) }))
      .toContain("P123 whole-batch request membership: exact reachable branch required");
  });
}
for (const [name, from, to, diagnostic] of [
  ["wrong registration", "Some((registration, requests)),", "Some((foreign_registration, requests)),", "P123 registered peer must forward its exact registration, requests and cancellation"],
  ["disabled cancellation", "should_cancel,\n            Some((registration, requests)),", "&mut || false,\n            Some((registration, requests)),", "P123 registered peer must forward its exact registration, requests and cancellation"],
  ["wrong outbox peer", "take_registration(\n                        owned_peer,", "take_registration(\n                        foreign_peer,", "P123 registered send must use the token outbox and propagate its result before emission"],
  ["direct unregistered take", "self.announcement_outboxes.take_registration(", "self.announcement_outboxes.take_peer_emissions(", "P123 registered send must use the token outbox and propagate its result before emission"],
  ["early registered completion", "Some((registration, requests)) => {", "Some((registration, requests)) => { return Ok(false);", "P123 registered send must use the token outbox and propagate its result before emission"],
] as const) {
  test(`registered production send refuses ${name}`, () => {
    const files = liveFiles();
    replaceRequired(files, terminalFile, from, to);
    expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toContain(diagnostic);
  });
}
for (const kind of ["comment", "literal", "disconnected", "test-only"] as const) {
  test(`registered production send refuses ${kind} decoy`, () => {
    const files = liveFiles();
    const actual = "self.announcement_outboxes.take_registration(";
    replaceRequired(files, terminalFile, actual, "self.announcement_outboxes.take_peer_emissions(");
    const decoy = kind === "comment" ? `/* ${actual} */`
      : kind === "literal" ? `let decoy = r#"${actual}"#;`
      : kind === "test-only" ? `#[cfg(test)] fn decoy() { ${actual} }`
      : `fn decoy() { ${actual} }`;
    files.set(terminalFile, `${files.get(terminalFile)}\n${decoy}`);
    expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) }))
      .toContain("P123 registered send must use the token outbox and propagate its result before emission");
  });
}
for (const [name, file, from, to, diagnostic] of [
  ["same hash held by another peer", "packages/open-bitcoin-node/src/sync/block_reconcile.rs", "if !retained.contains(hash) {", "if !retained.contains(hash) || true {", "P123 all-peer post-manager release: exact reachable branch required"],
  ["manager Ok receipt cleanup skipped", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let receipt_cleanup = release_unrequested_hashes(runtime, receipt);", "let receipt_cleanup = if result.is_err() { release_unrequested_hashes(runtime, receipt) } else { Ok(()) };", "P123 post-manager Ok/Err receipt cleanup must propagate both results"],
  ["manager Err receipt cleanup skipped", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let receipt_cleanup = release_unrequested_hashes(runtime, receipt);", "let receipt_cleanup = if result.is_ok() { release_unrequested_hashes(runtime, receipt) } else { Ok(()) };", "P123 post-manager Ok/Err receipt cleanup must propagate both results"],
  ["unrequested Block cleanup skipped", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "request_cleanup_result(\n            observation_cleanup,\n            release_unrequested_hashes(runtime, receipt),", "request_cleanup_result(\n            observation_cleanup,\n            if false { release_unrequested_hashes(runtime, receipt) } else { Ok(()) },", "P123 unrequested Block cleanup must propagate both results"],
  ["disconnect cleanup skipped", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let observed = runtime.network.finish_sync_peer(&self.token)?;\n        self.reconcile(runtime, observed).map(|_| ())", "let observed = runtime.network.finish_sync_peer(&self.token)?;\n        Ok(())", "P123 disconnect cleanup must preserve retained and denied request ownership"],
  ["dual errors hidden", "packages/open-bitcoin-node/src/sync/session.rs", "(Err(error), Err(cleanup)) => Err(SyncRuntimeError::Network {", "(Err(error), Err(_cleanup)) => Err(SyncRuntimeError::Network {", "P123 primary and cleanup failures must remain visible together"],
  ["pre-manager raw release", "packages/open-bitcoin-node/src/sync/session.rs", "let received =\n            requests.dispatch", "block_reconcile::release_inflight_for_message(self, &message);\n        let received =\n            requests.dispatch", "P123 raw pre-manager release must not contain block_reconcile::release_inflight_for_message"],
  ["all-peer collector truncated", "packages/open-bitcoin-node/src/sync/block_reconcile.rs", ".peer_ids()\n        .into_iter()\n        .filter_map", ".peer_ids()\n        .into_iter()\n        .take(1)\n        .filter_map", "P123 all-peer retained membership must be complete"],
  ["all-peer retained set shadowed", "packages/open-bitcoin-node/src/sync/block_reconcile.rs", ".collect();\n    for hash in hashes", ".collect();\n    let retained = retained.into_iter().take(1).collect::<BTreeSet<_>>();\n    for hash in hashes", "P123 all-peer post-manager release: reviewed production body changed"],
  ["retained set release omitted", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let previous = release_unrequested_hashes(runtime, &self.retained);", "let previous = Ok(());", "P123 retained and current request release must propagate through absence and presence"],
  ["current set release omitted", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "let current = release_unrequested_hashes(runtime, &requests);", "let current = Ok(());", "P123 retained and current request release must propagate through absence and presence"],
  ["absent peer hides prior cleanup", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "previous?;\n            self.maybe_peer = None;", "let _ = previous;\n            self.maybe_peer = None;", "P123 retained and current request release must propagate through absence and presence"],
  ["manager handoff packets cleared", "packages/open-bitcoin-node/src/sync/session.rs", "        filter_serving::send_filter_plans(", "        outbound.clear();\n        filter_serving::send_filter_plans(", "P123 manager handoff packets must remain intact until registered send"],
  ["manager handoff packets reassigned", "packages/open-bitcoin-node/src/sync/session.rs", "        filter_serving::send_filter_plans(", "        outbound = Vec::new();\n        filter_serving::send_filter_plans(", "P123 reviewed post-manager message flow: reviewed production body changed"],
  ["Idle targeted packets shadowed", "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", "                .map_err(ManagedNetworkAuthorityError::from)?;\n            handoff_core(", "                .map_err(ManagedNetworkAuthorityError::from)?;\n            let targeted = Vec::new();\n            handoff_core(", "P123 timeout packets must flow unchanged into handoff"],
  ["ordinary TCP write made dead", terminalFile, "session.send_encoded(buffer.bytes(), self.config.network.magic())?;", "if false { session.send_encoded(buffer.bytes(), self.config.network.magic())?; }", "P123 ordinary exact write must execute before acknowledgement in the same loop"],
] as const) {
  test(`current request cleanup refuses ${name}`, () => {
    const files = liveFiles();
    replaceRequired(files, file, from, to);
    expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) })).toContain(diagnostic);
  });
}
for (const [name, message] of [
  ["wrong-target error", "compact timeout action target does not match connected session"],
  ["missing membership error", "request batch lacks retained peer membership"],
  ["global capacity error", "request batch exceeds global in-flight capacity"],
] as const) {
  test(`current request error literal refuses ${name}`, () => {
    const files = liveFiles();
    replaceRequired(files, "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", message, "misleading request failure");
    expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) }))
      .toContain("P123 whole-batch request error literals changed");
  });
}
test("current PeerRequests handoff refuses a disconnected impl owner", () => {
  const files = liveFiles();
  const file = "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs";
  replaceRequired(files, file, "impl PeerRequests {", "impl DisconnectedRequests {");
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) }))
    .toContain("P123 message target handoff: unique enabled production owner required");
});
test("current free handoff refuses a disconnected module owner", () => {
  const files = liveFiles();
  const file = "packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs";
  const source = files.get(file)!;
  const handoff = maybeFunction(source, "handoff_core", true)!;
  files.set(file, source.slice(0, handoff.start) + "mod disconnected {\n" + source.slice(handoff.start, handoff.end)
    + "\n}\n" + source.slice(handoff.end));
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) }))
    .toContain("P123 whole-batch request accounting: unique enabled production owner required");
});
test("current ordinary writer refuses a disabled impl owner", () => {
  const files = liveFiles();
  replaceRequired(files, terminalFile, "impl super::DurableSyncRuntime {", "#[cfg(any())]\nimpl super::DurableSyncRuntime {");
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) }).some(row =>
    row.startsWith("P123 ordinary transport runtime owner"))).toBe(true);
});
test("current peer emission refuses dead TCP write before completion", () => {
  const files = liveFiles();
  const branch = `if let Err(write_error) = session.send_encoded(buffer.bytes(), network_magic) {
            drop(buffer);
            drop(buffers);
            return combined_error(
                write_error,
                abort_current_and_suffix(network, capability, emissions),
            );
        }`;
  replaceRequired(files, terminalFile, branch, `if false { ${branch} }`);
  expect(checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: writeFixture(files) }))
    .toContain("P123 peer emission write before completion: exact reachable branch required");
});
function writeFixture(files: Map<CorpusFile, string>): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase123-"));
  tempRoots.push(root);
  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }
  return root;
}
function liveFiles(): Map<CorpusFile, string> {
  const failures: string[] = [];
  const repo = path.resolve(import.meta.dir, "..");
  const files = new Map<CorpusFile, string>(TARGET_FILES.map((file) => [file, file === daemonFile ? readFileSync(path.join(repo, file), "utf8") : readText(repo, file, failures)]));
  files.set(canonicalFile, readFileSync(path.join(repo, canonicalFile), "utf8"));
  expect(failures).toEqual([]);
  return files;
}
function replaceRequired(files: Map<CorpusFile, string>, file: CorpusFile, from: string, to: string): void {
  const source = files.get(file) ?? "";
  expect(source).toContain(from);
  files.set(file, source.replace(from, to));
}
function mutate(file: TargetFile, from: string, to: string): Mutator {
  return (files) => {
    expect(files.get(file)).toContain(from);
    files.set(file, (files.get(file) ?? "").replaceAll(from, to));
  };
}
function mutateFirst(file: TargetFile, from: string, to: string): Mutator {
  return (files) => replaceRequired(files, file, from, to);
}
function append(file: TargetFile, addition: string): Mutator {
  return (files) => files.set(file, `${files.get(file) ?? ""}\n${addition}`);
}
