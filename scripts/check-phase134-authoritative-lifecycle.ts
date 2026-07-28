#!/usr/bin/env bun

import path from "node:path";

import { readSourceRoot } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");

const FILES = {
  network: "packages/open-bitcoin-node/src/network.rs",
  authority: "packages/open-bitcoin-node/src/network/runtime_authority.rs",
  dispatcher:
    "packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs",
  effectFacade:
    "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
  effects: "packages/open-bitcoin-node/src/network/lifecycle_effects.rs",
  projection:
    "packages/open-bitcoin-node/src/network/lifecycle_projection.rs",
  apply:
    "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs",
  reconcile:
    "packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs",
  assertions:
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs",
  admission:
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission.rs",
  partial:
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission/partial_package.rs",
  maintenance:
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs",
  effectTests:
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs",
  rpcPrefix:
    "packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs",
  rpcRuntime:
    "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
  singleton:
    "packages/open-bitcoin-node/src/network/admission_bridge/singleton.rs",
  package: "packages/open-bitcoin-node/src/network/admission_bridge/package.rs",
  mempool:
    "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  announcement:
    "packages/open-bitcoin-node/src/network/announcement_transport.rs",
  readme: "README.md",
  checker: "scripts/check-phase134-authoritative-lifecycle.ts",
  verify: "scripts/verify.sh",
} as const;

export const PHASE134_TARGET_FILES = [...new Set(Object.values(FILES))];

export const PHASE134_CLOSED_TARGETS = [
  {
    name: "compact",
    planField: "pub(super) compact: PreparedCompactProjection,",
    applyCall: "self.apply_prepared_compact(compact);",
    reconcileCall: "self.compact_mismatch_count(&canonical),",
    assertion: "compact_members(network)",
  },
  {
    name: "serving",
    planField: "pub(super) serving: PreparedServingProjection,",
    applyCall: "self.apply_prepared_serving(serving);",
    reconcileCall: "self.serving_mismatch_count(&canonical),",
    assertion: "network.relay_serving.lifecycle_members_for_test()",
  },
  {
    name: "fanout",
    planField: "pub(super) fanout: PreparedFanoutProjection,",
    applyCall: "self.apply_prepared_fanout(fanout);",
    reconcileCall:
      "self.relay_fanout.lifecycle_mismatch_count(&canonical),",
    assertion: "network.relay_fanout.lifecycle_members_for_test()",
  },
  {
    name: "peer",
    planField: "pub(super) peers: PreparedPeerLifecycleProjection,",
    applyCall: "self.apply_prepared_peer_lifecycle(peers);",
    reconcileCall: "self.peer_mismatch_count(&canonical),",
    assertion: "network.peer_manager.mempool_lifecycle_snapshot()",
  },
  {
    name: "unbroadcast",
    planField: "pub(super) unbroadcast: PreparedUnbroadcastProjection,",
    applyCall: "self.apply_prepared_unbroadcast(unbroadcast);",
    reconcileCall: "self.unbroadcast_mismatch_count(&canonical),",
    assertion: "network.unbroadcast_members()",
  },
  {
    name: "persistence",
    planField: "pub(super) persistence: PreparedPersistenceProjection,",
    applyCall: "self.apply_prepared_persistence(persistence);",
    reconcileCall: "self.persistence_mismatch_count(),",
    assertion: "network.dirty_generation()",
  },
  {
    name: "evidence",
    planField: "pub(super) evidence: PreparedLifecycleEvidence,",
    applyCall: "self.apply_prepared_evidence(evidence);",
    reconcileCall: "self.evidence_mismatch_count(),",
    assertion: "network.lifecycle_evidence_snapshot()",
  },
] as const;

