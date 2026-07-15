#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE121_CHECK =
  "bun run scripts/check-phase121-block-relay-metrics-log-runtime.ts";
const PHASE122_TEST =
  "bun test scripts/check-phase122-compact-relay-peer-completion.test.ts";
const PHASE122_CHECK =
  "bun run scripts/check-phase122-compact-relay-peer-completion.ts";
const FALLBACK_DEVIATION =
  "old-block full-witness-block fallback is intentionally omitted";
const SURFACE_ID = "v2-1-compact-relay-peer-completion";

const TARGET_FILES = [
  "packages/open-bitcoin-network/src/peer/compact_relay.rs",
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-network/src/peer/message_dispatch.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/action_translation.rs",
  "packages/open-bitcoin-node/src/network/block_serving.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs",
  "docs/parity/index.json",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type CheckOptions = { rootDir?: string };
type ParityIndex = {
  surfaces?: unknown[];
  checklist?: { surfaces?: unknown[] };
};
type NamedSurface = { name?: string; status?: string };
type ChecklistSurface = {
  id?: string;
  status?: string;
  requirements?: string[];
  known_gaps?: string[];
};

export function checkPhase122CompactRelayPeerCompletion(
  options: CheckOptions = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyBoundedProvenance(texts, failures);
  verifyPostConstructionRecording(texts, failures);
  verifyRequestPressurePrecedesProvenance(texts, failures);
  verifyTypedLiveResponse(texts, failures);
  verifyWitnessAndOrderTests(texts, failures);
  verifyMalformedDisconnect(texts, failures);
  verifyBenignSuppressionAndCleanup(texts, failures);
  verifyPhase112NoOpRemoved(texts, failures);
  verifyParityEvidence(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  return failures;
}

function verifyRequestPressurePrecedesProvenance(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const dispatch =
    texts.get("packages/open-bitcoin-network/src/peer/message_dispatch.rs") ?? "";
  const tests = texts.get("packages/open-bitcoin-network/src/peer/tests.rs") ?? "";
  requireOrdered(
    dispatch,
    [
      "request.index_deltas.len()",
      "resource_limit_disconnect_actions(pressure)",
      "if !peer.compact_announcements.contains(&request.block_hash)",
    ],
    "P122 request pressure before provenance suppression",
    failures,
  );
  requireContains(
    tests,
    "phase122_unannounced_getblocktxn_over_request_cap_disconnects_before_suppression",
    "P122 unannounced request pressure test",
    failures,
  );
}

function readText(repoRoot: string, file: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, file);
  if (!existsSync(absolutePath)) {
    failures.push(`P122 missing required corpus file: ${file}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}

function verifyBoundedProvenance(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const state = texts.get("packages/open-bitcoin-network/src/peer/compact_relay.rs") ?? "";
  const tests = texts.get("packages/open-bitcoin-network/src/peer/tests.rs") ?? "";
  for (const needle of [
    "MAX_COMPACT_ANNOUNCEMENT_PROVENANCE: usize = 11",
    "VecDeque<BlockHash>",
    "BTreeSet<BlockHash>",
  ]) {
    requireContains(state, needle, "P122 bounded peer provenance", failures);
  }
  requireContains(
    tests,
    "phase122_compact_announcement_provenance_is_idempotent_and_bounded",
    "P122 provenance bound test",
    failures,
  );
}

function verifyPostConstructionRecording(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const network = texts.get("packages/open-bitcoin-node/src/network.rs") ?? "";
  requireOrdered(
    network,
    [
      "announce_block_with_action",
      "matches!(maybe_message, Some(WireNetworkMessage::CompactBlock(_)))",
      ".record_compact_block_announcement(peer_id, block_hash)",
    ],
    "P122 post-construction announcement record",
    failures,
  );
}

function verifyTypedLiveResponse(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const peer = texts.get("packages/open-bitcoin-network/src/peer.rs") ?? "";
  const dispatch =
    texts.get("packages/open-bitcoin-network/src/peer/message_dispatch.rs") ?? "";
  const translation =
    texts.get("packages/open-bitcoin-node/src/network/action_translation.rs") ?? "";
  const serving =
    texts.get("packages/open-bitcoin-node/src/network/block_serving.rs") ?? "";
  requireContains(
    peer,
    "ServeCompactBlockTransactions(CompactBlockTransactionsRequest)",
    "P122 typed peer action",
    failures,
  );
  requireContains(
    dispatch,
    "self.handle_get_block_transactions(peer_id, request)",
    "P122 getblocktxn dispatch",
    failures,
  );
  for (const needle of [
    "PeerAction::ServeCompactBlockTransactions(request)",
    "WireNetworkMessage::BlockTxn(response)",
  ]) {
    requireContains(translation, needle, "P122 live action translation", failures);
  }
  requireContains(
    serving,
    "serve_managed_compact_block_transactions",
    "P122 managed compact serving",
    failures,
  );
}

function verifyWitnessAndOrderTests(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const tests =
    texts.get("packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs") ?? "";
  for (const needle of [
    "phase122_compact_announcement_then_getblocktxn_serves_ordered_witness_transactions",
    "ScriptWitness::new",
    "transactions: vec![expected_first, expected_second]",
  ]) {
    requireContains(tests, needle, "P122 witness/order test", failures);
  }
}

function verifyMalformedDisconnect(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const peerTests = texts.get("packages/open-bitcoin-network/src/peer/tests.rs") ?? "";
  const liveTests =
    texts.get("packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs") ??
    "";
  requireContains(
    peerTests,
    "phase122_compact_overflowing_getblocktxn_disconnects_and_peer_cleanup_drops_provenance",
    "P122 overflowing-index disconnect test",
    failures,
  );
  for (const needle of [
    "phase122_live_compact_getblocktxn_out_of_bounds_index_disconnects",
    "expect_err(\"out-of-bounds getblocktxn must disconnect\")",
  ]) {
    requireContains(liveTests, needle, "P122 out-of-bounds disconnect test", failures);
  }
}

function verifyBenignSuppressionAndCleanup(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const relayTests =
    texts.get("packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs") ?? "";
  const cleanupTests =
    texts.get("packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs") ?? "";
  for (const needle of [
    "phase122_compact_getblocktxn_is_silent_for_other_peer_or_unavailable_block",
    "phase122_compact_getblocktxn_is_silent_when_serving_becomes_ineligible",
    "outbound.is_empty()",
  ]) {
    requireContains(relayTests, needle, "P122 benign suppression test", failures);
  }
  requireContains(
    cleanupTests,
    "phase122_disconnect_drops_compact_announcement_provenance_for_reconnected_peer",
    "P122 peer-session cleanup test",
    failures,
  );
}

function verifyPhase112NoOpRemoved(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const tests = texts.get("packages/open-bitcoin-network/src/peer/tests.rs") ?? "";
  requireAbsent(
    tests,
    "phase112_bip152_wire_messages_are_peer_noops",
    "P122 stale Phase 112 no-op assertion",
    failures,
  );
}

function verifyParityEvidence(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const index = texts.get("docs/parity/index.json") ?? "";
  const catalog = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklist = texts.get("docs/parity/checklist.md") ?? "";
  verifyParityIndex(index, failures);
  for (const [text, label] of [
    [catalog, "P122 P2P parity evidence"],
    [checklist, "P122 checklist parity evidence"],
  ] as const) {
    requireContains(text, "HARD-01", label, failures);
    requireContains(text, FALLBACK_DEVIATION, label, failures);
  }
  for (const needle of [
    "packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/src/blockencodings.h",
    "packages/bitcoin-knots/test/functional/p2p_compactblocks.py",
    "test_getblocktxn_handler",
  ]) {
    requireContains(catalog, needle, "P122 pinned Knots anchors", failures);
  }
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`P122 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  const namedMatches = (parsed.surfaces ?? []).filter(
    (entry) => (entry as NamedSurface).name === SURFACE_ID,
  ) as NamedSurface[];
  if (namedMatches.length !== 1) {
    failures.push(`P122 parity index must contain exactly one surface: ${SURFACE_ID}`);
  } else if (namedMatches[0]?.status !== "done") {
    failures.push(`P122 parity index surface must be done: ${SURFACE_ID}`);
  }

  const checklistMatches = (parsed.checklist?.surfaces ?? []).filter(
    (entry) => (entry as ChecklistSurface).id === SURFACE_ID,
  ) as ChecklistSurface[];
  if (checklistMatches.length !== 1) {
    failures.push(`P122 parity checklist must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }
  const surface = checklistMatches[0];
  if (surface?.status !== "done") {
    failures.push(`P122 parity checklist surface must be done: ${SURFACE_ID}`);
  }
  if (!surface?.requirements?.includes("HARD-01")) {
    failures.push("P122 parity checklist surface missing HARD-01");
  }
  if (!surface?.known_gaps?.some((gap) => gap.includes(FALLBACK_DEVIATION))) {
    failures.push(`P122 parity checklist known gap missing ${FALLBACK_DEVIATION}`);
  }
}

function verifyVerifierWiring(verifyScript: string, failures: string[]): void {
  for (const needle of [
    PHASE122_TEST,
    PHASE122_CHECK,
    'run_step "test Phase 122 compact relay peer completion checker"',
    'run_step "check Phase 122 compact relay peer completion"',
  ]) {
    requireContains(verifyScript, needle, "P122 verifier wiring", failures);
  }
  requireOrdered(
    verifyScript,
    [PHASE121_CHECK, PHASE122_TEST, PHASE122_CHECK],
    "P122 verifier command order",
    failures,
  );
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) failures.push(`${label} missing ${needle}`);
}

function requireAbsent(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) failures.push(`${label} must not contain ${needle}`);
}

function requireOrdered(
  text: string,
  needles: readonly string[],
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle);
    if (index === -1) failures.push(`${label} missing ${needle}`);
    else if (index <= cursor) failures.push(`${label} has ${needle} out of order`);
    else cursor = index;
  }
}

if (import.meta.main) {
  const failures = checkPhase122CompactRelayPeerCompletion();
  if (failures.length > 0) {
    console.error("Phase 122 compact relay peer completion checker failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 122 compact relay peer completion checker passed.");
}
