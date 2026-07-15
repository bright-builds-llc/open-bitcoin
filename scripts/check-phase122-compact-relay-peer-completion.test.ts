import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase122CompactRelayPeerCompletion } from "./check-phase122-compact-relay-peer-completion";

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
type Mutator = (files: Map<TargetFile, string>) => void;
const tempRoots: string[] = [];
const fallback = "old-block full-witness-block fallback is intentionally omitted";

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with a complete Phase 122 compact relay corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase122CompactRelayPeerCompletion({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [
    "bounded provenance",
    "P122 bounded peer provenance missing MAX_COMPACT_ANNOUNCEMENT_PROVENANCE: usize = 11",
    mutate("packages/open-bitcoin-network/src/peer/compact_relay.rs", "MAX_COMPACT_ANNOUNCEMENT_PROVENANCE: usize = 11", "MAX_COMPACT_ANNOUNCEMENT_PROVENANCE: usize = 12"),
  ],
  [
    "post-construction recording",
    "P122 post-construction announcement record has .record_compact_block_announcement(peer_id, block_hash) out of order",
    (files: Map<TargetFile, string>) => {
      files.set(
        "packages/open-bitcoin-node/src/network.rs",
        ".record_compact_block_announcement(peer_id, block_hash) announce_block_with_action matches!(maybe_message, Some(WireNetworkMessage::CompactBlock(_)))",
      );
    },
  ],
  [
    "typed live response",
    "P122 live action translation missing WireNetworkMessage::BlockTxn(response)",
    mutate("packages/open-bitcoin-node/src/network/action_translation.rs", "WireNetworkMessage::BlockTxn(response)", "WireNetworkMessage::NotFound(response)"),
  ],
  [
    "witness-preserving order test",
    "P122 witness/order test missing transactions: vec![expected_first, expected_second]",
    mutate("packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs", "transactions: vec![expected_first, expected_second]", "transactions: vec![expected_second, expected_first]"),
  ],
  [
    "malformed disconnect test",
    "P122 out-of-bounds disconnect test missing expect_err(\"out-of-bounds getblocktxn must disconnect\")",
    mutate("packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs", "expect_err(\"out-of-bounds getblocktxn must disconnect\")", "expect(\"out-of-bounds request\")"),
  ],
  [
    "benign suppression test",
    "P122 benign suppression test missing phase122_compact_getblocktxn_is_silent_when_serving_becomes_ineligible",
    mutate("packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs", "phase122_compact_getblocktxn_is_silent_when_serving_becomes_ineligible", "phase122_compact_getblocktxn_disconnects_when_serving_becomes_ineligible"),
  ],
  [
    "peer-session cleanup test",
    "P122 peer-session cleanup test missing phase122_disconnect_drops_compact_announcement_provenance_for_reconnected_peer",
    mutate("packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs", "phase122_disconnect_drops_compact_announcement_provenance_for_reconnected_peer", "phase122_disconnect_preserves_compact_announcement_provenance"),
  ],
  [
    "stale Phase 112 no-op assertion",
    "P122 stale Phase 112 no-op assertion must not contain phase112_bip152_wire_messages_are_peer_noops",
    (files: Map<TargetFile, string>) => {
      const file = "packages/open-bitcoin-network/src/peer/tests.rs";
      files.set(file, `${files.get(file)}\nfn phase112_bip152_wire_messages_are_peer_noops() {}`);
    },
  ],
  [
    "top-level parity surface",
    "P122 parity index must contain exactly one surface: v2-1-compact-relay-peer-completion",
    mutate(
      "docs/parity/index.json",
      "v2-1-compact-relay-peer-completion",
      "v2-1-compact-relay-peer-partial",
    ),
  ],
  [
    "HARD-01 parity ownership",
    "P122 parity checklist surface missing HARD-01",
    mutate("docs/parity/index.json", "HARD-01", "HARD-02"),
  ],
  [
    "scoped fallback deviation",
    `P122 parity checklist known gap missing ${fallback}`,
    mutate("docs/parity/index.json", fallback, "old-block fallback is supported"),
  ],
  [
    "pinned Knots handler anchor",
    "P122 pinned Knots anchors missing test_getblocktxn_handler",
    mutate("docs/parity/catalog/p2p.md", "test_getblocktxn_handler", "test_compactblocks"),
  ],
  [
    "default verifier wiring",
    "P122 verifier wiring missing bun test scripts/check-phase122-compact-relay-peer-completion.test.ts",
    mutate("scripts/verify.sh", "bun test scripts/check-phase122-compact-relay-peer-completion.test.ts", ""),
  ],
] as const)("fails the %s mutation", (_label, expectedFailure, maybeMutate) => {
  // Arrange
  const root = createFixture(maybeMutate as Mutator);

  // Act
  const failures = checkPhase122CompactRelayPeerCompletion({ rootDir: root });

  // Assert
  expect(failures).toContain(expectedFailure);
});

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase122-"));
  tempRoots.push(root);
  const files = completeFiles();
  maybeMutate?.(files);
  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }
  return root;
}

