import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase126CompactRelayResidualHardening } from "./check-phase126-compact-relay-residual-hardening";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const TARGET_FILES = [
  "packages/open-bitcoin-network/src/peer/message_dispatch.rs",
  "packages/open-bitcoin-network/src/peer/compact_download_state.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs",
  "packages/open-bitcoin-node/Cargo.toml",
  "packages/open-bitcoin-node/BUILD.bazel",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-phase126-compact-relay-residual-hardening.ts",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type FixtureOptions = {
  maybeMutate?: (files: Map<TargetFile, string>) => void;
};

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes_when_compact_relay_residuals_are_hardened", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_generic_dispatch_synthesizes_empty_receive_facts", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-network/src/peer/message_dispatch.rs",
        "Err(NetworkError::CompactBlockReceiveFactsRequired)",
        "self.handle_compact_block_download(peer_id, payload, Default::default(), timestamp)",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 factless compact dispatch: generic dispatch must return CompactBlockReceiveFactsRequired");
});

test("fails_when_compact_receive_facts_implements_default", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-network/src/peer/compact_download_state.rs",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 receive facts default: CompactBlockReceiveFacts must not implement Default");
});

test("fails_when_a_managed_receive_path_bypasses_live_snapshots", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replaceFirst(
        files,
        "packages/open-bitcoin-node/src/network.rs",
        "self.handle_compact_block_receive(peer_id, payload, timestamp)?",
        "self.peer_manager.handle_message(peer_id, WireNetworkMessage::CompactBlock(payload), timestamp)?",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 managed receive paths: receive and sync entrypoints must inject live facts");
});

test("fails_when_live_receive_facts_drop_the_mempool_snapshot", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs",
        "candidates: &candidate_refs,",
        "candidates: &[],",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 live receive snapshots: compact facts must carry mempool and bounded extras");
});

test("fails_when_compact_nonce_entropy_is_not_lazy_and_fallible", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-node/src/network.rs",
        "getrandom::fill(&mut nonce_bytes)?;",
        "nonce_bytes.fill(0);",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 lazy fallible nonce: compact selection must acquire getrandom entropy in the node shell");
});

test("fails_when_compact_nonce_is_derived_from_block_hash_bytes", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-node/src/network.rs",
        "getrandom::fill(&mut nonce_bytes)?;",
        "nonce_bytes.copy_from_slice(&block_hash(&block.header).to_byte_array()[..8]);",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 nonce provenance: compact nonce must not derive from block-hash bytes");
});

test("fails_when_entropy_failure_can_emit_a_compact_block", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-node/src/network.rs",
        "Err(_) => self.peer_manager.announce_block(peer_id, block)?,",
        "Err(_) => self.peer_manager.announce_block_with_action(peer_id, block, announcement.action, 0)?,",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 entropy failure emission: entropy failure must fall back without cmpctblock");
});

test("fails_when_compact_provenance_is_recorded_without_actual_emission", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-node/src/network.rs",
        "if matches!(maybe_message, Some(WireNetworkMessage::CompactBlock(_))) {",
        "if announcement.action == CompactAnnouncementAction::AnnounceCompactBlock {",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 compact provenance: provenance must require an emitted cmpctblock");
});

test("fails_when_achieved_effect_evidence_ignores_the_emitted_message", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-node/src/network.rs",
        "maybe_message.as_ref(),",
        "None,",
      );
    },
  });

  // Act
  const failures = checkPhase126CompactRelayResidualHardening(root);

  // Assert
  expect(failures).toContain("P126 achieved effect evidence: evidence must derive from the emitted message");
});

test("fails_when_cargo_and_bazel_entropy_dependencies_diverge", () => {
  // Arrange
  const cargoRoot = createFixture({
    maybeMutate(files) {
      replace(files, "packages/open-bitcoin-node/Cargo.toml", 'getrandom = "0.3.4"', "");
    },
  });
  const bazelRoot = createFixture({
    maybeMutate(files) {
      replace(files, "packages/open-bitcoin-node/BUILD.bazel", '"@crate_index//:getrandom",', "");
    },
  });

  // Act
  const cargoFailures = checkPhase126CompactRelayResidualHardening(cargoRoot);
  const bazelFailures = checkPhase126CompactRelayResidualHardening(bazelRoot);

  // Assert
  expect(cargoFailures).toContain("P126 Cargo entropy dependency: open-bitcoin-node must declare getrandom");
  expect(bazelFailures).toContain("P126 Bazel entropy dependency: open-bitcoin-node must depend on getrandom");
});

test("fails_when_required_parity_or_breadcrumb_anchors_are_missing", () => {
  // Arrange
  const parityRoot = createFixture({
    maybeMutate(files) {
      replaceFirst(
        files,
        "docs/parity/index.json",
        '"packages/bitcoin-knots/src/blockencodings.h",',
        "",
      );
    },
  });
  const breadcrumbRoot = createFixture({
    maybeMutate(files) {
      replaceFirst(
        files,
        "docs/parity/source-breadcrumbs.json",
        '"packages/bitcoin-knots/src/blockencodings.h",',
        "",
      );
    },
  });

  // Act
  const parityFailures = checkPhase126CompactRelayResidualHardening(parityRoot);
  const breadcrumbFailures = checkPhase126CompactRelayResidualHardening(breadcrumbRoot);

  // Assert
  expect(parityFailures).toContain("P126 parity anchors: compact relay surfaces must retain exact Knots anchors");
  expect(breadcrumbFailures).toContain("P126 breadcrumb anchors: compact download group must retain exact Knots anchors");
});

test("fails_when_verifier_wiring_is_out_of_order_or_nonlocal", () => {
  // Arrange
  const orderRoot = createFixture({
    maybeMutate(files) {
      replaceFirst(
        files,
        "scripts/verify.sh",
        "bun test scripts/check-phase126-compact-relay-residual-hardening.test.ts",
        "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts",
      );
    },
  });
  const networkRoot = createFixture({
    maybeMutate(files) {
      append(
        files,
        "scripts/check-phase126-compact-relay-residual-hardening.ts",
        'fetch("https://example.invalid");',
      );
    },
  });

  // Act
  const orderFailures = checkPhase126CompactRelayResidualHardening(orderRoot);
  const networkFailures = checkPhase126CompactRelayResidualHardening(networkRoot);

  // Assert
  expect(orderFailures).toContain("P126 verifier visible order: Phase 126 must follow active traceability and precede Phase 117");
  expect(networkFailures).toContain("P126 deterministic scope: checker must remain local and deterministic");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase126-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>();
  for (const file of TARGET_FILES) {
    files.set(file, readFileSync(path.join(REPO_ROOT, file), "utf8"));
  }
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, text);
  }
  return root;
}

function replace(
  files: Map<TargetFile, string>,
  file: TargetFile,
  needle: string,
  replacement: string,
): void {
  const text = files.get(file) ?? "";
  if (!text.includes(needle)) throw new Error(`fixture needle missing in ${file}: ${needle}`);
  files.set(file, text.replaceAll(needle, replacement));
}

function replaceFirst(
  files: Map<TargetFile, string>,
  file: TargetFile,
  needle: string,
  replacement: string,
): void {
  const text = files.get(file) ?? "";
  if (!text.includes(needle)) throw new Error(`fixture needle missing in ${file}: ${needle}`);
  files.set(file, text.replace(needle, replacement));
}

function append(files: Map<TargetFile, string>, file: TargetFile, value: string): void {
  files.set(file, `${files.get(file) ?? ""}\n${value}\n`);
}
