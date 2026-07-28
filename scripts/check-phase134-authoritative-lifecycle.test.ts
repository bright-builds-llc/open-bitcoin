import { afterEach, expect, test } from "bun:test";
import {
  mkdirSync,
  mkdtempSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  PHASE134_CLOSED_TARGETS,
  PHASE134_TARGET_FILES,
  checkPhase134AuthoritativeLifecycle,
} from "./check-phase134-authoritative-lifecycle";
import {
  PHASE134_APPLY_TARGET_FILES,
  checkPhase134ApplyBoundaries,
} from "./check-phase134-apply-boundaries";
import { readSourceRoot } from "./source-corpus";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const tempRoots: string[] = [];
type Mutator = (files: Map<string, string>) => void;
type MutationCase = readonly [string, string, Mutator];

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

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 134 corpus", () => {
  // Arrange
  const root = createFixture(PHASE134_TARGET_FILES);

  // Act
  const failures = checkPhase134AuthoritativeLifecycle(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each(authorityMutations())(
  "rejects authority mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(targetMutations())(
  "rejects closed-target mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(effectMutations())(
  "rejects effect mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(scenarioMutations())(
  "rejects scenario mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(scopeMutations())(
  "rejects scope mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each([
  ["Result return", "fn apply_prepared_compact(", " -> Result<(), Error>"],
  ["question propagation", "fn apply_prepared_compact(", "\nlet _ = derive()?;"],
  ["identifier derivation", "fn apply_prepared_compact(", "\ntransaction_txid(&tx);"],
  ["I/O type", "fn apply_prepared_compact(", "\nFile::open(\"state\");"],
  ["I/O call", "fn apply_prepared_compact(", "\nwriter.write_all(bytes);"],
  ["async await", "fn apply_prepared_compact(", "\nfuture.await;"],
] as const)(
  "rejects exact apply-body mutation: %s",
  (_name, functionMarker, addition) => {
    // Arrange
    const root = createFixture(PHASE134_APPLY_TARGET_FILES, (files) => {
      insertInFunction(
        files,
        "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs",
        functionMarker,
        addition,
      );
    });

    // Act
    const failures = checkPhase134ApplyBoundaries(root);

    // Assert
    expect(failures).toHaveLength(1);
    expect(failures[0]).toContain("apply_prepared_compact");
  },
);