function completeFiles(): Map<TargetFile, string> {
  const parity = `HARD-01 ${fallback}`;
  const parityIndex = JSON.stringify({
    surfaces: [{ name: "v2-1-compact-relay-peer-completion", status: "done" }],
    checklist: {
      surfaces: [
        {
          id: "v2-1-compact-relay-peer-completion",
          status: "done",
          requirements: ["HARD-01"],
          known_gaps: [fallback],
        },
      ],
    },
  });
  return new Map<TargetFile, string>([
    [
      "packages/open-bitcoin-network/src/peer/compact_relay.rs",
      "MAX_COMPACT_ANNOUNCEMENT_PROVENANCE: usize = 11 VecDeque<BlockHash> BTreeSet<BlockHash>",
    ],
    [
      "packages/open-bitcoin-network/src/peer.rs",
      "ServeCompactBlockTransactions(CompactBlockTransactionsRequest)",
    ],
    [
      "packages/open-bitcoin-network/src/peer/message_dispatch.rs",
      "self.handle_get_block_transactions(peer_id, request)",
    ],
    [
      "packages/open-bitcoin-network/src/peer/tests.rs",
      "phase122_compact_announcement_provenance_is_idempotent_and_bounded phase122_compact_overflowing_getblocktxn_disconnects_and_peer_cleanup_drops_provenance",
    ],
    [
      "packages/open-bitcoin-node/src/network.rs",
      "announce_block_with_action matches!(maybe_message, Some(WireNetworkMessage::CompactBlock(_))) .record_compact_block_announcement(peer_id, block_hash)",
    ],
    [
      "packages/open-bitcoin-node/src/network/action_translation.rs",
      "PeerAction::ServeCompactBlockTransactions(request) WireNetworkMessage::BlockTxn(response)",
    ],
    [
      "packages/open-bitcoin-node/src/network/block_serving.rs",
      "serve_managed_compact_block_transactions",
    ],
    [
      "packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs",
      "phase122_compact_announcement_then_getblocktxn_serves_ordered_witness_transactions ScriptWitness::new transactions: vec![expected_first, expected_second] phase122_compact_getblocktxn_is_silent_for_other_peer_or_unavailable_block phase122_compact_getblocktxn_is_silent_when_serving_becomes_ineligible outbound.is_empty()",
    ],
    [
      "packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs",
      "phase122_disconnect_drops_compact_announcement_provenance_for_reconnected_peer",
    ],
    [
      "packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs",
      'phase122_live_compact_getblocktxn_out_of_bounds_index_disconnects expect_err("out-of-bounds getblocktxn must disconnect")',
    ],
    ["docs/parity/index.json", parityIndex],
    [
      "docs/parity/catalog/p2p.md",
      `${parity} packages/bitcoin-knots/src/net_processing.cpp packages/bitcoin-knots/src/blockencodings.h packages/bitcoin-knots/test/functional/p2p_compactblocks.py test_getblocktxn_handler`,
    ],
    ["docs/parity/checklist.md", parity],
    [
      "scripts/verify.sh",
      `bun run scripts/check-phase121-block-relay-metrics-log-runtime.ts
bun test scripts/check-phase122-compact-relay-peer-completion.test.ts
bun run scripts/check-phase122-compact-relay-peer-completion.ts
run_step "test Phase 122 compact relay peer completion checker"
run_step "check Phase 122 compact relay peer completion"`,
    ],
  ]);
}

function mutate(file: TargetFile, from: string, to: string): Mutator {
  return (files) => files.set(file, (files.get(file) ?? "").replace(from, to));
}
