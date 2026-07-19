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

import { checkPhase127AuthoritativeNetworkStateUnification } from "./check-phase127-authoritative-network-state-unification";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const TARGET_FILES = [
  "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
  "packages/open-bitcoin-rpc/src/context.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/context/inbound_status.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-node/src/network/runtime_authority.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store.rs",
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-rpc/tests/black_box_parity.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "scripts/check-phase127-authoritative-network-state-unification.ts",
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

test("passes_when_phase127_authoritative_composition_is_intact", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_production_constructs_a_duplicate_network_authority", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
        "open_authoritative_network_runtime(&runtime, maybe_runtime_store.clone())?;",
        [
          "open_authoritative_network_runtime(&runtime, maybe_runtime_store.clone())?;",
          "    let _duplicate =",
          "        open_authoritative_network_runtime(&runtime, maybe_runtime_store.clone())?;",
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);

  // Assert
  expect(failures).toContain(
    "P127 production authority: daemon must compose sync, inbound, and RPC from one authoritative runtime",
  );
});

test("fails_when_production_substitutes_a_transient_network_but_retains_the_old_anchor", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
        "authoritative_runtime.network.clone(),",
        [
          "ManagedNetworkHandle::transient_runtime(/* duplicate authority */),",
          "        // authoritative_runtime.network.clone(),",
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);

  // Assert
  expect(failures).toContain(
    "P127 production authority: daemon must compose sync, inbound, and RPC from one authoritative runtime",
  );
});

test("fails_when_production_serving_uses_cache_while_retaining_the_durable_anchor", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-rpc/src/context.rs",
        "source.load_block(intent.block_hash())",
        [
          "self.network.lookup_block(intent.block_hash())",
          "            // source.load_block(intent.block_hash())",
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);

  // Assert
  expect(failures).toContain(
    "P127 durable serving: production block resolution must use the request-scoped durable source",
  );
});

test("fails_when_operator_projection_defaults_while_retaining_the_snapshot_anchor", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-rpc/src/context/inbound_status.rs",
        "let network = self.network.operator_snapshot()?;",
        [
          "let network = ManagedNetworkOperatorSnapshot::default();",
          "        // let network = self.network.operator_snapshot()?;",
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);

  // Assert
  expect(failures).toContain(
    "P127 authoritative projection: RPC and operator status must use one owned network snapshot",
  );
});

test("fails_when_network_status_defaults_block_relay_while_retaining_the_projection_anchor", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-rpc/src/dispatch/node.rs",
        "block_relay: snapshot.block_relay().clone(),",
        [
          "block_relay: Default::default(),",
          "        // block_relay: snapshot.block_relay().clone(),",
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);

  // Assert
  expect(failures).toContain(
    "P127 authoritative projection: RPC and operator status must use one owned network snapshot",
  );
});

test("fails_when_network_status_directly_reads_block_relay_while_retaining_the_projection_anchor", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "packages/open-bitcoin-rpc/src/dispatch/node.rs",
        "block_relay: snapshot.block_relay().clone(),",
        [
          "block_relay: context.network.block_relay_evidence_status()?,",
          "        // block_relay: snapshot.block_relay().clone(),",
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);

  // Assert
  expect(failures).toContain(
    "P127 authoritative projection: direct block-relay projection must not bypass the owned snapshot",
  );
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase127-"));
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
  if (!text.includes(needle)) {
    throw new Error(`fixture needle missing in ${file}: ${needle}`);
  }
  files.set(file, text.replace(needle, replacement));
}
