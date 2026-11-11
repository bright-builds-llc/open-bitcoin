import path from "node:path";
import { createHash } from "node:crypto";
import { DEFAULT_REPO_ROOT, TARGET_FILES, TargetFile, CheckOptions } from "./constants.ts";
import { readText } from "./filesystem.ts";
import { verifySuccessfulWriteEvidence, verifyPrivateProjection, verifyAuthoritativeRuntime, verifyFocusedTests } from "./evidence.ts";
import { verifyPhase121Compatibility, verifyParity, verifyBreadcrumbs } from "./parity.ts";
import { normalizeWhitespace, requireContains, requireAbsent, requireOrdered } from "./helpers.ts";
import { normalizeRust, rustCallArguments, rustFunction, rustLetInitializers, stripRustNonCode } from "../rust-source-invariants.ts";
import { maybeFunction, rustCode } from "../check-phase157-index-catch-up/rust-evidence.ts";

export function checkPhase123RuntimeTimingEvidenceIntegrity(
  options: CheckOptions = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();
  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyTypedReceiveAndTcp(texts, failures);
  verifyProductionActivation(texts, failures);
  verifyIdleMaintenance(texts, failures);
  verifySuccessfulWriteEvidence(texts, failures);
  verifyPrivateProjection(texts, failures);
  verifyAuthoritativeRuntime(texts, failures);
  verifyFocusedTests(texts, failures);
  verifyPhase121Compatibility(texts, failures);
  verifyParity(texts, failures);
  verifyBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  return failures;
}

export function verifyProductionActivation(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  requireContains(
    normalizeWhitespace(rustFunction(sync, "pub fn open_with_block_relay_activation(")),
    "Self::open_with_runtime_activation( store, config, RelayActivationConfig::default(), block_relay_activation, false, )",
    "P123 block-relay compatibility activation",
    failures,
  );
  const openRuntime = texts.get("packages/open-bitcoin-node/src/sync/open_runtime.rs") ?? "";
  requireContains(
    normalizeWhitespace(rustFunction(openRuntime, "pub fn open_with_runtime_activation(")),
    "Self::open_with_configured_runtime_activation( store, config, relay_activation, block_relay_activation, inbound_enabled, BasicFilterStartupMode::PreserveSaved, )",
    "P123 runtime compatibility activation",
    failures,
  );
  const configured = rustFunction(openRuntime, "pub fn open_with_configured_runtime_activation(");
  requireOrdered(
    normalizeRust(configured),
    [
      "initialize_configured(&store, now, now, 0, false, u64::MAX, basic_filter_mode)?;",
      "store.load_chain_meta_for_open()?;",
      "store.load_all_undo_records()?;",
      "Chainstate::from_coins_cache(cache, active_chain, undo_by_block, maybe_counts)",
      "ManagedChainstate::from_recovered_chainstate( FjallChainstateStore::from_store(store.clone()), chainstate, lifecycle, )?;",
      "let network = Self::prepare_initialized_network(",
      "let consensus_params = config.network.consensus_params();",
      "Self::finish_initialized_runtime(store, network, config, consensus_params)",
    ].map(normalizeRust),
    "P123 configured sync production activation",
    failures,
  );
  const prepareCalls = rustCallArguments(configured, "Self::prepare_initialized_network");
  const finishCalls = rustCallArguments(configured, "Self::finish_initialized_runtime");
  const prepareArguments = ["&store", "managed", "&config", "basic_filter_mode", "relay_activation", "block_relay_activation", "inbound_enabled"];
  const finishArguments = ["store", "network", "config", "consensus_params"];
  if (prepareCalls.length !== 1 || prepareCalls[0]?.map(normalizeRust).join("|") !== prepareArguments.join("|")
    || finishCalls.length !== 1 || finishCalls[0]?.map(normalizeRust).join("|") !== finishArguments.join("|")) {
    failures.push("P123 configured sync production activation must preserve preparation and finish arguments");
  }
  const prepare = rustFunction(openRuntime, "pub(super) fn prepare_initialized_network(");
  const networkCalls = rustCallArguments(prepare, "ManagedPeerNetwork::from_initialized_chainstate");
  const networkArguments = ["managed", "local_config", "PolicyConfig::default()", "config.max_blocks_in_flight_per_peer", "relay_activation", "block_relay_activation", "inbound_enabled"];
  if (networkCalls.length !== 1 || networkCalls[0]?.map(normalizeRust).join("|") !== networkArguments.join("|")) {
    failures.push("P123 configured network activation must preserve initialized constructor arguments");
  }
  requireOrdered(
    normalizeRust(prepare),
    [
      "let maybe_configured_basic_indexing = match basic_filter_mode {",
      "BasicFilterStartupMode::Enabled => Some(true),",
      "BasicFilterStartupMode::Disabled => Some(false),",
      "BasicFilterStartupMode::PreserveSaved => { store.resolved_basic_filter_indexing_after_startup()? }",
      "let local_config = progress::local_peer_config(config);",
      "let mut network = ManagedPeerNetwork::from_initialized_chainstate(",
      "network.record_basic_indexing_configuration(maybe_configured_basic_indexing);",
      "if let Some(header_store) = store.load_header_store()? { network.seed_header_store(header_store); }",
      "Ok(network)",
    ].map(normalizeRust),
    "P123 configured network production activation",
    failures,
  );
  const daemon = texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  verifyDaemonActivation(daemon, failures);
  const fixture = texts.get("packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs") ?? "";
  const fixtureCalls = rustCallArguments(rustFunction(fixture, "pub(crate) fn fixture_open_authoritative_network_runtime("), "DurableSyncRuntime::open_with_configured_runtime_activation");
  const fixtureArguments = ["store", "runtime.sync.runtime.clone()", "runtime.relay", "runtime.block_serving", "runtime.inbound.enabled", "basic_filter_startup_mode(runtime)"];
  if (fixtureCalls.length !== 1 || fixtureCalls[0]?.map(normalizeRust).join("|") !== fixtureArguments.join("|")) {
    failures.push("P123 explicit fixture activation must preserve configured activation arguments");
  }
  for (const file of [
    "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs",
  ] as const) {
    requireContains(
      texts.get(file) ?? "",
      "DurableSyncRuntime::open_with_block_relay_activation(",
      `P123 production activation test path ${file}`,
      failures,
    );
  }

  const rpcNetwork = rustFunction(texts.get("packages/open-bitcoin-rpc/src/context/canonical_chain.rs") ?? "", "pub(super) fn build_context_network(");
  requireOrdered(
    rpcNetwork,
    ["ManagedPeerNetwork::new_with_block_relay_activation(", "config.block_serving"],
    "P123 inbound production activation wiring",
    failures,
  );
  const inboundTests = texts.get("packages/open-bitcoin-rpc/src/inbound_listener/tests.rs") ?? "";
  for (const needle of [
    "phase123_enabled_runtime_config_serves_and_acknowledges_inbound_block",
    "phase123_disabled_runtime_config_does_not_serve_inbound_block",
  ]) {
    requireContains(inboundTests, needle, "P123 inbound production activation tests", failures);
  }
}

