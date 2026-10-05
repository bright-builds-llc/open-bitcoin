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
const BLOCK_RESPONSE = "packages/open-bitcoin-node/src/sync/block_response.rs";
const SAVE_ERROR = "if let Err(error) = self.store.save_block(block, self.config.persist_mode) {";
const SAVE_RETURN = "return Err(error.into());";
const DURABLE_TRIGGER_FAILURE =
  "P128 durable trigger: accepted best-tip blocks must queue only after durable save";
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

test("rejects test-only implementation ownership with an intervening attribute", () => {
  // Arrange
  expect(checkPhase128ProductionCompactAnnouncementTransport(createFixture())).toEqual([]);
  const root = createFixture(replace(
    BLOCK_RESPONSE,
    "impl DurableSyncRuntime {",
    "#[cfg(test)]\n#[allow(dead_code)]\nimpl DurableSyncRuntime {",
  ));

  // Act
  const failures = checkPhase128ProductionCompactAnnouncementTransport(root);

  // Assert
  expect(failures).toEqual([DURABLE_TRIGGER_FAILURE]);
});

test.each([
  ["test attribute last", "#[allow(dead_code)]\n#[cfg(test)]"],
  ["test attribute between two ordinary attributes", "#[allow(dead_code)]\n#[cfg(test)]\n#[rustfmt::skip]"],
  ["test attribute before multiple ordinary attributes", "#[cfg(test)]\n#[allow(dead_code)]\n#[rustfmt::skip]"],
  ["test attribute after multiple ordinary attributes", "#[rustfmt::skip]\n#[allow(dead_code)]\n#[cfg(test)]"],
  ["test attribute separated by comment decoys", "#[cfg(test)]\n/* ] ; } impl UnrelatedRuntime { */\n#[allow(dead_code)]"],
  ["test attribute before bracketed ordinary metadata", "#[cfg(test)]\n#[allow(dead_code, reason = \"[; }] #[cfg(not(test))]\")]"],
  ["test attribute with token spacing", "# [ cfg ( test ) ]\n#[allow(dead_code)]"],
] as const)("rejects the %s on the owning implementation", (_label, attributes) => {
  // Arrange
  expect(checkPhase128ProductionCompactAnnouncementTransport(createFixture())).toEqual([]);
  const root = createFixture(replace(
    BLOCK_RESPONSE,
    "impl DurableSyncRuntime {",
    `${attributes}\nimpl DurableSyncRuntime {`,
  ));

  // Act
  const failures = checkPhase128ProductionCompactAnnouncementTransport(root);

  // Assert
  expect(failures).toEqual([DURABLE_TRIGGER_FAILURE]);
});

test.each([
  ["ordinary lint attribute", "#[allow(dead_code)]"],
  ["multiple ordinary attributes", "#[rustfmt::skip]\n#[allow(dead_code)]"],
  ["comment-only test attribute", "/* #[cfg(test)] */\n#[allow(dead_code)]"],
  ["line-comment test attribute", "// #[cfg(test)]\n#[allow(dead_code)]"],
  ["literal-only test attribute", '#[doc = "#[cfg(test)] ] ; }"]\n#[allow(dead_code)]'],
  ["raw-literal-only test attribute", '#[doc = r#"#[cfg(test)] ] ; }"#]\n#[allow(dead_code)]'],
  ["non-test configuration", "#[cfg(not(test))]\n#[allow(dead_code)]"],
  ["test attribute on a preceding implementation", "#[cfg(test)]\n#[allow(dead_code)]\nimpl UnrelatedRuntime {}\n#[allow(dead_code)]"],
  ["test attribute on a preceding declaration", "#[cfg(test)]\nconst TEST_MARKER: usize = 0;\n#[allow(dead_code)]"],
] as const)("accepts the %s beside the owning implementation", (_label, attributes) => {
  // Arrange
  expect(checkPhase128ProductionCompactAnnouncementTransport(createFixture())).toEqual([]);
  const root = createFixture(replace(
    BLOCK_RESPONSE,
    "impl DurableSyncRuntime {",
    `${attributes}\nimpl DurableSyncRuntime {`,
  ));

  // Act
  const failures = checkPhase128ProductionCompactAnnouncementTransport(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  ["removed durable save", replace(BLOCK_RESPONSE, SAVE_ERROR, "if let Err(error) = Ok::<(), SyncRuntimeError>(()) {")],
  ["ignored save failure", replace(BLOCK_RESPONSE, SAVE_RETURN, "let _ = error;")],
  ["accepted progress before error return", replace(BLOCK_RESPONSE, SAVE_RETURN, `progress.record_accepted_block(); ${SAVE_RETURN}`)],
  ["block hash before error return", replace(BLOCK_RESPONSE, SAVE_RETURN, `self.network.note_local_block_hash(block_hash(&block.header))?; ${SAVE_RETURN}`)],
  ["queue before error return", replace(BLOCK_RESPONSE, SAVE_RETURN, `self.queue_durable_tip_advanced(block.clone()); ${SAVE_RETURN}`)],
  ["queue before durable save", replace(BLOCK_RESPONSE, SAVE_ERROR, `self.queue_durable_tip_advanced(block.clone()); ${SAVE_ERROR}`)],
  ["conditional durable save", compose(replace(BLOCK_RESPONSE, SAVE_ERROR, `if false { ${SAVE_ERROR}`), replace(BLOCK_RESPONSE, "self.network\n                        .note_local_block_hash", "} self.network\n                        .note_local_block_hash"))],
  ["comment-only durable save", replace(BLOCK_RESPONSE, SAVE_ERROR, `/* ${SAVE_ERROR} */ if let Err(error) = Ok::<(), SyncRuntimeError>(()) {`)],
  ["literal-only durable save", replace(BLOCK_RESPONSE, SAVE_ERROR, `let _ = r#"${SAVE_ERROR}"#; if let Err(error) = Ok::<(), SyncRuntimeError>(()) {`)],
  ["test-only block disposition", replace(BLOCK_RESPONSE, "pub(super) fn record_block_disposition(", "#[cfg(test)]\n    pub(super) fn record_block_disposition(")],
  ["test-module-only block disposition", (files: Map<TargetFile, string>) => files.set(BLOCK_RESPONSE, `#[cfg(test)] mod tests {\n${files.get(BLOCK_RESPONSE)}\n}`)],
  ["test-only runtime implementation", replace(BLOCK_RESPONSE, "impl DurableSyncRuntime {", "#[cfg(test)]\nimpl DurableSyncRuntime {")],
  ["unrelated implementation save", replace(BLOCK_RESPONSE, "impl DurableSyncRuntime {", "impl UnrelatedRuntime {")],
] as const)("rejects the %s independently", (_label, mutate) => {
  // Arrange
  const root = createFixture(mutate);

  // Act
  const failures = checkPhase128ProductionCompactAnnouncementTransport(root);

  // Assert
  expect(failures).toEqual([DURABLE_TRIGGER_FAILURE]);
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

function compose(...mutators: Mutator[]): Mutator {
  return (files) => {
    for (const mutate of mutators) mutate(files);
  };
}
