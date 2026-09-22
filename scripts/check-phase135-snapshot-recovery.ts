#!/usr/bin/env bun

import path from "node:path";

import {
  addFailure,
  body,
  count,
  exactStructFields,
  hasAll,
  sameFields,
} from "./check-phase135-snapshot-recovery/source";
import { checkPersistedInputContract } from "./check-phase135-snapshot-recovery/persisted-input";
import {
  ARCHIVED_V22_REQUIREMENTS,
  V22_REQUIREMENTS_MILESTONE_NEEDLE,
  readPlanningRequirements,
  readSourceRoot,
} from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");

const FILES = {
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
  facade: "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
  effects: "packages/open-bitcoin-node/src/network/lifecycle_effects.rs",
  checkpointEffects:
    "packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs",
  evidence:
    "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs",
  coordinator: "packages/open-bitcoin-node/src/network/checkpoint.rs",
  store: "packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs",
  coins: "packages/open-bitcoin-node/src/storage/fjall_store/coins.rs",
  fjall: "packages/open-bitcoin-node/src/storage/fjall_store.rs",
  chainstateTypes: "packages/open-bitcoin-chainstate/src/types.rs",
  syncRuntime: "packages/open-bitcoin-node/src/sync/open_runtime.rs",
  startup: "packages/open-bitcoin-rpc/src/context/mempool_recovery.rs",
  startupContext: "packages/open-bitcoin-rpc/src/context/network.rs",
  daemonCheckpoint:
    "packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs",
  daemon: "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
  readme: "README.md",
  packageReadme: "packages/README.md",
  catalog: "docs/parity/catalog/mempool-policy.md",
  checklist: "docs/parity/checklist.md",
  index: "docs/parity/index.json",
  requirements: ".planning/REQUIREMENTS.md",
  checker: "scripts/check-phase135-snapshot-recovery.ts",
  checkerSource: "scripts/check-phase135-snapshot-recovery/source.ts",
  checkerPersistedInput:
    "scripts/check-phase135-snapshot-recovery/persisted-input.ts",
  checkerPersistedInputMutations:
    "scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts",
  test: "scripts/check-phase135-snapshot-recovery.test.ts",
  verify: "scripts/verify.sh",
} as const;

export const PHASE135_TARGET_FILES = [...new Set(Object.values(FILES))];

export const PHASE135_DIAGNOSTICS = {
  schema:
    "P135 schema: v2 must persist source facts only without changing the global schema",
  compatibility:
    "P135 compatibility: v1 must remain decode-only with conservative migration",
  bounds:
    "P135 load: byte, count, aggregate, and identity bounds must fail closed",
  topology:
    "P135 recovery: topology must be deterministic, bounded, and classify seven outcomes",
  staging:
    "P135 recovery: preparation must be side-effect-free with exact final membership",
  install:
    "P135 authority: recovery must install atomically through one lifecycle command",
  capture:
    "P135 capture: live snapshots must contain only canonical source facts",
  affine:
    "P135 effects: checkpoint capability and receipt ownership must remain affine and bounded",
  execution:
    "P135 persistence: encode and SyncAll durability must stay outside the authority lock",
  coordinator:
    "P135 coordinator: one flight must retain every achieved receipt until completion",
  evidence:
    "P135 evidence: durable generations and loss ranges must remain truthful and bounded",
  startup:
    "P135 daemon: recovery and private 300-second checkpointing must precede publication",
  shutdown:
    "P135 shutdown: producers must quiesce before final checkpoint and clean marking",
    parity:
    "P135 parity: evidence is done and MPDUR requirements are complete",
  claims:
    "P135 scope: broad relay, public-network, repair, and readiness claims must remain deferred",
  deterministic:
    "P135 checker: verification must remain deterministic and filesystem-only",
  verifier:
    "P135 verifier: mutation and live guards must immediately follow Phase 134",
} as const;