function authorityMutations(): MutationCase[] {
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

function targetMutations(): MutationCase[] {
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

function effectMutations(): MutationCase[] {
  const identityFields = [
    ["peer epoch", "    authority_epoch: AuthorityEpoch,", 1],
    ["peer generation", "    lifecycle_generation: LifecycleGeneration,", 1],
    ["peer effect", "    effect_id: PeerEffectId,", 1],
    ["peer session", "    peer_session_generation: PeerSessionGeneration,", 1],
    ["snapshot generation", "    persistence_generation: LifecycleGeneration,", 2],
    ["snapshot effect", "    effect_id: SnapshotEffectId,", 2],
    ["snapshot identity", "    snapshot_identity: SnapshotIdentity,", 2],
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
        "packages/open-bitcoin-node/src/network/lifecycle_effects.rs",
        "#[derive(Debug, PartialEq, Eq)]\npub struct SnapshotWriteReceipt",
        "#[derive(Debug, Clone, PartialEq, Eq)]\npub struct SnapshotWriteReceipt",
      ),
    ],
    ...identityFields.map(
      ([name, field, occurrence]): MutationCase => [
        `missing ${name}`,
        DIAGNOSTICS.identity,
        replaceNth(
          "packages/open-bitcoin-node/src/network/lifecycle_effects.rs",
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
        "packages/open-bitcoin-node/src/network/lifecycle_effects.rs",
        "if self.pending.len() >= MAX_PENDING_SNAPSHOT_EFFECTS {",
        "if false {",
      ),
    ],
    [
      "stale snapshot clears current dirty state",
      DIAGNOSTICS.stale,
      replace(
        "packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs",
        "if network.dirty_generation == Some(receipt.persistence_generation()) {",
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

function scenarioMutations(): MutationCase[] {
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

function scopeMutations(): MutationCase[] {
  const claims = [
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
    ...claims.map(
      (claim): MutationCase => [
        claim,
        DIAGNOSTICS.claims,
        append("README.md", `\n${claim}\n`),
      ],
    ),
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

function verifierMutations(): MutationCase[] {
  const verify = "scripts/verify.sh";
  const apply = "bun run scripts/check-phase134-apply-boundaries.ts";
  const test =
    "bun test scripts/check-phase134-authoritative-lifecycle.test.ts";
  const live =
    "bun run scripts/check-phase134-authoritative-lifecycle.ts";
  const phase117 =
    "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
  return [
    ["remove apply guard", DIAGNOSTICS.verifier, replace(verify, apply, "")],
    ["remove mutation guard", DIAGNOSTICS.verifier, replace(verify, test, "")],
    ["remove live guard", DIAGNOSTICS.verifier, replace(verify, live, "")],
    [
      "reorder apply guard",
      DIAGNOSTICS.verifier,
      replace(verify, `${apply}\n${test}`, `${test}\n${apply}`),
    ],
    [
      "reorder mutation guard",
      DIAGNOSTICS.verifier,
      replace(verify, `${test}\n${live}`, `${live}\n${test}`),
    ],
    [
      "reorder live guard",
      DIAGNOSTICS.verifier,
      replace(verify, `${live}\n${phase117}`, `${phase117}\n${live}`),
    ],
  ];
}

function assertExactFailure(expectedFailure: string, mutate: Mutator): void {
  // Arrange
  const root = createFixture(PHASE134_TARGET_FILES, mutate);

  // Act
  const failures = checkPhase134AuthoritativeLifecycle(root);

  // Assert
  expect(failures).toEqual([expectedFailure]);
}

function createFixture(
  relativePaths: readonly string[],
  maybeMutate?: Mutator,
): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase134-check-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  for (const relativePath of relativePaths) {
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

function replace(relativePath: string, search: string, replacement: string): Mutator {
  return (files) => {
    const source = requireFile(files, relativePath);
    expect(source).toContain(search);
    files.set(relativePath, source.replace(search, replacement));
  };
}

function replaceNth(
  relativePath: string,
  search: string,
  replacement: string,
  occurrence: number,
): Mutator {
  return (files) => {
    let source = requireFile(files, relativePath);
    let cursor = 0;
    for (let index = 1; index <= occurrence; index += 1) {
      const found = source.indexOf(search, cursor);
      expect(found).toBeGreaterThanOrEqual(0);
      if (index === occurrence) {
        source =
          source.slice(0, found) +
          replacement +
          source.slice(found + search.length);
        files.set(relativePath, source);
        return;
      }
      cursor = found + search.length;
    }
  };
}

function append(relativePath: string, addition: string): Mutator {
  return (files) => {
    files.set(relativePath, requireFile(files, relativePath) + addition);
  };
}

function insertAfter(relativePath: string, marker: string, addition: string): Mutator {
  return replace(relativePath, marker, marker + addition);
}

function replaceInFirst(
  relativePaths: readonly string[],
  search: string,
  replacement: string,
): Mutator {
  return (files) => {
    const relativePath = relativePaths.find((candidate) =>
      requireFile(files, candidate).includes(search),
    );
    expect(relativePath).toBeDefined();
    replace(relativePath ?? "", search, replacement)(files);
  };
}

function insertInFunction(
  files: Map<string, string>,
  relativePath: string,
  functionMarker: string,
  addition: string,
): void {
  const source = requireFile(files, relativePath);
  const functionStart = source.indexOf(functionMarker);
  expect(functionStart).toBeGreaterThanOrEqual(0);
  const brace = source.indexOf("{", functionStart);
  expect(brace).toBeGreaterThanOrEqual(0);
  files.set(
    relativePath,
    source.slice(0, brace + 1) + addition + source.slice(brace + 1),
  );
}

function requireFile(files: Map<string, string>, relativePath: string): string {
  const maybeSource = files.get(relativePath);
  if (maybeSource === undefined) {
    throw new Error(`missing fixture file: ${relativePath}`);
  }
  return maybeSource;
}