const DIAGNOSTICS = {
  authority:
    "P134 authority: ManagedNetworkHandle must remain the sole mutable lifecycle authority",
  dispatcher:
    "P134 dispatcher: every lifecycle and effect facade must use the shared dispatcher",
  direct:
    "P134 authority: adapters must not mutate lifecycle projections directly",
  io: "P134 authority: storage/network I/O must stay outside the authority lock",
  construction:
    "P134 targets: every closed projection target must be constructed",
  apply: "P134 targets: every closed projection target must be applied once",
  reconcile:
    "P134 targets: every closed projection target must be reconciled",
  assertion:
    "P134 targets: complete scenario assertions must cover every projection target",
  receipt:
    "P134 effects: receipts and write capabilities must remain affine",
  identity:
    "P134 effects: receipts must bind epoch, generation, effect, and family identity",
  bounds: "P134 effects: pending and completed effect ledgers must remain bounded",
  stale:
    "P134 effects: stale and duplicate completion must preserve newer authoritative state",
  prefix:
    "P134 effects: only each successfully written prefix may receive achieved credit",
  scenarios:
    "P134 scenarios: all eleven authoritative lifecycle scenarios must remain",
  normalReconcile:
    "P134 reconciliation: full reconciliation must not enter normal mutation paths",
  evidence:
    "P134 evidence: production lifecycle evidence must stay bounded and identifier-free",
  claims:
    "P134 scope: Phase 135-138 and broad relay/readiness claims must remain deferred",
  deterministic:
    "P134 checker: verification must remain deterministic and filesystem-only",
  verifier:
    "P134 verifier: apply, mutation, and live guards must immediately follow Phase 133",
} as const;

function blockBody(source: string, marker: string): string {
  const start = source.indexOf(marker);
  if (start < 0) {
    return "";
  }
  const brace = source.indexOf("{", start);
  if (brace < 0) {
    return "";
  }
  let depth = 0;
  for (let index = brace; index < source.length; index += 1) {
    if (source[index] === "{") {
      depth += 1;
    } else if (source[index] === "}") {
      depth -= 1;
      if (depth === 0) {
        return source.slice(brace + 1, index);
      }
    }
  }
  return "";
}

function structBody(source: string, name: string): string {
  return blockBody(source, `struct ${name}`);
}

function lineOccurrenceCount(source: string, line: string): number {
  return source.split("\n").filter((candidate) => candidate === line).length;
}

function addFailure(
  failures: string[],
  condition: boolean,
  diagnostic: string,
): void {
  if (condition && !failures.includes(diagnostic)) {
    failures.push(diagnostic);
  }
}

function hasAll(source: string, markers: readonly string[]): boolean {
  return markers.every((marker) => source.includes(marker));
}

