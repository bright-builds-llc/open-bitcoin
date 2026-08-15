import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  PHASE135_DIAGNOSTICS,
  PHASE135_TARGET_FILES,
  checkPhase135SnapshotRecovery,
} from "./check-phase135-snapshot-recovery";
import {
  body,
  directStatementIndex,
} from "./check-phase135-snapshot-recovery/source";
import {
  persistedInputMutations,
  type Phase135Mutation,
  type Phase135Mutator,
} from "./check-phase135-snapshot-recovery/persisted-input-mutations";
import { readSourceRoot } from "./source-corpus";
const REPO_ROOT = path.resolve(import.meta.dir, "..");
const tempRoots: string[] = [];
type Mutator = Phase135Mutator;
type Mutation = Phase135Mutation;

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 135 corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase135SnapshotRecovery(root);

  // Assert
  expect(failures).toEqual([]);
});

test("ignores braces and command tokens inside comments and string literals", () => {
  // Arrange
  const root = createFixture((files) => {
    insertAfter(
      "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs",
      "struct MempoolSnapshotV2RecordDto {",
      '\n    // } does not close the struct\n    const _: &str = "}";\n    const _: &str = r###"} {"###;',
    )(files);
    append(
      "scripts/verify.sh",
      '\n# bun test scripts/check-phase135-snapshot-recovery.test.ts\nignored="bun run scripts/check-phase135-snapshot-recovery.ts"\n',
    )(files);
  });

  // Act
  const failures = checkPhase135SnapshotRecovery(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  ["top level", "target();\n", 0],
  ["closure", "let decoy = || { target(); };\n", -1],
  ["local function", "fn decoy() { target(); }\n", -1],
  ["async block", "let decoy = async { target(); };\n", -1],
  ["nested block", "if guarded { target(); }\n", -1],
  ["unclosed block", "if guarded { target();\n", -1],
  ["unexpected closing block", "}\ntarget();\n", -1],
] as const)(
  "finds direct statements only at balanced top-level depth: %s",
  (_name, source, expected) =>
    expect(directStatementIndex(source, "target();")).toBe(expected),
);

test.each([
  ["preceding-line cfg(any())", "#[cfg(any())]\n    target();", -1],
  ["same-line cfg(any())", "    #[cfg(any())] target();", -1],
  ["blank line after cfg(any())", "#[cfg(any())]\n\n    target();", -1],
] as const)(
  "rejects attribute-disabled direct statements: %s",
  (_name, source, expected) =>
    expect(directStatementIndex(source, "target();")).toBe(expected),
);

test("masked comments, strings, and raw strings cannot spoof direct statements", () => {
  // Arrange
  const source = `fn guarded() {
// target(); }
let normal = "target(); }";
let raw = r###"target(); } {"###;
/* target(); { */
target();
}`;
  const maskedBody = body(source, "fn guarded()");
  // Act
  const actual = directStatementIndex(maskedBody, "target();");
  // Assert
  expect(actual).toBe(maskedBody.lastIndexOf("target();"));
});

test.each(contractMutations())(
  "rejects snapshot recovery mutation: %s",
  (_name, expected, mutate, exact = false) =>
    assertExpectedFailure(expected, mutate, exact),
);

test.each(claimMutations())(
  "rejects premature scope claim: %s",
  (_name, expected, mutate) => assertExpectedFailure(expected, mutate, false),
);

function contractMutations(): Mutation[] {
  const files = {
    snapshot: "packages/open-bitcoin-node/src/storage/mempool_snapshot.rs",
    codec: "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs",
    codecDecode:
      "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode.rs",
    codecKeyPreflight:
      "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode/key_preflight.rs",
    codecTransactionDecode:
      "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode/transaction.rs",
    storage: "packages/open-bitcoin-node/src/storage.rs",
    topology: "packages/open-bitcoin-node/src/network/recovery/topology.rs",
    staging: "packages/open-bitcoin-node/src/network/recovery/staging.rs",
    recovery:
      "packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs",
    authority:
      "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs",
    dispatcher:
      "packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs",
    facade:
      "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
    effects: "packages/open-bitcoin-node/src/network/lifecycle_effects.rs",
    checkpointEffects:
      "packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs",
    coordinator: "packages/open-bitcoin-node/src/network/checkpoint.rs",
    store: "packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs",
    fjall: "packages/open-bitcoin-node/src/storage/fjall_store.rs",
    chainstateTypes: "packages/open-bitcoin-chainstate/src/types.rs",
    syncRuntime: "packages/open-bitcoin-node/src/sync.rs",
    startup: "packages/open-bitcoin-rpc/src/context/mempool_recovery.rs",
    startupContext: "packages/open-bitcoin-rpc/src/context/network.rs",
    daemonCheckpoint:
      "packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs",
    daemon: "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
    verify: "scripts/verify.sh",
    index: "docs/parity/index.json",
    checklist: "docs/parity/checklist.md",
  } as const;

  return [
    ...persistedInputMutations(files, PHASE135_DIAGNOSTICS, replace),
    [
      "v2 stores a derived fee",
      PHASE135_DIAGNOSTICS.schema,
      insertAfter(
        files.codec,
        "struct MempoolSnapshotV2RecordDto {",
        "\n    fee: u64,",
      ),
    ],
    [
      "v2 stores an extra source-looking field",
      PHASE135_DIAGNOSTICS.schema,
      insertAfter(
        files.codec,
        "accepted_at_unix_seconds: Option<i64>,",
        "\n    source_note: String,",
      ),
    ],
    [
      "v2 loses explicit nullable age",
      PHASE135_DIAGNOSTICS.schema,
      replace(files.codec, "Option<i64>", "i64"),
    ],
    [
      "v2 null fabricates a known age",
      PHASE135_DIAGNOSTICS.schema,
      replace(files.codec, "None => MempoolAcceptanceTime::LegacyUnknown,", "None => MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(0)),"),
    ],
    [
      "global schema changes",
      PHASE135_DIAGNOSTICS.schema,
      replace(
        files.storage,
        "pub const CURRENT: Self = Self(1);",
        "pub const CURRENT: Self = Self(2);",
      ),
    ],
    [
      "v1 becomes an encode path",
      PHASE135_DIAGNOSTICS.compatibility,
      replace(
        files.codec,
        "MempoolSnapshotV2Dto::try_from(snapshot)",
        "MempoolSnapshotV1Dto::try_from(snapshot)",
      ),
    ],
    [
      "legacy restores unbroadcast",
      PHASE135_DIAGNOSTICS.compatibility,
      replace(
        files.snapshot,
        "unbroadcast_members: BTreeSet::new(),",
        "unbroadcast_members: legacy_unbroadcast,",
      ),
    ],
    [
      "compatibility operation survives only in a comment",
      PHASE135_DIAGNOSTICS.compatibility,
      replace(
        files.codec,
        "let payload = decode::decode_bounded_versioned(bytes, limits)?;",
        "// decode::decode_bounded_versioned(bytes, limits)\n    return Err(snapshot_failure(MempoolSnapshotError::StructuralCorruption));",
      ),
    ],
    [
      "outcome variant removed",
      PHASE135_DIAGNOSTICS.topology,
      replace(files.snapshot, "DroppedPolicyIncompatible,", ""),
    ],
    [
      "install bypasses lifecycle command",
      PHASE135_DIAGNOSTICS.install,
      replace(
        files.facade,
        ".apply_lifecycle_command(LifecycleCommand::InstallRecovery(prepared))",
        ".install_prepared_recovery(prepared)",
      ),
    ],
    [
      "install leaves dirty generation",
      PHASE135_DIAGNOSTICS.install,
      replace(
        files.authority,
        "self.dirty_generation = None;",
        "self.dirty_generation = Some(generation);",
      ),
    ],
    [
      "install operation survives only in an ordinary string",
      PHASE135_DIAGNOSTICS.install,
      replace(
        files.authority,
        "self.dirty_generation = None;",
        'let _guarded_operation = "self.dirty_generation = None;";',
      ),
    ],
    [
      "capture stores a derived fee",
      PHASE135_DIAGNOSTICS.capture,
      insertAfter(
        files.dispatcher,
        "LifecycleCommand::PrepareSnapshot(request) => {",
        "\n            let fee = entry.fee();",
      ),
    ],
    [
      "snapshot capability becomes cloneable",
      PHASE135_DIAGNOSTICS.affine,
      insertAfter(
        files.checkpointEffects,
        "pub struct SnapshotWriteCapability {",
        "\n    // #[derive(Clone)]",
      ),
    ],
    [
      "more than one snapshot may be pending",
      PHASE135_DIAGNOSTICS.affine,
      replace(
        files.effects,
        "pub const MAX_PENDING_SNAPSHOT_EFFECTS: usize = 1;",
        "pub const MAX_PENDING_SNAPSHOT_EFFECTS: usize = 2;",
      ),
    ],
    [
      "encode happens in dispatcher",
      PHASE135_DIAGNOSTICS.execution,
      insertAfter(
        files.dispatcher,
        "LifecycleCommand::PrepareSnapshot(request) => {",
        "\n            encode_mempool_snapshot(snapshot);",
      ),
    ],
    [
      "snapshot write loses Sync",
      PHASE135_DIAGNOSTICS.execution,
      replace(
        files.store,
        "save(bytes, PersistMode::Sync)",
        "save(bytes, PersistMode::Flush)",
      ),
    ],
    [
      "persistence operation survives only in a raw string",
      PHASE135_DIAGNOSTICS.execution,
      replace(
        files.store,
        "if let Err(error) = save(bytes, PersistMode::Sync) {",
        'let _guarded_operation = r###"save(bytes, PersistMode::Sync) } {"###;\n    if let Err(error) = save(bytes, PersistMode::Flush) {',
      ),
    ],
    [
      "Sync stops mapping to SyncAll",
      PHASE135_DIAGNOSTICS.execution,
      replace(
        files.fjall,
        "PersistMode::Sync => Some(FjallPersistMode::SyncAll),",
        "PersistMode::Sync => Some(FjallPersistMode::Buffer),",
      ),
    ],
    [
      "achieved receipt is aborted",
      PHASE135_DIAGNOSTICS.coordinator,
      insertAfter(
        files.coordinator,
        "CheckpointCoordinatorState::AchievedAwaitingCompletion(receipt) => Ok(",
        "\n                receipt.abort();",
      ),
    ],
    [
      "retained receipt state removed",
      PHASE135_DIAGNOSTICS.coordinator,
      replace(
        files.coordinator,
        "AchievedAwaitingCompletion(SnapshotWriteReceipt)",
        "Idle",
      ),
    ],
    [
      "loss range evidence removed",
      PHASE135_DIAGNOSTICS.evidence,
      replace(
        files.authority,
        "pub maybe_generation_loss_range: Option<CheckpointGenerationLossRange>,",
        "pub maybe_generation_loss_range: Option<()>,",
      ),
    ],
    [
      "checkpoint interval becomes public cadence",
      PHASE135_DIAGNOSTICS.startup,
      replace(
        files.daemonCheckpoint,
        "Duration::from_secs(300)",
        "Duration::from_secs(30)",
      ),
    ],
    [
      "mempool recovery precedes durable chainstate load",
      PHASE135_DIAGNOSTICS.startup,
      replace(
        files.startupContext,
        "let durable_chainstate = match effective_store.as_ref() {",
        "recover_mempool_snapshot_from_store_handle;\n        let durable_chainstate = match effective_store.as_ref() {",
      ),
    ],
    [
      "authoritative runtime bypasses confirmation migration",
      PHASE135_DIAGNOSTICS.startup,
      replace(
        files.syncRuntime,
        "load_chainstate_snapshot_with_confirmation_migration()?",
        "load_chainstate_snapshot()?",
      ),
    ],
    [
      "confirmation migration trusts block key identity",
      PHASE135_DIAGNOSTICS.startup,
      replace(
        files.store,
        "if actual_hash != position.block_hash {",
        "if false {",
      ),
    ],
    [
      "generic snapshot fabricates confirmation evidence",
      PHASE135_DIAGNOSTICS.startup,
      replace(
        files.chainstateTypes,
        "maybe_confirmed_txid_counts: None,",
        "maybe_confirmed_txid_counts: Some(HashMap::new()),",
      ),
    ],
    [
      "clean marker precedes settle",
      PHASE135_DIAGNOSTICS.shutdown,
      replace(
        files.daemonCheckpoint,
        "settle()?;\n    mark_clean()",
        "mark_clean()?;\n    settle()\n        .and_then(|()| mark_clean())",
      ),
    ],
    [
      "producer shutdown follows checkpoint",
      PHASE135_DIAGNOSTICS.shutdown,
      replace(
        files.daemon,
        "if let Some(worker) = maybe_sync_worker {",
        "if let Some(worker) = maybe_checkpoint_worker {",
      ),
    ],
    [
      "parity index status completes",
      PHASE135_DIAGNOSTICS.parity,
      replace(
        files.index,
        '"name": "v2 snapshot schema, checkpointing, and recovery",\n      "status": "in_progress"',
        '"name": "v2 snapshot schema, checkpointing, and recovery",\n      "status": "complete"',
      ),
    ],
    [
      "checklist requirement completes",
      PHASE135_DIAGNOSTICS.parity,
      replace(
        files.checklist,
        "| MPDUR-01 | Pending |",
        "| MPDUR-01 | Complete |",
      ),
    ],
    [
      "verifier order drifts",
      PHASE135_DIAGNOSTICS.verifier,
      replace(
        files.verify,
        "bun test scripts/check-phase135-snapshot-recovery.test.ts",
        "bun test scripts/check-phase117-sync-foundations.test.ts",
      ),
    ],
    [
      "verifier command is only commented",
      PHASE135_DIAGNOSTICS.verifier,
      replace(
        files.verify,
        "bun run scripts/check-phase135-snapshot-recovery.ts",
        "# bun run scripts/check-phase135-snapshot-recovery.ts",
      ),
    ],
  ];
}

function claimMutations(): Mutation[] {
  const claims = [
    "Runtime transaction import is enabled.",
    "General package wire support is enabled.",
    "Whole-mempool rebroadcast ships by default.",
    "Public relay is enabled by default.",
    "Production relay is enabled.",
    "Transaction propagation is guaranteed.",
    "Public-network CI is release-blocking.",
    "Destructive repair is supported.",
    "Open Bitcoin is production ready.",
  ];
  return claims.map(
    (claim) =>
      [
        claim,
        PHASE135_DIAGNOSTICS.claims,
        append("docs/parity/catalog/mempool-policy.md", `\n${claim}\n`),
      ] as const,
  );
}

function assertExpectedFailure(
  expected: string,
  mutate: Mutator,
  exact: boolean,
): void {
  // Arrange
  const root = createFixture(mutate);

  // Act
  const failures = checkPhase135SnapshotRecovery(root);

  // Assert
  if (exact) {
    expect(failures).toEqual([expected]);
    return;
  }
  expect(failures).toContain(expected);
}

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase135-check-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  for (const relativePath of PHASE135_TARGET_FILES) {
    files.set(relativePath, readSourceRoot(REPO_ROOT, relativePath));
  }
  maybeMutate?.(files);
  for (const [relativePath, contents] of files) {
    const destination = path.join(root, relativePath);
    mkdirSync(path.dirname(destination), { recursive: true });
    writeFileSync(destination, contents);
  }
  return root;
}

function replace(
  relativePath: string,
  search: string,
  replacement: string,
): Mutator {
  return (files) => {
    const source = requireFile(files, relativePath);
    expect(source).toContain(search);
    files.set(relativePath, source.replace(search, replacement));
  };
}

function insertAfter(
  relativePath: string,
  marker: string,
  addition: string,
): Mutator {
  return replace(relativePath, marker, marker + addition);
}

function append(relativePath: string, addition: string): Mutator {
  return (files) =>
    files.set(relativePath, requireFile(files, relativePath) + addition);
}

function requireFile(files: Map<string, string>, relativePath: string): string {
  const maybeSource = files.get(relativePath);
  if (maybeSource === undefined) {
    throw new Error(`missing fixture file: ${relativePath}`);
  }
  return maybeSource;
}