export function checkPhase135SnapshotRecovery(
  maybeRepoRoot: string = DEFAULT_REPO_ROOT,
): string[] {
  const sources = new Map(
    PHASE135_TARGET_FILES.map((file) => [
      file,
      file === FILES.requirements
        ? readPlanningRequirements(
            maybeRepoRoot,
            ARCHIVED_V22_REQUIREMENTS,
            V22_REQUIREMENTS_MILESTONE_NEEDLE,
          )
        : readSourceRoot(maybeRepoRoot, file),
    ]),
  );
  const get = (file: string): string => sources.get(file) ?? "";
  const failures: string[] = [];

  const snapshot = get(FILES.snapshot);
  const codec = get(FILES.codec);
  const codecDecode = [
    get(FILES.codecDecode),
    get(FILES.codecKeyPreflight),
    get(FILES.codecTransactionDecode),
  ].join("\n");
  const storage = get(FILES.storage);
  const v2Record = body(codec, "struct MempoolSnapshotV2RecordDto");
  const v2Dto = body(codec, "struct MempoolSnapshotV2Dto");
  const encode = body(codec, "pub(crate) fn encode_mempool_snapshot(");
  addFailure(
    failures,
    !sameFields(exactStructFields(codec, "struct MempoolSnapshotV2RecordDto"), [
      "accepted_at_unix_seconds",
      "transaction",
    ]) ||
      !sameFields(exactStructFields(codec, "struct MempoolSnapshotV2Dto"), [
        "captured_at_unix_seconds",
        "captured_generation",
        "format_version",
        "records",
        "unbroadcast_members",
      ]) ||
      !hasAll(v2Record, [
        "transaction: Vec<u8>",
        "accepted_at_unix_seconds: Option<i64>",
      ]) ||
      !hasAll(v2Dto, [
        "captured_generation: u64",
        "captured_at_unix_seconds: i64",
        "records: Vec<MempoolSnapshotV2RecordDto>",
        "unbroadcast_members: Vec<MempoolMemberIdentityDto>",
      ]) ||
      !snapshot.includes("pub const CURRENT: Self = Self(2);") ||
      !storage.includes("pub const CURRENT: Self = Self(2);") ||
      !codec.includes("MempoolAcceptanceTime::LegacyUnknown => None,") ||
      !codec.includes("None => MempoolAcceptanceTime::LegacyUnknown,") ||
      !get(FILES.codecDecode).includes(
        'accepted_at_unix_seconds: required(self.maybe_accepted_at, "accepted_at_unix_seconds")?',
      ) ||
      !encode.includes("MempoolSnapshotV2Dto::try_from(snapshot)"),
    PHASE135_DIAGNOSTICS.schema,
  );

  const legacy = body(snapshot, "pub fn from_legacy_v1(");
  const decode = body(
    codec,
    "pub fn decode_mempool_snapshot_with_limits(",
  );
  addFailure(
    failures,
    !decode.includes("decode::decode_bounded_versioned(bytes, limits)") ||
      !hasAll(codecDecode, [
        "MempoolSnapshotPayloadDto::CurrentV2",
        "MempoolSnapshotPayloadDto::LegacyV1",
      ]) ||
      !hasAll(legacy, [
        "source: MempoolSnapshotSource::LegacyV1",
        "unbroadcast_members: BTreeSet::new()",
      ]) ||
      !codec.includes(
        "(None, None, None) => Ok(MempoolAcceptanceTime::LegacyUnknown)",
      ) ||
      !codec.includes("MempoolOrigin::RecoveryUnknown") ||
      encode.includes("MempoolSnapshotV1Dto"),
    PHASE135_DIAGNOSTICS.compatibility,
  );

  const validate = body(snapshot, "pub fn try_from_compatibility(");
  addFailure(
    failures,
    !hasAll(validate, [
      "txid != actual_txid",
      "wtxid != actual_wtxid",
      "virtual_size != actual_virtual_size",
    ]) ||
      !snapshot.includes("StructuralCorruption") ||
      !snapshot.includes("ResourceBoundExceeded") ||
      !snapshot.includes("IdentityMismatch"),
    PHASE135_DIAGNOSTICS.bounds,
  );

  const dispatcher = get(FILES.dispatcher);
  const capture = body(
    dispatcher,
    "LifecycleCommand::PrepareSnapshot(request) =>",
  );
  const recoveryStatus = body(snapshot, "pub enum MempoolRecoveryStatus");
  const outcomes = [
    "Recovered",
    "DroppedConfirmed",
    "DroppedDuplicate",
    "DroppedMissingParent",
    "DroppedPolicyIncompatible",
    "DroppedExpired",
    "DroppedEvicted",
  ];
  addFailure(
    failures,
    !hasAll(get(FILES.topology), [
      "BTreeMap",
      "BTreeSet",
      "pop_first()",
    ]) ||
      body(
        get(FILES.topology),
        "pub(crate) fn prepare_recovery_topology(",
      ).length === 0 ||
      outcomes.some((variant) => !recoveryStatus.includes(variant)),
    PHASE135_DIAGNOSTICS.topology,
  );

  checkPersistedInputContract(
    {
      codec,
      codecDecode,
      capture,
      store: get(FILES.store),
      topology: get(FILES.topology),
      staging: get(FILES.staging),
      startup: get(FILES.startup),
    },
    failures,
    PHASE135_DIAGNOSTICS,
  );

  const facade = get(FILES.facade);
  const authority = get(FILES.authority);
  const install = body(
    authority,
    "pub(in crate::network) fn install_prepared_recovery(",
  );
  addFailure(
    failures,
    !dispatcher.includes(
      "LifecycleCommand::InstallRecovery(prepared) => network",
    ) ||
      count(
        facade,
        ".apply_lifecycle_command(LifecycleCommand::InstallRecovery(prepared))",
      ) !== 1 ||
      install.length === 0 ||
      !hasAll(install, [
        "PreparedRecoveryProjection::prepare(self, prepared)",
        "*self.mempool.mempool_mut() = staged_mempool;",
        "self.unbroadcast_members = unbroadcast_members;",
        "self.lifecycle_generation = generation;",
        "self.dirty_generation = None;",
        ".install_recovery(generation, maybe_captured_at)",
      ]),
    PHASE135_DIAGNOSTICS.install,
  );

  addFailure(
    failures,
    capture.length === 0 ||
      !hasAll(capture, [
        "MempoolSnapshotRecord::try_from_canonical",
        "entry.transaction.clone()",
        "entry.metadata.accepted_at",
        "MempoolSnapshot::try_new_current",
        "network.unbroadcast_members().clone()",
        "snapshot_effect_ledger.reserve_next",
      ]) ||
      /\b(?:fee|vsize)\b/.test(capture),
    PHASE135_DIAGNOSTICS.capture,
  );

  const effects = get(FILES.effects);
  const checkpointEffects = get(FILES.checkpointEffects);
  addFailure(
    failures,
    !effects.includes("pub const MAX_PENDING_SNAPSHOT_EFFECTS: usize = 1;") ||
      /#\[derive\([^\]]*Clone[^\]]*\)\]\s*pub struct (?:PreparedSnapshotWrite|SnapshotWriteCapability|SnapshotWriteReceipt)/.test(
        checkpointEffects,
      ) ||
      !checkpointEffects.includes(
        "if self.pending.len() >= MAX_PENDING_SNAPSHOT_EFFECTS {",
      ) ||
      !checkpointEffects.includes("SnapshotWriteReceipt") ||
      checkpointEffects.includes(
        "pub struct SnapshotWriteCapability {\n    // #[derive(Clone)]",
      ),
    PHASE135_DIAGNOSTICS.affine,
  );

  const store = get(FILES.store);
  const fjall = get(FILES.fjall);
  const execute = body(
    store,
    "\nfn execute_prepared_mempool_snapshot_write_with<",
  );
  const dispatch = body(
    dispatcher,
    "pub(in crate::network) fn apply_lifecycle_command",
  );
  addFailure(
    failures,
    execute.length === 0 ||
      !hasAll(execute, [
        "let (snapshot, capability) = prepared.into_parts();",
        "let bytes = match encode(&snapshot)",
        "save(bytes, PersistMode::Sync)",
        "capability.acknowledge_write(now(), CheckpointPersistenceStrength::Sync)",
      ]) ||
      !fjall.includes(
        "PersistMode::Sync => Some(FjallPersistMode::SyncAll),",
      ) ||
      /encode_mempool_snapshot|Fjall|SyncAll|put_bytes/.test(dispatch),
    PHASE135_DIAGNOSTICS.execution,
  );

  const coordinator = get(FILES.coordinator);
  const claim = body(coordinator, "fn claim_flight(&self)");
  const complete = body(coordinator, "fn complete_or_retain<");
  addFailure(
    failures,
    !hasAll(coordinator, [
      "enum CheckpointCoordinatorState",
      "Idle",
      "Persisting",
      "UnachievedAwaitingAbort(super::SnapshotWriteAbort)",
      "AchievedAwaitingCompletion(SnapshotWriteReceipt)",
      "const MAX_WRITES_PER_CALL: u8 = 2;",
    ]) ||
      !claim.includes(
        "CheckpointCoordinatorState::AchievedAwaitingCompletion(receipt)",
      ) ||
      !claim.includes(
        "CheckpointCoordinatorState::UnachievedAwaitingAbort(abort)",
      ) ||
      !complete.includes(
        "CheckpointCoordinatorState::AchievedAwaitingCompletion(receipt)",
      ) ||
      !coordinator.includes("fn abort_or_retain<") ||
      !coordinator.includes(
        "CheckpointCoordinatorState::UnachievedAwaitingAbort(abort)",
      ) ||
      /\breceipt\.abort\s*\(/.test(coordinator),
    PHASE135_DIAGNOSTICS.coordinator,
  );

  const evidence = get(FILES.evidence);
  addFailure(
    failures,
    !hasAll(evidence, [
      "maybe_last_durable_generation",
      "pub maybe_generation_loss_range: Option<CheckpointGenerationLossRange>",
      "CheckpointGenerationLossRange",
      "through_generation",
      "maybe_after_generation",
      "CheckpointPersistenceStrength::Sync",
    ]),
    PHASE135_DIAGNOSTICS.evidence,
  );

  const startup = get(FILES.startup);
  const startupContext = get(FILES.startupContext);
  const syncRuntimeConstruction = body(
    get(FILES.syncRuntime),
    "pub fn open_with_runtime_activation(",
  );
  const leftoverMigrate = body(
    get(FILES.coins),
    "fn migrate_schema_1_coins(",
  );
  const confirmationMigration = body(
    get(FILES.store),
    "pub fn load_chainstate_snapshot_with_confirmation_migration(",
  );
  const genericSnapshotConstruction = body(
    get(FILES.chainstateTypes),
    "impl ChainstateSnapshot {",
  );
  const startupConstruction = body(
    startupContext,
    "pub fn from_runtime_config_with_store(",
  );
  const chainstateLoad = startupConstruction.indexOf(
    "wallet_scan_chainstate_snapshot",
  );
  const handleConstruction = startupConstruction.indexOf(
    "ManagedNetworkHandle::from_network_fixture",
  );
  const recoveryInstall = startupConstruction.indexOf(
    "recover_mempool_snapshot_from_store_handle",
  );
  const publication = startupConstruction.indexOf("Self {");
  const daemonCheckpoint = get(FILES.daemonCheckpoint);
  addFailure(
    failures,
    !startup.includes("prepare_mempool_recovery_at") ||
      !startup.includes("install_mempool_recovery") ||
      !syncRuntimeConstruction.includes("initialize(") ||
      !syncRuntimeConstruction.includes("Chainstate::from_coins_cache") ||
      !syncRuntimeConstruction.includes("ManagedChainstate::from_chainstate") ||
      !leftoverMigrate.includes(
        "load_chainstate_snapshot_with_confirmation_migration()?",
      ) ||
      !hasAll(confirmationMigration, [
        "let actual_hash = block_hash(&block.header);",
        "actual_hash != position.block_hash",
        "StorageNamespace::Chainstate",
      ]) ||
      !genericSnapshotConstruction.includes(
        "maybe_confirmed_txid_counts: None",
      ) ||
      !startupConstruction.includes(
        "wallet_scan_chainstate_snapshot()?",
      ) ||
      chainstateLoad < 0 ||
      handleConstruction < chainstateLoad ||
      recoveryInstall < handleConstruction ||
      publication < recoveryInstall ||
      !daemonCheckpoint.includes("Duration::from_secs(300)") ||
      !daemonCheckpoint.includes("MempoolCheckpointCoordinator::new()"),
    PHASE135_DIAGNOSTICS.startup,
  );

  const daemon = get(FILES.daemon);
  const clean = body(daemonCheckpoint, "pub(super) fn settle_and_mark_clean<");
  const producerStop = daemon.indexOf(
    "if let Some(worker) = maybe_sync_worker {",
  );
  const finalCheckpoint = daemon.indexOf(
    "if let Some(worker) = maybe_checkpoint_worker {",
  );
  addFailure(
    failures,
    clean.indexOf("settle()?;") < 0 ||
      clean.indexOf("mark_clean()") < clean.indexOf("settle()?;") ||
      producerStop < 0 ||
      finalCheckpoint < producerStop ||
      !daemon.includes("worker.shutdown_and_mark_clean()?"),
    PHASE135_DIAGNOSTICS.shutdown,
  );

  const index = get(FILES.index);
  const catalog = get(FILES.catalog);
  const checklist = get(FILES.checklist);
  const requirements = get(FILES.requirements);
  let topLevelStatus = "invalid";
  let checklistStatus = "invalid";
  try {
    const parsed = JSON.parse(index) as {
      surfaces?: Array<{ name?: string; status?: string }>;
      checklist?: { surfaces?: Array<{ id?: string; status?: string }> };
    };
    topLevelStatus =
      parsed.surfaces?.find(
        (surface) =>
          surface.name === "v2 snapshot schema, checkpointing, and recovery",
      )?.status ?? "missing";
    checklistStatus =
      parsed.checklist?.surfaces?.find(
        (surface) =>
          surface.id === "v2-2-snapshot-schema-checkpointing-recovery",
      )?.status ?? "missing";
  } catch {
    topLevelStatus = "invalid";
  }
  addFailure(
    failures,
    topLevelStatus !== "done" ||
      checklistStatus !== "done" ||
      !checklist.includes(
        "| v2 snapshot schema, checkpointing, and recovery | Done |",
      ) ||
      !catalog.includes(
        "Format-owned candidate bounds govern persisted load and topology before current",
      ) ||
      !catalog.includes(
        "Fresh current policy governs live capture, replay, capacity trimming, and final",
      ) ||
      !checklist.includes(
        "Format-owned candidate bounds govern persisted load/topology; fresh current policy governs live capture, replay, and final membership",
      ) ||
      !index.includes(
        "Format-owned candidate bounds govern persisted load and topology; fresh current policy governs live capture, replay, and final membership",
      ) ||
      !hasAll([catalog, checklist, index].join("\n"), [
        "WR-01 remains open and non-blocking",
      ]) ||
      ["MPDUR-01", "MPDUR-02", "MPDUR-03", "MPDUR-04"].some(
        (id) =>
          !requirements.includes(`- [x] **${id}**`) ||
          !checklist.includes(`| ${id} | Complete |`),
      ),
    PHASE135_DIAGNOSTICS.parity,
  );

  const claimCorpus = [
    FILES.readme,
    FILES.packageReadme,
    FILES.catalog,
    FILES.checklist,
    FILES.index,
  ]
    .map(get)
    .join("\n")
    .toLowerCase();
  const prohibitedClaims = [
    /runtime transaction import is enabled/,
    /general package wire support is enabled/,
    /whole-mempool rebroadcast (?:ships|is enabled)/,
    /public relay is enabled by default/,
    /production relay is enabled/,
    /transaction propagation is guaranteed/,
    /public-network ci is (?:the default|release-blocking)/,
    /destructive repair is (?:supported|enabled)/,
    /(?:open bitcoin is production ready|ready for production)/,
  ];
  addFailure(
    failures,
    prohibitedClaims.some((claim) => claim.test(claimCorpus)),
    PHASE135_DIAGNOSTICS.claims,
  );

  const checker = [
    get(FILES.checker),
    get(FILES.checkerSource),
    get(FILES.checkerPersistedInput),
    get(FILES.checkerPersistedInputMutations),
  ].join("\n");
  addFailure(
    failures,
    /Bun\.(?:spawn|spawnSync)|fetch\s*\(|https?:\/\/|Date\.now|Math\.random/.test(
      checker,
    ),
    PHASE135_DIAGNOSTICS.deterministic,
  );

  const verify = get(FILES.verify);
  const expectedOrder = [
    "check-phase134-authoritative-lifecycle.test.ts",
    "check-phase134-apply-boundaries.ts",
    "check-phase134-authoritative-lifecycle.ts",
    "check-phase135-snapshot-recovery.test.ts",
    "check-phase135-snapshot-recovery.ts",
    "check-phase117-parity-uat-release-boundary.test.ts",
    "check-phase117-parity-uat-release-boundary.ts",
  ];
  const observed = verify
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => !line.startsWith("#"))
    .map((line) => {
      const command = line.match(
        /^(?:bun\s+(?:test|run)|run_step\s+"[^"]+"\s+bun\s+(?:test|run))\s+(\S+)/,
      );
      return expectedOrder.find((token) => command?.[1].endsWith(token));
    })
    .filter((token): token is string => token !== undefined);
  addFailure(
    failures,
    observed.length !== expectedOrder.length * 2 ||
      observed.some(
        (token, index) => token !== expectedOrder[index % expectedOrder.length],
      ),
    PHASE135_DIAGNOSTICS.verifier,
  );

  return failures;
}

if (import.meta.main) {
  const failures = checkPhase135SnapshotRecovery();
  if (failures.length > 0) {
    for (const failure of failures) console.error(failure);
    process.exit(1);
  }
  console.log("Phase 135 snapshot recovery invariants verified.");
}
