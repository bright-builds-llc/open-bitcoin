#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE127_CHECK =
  "bun run scripts/check-phase127-authoritative-network-state-unification.ts";
const PHASE128_TEST =
  "bun test scripts/check-phase128-production-compact-announcement-transport.test.ts";
const PHASE128_CHECK =
  "bun run scripts/check-phase128-production-compact-announcement-transport.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const ARCHIVED_V21_ROADMAP = ".planning/milestones/v2.1-ROADMAP.md";

export const PHASE128_TARGET_FILES = [
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-network/src/peer/message_dispatch.rs",
  "packages/open-bitcoin-network/src/peer/compact_relay.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/announcement_transport.rs",
  "packages/open-bitcoin-node/src/network/runtime_authority.rs",
  "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
  "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
  "packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs",
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-node/src/sync/block_response.rs",
  "packages/open-bitcoin-node/src/sync/block_reconcile.rs",
  "packages/open-bitcoin-node/src/sync/session.rs",
  "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
  "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
  ".planning/ROADMAP.md",
  ".planning/PROJECT.md",
  "scripts/check-phase128-production-compact-announcement-transport.ts",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof PHASE128_TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;

export function checkPhase128ProductionCompactAnnouncementTransport(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ??
      process.env.OPEN_BITCOIN_PHASE128_REPO_ROOT ??
      DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = loadCorpus(repoRoot, failures);
  checkBilateralNegotiation(texts, failures);
  checkPostDurableTrigger(texts, failures);
  checkLiveFactsAndBoundedEmissions(texts, failures);
  checkProductionWriteBoundaries(texts, failures);
  checkPostWriteEvidence(texts, failures);
  checkObservabilityAndProductionProof(texts, failures);
  checkBoundedScope(texts, failures);
  checkVerifier(texts, failures);
  return failures;
}

function loadCorpus(repoRoot: string, failures: string[]): TextCorpus {
  const texts = new Map<TargetFile, string>();
  for (const file of PHASE128_TARGET_FILES) {
    const sourceFile =
      file === ".planning/ROADMAP.md" &&
      existsSync(path.join(repoRoot, ARCHIVED_V21_ROADMAP))
        ? ARCHIVED_V21_ROADMAP
        : file;
    const absolutePath = path.join(repoRoot, sourceFile);
    if (!existsSync(absolutePath)) {
      failures.push(`P128 missing target: ${file}`);
      texts.set(file, "");
      continue;
    }
    texts.set(file, readFileSync(absolutePath, "utf8"));
  }
  return texts;
}

function checkBilateralNegotiation(
  texts: TextCorpus,
  failures: string[],
): void {
  const relay =
    texts.get("packages/open-bitcoin-network/src/peer/compact_relay.rs") ?? "";
  if (
    !orderedFragments(relay, [
      "*state = LocalCompactRelayOfferState::Scheduled",
      "version: BIP152_COMPACT_BLOCKS_VERSION,",
      "Some(SendCompactMessage {",
      "announce: false,",
      "version: BIP152_COMPACT_BLOCKS_VERSION,",
    ])
  ) {
    failures.push(
      "P128 local offer: production handshake must schedule sendcmpct(false, version 2)",
    );
  }

  const dispatch =
    texts.get("packages/open-bitcoin-network/src/peer/message_dispatch.rs") ??
    "";
  const verack = section(dispatch, "fn handle_verack(", "\n    fn handle_headers(");
  if (
    !orderedFragments(verack, [
      "peer.remote_verack_received = true;",
      "self.maybe_schedule_local_compact_offer(peer_id)?",
      "actions.push(PeerAction::Send(WireNetworkMessage::SendCompact(message)));",
    ])
  ) {
    failures.push(
      "P128 post-Verack dispatch: established handshake must enqueue the local compact offer",
    );
  }

  const peer = texts.get("packages/open-bitcoin-network/src/peer.rs") ?? "";
  const directionalAnchors = [
    "pub local_compact_relay_offer: LocalCompactRelayOfferState,",
    "pub compact_relay: CompactRelayPeerState,",
    "peer.maybe_remote_protocol_version,",
    "peer.local_compact_relay_offer,",
    "self.high_bandwidth_preference = CompactRelayPreference::Requested;",
    "self.low_bandwidth_preference = CompactRelayPreference::Requested;",
  ];
  if (
    !directionalAnchors
      .slice(0, 4)
      .every((anchor) => peer.includes(anchor)) ||
    !directionalAnchors.slice(4).every((anchor) => relay.includes(anchor))
  ) {
    failures.push(
      "P128 directional negotiation: remote sendcmpct must retain high and low preference",
    );
  }
}

function checkPostDurableTrigger(
  texts: TextCorpus,
  failures: string[],
): void {
  const response =
    texts.get("packages/open-bitcoin-node/src/sync/block_response.rs") ?? "";
  if (
    !orderedFragments(response, [
      "self.store.save_block(block, self.config.persist_mode)?;",
      ".note_local_block_hash(block_hash(&block.header))?;",
      "if self.block_is_current_best_tip(block)? {",
      "self.queue_durable_tip_advanced(block.clone());",
    ])
  ) {
    failures.push(
      "P128 durable trigger: accepted best-tip blocks must queue only after durable save",
    );
  }

  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  const persistAndDispatch = section(
    response,
    "pub(super) fn persist_progress_and_dispatch_tip(",
    "\n    }",
  );
  if (
    !sync.includes("pub struct DurableTipAdvanced") ||
    !orderedFragments(sync, [
      "let outboxes = announcement_outboxes_for_sink.snapshots()?;",
      "announcement_network.prepare_block_announcements(event.block(), &outboxes)?;",
      "announcement_outboxes_for_sink.enqueue_prepared(outcomes)",
    ]) ||
    !orderedFragments(persistAndDispatch, [
      "self.persist_progress()",
      "self.dispatch_pending_durable_tip()",
    ])
  ) {
    failures.push(
      "P128 durable dispatch: persistence must precede tip announcement dispatch",
    );
  }

  const reconcile =
    texts.get("packages/open-bitcoin-node/src/sync/block_reconcile.rs") ?? "";
  if (
    !orderedFragments(reconcile, [
      "let mut maybe_final_connected_block = None;",
      "maybe_final_connected_block = Some(block);",
      "if queue_durable_tip && let Some(block) = maybe_final_connected_block",
      "runtime.queue_durable_tip_advanced(block);",
    ])
  ) {
    failures.push(
      "P128 reconciliation trigger: a live multi-block extension must announce only its final durable tip",
    );
  }
}

function checkLiveFactsAndBoundedEmissions(
  texts: TextCorpus,
  failures: string[],
): void {
  const transport =
    texts.get(
      "packages/open-bitcoin-node/src/network/announcement_transport.rs",
    ) ?? "";
  const liveFactAnchors = [
    "let peer_has_previous_header = peer",
    ".contains(&block.header.previous_block_hash);",
    "let peer_has_current_header = peer.compact_announcements.contains(&block_hash);",
    "peer_has_previous_header,",
    "peer_has_current_header,",
  ];
  const hasConstantFacts = [
    "peer_has_previous_header: true",
    "peer_has_previous_header: false",
    "peer_has_current_header: true",
    "peer_has_current_header: false",
  ].some((anchor) => transport.includes(anchor));
  if (
    !liveFactAnchors.every((anchor) => transport.includes(anchor)) ||
    hasConstantFacts
  ) {
    failures.push(
      "P128 live peer facts: announcement policy must derive both header facts per peer",
    );
  }

  const emission = section(
    transport,
    "pub struct PeerEmission {",
    "\n}\n\nimpl PeerEmission",
  );
  const receipt = section(
    transport,
    "pub struct PeerEmissionReceipt {",
    "\n}\n\nimpl PeerEmissionReceipt",
  );
  const capability = section(
    transport,
    "pub struct PeerEmissionWriteCapability {",
    "\n}\n\nimpl PeerEmissionWriteCapability",
  );
  if (
    !transport.includes(
      "#[derive(Debug, PartialEq, Eq)]\npub struct PeerEmission {",
    ) ||
    ![
      "peer_id: PeerId,",
      "message: WireNetworkMessage,",
      "capability: PeerEmissionWriteCapability,",
    ].every((anchor) => emission.includes(anchor)) ||
    ![
      "effect_capability: PeerEffectCapability,",
      "block_hash: BlockHash,",
      "evidence_reason: CompactAnnouncementReason,",
      "write_kind: PeerEmissionWriteKind,",
    ].every((anchor) => capability.includes(anchor)) ||
    !transport.includes(
      "pub fn into_parts(self) -> (PeerId, WireNetworkMessage, PeerEmissionWriteCapability)",
    ) ||
    !transport.includes("pub fn acknowledge_write(self) -> PeerEmissionReceipt") ||
    receipt.includes("Clone")
  ) {
    failures.push(
      "P128 owned emission: PeerEmission must bind message, peer, block, and a consuming receipt",
    );
  }

  const session =
    texts.get("packages/open-bitcoin-node/src/sync/session.rs") ?? "";
  if (
    !transport.includes("PHASE94_MAX_PEER_QUEUED_MESSAGES") ||
    !transport.includes("if outbox.is_full() {") ||
    !session.includes("BTreeMap<PeerId, AnnouncementOutbox>") ||
    !session.includes("emissions: VecDeque<PeerEmission>") ||
    !session.includes(
      "if outbox.emissions.len() >= PHASE94_MAX_PEER_QUEUED_MESSAGES",
    )
  ) {
    failures.push(
      "P128 bounded transport: preparation and session outboxes must enforce queue limits",
    );
  }
}

function checkProductionWriteBoundaries(
  texts: TextCorpus,
  failures: string[],
): void {
  const session =
    texts.get("packages/open-bitcoin-node/src/sync/session.rs") ?? "";
  const outbound = section(
    session,
    "pub(super) fn send_all_for_peer",
    "\n    pub(super) fn peer_handshake_complete",
  );
  if (
    !orderedFragments(outbound, [
      "let (target_peer_id, message, capability) = emission.into_parts();",
      "session.send(&message, self.config.network.magic())?;",
      "capability.acknowledge_write()",
    ])
  ) {
    failures.push(
      "P128 outbound write boundary: receipt completion must follow the session send",
    );
  }

  const inbound =
    texts.get(
      "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
    ) ?? "";
  const drain = section(
    inbound,
    "async fn drain_inbound_announcements(",
    "\nasync fn acknowledge_inbound_response_write(",
  );
  const executor = section(
    inbound,
    "pub(super) async fn execute_inbound_emissions",
    "\n#[allow(clippy::too_many_arguments)]\nstruct SocketInboundEmissionExecutor",
  );
  const socketExecutor = section(
    inbound,
    "impl InboundEmissionExecutor for SocketInboundEmissionExecutor",
    "\n#[allow(clippy::too_many_arguments)]\nasync fn drain_inbound_announcements",
  );
  if (
    !drain.includes("execute_inbound_emissions(emissions, peer_id, &mut executor).await") ||
    !orderedFragments(executor, [
      "match executor.write(&bytes).await",
      "InboundEmissionWriteResult::Written => {",
      "capability.acknowledge_write()",
    ]) ||
    !orderedFragments(socketExecutor, [
      "fn complete(&mut self, receipt: PeerEmissionReceipt)",
      ".complete_peer_emission(receipt)",
    ])
  ) {
    failures.push(
      "P128 inbound write boundary: receipt completion must occur only after Written",
    );
  }

  const daemon =
    texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  if (
    !daemon.includes("announcement_outboxes: sync_runtime.announcement_outboxes(),") ||
    !daemon.includes("authoritative_runtime.announcement_outboxes,")
  ) {
    failures.push(
      "P128 production composition: outbound and inbound sessions must share authoritative outboxes",
    );
  }
}

function checkPostWriteEvidence(
  texts: TextCorpus,
  failures: string[],
): void {
  const evidence =
    texts.get(
      "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
    ) ?? "";
  const completion =
    texts.get(
      "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
    ) ?? "";
  if (
    !orderedFragments(completion, [
      "pub fn complete_peer_emission(",
      ".apply_lifecycle_command(LifecycleCommand::CompletePeerEmission(receipt))",
    ]) ||
    !completion.includes(
      "LifecycleCommandResult::PeerEffectCompleted(completion) => Ok(completion),",
    ) ||
    completion.includes("try_mutate") ||
    !orderedFragments(evidence, [
      "fn record_peer_emission(",
      "evidence.records_header_provenance()",
      ".record_compact_block_announcement(peer_id, evidence.block_hash())?;",
      ".record_announcement(evidence.evidence_reason());",
    ])
  ) {
    failures.push(
      "P128 receipt evidence: consuming completion must bind provenance and fixed achieved outcome",
    );
  }

  const network = texts.get("packages/open-bitcoin-node/src/network.rs") ?? "";
  const preparation =
    texts.get(
      "packages/open-bitcoin-node/src/network/announcement_transport.rs",
    ) ?? "";
  if (
    network.includes(".record_compact_block_announcement(") ||
    preparation.includes(".record_compact_block_announcement(") ||
    preparation.includes(".record_announcement(")
  ) {
    failures.push(
      "P128 post-write evidence: preparation paths must not mutate achieved announcement evidence",
    );
  }
}

function checkObservabilityAndProductionProof(
  texts: TextCorpus,
  failures: string[],
): void {
  const productionTests =
    texts.get(
      "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
    ) ?? "";
  if (
    !productionTests.includes(
      "production_announcement_transport_cases_fanout_uses_live_peer_facts",
    ) ||
    !productionTests.includes(
      "production_announcement_transport_cases_partial_failure_credits_only_prefix_and_redacts",
    )
  ) {
    failures.push(
      "P128 production proof: tests must cover live-fact fanout and successful-prefix failure semantics",
    );
  }
  if (
    !orderedFragments(productionTests, [
      "let status = runtime",
      ".block_relay_evidence_status()",
      "let metrics = block_relay_metric_samples(&status, 0, TRANSPORT_TIMESTAMP as u64);",
      "let log = block_relay_log_record(&status, 0, TRANSPORT_TIMESTAMP as u64);",
    ]) ||
    !productionTests.includes('"compact_announced_count"') ||
    !productionTests.includes('"compact_headers_fallback_count"') ||
    !productionTests.includes('"compact_inventory_fallback_count"')
  ) {
    failures.push(
      "P128 observability: end-to-end tests must project fixed metrics and logs from post-write status",
    );
  }
}

function checkBoundedScope(texts: TextCorpus, failures: string[]): void {
  const roadmap = texts.get(".planning/ROADMAP.md") ?? "";
  const project = texts.get(".planning/PROJECT.md") ?? "";
  const requiredRoadmapClaims = [
    "**Requirements:** CMP-04, CMP-05, OBS-03",
    "live peer header facts",
    "written through the real peer transport",
    "only after a successful transport write",
  ];
  const requiredProjectClaims = [
    "v2.1 does not imply public relay defaults, production service operation, production-funds wallet use, public-network CI, or production full-node readiness.",
    "package relay, bloom/filter serving, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use deferred",
  ];
  const archivedRoadmapClaims = [
    "v2.1 Block Serving and Compact Block Relay Boundary shipped",
    "[v2.1-ROADMAP.md](milestones/v2.1-ROADMAP.md)",
    "[v2.1-REQUIREMENTS.md](milestones/v2.1-REQUIREMENTS.md)",
  ];
  if (
    !(
      requiredRoadmapClaims.every((claim) => roadmap.includes(claim)) ||
      archivedRoadmapClaims.every((claim) => roadmap.includes(claim))
    ) ||
    !requiredProjectClaims.every((claim) => project.includes(claim))
  ) {
    failures.push(
      "P128 bounded scope: package, filter, public-default, public-network, and production claims must stay deferred",
    );
  }
}

function checkVerifier(texts: TextCorpus, failures: string[]): void {
  const verify = texts.get("scripts/verify.sh") ?? "";
  const visible = visibleCommandOrder(verify);
  const requiredVisible = [
    PHASE127_CHECK,
    PHASE128_TEST,
    PHASE128_CHECK,
    PHASE117_TEST,
  ];
  const requiredSteps = [
    `run_step "check Phase 127 authoritative network state unification" ${PHASE127_CHECK}`,
    `run_step "test Phase 128 production compact announcement transport checker" ${PHASE128_TEST}`,
    `run_step "check Phase 128 production compact announcement transport" ${PHASE128_CHECK}`,
    `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`,
  ];
  if (
    !orderedLines(visible, requiredVisible) ||
    !orderedLines(verify, requiredSteps)
  ) {
    failures.push(
      "P128 verifier wiring: mutation test and production checker must run before the final Phase 117 gate",
    );
  }

  const checker =
    texts.get(
      "scripts/check-phase128-production-compact-announcement-transport.ts",
    ) ?? "";
  const forbiddenTokens = [
    "fetch" + "(",
    "Bun." + "spawn",
    "node:" + "child_process",
    "http" + "://",
    "https" + "://",
  ];
  if (forbiddenTokens.some((token) => checker.includes(token))) {
    failures.push(
      "P128 deterministic scope: checker must remain local and public-network-free",
    );
  }
}

function section(text: string, startMarker: string, endMarker: string): string {
  const start = text.indexOf(startMarker);
  if (start === -1) return "";
  const end = text.indexOf(endMarker, start + startMarker.length);
  return text.slice(start, end === -1 ? text.length : end);
}

function visibleCommandOrder(text: string): string {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = text.indexOf(marker);
  if (start === -1) return "";
  const bodyStart = start + marker.length;
  const end = text.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  return end === -1 ? "" : text.slice(bodyStart, end);
}

function orderedFragments(text: string, fragments: readonly string[]): boolean {
  let cursor = -1;
  for (const fragment of fragments) {
    const index = text.indexOf(fragment, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

function orderedLines(text: string, requiredLines: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const required of requiredLines) {
    const index = lines.indexOf(required, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

if (import.meta.main) {
  const failures = checkPhase128ProductionCompactAnnouncementTransport();
  if (failures.length > 0) {
    console.error(
      "Phase 128 production compact announcement transport check failed:",
    );
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log(
    "Phase 128 production compact announcement transport validated.",
  );
}
