import { afterEach, expect, test } from "bun:test";
import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  PHASE128_TARGET_FILES,
  checkPhase128ProductionCompactAnnouncementTransport,
} from "./check-phase128-production-compact-announcement-transport";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const ARCHIVED_V21_ROADMAP = ".planning/milestones/v2.1-ROADMAP.md";
type TargetFile = (typeof PHASE128_TARGET_FILES)[number];
type Mutator = (files: Map<TargetFile, string>) => void;
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 128 production transport corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures =
    checkPhase128ProductionCompactAnnouncementTransport(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [
    "local low-bandwidth version-2 offer",
    "P128 local offer: production handshake must schedule sendcmpct(false, version 2)",
    replace(
      "packages/open-bitcoin-network/src/peer/compact_relay.rs",
      "announce: false,",
      "announce: true,",
    ),
  ],
  [
    "post-Verack offer dispatch",
    "P128 post-Verack dispatch: established handshake must enqueue the local compact offer",
    replace(
      "packages/open-bitcoin-network/src/peer/message_dispatch.rs",
      "self.maybe_schedule_local_compact_offer(peer_id)?",
      "None",
    ),
  ],
  [
    "directional remote high-bandwidth preference",
    "P128 directional negotiation: remote sendcmpct must retain high and low preference",
    replace(
      "packages/open-bitcoin-network/src/peer/compact_relay.rs",
      "self.high_bandwidth_preference = CompactRelayPreference::Requested;",
      "self.high_bandwidth_preference = CompactRelayPreference::NotRequested;",
    ),
  ],
  [
    "post-durable block trigger",
    "P128 durable trigger: accepted best-tip blocks must queue only after durable save",
    replace(
      "packages/open-bitcoin-node/src/sync/block_response.rs",
      "self.queue_durable_tip_advanced(block.clone());",
      "self.clear_pending_durable_tip();",
    ),
  ],
  [
    "persistence before dispatch",
    "P128 durable dispatch: persistence must precede tip announcement dispatch",
    replace(
      "packages/open-bitcoin-node/src/sync/block_response.rs",
      "self.dispatch_pending_durable_tip()",
      "Ok(())",
    ),
  ],
  [
    "live previous-header fact",
    "P128 live peer facts: announcement policy must derive both header facts per peer",
    replace(
      "packages/open-bitcoin-node/src/network/announcement_transport.rs",
      "peer_has_previous_header,",
      "peer_has_previous_header: false,",
    ),
  ],
  [
    "live current-header fact",
    "P128 live peer facts: announcement policy must derive both header facts per peer",
    replace(
      "packages/open-bitcoin-node/src/network/announcement_transport.rs",
      "peer_has_current_header,",
      "peer_has_current_header: false,",
    ),
  ],
  [
    "owned non-clone emission",
    "P128 owned emission: PeerEmission must bind message, peer, block, and a consuming receipt",
    replace(
      "packages/open-bitcoin-node/src/network/announcement_transport.rs",
      "#[derive(Debug, PartialEq, Eq)]\npub struct PeerEmission {",
      "#[derive(Debug, Clone, PartialEq, Eq)]\npub struct PeerEmission {",
    ),
  ],
  [
    "bounded peer outbox",
    "P128 bounded transport: preparation and session outboxes must enforce queue limits",
    replace(
      "packages/open-bitcoin-node/src/network/announcement_transport.rs",
      "if outbox.is_full() {",
      "if false {",
    ),
  ],
  [
    "bounded session outbox",
    "P128 bounded transport: preparation and session outboxes must enforce queue limits",
    replace(
      "packages/open-bitcoin-node/src/sync/session.rs",
      "if outbox.emissions.len() >= PHASE94_MAX_PEER_QUEUED_MESSAGES",
      "if false",
    ),
  ],
  [
    "outbound post-write receipt",
    "P128 outbound write boundary: receipt completion must follow the session send",
    replace(
      "packages/open-bitcoin-node/src/sync/session/emission_terminal.rs",
      "capability.acknowledge_write()",
      "drop(capability)",
    ),
  ],
  [
    "inbound post-write receipt",
    "P128 inbound write boundary: receipt completion must occur only after Written",
    replace(
      "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
      "capability.acknowledge_write()",
      "false",
    ),
  ],
  [
    "no preparation-time achieved-effect credit",
    "P128 post-write evidence: preparation paths must not mutate achieved announcement evidence",
    append(
      "packages/open-bitcoin-node/src/network/announcement_transport.rs",
      "self.block_relay_evidence.record_announcement(reason);",
    ),
  ],
  [
    "atomic post-write achieved-effect credit",
    "P128 receipt evidence: consuming completion must bind provenance and fixed achieved outcome",
    replace(
      "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
      ".apply_lifecycle_command(LifecycleCommand::CompletePeerEmission(receipt))",
      ".apply_lifecycle_command(LifecycleCommand::CompletePeerEffect(receipt))",
    ),
  ],
  [
    "authoritative fixed metric and log projection",
    "P128 observability: end-to-end tests must project fixed metrics and logs from post-write status",
    replace(
      "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
      "let metrics = block_relay_metric_samples(&status, 0, TRANSPORT_TIMESTAMP as u64);",
      "let metrics = Vec::new();",
    ),
  ],
  [
    "production end-to-end fanout",
    "P128 production proof: tests must cover live-fact fanout and successful-prefix failure semantics",
    replace(
      "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
      "production_announcement_transport_cases_fanout_uses_live_peer_facts",
      "helper_only_fanout_uses_constants",
    ),
  ],
  [
    "bounded no-claim scope",
    "P128 bounded scope: package, filter, public-default, public-network, and production claims must stay deferred",
    replace(
      ".planning/PROJECT.md",
      "v2.1 does not imply public relay defaults, production service operation, production-funds wallet use, public-network CI, or production full-node readiness.",
      "v2.1 enables production public relay defaults.",
    ),
  ],
  [
    "default verifier wiring",
    "P128 verifier wiring: mutation test and production checker must run before the final Phase 117 gate",
    replace(
      "scripts/verify.sh",
      "bun test scripts/check-phase128-production-compact-announcement-transport.test.ts",
      "",
    ),
  ],
  [
    "local deterministic checker",
    "P128 deterministic scope: checker must remain local and public-network-free",
    append(
      "scripts/check-phase128-production-compact-announcement-transport.ts",
      'fetch("https://example.invalid");',
    ),
  ],
] as const)(
  "fails the %s mutation",
  (_label, expectedFailure, maybeMutate) => {
    // Arrange
    const root = createFixture(maybeMutate as Mutator);

    // Act
    const failures =
      checkPhase128ProductionCompactAnnouncementTransport(root);

    // Assert
    expect(failures).toContain(expectedFailure);
  },
);

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase128-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>();
  for (const file of PHASE128_TARGET_FILES) {
    files.set(file, readFileSync(path.join(REPO_ROOT, file), "utf8"));
  }
  maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, text);
  }
  const archivedRoadmapPath = path.join(root, ARCHIVED_V21_ROADMAP);
  mkdirSync(path.dirname(archivedRoadmapPath), { recursive: true });
  writeFileSync(
    archivedRoadmapPath,
    readFileSync(path.join(REPO_ROOT, ARCHIVED_V21_ROADMAP), "utf8"),
  );
  return root;
}

function replace(
  file: TargetFile,
  needle: string,
  replacement: string,
): Mutator {
  return (files) => {
    const text = files.get(file) ?? "";
    if (!text.includes(needle)) {
      throw new Error(`fixture needle missing in ${file}: ${needle}`);
    }
    files.set(file, text.replace(needle, replacement));
  };
}

function append(file: TargetFile, value: string): Mutator {
  return (files) => files.set(file, `${files.get(file) ?? ""}\n${value}\n`);
}