function verifyDaemonActivation(daemon: string, failures: string[]): void {
  const code = stripRustNonCode(daemon);
  const fail = () => failures.push("P123 daemon sync activation wiring must preserve configured activation arguments");
  const declarations = [...daemon.matchAll(/#\[path = "open_bitcoind\/canonical_startup\.rs"\]\s*mod canonical_startup;/g)]
    .filter(match => code.slice(match.index).startsWith("#[path"));
  const imports = [...code.matchAll(/\buse\s+canonical_startup\s*::\s*\{\s*open_authoritative_network_runtime\s*,\s*open_configured_history_runtime\s*,\s*open_runtime_store\s*,?\s*\}\s*;/g)];
  const unconditional = (start: number) => !code.slice(Math.max(code.lastIndexOf(";", start - 1), code.lastIndexOf("}", start - 1)) + 1, start).trim();
  const owner = (name: string, attributes: string) => {
    const maybeOwner = maybeFunction(daemon, name, true);
    return maybeOwner && normalizeRust(maybeOwner.attributes) === attributes
      && directRustOwner(daemon, maybeOwner.start, null) ? maybeOwner : undefined;
  };
  const main = owner("main", ""), application = owner("run_application", "#[tokio::main]async");
  const serve = owner("serve_authoritative_runtime", "async"), opened = owner("open_authoritative_network_runtime", "pub(super)");
  const history = owner("open_configured_history_runtime", "pub(super)");
  const preflight = owner("preflight", "");
  if (declarations.length !== 1 || imports.length !== 1 || !unconditional(declarations[0].index) || !unconditional(imports[0].index)
    || !main || !application || !serve || !opened || !history || !preflight) { fail(); return; }
  const exactInitializer = (body: string, binding: string, expression: string) => {
    const initializers = rustLetInitializers(body, binding);
    return initializers.length === 1 && normalizeRust(initializers[0]) === normalizeRust(expression);
  };
  const mainBody = "if let Some(code) = FjallNodeStore::maybe_run_inspection_worker() { std::process::exit(code); } run_application()";
  const ordinaryContext = "match authoritative_runtime.maybe_prebuilt_context.take() { Some(context) => context, None => ManagedRpcContext::from_runtime_config_with_network_handle(&runtime, authoritative_runtime.network.clone(), maybe_runtime_store.clone(),)?, }";
  const serveArgs = ["runtime", "bind_address", "auth", "maybe_runtime_store", "authoritative_runtime"];
  const serveCalls = rustCallArguments(application.body, "serve_authoritative_runtime");
  const stores = rustLetInitializers(application.body, "maybe_runtime_store").map(normalizeRust);
  const contexts = rustLetInitializers(serve.body, "context").map(normalizeRust);
  const projectedStore = "match &opened { OpenedAuthoritativeRuntime::Durable(authoritative) => authoritative.maybe_sync_runtime.as_ref().map(|sync| sync.store().clone()), #[cfg(test)] OpenedAuthoritativeRuntime::Transient(_) => None, }";
  const selectedOpen = "match open_configured_history_runtime(&runtime)? { Some(opened) => opened, None => { let maybe_runtime_store = open_runtime_store(&runtime)?; open_authoritative_network_runtime(&runtime, maybe_runtime_store)? } }";
  if (normalizeRust(main.body) !== normalizeRust(mainBody)
    || !exactInitializer(application.body, "runtime", "load_runtime_config()?")
    || stores.length !== 2 || stores[0] !== normalizeRust("open_runtime_store(&runtime)?") || stores[1] !== normalizeRust(projectedStore)
    || !exactInitializer(application.body, "opened", selectedOpen)
    || serveCalls.length !== 2 || serveCalls.some(args => args.map(normalizeRust).join("|") !== serveArgs.join("|"))
    || contexts.length !== 2 || contexts[0] !== normalizeRust(ordinaryContext) || contexts[1] !== "shared_context.lock().await") fail();
  // This closed capability flow supplements, never replaces, ordinary and fixture activation checks.
  const expectedOpener = `fn open_authoritative_network_runtime(runtime: &RuntimeConfig, maybe_opened: Option<OpenedCanonicalGenesis>,) -> Result<OpenedAuthoritativeRuntime, DaemonSyncPreflightError> {
    let chain = preflight(runtime)?;
    let Some(opened) = maybe_opened else { return Err(compatibility_refusal(runtime, chain)); };
    let (context, sync_runtime) = ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened).map_err(|_| compatibility_refusal(runtime, chain))?;
    match assemble_durable_runtime(runtime, sync_runtime)? {
      OpenedAuthoritativeRuntime::Durable(mut authoritative) => { authoritative.maybe_prebuilt_context = Some(context); Ok(OpenedAuthoritativeRuntime::Durable(authoritative)) }
      #[cfg(test)] OpenedAuthoritativeRuntime::Transient(_) => Err(compatibility_refusal(runtime, chain)),
    }
  }`;
  const expectedPreflight = `runtime.validate_peer_block_filters().map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
    let chain = selected_canonical_chain(runtime).map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
    let activation = runtime.sync.is_enabled() || runtime.inbound.enabled || runtime.relay.enabled || runtime.block_serving.block_serving.enabled || runtime.block_serving.compact_relay.enabled;
    if activation { chain.spec().require_ordinary_history().map_err(|_| compatibility_refusal(runtime, chain))?; }
    Ok(chain)`;
  if (normalizeRust(stripRustNonCode(daemon.slice(opened.start, opened.end))) !== normalizeRust(expectedOpener)
    || normalizeRust(preflight.body) !== normalizeRust(expectedPreflight)) fail();
  const historyBody = normalizeRust(history.body);
  reviewedBody(history.body, "95f185b2202b3ce6eae06180125377107b8bf114a6afcb9beea53eae6dcd2ad9",
    "P123 bounded Regtest history factory", failures);
  for (const anchor of [
    "let chain = preflight(runtime)?;",
    "chain != CanonicalChain::Regtest || !trigger { return Ok(None); }",
    "ManagedRpcContext::from_canonical_history_runtime_config(runtime)",
    "assemble_durable_runtime(runtime, sync_runtime)?",
    "authoritative.maybe_prebuilt_context = Some(context);",
    "Ok(Some(OpenedAuthoritativeRuntime::Durable(authoritative)))",
  ]) if (!historyBody.includes(normalizeRust(anchor))) fail();
  rustGate(history.body, "if chain != CanonicalChain::Regtest || !trigger { return Ok(None); }", 0,
    "P123 daemon bounded Regtest activation gate", failures);
  if (rustCallArguments(history.body, "ManagedRpcContext::from_canonical_history_runtime_config")
    .map(args => args.map(normalizeRust).join("|")) .join("|") !== "runtime"
    || /open_ordinary_configured|from_fixture_runtime_config/.test(historyBody)) fail();
}

export function verifyTypedReceiveAndTcp(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const types = texts.get("packages/open-bitcoin-node/src/sync/types.rs") ?? "";
  for (const needle of [
    "pub enum SyncPeerReceiveOutcome",
    "Message(WireNetworkMessage)",
    "Idle",
    "Closed",
    "Result<SyncPeerReceiveOutcome, SyncRuntimeError>",
  ]) {
    requireContains(types, needle, "P123 typed receive outcomes", failures);
  }
  requireContains(
    texts.get("packages/open-bitcoin-node/src/lib.rs") ?? "",
    "SyncPeerReceiveOutcome",
    "P123 public receive re-export",
    failures,
  );

  for (const file of [
    "packages/open-bitcoin-node/src/sync/tcp.rs",
    "packages/open-bitcoin-bench/src/runtime_fixtures.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs",
  ] as const) {
    const text = texts.get(file) ?? "";
    requireContains(
      text,
      "Result<SyncPeerReceiveOutcome, SyncRuntimeError>",
      `P123 first-party receive migration ${file}`,
      failures,
    );
    requireAbsent(
      text,
      "Result<Option<WireNetworkMessage>, SyncRuntimeError>",
      `P123 first-party receive migration ${file}`,
      failures,
    );
  }

  const tcp = texts.get("packages/open-bitcoin-node/src/sync/tcp.rs") ?? "";
  for (const needle of [
    "Ok(0) if allow_clean_idle && filled == 0",
    "return Ok(ReadStageOutcome::Closed)",
    "allow_clean_idle",
    "filled == 0",
    "io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut",
    "return Ok(ReadStageOutcome::Idle)",
    "unexpected EOF after {filled} of {} frame bytes",
    "payload read ended without a complete frame",
  ]) {
    requireContains(tcp, needle, "P123 byte-progress TCP framing", failures);
  }
}

export function verifyIdleMaintenance(texts: Map<TargetFile, string>, failures: string[]): void {
  const session = texts.get("packages/open-bitcoin-node/src/sync/session.rs") ?? "";
  const requests = texts.get("packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs") ?? "";
  const reconcile = texts.get("packages/open-bitcoin-node/src/sync/block_reconcile.rs") ?? "";
  const terminal = texts.get("packages/open-bitcoin-node/src/sync/session/emission_terminal.rs") ?? "";
  verifySessionHandoff(session, requests, failures);
  verifyRequestAccounting(session, requests, reconcile, failures);
  verifySessionTransport(session, terminal, texts, failures);
}

function productionBody(source: string, symbol: string, label: string, failures: string[]): string {
  const maybeOwner = maybeFunction(source, symbol, true);
  const methodOwners = new Map([
    ["sync_connected_peer_with_cancel", "DurableSyncRuntime"], ["run_peer_messages", "DurableSyncRuntime"],
    ["process_peer_message", "DurableSyncRuntime"], ["block_extends_active_tip", "DurableSyncRuntime"],
    ["expire", "PeerRequests"], ["handoff", "PeerRequests"], ["reconcile", "PeerRequests"],
    ["dispatch", "PeerRequests"], ["settle_authorized", "PeerRequests"], ["record_ignored", "PeerRequests"],
    ["finish", "PeerRequests"], ["pending", "PeerRequests"],
  ]);
  if (!maybeOwner || /\bcfg\s*\(/.test(maybeOwner.attributes)
    || !directRustOwner(source, maybeOwner.start, methodOwners.get(symbol) ?? null)) {
    failures.push(`${label}: unique enabled production owner required`);
  }
  return maybeOwner?.body ?? "";
}

function directRustOwner(source: string, offset: number, maybeImpl: string | null): boolean {
  const code = rustCode(source);
  const depth = (text: string) => [...text].reduce((value, char) => value + Number(char === "{") - Number(char === "}"), 0);
  if (maybeImpl === null) return depth(code.slice(0, offset)) === 0;
  const matches = [...code.matchAll(new RegExp(`\\bimpl\\s+${maybeImpl}\\s*\\{`, "g"))];
  if (matches.length !== 1 || depth(code.slice(0, matches[0].index)) !== 0) return false;
  const before = code.slice(Math.max(code.lastIndexOf("}", matches[0].index - 1), code.lastIndexOf(";", matches[0].index - 1)) + 1, matches[0].index);
  if (/#\s*\[\s*cfg/.test(before)) return false;
  const open = code.indexOf("{", matches[0].index);
  let level = 1;
  for (let index = open + 1; index < code.length; index++) {
    if (index === offset) return level === 1;
    level += Number(code[index] === "{") - Number(code[index] === "}");
    if (level === 0) return false;
  }
  return false;
}

function rustOrder(body: string, anchors: readonly string[], label: string, failures: string[]): void {
  requireOrdered(normalizeRust(body), anchors.map(normalizeRust), label, failures);
}

function rustGate(body: string, expected: string, depth: number, label: string, failures: string[]): void {
  const code = normalizeRust(body);
  const gate = normalizeRust(expected);
  const index = code.indexOf(gate);
  const prefix = code.slice(0, index < 0 ? 0 : index);
  const actualDepth = [...prefix].reduce((value, char) => value + Number(char === "{") - Number(char === "}"), 0);
  if (index < 0 || code.indexOf(gate, index + gate.length) >= 0 || actualDepth !== depth) {
    failures.push(`${label}: exact reachable branch required`);
  }
}

function reviewedBody(body: string, sha256: string, label: string, failures: string[]): void {
  if (createHash("sha256").update(normalizeRust(body)).digest("hex") !== sha256) {
    failures.push(`${label}: reviewed production body changed`);
  }
}

function verifySessionHandoff(session: string, requests: string, failures: string[]): void {
  const connected = productionBody(session, "sync_connected_peer_with_cancel", "P123 session registration", failures);
  const run = productionBody(session, "run_peer_messages", "P123 idle clock target send order", failures);
  const process = productionBody(session, "process_peer_message", "P123 post-manager message wake", failures);
  reviewedBody(run, "c1d7fe1522d29dfef4ab69e5a45ec78ae579f3da27b843866c3202d139d3038e",
    "P123 reviewed session message loop", failures);
  reviewedBody(process, "b4699e6c8cd85d8a0d42aab843a76866126253c8baa593cfca701169ee7db97a",
    "P123 reviewed post-manager message flow", failures);
  requireAbsent(normalizeRust(process), "block_reconcile::release_inflight_for_message",
    "P123 raw pre-manager release", failures);
  rustOrder(connected, ["register_peer(peer_id)", "connect_sync_peer(peer_id, timestamp)?", "PeerRequests::new(token)",
    "self.run_peer_messages(", "finish_registration(", "requests.finish(self)", "request_cleanup_result("],
  "P123 session registration", failures);
  rustOrder(run, ["outbound.extend(turn.requests.request_missing(self)?);", "turn.requests.renew(self, &outbound)?",
    "self.send_all_for_registered_peer_with_cancel(", "SyncPeerReceiveOutcome::Message(message) =>",
    "let timestamp = (controls.0)();", "messages_received = messages_received.saturating_add(1);",
    "self.process_peer_message(session, turn, message, timestamp, controls.1)?",
    "SyncPeerReceiveOutcome::Idle =>", "let timestamp = (controls.0)();", "turn.requests.expire(self, timestamp)?",
    "turn.requests.renew(self, &outbound)?", "self.send_all_for_registered_peer_with_cancel(",
    "turn.requests.pending(self)", "SyncPeerReceiveOutcome::Closed =>"],
  "P123 idle clock target send order", failures);
  rustOrder(run, ["SyncPeerReceiveOutcome::Message(message) =>", "let timestamp = (controls.0)();",
    "messages_received = messages_received.saturating_add(1);", "SyncPeerReceiveOutcome::Idle =>"],
  "P123 message receive clock", failures);
  const messageBranch = run.split("SyncPeerReceiveOutcome::Message(message) =>")[1]?.split("SyncPeerReceiveOutcome::Idle =>")[0] ?? "";
  if (!normalizeRust(messageBranch).includes("lettimestamp=(controls.0)();messages_received=messages_received.saturating_add(1);")) {
    failures.push("P123 message receive clock missing or out of order lettimestamp=(controls.0)();");
  }
  const runSends = rustCallArguments(run, "self.send_all_for_registered_peer_with_cancel");
  const runArgs = "session|turn.peer_id|&outbound|controls.1|turn.registration|turn.requests";
  if (runSends.length !== 2 || runSends.some(args => args.map(normalizeRust).join("|") !== runArgs)) {
    failures.push("P123 idle clock target send order must forward current peer, packets and cancellation");
  }
  rustOrder(process, ["requests.renew(self, &[])?", "inflight_receipt_hashes(&message, &self.inflight_blocks)",
    "if let Some(block) = maybe_block.as_ref()", "!block_response_was_requested", "requests.record_ignored(",
    "self.record_unrequested_block_response(", "requests.request_missing(self)?", "requests.dispatch(self, message, current_timestamp, &receipt_hashes, progress)?",
    "let mut outbound = requests.handoff(", "requests.renew(self, &outbound)?", "filter_serving::send_filter_plans(",
    "block_reconcile::reconcile_best_chain_for_live_session(self, current_timestamp)?",
    "self.record_reconcile_progress(reconcile_progress);", "self.persist_progress_and_dispatch_tip()?;",
    "outbound.extend(requests.request_missing(self)?);", "requests.renew(self, &outbound)?",
    "self.send_all_for_registered_peer_with_cancel("],
  "P123 post-manager message wake", failures);
  const processed = normalizeRust(process);
  const handoffStart = processed.indexOf("letmutoutbound=requests.handoff(");
  const finalSend = processed.lastIndexOf("self.send_all_for_registered_peer_with_cancel(");
  const handoffSpan = handoffStart < 0 || finalSend <= handoffStart ? "" : processed.slice(handoffStart, finalSend);
  const outboundMethods = [...handoffSpan.matchAll(/(?<![.\w])outbound\.([a-z_]+)\(/g)].map(match => match[1]);
  if (outboundMethods.join("|") !== "extend" || handoffSpan.split("letmutoutbound=").length !== 2
    || handoffSpan.includes("&mutoutbound") || handoffSpan.split("returnOk(false)").length !== 3
    || handoffSpan.includes("returnErr(")) {
    failures.push("P123 manager handoff packets must remain intact until registered send");
  }
  const messageSends = rustCallArguments(process, "self.send_all_for_registered_peer_with_cancel");
  const messageArgs = "session|peer_id|&outbound|should_cancel|registration|requests";
  if (messageSends.length !== 2 || messageSends.some(args => args.map(normalizeRust).join("|") !== messageArgs)) {
    failures.push("P123 post-manager message wake must forward current peer, packets and cancellation");
  }
  for (const anchor of ["self.block_extends_active_tip(block)?", "progress.record_validated_headers(header_count)",
    "progress.record_block_notfound()", "self.record_block_disposition("]) {
    requireContains(normalizeRust(process), normalizeRust(anchor), "P123 session timestamp propagation", failures);
  }
  requireAbsent(session, "MAX_CONSECUTIVE_IDLE_WAKES_PER_SESSION", "P123 no fixed idle-wake cutoff", failures);
  const expire = productionBody(requests, "expire", "P123 intact targeted timeout handoff", failures);
  const handoff = productionBody(requests, "handoff", "P123 intact targeted timeout handoff", failures);
  rustOrder(expire, ["owner.expire(timestamp)", "handoff_core(", "targeted,"], "P123 intact targeted timeout handoff", failures);
  const exactExpire = "self.require_active()?;letnetwork=runtime.network.clone();network.with_sync_peer_requests(&self.token,|mutowner|{lettargeted=owner.expire(timestamp).map_err(ManagedNetworkAuthorityError::from)?;handoff_core(&mutowner,&mutruntime.inflight_blocks,runtime.config.max_blocks_in_flight_total,targeted,)})?";
  if (normalizeRust(expire) !== exactExpire) failures.push("P123 timeout packets must flow unchanged into handoff");
  rustOrder(handoff, ["handoff_core(", "targeted,"], "P123 message target handoff", failures);
}

function verifyRequestAccounting(session: string, requests: string, reconcile: string, failures: string[]): void {
  const core = productionBody(requests, "handoff_core", "P123 whole-batch request accounting", failures);
  reviewedBody(core, "12026d6e72e6526ba792a7d99c68c14f1981a8575a16cce1bcd9038538ca154a",
    "P123 whole-batch request accounting", failures);
  const maybeHandoffCore = maybeFunction(requests, "handoff_core", true);
  if (!maybeHandoffCore || createHash("sha256").update(requests.slice(maybeHandoffCore.start, maybeHandoffCore.end)).digest("hex")
    !== "71031dfe07933ae137571212fc08ad6ad5f7d044162ff8194cef9936766a7f69") {
    failures.push("P123 whole-batch request error literals changed");
  }
  rustOrder(core, ["if targeted.is_empty()", "targeted.iter().any(|(target, _)| *target != owner.peer())",
    "let requested: BTreeSet<_> = owner.requested()?.into_iter().collect();", "let mut hashes = BTreeSet::new();",
    "for (_, message) in &targeted", "WireNetworkMessage::GetData(inventory)",
    "InventoryType::Block | InventoryType::WitnessBlock", "BlockHash::from(item.object_hash)",
    "if !requested.contains(&hash)", "hashes.insert(hash);",
    "hashes.difference(inflight).count() > cap.saturating_sub(inflight.len())",
    "SyncRuntimeError::ResourceLimit", "inflight.extend(hashes);",
    "targeted.into_iter().map(|(_, message)| message).collect()"],
  "P123 whole-batch request accounting", failures);
  rustGate(core, "if targeted.iter().any(|(target, _)| *target != owner.peer()) { return Err(SyncRuntimeError::Network { message: .to_owned(), }); }", 0,
    "P123 whole-batch request target", failures);
  rustGate(core, "if !requested.contains(&hash) { return Err(SyncRuntimeError::Network { message: .to_owned(), }); }", 4,
    "P123 whole-batch request membership", failures);
  rustGate(core, "if hashes.difference(inflight).count() > cap.saturating_sub(inflight.len()) { return Err(SyncRuntimeError::ResourceLimit { message: .to_owned(), }); }", 0,
    "P123 whole-batch request capacity", failures);
  const coreCode = normalizeRust(core);
  const capacity = coreCode.indexOf("ifhashes.difference(inflight).count()>cap.saturating_sub(inflight.len())");
  const requested = coreCode.indexOf("letrequested:BTreeSet<_>=owner.requested()?.into_iter().collect();");
  const exactPreflight = "iftargeted.is_empty(){returnOk(Vec::new());}iftargeted.iter().any(|(target,_)|*target!=owner.peer()){returnErr(SyncRuntimeError::Network{message:.to_owned(),});}";
  if (requested < 0 || coreCode.slice(0, requested) !== exactPreflight
    || coreCode.slice(0, capacity < 0 ? 0 : capacity).includes("SyncRuntimeError::ResourceLimit")) {
    failures.push("P123 already-accounted resend must remain available at capacity");
  }
  const requestCore = productionBody(requests, "request_core", "P123 ordinary request accounting", failures);
  reviewedBody(requestCore, "f7c43399e4efdb8da65dc4a8ae607dae9e8341a60700c6877a9b2e78c610fcc8",
    "P123 reviewed ordinary request core", failures);
  reviewedBody(productionBody(requests, "handoff", "P123 message target handoff", failures),
    "88ac66f3bf68795d0fed3f13fd01dec66382da53756092c1d8367894b04f0fd1",
    "P123 reviewed message handoff", failures);
  rustOrder(requestCore, [".filter(|hash| !inflight.contains(hash))", ".take(cap.saturating_sub(inflight.len()))",
    "owner.request(&requested)?", "handoff_core(", "outbound.into_iter().map(|message| (peer, message)).collect()"],
  "P123 ordinary request accounting", failures);
  const release = productionBody(reconcile, "release_unrequested_hashes", "P123 all-peer post-manager release", failures);
  reviewedBody(release, "851048a29c9cfeb35acc5d988e1fb7d99e34d255ff1c9f8902c54ec3cf7e5553",
    "P123 all-peer post-manager release", failures);
  rustOrder(release, ["manager.peer_ids()", ".filter_map(|peer| manager.peer_state(peer))",
    ".flat_map(|peer| peer.requested_blocks.iter().copied())", "for hash in hashes",
    "if !retained.contains(hash)", "runtime.inflight_blocks.remove(hash)"],
  "P123 all-peer post-manager release", failures);
  const allPeers = "letretained:BTreeSet<_>=manager.peer_ids().into_iter().filter_map(|peer|manager.peer_state(peer)).flat_map(|peer|peer.requested_blocks.iter().copied()).collect();";
  if (!normalizeRust(release).includes(allPeers)) failures.push("P123 all-peer retained membership must be complete");
  rustGate(release, "if !retained.contains(hash) { runtime.inflight_blocks.remove(hash); }", 1,
    "P123 all-peer post-manager release", failures);
  if (maybeFunction(reconcile, "release_inflight_for_message", true)) {
    failures.push("P123 raw pre-manager release must remain test-only");
  }
  const dispatch = productionBody(requests, "dispatch", "P123 manager receive ordering", failures);
  const settle = productionBody(requests, "settle_authorized", "P123 post-manager receipt reconciliation", failures);
  const reconcileRequests = productionBody(requests, "reconcile", "P123 retained request reconciliation", failures);
  const ignored = productionBody(requests, "record_ignored", "P123 unrequested Block release", failures);
  const finish = productionBody(requests, "finish", "P123 disconnect request release", failures);
  rustOrder(dispatch, ["begin_sync_peer_dispatch(", "continuation.resolve()", "self.settle_dispatch(runtime, outcome, receipt)"],
  "P123 manager receive ordering", failures);
  rustOrder(dispatch, ["continuation.resolve()", "progress.record_activity(timestamp)",
    "self.settle_dispatch(runtime, outcome, receipt)"], "P123 manager receive ordering", failures);
  rustOrder(settle, ["authorized.into_parts()", "self.reconcile(runtime, observed)",
    "release_unrequested_hashes(runtime, receipt)", "request_cleanup_result(", "result.map_err(Into::into)"],
  "P123 post-manager receipt reconciliation", failures);
  const expectedSettle = "let(observed,result)=authorized.into_parts();letobservation_cleanup=self.reconcile(runtime,observed).map(|_|());letreceipt_cleanup=release_unrequested_hashes(runtime,receipt);request_cleanup_result(result.map_err(Into::into),request_cleanup_result(observation_cleanup,receipt_cleanup),)";
  if (normalizeRust(settle) !== expectedSettle) failures.push("P123 post-manager Ok/Err receipt cleanup must propagate both results");
  const expectedReconcile = "letprevious=release_unrequested_hashes(runtime,&self.retained);letSyncPeerObservation::Present{requests,peer,compact_pending,}=observedelse{previous?;self.maybe_peer=None;self.compact_pending=false;returnOk(false);};letcurrent=release_unrequested_hashes(runtime,&requests);request_cleanup_result(previous,current)?;self.retained=requests;self.maybe_peer=Some(*peer);self.compact_pending=compact_pending;Ok(true)";
  if (normalizeRust(reconcileRequests) !== expectedReconcile) failures.push("P123 retained and current request release must propagate through absence and presence");
  rustOrder(ignored, ["observe_sync_peer(&self.token)?", "progress.record_activity(timestamp)",
    "self.reconcile(runtime, observed)", "release_unrequested_hashes(runtime, receipt)"],
  "P123 unrequested Block release", failures);
  const expectedIgnored = "self.require_active()?;letobserved=runtime.network.observe_sync_peer(&self.token)?;runtime.network.with_sync_peer_requests(&self.token,|_owner|{progress.record_activity(timestamp);})?;letobservation_cleanup=self.reconcile(runtime,observed).map(|_|());request_cleanup_result(observation_cleanup,release_unrequested_hashes(runtime,receipt),)";
  if (normalizeRust(ignored) !== expectedIgnored) failures.push("P123 unrequested Block cleanup must propagate both results");
  rustOrder(finish, ["finish_sync_peer(&self.token)?", "self.reconcile(runtime, observed)"],
  "P123 disconnect request release", failures);
  const expectedFinish = "ifletSome(denied)=self.maybe_denied_requests.as_ref(){runtime.network.require_absent_sync_peer(&self.token)?;letretained_cleanup=release_unrequested_hashes(runtime,&self.retained);letdenied_cleanup=release_unrequested_hashes(runtime,denied);request_cleanup_result(retained_cleanup,denied_cleanup)?;self.retained.clear();self.maybe_denied_requests=None;returnOk(());}letobserved=runtime.network.finish_sync_peer(&self.token)?;self.reconcile(runtime,observed).map(|_|())";
  if (normalizeRust(finish) !== expectedFinish) failures.push("P123 disconnect cleanup must preserve retained and denied request ownership");
  const pending = productionBody(requests, "pending", "P123 pending response retention", failures);
  rustOrder(pending, ["self.compact_pending", "self.retained.iter()", "runtime.inflight_blocks.contains(hash)"],
  "P123 pending response retention", failures);
  const cleanup = productionBody(session, "request_cleanup_result", "P123 dual-error honesty", failures);
  for (const anchor of ["match (result, cleanup)", "(result, Ok(())) => result",
    "(Ok(_), Err(error)) => Err(error)", "(Err(error), Err(cleanup)) => Err(SyncRuntimeError::Network",
    "format!(", "error", "cleanup"]) {
    requireContains(normalizeRust(cleanup), normalizeRust(anchor), "P123 dual-error honesty", failures);
  }
  const expectedCleanup = "match(result,cleanup){(result,Ok(()))=>result,(Ok(_),Err(error))=>Err(error),(Err(error),Err(cleanup))=>Err(SyncRuntimeError::Network{message:format!(),}),}";
  if (normalizeRust(cleanup) !== expectedCleanup) failures.push("P123 primary and cleanup failures must remain visible together");
}

function verifySessionTransport(session: string, terminal: string, texts: Map<TargetFile, string>, failures: string[]): void {
  const emissions = productionBody(terminal, "send_peer_emissions", "P123 sync peer emission post-write completion", failures);
  reviewedBody(emissions, "03b5017153909d5703c9b241999c9f6e0d60e098c6d89868daeaa9bfbc9217bb",
    "P123 reviewed peer emission writer", failures);
  rustOrder(emissions, ["let preparation = emissions.iter().try_for_each", ".encode_pending_network_message(peer_id, emission.message())",
    "buffers.push_back(buffer);", "while let Some(emission) = emissions.pop_front()",
    "if let Err(write_error) = session.send_encoded(buffer.bytes(), network_magic)",
    "return combined_error(", "capability.acknowledge_write()", "network.complete_peer_emission(receipt)"],
  "P123 sync peer emission post-write completion", failures);
  rustGate(emissions, "if let Err(write_error) = session.send_encoded(buffer.bytes(), network_magic) { drop(buffer); drop(buffers); return combined_error(write_error, abort_current_and_suffix(network, capability, emissions), ); }", 1,
    "P123 peer emission write before completion", failures);
  productionBody(texts.get("packages/open-bitcoin-node/src/sync/block_response.rs") ?? "", "block_extends_active_tip",
    "P123 compact fallback active-tip classification", failures);
  requireContains(normalizeRust(session), normalizeRust("self.block_extends_active_tip(block)?"),
    "P123 compact fallback active-tip response classification", failures);
  requireContains(texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "",
    "pub fn sync_until_idle_with_clock_and_cancel", "P123 cancellation-aware sync API", failures);
  rustOrder(texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "",
    ["let mut shutdown_latched = false;", "let mut should_cancel = ||", "daemon_sync_shutdown_requested(&shutdown_receiver)",
      "sync_until_idle_with_clock_and_cancel(", "if shutdown_latched"],
    "P123 daemon live-session cancellation", failures);
}
