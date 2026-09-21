import { PHASE134_CLOSED_TARGETS } from "../check-phase134-authoritative-lifecycle";
import {
  type MutationCase,
  type Mutator,
  append,
  insertAfter,
  replace,
  replaceInFirst,
  replaceNth,
} from "./fixture";

export const DIAGNOSTICS = {
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

export function authorityMutations(): MutationCase[] {
  const secondOwners = [
    ["mempool", "    mempool: ManagedMempool,", "    shadow_mempool: ManagedMempool,\n"],
    [
      "unbroadcast",
      "    unbroadcast_members: BTreeSet<open_bitcoin_mempool::MempoolMemberIdentity>,",
      "    shadow_unbroadcast_members: BTreeSet<open_bitcoin_mempool::MempoolMemberIdentity>,\n",
    ],
    [
      "generation",
      "    lifecycle_generation: lifecycle_projection::LifecycleGeneration,",
      "    shadow_lifecycle_generation: lifecycle_projection::LifecycleGeneration,\n",
    ],
  ] as const;
  const ioMutations = [
    "TcpStream::connect(\"127.0.0.1:1\");",
    "FjallNodeStore::open(\"state\");",
    "writer.write_all(bytes);",
    "socket.write(bytes);",
    "future.await;",
  ];
  return [
    [
      "direct adapter mutation",
      DIAGNOSTICS.direct,
      append(
        "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
        "\nfn bypass(network: &mut ManagedPeerNetwork) { network.mempool_mut(); }\n",
      ),
    ],
    [
      "dispatcher bypass",
      DIAGNOSTICS.dispatcher,
      replace(
        "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
        ".apply_lifecycle_command(LifecycleCommand::PrepareRelay(",
        ".bypass_lifecycle_dispatcher(LifecycleCommand::PrepareRelay(",
      ),
    ],
    [
      "second peer-emission mutation interval",
      DIAGNOSTICS.dispatcher,
      insertAfter(
        "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
        "    pub fn complete_peer_emission(",
        "\n        self.try_mutate(|network| network.record_peer_emission_for_test());",
      ),
    ],
    ...secondOwners.map(
      ([name, marker, addition]): MutationCase => [
        `second ${name} owner`,
        DIAGNOSTICS.authority,
        insertAfter("packages/open-bitcoin-node/src/network.rs", marker, addition),
      ],
    ),
    ...ioMutations.map(
      (statement): MutationCase => [
        `I/O under authority: ${statement}`,
        DIAGNOSTICS.io,
        insertAfter(
          "packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs",
          "        apply_lifecycle_command(&mut network, command)",
          `;\n        ${statement}`,
        ),
      ],
    ),
    [
      "normal-path reconciliation",
      DIAGNOSTICS.normalReconcile,
      append(
        "packages/open-bitcoin-node/src/network/admission_bridge/singleton.rs",
        "\nfn forbidden(network: &ManagedPeerNetwork) { network.reconcile_lifecycle_projection(); }\n",
      ),
    ],
  ];
}

export function targetMutations(): MutationCase[] {
  const mutations: MutationCase[] = [];
  for (const target of PHASE134_CLOSED_TARGETS) {
    mutations.push(
      [
        `${target.name} construction`,
        DIAGNOSTICS.construction,
        replace(
          "packages/open-bitcoin-node/src/network/lifecycle_projection.rs",
          target.planField,
          `// removed ${target.name} plan field`,
        ),
      ],
      [
        `${target.name} apply`,
        DIAGNOSTICS.apply,
        replace(
          "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs",
          target.applyCall,
          `// removed ${target.name} apply`,
        ),
      ],
      [
        `${target.name} reconciliation`,
        DIAGNOSTICS.reconcile,
        replace(
          "packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs",
          target.reconcileCall,
          "0",
        ),
      ],
      [
        `${target.name} complete assertion`,
        DIAGNOSTICS.assertion,
        replace(
          "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs",
          target.assertion,
          `removed_${target.name}_assertion`,
        ),
      ],
    );
  }
  return mutations;
}

export function effectMutations(): MutationCase[] {
  const identityFields = [
    ["peer epoch", "packages/open-bitcoin-node/src/network/lifecycle_effects.rs", "    authority_epoch: AuthorityEpoch,", 1],
    ["peer generation", "packages/open-bitcoin-node/src/network/lifecycle_effects.rs", "    lifecycle_generation: LifecycleGeneration,", 1],
    ["peer effect", "packages/open-bitcoin-node/src/network/lifecycle_effects.rs", "    effect_id: PeerEffectId,", 1],
    ["peer session", "packages/open-bitcoin-node/src/network/lifecycle_effects.rs", "    peer_session_generation: PeerSessionGeneration,", 1],
    ["snapshot generation", "packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs", "    persistence_generation: LifecycleGeneration,", 2],
    ["snapshot effect", "packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs", "    effect_id: SnapshotEffectId,", 2],
    ["snapshot identity", "packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs", "    snapshot_identity: SnapshotIdentity,", 2],
  ] as const;
  return [
    [
      "Clone peer receipt",
      DIAGNOSTICS.receipt,
      replace(
        "packages/open-bitcoin-node/src/network/lifecycle_effects.rs",
        "#[derive(Debug, PartialEq, Eq)]\npub struct PeerEffectReceipt",
        "#[derive(Debug, Clone, PartialEq, Eq)]\npub struct PeerEffectReceipt",
      ),
    ],
    [
      "Clone snapshot receipt",
      DIAGNOSTICS.receipt,
      replace(
        "packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs",
        "#[derive(Debug, PartialEq, Eq)]\npub struct SnapshotWriteReceipt",
        "#[derive(Debug, Clone, PartialEq, Eq)]\npub struct SnapshotWriteReceipt",
      ),
    ],
    ...identityFields.map(
      ([name, relativePath, field, occurrence]): MutationCase => [
        `missing ${name}`,
        DIAGNOSTICS.identity,
        replaceNth(
          relativePath,
          field,
          `    // removed ${name}`,
          occurrence,
        ),
      ],
    ),
    [
      "unbounded peer ledger",
      DIAGNOSTICS.bounds,
      replace(
        "packages/open-bitcoin-node/src/network/lifecycle_effects.rs",
        "if self.pending.len() >= MAX_PENDING_PEER_EFFECTS {",
        "if false {",
      ),
    ],
    [
      "unbounded snapshot ledger",
      DIAGNOSTICS.bounds,
      replace(
        "packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs",
        "if self.pending.len() >= MAX_PENDING_SNAPSHOT_EFFECTS {",
        "if false {",
      ),
    ],
    [
      "stale snapshot clears current dirty state",
      DIAGNOSTICS.stale,
      replace(
        "packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs",
        "if is_fresh && network.dirty_generation == Some(receipt.persistence_generation()) {",
        "if network.dirty_generation.is_some() {",
      ),
    ],
    [
      "successful prefix not completed",
      DIAGNOSTICS.prefix,
      replace(
        "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
        "executor.complete(capability.acknowledge_write()).is_err()",
        "false",
      ),
    ],
  ];
}

export function scenarioMutations(): MutationCase[] {
  const scenarios = [
    ["full package", "full_package_projects_parent_first_final_membership_across_every_target"],
    ["partial package", "partial_package_projects_only_the_parent_survivor"],
    ["replacement", "replacement_package_tears_down_both_victim_aliases_and_fingerprint"],
    [
      "pressure",
      "pressure_eviction_tears_down_descendant_before_ancestor_across_every_projection",
    ],
    ["expiry", "expiry_removes_descendants_from_every_projection_and_advances_once"],
    [
      "connected block",
      "connected_block_conflict_removes_descendants_from_every_projection",
    ],
    ["reorg", "reorg_steps_apply_sequentially_and_reconcile_each_generation"],
    ["failed admission", "failed_package_admission_is_an_all_projection_noop"],
    [
      "stale receipt",
      "stale_snapshot_completion_records_truth_without_clearing_newer_dirty_state",
    ],
    ["duplicate receipt", "duplicate_peer_completion_precedes_stale_session_detection"],
    [
      "partial I/O",
      "phase134_rpc_successful_prefix_write_failure_stops_before_third_command",
    ],
  ] as const;
  const files = [
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission.rs",
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission/partial_package.rs",
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs",
    "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs",
    "packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs",
  ];
  return scenarios.map(([name, scenario]): MutationCase => [
    name,
    DIAGNOSTICS.scenarios,
    replaceInFirst(files, scenario, `removed_${name.replaceAll(" ", "_")}`),
  ]);
}

export function scopeMutations(): MutationCase[] {
  return [
    [
      "high-cardinality evidence",
      DIAGNOSTICS.evidence,
      insertAfter(
        "packages/open-bitcoin-node/src/network/lifecycle_projection.rs",
        "pub(super) struct LifecycleEvidenceSnapshot {",
        "\n    pub(super) txids: Vec<Txid>,",
      ),
    ],
    [
      "networked checker",
      DIAGNOSTICS.deterministic,
      append(
        "scripts/check-phase134-authoritative-lifecycle.ts",
        '\nBun.spawn(["git", "status"]);\n',
      ),
    ],
    ...verifierMutations(),
  ];
}

export function verifierMutations(): MutationCase[] {
  const verify = "scripts/verify.sh";
  const apply = "bun run scripts/check-phase134-apply-boundaries.ts";
  const test =
    "bun test scripts/check-phase134-authoritative-lifecycle.test.ts";
  const live =
    "bun run scripts/check-phase134-authoritative-lifecycle.ts";
  const phase135Test =
    "bun test scripts/check-phase135-snapshot-recovery.test.ts";
  const phase135Live =
    "bun run scripts/check-phase135-snapshot-recovery.ts";
  const phase117 =
    "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
  return [
    ["remove apply guard", DIAGNOSTICS.verifier, replace(verify, apply, "")],
    ["remove mutation guard", DIAGNOSTICS.verifier, replace(verify, test, "")],
    ["remove live guard", DIAGNOSTICS.verifier, replace(verify, live, "")],
    [
      "reorder mutation guard",
      DIAGNOSTICS.verifier,
      replace(verify, `${test}\n${apply}`, `${apply}\n${test}`),
    ],
    [
      "reorder apply guard",
      DIAGNOSTICS.verifier,
      replace(verify, `${apply}\n${live}`, `${live}\n${apply}`),
    ],
    [
      "reorder live guard",
      DIAGNOSTICS.verifier,
      replace(
        verify,
        `${live}\n${phase135Test}\n${phase135Live}\n${phase117}`,
        `${phase135Test}\n${phase135Live}\n${phase117}\n${live}`,
      ),
    ],
  ];
}