export function checkPhase134AuthoritativeLifecycle(
  maybeRepoRoot: string = DEFAULT_REPO_ROOT,
): string[] {
  const source = new Map(
    PHASE134_TARGET_FILES.map((file) => [
      file,
      readSourceRoot(maybeRepoRoot, file),
    ]),
  );
  const get = (file: string): string => source.get(file) ?? "";
  const failures: string[] = [];

  const network = get(FILES.network);
  const authority = get(FILES.authority);
  addFailure(
    failures,
    authority.split("authority: Arc<Mutex<AuthoritativeNetwork>>").length - 1 !==
      1 ||
      /shadow_(?:mempool|unbroadcast|lifecycle_generation)/.test(network),
    DIAGNOSTICS.authority,
  );

  const dispatcher = get(FILES.dispatcher);
  const effectFacade = get(FILES.effectFacade);
  const lifecycleCommands = [
    "LifecycleCommand::SingletonAdmission(plan)",
    "LifecycleCommand::PackageAdmission(plan)",
    "LifecycleCommand::Pressure(plan)",
    "LifecycleCommand::Expiry(plan)",
    "LifecycleCommand::ConnectedBlock(plan)",
    "LifecycleCommand::ReorgStep(plan)",
    "LifecycleCommand::Maintenance(plan)",
    "LifecycleCommand::PrepareSnapshot(_request)",
    "LifecycleCommand::PrepareRelay(request)",
    "LifecycleCommand::CompletePeerEffect(receipt)",
    "LifecycleCommand::CompleteSnapshotEffect(receipt)",
  ];
  const effectFacadeCalls = [
    ".apply_lifecycle_command(LifecycleCommand::PrepareRelay(",
    ".apply_lifecycle_command(LifecycleCommand::PrepareSnapshot(",
    ".apply_lifecycle_command(LifecycleCommand::CompletePeerEffect(receipt))",
    ".apply_lifecycle_command(LifecycleCommand::CompleteSnapshotEffect(receipt))",
  ];
  addFailure(
    failures,
    !hasAll(dispatcher, lifecycleCommands) ||
      !hasAll(effectFacade, effectFacadeCalls),
    DIAGNOSTICS.dispatcher,
  );

  const adapterCorpus = [
    FILES.singleton,
    FILES.package,
    FILES.mempool,
    FILES.announcement,
    FILES.rpcRuntime,
  ]
    .map(get)
    .join("\n");
  addFailure(
    failures,
    /\.(?:mempool_mut)\s*\(|unbroadcast_members\.(?:insert|remove|clear)\s*\(|\.apply_prepared_lifecycle\s*\(/.test(
      adapterCorpus,
    ),
    DIAGNOSTICS.direct,
  );
  addFailure(
    failures,
    /\b(?:TcpStream|UdpSocket|Fjall|File::|OpenOptions::)|\.(?:read|write|write_all)\s*\(|\bawait\b/.test(
      dispatcher,
    ),
    DIAGNOSTICS.io,
  );

  const projection = get(FILES.projection);
  const apply = get(FILES.apply);
  const reconcile = get(FILES.reconcile);
  const assertions = blockBody(
    get(FILES.assertions),
    "fn assert_complete_projection",
  );
  addFailure(
    failures,
    PHASE134_CLOSED_TARGETS.some(
      (target) => !projection.includes(target.planField),
    ),
    DIAGNOSTICS.construction,
  );
  addFailure(
    failures,
    PHASE134_CLOSED_TARGETS.some(
      (target) => apply.split(target.applyCall).length - 1 !== 1,
    ),
    DIAGNOSTICS.apply,
  );
  addFailure(
    failures,
    PHASE134_CLOSED_TARGETS.some(
      (target) => !reconcile.includes(target.reconcileCall),
    ),
    DIAGNOSTICS.reconcile,
  );
  addFailure(
    failures,
    PHASE134_CLOSED_TARGETS.some(
      (target) => !assertions.includes(target.assertion),
    ),
    DIAGNOSTICS.assertion,
  );

  const effects = get(FILES.effects);
  const affineTypes = [
    "PeerEffectCapability",
    "PeerEffectReceipt",
    "PreparedSnapshotWrite",
    "SnapshotWriteCapability",
    "SnapshotWriteReceipt",
  ];
  addFailure(
    failures,
    affineTypes.some((name) =>
      new RegExp(
        `#\\[derive\\([^\\]]*Clone[^\\]]*\\)\\]\\s*pub struct ${name}\\b`,
      ).test(effects),
    ),
    DIAGNOSTICS.receipt,
  );
  const identityFieldCounts = [
    ["    authority_epoch: AuthorityEpoch,", 4],
    ["    lifecycle_generation: LifecycleGeneration,", 2],
    ["    effect_id: PeerEffectId,", 2],
    ["    peer_id: PeerId,", 2],
    ["    peer_session_generation: PeerSessionGeneration,", 2],
    ["    persistence_generation: LifecycleGeneration,", 2],
    ["    effect_id: SnapshotEffectId,", 2],
    ["    snapshot_identity: SnapshotIdentity,", 2],
  ] as const;
  addFailure(
    failures,
    identityFieldCounts.some(
      ([field, expected]) => lineOccurrenceCount(effects, field) !== expected,
    ),
    DIAGNOSTICS.identity,
  );
  const boundMarkers = [
    "if self.pending.len() >= MAX_PENDING_PEER_EFFECTS {",
    "if self.completed_order.len() <= MAX_COMPLETED_PEER_EFFECTS {",
    "if self.pending.len() >= MAX_PENDING_SNAPSHOT_EFFECTS {",
    "if self.completed_order.len() <= MAX_COMPLETED_SNAPSHOT_EFFECTS {",
    "completed_order: VecDeque<PeerEffectId>",
    "completed_order: VecDeque<SnapshotEffectId>",
  ];
  addFailure(
    failures,
    !hasAll(effects, boundMarkers),
    DIAGNOSTICS.bounds,
  );
  const staleMarkers = [
    "if network.peer_effect_ledger.is_completed(effect_id)",
    "if network.snapshot_effect_ledger.is_completed(effect_id)",
    "EffectCompletion::AlreadyApplied",
    "EffectCompletion::AchievedButStale",
    "if network.dirty_generation == Some(receipt.persistence_generation()) {",
  ];
  addFailure(
    failures,
    !hasAll(dispatcher, staleMarkers),
    DIAGNOSTICS.stale,
  );
  addFailure(
    failures,
    !get(FILES.rpcRuntime).includes(
      "executor.complete(capability.acknowledge_write()).is_err()",
    ),
    DIAGNOSTICS.prefix,
  );

  const scenarioCorpus = [
    FILES.admission,
    FILES.partial,
    FILES.maintenance,
    FILES.effectTests,
    FILES.rpcPrefix,
  ]
    .map(get)
    .join("\n");
  const scenarios = [
    "full_package_projects_parent_first_final_membership_across_every_target",
    "partial_package_projects_only_the_parent_survivor",
    "replacement_package_tears_down_both_victim_aliases_and_fingerprint",
    "pressure_eviction_tears_down_descendant_before_ancestor_across_every_projection",
    "expiry_removes_descendants_from_every_projection_and_advances_once",
    "connected_block_conflict_removes_descendants_from_every_projection",
    "reorg_steps_apply_sequentially_and_reconcile_each_generation",
    "failed_package_admission_is_an_all_projection_noop",
    "stale_snapshot_completion_records_truth_without_clearing_newer_dirty_state",
    "duplicate_peer_completion_precedes_stale_session_detection",
    "phase134_rpc_successful_prefix_write_failure_stops_before_third_command",
  ];
  addFailure(
    failures,
    !hasAll(scenarioCorpus, scenarios),
    DIAGNOSTICS.scenarios,
  );
  addFailure(
    failures,
    [
      FILES.singleton,
      FILES.package,
      FILES.mempool,
      FILES.announcement,
      FILES.rpcRuntime,
    ]
      .map(get)
      .some((contents) => contents.includes("reconcile_lifecycle_projection(")),
    DIAGNOSTICS.normalReconcile,
  );

  const evidence = structBody(projection, "LifecycleEvidenceSnapshot");
  const evidenceFields = evidence
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.startsWith("pub(super) "));
  addFailure(
    failures,
    evidenceFields.length === 0 ||
      evidenceFields.some((field) => !field.endsWith(": u64,")) ||
      /\b(?:Txid|Wtxid|Vec|String|BTreeMap|BTreeSet)\b/.test(evidence),
    DIAGNOSTICS.evidence,
  );

  const broadClaims = [
    "Phase 135 is implemented.",
    "Phase 136 is implemented.",
    "Phase 137 is implemented.",
    "Phase 138 is implemented.",
    "Open Bitcoin supports a general package wire.",
    "Open Bitcoin ships whole-mempool rebroadcast.",
    "Open Bitcoin supports public/default relay.",
    "Open Bitcoin guarantees transaction propagation.",
    "Open Bitcoin runs public-network CI.",
    "Open Bitcoin is production ready.",
  ];
  addFailure(
    failures,
    broadClaims.some((claim) => get(FILES.readme).includes(claim)),
    DIAGNOSTICS.claims,
  );
  addFailure(
    failures,
    /Bun\s*\.\s*spawn|\bfetch\s*\(|https?:\/\//.test(get(FILES.checker)) ||
      get(FILES.checker).includes(["node", "child_process"].join(":")),
    DIAGNOSTICS.deterministic,
  );

  const verifierNeedles = [
    ["bun test scripts/check-phase133-package-aware-download-orphan-bridge.test.ts", "phase133-test"],
    ["bun run scripts/check-phase133-package-aware-download-orphan-bridge.ts", "phase133-live"],
    ["bun run scripts/check-phase134-apply-boundaries.ts", "phase134-apply"],
    ["bun test scripts/check-phase134-authoritative-lifecycle.test.ts", "phase134-test"],
    ["bun run scripts/check-phase134-authoritative-lifecycle.ts", "phase134-live"],
    ["bun test scripts/check-phase117-parity-uat-release-boundary.test.ts", "phase117-test"],
    ["bun run scripts/check-phase117-parity-uat-release-boundary.ts", "phase117-live"],
  ] as const;
  const verifierTokens = get(FILES.verify)
    .split("\n")
    .flatMap((line) => {
      const maybeNeedle = verifierNeedles.find(([needle]) =>
        line.includes(needle),
      );
      return maybeNeedle ? [maybeNeedle[1]] : [];
    });
  const expectedVerifierSequence = verifierNeedles.map(([, token]) => token);
  addFailure(
    failures,
    verifierTokens.join("\n") !==
      [...expectedVerifierSequence, ...expectedVerifierSequence].join("\n"),
    DIAGNOSTICS.verifier,
  );

  return failures;
}

if (import.meta.main) {
  const failures = checkPhase134AuthoritativeLifecycle();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }
  console.log(
    "Phase 134 authoritative lifecycle authority, targets, effects, scenarios, evidence, and scope are guarded.",
  );
}
